use super::*;

pub mod axum_ {
    use bzauth_rs::runtimes::axum::AxumRuntimeOptions;

    use super::*;

    fn with_tracing<F, Out>(f: F) -> Out
    where
        F: FnOnce() -> Out,
    {
        let subscriber = tracing_subscriber::fmt()
            .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
            .with_line_number(true)
            .with_file(true)
            .with_span_events(tracing_subscriber::fmt::format::FmtSpan::CLOSE)
            .finish();

        tracing::subscriber::with_default(subscriber, || f())
    }

    pub async fn run<F, Fut, Out>(signals: Signals, options: AxumRuntimeOptions, f: F) -> Out
    where
        F: FnOnce() -> Fut,
        Fut: Future<Output = Out>,
    {
        let server_future = server_provider::axum_::start(signals.clone());
        let runtime_future = server_runtime::axum::start(signals.clone(), options);

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
        println!("[environment] Waiting for servers to start...");
        signals.wait_for_ready().await; // one server is ready
        println!("[environment] At least one server is ready");
        signals.wait_for_ready().await; // both servers are ready
        println!("[environment] Both servers are ready");

        // Run the test function
        println!("[environment] Running test function");
        let result = with_tracing(|| f()).await;
        println!("[environment] Test function completed");

        // Signal the servers to shut down
        println!("[environment] Signaling servers to shut down");
        signals.notify_shutdown();
        println!("[environment] Servers signaled to shut down");

        // Wait for the server to finish shutting down
        println!("[environment] Waiting for server handles to complete");
        let _ = server_handle.await;
        let _ = runtime_handle.await;
        println!("[environment] Server handles completed");

        result
    }
}
