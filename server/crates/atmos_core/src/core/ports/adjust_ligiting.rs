use crate::{core::models::adjust_lighting::AdjustLigtingRequest, inbound::http::api::ApiError};

#[trait_variant::make(Send)]
pub trait AdjustLigtingRepository: Send + Sync + Clone + 'static {
    async fn adjust_lighting(&self, req: &AdjustLigtingRequest) -> Result<(), AdjustLigtingError>;
}

#[derive(Debug, thiserror::Error)]
pub enum AdjustLigtingError {
    // #[error("blog with name {name} already exists")]
    // Duplicate { name: AuthorName },
    // #[error(transparent)]
    // Unknown(#[from] eyre::Error),
    // // to be extended as new error scenarios are introduced
}

impl From<AdjustLigtingError> for ApiError {
    fn from(e: AdjustLigtingError) -> Self {
        ApiError::InternalServerError(e.to_string())
    }
}
