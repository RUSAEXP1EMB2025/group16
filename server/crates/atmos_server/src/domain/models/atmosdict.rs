use atmos_dict::error::AtmosdictError;

use crate::inbound::http::api::ApiError;

#[derive(Debug, thiserror::Error)]
pub enum GetAtmosdictError {
    #[error("Failed to get wordlist: {0}")]
    GetWordlist(AtmosdictError),
}

impl From<GetAtmosdictError> for ApiError {
    fn from(e: GetAtmosdictError) -> Self {
        ApiError::InternalServer(e.to_string())
    }
}
