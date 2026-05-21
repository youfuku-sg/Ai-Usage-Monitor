## Context

Gitea をプライマリリポジトリ（全データ保管庫）、GitHub を CI/CD・リリース専用として運用する。  
Gitea の act_runner は Ubuntu コンテナで動作するため Windows バイナリのビルドができない。  
そのため、コード検証は Gitea で、Windows ビルド・リリースは GitHub で行う役割分担とする。

GitHub にはコード以外（`docs/` `openspec/` `.claude/`）を含めないため、  
Gitea → GitHub の同期時にこれらを除外するフィルタリングが必要。

## Goals / Non-Goals

**Goals:**
- Gitea push をトリガーに `cargo check / test / clippy` を実行する
- main / develop への push 時に GitHub へコードのみを同期する
- `v*` タグ push 時に GitHub Actions で Windows バイナリをビルド・リリースを作成する

**Non-Goals:**
- NSIS インストーラ・自動アップデート用 `latest.json` の生成（別 change で対応）
- Gitea 上での Windows ビルド（act_runner の制約により不可）

## Decisions

### 同期方式: filter-branch によるディレクトリ除外

**決定**: `git filter-branch` で `docs/` `openspec/` `.claude/` を除外した一時ブランチを作り、GitHub へ force-push する。

**理由**: Gitea のプッシュミラー機能は全ファイルをそのまま同期するため除外ができない。  
filter-branch であればワークフロー内で柔軟に対象ディレクトリを制御できる。

**代替案**: `git-filter-repo`（より高速・安全）  
→ act_runner の Ubuntu イメージに `git-filter-repo` が入っていない場合のインストールが必要なため、  
　まず標準の `filter-branch` で実装し、問題があれば移行する。

### GitHub Actions トリガー: タグ push のみ

**決定**: GitHub Actions のビルドワークフローは `v*` タグ push のみをトリガーとする。

**理由**: コード同期は Gitea → GitHub の force-push で行われるため、  
GitHub 側で通常の push をトリガーにすると不要なビルドが走る。  
リリース成果物が必要なのはタグ時のみ。

### バイナリ形式: exe 単体（暫定）

**決定**: 初期は `cargo build --release` の出力 exe をそのまま GitHub Release にアップロードする。

**理由**: NSIS インストーラや自動アップデートの仕組みは別 change で整備する。  
まず CI/CD パイプラインを動かすことを優先する。

## Risks / Trade-offs

- `filter-branch` の force-push → GitHub 側のブランチ履歴が Gitea と異なる  
  → 運用上問題なし（GitHub はコード参照・ビルド専用のため履歴の整合性は不要）

- `GITHUB_MIRROR_TOKEN` の権限漏洩 → GitHub への push が可能なトークンのため管理注意  
  → Gitea のシークレットとして保存。最小権限（対象リポジトリの `contents: write` のみ）で発行する

- act_runner が落ちていると同期が止まる → GitHub 側が古いコードのままになる  
  → 影響範囲は CI/CD のみ。手動で push する手順を `開発フロー.md` に記載する
