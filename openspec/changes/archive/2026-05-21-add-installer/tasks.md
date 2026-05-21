## 事前確認（実装前に決める）

- [x] 0.1 インストーラー版のインストール先を決める（`%LOCALAPPDATA%` 推奨 or `%ProgramFiles%`）
- [x] 0.2 インストーラー版での自動更新の扱いを決める（有効にする場合は 0.1 で `%LOCALAPPDATA%` を選ぶ）
- [x] 0.3 `update-install-section` が完了していることを確認する（`Cargo.toml` の `repository` が自分のリポジトリを指している必要がある）

## 1. cargo-wix のセットアップ

- [x] 1.1 `cargo install cargo-wix` を実行してツールをインストールする
- [x] 1.2 `cargo wix init` を実行して `wix/main.wxs` を生成する
- [x] 1.3 `wix/main.wxs` の `ProductName`・`Manufacturer`・インストール先を編集する
- [x] 1.4 スタートメニューショートカットの設定を追加する

## 2. ローカルビルド確認

- [x] 2.1 `cargo wix` で `.msi` がビルドできることを確認する
- [x] 2.2 生成された `.msi` を実行してインストール・アンインストールが正常に動くことを確認する
- [x] 2.3 インストーラー版の自動更新動作を確認する（0.1 の選択に応じて）

## 3. GitHub Actions の設定

- [x] 3.1 `.github/workflows/release.yml` を作成する（タグ push をトリガーにポータブル exe と msi を両方ビルド）
- [x] 3.2 GitHub Actions で msi のビルドが通ることを確認する
- [x] 3.3 Releases に `ai-usage-monitor.exe` と `ai-usage-monitor-{version}.msi` の両方が添付されることを確認する

## 4. README の更新

- [x] 4.1 インストールセクションにインストーラー版とポータブル版の両方の説明を追加する（`README.md`）
