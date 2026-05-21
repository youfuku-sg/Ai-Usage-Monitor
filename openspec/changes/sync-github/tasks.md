## 1. 事前準備（シークレット登録）

- [ ] 1.1 Gitea の `GITHUB_MIRROR_TOKEN` シークレットを登録する（GitHub PAT、対象リポジトリの `contents: write` 権限）
- [ ] 1.2 Gitea の `GITHUB_REPO_URL` シークレットを登録する（例: `github.com/{owner}/AiPulseHub.git`）
- [ ] 1.3 GitHub の `GITEA_UPDATE_TOKEN` / `GITEA_UPDATE_ENDPOINT` / `GITHUB_UPDATE_TOKEN` / `GITHUB_UPDATE_ENDPOINT` シークレットを登録する

## 2. Gitea CI ワークフロー

- [x] 2.1 `.gitea/workflows/ci.yml` を作成する（cargo check / test / clippy、Cargo キャッシュ付き）
- [ ] 2.2 Gitea に push して ci.yml が正常に動作することを確認する

## 3. GitHub 同期ワークフロー

- [x] 3.1 `.gitea/workflows/sync-github.yml` を作成する（main / develop push 時に docs/ openspec/ .claude/ を除外して GitHub へ force-push）
- [ ] 3.2 develop ブランチへ push し、GitHub に docs/ openspec/ .claude/ が含まれないことを確認する
- [ ] 3.3 main ブランチへ push し、GitHub に同様に同期されることを確認する

## 4. GitHub リリースワークフロー

- [x] 4.1 `.github/workflows/build.yml` を作成する（v* タグトリガー、windows-latest で cargo build --release、GitHub Release 作成）
- [ ] 4.2 テスト用タグ（例: `v0.0.1-test`）を push し、GitHub Actions が起動してビルド・Release が作成されることを確認する
- [ ] 4.3 テスト用タグ・Release を削除してクリーンアップする
