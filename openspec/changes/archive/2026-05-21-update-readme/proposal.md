## Why

現在の README はフォーク元（Claude Code Usage Monitor）のものがそのまま残っており、プロジェクト名・機能・インストール方法・言語がすべて異なっている。AiPulseHub として公開するにあたり、実態に合った日本語の README に書き直す必要がある。

## What Changes

- README.md を全面的に書き直す
  - プロジェクト名を AiPulseHub に変更
  - 言語を日本語に変更
  - 機能説明を AiPulseHub の仕様に合わせて更新（REST API・閾値通知・ウィジェット）
  - フォーク元固有の記述を削除（WinGet インストール・Codex 機能・7日ウィンドウ・多言語設定）
  - インストール方法を GitHub Releases からの exe 直接ダウンロードに変更
  - プライバシー・セキュリティの記述を AiPulseHub の実装に合わせて更新

## Capabilities

### New Capabilities

- `readme`: AiPulseHub の README コンテンツ（構成・記載内容の要件）

### Modified Capabilities

（なし）

## Impact

- 変更ファイル: `README.md` のみ
- ソースコードへの変更なし
- GitHub にも同期されるファイルのため、公開内容として適切な記述が必要

## Non-goals

- 英語版 README の作成
- スクリーンショット・GIF の更新（画像素材が未準備のため別 change で対応）
- CHANGELOG.md・LICENSE の変更
