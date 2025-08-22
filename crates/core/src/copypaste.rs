use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread::{self, JoinHandle};
use std::time::Duration;

/// Watches a clipboard-like source for changes and triggers a callback when
/// new content is detected.
pub struct ClipboardWatcher<F>
where
    F: Fn() -> Option<String> + Send + Sync + 'static,
{
    getter: Arc<F>,
    stop: Arc<AtomicBool>,
    handle: Option<JoinHandle<()>>,
    interval: Duration,
}

impl<F> ClipboardWatcher<F>
where
    F: Fn() -> Option<String> + Send + Sync + 'static,
{
    /// Create a new watcher with the provided clipboard getter function.
    pub fn new(getter: F) -> Self {
        Self {
            getter: Arc::new(getter),
            stop: Arc::new(AtomicBool::new(false)),
            handle: None,
            interval: Duration::from_millis(500),
        }
    }

    /// Start watching the clipboard and invoke `callback` on changes.
    pub fn start<C>(&mut self, mut callback: C)
    where
        C: FnMut(String) + Send + 'static,
    {
        let getter = self.getter.clone();
        let stop = self.stop.clone();
        let interval = self.interval;
        self.handle = Some(thread::spawn(move || {
            let mut last: Option<String> = None;
            while !stop.load(Ordering::SeqCst) {
                if let Some(current) = getter() {
                    if last.as_ref().map_or(true, |l| *l != current) {
                        callback(current.clone());
                        last = Some(current);
                    }
                }
                thread::sleep(interval);
            }
        }));
    }

    /// Stop watching the clipboard.
    pub fn stop(&mut self) {
        self.stop.store(true, Ordering::SeqCst);
        if let Some(handle) = self.handle.take() {
            let _ = handle.join();
        }
    }
}
