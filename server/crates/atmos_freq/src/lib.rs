//! これはサイト情報から算出される雰囲気指数を取り扱うライブラリである。
pub mod page;

use atmos_dict::Atmosdict;
use page::{general::from_general, netflix::from_netflix, youtube::from_youtube};
use url::Url;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum SiteInfo {
    Youtube { url: Url },
    Netflix { title: String },
    General { keywords: Vec<String> },
}

pub async fn calc_atmosfreq<A>(siteinfo: &SiteInfo, atmosdict: A) -> f64
where
    A: AsRef<Atmosdict>,
{
    let atmosdict = atmosdict.as_ref();

    match siteinfo {
        SiteInfo::Youtube { url } => from_youtube(url, atmosdict).await,
        SiteInfo::Netflix { title } => from_netflix(title).await,
        SiteInfo::General { keywords } => from_general(keywords, atmosdict).await,
    }
}

#[cfg(test)]
mod test {
    use atmos_config::Config;
    use atmos_dict::Atmosdict;

    use crate::{SiteInfo, calc_atmosfreq};

    #[tokio::test]
    async fn test_calc_atmosfreq_from_general() {
        // TODO: テストケースの実装
        let keywords = vec!["a", "b", "c"];
        let expect_result = 0.0;

        let keywords = keywords.into_iter().map(String::from).collect();

        let config = Config::from_env();
        let atmosdict = Atmosdict::new(&config.database_path).await.unwrap();

        let atmosfreq = calc_atmosfreq(&SiteInfo::General { keywords }, &atmosdict).await;
        assert_eq!(atmosfreq, expect_result)
    }
}
