use crate::core::models::AtmosFreq;

use remo_api::apis::configuration::Configuration;

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

    pub fn calc_lighting_amount(&self, atmosfreq: AtmosFreq) -> i32 {
        // TODO:  Calculate lighting amount from atmosfreq
        todo!()
    }

    pub fn apply_lighting(&self, lighting_amount: i32) {
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
