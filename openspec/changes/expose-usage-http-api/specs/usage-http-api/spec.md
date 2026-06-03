## ADDED Requirements

### Requirement: GET /usage エンドポイントを提供する

システムは `GET http://localhost:{port}/usage` に対して、直近のポーリング結果を JSON で返さなければならない（SHALL）。レスポンスの Content-Type は `application/json` でなければならない（SHALL）。

レスポンス例:
```json
{
  "claude_code": {
    "session": {
      "percentage": 42.5,
      "resets_at": "2026-06-03T14:00:00+00:00"
    }
  },
  "codex": null,
  "polled_at": "2026-06-03T09:30:00+00:00"
}
```

#### Scenario: ポーリング済みの状態でリクエストされた場合
- **WHEN** アプリが起動後少なくとも1回ポーリングを完了した状態で `GET /usage` が呼ばれた
- **THEN** HTTP 200 と現在の使用量 JSON を返す

#### Scenario: まだポーリング未実施の状態でリクエストされた場合
- **WHEN** アプリ起動直後（初回ポーリング完了前）に `GET /usage` が呼ばれた
- **THEN** HTTP 200 と `{"claude_code": null, "codex": null, "polled_at": null}` を返す

#### Scenario: ポーリングがエラーだった場合
- **WHEN** 直近のポーリングが認証エラーまたはネットワーク障害で失敗した後に `GET /usage` が呼ばれた
- **THEN** HTTP 200 と直前の成功データを返す（データがない場合は null フィールド）

### Requirement: サーバーはポート競合時にも起動を継続する

ポートが使用中でバインドできない場合、システムはアプリの起動を中断してはならない（SHALL NOT）。エラーをログに記録した上で、HTTP サーバーなしで動作を継続しなければならない（SHALL）。

#### Scenario: 指定ポートが使用中の場合
- **WHEN** アプリ起動時に設定ポートがすでに別プロセスに使用されている
- **THEN** diagnose ログにエラーを記録し、トレイアイコン・ウィンドウは正常に機能する

### Requirement: サーバーはローカルホストのみバインドする

システムは `127.0.0.1:{port}` のみにバインドしなければならない（SHALL）。外部ネットワークインターフェースに対してリッスンしてはならない（SHALL NOT）。

#### Scenario: 外部からのアクセス
- **WHEN** 別マシンから `http://{local-ip}:{port}/usage` にアクセスが試みられた
- **THEN** 接続が拒否される（バインドされていない）
