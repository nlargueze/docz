//! Server

use anyhow::{anyhow, Error, Result};
use async_stream::stream;
use futures_core::Stream;
use log::{debug, trace, warn};
use salvo::{prelude::*, sse};
use tokio::sync::broadcast;

use crate::{
    watch::{EventExt, Watcher},
    Service,
};

/// Serve options
#[derive(Debug)]
pub struct ServeOptions {
    /// Service port
    pub port: u16,
    /// Opens the browser when launching the server
    pub open: bool,
    /// Watches the source files and restarts the server
    pub watch: bool,
}

impl Default for ServeOptions {
    fn default() -> Self {
        Self {
            port: 3000,
            open: false,
            watch: false,
        }
    }
}

impl ServeOptions {
    /// Creates a new set of values
    pub fn new() -> Self {
        Self::default()
    }
}

/// Wrapper for rebuilt channel sender
#[derive(Clone)]
struct RebuiltChannelSender(broadcast::Sender<()>);

impl Service {
    /// Servers the build
    pub async fn serve(self, opts: ServeOptions) -> Result<()> {
        // initial build
        self.build_once()?;
        debug!("Build - OK");

        // Rebuilt channel
        let (tx_rebuilt, _rx_rebuilt) = broadcast::channel(100);

        // setup server
        let addr = format!("127.0.0.1:{}", opts.port);
        let addr_full = format!("http://{}", addr);
        let acceptor = TcpListener::new(addr.as_str()).bind().await;
        let serve_dir = self.config.build_dir().join("html");
        let router = Router::new()
            .hoop(affix::inject(RebuiltChannelSender(tx_rebuilt.clone())))
            .push(Router::with_path("ss-events").get(get_sse_events))
            .push(
                Router::with_path("<**path>").get(
                    StaticDir::new([&serve_dir])
                        .defaults("index.html")
                        .listing(true),
                ),
            );
        let server = Server::new(acceptor);

        // server task
        let server_task = tokio::spawn(async move {
            eprintln!("Serving on http://{}", addr);
            server.serve(router).await;
            Ok(()) as Result<(), Error>
        });

        // watch task
        let watched_dirs = self.watched_dirs();
        let mut watcher = Watcher::new(watched_dirs, Some(200))?;
        let rx_watch = if opts.watch {
            Some(watcher.start()?)
        } else {
            None
        };

        // rebuild task
        let rebuild_task = tokio::spawn(async move {
            if let Some(mut rx_watch) = rx_watch {
                loop {
                    rx_watch.changed().await?;
                    let event = rx_watch.borrow().clone();
                    if event.triggers_rebuild() {
                        trace!("Rebuilding ...");
                        self.build_once()?;

                        match tx_rebuilt.send(()) {
                            Ok(_n) => {
                                debug!("Sent rebuilt channel signal");
                            }
                            Err(err) => {
                                warn!("Failed to send rebuilt signal: {err} ");
                                return Err(anyhow!(err));
                            }
                        }
                    }
                }
            } else {
                Ok(()) as Result<(), Error>
            }
        });

        // open
        if opts.open {
            if let Err(err) = open::that(&addr_full) {
                eprintln!("An error occurred when opening '{}': {}", addr_full, err)
            }
        }

        Ok(tokio::try_join!(server_task, rebuild_task).map(|_| ())?)
    }
}

/// Subscribe to server events
#[handler]
async fn get_sse_events(depot: &mut Depot, res: &mut Response) {
    debug!("GET /ss-events");
    let tx = depot.obtain::<RebuiltChannelSender>().unwrap();
    let rx = tx.0.subscribe();
    let stream = create_sse_stream(rx);
    sse::streaming(res, stream).ok();
}

/// Creates a stream for SSE events
fn create_sse_stream(
    mut rx: broadcast::Receiver<()>,
) -> impl Stream<Item = Result<SseEvent, broadcast::error::RecvError>> {
    stream! {
        loop {
            match  rx.recv().await {
                Ok(_) => {
                    debug!("Sending SSE rebuilt event");
                    let event = SseEvent::default().text("rebuilt");
                    yield Ok(event);
                },
                Err(err) => {
                    warn!("Error receiving rebuilt signal: {err}");
                    continue;
                }
            }
        }
    }
}
