# CLAUDE.md — Ai-Usage-Monitor プロジェクト設定

Claude Code がこのリポジトリで作業する際に常に従うルールを記載する。

---

## ブランチ戦略

詳細は `docs/仕様書/ブランチ戦略.md` を参照。以下は必須ルールの要約。

### ブランチ構成

| ブランチ | 分岐元 | 役割 |
|---------|-------|------|
| `main` | — | リリース済み安定版。タグ付きコミットのみ |
| `develop` | — | 開発統合。次リリースの最新 |
| `feature/{openspecの改修タイトル}` | `develop` | 機能追加・バグ修正 |
| `chore/{内容}` | `develop` | 設定・依存関係・ビルドのみの変更 |
| `release/v{version}` | `main` | リリース準備 |
| `hotfix/v{version}` | `main` | 本番緊急修正 |
| `gitea-private` | — | docs/ / openspec/ / .claude/ 等の内部資料専用 |

### 必須ルール

- **main への直接 push 禁止** — 必ず `release/*` または `hotfix/*` 経由
- **develop への直接 push 禁止** — 必ず `feature/*` または `chore/*` 経由（緊急除く）
- **マージは必ず `--no-ff`** — マージコミットを残してブランチ履歴を明確にする
- **feature のリリースフロー** — `main` から `release/*` を切り、そこへ `feature/*` をマージしてからリリース手順を実行する。`feature/*` を直接 `main` へマージしない
- **feature ブランチは削除しない** — マージ後も履歴として残す。削除するのは `release/*` のみ
- **ドキュメント変更は feature に混ぜない** — `docs/` / `openspec/` 等の変更は `gitea-private` に直接コミットする
- **タグは main のみ** — バージョンタグは必ず `main` のマージコミットに打つ
- **feature のブランチ名** — openspec の `changes/` 配下のディレクトリ名と必ず一致させる

### リリースフロー（feature のリリース）

```
1. main から release ブランチを切る
   git checkout -b release/v{version} main

2. feature をマージ
   git merge --no-ff feature/{name}

3. Cargo.toml のバージョン更新 + CHANGELOG.md 更新
   git commit -m "chore: release v{version}"

4. main へマージ
   git checkout main && git merge --no-ff release/v{version}

5. タグを打つ
   git tag v{version} && git push gitea main --tags

6. develop へマージバック
   git checkout develop && git merge --no-ff release/v{version}

7. release ブランチを削除（feature は残す）
   git branch -d release/v{version}
```

### コミットメッセージ（Conventional Commits）

`feat:` / `fix:` / `chore:` / `docs:` / `refactor:` / `test:`

---

## リモート構成

| リモート | 用途 | 同期ブランチ |
|---------|------|------------|
| `gitea` | 開発の中心・全データ保管 | 全ブランチ |
| `github` | CI/CD・リリース専用 | `main` / `develop` のみ（自動同期） |
