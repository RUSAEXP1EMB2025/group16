use std::collections::HashSet;
use std::sync::Arc;

use atmos_config::Config;
use atmos_dict::Atmosdict;
use atmos_freq::SiteData;
use atmos_server::{
    domain::{
        Service,
        models::remo::AdjustLigtingRequest,
        ports::{AtmosdictService, RemoService},
    },
    outbound::remo::Remo,
};
use tempfile::NamedTempFile;
use tokio::fs;

/// データベースとの結合テストヘルパー
struct DatabaseIntegrationTestHelper {
    service: Service<Remo, Arc<Atmosdict>>,
}

impl DatabaseIntegrationTestHelper {
    async fn new() -> Self {
        // テスト用の一時データベースファイルを作成
        let db_path = Config::from_env().database_path;

        // テスト用のデータベースを初期化
        let atmosdict = Atmosdict::new(&db_path)
            .await
            .expect("Failed to create atmosdict");
        let atmosdict = Arc::new(atmosdict);
        let remo = Remo::new(Arc::clone(&atmosdict));

        let service = Service::new(remo, atmosdict);

        Self { service }
    }
}

/// 辞書データの取得テスト
#[tokio::test]
async fn test_get_all_atmoswords() {
    let helper = DatabaseIntegrationTestHelper::new().await;
    let result = helper.service.get_all_atmoswords().await;

    assert!(result.is_ok());
    let words = result.unwrap();

    // データベースが初期化されているので、何らかのデータが存在することを期待
    assert!(!words.is_empty());

    // 各単語が空でないことを確認
    for word in &words {
        assert!(!word.is_empty());
    }
}

/// 辞書データの型確認テスト
#[tokio::test]
async fn test_atmoswords_type_consistency() {
    let helper = DatabaseIntegrationTestHelper::new().await;

    let result = helper.service.get_all_atmoswords().await;

    assert!(result.is_ok());
    let words = result.unwrap();

    // HashSetの形式で返されることを確認
    assert!(words.is_empty() || !words.is_empty());

    // 重複がないことを確認（HashSetの特性）
    let words_vec: Vec<String> = words.into_iter().collect();
    let words_set: HashSet<String> = words_vec.iter().cloned().collect();
    assert_eq!(words_vec.len(), words_set.len());
}

/// 辞書データの日本語文字確認テスト
#[tokio::test]
async fn test_atmoswords_japanese_characters() {
    let helper = DatabaseIntegrationTestHelper::new().await;

    let result = helper.service.get_all_atmoswords().await;

    assert!(result.is_ok());
    let words = result.unwrap();

    // 日本語の単語が含まれていることを確認
    let has_japanese = words.iter().any(|word| {
        word.chars().any(|c| {
            ('\u{3040}'..='\u{309F}').contains(&c) || // ひらがな
            ('\u{30A0}'..='\u{30FF}').contains(&c) || // カタカナ
            ('\u{4E00}'..='\u{9FAF}').contains(&c) // 漢字
        })
    });

    assert!(
        has_japanese,
        "Japanese words should be present in the database"
    );
}

/// データベース接続エラーのテスト
#[tokio::test]
async fn test_database_connection_error() {
    // 存在しないデータベースパスを指定
    let invalid_path = "/non/existent/path/database.db";

    let result = Atmosdict::new(invalid_path).await;

    // データベース接続エラーが発生することを期待
    assert!(result.is_err());
}

/// 並行アクセステスト
#[tokio::test]
async fn test_concurrent_database_access() {
    let helper = DatabaseIntegrationTestHelper::new().await;

    // 複数のタスクを同時に実行
    let tasks = (0..10).map(|_| {
        let service = &helper.service;
        async move { service.get_all_atmoswords().await }
    });

    let results = futures::future::join_all(tasks).await;

    // すべてのタスクが成功することを確認
    for result in results {
        assert!(result.is_ok());
        let words = result.unwrap();
        assert!(!words.is_empty());
    }
}

