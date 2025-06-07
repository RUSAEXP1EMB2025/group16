use crate::{
    domain::{
        models::lighting::{AdjustLigtingRequest, Url},
        ports::lighting::LigtingRepository,
    },
    inbound::http::{
        AppState,
        api::{ApiError, ApiSuccess},
    },
};

use axum::{Json, extract::State, http::StatusCode};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, ToSchema)]
pub struct AdjustLightingHttpRequestBody {
    pub remo_token: String,
    pub url: String,
    pub texts: Vec<String>,
}

impl AdjustLightingHttpRequestBody {
    fn try_into_domain(self) -> Result<AdjustLigtingRequest, ParseAdjustLightingHttpRequestError> {
        Ok(AdjustLigtingRequest {
            remo_token: self.remo_token,
            url: Url::from(self.url.clone()),
            texts: self.texts,
        })
    }
}

#[derive(Debug, Clone, thiserror::Error)]
enum ParseAdjustLightingHttpRequestError {
    // TODO: エラーを定義する
}

impl From<ParseAdjustLightingHttpRequestError> for ApiError {
    fn from(e: ParseAdjustLightingHttpRequestError) -> Self {
        Self::FailedToAdjustLights(format!("Failed to adjust lights: {}", e))
    }
}

// TODO: レスポンスを定義する
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct AdjustLightingHttpResponseData;

#[utoipa::path(
    post,
    path = "/lighting",
    summary = "Adjust Lighting",
    description = "部屋の電気をサイト内容から調整",
    request_body = AdjustLightingHttpRequestBody,
    responses(
        (status = 200, description = "Success"),
    ),
)]
pub async fn adjust_lighting<AL: LigtingRepository>(
    State(state): State<AppState<AL>>,
    Json(body): Json<AdjustLightingHttpRequestBody>,
) -> Result<ApiSuccess<AdjustLightingHttpResponseData>, ApiError> {
    let domain_req = body.try_into_domain()?;

    state
        .lighting_repository
        .adjust(&domain_req)
        .await
        .map_err(ApiError::from)
        .map(|_| ApiSuccess::new(StatusCode::OK, AdjustLightingHttpResponseData))
}
