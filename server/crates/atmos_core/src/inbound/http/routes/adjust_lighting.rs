use crate::{
    core::{
        models::{
            AdjustLigtingRequest,
            adjust_lighting::{SiteInfo, Url},
        },
        ports::AdjustLigtingRepository,
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
    pub html: String,
}

impl AdjustLightingHttpRequestBody {
    fn try_into_domain(self) -> Result<AdjustLigtingRequest, ParseAdjustLightingHttpRequestError> {
        let url = Url::from(self.url);
        Ok(AdjustLigtingRequest {
            remo_token: self.remo_token,
            url: url.clone(),
            site_info: SiteInfo::new(url, self.html),
        })
    }
}

#[derive(Debug, Clone, thiserror::Error)]
enum ParseAdjustLightingHttpRequestError {
    // TODO: Define errors
}

impl From<ParseAdjustLightingHttpRequestError> for ApiError {
    fn from(e: ParseAdjustLightingHttpRequestError) -> Self {
        Self::FailedToAdjustLights(format!("Failed to adjust lights: {}", e))
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct AdjustLightingHttpResponseData;

#[utoipa::path(
    post,
    path = "/adjust-lighting",
    summary = "Adjust Lighting",
    description = "部屋の電気をサイト内容から調整",
    request_body = AdjustLightingHttpRequestBody,
    responses(
        (status = 200, description = "Success"),
    ),
)]
pub async fn adjust_lighting<AL: AdjustLigtingRepository>(
    State(state): State<AppState<AL>>,
    Json(body): Json<AdjustLightingHttpRequestBody>,
) -> Result<ApiSuccess<AdjustLightingHttpResponseData>, ApiError> {
    let domain_req = body.try_into_domain()?;

    state
        .adjust_lighting_repository
        .adjust_lighting(&domain_req)
        .await
        .map_err(ApiError::from)
        .map(|_| ApiSuccess::new(StatusCode::OK, AdjustLightingHttpResponseData))
}
