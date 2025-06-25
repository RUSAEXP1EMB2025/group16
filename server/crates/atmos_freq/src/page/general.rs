use atmos_dict::Atmosdict;

use crate::AtmosFreq;

impl AtmosFreq {
    pub fn from_general(keywords: &Vec<String>) -> Self {
        let atmos_dict = Atmosdict::new();
        let (pos_dict, neg_dict) = atmos_dict.get_pos_neg().unwrap();

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

    fn sample_words(s: Vec<&str>) -> Vec<String> {
        s.into_iter().map(String::from).collect()
    }

    #[test]
    fn test_generate_atmosfreq_from_general() {
        let words = sample_words(vec!["ホラー"]);
        let atomos_freq = AtmosFreq::from_general(&words);
        dbg!(atomos_freq);

        let words = sample_words(vec!["ホラー", "ホラー"]);
        let atomos_freq = AtmosFreq::from_general(&words);
        dbg!(atomos_freq);

        let words = sample_words(vec!["ホラー", "感謝"]);
        let atomos_freq = AtmosFreq::from_general(&words);
        dbg!(atomos_freq);

        let words = sample_words(vec!["ホラー", "感謝", "感謝"]);
        let atomos_freq = AtmosFreq::from_general(&words);
        dbg!(atomos_freq);

        let words = sample_words(vec!["感謝"]);
        let atomos_freq = AtmosFreq::from_general(&words);
        dbg!(atomos_freq);

        let words = sample_words(vec!["感謝", "感謝"]);
        let atomos_freq = AtmosFreq::from_general(&words);
        dbg!(atomos_freq);

        let words = sample_words(vec!["a", "感謝"]);
        let atomos_freq = AtmosFreq::from_general(&words);
        dbg!(atomos_freq);

        let words = sample_words(vec!["a", "ホラー"]);
        let atomos_freq = AtmosFreq::from_general(&words);
        dbg!(atomos_freq);
    }
}
