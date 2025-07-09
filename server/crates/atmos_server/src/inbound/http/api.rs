use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use color_eyre::eyre;
use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ApiError {
    InternalServer(String),
    NotFound(String),
}

impl From<eyre::Error> for ApiError {
    fn from(e: eyre::Error) -> Self {
        Self::InternalServer(e.to_string())
    }
}

/// Generic response structure shared by all API responses.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ApiResponseBody<T: Serialize + PartialEq> {
    status_code: u16,
    data: T,
}

impl<T: Serialize + PartialEq> ApiResponseBody<T> {
    pub fn new(status_code: StatusCode, data: T) -> Self {
        Self {
            status_code: status_code.as_u16(),
            data,
        }
    }
}

impl ApiResponseBody<ApiErrorData> {
    pub fn new_error(status_code: StatusCode, message: String) -> Self {
        Self {
            status_code: status_code.as_u16(),
            data: ApiErrorData { message },
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ApiErrorData {
    pub message: String,
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        use ApiError::*;

        match self {
            InternalServer(e) => {
                tracing::error!("{}", e);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(ApiResponseBody::new_error(
                        StatusCode::INTERNAL_SERVER_ERROR,
                        "Internal server error".to_string(),
                    )),
                )
                    .into_response()
            }
            NotFound(e) => {
                tracing::error!("{}", e);
                (
                    StatusCode::NOT_FOUND,
                    Json(ApiResponseBody::new_error(
                        StatusCode::NOT_FOUND,
                        "Not found".to_string(),
                    )),
                )
                    .into_response()
            }
        }
    }
}
