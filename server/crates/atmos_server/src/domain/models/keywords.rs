use crate::inbound::http::api::ApiError;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct GetKeywordsRequest {}

#[derive(Debug, thiserror::Error)]
pub enum GetKeywordsError {
    // TODO: エラーを定義する
}

impl From<GetKeywordsError> for ApiError {
    fn from(e: GetKeywordsError) -> Self {
        ApiError::InternalServerError(e.to_string())
    }
}
