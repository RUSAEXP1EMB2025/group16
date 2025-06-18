use super::Service;
use crate::{
    domain::{
        models::lighting::{AdjustLigtingRequest, GetLigtingSignalsRequest, TargetLightingAmount},
        ports::lighting::{AdjustLigtingError, GetLightingSignalsError, LigtingRepository},
    },
    outbound::remo::Remo,
};

use atmos_freq::AtmosFreq;
use remo_api::models::Signal;

impl LigtingRepository for Service {
    async fn adjust(&self, req: &AdjustLigtingRequest) -> Result<(), AdjustLigtingError> {
        let remo = Remo::new(&req.remo_token);
        let current_lighting_amount = remo.get_lighting_amount().await.unwrap();
        let atmosfreq = AtmosFreq::new(&req.url, &req.texts);

        let target_lighting_amount = TargetLightingAmount::new(atmosfreq, current_lighting_amount);

        remo.apply_lighting(target_lighting_amount).await.unwrap();

        Ok(())
    }

    async fn get_signals(
        &self,
        req: &GetLigtingSignalsRequest,
    ) -> Result<Vec<Signal>, GetLightingSignalsError> {
        let remo = Remo::new(&req.remo_token);
        let ligitng_signals = remo.get_lighting_signals().await.unwrap();
        Ok(ligitng_signals)
    }
}
