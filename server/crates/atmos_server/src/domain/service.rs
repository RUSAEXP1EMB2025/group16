use std::collections::HashSet;

use atmos_dict::error::AtmosdictError;
use remo_api::models::Signal;

use super::{
    models::remo::{
        AdjustLigtingError, AdjustLigtingRequest, GetLightingSignalsError,
        GetLigtingSignalsRequest, LightingSignals,
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
    ) -> Result<LightingSignals, GetLightingSignalsError> {
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
    async fn get_all_atmoswords(&self) -> Result<HashSet<String>, AtmosdictError> {
        self.atmosdict_repository.get_all_atmoswords().await
    }

    async fn get_pos_neg_atmoswords(
        &self,
    ) -> Result<(HashSet<String>, HashSet<String>), AtmosdictError> {
        self.atmosdict_repository.get_pos_neg_atmoswords().await
    }
}
