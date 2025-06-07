use crate::domain::{
    models::lighting::{CurrentLightingAmount, TargetLightingAmount},
    ports::lighting::AdjustLigtingError,
};

use color_eyre::eyre;
use remo_api::{apis::configuration::Configuration, models::Signal};

#[derive(Clone)]
pub struct Remo {
    config: Configuration,
}

impl Remo {
    pub fn new(token: &str) -> Self {
        let config = Configuration {
            oauth_access_token: Some(token.to_owned()),

            ..Default::default()
        };

        Remo { config }
    }

    /// 電気のみの信号達を取得する
    pub fn get_lighting_signals(&self) -> eyre::Result<Vec<Signal>> {
        // TODO: NatureRemoのAPIを使用して，電気のみの信号達を取得する
        todo!()
    }

    /// Remoから現在の部屋の明るさを取得
    pub fn get_lighting_amount(&self) -> eyre::Result<CurrentLightingAmount> {
        // TODO: NatureRemoのAPIを使用して，現在の部屋の明るさを取得する
        todo!()
    }

    /// 目標の明るさまで明るさを調整する
    ///
    /// * `lighting_amount`: 明るさの数値
    pub fn apply_lighting(
        &self,
        target_lighting_amount: TargetLightingAmount,
    ) -> Result<(), AdjustLigtingError> {
        // TODO: NatureRemoのAPIを利用して，目標の明るさまで調整する
        todo!()
    }
}

#[cfg(test)]
mod test {
    use super::Remo;
    use crate::domain::models::lighting::TargetLightingAmount;

    use std::env;

    #[test]
    fn test_get_lighting_signals() {
        let token = env::var("REMO_TOKEN").expect("TOKEN Not found");
        let remo = Remo::new(&token);
        assert!(remo.get_lighting_signals().is_ok());
    }

    #[test]
    fn test_get_lighting_amount() {
        let token = env::var("REMO_TOKEN").expect("TOKEN Not found");
        let remo = Remo::new(&token);
        assert!(remo.get_lighting_amount().is_ok())
    }

    #[test]
    fn test_apply_lighting() {
        let token = env::var("REMO_TOKEN").expect("TOKEN Not found");
        let remo = Remo::new(&token);
        let target_lighting_amount = TargetLightingAmount::from(2);
        assert!(remo.apply_lighting(target_lighting_amount).is_ok())
    }
}
