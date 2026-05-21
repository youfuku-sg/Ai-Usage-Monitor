# Changelog

All notable changes to this project will be documented in this file.
The format is based on [Keep a Changelog](https://keepachangelog.com/ja/1.1.0/).

## [0.0.5] - 2026-05-21

### Fixed
- MSI ビルドエラーを修正（WiX CNDL0230: ファイル＋レジストリ KeyPath を持つコンポーネントに明示的 GUID を指定）

## [0.0.4] - 2026-05-21

### Fixed
- MSI ビルドエラーを修正（WiX ICE38/ICE64: perUser + LOCALAPPDATA インストール時の KeyPath をレジストリキーに変更）

## [0.0.3] - 2026-05-21

### Added
- MSI インストーラーを追加（`%LOCALAPPDATA%` インストール、スタートメニューショートカット付き）
- GitHub Actions でポータブル exe と MSI の両方をビルドして Releases に添付

## [0.0.2] - 2026-05-21

### Changed
- インストールセクションを GitHub Releases からのダウンロード手順のみに更新
- 使い方セクションを exe 直接実行の説明に整合
- Cargo.toml のリポジトリ・著作権メタデータを自分のリポジトリに更新

## [0.0.1] - 2026-05-21

### Changed
- 日本語 README の初版（プロジェクト概要・はじめに・フォーク元クレジット）
- プロジェクト名を Ai-Usage-Monitor に変更
