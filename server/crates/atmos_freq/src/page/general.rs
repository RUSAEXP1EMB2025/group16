use crate::AtmosFreq;
use csv::Reader;
use derive_more::From;
use std::collections::HashSet;
use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;
use url::Url;

impl AtmosFreq {
    pub fn from_general(keywords: &Vec<String>) -> Self {
        let cargo_manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let database_path = cargo_manifest_dir.join("data").join("database.csv");

        let (pos_dict, neg_dict) = load_dicts_from_csv(&database_path).unwrap();

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

fn load_dicts_from_csv(
    path: &PathBuf,
) -> Result<(HashSet<String>, HashSet<String>), Box<dyn std::error::Error>> {
    let file = File::open(path)?;
    let mut rdr = Reader::from_reader(BufReader::new(file));

    let mut pos_dict = HashSet::new();
    let mut neg_dict = HashSet::new();

    for result in rdr.records() {
        let record = result?;
        if let Some(pos) = record.get(0) {
            if !pos.trim().is_empty() {
                pos_dict.insert(pos.trim().to_string());
            }
        }
        if let Some(neg) = record.get(1) {
            if !neg.trim().is_empty() {
                neg_dict.insert(neg.trim().to_string());
            }
        }
    }

    Ok((pos_dict, neg_dict))
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
