## ADDED Requirements

### Requirement: コード検証ワークフロー
Gitea Actions は push および PR のたびに `cargo check` / `cargo test` / `cargo clippy` を実行し、コードの品質を検証しなければならない（SHALL）。

#### Scenario: push 時に検証が実行される
- **WHEN** 任意のブランチへ push が行われた
- **THEN** `cargo check` / `cargo test` / `cargo clippy` が順に実行される

#### Scenario: clippy に警告があればワークフローが失敗する
- **WHEN** `cargo clippy -- -D warnings` が警告を検出した
- **THEN** ワークフローが non-zero で終了し、Gitea 上でエラーが表示される

#### Scenario: 全ステップが通過すればワークフローが成功する
- **WHEN** check / test / clippy がすべてゼロで終了した
- **THEN** ワークフローが成功ステータスで完了する

### Requirement: Cargo キャッシュ
ワークフローは `~/.cargo/registry` と `target/` をキャッシュし、2回目以降のビルド時間を短縮しなければならない（SHALL）。

#### Scenario: キャッシュが存在する場合に再利用される
- **WHEN** 同一 `Cargo.lock` ハッシュのキャッシュが存在する
- **THEN** キャッシュが復元され、依存クレートのダウンロードがスキップされる
