use remo_api::models::Signal;

use super::{
    models::{
        atmosdict::GetAtmosdictError,
        remo::{
            AdjustLigtingError, AdjustLigtingRequest, GetLightingSignalsError,
            GetLigtingSignalsRequest,
        },
    },
    ports::{AtmosdictRepository, AtmosdictService, RemoRepository, RemoService},
};

#[derive(Clone)]
pub struct Service<LR, KR>
where
    LR: RemoRepository,
    KR: AtmosdictRepository,
{
    lighting_repository: LR,
    keywords_repository: KR,
}

impl<LR, KR> Service<LR, KR>
where
    LR: RemoRepository,
    KR: AtmosdictRepository,
{
    pub fn new(lighting_repository: LR, keywords_repository: KR) -> Self {
        Self {
            lighting_repository,
            keywords_repository,
        }
    }

    pub fn lighting_repository(&self) -> &LR {
        &self.lighting_repository
    }

    pub fn keywords_repository(&self) -> &KR {
        &self.keywords_repository
    }
}

impl<LR, KR> RemoService for Service<LR, KR>
where
    LR: RemoRepository,
    KR: AtmosdictRepository,
{
    async fn get_lighting_signals(
        &self,
        req: &GetLigtingSignalsRequest,
    ) -> Result<Vec<Signal>, GetLightingSignalsError> {
        self.lighting_repository.get_lighting_signals(req).await
    }

    async fn adjust_lighting(&self, req: &AdjustLigtingRequest) -> Result<(), AdjustLigtingError> {
        self.lighting_repository.adjust_lighting(req).await
    }
}

impl<LR, KR> AtmosdictService for Service<LR, KR>
where
    LR: RemoRepository,
    KR: AtmosdictRepository,
{
    async fn get_atmos_dict(&self) -> Result<Vec<String>, GetAtmosdictError> {
        self.keywords_repository.get_atmos_dict().await
    }
}
