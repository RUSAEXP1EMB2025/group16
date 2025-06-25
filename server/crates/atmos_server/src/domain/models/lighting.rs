use atmos_freq::AtmosFreq;
use derive_more::From;
use url::Url;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct AdjustLigtingRequest {
    pub remo_token: String,
    pub url: Url,
    pub texts: Vec<String>,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct GetLigtingSignalsRequest {
    pub remo_token: String,
}

/// 現在の部屋の明るさ
#[derive(PartialEq, Debug, From)]
pub struct CurrentLightingAmount(f32);

/// 明るさの目標値
#[derive(PartialEq, Debug, From)]
pub struct TargetLightingAmount(f64);

impl TargetLightingAmount {
    /// 雰囲気指数から部屋の明るさを考慮して目標の明るさを算出する
    ///
    /// * `atmosfreq`: 雰囲気指数
    /// * `current_lighting_amount`: 現在の部屋の明るさ
    pub fn new(atmosfreq: AtmosFreq, current_lighting_amount: CurrentLightingAmount) -> Self {
        // TODO:  雰囲気指数と現在の明るさから，新しい明るさの目標値を計算する。
        let amount = todo!();
        CurrentLightingAmount(amount);
    }
}

#[cfg(test)]
mod test {
    use super::{CurrentLightingAmount, TargetLightingAmount};
    use atmos_freq::AtmosFreq;

    #[test]
    fn test_calc_target_lighting_amount() {
        // TODO: 雰囲気指数の値を調整する
        let atmosfreq = AtmosFreq::from(50.0);
        // TODO: 現在の明るさ値を調整する
        let current_lighting_amount = CurrentLightingAmount::from(50.0);

        // TODO: 計算結果の値を調整する
        let expect_result = 0.0;

        let target_lightint_amount = TargetLightingAmount::new(atmosfreq, current_lighting_amount);
        assert_eq!(
            target_lightint_amount,
            TargetLightingAmount::from(expect_result)
        );
    }
}
