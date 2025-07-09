use std::collections::HashSet;

use axum::{Json, extract::State};
use serde::Serialize;
use utoipa::ToSchema;

use crate::{
    domain::ports::{AtmosdictService, RemoService},
    inbound::http::{AppState, api::ApiError},
};

#[derive(Debug, Clone, PartialEq, Serialize, ToSchema)]
pub struct GetAtmosdictHttpResponseData {
    atmoswords: HashSet<String>,
}

#[utoipa::path(
    get,
    path = "/atmoswords",
    summary = "Get Atmoswords",
    description = "サイトから取得するべきキーワード辞書を取得",
    responses(
        (status = 200, description = "Success", body = GetAtmosdictHttpResponseData),
    ),
)]
pub async fn get_atmoswords<S: RemoService + AtmosdictService>(
    State(state): State<AppState<S>>,
) -> Result<Json<GetAtmosdictHttpResponseData>, ApiError> {
    let body = state
        .service
        .get_all_atmoswords()
        .await
        .map_err(ApiError::from)
        .map(|atmoswords| GetAtmosdictHttpResponseData { atmoswords })?;
    Ok(Json(body))
}
