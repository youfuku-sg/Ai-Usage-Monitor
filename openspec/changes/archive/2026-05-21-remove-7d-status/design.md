# Design: remove-7d-status

## 変更対象ファイル

### 1. `src/models.rs`

`UsageData` から `weekly` フィールドを削除する。

```rust
// 変更前
pub struct UsageData {
    pub session: Option<UsageSection>,
    pub weekly: Option<UsageSection>,
}

// 変更後
pub struct UsageData {
    pub session: Option<UsageSection>,
}
```

### 2. `src/poller.rs`

| 箇所 | 変更内容 |
|------|---------|
| `UsageResponse` struct (L35) | `seven_day` フィールドを削除 |
| `try_usage_endpoint()` (L553) | `weekly` への代入を削除 |
| `parse_rate_limit_headers()` (L611) | 7d ヘッダーの読み取りを削除 |
| `format_line()` (L1024) | weekly のフォーマット処理を削除 |

### 3. `src/window.rs`

| 箇所 | 変更内容 |
|------|---------|
| ツールチップ (L297, L309) | `7d: {weekly}` の部分を削除 |
| ウィジェット描画 (L1432–1441) | weekly 行の描画ブロックを削除 |
| `refresh_usage_texts()` (L410–435) | `weekly_text` の更新処理を削除 |

### 4. `src/localization/*.rs`（全言語）

`weekly_window` フィールドを削除する。

## 実装上の注意

- `weekly_text` を参照している箇所をすべて洗い出してコンパイルエラーがないことを確認する
- `UsageData::weekly` を参照している箇所も同様に確認する
- ウィジェットのレイアウト（高さ・行数）が 7d 削除後に崩れないか確認する
