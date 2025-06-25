use crate::inbound::http::api::ApiError;

#[derive(Debug, thiserror::Error)]
pub enum GetAtmosdictError {
    // TODO: エラーを定義する
    #[error("")]
    Sample,
}

impl From<GetAtmosdictError> for ApiError {
    fn from(e: GetAtmosdictError) -> Self {
        ApiError::InternalServerError(e.to_string())
    }
}
