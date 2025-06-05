use ts_rs::TS;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, TS)]
#[ts(export)]
pub struct AdjustLigtingRequest {
    pub remo_token: String,
    pub url: Url,
    pub site_info: SiteInfo,
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
