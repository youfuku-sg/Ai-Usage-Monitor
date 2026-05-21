# Design: update-check-interval

## 変更対象ファイル

### `src/window.rs` — `update_check_interval()`

```rust
// 変更前
fn update_check_interval() -> Duration {
    Duration::from_secs(24 * 60 * 60)
}

// 変更後
fn update_check_interval() -> Duration {
    Duration::from_secs(60 * 60)
}
```
