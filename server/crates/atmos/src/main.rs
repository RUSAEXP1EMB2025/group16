use atmos_server::{
    domain::Service,
    inbound::http::{HttpServer, HttpServerConfig},
    outbound::remo::Remo,
};
use color_eyre::eyre;
use tracing_subscriber::{layer::SubscriberExt as _, util::SubscriberInitExt as _};

#[tokio::main]
async fn main() -> eyre::Result<()> {
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer().pretty())
        .init();

    let server_config = HttpServerConfig {
        port: std::env::var("BACKEND_PORT").unwrap_or_else(|_| String::from("5152")),
    };

    // TODO: 実際のRepository実装をここに注入する
    // 現在はダミー実装を使用
    let remo = Remo;

    let service = Service::new(remo, keywords_repository);
    let http_server = HttpServer::new(service, server_config).await.unwrap();

    http_server.run().await
}
