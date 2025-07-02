use axum::{Json, extract::State, http::StatusCode};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::{
    domain::{
        models::remo::{GetLigtingSignalsRequest, LightingSignals},
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
    fn try_into_domain(self) -> GetLigtingSignalsRequest {
        GetLigtingSignalsRequest {
            remo_token: self.remo_token,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct GetLightingSignalsHttpResponseData {
    signals: LightingSignals,
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
    let domain_req = body.try_into_domain();

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
