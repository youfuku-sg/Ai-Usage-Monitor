## Why

Gitea を全データの保管庫・開発の中心とし、GitHub を CI/CD・リリース専用として役割を分担する。現状はリポジトリ構成が未整備で、Gitea と GitHub の使い分けが定義されていない。

## What Changes

- Gitea Actions ワークフローを2本追加する
  - `ci.yml`: push / PR のたびに `cargo check / test / clippy` を実行
  - `sync-github.yml`: main / develop への push 時に `docs/` `openspec/` `.claude/` を除外した上で GitHub へ同期
- GitHub Actions ワークフローを1本追加する
  - `build.yml`: `v*` タグ push 時に Windows バイナリをビルド・リリース作成
- `config.toml` のサンプルを追加する（設定ファイルの雛形として）

## Capabilities

### New Capabilities

- `gitea-ci`: Gitea 上でのコード検証（check / test / clippy）
- `github-sync`: docs/ / openspec/ / .claude/ を除外して GitHub へ同期する Gitea Actions ワークフロー
- `github-release`: v* タグトリガーで Windows バイナリをビルド・GitHub Release を作成する GitHub Actions ワークフロー

### Modified Capabilities

（なし）

## Impact

- 追加ファイル: `.gitea/workflows/ci.yml`, `.gitea/workflows/sync-github.yml`, `.github/workflows/build.yml`
- Gitea・GitHub それぞれにシークレットの登録が必要（`GITHUB_MIRROR_TOKEN` 等）
- 既存ソースコードへの変更なし

## Non-goals

- Tauri / NSIS インストーラのパッケージング（ビルド成果物は exe 単体で暫定対応）
- 自動アップデート用 `latest.json` の生成（別の change で対応）
- Codex 関連の CI 対応
