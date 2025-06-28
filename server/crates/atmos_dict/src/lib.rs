//! これはポジティブ/ネガティブな単語を操作するライブラリである。

pub mod error;

use color_eyre::eyre;
use error::AtmosdictError;
use std::{collections::HashSet, fs::File, io::BufReader, path::PathBuf};

/// ポジティブ/ネガティブな単語を操作する構造体
/// ## 使い方
/// ```
/// let atmosdict = AtmosDict::new();
///
/// // 全ての単語を取得
/// let all_words = atmosdict.get_all();
///
/// // ポジティブ/ネガティブに分けて単語を取得
/// let (pos_words, neg_words) = atmosdict.get_pos_neg();
///
/// ```
#[derive(Clone)]
pub struct Atmosdict {
    /// 辞書データファイルのパス
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

    /// 全てのキーワードを取得
    pub fn get_all(&self) -> Result<Vec<String>, AtmosdictError> {
        todo!()
    }

    /// キーワードをポジティブ/ネガティブに分けて取得
    pub fn get_pos_neg(&self) -> Result<(HashSet<String>, HashSet<String>), AtmosdictError> {
        let file = File::open(self.dict_path.clone()).map_err(AtmosdictError::LoadDataFile)?;
        let mut rdr = csv::Reader::from_reader(BufReader::new(file));

        let mut pos_dict = HashSet::new();
        let mut neg_dict = HashSet::new();

        for result in rdr.records() {
            let record = result.map_err(AtmosdictError::ReadRecords)?;
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
