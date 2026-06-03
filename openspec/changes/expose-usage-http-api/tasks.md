## 1. 依存関係の追加

- [ ] 1.1 `Cargo.toml` に `axum 0.8.x`、`tokio`（rt-multi-thread, macros）、`toml` を追加する
- [ ] 1.2 `cargo build` でコンパイルエラーがないことを確認する

## 2. 設定ファイルの実装

- [ ] 2.1 `src/config.rs` を新規作成し、`AppConfig` 構造体（`server.enabled`, `server.port`）と `load()` 関数を実装する（ファイル不在・パース失敗時はデフォルト値にフォールバック）
- [ ] 2.2 `src/main.rs` で `config::load()` を呼び出し、起動時に設定を読み込む

## 3. 共有状態の実装

- [ ] 3.1 `src/state.rs` を新規作成し、`SharedState = Arc<RwLock<Option<AppUsageData>>>` 型エイリアスを定義する
- [ ] 3.2 `models.rs` の `AppUsageData` に `#[derive(Serialize)]` を追加し、JSON シリアライズを有効にする
- [ ] 3.3 `AppUsageData` に `polled_at: Option<String>` フィールドを追加する（ISO 8601 文字列）

## 4. HTTP サーバーの実装

- [ ] 4.1 `src/api/` ディレクトリと `src/api/mod.rs` を作成する
- [ ] 4.2 `src/api/server.rs` を作成し、axum ルーター（`GET /usage`）を実装する
- [ ] 4.3 `GET /usage` ハンドラーが `SharedState` を read して JSON レスポンスを返すよう実装する
- [ ] 4.4 `src/main.rs` で `std::thread::spawn` + Tokio Runtime を使って HTTP サーバーを起動する（`127.0.0.1:{port}` バインド）
- [ ] 4.5 ポート競合時（`TcpListener::bind` 失敗）に `diagnose::log` してサーバーなしで継続するエラーハンドリングを追加する
- [ ] 4.6 `AppConfig.server.enabled = false` のときはサーバーを起動しないよう分岐する

## 5. 既存コードへの接続

- [ ] 5.1 `src/window.rs` の `window::run()` シグネチャに `SharedState` を追加し、ポーリング成功時に `shared_state.write()` で結果を書き込む
- [ ] 5.2 `src/main.rs` で `SharedState::default()` を生成し、サーバースレッドと `window::run()` の両方に `Arc::clone` して渡す

## 6. 動作確認

- [ ] 6.1 `cargo build --release` でコンパイルが成功することを確認する
- [ ] 6.2 アプリを起動し `curl http://localhost:8765/usage` が JSON を返すことを手動確認する
- [ ] 6.3 `config.toml` に `enabled = false` を設定して起動し、ポートがリッスンされないことを確認する
- [ ] 6.4 使用中のポートを設定してポート競合エラーのフォールバックが機能することを確認する
