# Design: add-codex-line

## 変更対象ファイル

### `src/models.rs`

Codex 用の使用量データ構造を追加する。Claude の `UsageData` に相当するものを Codex 向けに定義する（または共通化する）。

### `src/poller.rs`（または新規 `src/codex_poller.rs`）

Codex 使用量 API から数時間ウィンドウの使用率・リセット時刻を取得する。  
アクセス不可の場合は `None` を返し、ウィジェット側で行を非表示にする。

### `src/window.rs`

```
// 変更後のウィジェット表示イメージ
claude: 40% · 15:30
codex:  30% · 18:45   ← アクセス可能な場合のみ表示
```

- 行数に応じてウィジェットの高さを動的に計算する
- Codex が `None` の場合は 2 行目を描画しない

### `src/localization/*.rs`（全言語）

```rust
codex_label: "codex:",
```

## 実装上の注意

- `claude:` と `codex:` のラベル幅を揃えて数値の縦位置が整列するようにする
- Codex のアクセス可否はポーリングのたびに再判定し、契約後にアプリ再起動なしで表示が出るようにする
- 数時間ウィンドウのみ表示（7d 相当は表示しない）
