use oauth2::{EndpointNotSet, EndpointSet};

use crate::mock::consts::{MOCK_AUTHORISE, MOCK_CALLBACK, MOCK_PROFILE, MOCK_TOKEN};
use crate::mock::provider::{MOCK_PROVIDER_HOST, MOCK_PROVIDER_PORT};
use crate::mock::{MOCK_PROVIDER_CLIENT_ID, MOCK_PROVIDER_CLIENT_SECRET};

pub mod axum_ {

    use axum::Json;
    use axum::response::Html;
    use axum::routing::{Router, get, post};

    use super::*;
    use crate::mock::runtime::MOCK_AUTH_URL;
    use crate::mock::{MOCK_PROVIDER_NAME, Signals};

    async fn authorise() -> Html<String> {
        println!("Mock Authorisation Endpoint Hit");
        format!(
            r#"
            <!DOCTYPE html>
            <html>
                <head>
                    <title>Mock Authorisation</title>
                </head>
                <body>
                    <h1>Mock Authorisation</h1>
                    <p>This is a mock authorisation page.</p>
                </body>
                <script>
                    // Mock JavaScript to simulate authorisation
                    console.log('Mock Authorisation Script Loaded');
                    // Simulate a redirect to the callback URL
                    let code = window.location.search.split('code=')[1];
                    let state = window.location.search.split('state=')[1];
                    window.location.href = '{}?code=' + code + '&state=' + state;
                </script>
            </html>"#,
            format!("{}/{}/{}", MOCK_AUTH_URL, MOCK_CALLBACK, MOCK_PROVIDER_NAME)
        )
        .to_string()
        .into()
    }

    async fn callback() -> String {
        "This is a mock callback".to_string()
    }

    async fn token() -> Json<serde_json::Value> {
        println!("Mock Token Endpoint Hit");
        Json(serde_json::json!({
            "access_token": "mock_access_token",
            "token_type": "Bearer",
            "expires_in": 3600,
            "refresh_token": "mock_refresh_token",
        }))
    }

    async fn userinfo() -> Json<serde_json::Value> {
        println!("Mock Userinfo Endpoint Hit");
        Json(serde_json::json!({
            "sub": "1234567890",
            "name": "John Doe",
            "email": "john.doe@email.com"
        }))
    }

    fn router() -> Router {
        Router::new()
            .route(format!("/{}", MOCK_AUTHORISE).as_str(), get(authorise))
            .route(format!("/{}", MOCK_CALLBACK).as_str(), get(callback))
            .route(format!("/{}", MOCK_TOKEN).as_str(), post(token))
            .route(format!("/{}", MOCK_PROFILE).as_str(), get(userinfo))
    }

    pub async fn start(signals: Signals) -> Result<(), std::io::Error> {
        let app = router();
        let addr = (MOCK_PROVIDER_HOST, MOCK_PROVIDER_PORT);
        let listener = tokio::net::TcpListener::bind(addr).await?;

        // Notify that the server is ready
        signals.notify_ready();

        axum::serve(listener, app.into_make_service())
            .with_graceful_shutdown(async move {
                // Wait for the shutdown signal
                signals.wait_for_shutdown().await;
                println!("Shutting down mock auth server...");
            })
            .await
    }
}

pub type MockOauthClient = oauth2::basic::BasicClient<
    EndpointSet,
    EndpointNotSet,
    EndpointNotSet,
    EndpointNotSet,
    EndpointSet,
>;

pub fn get_client() -> MockOauthClient {
    use oauth2::basic::BasicClient;
    use oauth2::{AuthUrl, ClientId, ClientSecret, TokenUrl};

    let client_id = ClientId::new(MOCK_PROVIDER_CLIENT_ID.to_string());
    let client_secret = ClientSecret::new(MOCK_PROVIDER_CLIENT_SECRET.to_string());
    let auth_url = AuthUrl::new(format!(
        "http://{}:{}/{}",
        MOCK_PROVIDER_HOST, MOCK_PROVIDER_PORT, MOCK_AUTHORISE
    ))
    .unwrap();
    let token_url = TokenUrl::new(format!(
        "http://{}:{}/{}",
        MOCK_PROVIDER_HOST, MOCK_PROVIDER_PORT, MOCK_TOKEN
    ))
    .unwrap();

    BasicClient::new(client_id)
        .set_client_secret(client_secret)
        .set_auth_uri(auth_url)
        .set_token_uri(token_url)
}
