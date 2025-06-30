use color_eyre::eyre::{self, Context as _};
use std::env;

const DATABASE_URL_KEY: &str = "DATABASE_URL";
const SERVER_PORT_KEY: &str = "SERVER_PORT";

pub struct Config {
    pub database_url: String,
    pub server_port: String,
}

impl Config {
    pub fn from_env() -> eyre::Result<Self> {
        let server_port = load_env(SERVER_PORT_KEY).unwrap_or_else(|_| String::from("5152"));
        let database_url = load_env(DATABASE_URL_KEY)?;

        Ok(Config {
            server_port,
            database_url,
        })
    }
}

fn load_env(key: &str) -> eyre::Result<String> {
    env::var(key).with_context(|| format!("failed to load environment variable {}", key))
}
