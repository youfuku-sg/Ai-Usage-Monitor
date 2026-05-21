# Design: update-install-section

## 変更対象ファイル

### 1. `README.md` — インストールセクション

#### 変更前
```markdown
## インストール

WinGet で最新バージョンをインストールします：

```powershell
winget install CodeZeno.ClaudeCodeUsageMonitor
```

WinGet を使わない場合は、[Releases](https://github.com/CodeZeno/Claude-Code-Usage-Monitor/releases) ページから最新の `ai-usage-monitor.exe` をダウンロードして直接実行することもできます。
```

#### 変更後
```markdown
## インストール

[Releases](https://github.com/{username}/Ai-Usage-Monitor/releases) ページから最新の `ai-usage-monitor.exe` をダウンロードして直接実行します。
```

> `{username}` は実装時に確定する。

---

### 2. `README.md` — 使い方セクション

WinGet 前提の起動説明になっているため、ダウンロードした exe を直接実行する形に整合させる。

```markdown
## 使い方

ダウンロードした `ai-usage-monitor.exe` を実行します。または任意のディレクトリに配置して以下のコマンドでも起動できます：

```powershell
ai-usage-monitor
```
```

---

### 3. `Cargo.toml` — パッケージメタデータ

| フィールド | 変更前 | 変更後 |
|-----------|--------|--------|
| `repository` | `https://github.com/CodeZeno/Claude-Code-Usage-Monitor` | `https://github.com/{username}/Ai-Usage-Monitor` |
| `homepage` | `https://codezeno.com.au` | `https://github.com/{username}/Ai-Usage-Monitor` |
| `description` | `Claude Code Usage Monitor` | `Ai-Usage-Monitor` |
| `[package.metadata.winres] CompanyName` | `Code Zeno Pty Ltd` | `{自分の名前 or ハンドル}` |
| `[package.metadata.winres] ProductName` | `Claude Code Usage Monitor` | `Ai Usage Monitor` |
| `[package.metadata.winres] FileDescription` | `Claude Code Usage Monitor` | `Ai Usage Monitor` |
| `[package.metadata.winres] LegalCopyright` | `Copyright (C) 2026 Code Zeno Pty Ltd` | `Copyright (C) 2026 {自分の名前}` |

> `package.name` は `claude-code-usage-monitor` のままでも可（バイナリ名に影響するため慎重に判断する）。

---

## 実装上の注意

- GitHub の username（リポジトリ URL）は実装前にユーザーが確定させる
- `Cargo.toml` の `package.name` を変更する場合、生成バイナリ名も変わるため使い方セクションとの整合確認が必要
- 自動更新は `Cargo.toml` の `repository` から `owner/repo` を導出して GitHub API を叩く仕組みのため、`repository` の更新が自動更新の向き先変更も兼ねる
