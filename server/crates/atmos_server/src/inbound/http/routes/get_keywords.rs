use axum::{Json, extract::State, http::StatusCode};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::{
    domain::{
        models::keywords::GetKeywordsRequest,
        ports::{KeywordsService, RemoService},
    },
    inbound::http::{
        AppState,
        api::{ApiError, ApiSuccess},
    },
};

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, ToSchema)]
pub struct GetKeywordsHttpRequestBody {}

impl GetKeywordsHttpRequestBody {
    pub fn try_into_domain(self) -> Result<GetKeywordsRequest, ParseGetKeywordsHttpRequestError> {
        Ok(GetKeywordsRequest {})
    }
}

#[derive(Debug, Clone, thiserror::Error)]
pub enum ParseGetKeywordsHttpRequestError {
    // TODO: エラーを定義する
}

impl From<ParseGetKeywordsHttpRequestError> for ApiError {
    fn from(e: ParseGetKeywordsHttpRequestError) -> Self {
        Self::FailedToAdjustLights(format!("Failed to get wordlist: {}", e))
    }
}

// TODO: レスポンスを定義する
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct GetKeywordsHttpResponseData {
    wordlist: Vec<String>,
}

#[utoipa::path(
    get,
    path = "/wordlist",
    summary = "Get wordlist",
    description = "サイトから取得するべきキーワードを取得",
    request_body = GetKeywordsHttpRequestBody,
    responses(
        (status = 200, description = "Success"),
    ),
)]
pub async fn get_keywords<S: RemoService + KeywordsService>(
    State(state): State<AppState<S>>,
    Json(body): Json<GetKeywordsHttpRequestBody>,
) -> Result<ApiSuccess<GetKeywordsHttpResponseData>, ApiError> {
    let domain_req = body.try_into_domain()?;

    state
        .service
        .get_keywords(&domain_req)
        .await
        .map_err(ApiError::from)
        .map(|wordlist| ApiSuccess::new(StatusCode::OK, GetKeywordsHttpResponseData { wordlist }))
}
