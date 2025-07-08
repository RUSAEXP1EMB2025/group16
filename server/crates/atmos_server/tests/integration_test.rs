use std::sync::Arc;

use axum::{
    body::Body,
    extract::Request,
    http::{self, StatusCode},
    response::Response,
    routing::{get, post},
};
use serde_json::json;
use tower::ServiceExt;

use atmos_dict::Atmosdict;
use atmos_server::{
    domain::Service,
    inbound::http::{AppState, routes},
    outbound::remo::Remo,
};

/// HTTPサーバーの結合テストヘルパー
struct IntegrationTestHelper {
    router: axum::Router,
    base_url: String,
}

impl IntegrationTestHelper {
    async fn new() -> Self {
        let config = atmos_config::Config::from_env();

        let atmosdict = Atmosdict::new(&config.database_path)
            .await
            .expect("Failed to create atmosdict");
        let atmosdict = Arc::new(atmosdict);
        let remo = Remo::new(Arc::clone(&atmosdict));

        let service = Service::new(remo, atmosdict);

        // 直接ルーターを構築
        let state = AppState {
            service: Arc::new(service),
        };

        let router = axum::Router::new()
            .route("/lighting", post(routes::adjust_lighting))
            .route("/lighting", get(routes::get_lighting_signals))
            .route("/atmosdict", get(routes::get_atmoswords))
            .with_state(state);

        Self {
            router,
            base_url: "http://localhost".to_string(),
        }
    }

    async fn send_request(&self, request: Request<Body>) -> Response<Body> {
        self.router.clone().oneshot(request).await.unwrap()
    }

    async fn get(&self, path: &str) -> Response<Body> {
        let request = Request::builder()
            .method(http::Method::GET)
            .uri(format!("{}{}", self.base_url, path))
            .body(Body::empty())
            .unwrap();

        self.send_request(request).await
    }

    async fn post(&self, path: &str, body: serde_json::Value) -> Response<Body> {
        let request = Request::builder()
            .method(http::Method::POST)
            .uri(format!("{}{}", self.base_url, path))
            .header("Content-Type", "application/json")
            .body(Body::from(body.to_string()))
            .unwrap();

        self.send_request(request).await
    }
}

/// 辞書取得APIのテスト
#[tokio::test]
async fn test_get_atmoswords() {
    let helper = IntegrationTestHelper::new().await;

    let response = helper.get("/atmosdict").await;

    assert_eq!(response.status(), StatusCode::OK);

    let body = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .unwrap();
    let body_str = String::from_utf8(body.to_vec()).unwrap();
    let json: serde_json::Value = serde_json::from_str(&body_str).unwrap();

    assert!(json.get("data").is_some());
    assert!(json["data"].get("atmoswords").is_some());
    assert!(json["data"]["atmoswords"].is_array());
}

/// 照明信号取得APIのテスト
#[tokio::test]
async fn test_get_lighting_signals() {
    let helper = IntegrationTestHelper::new().await;

    let request_body = json!({
        "remo_token": "test_token"
    });

    let response = helper.post("/lighting", request_body).await;

    // 実際のAPIトークンがないため、エラーレスポンスを期待
    assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
}

/// 照明調整APIのテスト - YouTube URL
#[tokio::test]
async fn test_adjust_lighting_youtube() {
    let helper = IntegrationTestHelper::new().await;

    let request_body = json!({
        "remo_token": "test_token",
        "site_data": {
            "Youtube": {
                "url": "https://www.youtube.com/watch?v=dQw4w9WgXcQ"
            }
        }
    });

    let response = helper.post("/lighting", request_body).await;

    // 実際のAPIトークンがないため、エラーレスポンスを期待
    assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
}

/// 照明調整APIのテスト - Netflix タイトル
#[tokio::test]
async fn test_adjust_lighting_netflix() {
    let helper = IntegrationTestHelper::new().await;

    let request_body = json!({
        "remo_token": "test_token",
        "site_data": {
            "Netflix": {
                "title": "The Crown"
            }
        }
    });

    let response = helper.post("/lighting", request_body).await;

    // 実際のAPIトークンがないため、エラーレスポンスを期待
    assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
}

/// 照明調整APIのテスト - 汎用キーワード
#[tokio::test]
async fn test_adjust_lighting_generic() {
    let helper = IntegrationTestHelper::new().await;

    let request_body = json!({
        "remo_token": "test_token",
        "site_data": {
            "Generic": {
                "keywords": ["平和", "リラックス", "癒し"]
            }
        }
    });

    let response = helper.post("/lighting", request_body).await;

    // 実際のAPIトークンがないため、エラーレスポンスを期待
    assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
}

/// 不正なリクエストボディのテスト
#[tokio::test]
async fn test_invalid_request_body() {
    let helper = IntegrationTestHelper::new().await;

    let request_body = json!({
        "invalid_field": "invalid_value"
    });

    let response = helper.post("/lighting", request_body).await;

    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
}

/// 不正なYouTube URLのテスト
#[tokio::test]
async fn test_invalid_youtube_url() {
    let helper = IntegrationTestHelper::new().await;

    let request_body = json!({
        "remo_token": "test_token",
        "site_data": {
            "Youtube": {
                "url": "invalid_url"
            }
        }
    });

    let response = helper.post("/lighting", request_body).await;

    assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
}

/// CORS設定のテスト
#[tokio::test]
async fn test_cors_headers() {
    let helper = IntegrationTestHelper::new().await;

    let request = Request::builder()
        .method(http::Method::OPTIONS)
        .uri(format!("{}/lighting", helper.base_url))
        .header("Origin", "https://www.youtube.com")
        .header("Access-Control-Request-Method", "POST")
        .header("Access-Control-Request-Headers", "content-type")
        .body(Body::empty())
        .unwrap();

    let response = helper.send_request(request).await;

    assert_eq!(response.status(), StatusCode::OK);
    assert!(
        response
            .headers()
            .contains_key("access-control-allow-origin")
    );
    assert!(
        response
            .headers()
            .contains_key("access-control-allow-methods")
    );
    assert!(
        response
            .headers()
            .contains_key("access-control-allow-headers")
    );
}

/// APIドキュメントのテスト
#[tokio::test]
async fn test_openapi_documentation() {
    let helper = IntegrationTestHelper::new().await;

    let response = helper.get("/swagger-ui").await;

    assert_eq!(response.status(), StatusCode::OK);

    let response = helper.get("/api-docs/openapi.json").await;

    assert_eq!(response.status(), StatusCode::OK);

    let body = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .unwrap();
    let body_str = String::from_utf8(body.to_vec()).unwrap();
    let json: serde_json::Value = serde_json::from_str(&body_str).unwrap();

    assert!(json.get("openapi").is_some());
    assert!(json.get("paths").is_some());
    assert!(json["paths"].get("/lighting").is_some());
    assert!(json["paths"].get("/atmosdict").is_some());
}
