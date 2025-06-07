pub mod api;
pub mod routes;

use crate::domain::ports::lighting::LigtingRepository;

use axum::routing::{get, post};
use color_eyre::eyre::{self, Context as _};
use std::sync::Arc;
use tokio::net;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

/// HTTPサーバーの設定
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HttpServerConfig<'a> {
    pub port: &'a str,
}

#[derive(Debug, Clone)]
pub struct AppState<LR: LigtingRepository> {
    lighting_repository: Arc<LR>,
}

pub struct HttpServer<'a> {
    router: axum::Router,
    listener: net::TcpListener,
    port: &'a str,
}

impl<'a> HttpServer<'a> {
    pub async fn new<LR: LigtingRepository>(
        adjust_lighting_repository: LR,
        config: HttpServerConfig<'a>,
    ) -> eyre::Result<Self> {
        let trace_layer = tower_http::trace::TraceLayer::new_for_http().make_span_with(
            |request: &axum::extract::Request<_>| {
                let uri = request.uri().to_string();
                tracing::info_span!("http_request", method = ?request.method(), uri)
            },
        );

        let state = AppState {
            lighting_repository: Arc::new(adjust_lighting_repository),
        };

        let router = axum::Router::new()
            .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()))
            .route("/lighting", post(routes::adjust_lighting::<LR>))
            .route("/lighting", get(routes::get_lighting_signals::<LR>))
            .layer(trace_layer)
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
    routes::get_lighting_signals::get_lighting_signals
))]
struct ApiDoc;
