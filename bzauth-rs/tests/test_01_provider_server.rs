mod mock;

#[tokio::test]
#[cfg_attr(
    not(feature = "test_sequential"),
    ignore = "this test cannot run in parallel"
)]
async fn test_provider_server() {
    let signals = mock::Signals::new();

    // Start the mock provider server
    let server_future = mock::provider_server::axum_::start(signals.clone());

    // Run the server in a separate task
    let server_handle = tokio::spawn(async move {
        if let Err(e) = server_future.await {
            assert!(false, "Failed to start mock provider server: {}", e);
        }
    });

    // Wait for the server to be ready
    signals.wait_for_ready().await;

    // Signal the server to shut down
    signals.notify_shutdown();

    // Wait for the server to finish shutting down
    let _ = server_handle.await;
}
