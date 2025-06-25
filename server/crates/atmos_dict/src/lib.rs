use color_eyre::Result;
use std::{collections::HashSet, fs::File, io::BufReader, path::PathBuf};

#[derive(Clone)]
pub struct Atmosdict {
    dict_path: PathBuf,
}

impl Default for Atmosdict {
    fn default() -> Self {
        let cargo_manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let dict_path = cargo_manifest_dir.join("data").join("database.csv");
        Atmosdict { dict_path }
    }
}

impl Atmosdict {
    pub fn new() -> Self {
        Atmosdict::default()
    }

    pub fn get_all(&self) -> Result<Vec<String>> {
        todo!()
    }

    pub fn get_pos_neg(&self) -> Result<(HashSet<String>, HashSet<String>)> {
        let file = File::open(self.dict_path.clone())?;
        let mut rdr = csv::Reader::from_reader(BufReader::new(file));

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
}
