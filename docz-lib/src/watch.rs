//! Watcher

use std::{path::PathBuf, time::Instant};

use anyhow::Result;
use log::{debug, warn};
use notify::{Event, EventKind, FsEventWatcher, RecursiveMode, Watcher as _};
use tokio::sync::watch;

/// Watcher
#[derive(Debug)]
pub(crate) struct Watcher {
    dirs: Vec<PathBuf>,
    watcher: FsEventWatcher,
    rx: watch::Receiver<Event>,
}

impl Watcher {
    /// Creates a new watcher
    ///
    /// We pass the dirs to watch, and an optional debounce time (in ms)
    pub fn new(dirs: Vec<PathBuf>, debounce: Option<u128>) -> Result<Self> {
        let (tx, rx) = watch::channel(Event::default());
        let mut last_event = Instant::now();
        let watcher =
            notify::recommended_watcher(move |res: Result<notify::Event, notify::Error>| {
                match res {
                    Ok(event) => {
                        debug!("Watch event: {:?}", event);

                        // NB: debouncing
                        if let Some(debounce_ms) = debounce {
                            if last_event.elapsed().as_millis() < debounce_ms {
                                // debug!("Watch event debounced");
                                last_event = Instant::now();
                                return;
                            }
                        }
                        last_event = Instant::now();

                        if let Err(err) = tx.send(event) {
                            warn!("Error sending watch event: {}", err)
                        }
                    }
                    Err(e) => warn!("watch error: {:?}", e),
                }
            })?;

        Ok(Self {
            dirs: dirs.iter().map(|p| p.to_path_buf()).collect(),
            watcher,
            rx,
        })
    }

    /// Starts watching
    pub fn start(&mut self) -> Result<watch::Receiver<Event>> {
        for dir in &self.dirs {
            self.watcher.watch(dir, RecursiveMode::Recursive)?;
        }
        Ok(self.rx.clone())
    }
}

/// Extension trait for events
pub(crate) trait EventExt {
    /// Triggers a rebuild
    fn triggers_rebuild(&self) -> bool;
}

impl EventExt for Event {
    fn triggers_rebuild(&self) -> bool {
        matches!(
            self.kind,
            EventKind::Create(_) | EventKind::Modify(_) | EventKind::Remove(_)
        )
    }
}
