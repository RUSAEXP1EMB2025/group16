use crate::domain::{models::lighting::TargetLightingAmount, ports::lighting::AdjustLigtingError};

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
        // TODO:  Get lighting signals
        todo!()
    }

    /// Remoから現在の部屋の明るさを取得
    pub fn get_lighting_amount(&self) -> eyre::Result<i32> {
        // TODO: Get current lighting amount in the room
        todo!()
    }

    /// 目標の明るさまで明るさを調整する
    ///
    /// * `lighting_amount`: 明るさの数値
    pub fn apply_lighting(
        &self,
        target_lighting_amount: TargetLightingAmount,
    ) -> Result<(), AdjustLigtingError> {
        // TODO: Apply lighting
        todo!()
    }
}

// mod test {
//     use super::Remo;
//     use crate::core::models::AdjustLigtingRequest;
//
//     use std::env;
//
//     fn test_adjust_lighting() {
//         let token = env::var("REMO_TOKEN").expect("TOKEN Not found");
//         let mock_req = AdjustLigtingRequest {
//             remo_token: token,
//             url: String::from(""),
//             text: String::from(""),
//         };
//
//         let remo = Remo::new();
//     }
// }
