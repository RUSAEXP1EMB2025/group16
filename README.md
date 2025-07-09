※ Claudeはコミットメッセージの自動生成に使用しています。

# Group16: Atmos - インテリジェント雰囲気照明制御システム

Atmosは、Webコンテンツを解析して自動的にNature Remoスマートホームデバイス経由で室内照明を調整するインテリジェント雰囲気照明制御システムです。

## プロダクト概要

このシステムは、ブラウザ拡張機能がWebページからコンテンツを抽出し、Rustバックエンドサーバーがそのコンテンツを処理して「雰囲気指数」を計算し、照明を制御します。

## アーキテクチャ

このプロジェクトは**ヘキサゴナルアーキテクチャ**（ドメイン駆動設計）に従います：

- **ドメイン層**: 雰囲気計算のコアビジネスロジック（`AtmosFreq`、`AdjustLightingRequest`）
- **インバウンドポート**: HTTP APIエンドポイント（`GET/POST /lighting`）
- **アウトバウンドポート**: カスタム`remo_api`クレートによるNature Remo API統合
- **技術スタック**: Rust（Axum）バックエンド、Svelte 5 + WXT拡張機能、TypeScript全般

## モジュール構成

### サーバー側 (Rust)
- `server/crates/atmos/` - メインサーバーバイナリ
- `server/crates/atmos_server/` - コアビジネスロジックとドメインモデル
  - `src/domain/` - ドメインモデル、ポート、サービス定義
  - `src/inbound/http/routes/` - HTTP APIルートハンドラー
  - `src/outbound/` - リポジトリ実装
- `server/crates/remo_api/` - Nature Remo APIクライアント（生成済み）
- `server/crates/atmos_dict/` - 辞書機能とエラーハンドリング
- `server/crates/atmos_freq/` - 雰囲気指数計算ロジック
- `server/crates/atmos_config/` - 設定管理

### クライアント側 (拡張機能)
- `client/atmos_extension/src/api/` - 生成されたAPIクライアントと型定義
- `client/atmos_extension/src/entrypoints/` - 拡張機能のエントリーポイント
  - `background.ts` - バックグラウンドスクリプト
  - `content.ts` - コンテンツスクリプト
  - `popup/` - ポップアップUIコンポーネント
- `client/atmos_extension/src/lib/` - 共有Svelteコンポーネント

### データベース・インフラ
- `db/` - データベースセットアップとマイグレーション
- `taskfile/` - タスク自動化設定
- `docs/` - プロジェクトドキュメントと設計ファイル

## 起動方法

### 1. Taskfileを使用したローカル開発

開発環境の起動:
```bash
task dev
```

個別のコンポーネント起動:
```bash
# 拡張機能のみ
task extension:dev

# サーバーのみ
task server:dev
```

### 2. Dockerを使用した起動

Docker環境での起動:
```bash
task up
```

Docker環境の停止:
```bash
task down
```

## 開発用コマンド

### コード生成
```bash
task gen  # TypeScript型とAPIクライアントを生成
```

### テスト実行
```bash
task server:test  # サーバーテスト実行
```

### フォーマット・リント
```bash
task extension:format-w  # 拡張機能コードフォーマット
task extension:lint-w    # 拡張機能リント
task extension:check-w   # 拡張機能フルチェック
```

## 技術スタック

- **バックエンド**: Rust, Axum, Tokio
- **フロントエンド**: Svelte 5, TypeScript, WXT
- **パッケージ管理**: pnpm（拡張機能）, Cargo（Rust）
- **タスクランナー**: Taskfile
- **API**: OpenAPI仕様, 型安全なクライアント生成
