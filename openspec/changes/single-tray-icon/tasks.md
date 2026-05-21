## 1. tray_icon_data_from_state() の変更

- [ ] 1.1 `src/window.rs` の `tray_icon_data_from_state()` を、常に `TrayIconKind::Claude` / `percent: None` の1件のみを返すよう変更する
- [ ] 1.2 ツールチップを有効なモデルの使用率をまとめた内容にする

## 2. 通知の統一

- [ ] 2.1 Codex の認証エラー通知を `TrayIconKind::Claude`（単一アイコン）経由に変更する

## 3. 動作確認

- [ ] 3.1 `cargo build` が通ることを確認する
- [ ] 3.2 Claude Code のみ有効時にアイコンが1つ表示されることを確認する
- [ ] 3.3 Claude Code + Codex 両方有効時もアイコンが1つのみであることを確認する
- [ ] 3.4 バルーン通知が引き続き動作することを確認する
