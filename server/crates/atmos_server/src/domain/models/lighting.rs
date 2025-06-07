use ts_rs::TS;

use super::AtmosFreq;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, TS)]
#[ts(export)]
pub struct AdjustLigtingRequest {
    pub remo_token: String,
    pub url: Url,
    pub site_info: SiteInfo,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, TS)]
#[ts(export)]
pub struct GetLigtingSignalsRequest {
    pub remo_token: String,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, TS)]
pub enum SiteInfo {
    Youtube { tags: Vec<String> },
    Netflix { title: String },
    Others { text: String },
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

impl SiteInfo {
    pub fn new(url: Url, html: String) -> Self {
        // TODO: Get SiteKind from string
        SiteInfo::Others {
            text: String::from(""),
        }
    }
}

pub struct TargetLightingAmount(i32);

impl From<AtmosFreq> for TargetLightingAmount {
    /// 雰囲気指数から部屋の明るさを考慮して目標の明るさを算出する
    ///
    /// * `atmosfreq`: 雰囲気指数[AtmosFreq]
    fn from(atmosfreq: AtmosFreq) -> Self {
        // TODO: Calculate target lighting amount
        let amount = todo!();
        TargetLightingAmount(amount);
    }
}
