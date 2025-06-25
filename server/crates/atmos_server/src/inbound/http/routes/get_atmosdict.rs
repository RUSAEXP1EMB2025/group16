use axum::{extract::State, http::StatusCode};
use serde::Serialize;

use crate::{
    domain::ports::{AtmosdictService, RemoService},
    inbound::http::{
        AppState,
        api::{ApiError, ApiSuccess},
    },
};

// TODO: レスポンスを定義する
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct GetAtmosdictHttpResponseData {
    wordlist: Vec<String>,
}

#[utoipa::path(
    get,
    path = "/atmosdict",
    summary = "Get Atmosdict",
    description = "サイトから取得するべきキーワード辞書を取得",
    responses(
        (status = 200, description = "Success"),
    ),
)]
pub async fn get_atmosdict<S: RemoService + AtmosdictService>(
    State(state): State<AppState<S>>,
) -> Result<ApiSuccess<GetAtmosdictHttpResponseData>, ApiError> {
    state
        .service
        .get_atmos_dict()
        .await
        .map_err(ApiError::from)
        .map(|wordlist| ApiSuccess::new(StatusCode::OK, GetAtmosdictHttpResponseData { wordlist }))
}
