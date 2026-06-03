## Why

外部ツール（ClawdMeter など）が Claude Code の使用量をリアルタイムで取得できるよう、本アプリが収集した使用量データを localhost の REST エンドポイントで公開する。現状、データはトレイアイコン・ウィンドウ表示にしか使われておらず、外部プロセスが参照する手段が存在しない。

## What Changes

- axum を依存関係に追加し、Tokio ランタイム上で HTTP サーバー（デフォルトポート 8765）を起動する
- 収集済みの使用量データを保持する共有状態（`Arc<RwLock<AppState>>`）を導入する
- `GET /usage` エンドポイントを実装し、現在の使用量データを JSON で返す
- コレクタースレッドがポーリング結果を共有状態に書き込むよう変更する
- `config.toml` でポート番号・サーバー有効/無効を設定可能にする

**Non-goals:**
- 認証・認可（ローカル専用のため不要）
- HTTPS 対応
- 書き込み系エンドポイント（POST / PUT / DELETE）
- Codex 以外の AI サービス向けエンドポイントの追加

## Capabilities

### New Capabilities

- `usage-http-api`: `GET /usage` エンドポイントで Claude Code および Codex の使用量データを JSON 配信する
- `app-config`: `config.toml` による起動時設定（HTTP ポート・サーバー有効化フラグ）

### Modified Capabilities

（なし）

## Impact

- **依存関係追加**: `axum 0.8.x`, `tokio`（features: rt-multi-thread, macros）
- **変更ファイル**: `Cargo.toml`, `src/main.rs`, `src/window.rs`, `src/poller.rs`, `src/models.rs`
- **新規ファイル**: `src/config.rs`, `src/state.rs`, `src/api/server.rs`
- **バイナリサイズ**: axum + Tokio の追加により増加見込み（release ビルドで +1〜2 MB 程度）
- **ポート競合**: 8765 が使用中の場合は起動ログにエラーを記録し、サーバーなしで動作継続する
