mod mock;

use bzauth_rs::auth::AuthOptions;
use bzauth_rs::runtimes::axum::AxumRuntimeOptions;
use mock::server_runtime::MOCK_AUTH_URL;
use mock::{JsonStore, JsonStoreTypes, MOCK_PROVIDER_NAME, MockAdaptor, MockProvider, requests};
use tempfile::NamedTempFile;

#[tokio::test]
#[cfg_attr(
    not(feature = "test_sequential"),
    ignore = "this test cannot run in parallel"
)]
async fn test_00_environment() {
    let signals = mock::Signals::new();

    let tmpfile = NamedTempFile::new().expect("Failed to create temp file");
    let path = tmpfile.path();

    let json_store = JsonStore::new(&JsonStoreTypes::File(path));
    let auth_options = AuthOptions::new()
        .add_provider(Box::new(MockProvider))
        .with_adaptor(Box::new(MockAdaptor::new(json_store)));
    let options = AxumRuntimeOptions::from_options(auth_options);

    // Start the mock auth server
    mock::environment::axum_::run(signals, options, || async {
        // Here you would typically run your tests against the mock server
        // For example, you could make requests to the server and assert responses
        println!("Mock auth server is running. You can now run your tests against it.");
    })
    .await;
}

#[tokio::test]
#[cfg_attr(
    not(feature = "test_sequential"),
    ignore = "this test cannot run in parallel"
)]
async fn test_01_auth_server_authorize() {
    let signals = mock::Signals::new();

    let tmpfile = NamedTempFile::new().expect("Failed to create temp file");
    let path = tmpfile.path();

    let json_store = JsonStore::new(&JsonStoreTypes::File(path));
    let auth_options = AuthOptions::new()
        .add_provider(Box::new(MockProvider))
        .with_adaptor(Box::new(MockAdaptor::new(json_store)));
    let options = AxumRuntimeOptions::from_options(auth_options);

    // Start the mock auth server
    mock::environment::axum_::run(signals, options, || async {
        // Here you would typically run your tests against the mock server
        // For example, you could make requests to the server and assert responses

        // Fetch the authorization URL
        let response = requests::make_authorisation_request().await;

        assert!(
            response.status().is_success(),
            "Authorization request failed"
        );
    })
    .await;
}

#[tokio::test]
#[cfg_attr(
    not(feature = "test_sequential"),
    ignore = "this test cannot run in parallel"
)]
async fn test_02_auth_server_callback() {
    let signals = mock::Signals::new();

    let tmpfile = NamedTempFile::new().expect("Failed to create temp file");
    let path = tmpfile.path();

    let json_store = JsonStore::new(&JsonStoreTypes::File(path));
    let auth_options = AuthOptions::new()
        .add_provider(Box::new(MockProvider))
        .with_adaptor(Box::new(MockAdaptor::new(json_store.clone())));
    let options = AxumRuntimeOptions::from_options(auth_options);

    // Start the mock auth server
    mock::environment::axum_::run(signals, options, || async {
        // Simulate a callback request
        let response = requests::make_callback_request(MOCK_AUTH_URL, MOCK_PROVIDER_NAME).await;

        let url = response.url().to_string();
        let status = response.status();

        // Debug: print all headers
        println!("Response status: {}", status);
        println!("Response headers: {:?}", response.headers());

        // Check for Set-Cookie headers (clone before consuming response)
        let cookies = response
            .headers()
            .get_all("set-cookie")
            .iter()
            .map(|v| v.to_str().unwrap_or("").to_string())
            .collect::<Vec<_>>();

        println!("Found cookies: {:?}", cookies);

        let body = response.text().await.expect("Failed to read response text");
        println!("Response body: {}", body);

        // Assert that the request returned a redirect (302 or 307)
        assert!(
            status.is_redirection(),
            "Callback should return a redirect:\n\turl: {}\n\tstatus: {}\n\tbody: {}",
            url,
            status,
            body
        ); // Assert that the session cookie was set
        assert!(
            !cookies.is_empty(),
            "No cookies were set in the response: {}",
            cookies.join("; ")
        );

        // Print the cookies for debugging
        println!("Cookies set in response: {}", cookies.join("; "));

        // Print the json store
        let data = json_store
            .get_data()
            .expect("Failed to get data from json store");
        println!(
            "Final JSON store: {}",
            serde_json::to_string(&data).unwrap()
        );
    })
    .await;
}
