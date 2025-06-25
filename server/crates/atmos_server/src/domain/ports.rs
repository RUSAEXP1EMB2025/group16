use remo_api::models::Signal;

use super::models::{
    keywords::{GetKeywordsError, GetKeywordsRequest},
    remo::{
        AdjustLigtingError, AdjustLigtingRequest, GetLightingSignalsError, GetLigtingSignalsRequest,
    },
};

#[trait_variant::make(Send)]
pub trait RemoRepository: Send + Sync + Clone + 'static {
    async fn get_lighting_signals(
        &self,
        req: &GetLigtingSignalsRequest,
    ) -> Result<Vec<Signal>, GetLightingSignalsError>;

    async fn adjust_lighting(&self, req: &AdjustLigtingRequest) -> Result<(), AdjustLigtingError>;
}

#[trait_variant::make(Send)]
pub trait KeywordsRepository: Send + Sync + Clone + 'static {
    async fn get_keywords(&self, req: &GetKeywordsRequest)
    -> Result<Vec<String>, GetKeywordsError>;
}

#[trait_variant::make(Send)]
pub trait RemoService {
    async fn get_lighting_signals(
        &self,
        req: &GetLigtingSignalsRequest,
    ) -> Result<Vec<Signal>, GetLightingSignalsError>;

    async fn adjust_lighting(&self, req: &AdjustLigtingRequest) -> Result<(), AdjustLigtingError>;
}

#[trait_variant::make(Send)]
pub trait KeywordsService: Send + Sync + Clone + 'static {
    async fn get_keywords(&self, req: &GetKeywordsRequest)
    -> Result<Vec<String>, GetKeywordsError>;
}
