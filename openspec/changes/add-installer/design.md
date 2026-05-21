# Design: add-installer

## 使用ツール

`cargo-wix`（https://github.com/volks73/cargo-wix）

- `cargo install cargo-wix` でインストール
- `cargo wix init` で `wix/main.wxs` を生成
- `cargo wix` で `.msi` をビルド

## ファイル構成（追加）

```
wix/
  main.wxs          # WiX インストーラー定義ファイル（cargo wix init で生成・カスタマイズ）
.github/
  workflows/
    release.yml     # リリース時に msi + ポータブル exe を両方ビルドして Releases にアップ
```

## `wix/main.wxs` のカスタマイズ項目

| 項目 | 設定内容 |
|------|---------|
| ProductName | `Ai Usage Monitor` |
| Manufacturer | `{自分の名前 or ハンドル}` |
| インストール先 | `%ProgramFiles%\Ai Usage Monitor\` |
| スタートメニュー | ショートカット追加 |
| アンインストール | Add/Remove Programs に登録 |
| `Start with Windows` | インストーラーでは設定しない（アプリ内の右クリックメニューで管理） |

## GitHub Actions リリースフロー

```yaml
# トリガー: main ブランチへの vX.X.X タグ push
# 成果物:
#   - ai-usage-monitor.exe          （ポータブル版）
#   - ai-usage-monitor-{ver}.msi    （インストーラー版）
```

## 自動更新との関係

インストーラー版でインストールした場合、`updater.rs` の `current_install_channel()` は現在 WinGet パス以外を `Portable` として扱う。インストーラー版も同じ `Portable` チャンネルとして扱い、自己更新（GitHub Releases から exe をダウンロードして置き換え）を利用する。

> インストール先が `Program Files` の場合は書き込み権限がないため自己更新が失敗する。インストール先を `%LOCALAPPDATA%` にするか、インストーラー版では自動更新を無効化するかを実装時に判断する。

## README への追記

```markdown
## インストール

**インストーラー版（推奨）**

[Releases](https://github.com/{username}/Ai-Usage-Monitor/releases) ページから最新の `ai-usage-monitor-{version}.msi` をダウンロードして実行します。

**ポータブル版**

`ai-usage-monitor.exe` をダウンロードして任意の場所に配置し、直接実行します。
```
