use thiserror::Error;

pub type APIResult<T> = Result<T, APIError>;

#[derive(Error, Debug)]
pub enum APIError {
    #[error("Error: {}", error)]
    Error { error: String },

    #[error("Authorization failed")]
    AuthError,

    #[error("Client error: {}", error)]
    HTTPError { error: reqwest::Error },

    #[error("Deserialize failed: {}", error)]
    DeserializeError { error: serde_json::Error },

    #[error("Serialize failed")]
    SerializeError,
}

impl From<reqwest::Error> for APIError {
    fn from(error: reqwest::Error) -> Self {
        APIError::HTTPError { error }
    }
}

impl From<serde_json::Error> for APIError {
    fn from(error: serde_json::Error) -> Self {
        APIError::DeserializeError { error }
    }
}

impl<E> From<remo_api::apis::Error<E>> for APIError {
    fn from(value: remo_api::apis::Error<E>) -> Self {
        APIError::Error {
            error: value.to_string(),
        }
    }
}
