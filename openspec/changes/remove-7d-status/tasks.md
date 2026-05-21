## 1. データモデルの変更

- [ ] 1.1 `src/models.rs` の `UsageData` から `weekly` フィールドを削除する

## 2. ポーリング・パース処理の変更

- [ ] 2.1 `src/poller.rs` の `UsageResponse` から `seven_day` フィールドを削除する
- [ ] 2.2 `src/poller.rs` の `try_usage_endpoint()` で `weekly` に代入している箇所を削除する
- [ ] 2.3 `src/poller.rs` の `parse_rate_limit_headers()` で 7d ヘッダーを読み取っている箇所を削除する
- [ ] 2.4 `src/poller.rs` の `format_line()` で weekly をフォーマットしている箇所を削除する

## 3. UI の変更

- [ ] 3.1 `src/window.rs` のツールチップから `7d: {weekly}` の表示を削除する
- [ ] 3.2 `src/window.rs` のウィジェット描画から weekly 行を削除する
- [ ] 3.3 `src/window.rs` の `refresh_usage_texts()` から `weekly_text` の更新処理を削除する

## 4. ローカライズの変更

- [ ] 4.1 `src/localization/` 配下の全言語ファイルから `weekly_window` フィールドを削除する

## 5. 動作確認

- [ ] 5.1 `cargo build` が通ることを確認する
- [ ] 5.2 ウィジェットに 5h のみ表示され、7d が消えていることを確認する
- [ ] 5.3 ツールチップに 7d が表示されないことを確認する
