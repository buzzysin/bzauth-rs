use super::*;

pub mod axum_ {
    use bzauth_rs::runtimes::axum::AxumRuntimeOptions;

    use super::*;

    pub async fn run<F, Fut>(signals: Signals, options: AxumRuntimeOptions, f: F)
    where
        F: FnOnce() -> Fut,
        Fut: Future<Output = ()>,
    {
        let server_future = provider_server::axum_::start(signals.clone());
        let runtime_future = runtime::axum::start(signals.clone(), options);

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
        println!("Waiting for servers to start...");
        signals.wait_for_ready().await; // one server is ready
        println!("At least one server is ready");
        signals.wait_for_ready().await; // both servers are ready
        println!("Both servers are ready");

        // Run the test function
        println!("Running test function");
        f().await;
        println!("Test function completed");

        // Signal the servers to shut down
        println!("Signaling servers to shut down");
        signals.notify_shutdown();
        println!("Servers signaled to shut down");

        // Wait for the server to finish shutting down
        println!("Waiting for server handles to complete");
        let _ = server_handle.await;
        let _ = runtime_handle.await;
        println!("Server handles completed");
    }
}
