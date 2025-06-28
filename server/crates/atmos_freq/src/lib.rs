//! これはサイト情報から算出される雰囲気指数を取り扱うライブラリである。
pub mod page;

use derive_more::From;
use url::Url;

// 雰囲気指数 (0 ~ 100.0)
#[derive(PartialEq, Debug, From)]
pub struct AtmosFreq(f64);

impl AtmosFreq {
    pub async fn new(url: &Url, keywords: &Vec<String>) -> Self {
        match url.as_str() {
            "path/to/youtube" => Self::from_youtube(url, keywords).await,
            "path/to/netflix" => Self::from_netflix(keywords),
            _ => Self::from_general(keywords),
        }
    }
}

#[cfg(test)]
mod test {
    use url::Url;

    use super::AtmosFreq;

    #[tokio::test]
    async fn test_calc_atmosfreq_from_general() {
        // TODO: テストケースの実装
        let texts = vec!["a", "b", "c"];
        let url = Url::parse("").unwrap();
        let expect_result = 0.0;

        let texts = texts.into_iter().map(String::from).collect();

        let atmosfreq = AtmosFreq::new(&url, &texts).await;
        assert_eq!(atmosfreq, AtmosFreq::from(expect_result))
    }
}
