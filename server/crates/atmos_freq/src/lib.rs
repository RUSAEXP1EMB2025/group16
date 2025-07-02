//! これはサイト情報から算出される雰囲気指数を取り扱うライブラリである。
pub mod page;

use derive_more::From;
use url::Url;

// 雰囲気指数 (0 ~ 100.0)
#[derive(PartialEq, Debug, From)]
pub struct AtmosFreq(f64);

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum SiteInfo {
    Youtube { url: Url },
    Netflix { title: String },
    General { keywords: Vec<String> },
}

impl AtmosFreq {
    pub async fn new(siteinfo: &SiteInfo) -> Self {
        match siteinfo {
            SiteInfo::Youtube { url } => Self::from_youtube(url).await,
            SiteInfo::Netflix { title } => Self::from_netflix(title).await,
            SiteInfo::General { keywords } => Self::from_general(keywords),
        }
    }
}

#[cfg(test)]
mod test {
    use super::AtmosFreq;
    use crate::SiteInfo;

    #[tokio::test]
    async fn test_calc_atmosfreq_from_general() {
        // TODO: テストケースの実装
        let keywords = vec!["a", "b", "c"];
        let expect_result = 0.0;

        let keywords = keywords.into_iter().map(String::from).collect();

        let atmosfreq = AtmosFreq::new(&SiteInfo::General { keywords }).await;
        assert_eq!(atmosfreq, AtmosFreq::from(expect_result))
    }
}
