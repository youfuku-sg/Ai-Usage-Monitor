## 1. 変更

- [x] 1.1 `src/window.rs` の `update_check_interval()` を `Duration::from_secs(60 * 60)`（1時間）に変更する

## 2. 動作確認

- [x] 2.1 `cargo build` が通ることを確認する
- [x] 2.2 起動から 1 時間後にアップデートチェックが走ることを確認する
