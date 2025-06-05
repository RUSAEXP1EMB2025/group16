use crate::{
    core::{
        models::{AdjustLigtingRequest, AtmosFreq},
        ports::{AdjustLigtingError, AdjustLigtingRepository},
    },
    outbound::remo::Remo,
};

#[derive(Clone)]
pub struct Service;

impl AdjustLigtingRepository for Service {
    async fn adjust_lighting(&self, req: &AdjustLigtingRequest) -> Result<(), AdjustLigtingError> {
        let atmosfreq = AtmosFreq::new(&req.site_info);

        let remo = Remo::new(&req.remo_token);
        let lighting_amount = remo.calc_lighting_amount(atmosfreq);
        remo.apply_lighting(lighting_amount);

        Ok(())
    }
}
