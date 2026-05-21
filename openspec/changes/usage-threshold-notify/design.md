# Design: usage-threshold-notify

## 閾値チェックのタイミング

ポーリング完了後（`WM_APP_POLL_COMPLETE` 受信時）に使用率を閾値と比較する。

## 閾値の管理

`settings.json` に以下を追加する：

```json
{
  "notify_thresholds": [80, 90, 95, 98]
}
```

デフォルト値は `[80, 90, 95, 98]`。空配列にすると通知なし。

## 既発火状態の管理

`AppState` に `notified_thresholds: Vec<u8>` を追加する。  
5h ウィンドウのリセット後（`is_past_reset` が true → false に変わったとき）にクリアする。

## 通知フォーマット

```
タイトル: Claude Code 使用率アラート
本文:    使用率が 80% に達しました。リセット予定: 15:30
```

リセット時刻は HH:MM 形式（ローカル時刻）。時刻が不明な場合は省略。

## 変更対象ファイル

### `src/settings.rs`（または設定読み書き箇所）

`notify_thresholds: Vec<u8>` フィールドを追加。デフォルト `[80, 90, 95, 98]`。

### `src/window.rs`

- `AppState` に `notified_thresholds: Vec<u8>` を追加
- ポーリング完了ハンドラに閾値チェック処理を追加
- 閾値超過時に `tray_icon::notify_balloon()` でトースト通知を送出
- ウィンドウリセット検出時に `notified_thresholds` をクリア

### `src/localization/*.rs`（全言語）

通知用の文字列を追加：
```rust
notify_threshold_title: "Claude Code 使用率アラート",
notify_threshold_body: "使用率が {pct}% に達しました。リセット予定: {time}",
```

## 実装上の注意

- 閾値は昇順にソートして低い方から順にチェックする
- 既に `notified_thresholds` に含まれる閾値は通知しない
- 現在の使用率以下のすべての閾値を一度に登録する（例: 85% のとき 80% を登録）
