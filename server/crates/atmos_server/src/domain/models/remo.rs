use atmos_freq::{AtmosFreq, SiteInfo};
use color_eyre::eyre;
use derive_more::From;
use remo_api::models::{ApplianceResponseSignalsInner, Signal};
use serde::Serialize;
use utoipa::ToSchema;

use crate::inbound::http::api::ApiError;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct AdjustLigtingRequest {
    pub remo_token: String,
    pub site_info: SiteInfo,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct GetLigtingSignalsRequest {
    pub remo_token: String,
}

#[derive(Debug, thiserror::Error)]
pub enum GetLightingSignalsError {
    #[error("Failed to get lighting signals")]
    GetLightingSignals(eyre::Report),
}

impl From<GetLightingSignalsError> for ApiError {
    fn from(e: GetLightingSignalsError) -> Self {
        ApiError::InternalServer(e.to_string())
    }
}

#[derive(Debug, thiserror::Error)]
pub enum AdjustLigtingError {
    #[error("Failed to get lighting amount: {0}")]
    GetLightingAmount(eyre::Report),
    #[error("Failed to apply lighing to the device: {0}")]
    ApplyLighting(eyre::Report),
}

impl From<AdjustLigtingError> for ApiError {
    fn from(e: AdjustLigtingError) -> Self {
        ApiError::InternalServer(e.to_string())
    }
}

#[derive(Debug, Clone, PartialEq, Default, Serialize)]
pub struct LightingSignals {
    pub on: Signal,
    pub off: Signal,
    pub up: Signal,
    pub down: Signal,
}

impl TryFrom<Vec<Signal>> for LightingSignals {
    type Error = eyre::Report;

    fn try_from(signals: Vec<Signal>) -> Result<Self, Self::Error> {
        let find_signal = |name: &str| -> Result<Signal, eyre::Report> {
            signals
                .iter()
                .find(|s| s.name.as_deref() == Some(name))
                .cloned()
                .ok_or_else(|| eyre::eyre!("Signal '{}' not found", name))
        };

        Ok(LightingSignals {
            on: find_signal("on")?,
            off: find_signal("off")?,
            up: find_signal("up")?,
            down: find_signal("down")?,
        })
    }
}

pub struct SendLightingSignalRequest {
    pub amount: i32,
    pub signals: Signal,
}

impl SendLightingSignalRequest {
    pub fn new(
        current_lighting_amount: f32,
        atmosfreq: &AtmosFreq,
        lighting_signals: &LightingSignals,
    ) -> Self {
        // TODO: create [SendLightingSignalRequest]
        todo!()
    }
}

#[cfg(test)]
mod test {
    use atmos_freq::AtmosFreq;

    use crate::domain::models::remo::{LightingSignals, SendLightingSignalRequest};

    #[test]
    fn test_calc_target_lighting_amount() {
        // TODO: 雰囲気指数の値を調整する
        let atmosfreq = AtmosFreq::from(50.0);
        // TODO: 現在の明るさ値を調整する
        let current_lighting_amount = 50.0;

        let lighting_signals = LightingSignals::default();

        let send_lighting_signal_request =
            SendLightingSignalRequest::new(current_lighting_amount, &atmosfreq, &lighting_signals);

        // TODO: 結果予想の値を調整する
        assert_eq!(send_lighting_signal_request.amount, 0);
    }
}
