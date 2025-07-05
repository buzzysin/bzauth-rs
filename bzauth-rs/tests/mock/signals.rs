use std::sync::Arc;

use tokio::sync::Notify;

#[derive(Debug, Clone)]
pub struct Signals {
    shutdown: Arc<Notify>,
    ready: Arc<Notify>,
}

impl Signals {
    pub fn new() -> Self {
        Self {
            shutdown: Arc::new(Notify::new()),
            ready: Arc::new(Notify::new()),
        }
    }

    pub fn ready(&self) -> Arc<Notify> {
        self.ready.clone()
    }

    pub fn shutdown(&self) -> Arc<Notify> {
        self.shutdown.clone()
    }

    pub fn notify_ready(&self) {
        self.ready.notify_one();
    }

    pub fn notify_shutdown(&self) {
        self.shutdown.notify_waiters();
    }

    pub async fn wait_for_ready(&self) {
        self.ready.notified().await;
    }

    pub async fn wait_for_shutdown(&self) {
        self.shutdown.notified().await;
    }
}
