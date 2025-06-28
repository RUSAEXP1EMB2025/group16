use crate::domain::{
    models::remo::{
        AdjustLigtingError, AdjustLigtingRequest, CurrentLightingAmount, GetLightingSignalsError,
        GetLigtingSignalsRequest, TargetLightingAmount,
    },
    ports::RemoRepository,
};

use atmos_freq::AtmosFreq;
use color_eyre::eyre;
use remo_api::{
    apis::{configuration::Configuration, default_api::call_1_devices_get},
    models::Signal,
};

#[derive(Clone)]
pub struct Remo;

impl Remo {
    fn config(token: &str) -> Configuration {
        Configuration {
            oauth_access_token: Some(token.to_owned()),

            ..Default::default()
        }
    }

    /// 電気のみの信号達を取得する
    pub async fn get_lighting_signals(&self, token: &str) -> eyre::Result<Vec<Signal>> {
        // TODO: NatureRemoのAPIを使用して，電気のみの信号達を取得する
        todo!()
    }

    /// Remoから現在の部屋の明るさを取得
    pub async fn get_lighting_amount(&self, token: &str) -> eyre::Result<CurrentLightingAmount> {
        let devices = call_1_devices_get(&Self::config(token)).await?;
        let device = devices.first().unwrap();
        let events = device.newest_events.as_ref().unwrap();
        let il = events.get("il").unwrap();
        let lighting_lighting_amount = il.val.unwrap();
        Ok(CurrentLightingAmount::from(lighting_lighting_amount))
    }

    /// 目標の明るさまで明るさを調整する
    ///
    /// * `lighting_amount`: 明るさの数値
    pub async fn apply_lighting(
        &self,
        token: &str,
        target_lighting_amount: TargetLightingAmount,
    ) -> Result<(), AdjustLigtingError> {
        // TODO: NatureRemoのAPIを利用して，目標の明るさまで調整する

        todo!()
    }
}

impl RemoRepository for Remo {
    async fn adjust_lighting(&self, req: &AdjustLigtingRequest) -> Result<(), AdjustLigtingError> {
        let current_lighting_amount = self.get_lighting_amount(&req.remo_token).await.unwrap();
        let atmosfreq = AtmosFreq::new(&req.url, &req.texts).await;

        let target_lighting_amount = TargetLightingAmount::new(atmosfreq, current_lighting_amount);

        self.apply_lighting(&req.remo_token, target_lighting_amount)
            .await
            .unwrap();

        Ok(())
    }

    async fn get_lighting_signals(
        &self,
        req: &GetLigtingSignalsRequest,
    ) -> Result<Vec<Signal>, GetLightingSignalsError> {
        let remo = Remo;
        let ligitng_signals = remo.get_lighting_signals(&req.remo_token).await.unwrap();
        Ok(ligitng_signals)
    }
}

#[cfg(test)]
mod test {
    use super::Remo;
    use crate::domain::models::remo::TargetLightingAmount;

    use std::env;

    #[tokio::test]
    async fn test_get_lighting_signals() {
        dotenvy::dotenv().unwrap();
        let token = env::var("REMO_TOKEN").expect("TOKEN Not found");
        let remo = Remo;
        assert!(remo.get_lighting_signals(&token).await.is_ok());
    }

    #[tokio::test]
    async fn test_get_lighting_amount() {
        dotenvy::dotenv().unwrap();
        let token = env::var("REMO_TOKEN").expect("TOKEN Not found");
        let remo = Remo;
        let amount = remo.get_lighting_amount(&token).await;
        dbg!(&amount);
        assert!(amount.is_ok());
    }

    #[tokio::test]
    async fn test_apply_lighting() {
        dotenvy::dotenv().unwrap();
        let token = env::var("REMO_TOKEN").expect("TOKEN Not found");
        let remo = Remo;
        // TODO: 目標の明るさ値を調整する
        let target_lighting_amount = TargetLightingAmount::from(2.0);
        assert!(
            remo.apply_lighting(&token, target_lighting_amount)
                .await
                .is_ok()
        )
    }
}
