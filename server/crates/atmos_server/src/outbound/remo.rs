use std::sync::Arc;

use crate::domain::{
    models::remo::{
        AdjustLigtingError, AdjustLigtingRequest, CurrentLightingAmount, GetLightingSignalsError,
        GetLigtingSignalsRequest, TargetLightingAmount,
    },
    ports::RemoRepository,
};

use atmos_dict::Atmosdict;
use atmos_freq::AtmosFreq;
use color_eyre::eyre::{self, ContextCompat};
use remo_api::{
    apis::{configuration::Configuration, default_api::{call_1_appliances_applianceid_signals_get, call_1_appliances_get, call_1_devices_get}},
    models::{ApplianceResponseLightButtonsInner, Signal},
};

#[derive(Clone)]
pub struct Remo {
    atmosdict: Arc<Atmosdict>,
}

impl Remo {
    pub fn new(atmosdict: Arc<Atmosdict>) -> Self {
        Remo { atmosdict }
    }

    fn config(token: &str) -> Configuration {
        Configuration {
            oauth_access_token: Some(token.to_owned()),

            ..Default::default()
        }
    }

    /// 電気のみの信号達を取得する
    pub async fn get_lighting_signals(&self, token: &str) -> eyre::Result<Vec<ApplianceResponseLightButtonsInner>> {
        let appliances = call_1_appliances_get(&Self::config(token)).await.unwrap();
        let light_appliance = appliances.iter().find(|appliance|appliance.light.is_some()).unwrap();
        let buttons = light_appliance.light.clone().unwrap().buttons.unwrap();
        Ok(buttons)
    }

    /// Remoから現在の部屋の明るさを取得
    pub async fn get_lighting_amount(&self, token: &str) -> eyre::Result<CurrentLightingAmount> {
        let devices = call_1_devices_get(&Self::config(token)).await?;
        let device = devices.first().context("Device not found")?;
        let events = device
            .newest_events
            .as_ref()
            .context("Event not found in device")?;

        let il = events
            .get("il")
            .context("illumination not found in event")?;
        let lighting_lighting_amount = il.val.context("Value not found in illumination")?;
        Ok(CurrentLightingAmount::from(lighting_lighting_amount))
    }

    /// 目標の明るさまで明るさを調整する
    ///
    /// * `lighting_amount`: 明るさの数値
    pub async fn apply_lighting(
        &self,
        token: &str,
        target_lighting_amount: TargetLightingAmount,
    ) -> eyre::Result<()> {
        // TODO: NatureRemoのAPIを利用して，目標の明るさまで調整する

        todo!()
    }
}

impl RemoRepository for Remo {
    async fn adjust_lighting(&self, req: &AdjustLigtingRequest) -> Result<(), AdjustLigtingError> {
        let current_lighting_amount = self
            .get_lighting_amount(&req.remo_token)
            .await
            .map_err(AdjustLigtingError::GetLightingAmount)?;

        let atmosfreq = AtmosFreq::new(&req.site_info, Arc::clone(&self.atmosdict)).await;
        let target_lighting_amount = TargetLightingAmount::new(atmosfreq, current_lighting_amount);

        self.apply_lighting(&req.remo_token, target_lighting_amount)
            .await
            .map_err(AdjustLigtingError::ApplyLighting)?;

        Ok(())
    }

    async fn get_lighting_signals(
        &self,
        req: &GetLigtingSignalsRequest,
    ) -> Result<Vec<Signal>, GetLightingSignalsError> {
        let ligitng_signals = self
            .get_lighting_signals(&req.remo_token)
            .await
            .map_err(GetLightingSignalsError::GetLightingSignals)?;
        Ok(ligitng_signals)
    }
}

#[cfg(test)]
mod test {
    use atmos_config::Config;
    use atmos_dict::Atmosdict;
    use std::sync::Arc;

    use super::Remo;
    use crate::domain::models::remo::TargetLightingAmount;

    async fn atmosdict() -> Arc<Atmosdict> {
        let database_path = Config::from_env().database_path;
        let atmosdict = Atmosdict::new(&database_path).await.unwrap();
        Arc::new(atmosdict)
    }

    #[tokio::test]
    async fn test_get_lighting_signals() {
        let remo_token = Config::from_env().remo_token.unwrap();
        let remo = Remo {
            atmosdict: atmosdict().await,
        };
        assert!(remo.get_lighting_signals(&remo_token).await.is_ok());
    }

    #[tokio::test]
    async fn test_get_lighting_amount() {
        let remo_token = Config::from_env().remo_token.unwrap();
        let remo = Remo {
            atmosdict: atmosdict().await,
        };
        let amount = remo.get_lighting_amount(&remo_token).await;
        dbg!(&amount);
        assert!(amount.is_ok());
    }

    #[tokio::test]
    async fn test_apply_lighting() {
        let remo_token = Config::from_env().remo_token.unwrap();
        let remo = Remo {
            atmosdict: atmosdict().await,
        };
        // TODO: 目標の明るさ値を調整する
        let target_lighting_amount = TargetLightingAmount::from(2.0);
        assert!(
            remo.apply_lighting(&remo_token, target_lighting_amount)
                .await
                .is_ok()
        )
    }
}
