mod mock;

use std::sync::Arc;

use bzauth_rs::auth::AuthOptions;
use bzauth_rs::runtimes::axum::AxumRuntimeOptions;
use mock::{JsonStore, JsonStoreTypes, MockAdaptor, MockProvider};
use tokio::sync::Notify;

#[tokio::test]
async fn test_runtime_server() {
    let shutdown_receiver = Arc::new(Notify::new());
    let json_store = JsonStore::new(&JsonStoreTypes::Memory);
    let auth_options = AuthOptions::new()
        .add_provider(Box::new(MockProvider))
        .with_adaptor(Box::new(MockAdaptor::new(json_store)));
    let options = AxumRuntimeOptions::new(auth_options);

    // Start the mock auth server
    let server_future = mock::runtime::axum::start(shutdown_receiver.clone(), options);

    // Run the server in a separate task
    let server_handle = tokio::spawn(async move {
        if let Err(e) = server_future.await {
            assert!(false, "Failed to start mock auth server: {}", e);
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
