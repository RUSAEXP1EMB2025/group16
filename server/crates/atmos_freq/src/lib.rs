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
