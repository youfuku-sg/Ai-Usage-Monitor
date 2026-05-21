## ADDED Requirements

### Requirement: プロジェクト名と概要
README の冒頭はプロジェクト名 `AiPulseHub` で始まり、バッジ（platform: Windows / license: MIT）と1〜2文の概要を記載しなければならない（SHALL）。概要は「Claude Code の使用量をバックグラウンドで収集し、REST API・トースト通知・タスクバーウィジェットで提供するローカルハブ」という趣旨を含むこと。

#### Scenario: プロジェクト名が正しい
- **WHEN** README.md を開いた
- **THEN** 見出しに `AiPulseHub` と表示される

#### Scenario: フォーク元の名称が含まれない
- **WHEN** README.md を参照した
- **THEN** `Claude Code Usage Monitor` という文字列が README に存在しない

### Requirement: 機能一覧
README は AiPulseHub が提供する以下の機能を列挙しなければならない（SHALL）。

- タスクバーウィジェット（1行: Claude使用率・リセット時刻）
- タスクトレイアイコン（使用率バッジ・右クリックメニュー）
- REST API（`GET /status` `GET /status/claude` `GET /health`、ポート 8765）
- トースト通知（使用率が 80/90/95/98% に達した時点）
- 自動アップデート（Gitea 優先・GitHub フォールバック）

#### Scenario: AiPulseHub 固有の機能が記載されている
- **WHEN** 機能一覧セクションを参照した
- **THEN** REST API・閾値通知（80/90/95/98%）・ウィジェットが記載されている

#### Scenario: 廃止された機能が記載されていない
- **WHEN** README.md を参照した
- **THEN** 7日ウィンドウ・WinGet・Codex 機能・多言語設定への言及が存在しない

### Requirement: 動作環境
README は動作環境として Windows 11・Claude Code 認証済み・WSL 対応を記載しなければならない（SHALL）。

#### Scenario: 動作環境が記載されている
- **WHEN** 動作環境セクションを参照した
- **THEN** Windows 11・Claude Code 認証情報・WSL サポートが記載されている

### Requirement: インストール手順
README は GitHub Releases からの exe 直接ダウンロードによるインストール方法を記載しなければならない（SHALL）。WinGet の手順を記載してはならない（SHALL NOT）。

#### Scenario: 正しいインストール手順が記載されている
- **WHEN** インストールセクションを参照した
- **THEN** GitHub Releases ページへのリンクと exe のダウンロード手順が記載されている

#### Scenario: WinGet の手順が含まれない
- **WHEN** README.md を参照した
- **THEN** `winget` コマンドが存在しない

### Requirement: REST API の説明
README は REST API のエンドポイント一覧（パス・概要）を記載しなければならない（SHALL）。

#### Scenario: エンドポイント一覧が記載されている
- **WHEN** REST API セクションを参照した
- **THEN** `GET /status` `GET /status/claude` `GET /health` とその概要が記載されている

### Requirement: プライバシーとセキュリティ
README はアプリが読み取る情報・送信する情報・保存する情報を明記しなければならない（SHALL）。

#### Scenario: 読み取り・送信・保存の情報が記載されている
- **WHEN** プライバシーセクションを参照した
- **THEN** 認証情報ファイルのパス・Anthropic API への通信・ローカル保存内容が記載されている

### Requirement: 言語
README 全体を日本語で記述しなければならない（SHALL）。コードブロック・コマンド・URL は除く。

#### Scenario: 日本語で記述されている
- **WHEN** README.md を参照した
- **THEN** 本文の説明文が日本語で書かれている
