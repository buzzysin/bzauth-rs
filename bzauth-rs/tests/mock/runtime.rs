pub const MOCK_AUTH_HOST: &str = "localhost";
pub const MOCK_AUTH_PORT: u16 = 8080;
pub const MOCK_AUTH_URL: &str = "http://localhost:8080";

pub mod axum {
    use std::sync::Arc;

    use axum::Extension;
    use axum::routing::Router;
    use bzauth_rs::runtimes::axum::{AxumRuntime, AxumRuntimeOptions};
    use tokio::sync::Notify;

    use super::{MOCK_AUTH_HOST, MOCK_AUTH_PORT};

    pub async fn start(
        shutdown_receiver: Arc<Notify>,
        options: AxumRuntimeOptions,
    ) -> Result<(), std::io::Error> {
        let AxumRuntime { routes, auth } = AxumRuntime::from_options(options);

        let app = Router::new().merge(routes).layer(Extension(auth));
        let addr = (MOCK_AUTH_HOST, MOCK_AUTH_PORT);
        let listener = tokio::net::TcpListener::bind(addr).await?;

        axum::serve(listener, app.into_make_service())
            .with_graceful_shutdown(async move {
                shutdown_receiver.notified().await;
            })
            .await
    }
}
