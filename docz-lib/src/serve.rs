//! Server

use anyhow::{anyhow, Context, Error, Result};
use async_stream::stream;
use futures_core::Stream;
use log::{debug, trace, warn};
use salvo::{prelude::*, sse};
use tokio::sync::broadcast;

use crate::{
    watch::{EventExt, WatchOptions, Watcher},
    Service,
};

/// Serve options
pub struct ServeOptions {
    /// Service port
    pub port: u16,
    /// Opens the browser when launching the server
    pub open: bool,
}

impl Default for ServeOptions {
    fn default() -> Self {
        Self {
            port: 3000,
            open: false,
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
    /// Serves the build
    ///
    /// If `watch_opts` is set, it will watch for changes and rebuild the docs
    pub async fn serve(
        mut self,
        opts: ServeOptions,
        watch_opts: Option<WatchOptions>,
    ) -> Result<()> {
        // init
        let addr = format!("127.0.0.1:{}", opts.port);
        let addr_full = format!("http://{}", addr);
        let serve_dir = self.config.build_dir().join("html");
        let (tx_rebuilt, _rx_rebuilt) = broadcast::channel(100);

        // build
        self.build()?;
        debug!("Build - OK");

        // server task
        let tx_rebuilt_server = tx_rebuilt.clone();
        let server_task = tokio::spawn(async move {
            let router = Router::new()
                .hoop(affix::inject(RebuiltChannelSender(tx_rebuilt_server)))
                .push(Router::with_path("ss-events").get(get_sse_events))
                .push(
                    Router::with_path("<**path>").get(
                        StaticDir::new([&serve_dir])
                            .defaults("index.html")
                            .listing(true),
                    ),
                );
            let acceptor = TcpListener::new(addr.as_str()).bind().await;
            let server = Server::new(acceptor);

            eprintln!("Serving on http://{}", addr);
            server.serve(router).await;
            Ok(()) as Result<(), Error>
        });

        // watch task
        let watched_dirs = self.watched_dirs();
        let mut watcher = Watcher::new(watched_dirs, Some(200))?;
        let (rx_watch, on_rebuilt) = if let Some(watch_opts) = watch_opts {
            (Some(watcher.start()?), watch_opts.on_rebuilt)
        } else {
            (None, None)
        };

        // rebuild task
        let rebuild_task = tokio::spawn(async move {
            if let Some(mut rx_watch) = rx_watch {
                loop {
                    rx_watch.changed().await?;
                    let event = rx_watch.borrow().clone();
                    if event.triggers_rebuild() {
                        trace!("Rebuilding ...");
                        self.reload()?;
                        self.build()?;
                        if let Some(on_rebuilt) = on_rebuilt {
                            on_rebuilt(event);
                        }

                        if let Err(err) = tx_rebuilt.send(()) {
                            warn!("Failed to send rebuilt signal: {err} ");
                            return Err(anyhow!(err));
                        } else {
                            debug!("Sent rebuilt channel signal");
                        }
                    }
                }
            } else {
                Ok(()) as Result<(), Error>
            }
        });

        // open
        if opts.open {
            open::that(&addr_full).context(format!("Failed to open '{}'", addr_full))?;
        }

        Ok(tokio::try_join!(server_task, rebuild_task).map(|_res| ())?)
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
) -> impl Stream<Item = ::std::result::Result<SseEvent, broadcast::error::RecvError>> {
    stream! {
        loop {
            if let Err(err) = rx.recv().await {
                warn!("Error receiving rebuilt signal: {err}");
                continue;
            } else {
                debug!("Sending SSE rebuilt event");
                let event = SseEvent::default().text("rebuilt");
                yield Ok(event);
            }
        }
    }
}
