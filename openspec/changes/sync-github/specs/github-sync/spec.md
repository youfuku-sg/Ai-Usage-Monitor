## ADDED Requirements

### Requirement: Gitea 専用ディレクトリの除外
GitHub への同期時に `docs/` `openspec/` `.claude/` を除外しなければならない（SHALL）。これらのディレクトリは Gitea のみで管理する。

#### Scenario: main ブランチへの push 時に同期が実行される
- **WHEN** main ブランチへ push が行われた
- **THEN** `docs/` `openspec/` `.claude/` を除外したコードのみが GitHub の main ブランチへ同期される

#### Scenario: develop ブランチへの push 時に同期が実行される
- **WHEN** develop ブランチへ push が行われた
- **THEN** `docs/` `openspec/` `.claude/` を除外したコードのみが GitHub の develop ブランチへ同期される

#### Scenario: 除外対象ディレクトリが GitHub に存在しない
- **WHEN** 同期が完了した
- **THEN** GitHub 上のリポジトリに `docs/` `openspec/` `.claude/` が存在しない

### Requirement: 同期の認証
GitHub への push には `GITHUB_MIRROR_TOKEN` シークレットを使用しなければならない（SHALL）。

#### Scenario: トークンが設定されている場合に同期が成功する
- **WHEN** Gitea の `GITHUB_MIRROR_TOKEN` シークレットが有効な PAT に設定されている
- **THEN** GitHub への force-push が成功する

#### Scenario: トークンが未設定の場合に同期が失敗する
- **WHEN** `GITHUB_MIRROR_TOKEN` シークレットが空または未設定である
- **THEN** ワークフローが認証エラーで失敗する
