## Context

現在のアーキテクチャは Win32 メッセージループ（メインスレッド）とポーリングスレッドの2スレッド構成で、共有状態は存在しない。ポーリング結果は `WM_APP` メッセージ経由で直接 UI スレッドに渡されており、外部プロセスがアクセスできる I/O 境界がない。

HTTP サーバーを追加するには、以下の課題を解決する必要がある：
1. **共有状態**: ポーリング結果をサーバースレッドと UI スレッドが両方参照できる形で保持する
2. **スレッドモデルの拡張**: Tokio ランタイムを Win32 メッセージループと共存させる
3. **既存コードへの侵襲を最小化**: `window.rs` / `poller.rs` の大規模改修を避ける

## Goals / Non-Goals

**Goals:**
- `GET /usage` で JSON レスポンスを返すローカル HTTP サーバーを実装する
- ポーリング結果を `Arc<RwLock<AppState>>` で共有し、サーバーと UI の両方から参照できるようにする
- `config.toml` でポート番号を変更可能にする
- ポート競合時もアプリが正常起動できるようにする（サーバーなしで継続）

**Non-Goals:**
- HTTPS、認証、CORS 設定
- `/usage` 以外のエンドポイント（将来拡張は別 change で対応）
- Tokio への全面移行（`ureq` による同期ポーリングは維持）

## Decisions

### D1: HTTP サーバーに axum を採用する

**採用理由**: Rust の非同期 HTTP エコシステムで最も実績があり、Tokio との統合がネイティブ。`tiny-http` や `actix-web` も検討したが、前者は機能が薄く将来拡張コストが高い、後者は独自の Actor モデルで Tokio との混在が複雑になる。

**代替案**: `tiny-http`（同期、依存が軽量）→ 将来の拡張性を優先して不採用。

### D2: Tokio ランタイムを専用スレッドで起動する

Win32 メッセージループはメインスレッドを占有するため、`tokio::main` は使えない。`std::thread::spawn` で別スレッドを立て、その中で `Runtime::new().block_on(...)` する。これにより既存の `window::run()` エントリポイントを変更せずに済む。

```
main()
  ├─ SharedState を生成
  ├─ thread::spawn → tokio Runtime → axum serve
  └─ window::run(shared_state)  // Win32 メッセージループ（ブロッキング）
```

### D3: 共有状態は `Arc<RwLock<AppUsageData>>` で表現する

ポーリング結果の型 `AppUsageData` はすでに `models.rs` に定義済みのため、それをそのまま `RwLock` で包む。書き込みはポーリングスレッドのみ、読み取りはサーバーと UI の両方から行う（読み取り頻度が高い想定）。

```rust
pub type SharedState = Arc<RwLock<Option<AppUsageData>>>;
```

`None` は「まだポーリング未実施」を表す。

### D4: `window.rs` への変更は `SharedState` の受け渡しのみに限定する

`window::run()` のシグネチャに `SharedState` を追加し、ポーリング成功時に `shared_state.write()` で書き込む。UI 描画ロジックは `shared_state.read()` から取得するよう変更する。Win32 固有処理（メッセージポンプ、トレイ操作）はそのまま維持する。

### D5: `config.toml` は `%APPDATA%\Claude Code Usage Monitor\config.toml` に配置する

既存の認証情報探索ロジックが `dirs::data_local_dir()` を使用しているため、同一ディレクトリに揃える。ファイルが存在しない場合はデフォルト値を使用し、エラーにはしない。

```toml
[server]
enabled = true
port = 8765
```

## Risks / Trade-offs

| リスク | 緩和策 |
|--------|--------|
| ポート 8765 が使用中 | `TcpListener::bind` エラーを `diagnose::log` に記録し、サーバーなしで起動継続 |
| `RwLock` の write 競合による UI 遅延 | write はポーリング完了後の一度のみ（30秒ごと）。UI の read は非ブロッキングで十分 |
| axum + Tokio によるバイナリサイズ増加 | release プロファイルの `opt-level = "z"` + `lto = true` で最小化。許容範囲（+2 MB 程度）と判断 |
| `config.toml` パース失敗 | `toml` クレートのデシリアライズエラーをログ記録してデフォルト値にフォールバック |

## Migration Plan

1. feature ブランチ `feature/expose-usage-http-api` を `develop` から切る
2. 依存関係追加 → 共有状態 → サーバー実装 → 既存コード接続 の順で実装（tasks.md 参照）
3. ローカルで `curl localhost:8765/usage` が JSON を返すことを手動確認
4. develop へ PR マージ → release フローへ

**ロールバック**: feature ブランチを revert するだけ。`config.toml` が存在しない環境では動作変化なし。

## Open Questions

- レスポンス JSON のフィールド名・構造は ClawdMeter 側の期待する形式と合わせるべきか？（現時点では `AppUsageData` の既存フィールドをそのまま返す方針）
- `toml` クレートを追加するか、手動パースで済ませるか（シンプルな設定なら手動でも可）
