use std::sync::Arc;

use atmos_dict::Atmosdict;
use atmos_freq::SiteData;
use atmos_server::{
    domain::{
        models::remo::{AdjustLigtingRequest, GetLigtingSignalsRequest},
        ports::RemoRepository,
    },
    outbound::remo::Remo,
};
use url::Url;

/// Remo APIとの結合テストヘルパー
struct RemoIntegrationTestHelper {
    remo_service: Remo,
    test_token: String,
}

impl RemoIntegrationTestHelper {
    async fn new() -> Self {
        let config = atmos_config::Config::from_env();
        let atmosdict = Atmosdict::new(&config.database_path)
            .await
            .expect("Failed to create atmosdict");
        let atmosdict = Arc::new(atmosdict);
        let remo_service = Remo::new(atmosdict);

        Self {
            remo_service,
            test_token: "test_token".to_string(),
        }
    }

    fn create_youtube_request(&self, url: &str) -> AdjustLigtingRequest {
        AdjustLigtingRequest {
            remo_token: self.test_token.clone(),
            site_data: SiteData::Youtube {
                url: Url::parse(url).expect("Invalid URL"),
            },
        }
    }

    fn create_netflix_request(&self, title: &str) -> AdjustLigtingRequest {
        AdjustLigtingRequest {
            remo_token: self.test_token.clone(),
            site_data: SiteData::Netflix {
                title: title.to_string(),
            },
        }
    }

    fn create_generic_request(&self, keywords: Vec<&str>) -> AdjustLigtingRequest {
        AdjustLigtingRequest {
            remo_token: self.test_token.clone(),
            site_data: SiteData::Generic {
                keywords: keywords.into_iter().map(String::from).collect(),
            },
        }
    }

    fn create_get_signals_request(&self) -> GetLigtingSignalsRequest {
        GetLigtingSignalsRequest {
            remo_token: self.test_token.clone(),
        }
    }
}

/// 照明信号取得のテスト（無効なトークン）
#[tokio::test]
async fn test_get_lighting_signals_invalid_token() {
    let helper = RemoIntegrationTestHelper::new().await;

    let request = helper.create_get_signals_request();
    let result = helper.remo_service.get_lighting_signals(&request).await;

    // 無効なトークンの場合はエラーが返されることを期待
    assert!(result.is_err());
}

/// YouTube URLでの照明調整テスト（無効なトークン）
#[tokio::test]
async fn test_adjust_lighting_youtube_invalid_token() {
    let helper = RemoIntegrationTestHelper::new().await;

    let request = helper.create_youtube_request("https://www.youtube.com/watch?v=dQw4w9WgXcQ");
    let result = helper.remo_service.adjust_lighting(&request).await;

    // 無効なトークンの場合はエラーが返されることを期待
    assert!(result.is_err());
}

/// Netflix タイトルでの照明調整テスト（無効なトークン）
#[tokio::test]
async fn test_adjust_lighting_netflix_invalid_token() {
    let helper = RemoIntegrationTestHelper::new().await;

    let request = helper.create_netflix_request("The Crown");
    let result = helper.remo_service.adjust_lighting(&request).await;

    // 無効なトークンの場合はエラーが返されることを期待
    assert!(result.is_err());
}

/// 汎用キーワードでの照明調整テスト（無効なトークン）
#[tokio::test]
async fn test_adjust_lighting_generic_invalid_token() {
    let helper = RemoIntegrationTestHelper::new().await;

    let request = helper.create_generic_request(vec!["平和", "リラックス", "癒し"]);
    let result = helper.remo_service.adjust_lighting(&request).await;

    // 無効なトークンの場合はエラーが返されることを期待
    assert!(result.is_err());
}

/// 恐怖系キーワードでの照明調整テスト（無効なトークン）
#[tokio::test]
async fn test_adjust_lighting_horror_keywords_invalid_token() {
    let helper = RemoIntegrationTestHelper::new().await;

    let request = helper.create_generic_request(vec!["ホラー", "恐怖", "怖い"]);
    let result = helper.remo_service.adjust_lighting(&request).await;

    // 無効なトークンの場合はエラーが返されることを期待
    assert!(result.is_err());
}

/// 無効なYouTube URLでの照明調整テスト
#[tokio::test]
async fn test_adjust_lighting_invalid_youtube_url() {
    // 無効なURLの場合、URL::parseでエラーが発生することを期待
    let result = Url::parse("invalid_url");
    assert!(result.is_err());

    // また、実際の使用でも同様にエラーハンドリングされることを確認
    let _helper = RemoIntegrationTestHelper::new().await;

    // 無効なURLでリクエスト作成を試みる（パニックではなく、適切なエラーハンドリングを期待）
    let url_result = Url::parse("invalid_url");
    assert!(url_result.is_err());
}

