# Proposal: update-install-section

## 概要

README のインストールセクションおよび関連メタデータが、フォーク元（CodeZeno）のものを指したままになっている。自分のリポジトリ・リリースに向けて更新する。

## 背景・動機

フォーク後のリネーム作業（Ai-Usage-Monitor）はすでに完了しているが、以下がフォーク元のままのため、ユーザーが誤ったリリースページからダウンロードしてしまう可能性がある：

- `README.md` のインストールコマンド（`winget install CodeZeno.ClaudeCodeUsageMonitor`）
- `README.md` のリリースページリンク（`github.com/CodeZeno/...`）
- `README.md` の使い方セクションの起動説明
- `Cargo.toml` の `repository`・`homepage`・`CompanyName` 等のメタデータ

## ゴール

1. README のインストールセクションを自分の GitHub Releases からのダウンロード手順のみに書き換える
2. `Cargo.toml` のメタデータを自分のリポジトリ情報に更新する

## スコープ外

- WinGet 対応（別途検討しない）
- インストーラー形式への変更（別起票で扱う）
- 実行ファイル名・コマンド名の変更（`ai-usage-monitor` のまま維持）
