pub mod page;

use atmos_dict::Atmosdict;
use page::{from_general, from_netflix, from_youtube};
use url::Url;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum SiteData {
    Youtube { url: Url },
    Netflix { title: String },
    Generic { keywords: Vec<String> },
}

pub async fn calc_atmosfreq<A>(siteinfo: &SiteData, atmosdict: A) -> f64
where
    A: AsRef<Atmosdict>,
{
    let atmosdict = atmosdict.as_ref();

    match siteinfo {
        SiteData::Youtube { url } => from_youtube(url, atmosdict).await,
        SiteData::Netflix { title } => from_netflix(title).await,
        SiteData::Generic { keywords } => from_general(keywords, atmosdict).await,
    }
}

#[cfg(test)]
mod test {
    use atmos_config::Config;
    use atmos_dict::Atmosdict;

    use crate::{SiteData, calc_atmosfreq};

    #[tokio::test]
    async fn test_calc_atmosfreq_from_general() {
        // TODO: テストケースの実装
        let keywords = vec!["a", "b", "c"];
        let expect_result = 0.0;

        let keywords = keywords.into_iter().map(String::from).collect();

        let config = Config::from_env();
        let atmosdict = Atmosdict::new(&config.database_path).await.unwrap();

        let atmosfreq = calc_atmosfreq(&SiteData::Generic { keywords }, &atmosdict).await;
        assert_eq!(atmosfreq, expect_result)
    }
}
