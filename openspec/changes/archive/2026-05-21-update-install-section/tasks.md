## 事前確認（実装前に決める）

- [x] 0.1 GitHub の username を確定する（リリース URL に使用）
- [x] 0.2 `Cargo.toml` に記載する会社名・著作権表示の名義を決める

## 1. README の更新

- [x] 1.1 インストールセクションを GitHub Releases からのダウンロード手順のみに書き換える（`README.md`）
- [x] 1.2 使い方セクションの起動説明を exe 直接実行と整合させる（`README.md`）
- [x] 1.3 フォーク元の Releases リンク（`github.com/CodeZeno/...`）をすべて自分のリポジトリに置換する（`README.md`）

## 2. Cargo.toml メタデータの更新

- [x] 2.1 `repository` を自分のリポジトリ URL に変更する（`Cargo.toml`）
- [x] 2.2 `homepage` を自分のリポジトリ URL に変更する（`Cargo.toml`）
- [x] 2.3 `description` を更新する（`Cargo.toml`）
- [x] 2.4 `[package.metadata.winres]` の `CompanyName`・`ProductName`・`FileDescription`・`LegalCopyright` を更新する（`Cargo.toml`）

## 3. 動作確認

- [x] 3.1 `cargo build` が通ることを確認する
- [x] 3.2 README のリンクがすべて自分のリポジトリを指していることを目視確認する
