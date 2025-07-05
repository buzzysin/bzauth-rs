mod mock;

use std::sync::Arc;

use bzauth_rs::auth::AuthOptions;
use bzauth_rs::runtimes::axum::AxumRuntimeOptions;
use mock::runtime::MOCK_AUTH_URL;
use mock::{JsonStore, JsonStoreTypes, MOCK_PROVIDER_NAME, MockAdaptor, MockProvider};
use tokio::sync::Notify;

#[tokio::test]
async fn test_environment() {
    let shutdown_receiver = Arc::new(Notify::new());
    let json_store = JsonStore::new(&JsonStoreTypes::File("mock_store.json"));
    let auth_options = AuthOptions::new()
        .add_provider(Box::new(MockProvider))
        .with_adaptor(Box::new(MockAdaptor::new(json_store)));
    let options = AxumRuntimeOptions::new(auth_options);

    // Start the mock auth server
    mock::environment::axum::run(shutdown_receiver.clone(), options, || async {
        // Here you would typically run your tests against the mock server
        // For example, you could make requests to the server and assert responses
        println!("Mock auth server is running. You can now run your tests against it.");
    })
    .await;

    // Signal the server to shut down
    shutdown_receiver.notify_waiters();
}

#[tokio::test]
async fn test_auth_server_authorize() {
    let shutdown_receiver = Arc::new(Notify::new());
    let json_store = JsonStore::new(&JsonStoreTypes::File("mock_store.json"));
    let auth_options = AuthOptions::new()
        .add_provider(Box::new(MockProvider))
        .with_adaptor(Box::new(MockAdaptor::new(json_store)));
    let options = AxumRuntimeOptions::new(auth_options);

    // Start the mock auth server
    mock::environment::axum::run(shutdown_receiver.clone(), options, || async {
        // Here you would typically run your tests against the mock server
        // For example, you could make requests to the server and assert responses

        // Fetch the authorization URL
        let client = mock::provider_server::get_client();
        let (url, _) = client
            .authorize_url(oauth2::CsrfToken::new_random)
            .add_scope(oauth2::Scope::new("read".to_string()))
            .url();
        println!("Authorization URL: {}", url);

        // Make the reqwest
        let response = reqwest::get(url.to_string())
            .await
            .expect("Failed to make request to auth server");

        assert!(
            response.status().is_success(),
            "Authorization request failed"
        );
    })
    .await;

    // Signal the server to shut down
    shutdown_receiver.notify_waiters();
}

#[tokio::test]
#[ignore = "This test is not fully complete in its implementation"]
async fn test_auth_server_callback() {
    let shutdown_receiver = Arc::new(Notify::new());
    let json_store = JsonStore::new(&JsonStoreTypes::File("mock_store.json"));
    let auth_options = AuthOptions::new()
        .add_provider(Box::new(MockProvider))
        .with_adaptor(Box::new(MockAdaptor::new(json_store)));
    let options = AxumRuntimeOptions::new(auth_options);

    // Start the mock auth server
    mock::environment::axum::run(shutdown_receiver.clone(), options, || async {
        // Simulate a callback request

        let client = reqwest::Client::new();
        let response = client
            .post(format!("{}/callback/{}", MOCK_AUTH_URL, MOCK_PROVIDER_NAME))
            .send()
            .await
            .expect("Failed to make request to auth server");

        assert!(
            response.status().is_success(),
            "Callback request failed, status: {}, body: {}",
            response.status(),
            response.text().await.expect("Failed to read response text")
        );
    })
    .await;

    // Signal the server to shut down
    shutdown_receiver.notify_waiters();
}
