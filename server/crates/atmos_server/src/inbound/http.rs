pub mod api;
pub mod routes;

use axum::{
    http::{HeaderValue, Method, header},
    routing::{get, post},
};
use color_eyre::eyre::{self, Context as _};
use std::sync::Arc;
use tokio::net;
use tower_http::cors::{AllowOrigin, CorsLayer};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use crate::domain::{
    Service,
    ports::{AtmosdictRepository, AtmosdictService, RemoRepository, RemoService},
};

/// HTTPサーバーの設定
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HttpServerConfig {
    pub port: String,
}

#[derive(Debug, Clone)]
pub struct AppState<S: RemoService + AtmosdictService> {
    pub service: Arc<S>,
}

pub struct HttpServer {
    router: axum::Router,
    listener: net::TcpListener,
    port: String,
}

impl HttpServer {
    pub async fn new<LR, KR>(
        service: Service<LR, KR>,
        config: HttpServerConfig,
    ) -> eyre::Result<Self>
    where
        LR: RemoRepository,
        KR: AtmosdictRepository,
    {
        let trace_layer = tower_http::trace::TraceLayer::new_for_http().make_span_with(
            |request: &axum::extract::Request<_>| {
                let uri = request.uri().to_string();
                tracing::info_span!("http_request", method = ?request.method(), uri)
            },
        );

        let state = AppState {
            service: Arc::new(service),
        };

        let cors = CorsLayer::new()
            .allow_origin(AllowOrigin::any())
            .allow_methods(vec![Method::GET, Method::POST, Method::PUT, Method::DELETE])
            .allow_headers(vec![
                header::ACCEPT,
                header::CONTENT_TYPE,
                header::AUTHORIZATION,
            ]);

        let router = axum::Router::new()
            .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()))
            .route("/lighting", post(routes::adjust_lighting))
            .route("/lighting", get(routes::get_lighting_signals))
            .route("/atmosdict", get(routes::get_atmoswords))
            .layer(trace_layer)
            .layer(cors)
            .with_state(state);

        let listener = net::TcpListener::bind(format!("0.0.0.0:{}", config.port))
            .await
            .with_context(|| format!("failed to listen on {}", config.port))?;

        Ok(Self {
            router,
            listener,
            port: config.port,
        })
    }

    pub async fn run(self) -> eyre::Result<()> {
        tracing::debug!("listening on {}", self.listener.local_addr().unwrap());
        tracing::debug!(
            "you can see swagger here: http://localhost:{}/swagger-ui",
            self.port
        );
        axum::serve(self.listener, self.router)
            .await
            .context("received error from running server")?;

        Ok(())
    }
}

#[derive(OpenApi)]
#[openapi(paths(
    routes::adjust_lighting::adjust_lighting,
    routes::get_lighting_signals::get_lighting_signals,
    routes::get_atmoswords::get_atmoswords
))]
struct ApiDoc;
