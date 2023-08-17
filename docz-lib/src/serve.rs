//! Server

use std::time::Instant;

use anyhow::{Error, Result};
use async_stream::stream;
use futures_core::Stream;
use log::{debug, warn};
use notify::{EventKind, RecursiveMode, Watcher};
use salvo::{prelude::*, sse};
use tokio::sync::{broadcast, watch};

use crate::Service;

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
        self.build()?;
        debug!("Build - OK");

        // setup watch
        let (tx_watch, mut rx_watch) = watch::channel(false);
        let mut watcher = notify::recommended_watcher(
            move |res: Result<notify::Event, notify::Error>| match res {
                Ok(event) => {
                    // debug!("Received event: {:?}", event);
                    let rebuild = matches!(
                        event.kind,
                        EventKind::Create(_) | EventKind::Modify(_) | EventKind::Remove(_)
                    );
                    if let Err(err) = tx_watch.send(rebuild) {
                        warn!("Error sending watch event: {}", err)
                    }
                }
                Err(e) => warn!("watch error: {:?}", e),
            },
        )?;
        if opts.watch {
            let src_dir = self.config.src_dir();
            debug!("Watching {:?}", src_dir);
            watcher.watch(&src_dir, RecursiveMode::Recursive).unwrap();
        }

        // Rebuiltsignal channel
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

        let server_task = tokio::spawn(async move {
            eprintln!("Serving on http://{}", addr);
            server.serve(router).await;
            Ok(()) as Result<(), Error>
        });

        let rebuild_task = tokio::spawn(async move {
            let mut last_restart = Instant::now();
            let err = loop {
                match rx_watch.changed().await {
                    Ok(_) => {
                        let rebuild = *rx_watch.borrow();
                        if rebuild {
                            // NB: debouncing
                            if last_restart.elapsed().as_millis() < 200 {
                                continue;
                            }
                            last_restart = Instant::now();

                            debug!("Rebuilding ...");
                            self.build()?;
                            debug!("Rebuilt OK");
                            match tx_rebuilt.send(()) {
                                Ok(_n) => {
                                    debug!("Sent rebuilt channel signal");
                                }
                                Err(err) => {
                                    warn!("Failed to send rebuilt signal: {err} ");
                                    break err;
                                }
                            }
                        }
                    }
                    Err(err) => {
                        warn!("failed to received watch signal {err}");
                    }
                }
            };
            Err(err.into()) as Result<(), Error>
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
