use std::collections::HashSet;

use axum::{extract::State, http::StatusCode};
use serde::Serialize;

use crate::{
    domain::ports::{AtmosdictService, RemoService},
    inbound::http::{
        AppState,
        api::{ApiError, ApiSuccess},
    },
};

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct GetAtmosdictHttpResponseData {
    atmoswords: HashSet<String>,
}

#[utoipa::path(
    get,
    path = "/atmoswords",
    summary = "Get Atmoswords",
    description = "サイトから取得するべきキーワード辞書を取得",
    responses(
        (status = 200, description = "Success"),
    ),
)]
pub async fn get_atmoswords<S: RemoService + AtmosdictService>(
    State(state): State<AppState<S>>,
) -> Result<ApiSuccess<GetAtmosdictHttpResponseData>, ApiError> {
    state
        .service
        .get_all_atmoswords()
        .await
        .map_err(ApiError::from)
        .map(|atmoswords| {
            ApiSuccess::new(StatusCode::OK, GetAtmosdictHttpResponseData { atmoswords })
        })
}
