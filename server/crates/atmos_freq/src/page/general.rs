use atmos_dict::Atmosdict;

use crate::AtmosFreq;

impl AtmosFreq {
    /// サイトから雰囲気指数を算出
    pub async fn from_general(keywords: &Vec<String>, atmosdict: &Atmosdict) -> Self {
        let pos_dict = atmosdict.get_positive().await.unwrap();
        let neg_dict = atmosdict.get_negative().await.unwrap();

        let mut pos_count = 0;
        let mut neg_count = 0;

        for word in keywords {
            if pos_dict.contains(word) {
                pos_count += 1;
            }
            if neg_dict.contains(word) {
                neg_count += 1;
            }
        }
        let total = pos_count + neg_count;

        let score = (pos_count as f64 / total as f64) * 100.0;
        AtmosFreq(score)
    }
}

#[cfg(test)]
mod test {
    use crate::AtmosFreq;
    use atmos_dict::Atmosdict;

    fn sample_words(s: Vec<&str>) -> Vec<String> {
        s.into_iter().map(String::from).collect()
    }

    #[tokio::test]
    async fn test_generate_atmosfreq_from_general() {
        let config = atmos_config::Config::from_env();
        let atmosdict = Atmosdict::new(&config.database_path).await.unwrap();

        let words = sample_words(vec!["ホラー"]);
        let atomos_freq = AtmosFreq::from_general(&words, &atmosdict).await;
        dbg!(atomos_freq);

        let words = sample_words(vec!["ホラー", "ホラー"]);
        let atomos_freq = AtmosFreq::from_general(&words, &atmosdict).await;
        dbg!(atomos_freq);

        let words = sample_words(vec!["ホラー", "感謝"]);
        let atomos_freq = AtmosFreq::from_general(&words, &atmosdict).await;
        dbg!(atomos_freq);

        let words = sample_words(vec!["ホラー", "感謝", "感謝"]);
        let atomos_freq = AtmosFreq::from_general(&words, &atmosdict).await;
        dbg!(atomos_freq);

        let words = sample_words(vec!["感謝"]);
        let atomos_freq = AtmosFreq::from_general(&words, &atmosdict).await;
        dbg!(atomos_freq);

        let words = sample_words(vec!["感謝", "感謝"]);
        let atomos_freq = AtmosFreq::from_general(&words, &atmosdict).await;
        dbg!(atomos_freq);

        let words = sample_words(vec!["a", "感謝"]);
        let atomos_freq = AtmosFreq::from_general(&words, &atmosdict).await;
        dbg!(atomos_freq);

        let words = sample_words(vec!["a", "ホラー"]);
        let atomos_freq = AtmosFreq::from_general(&words, &atmosdict).await;
        dbg!(atomos_freq);
    }
}
