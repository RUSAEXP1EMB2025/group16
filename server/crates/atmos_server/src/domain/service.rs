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
pub struct Service<RR, KR>
where
    RR: RemoRepository,
    KR: AtmosdictRepository,
{
    remo_repository: RR,
    atmosdict_repository: KR,
}

impl<RR, KR> Service<RR, KR>
where
    RR: RemoRepository,
    KR: AtmosdictRepository,
{
    pub fn new(remo_repository: RR, atmosdict_repository: KR) -> Self {
        Self {
            remo_repository,
            atmosdict_repository,
        }
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
        self.remo_repository.get_lighting_signals(req).await
    }

    async fn adjust_lighting(&self, req: &AdjustLigtingRequest) -> Result<(), AdjustLigtingError> {
        self.remo_repository.adjust_lighting(req).await
    }
}

impl<LR, KR> AtmosdictService for Service<LR, KR>
where
    LR: RemoRepository,
    KR: AtmosdictRepository,
{
    async fn get_atmos_dict(&self) -> Result<Vec<String>, GetAtmosdictError> {
        self.atmosdict_repository.get_atmos_dict().await
    }
}
