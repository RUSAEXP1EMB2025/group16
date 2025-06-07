use remo_api::models::Signal;

use crate::{
    domain::models::lighting::{AdjustLigtingRequest, GetLigtingSignalsRequest},
    inbound::http::api::ApiError,
};

#[trait_variant::make(Send)]
pub trait LigtingRepository: Send + Sync + Clone + 'static {
    async fn get_signals(
        &self,
        req: &GetLigtingSignalsRequest,
    ) -> Result<Vec<Signal>, GetLightingSignalsError>;

    async fn adjust(&self, req: &AdjustLigtingRequest) -> Result<(), AdjustLigtingError>;
}

#[derive(Debug, thiserror::Error)]
pub enum GetLightingSignalsError {}

impl From<GetLightingSignalsError> for ApiError {
    fn from(e: GetLightingSignalsError) -> Self {
        ApiError::InternalServerError(e.to_string())
    }
}

#[derive(Debug, thiserror::Error)]
pub enum AdjustLigtingError {}

impl From<AdjustLigtingError> for ApiError {
    fn from(e: AdjustLigtingError) -> Self {
        ApiError::InternalServerError(e.to_string())
    }
}
