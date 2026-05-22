# Design: widen-service-label

## 変更対象ファイル

### `src/window.rs`

`LABEL_WIDTH` 定数を変更する。

```rust
// 変更前
const LABEL_WIDTH: i32 = 18;

// 変更後
const LABEL_WIDTH: i32 = 36;
```

`LABEL_WIDTH` は `total_widget_width_for()` でウィジェット全体の横幅計算に使用されているため、値を変更するだけでウィジェット幅も自動的に広がる。

## 確認ポイント

- `claude:` / `codex:` の7文字が切れずに表示されること
- ウィジェット全体の横幅が適切に広がっていること
- DPI スケール環境（125% / 150%）でも正しく表示されること
