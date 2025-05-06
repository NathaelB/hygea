use axum::{
    Router,
    http::{
        HeaderValue, Method,
        header::{ACCEPT, AUTHORIZATION, CONTENT_LENGTH, CONTENT_TYPE, LOCATION},
    },
};
use checkin::router::checkin_routes;
use snafu::ResultExt;
use tower_http::cors::CorsLayer;
use tracing::{info, info_span};
use user::router::user_routes;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

pub mod checkin;
pub mod user;

use crate::application::state::AppState;

use super::openapi::ApiDoc;

pub struct HttpServerConfig {
    port: String,
}

impl HttpServerConfig {
    pub fn new(port: String) -> Self {
        Self { port }
    }
}

pub struct HttpServer {
    router: Router,
    listener: tokio::net::TcpListener,
}

impl HttpServer {
    pub async fn new(
        config: HttpServerConfig,
        app_state: AppState,
    ) -> Result<Self, snafu::Whatever> {
        let trace_layer = tower_http::trace::TraceLayer::new_for_http().make_span_with(
            |request: &axum::extract::Request| {
                let uri: String = request.uri().to_string();
                info_span!("http_request", method = ?request.method(), uri)
            },
        );

        let allowed_origins: Vec<HeaderValue> = vec![
            HeaderValue::from_static("http://localhost:3000"),
            HeaderValue::from_static("http://localhost:5173"),
        ];

        let cors = CorsLayer::new()
            .allow_methods([
                Method::GET,
                Method::POST,
                Method::DELETE,
                Method::PUT,
                Method::PATCH,
                Method::OPTIONS,
            ])
            .allow_origin(allowed_origins)
            .allow_headers([
                AUTHORIZATION,
                CONTENT_TYPE,
                CONTENT_LENGTH,
                ACCEPT,
                LOCATION,
            ])
            .allow_credentials(true);

        let router = Router::new()
            .merge(SwaggerUi::new("/swagger-ui").url("/api-doc/openapi.json", ApiDoc::openapi()))
            .merge(user_routes())
            .merge(checkin_routes())
            .layer(trace_layer)
            .layer(cors)
            .with_state(app_state);

        let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", config.port))
            .await
            .with_whatever_context(|_| format!("Failed to bind to port {}", config.port))?;

        Ok(Self { router, listener })
    }

    pub async fn run(self) -> Result<(), snafu::Whatever> {
        let local_addr = self
            .listener
            .local_addr()
            .with_whatever_context(|_| "Failed to get local address")?;
        info!("Server is running on http://{}", local_addr,);

        axum::serve(self.listener, self.router)
            .await
            .with_whatever_context(|_| format!("Failed to start server on {}", local_addr))?;

        Ok(())
    }
}
