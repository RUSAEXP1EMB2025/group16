use atmos_freq::SiteData;
use color_eyre::eyre;
use remo_api::models::Signal;
use serde::Serialize;

use crate::inbound::http::api::ApiError;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct AdjustLigtingRequest {
    pub remo_token: String,
    pub site_data: SiteData,
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
                .find(|s| s.name == Some(String::from(name)))
                .cloned()
                .ok_or_else(|| eyre::eyre!("Signal '{}' not found", name))
        };

        Ok(LightingSignals {
            on: find_signal("on")?,
            off: find_signal("off")?,
            up: find_signal("top")?,
            down: find_signal("bottom")?,
        })
    }
}
