mod mock;

use bzauth_rs::auth::AuthOptions;
use bzauth_rs::runtimes::axum::AxumRuntimeOptions;
use mock::{JsonStore, JsonStoreTypes, MockAdaptor, MockProvider};
use tempfile::NamedTempFile;

#[tokio::test]
#[cfg_attr(
    not(feature = "test_sequential"),
    ignore = "this test cannot run in parallel"
)]
async fn test_runtime_server() {
    let signals = mock::Signals::new();

    let tmpfile = NamedTempFile::new().expect("Failed to create temp file");
    let path = tmpfile.path();

    let json_store = JsonStore::new(&JsonStoreTypes::File(path));
    let auth_options = AuthOptions::new()
        .add_provider(Box::new(MockProvider))
        .with_adaptor(Box::new(MockAdaptor::new(json_store)));
    let options = AxumRuntimeOptions::new(auth_options);

    // Start the mock auth server
    let server_future = mock::runtime::axum::start(signals.clone(), options);

    // Run the server in a separate task
    let server_handle = tokio::spawn(async move {
        if let Err(e) = server_future.await {
            assert!(false, "Failed to start mock auth server: {}", e);
        }
    });

    // Wait for the server to be ready
    signals.wait_for_ready().await;

    // Signal the server to shut down
    signals.notify_shutdown();

    // Wait for the server to finish shutting down
    let _ = server_handle.await;
}
