use color_eyre::eyre::{self, Context as _};
use std::{env, path::PathBuf, sync::Once};

static INIT: Once = Once::new();

const DATABASE_PATH: &str = "DATABASE_PATH";
const SERVER_PORT: &str = "SERVER_PORT";
const YOUTUBE_API_KEY: &str = "YOUTUBE_API_KEY";
const NETFLIX_API_KEY: &str = "NETFLIX_API_KEY";
const REMO_TOKEN: &str = "REMO_TOKEN";

#[derive(Debug, Default)]
pub struct Config {
    pub database_path: String,
    pub server_port: String,
    pub youtube_api_key: String,
    pub netflix_api_key: String,
    pub remo_token: Option<String>,
}

impl Config {
    pub fn from_env() -> Config {
        let mut config = Config::default();
        INIT.call_once(|| {
            let database_path = load_env(DATABASE_PATH).unwrap();
            let database_path = PathBuf::from(database_path);

            let mut cargo_manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
            cargo_manifest_dir.pop(); //crates
            cargo_manifest_dir.pop(); //server
            cargo_manifest_dir.pop(); //root

            let database_path = cargo_manifest_dir.join(database_path);
            let database_path = database_path.to_str().unwrap();

            config = Config {
                server_port: load_env(SERVER_PORT).unwrap_or_else(|_| String::from("5152")),
                database_path: database_path.to_string(),
                youtube_api_key: load_env(YOUTUBE_API_KEY).unwrap(),
                netflix_api_key: load_env(NETFLIX_API_KEY).unwrap(),
                remo_token: load_env(REMO_TOKEN).ok(),
            };
        });

        config
    }
}

fn load_env(key: &str) -> eyre::Result<String> {
    env::var(key).with_context(|| format!("failed to load environment variable {}", key))
}