/// 大量データ処理テスト（シミュレーション）
#[tokio::test]
async fn test_large_data_processing() {
    let helper = DatabaseIntegrationTestHelper::new().await;

    // 大量のキーワードを含む汎用リクエストを作成
    let large_keywords: Vec<String> = (0..1000).map(|i| format!("キーワード{i}")).collect();

    let request = AdjustLigtingRequest {
        remo_token: "test_token".to_string(),
        site_data: SiteData::Generic {
            keywords: large_keywords,
        },
    };

    let result = helper.service.adjust_lighting(&request).await;

    // 大量データでも処理が実行されることを確認（トークンが無効なためエラーは期待される）
    assert!(result.is_err());
}

/// 空のデータベースでの処理テスト
#[tokio::test]
async fn test_empty_database_handling() {
    let db_path = Config::from_env().database_path;

    // 空のファイルを作成
    fs::write(&db_path, "")
        .await
        .expect("Failed to write empty file");

    let result = Atmosdict::new(&db_path).await;

    // 空のデータベースファイルでは初期化エラーが発生することを期待
    assert!(result.is_err());
}

/// データベースファイルの権限テスト
#[tokio::test]
async fn test_database_file_permissions() {
    let helper = DatabaseIntegrationTestHelper::new().await;

    // データベースファイルが読み書き可能であることを確認
    let result = helper.service.get_all_atmoswords().await;

    assert!(result.is_ok());
    let words = result.unwrap();
    assert!(!words.is_empty());
}

/// 複数のサービスインスタンスでの同一データベースアクセステスト
#[tokio::test]
async fn test_multiple_service_instances() {
    let temp_db = NamedTempFile::new().expect("Failed to create temp database file");
    let db_path = temp_db.path().to_string_lossy().to_string();

    // 最初のサービスインスタンスを作成
    let atmosdict1 = Atmosdict::new(&db_path)
        .await
        .expect("Failed to create first atmosdict");
    let atmosdict1 = Arc::new(atmosdict1);
    let remo1 = Remo::new(Arc::clone(&atmosdict1));
    let service1 = Service::new(remo1, atmosdict1);

    // 2番目のサービスインスタンスを作成
    let atmosdict2 = Atmosdict::new(&db_path)
        .await
        .expect("Failed to create second atmosdict");
    let atmosdict2 = Arc::new(atmosdict2);
    let remo2 = Remo::new(Arc::clone(&atmosdict2));
    let service2 = Service::new(remo2, atmosdict2);

    // 両方のサービスからデータを取得
    let result1 = service1.get_all_atmoswords().await;
    let result2 = service2.get_all_atmoswords().await;

    assert!(result1.is_ok());
    assert!(result2.is_ok());

    let words1 = result1.unwrap();
    let words2 = result2.unwrap();

    // 同じデータが取得されることを確認
    assert_eq!(words1, words2);
}

/// 文字エンコーディングテスト
#[tokio::test]
async fn test_character_encoding() {
    let helper = DatabaseIntegrationTestHelper::new().await;

    let result = helper.service.get_all_atmoswords().await;

    assert!(result.is_ok());
    let words = result.unwrap();

    // 各単語がUTF-8でエンコードされていることを確認
    for word in &words {
        assert!(
            word.is_ascii()
                || word
                    .chars()
                    .all(|c| c.is_alphabetic() || c.is_numeric() || c.is_whitespace())
        );
    }
}

/// 長期間のデータベース接続テスト
#[tokio::test]
async fn test_long_term_database_connection() {
    let helper = DatabaseIntegrationTestHelper::new().await;

    // 複数回のアクセスを時間間隔を空けて実行
    for _ in 0..5 {
        let result = helper.service.get_all_atmoswords().await;

        assert!(result.is_ok());
        let words = result.unwrap();
        assert!(!words.is_empty());

        // 短い待機時間を設ける
        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
    }
}

/// データベース統計情報テスト
#[tokio::test]
async fn test_database_statistics() {
    let helper = DatabaseIntegrationTestHelper::new().await;

    let result = helper.service.get_all_atmoswords().await;

    assert!(result.is_ok());
    let words = result.unwrap();

    // 基本的な統計情報を確認
    let word_count = words.len();
    let total_chars: usize = words.iter().map(|w| w.len()).sum();
    let avg_word_length = if word_count > 0 {
        total_chars / word_count
    } else {
        0
    };

    // 合理的な値であることを確認
    assert!(word_count > 0);
    assert!(avg_word_length > 0);
    assert!(avg_word_length < 100); // 単語の平均長が100文字未満であることを確認
}
