

### 2. Dev Container で開く

VS Code でリポジトリを開き、コマンドパレット（`F1`）から：

```
Dev Containers: Reopen in Container
```

コンテナ起動後、以下が自動実行されます：

- `npm install`
- `gh auth setup-git`（git 操作に gh 認証を使用する設定）
- `npm start`（ポート 3005 でブラウザが自動オープン）

### 1. ホストマシンで GitHub CLI 認証（初回のみ）

**コンテナを開く前に**、ホスト側で認証を済ませてください。

```bash
gh auth login
```

対話形式で以下を選択します：

```
? Where do you use GitHub?                            → GitHub.com
? What is your preferred protocol for Git operations? → HTTPS
? How would you like to authenticate GitHub CLI?      → Login with a web browser
```

ブラウザで認証を完了すると、次回以降はコンテナ起動時に自動で引き継がれます。