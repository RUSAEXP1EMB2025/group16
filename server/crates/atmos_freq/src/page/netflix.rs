use atmos_config::Config;

pub async fn from_netflix(title: &str) -> f64 {
    let api_key = Config::from_env().netflix_api_key;
    let result = omdb::title(title).apikey(api_key).get().await;

    match result {
        Ok(movie) => {
            if movie.kind == omdb::Kind::Movie {
                0.0
            } else {
                100.0
            }
        }
        Err(_) => 100.0,
    }
}

#[cfg(test)]
mod test {
    use tokio;

    use crate::page::netflix::from_netflix;

    #[tokio::test]
    async fn test_generate_atmosfreq_from_netflix_for_movie() {
        let movie_title = "Inception";
        let atmos_freq = from_netflix(movie_title).await;
        assert_eq!(atmos_freq, 0.0,);
    }

    #[tokio::test]
    async fn test_generate_atmosfreq_from_netflix_for_series() {
        let series_title = "Breaking Bad";
        let atmos_freq = from_netflix(series_title).await;
        assert_eq!(atmos_freq, 100.0,);
    }

    #[tokio::test]
    async fn test_generate_atmosfreq_from_netflix_for_unknown_title() {
        let unknown_title = "This Title Does Not Exist In OMDB 12345";
        let result = from_netflix(unknown_title).await;
        assert_eq!(result, 100.0,);
    }
}
