use std::env;

use crate::AtmosFreq;

impl AtmosFreq {
    pub async fn from_netflix(title: &str) -> Self {
        let api_key = env::var("NETFLIX_API_KEY").unwrap();
        let result = omdb::title(title).apikey(api_key).get().await;

        match result {
            Ok(movie) => {
                if movie.kind == omdb::Kind::Movie {
                    AtmosFreq(0.0)
                } else {
                    AtmosFreq(100.0)
                }
            }
            Err(_) => AtmosFreq(100.0),
        }
    }
}

#[cfg(test)]
mod test {
    use crate::AtmosFreq;
    use atmos_config::Config;
    use tokio;

    #[tokio::test]
    async fn test_generate_atmosfreq_from_netflix_for_movie() {
        let netflix_api_key = Config::from_env();
        let movie_title = "Inception";
        let atmos_freq = AtmosFreq::from_netflix(movie_title).await;
        // Inception is a movie, so we expect 0.0
        assert_eq!(
            atmos_freq.0, 0.0,
            "Expected 0.0 for a movie, got {}",
            atmos_freq.0
        );
    }

    #[tokio::test]
    async fn test_generate_atmosfreq_from_netflix_for_series() {
        let netflix_api_key = Config::from_env();
        let series_title = "Breaking Bad";
        let atmos_freq = AtmosFreq::from_netflix(series_title).await;
        // Breaking Bad is a series, so we expect 100.0
        assert_eq!(
            atmos_freq.0, 100.0,
            "Expected 100.0 for a series, got {}",
            atmos_freq.0
        );
    }

    #[tokio::test]
    async fn test_generate_atmosfreq_from_netflix_for_unknown_title() {
        let netflix_api_key = Config::from_env();
        let unknown_title = "This Title Does Not Exist In OMDB 12345";
        let result = AtmosFreq::from_netflix(unknown_title).await;
        assert_eq!(
            result.0, 100.0,
            "Expected 100.0 for an unknown or non-movie title, got {}",
            result.0
        );
    }
}
