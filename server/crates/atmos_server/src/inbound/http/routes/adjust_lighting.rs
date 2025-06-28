use crate::{
    domain::{
        models::remo::AdjustLigtingRequest,
        ports::{AtmosdictService, RemoService},
    },
    inbound::http::{
        AppState,
        api::{ApiError, ApiSuccess},
    },
};

use atmos_freq::SiteInfo;
use axum::{Json, extract::State, http::StatusCode};
use serde::{Deserialize, Serialize};
use url::Url;
use utoipa::ToSchema;

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, ToSchema)]
pub struct AdjustLightingHttpRequestBody {
    pub remo_token: String,
    pub url: String,
    pub keywords: Vec<String>,
}

impl AdjustLightingHttpRequestBody {
    fn try_into_domain(self) -> Result<AdjustLigtingRequest, ParseAdjustLightingHttpRequestError> {
        type Error = ParseAdjustLightingHttpRequestError;

        let site_info = match self.url.as_str() {
            url if url.contains("youtube") => {
                let parsed_url = Url::parse(url).map_err(Error::InvalidUrlFormat)?;
                SiteInfo::Youtube { url: parsed_url }
            }
            url if url.contains("netflix") => {
                let title = self
                    .keywords
                    .into_iter()
                    .next()
                    .ok_or(Error::TitleNotFoundForNetflix)?;
                SiteInfo::Netflix { title }
            }
            _ => SiteInfo::General {
                keywords: self.keywords,
            },
        };

        Ok(AdjustLigtingRequest {
            remo_token: self.remo_token,
            site_info,
        })
    }
}

#[derive(Debug, Clone, thiserror::Error)]
enum ParseAdjustLightingHttpRequestError {
    #[error("Url format is invalid")]
    InvalidUrlFormat(url::ParseError),

    #[error("Title not found for Netflix")]
    TitleNotFoundForNetflix,
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
pub async fn adjust_lighting<S: RemoService + AtmosdictService>(
    State(state): State<AppState<S>>,
    Json(body): Json<AdjustLightingHttpRequestBody>,
) -> Result<ApiSuccess<AdjustLightingHttpResponseData>, ApiError> {
    let domain_req = body.try_into_domain()?;

    state
        .service
        .adjust_lighting(&domain_req)
        .await
        .map_err(ApiError::from)
        .map(|_| ApiSuccess::new(StatusCode::OK, AdjustLightingHttpResponseData))
}
