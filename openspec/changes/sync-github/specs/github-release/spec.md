## ADDED Requirements

### Requirement: タグトリガーによるビルド
GitHub Actions は `v*` パターンのタグ push をトリガーに Windows x86_64 バイナリをビルドしなければならない（SHALL）。

#### Scenario: v* タグ push でビルドが開始される
- **WHEN** `v1.0.0` のような `v*` パターンのタグが GitHub へ push された
- **THEN** `windows-latest` ランナーで `cargo build --release` が実行される

#### Scenario: 通常の push ではビルドが実行されない
- **WHEN** タグを伴わない通常の push が GitHub へ行われた
- **THEN** ビルドワークフローは実行されない

### Requirement: GitHub Release の自動作成
ビルド成功後、タグに対応する GitHub Release を作成し成果物をアップロードしなければならない（SHALL）。

#### Scenario: ビルド成功時に Release が作成される
- **WHEN** `cargo build --release` が成功した
- **THEN** タグ名に対応する GitHub Release が作成され、`AiPulseHub.exe` がアップロードされる

#### Scenario: ビルド失敗時に Release が作成されない
- **WHEN** `cargo build --release` がエラーで終了した
- **THEN** GitHub Release は作成されない

### Requirement: ビルド時シークレットの注入
アップデーター用のエンドポイント・トークンはビルド時に環境変数として注入しなければならない（SHALL）。

#### Scenario: シークレットがビルドに注入される
- **WHEN** GitHub Actions がビルドを実行する
- **THEN** `GITEA_UPDATE_TOKEN` `GITEA_UPDATE_ENDPOINT` `GITHUB_UPDATE_TOKEN` `GITHUB_UPDATE_ENDPOINT` が環境変数としてセットされた状態で `cargo build` が実行される
