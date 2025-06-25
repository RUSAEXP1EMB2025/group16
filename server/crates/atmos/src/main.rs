use atmos_dict::Atmosdict;
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

    let remo = Remo;
    let atmos_dict = Atmosdict::new();

    let service = Service::new(remo, atmos_dict);
    let http_server = HttpServer::new(service, server_config).await.unwrap();

    http_server.run().await
}
