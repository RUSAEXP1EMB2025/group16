//! これはサイト情報から算出される雰囲気指数を取り扱うライブラリである。
pub mod page;

use atmos_dict::Atmosdict;
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
    // &AtmosdictとArc<Atmosdict>の両方を受け取れるように、AsRefで取得する
    pub async fn new<A>(siteinfo: &SiteInfo, atmosdict: A) -> Self
    where
        A: AsRef<Atmosdict>,
    {
        let atmosdict = atmosdict.as_ref();

        match siteinfo {
            SiteInfo::Youtube { url } => Self::from_youtube(url, atmosdict).await,
            SiteInfo::Netflix { title } => Self::from_netflix(title).await,
            SiteInfo::General { keywords } => Self::from_general(keywords, atmosdict).await,
        }
    }
}

#[cfg(test)]
mod test {
    use atmos_config::Config;
    use atmos_dict::Atmosdict;

    use super::AtmosFreq;
    use crate::SiteInfo;

    #[tokio::test]
    async fn test_calc_atmosfreq_from_general() {
        // TODO: テストケースの実装
        let keywords = vec!["a", "b", "c"];
        let expect_result = 0.0;

        let keywords = keywords.into_iter().map(String::from).collect();

        let config = Config::from_env();
        let atmosdict = Atmosdict::new(&config.database_path).await.unwrap();

        let atmosfreq = AtmosFreq::new(&SiteInfo::General { keywords }, &atmosdict).await;
        assert_eq!(atmosfreq, AtmosFreq::from(expect_result))
    }
}
