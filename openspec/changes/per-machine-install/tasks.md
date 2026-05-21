## 1. wix/main.wxs の変更

- [ ] 1.1 `<Package InstallScope='perUser'>` を削除して per-machine に変更する
- [ ] 1.2 インストール先を `LocalAppDataFolder` から `ProgramFiles64Folder` に変更する
- [ ] 1.3 `binary0` コンポーネントを file KeyPath に戻す（HKCU registry KeyPath の workaround を削除）
- [ ] 1.4 `RemoveFolder` / `RegistryValue`（workaround 分）を `binary0` から削除する

## 2. updater.rs の変更

- [ ] 2.1 MSI インストール判定ロジックを実装する（レジストリで判定 or インストールパスで判定）
- [ ] 2.2 MSI インストール時の更新フローを実装する（msi をダウンロードして ShellExecute で実行）
- [ ] 2.3 ポータブル版の既存更新フローが壊れていないことを確認する

## 3. 動作確認

- [ ] 3.1 `cargo wix` で MSI がビルドできることを確認する
- [ ] 3.2 MSI を実行して `%ProgramFiles%\AiUsageMonitor\` にインストールされることを確認する
- [ ] 3.3 「アプリと機能」に表示されることを確認する
- [ ] 3.4 「アプリと機能」からアンインストールできることを確認する
- [ ] 3.5 MSI インストール版でアップデート確認が動き、UAC 経由で更新が適用されることを確認する
- [ ] 3.6 ポータブル版の自動更新が引き続き動くことを確認する
