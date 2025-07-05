use super::*;

pub mod axum {
    use std::sync::Arc;

    use bzauth_rs::runtimes::axum::AxumRuntimeOptions;
    use tokio::sync::Notify;

    use super::*;

    pub async fn run<F, Fut>(shutdown_receiver: Arc<Notify>, options: AxumRuntimeOptions, f: F)
    where
        F: FnOnce() -> Fut,
        Fut: Future<Output = ()>,
    {
        let server_future = provider_server::axum_::start(shutdown_receiver.clone());
        let runtime_future = runtime::axum::start(shutdown_receiver.clone(), options);

        // Run the server in a separate task
        let server_handle = tokio::spawn(async move {
            if let Err(e) = server_future.await {
                assert!(false, "Failed to start mock auth server: {}", e);
            }
        });
        let runtime_handle = tokio::spawn(async move {
            if let Err(e) = runtime_future.await {
                assert!(false, "Failed to start mock auth runtime: {}", e);
            }
        });

        // Allow some time for the servers to start
        tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;

        f().await;

        // Signal the server to shut down
        shutdown_receiver.notify_waiters();

        // Wait for the server to finish shutting down
        let _ = server_handle.await;
        let _ = runtime_handle.await;
    }
}
