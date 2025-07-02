use std::collections::HashSet;

use atmos_dict::error::AtmosdictError;
use remo_api::models::Signal;

use super::models::remo::{
    AdjustLigtingError, AdjustLigtingRequest, GetLightingSignalsError, GetLigtingSignalsRequest,
    LightingSignals,
};

#[trait_variant::make(Send)]
pub trait RemoRepository: Send + Sync + Clone + 'static {
    async fn get_lighting_signals(
        &self,
        req: &GetLigtingSignalsRequest,
    ) -> Result<LightingSignals, GetLightingSignalsError>;

    async fn adjust_lighting(&self, req: &AdjustLigtingRequest) -> Result<(), AdjustLigtingError>;
}

#[trait_variant::make(Send)]
pub trait AtmosdictRepository: Send + Sync + Clone + 'static {
    async fn get_all_atmoswords(&self) -> Result<HashSet<String>, AtmosdictError>;
    async fn get_pos_neg_atmoswords(
        &self,
    ) -> Result<(HashSet<String>, HashSet<String>), AtmosdictError>;
}

#[trait_variant::make(Send)]
pub trait RemoService {
    async fn get_lighting_signals(
        &self,
        req: &GetLigtingSignalsRequest,
    ) -> Result<LightingSignals, GetLightingSignalsError>;

    async fn adjust_lighting(&self, req: &AdjustLigtingRequest) -> Result<(), AdjustLigtingError>;
}

#[trait_variant::make(Send)]
pub trait AtmosdictService: Send + Sync + Clone + 'static {
    async fn get_all_atmoswords(&self) -> Result<HashSet<String>, AtmosdictError>;
    async fn get_pos_neg_atmoswords(
        &self,
    ) -> Result<(HashSet<String>, HashSet<String>), AtmosdictError>;
}
