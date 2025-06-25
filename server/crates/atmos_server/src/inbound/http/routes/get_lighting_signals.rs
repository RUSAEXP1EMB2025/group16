use axum::{Json, extract::State, http::StatusCode};
use remo_api::models::Signal;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::{
    domain::{
        models::remo::GetLigtingSignalsRequest,
        ports::{AtmosdictService, RemoService},
    },
    inbound::http::{
        AppState,
        api::{ApiError, ApiSuccess},
    },
};

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, ToSchema)]
pub struct GetLightingSignalsHttpRequestBody {
    pub remo_token: String,
}

impl GetLightingSignalsHttpRequestBody {
    fn try_into_domain(
        self,
    ) -> Result<GetLigtingSignalsRequest, ParseGetLightingSignalsHttpRequestError> {
        Ok(GetLigtingSignalsRequest {
            remo_token: self.remo_token,
        })
    }
}

#[derive(Debug, Clone, thiserror::Error)]
enum ParseGetLightingSignalsHttpRequestError {
    // TODO: エラーを定義する
}

impl From<ParseGetLightingSignalsHttpRequestError> for ApiError {
    fn from(e: ParseGetLightingSignalsHttpRequestError) -> Self {
        Self::FailedToAdjustLights(format!("Failed to adjust lights: {}", e))
    }
}

// TODO: レスポンスを定義する
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct GetLightingSignalsHttpResponseData {
    signals: Vec<Signal>,
}

#[utoipa::path(
    get,
    path = "/lighting",
    summary = "Get lighting signals",
    description = "登録されている電気の信号を取得",
    request_body = GetLightingSignalsHttpRequestBody,
    responses(
        (status = 200, description = "Success"),
    ),
)]
pub async fn get_lighting_signals<S: RemoService + AtmosdictService>(
    State(state): State<AppState<S>>,
    Json(body): Json<GetLightingSignalsHttpRequestBody>,
) -> Result<ApiSuccess<GetLightingSignalsHttpResponseData>, ApiError> {
    let domain_req = body.try_into_domain()?;

    state
        .service
        .get_lighting_signals(&domain_req)
        .await
        .map_err(ApiError::from)
        .map(|signals| {
            ApiSuccess::new(
                StatusCode::OK,
                GetLightingSignalsHttpResponseData { signals },
            )
        })
}
