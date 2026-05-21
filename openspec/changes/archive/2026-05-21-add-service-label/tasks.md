## 事前確認

- [x] 0.1 `claude-single-line` が完了済みであることを確認する

## 1. ラベル追加

- [x] 1.1 `src/window.rs` の Claude 行描画に `claude:` プレフィックスを追加する
- [x] 1.2 ラベル幅を固定し、将来の `codex:` 行と縦位置が揃うようにする

## 2. ローカライズ対応

- [x] 2.1 `src/localization/*.rs` に `claude_label: "claude:"` を追加する（全言語）

## 3. 動作確認

- [x] 3.1 `cargo build` が通ることを確認する
- [x] 3.2 ウィジェットに `claude: XX% · ...` の形式で表示されることを確認する
