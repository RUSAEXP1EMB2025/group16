#[derive(Debug, thiserror::Error)]
pub enum AtmosdictError {
    #[error("Failed to execute database operation: {0}")]
    Database(#[from] sqlx::Error),
}
