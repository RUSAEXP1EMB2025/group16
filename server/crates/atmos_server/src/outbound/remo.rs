use std::{sync::Arc, time::Duration};

use crate::domain::{
    models::remo::{
        AdjustLigtingError, AdjustLigtingRequest, GetLightingSignalsError,
        GetLigtingSignalsRequest, LightingSignals,
    },
    ports::RemoRepository,
};

use atmos_dict::Atmosdict;
use atmos_freq::calc_atmosfreq;
use color_eyre::eyre::{self, ContextCompat};
use remo_api::{
    apis::{
        configuration::Configuration,
        default_api::{
            call_1_appliances_get, call_1_devices_get, call_1_signals_signalid_send_post,
        },
    },
    models::Signal,
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
    async fn get_lighting_signals(&self, token: &str) -> eyre::Result<LightingSignals> {
        let appliances = call_1_appliances_get(&Self::config(token)).await.unwrap();
        let light_appliance = appliances
            .iter()
            .find(|appliance| appliance.nickname == Some(String::from("Light")))
            .unwrap();

        let signals = light_appliance
            .signals
            .clone()
            .unwrap()
            .iter()
            .map(|s| Signal {
                id: s.id.clone(),
                image: s.image.clone(),
                name: s.name.clone(),
            })
            .collect::<Vec<Signal>>();

        let lighting_signal = LightingSignals::try_from(signals).unwrap();
        Ok(lighting_signal)
    }

    /// Remoから現在の部屋の明るさを取得
    async fn get_lighting_amount(&self, token: &str) -> eyre::Result<f32> {
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

        Ok(lighting_lighting_amount)
    }

    /// 目標の明るさまで明るさを調整する
    ///
    /// * `lighting_amount`: 明るさの数値
    async fn apply_lighting(
        &self,
        token: &str,
        atmosfreq: f64,
        _current_lighting_amount: f32,
    ) -> eyre::Result<()> {
        let lighting_signals = self.get_lighting_signals(token).await.unwrap();
        let signals = Remo::create_signals(atmosfreq, &lighting_signals);

        for signal in signals {
            let _ = call_1_signals_signalid_send_post(
                &Remo::config(token),
                signal.id.as_ref().unwrap(),
            )
            .await;
            tokio::time::sleep(Duration::from_secs(1)).await;
        }
        Ok(())
    }

    fn create_signals(atmosfreq: f64, lighting_signals: &LightingSignals) -> Vec<&Signal> {
        match atmosfreq {
            0.0 => vec![&lighting_signals.off],
            0.0..30.0 => vec![
                &lighting_signals.on,
                &lighting_signals.on,
                &lighting_signals.down,
            ],
            30.0..70.0 => vec![
                &lighting_signals.on,
                &lighting_signals.on,
                &lighting_signals.down,
                &lighting_signals.down,
            ],
            70.0..80.0 => vec![
                &lighting_signals.on,
                &lighting_signals.on,
                &lighting_signals.down,
                &lighting_signals.down,
                &lighting_signals.down,
            ],
            100.0 => vec![&lighting_signals.on],
            _ => Vec::new(),
        }
    }
}

impl RemoRepository for Remo {
    async fn adjust_lighting(&self, req: &AdjustLigtingRequest) -> Result<(), AdjustLigtingError> {
        let current_lighting_amount = self
            .get_lighting_amount(&req.remo_token)
            .await
            .map_err(AdjustLigtingError::GetLightingAmount)?;

        let atmosfreq = calc_atmosfreq(&req.site_data, Arc::clone(&self.atmosdict)).await;

        self.apply_lighting(&req.remo_token, atmosfreq, current_lighting_amount)
            .await
            .map_err(AdjustLigtingError::ApplyLighting)?;

        Ok(())
    }

    async fn get_lighting_signals(
        &self,
        req: &GetLigtingSignalsRequest,
    ) -> Result<LightingSignals, GetLightingSignalsError> {
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
        let current_lighting_amount = 2.0;
        let atmosfreq = 0.0;

        assert!(
            remo.apply_lighting(&remo_token, atmosfreq, current_lighting_amount)
                .await
                .is_ok()
        )
    }
}
