#[derive(Debug, thiserror::Error)]
pub enum AtmosdictError {
    #[error("Failed to load dictionary data file: {0}")]
    LoadDataFile(std::io::Error),

    #[error("Failed to read records: {0}")]
    ReadRecords(csv::Error),
}
