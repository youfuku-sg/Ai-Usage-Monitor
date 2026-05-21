# Design: single-tray-icon

## 変更方針

`tray_icon_data_from_state()` が返すアイコンリストを、常に Claude ロゴ1件のみにする。

## 変更対象ファイル

### `src/window.rs` — `tray_icon_data_from_state()`

```rust
// 変更前: モデルごとにアイコンを push
// 変更後: 常に1件のみ返す
fn tray_icon_data_from_state() -> Vec<tray_icon::TrayIconData> {
    vec![tray_icon::TrayIconData {
        kind: tray_icon::TrayIconKind::Claude,
        percent: None,   // ロゴのみ（使用率バッジなし）
        tooltip: /* 現在の使用率情報をツールチップに残す */,
    }]
}
```

`percent: None` にすることで `create_icon` がロゴのみを描画する（既存の分岐を利用）。

ツールチップは有効なモデルの使用率をまとめて表示する（例: `Claude Code 5h: 40%`）。

## 実装上の注意

- `notify_balloon` は `TrayIconKind` を受け取るため、通知は `TrayIconKind::Claude` に統一する
- Codex の認証エラー通知も同じ1つのアイコン経由で出す
