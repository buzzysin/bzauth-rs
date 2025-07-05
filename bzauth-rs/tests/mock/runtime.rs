pub const MOCK_AUTH_HOST: &str = "localhost";
pub const MOCK_AUTH_PORT: u16 = 8080;
pub const MOCK_AUTH_URL: &str = "http://localhost:8080";

pub mod axum {

    use axum::Extension;
    use axum::routing::Router;
    use bzauth_rs::runtimes::axum::{AxumRuntime, AxumRuntimeOptions};

    use super::{MOCK_AUTH_HOST, MOCK_AUTH_PORT};
    use crate::mock::Signals;

    pub async fn start(
        signals: Signals,
        options: AxumRuntimeOptions,
    ) -> Result<(), std::io::Error> {
        let AxumRuntime { routes, auth } = AxumRuntime::from_options(options);

        let app = Router::new()
        .route("/", axum::routing::get(|| async { "Mock Auth Server: Welcome Home 🏠" }))
            .route("/health", axum::routing::get(|| async { "OK" }))
            // Add the auth routes
        .merge(routes).layer(Extension(auth));
        let addr = (MOCK_AUTH_HOST, MOCK_AUTH_PORT);
        let listener = tokio::net::TcpListener::bind(addr).await?;

        // Notify that the server is ready
        signals.notify_ready();

        axum::serve(listener, app.into_make_service())
            .with_graceful_shutdown(async move {
                // Wait for the shutdown signal
                signals.wait_for_shutdown().await;
                println!("Shutting down mock auth runtime...");
            })
            .await
    }
}
