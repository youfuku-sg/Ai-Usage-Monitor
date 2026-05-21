# Design: per-machine-install

## 変更対象ファイル

### 1. `wix/main.wxs`

| 項目 | 変更前 | 変更後 |
|------|--------|--------|
| `InstallScope` | `perUser` | 削除（デフォルトで per-machine） |
| インストール先 | `LocalAppDataFolder\AiUsageMonitor\` | `ProgramFiles64Folder\AiUsageMonitor\` |
| `binary0` の KeyPath | HKCU レジストリ（workaround） | ファイル（`KeyPath='yes'`）に戻す |
| `binary0` の `RemoveFolder` | あり（LOCALAPPDATA 用） | 削除 |
| `binary0` の `RegistryValue` | HKCU 用 KeyPath | 削除 |
| `ApplicationShortcut` の `RegistryValue` | HKCU | HKCU のまま（ショートカットは per-user で問題なし） |

```xml
<!-- 変更後の binary0 コンポーネント -->
<Component Id='binary0' Guid='AA90DAF9-E342-40FD-80A6-3AAFBFD77769'>
    <File
        Id='exe0'
        Name='ai-usage-monitor.exe'
        DiskId='1'
        Source='target\release\claude-code-usage-monitor.exe'
        KeyPath='yes'/>
</Component>
```

インストール先ディレクトリ：
```xml
<Directory Id='ProgramFiles64Folder'>
    <Directory Id='APPLICATIONFOLDER' Name='AiUsageMonitor'>
```

### 2. `src/updater.rs`

インストール種別（MSI per-machine / ポータブル）を判定し、更新方式を切り替える。

**判定方法**: レジストリ `HKLM\Software\Microsoft\Windows\CurrentVersion\Uninstall\` 配下に本アプリのエントリが存在するかで判定する。

**MSI インストール時の更新フロー**:
```
1. GitHub Releases から最新の .msi をダウンロード（既存の exe ダウンロードと同様）
2. ShellExecute で msi を実行（UAC プロンプトが出る）
3. ユーザーが「はい」を押すとインストーラーが起動し更新完了
```

**ポータブル版の更新フロー**: 既存の exe 自己置換をそのまま維持。

## 実装上の注意

- `%ProgramFiles%` へのインストールは管理者権限が必要（UAC が出る）
- `ProgramFiles64Folder` を使うことで 64bit パスに確実にインストールする
- ポータブル版との判定を確実に行い、ポータブル版の更新動作を壊さないこと
