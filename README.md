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

### サーバー側 (`server/`)
- `server/crates/atmos/` - メインサーバーバイナリ
- `server/crates/atmos_server/` - コアビジネスロジックとドメインモデル
- `server/crates/remo_api/` - Nature Remo APIクライアント
- `server/crates/atmos_dict/` - エラーハンドリングとドメインモデル

### クライアント側 (`client/atmos_extension/`)
- `client/atmos_extension/src/api/` - 生成されたAPIクライアントと型定義
- `client/atmos_extension/src/entrypoints/` - 拡張機能のエントリーポイント

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
