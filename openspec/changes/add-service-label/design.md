# Design: add-service-label

## 変更対象ファイル

### `src/window.rs`

Claude 行の描画テキストの先頭に `strings.claude_label` を追加する。

```
// 変更前
"40% · 3h 20m"

// 変更後
"claude: 40% · 3h 20m"
```

ラベル部分は固定幅（例: 8文字）でパディングし、複数サービスが並んだときに数値が縦に揃うようにする。

### `src/localization/*.rs`（全言語）

```rust
// 追加
claude_label: "claude:",
```

## 実装上の注意

- ラベル幅は `codex:` と揃えること（7文字）
- ウィジェット全体の幅が足りなくなる場合は横幅を調整する
