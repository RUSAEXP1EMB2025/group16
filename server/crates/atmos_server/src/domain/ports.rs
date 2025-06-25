use remo_api::models::Signal;

use super::models::{
    atmosdict::GetAtmosdictError,
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
pub trait AtmosdictRepository: Send + Sync + Clone + 'static {
    async fn get_atmos_dict(&self) -> Result<Vec<String>, GetAtmosdictError>;
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
pub trait AtmosdictService: Send + Sync + Clone + 'static {
    async fn get_atmos_dict(&self) -> Result<Vec<String>, GetAtmosdictError>;
}
