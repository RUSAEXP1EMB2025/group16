#![allow(clippy::too_many_arguments)]

pub mod api;
pub mod client;
pub mod error;
pub use client::Client;

use error::APIError;
use reqwest::Response;
use serde::de::DeserializeOwned;

pub(crate) trait Deserializable {
    async fn deserialize<T: DeserializeOwned>(self) -> Result<T, APIError>;
}

impl Deserializable for serde_json::Value {
    async fn deserialize<T: DeserializeOwned>(self) -> Result<T, APIError> {
        let deserialized_text = serde_json::from_value::<T>(self)?;
        Ok(deserialized_text)
    }
}

impl Deserializable for String {
    async fn deserialize<T: DeserializeOwned>(self) -> Result<T, APIError> {
        let deserialized_text = serde_json::from_str::<T>(&self)?;
        Ok(deserialized_text)
    }
}

impl Deserializable for Response {
    async fn deserialize<T: DeserializeOwned>(self) -> Result<T, APIError> {
        let text = self.text().await?;
        let deserialized_text = serde_json::from_str::<T>(&text)?;
        Ok(deserialized_text)
    }
}