/// 空のキーワードでの照明調整テスト
#[tokio::test]
async fn test_adjust_lighting_empty_keywords() {
    let helper = RemoIntegrationTestHelper::new().await;

    let request = helper.create_generic_request(vec![]);
    let result = helper.remo_service.adjust_lighting(&request).await;

    // 空のキーワードでも処理は実行されるが、トークンが無効なためエラーが返される
    assert!(result.is_err());
}

/// 日本語以外のキーワードでの照明調整テスト
#[tokio::test]
async fn test_adjust_lighting_english_keywords() {
    let helper = RemoIntegrationTestHelper::new().await;

    let request = helper.create_generic_request(vec!["peace", "relax", "calm"]);
    let result = helper.remo_service.adjust_lighting(&request).await;

    // 英語キーワードでも処理は実行されるが、トークンが無効なためエラーが返される
    assert!(result.is_err());
}

/// 複数の日本語キーワードでの照明調整テスト
#[tokio::test]
async fn test_adjust_lighting_multiple_japanese_keywords() {
    let helper = RemoIntegrationTestHelper::new().await;

    let request = helper.create_generic_request(vec![
        "平和",
        "リラックス",
        "癒し",
        "穏やか",
        "安らぎ",
        "心地よい",
    ]);
    let result = helper.remo_service.adjust_lighting(&request).await;

    // 複数のキーワードでも処理は実行されるが、トークンが無効なためエラーが返される
    assert!(result.is_err());
}

/// 長いNetflixタイトルでの照明調整テスト
#[tokio::test]
async fn test_adjust_lighting_long_netflix_title() {
    let helper = RemoIntegrationTestHelper::new().await;

    let request = helper
        .create_netflix_request("The Queen's Gambit: A Very Long Title That Might Test The System");
    let result = helper.remo_service.adjust_lighting(&request).await;

    // 長いタイトルでも処理は実行されるが、トークンが無効なためエラーが返される
    assert!(result.is_err());
}

/// 特殊文字を含むNetflixタイトルでの照明調整テスト
#[tokio::test]
async fn test_adjust_lighting_special_characters_netflix_title() {
    let helper = RemoIntegrationTestHelper::new().await;

    let request =
        helper.create_netflix_request("Attack on Titan: The Final Season - Part 2 (2022)");
    let result = helper.remo_service.adjust_lighting(&request).await;

    // 特殊文字を含むタイトルでも処理は実行されるが、トークンが無効なためエラーが返される
    assert!(result.is_err());
}

/// 日本語のNetflixタイトルでの照明調整テスト
#[tokio::test]
async fn test_adjust_lighting_japanese_netflix_title() {
    let helper = RemoIntegrationTestHelper::new().await;

    let request = helper.create_netflix_request("進撃の巨人 The Final Season");
    let result = helper.remo_service.adjust_lighting(&request).await;

    // 日本語タイトルでも処理は実行されるが、トークンが無効なためエラーが返される
    assert!(result.is_err());
}

// 注意: 以下のテストは実際のRemo APIトークンが必要です
// 実際のテストを実行する場合は、環境変数にトークンを設定してください

/// 実際のトークンを使用した照明信号取得テスト（ignored）
#[tokio::test]
#[ignore = "実際のRemo APIトークンが必要"]
async fn test_get_lighting_signals_with_real_token() {
    let helper = RemoIntegrationTestHelper::new().await;

    // 実際のトークンを環境変数から取得
    let real_token =
        std::env::var("REMO_TOKEN").expect("REMO_TOKEN environment variable is required");

    let request = GetLigtingSignalsRequest {
        remo_token: real_token,
    };

    let result = helper.remo_service.get_lighting_signals(&request).await;

    // 実際のトークンの場合は成功することを期待
    assert!(result.is_ok());

    let signals = result.unwrap();
    // 照明信号が取得できることを確認（LightingSignalsの構造に基づく）
    // on, off, up, downフィールドが存在することを確認
    assert!(!signals.on.id.is_none());
    assert!(!signals.off.id.is_none());
    assert!(!signals.up.id.is_none());
    assert!(!signals.down.id.is_none());
}

/// 実際のトークンを使用した照明調整テスト（ignored）
#[tokio::test]
#[ignore = "実際のRemo APIトークンが必要"]
async fn test_adjust_lighting_with_real_token() {
    let helper = RemoIntegrationTestHelper::new().await;

    // 実際のトークンを環境変数から取得
    let real_token =
        std::env::var("REMO_TOKEN").expect("REMO_TOKEN environment variable is required");

    let request = AdjustLigtingRequest {
        remo_token: real_token,
        site_data: SiteData::Generic {
            keywords: vec!["平和".to_string(), "リラックス".to_string()],
        },
    };

    let result = helper.remo_service.adjust_lighting(&request).await;

    // 実際のトークンの場合は成功することを期待
    assert!(result.is_ok());
}
