## ADDED Requirements

### Requirement: config.toml で HTTP サーバー設定を上書きできる

システムは `%APPDATA%\Claude Code Usage Monitor\config.toml` を起動時に読み込み、HTTP サーバーの動作を制御しなければならない（SHALL）。ファイルが存在しない場合はデフォルト値を使用しなければならない（SHALL）。

設定ファイルの形式:
```toml
[server]
enabled = true   # デフォルト: true
port = 8765      # デフォルト: 8765
```

#### Scenario: config.toml が存在しない場合
- **WHEN** `config.toml` がディレクトリに存在しない状態でアプリが起動した
- **THEN** デフォルト値（enabled=true, port=8765）でサーバーが起動する

#### Scenario: カスタムポートが設定されている場合
- **WHEN** `config.toml` に `[server]\nport = 9000` が記述されている状態でアプリが起動した
- **THEN** `localhost:9000` でサーバーがリッスンする

#### Scenario: サーバーが無効化されている場合
- **WHEN** `config.toml` に `[server]\nenabled = false` が記述されている状態でアプリが起動した
- **THEN** HTTP サーバーは起動せず、トレイアイコン・ウィンドウは正常に機能する

### Requirement: config.toml のパース失敗はアプリ起動を妨げない

`config.toml` の構文エラーや不正な値が存在する場合、システムはエラーをログに記録しデフォルト値にフォールバックしなければならない（SHALL）。アプリの起動を中断してはならない（SHALL NOT）。

#### Scenario: config.toml に構文エラーがある場合
- **WHEN** `config.toml` が不正な TOML 構文を含む状態でアプリが起動した
- **THEN** diagnose ログにパースエラーを記録し、デフォルト設定でアプリが起動する
