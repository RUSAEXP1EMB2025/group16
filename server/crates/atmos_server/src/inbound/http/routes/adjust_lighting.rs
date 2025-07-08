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

use atmos_freq::SiteData;
use axum::{Json, extract::State, http::StatusCode};
use serde::{Deserialize, Serialize};
use url::Url;
use utoipa::ToSchema;

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, ToSchema)]
pub struct AdjustLightingHttpRequestBody {
    pub remo_token: String,
    pub site_data: SiteDataRequest,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, ToSchema)]
pub enum SiteDataRequest {
    Youtube { url: String },
    Netflix { title: String },
    Generic { keywords: Vec<String> },
}

impl AdjustLightingHttpRequestBody {
    fn try_into_domain(self) -> Result<AdjustLigtingRequest, ParseAdjustLightingHttpRequestError> {
        type Error = ParseAdjustLightingHttpRequestError;

        let site_data = match self.site_data {
            SiteDataRequest::Youtube { url } => {
                let parsed_url = Url::parse(&url).map_err(Error::InvalidUrlFormat)?;
                SiteData::Youtube { url: parsed_url }
            }
            SiteDataRequest::Netflix { title } => SiteData::Netflix { title },
            SiteDataRequest::Generic { keywords } => SiteData::Generic { keywords },
        };

        Ok(AdjustLigtingRequest {
            remo_token: self.remo_token,
            site_data,
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
        Self::InternalServer(format!("Invalid params: {}", e))
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct AdjustLightingHttpResponseData {
    message: &'static str,
}

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
        .map(|_| {
            ApiSuccess::new(
                StatusCode::OK,
                AdjustLightingHttpResponseData {
                    message: "Successfly adjusted lighting amount",
                },
            )
        })
}
