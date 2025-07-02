use std::sync::Arc;

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
    let config = atmos_config::Config::from_env();

    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer().pretty())
        .init();

    let server_config = HttpServerConfig {
        port: config.server_port,
    };

    let atmosdict = Atmosdict::new(&config.database_path).await?;
    let atmosdict = Arc::new(atmosdict);
    let remo = Remo::new(Arc::clone(&atmosdict));

    let service = Service::new(remo, atmosdict);
    let http_server = HttpServer::new(service, server_config).await.unwrap();

    http_server.run().await
}
