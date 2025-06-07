use derive_more::From;
use ts_rs::TS;

use super::AtmosFreq;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, TS)]
#[ts(export)]
pub struct AdjustLigtingRequest {
    pub remo_token: String,
    pub url: Url,
    pub texts: Vec<String>,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, TS)]
#[ts(export)]
pub struct GetLigtingSignalsRequest {
    pub remo_token: String,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, TS)]
pub struct Url(String);

impl From<String> for Url {
    fn from(url: String) -> Self {
        Url(url)
    }
}

impl From<&str> for Url {
    fn from(url: &str) -> Self {
        Url(url.to_owned())
    }
}

/// 現在の部屋の明るさ
#[derive(From)]
pub struct CurrentLightingAmount(i32);

/// 明るさの目標値
#[derive(From)]
pub struct TargetLightingAmount(i32);

impl TargetLightingAmount {
    /// 雰囲気指数から部屋の明るさを考慮して目標の明るさを算出する
    ///
    /// * `atmosfreq`: 雰囲気指数
    /// * `current_lighting_amount`: 現在の部屋の明るさ
    pub fn new(
        atmosfreq: impl Into<AtmosFreq>,
        current_lighting_amount: CurrentLightingAmount,
    ) -> Self {
        let atmosfreq: AtmosFreq = atmosfreq.into();

        // TODO:  雰囲気指数と現在の明るさから，新しい明るさの目標値を計算する。
        let amount = todo!();
        CurrentLightingAmount(amount);
    }
}
