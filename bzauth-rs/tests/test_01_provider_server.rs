mod mock;

use std::sync::Arc;

use tokio::sync::Notify;

#[tokio::test]
async fn test_provider_server() {
    let shutdown_receiver = Arc::new(Notify::new());

    // Start the mock provider server
    let server_future = mock::provider_server::axum_::start(shutdown_receiver.clone());

    // Run the server in a separate task
    let server_handle = tokio::spawn(async move {
        if let Err(e) = server_future.await {
            assert!(false, "Failed to start mock provider server: {}", e);
        }
    });

    // Allow some time for the server to start
    tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;

    // Here you would typically run your tests against the mock server

    // Signal the server to shut down
    shutdown_receiver.notify_waiters();

    // Wait for the server to finish shutting down
    let _ = server_handle.await;
}
