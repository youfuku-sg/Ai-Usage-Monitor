#!/bin/bash
set -e

if [ -z "$GITEA_TOKEN" ] || [ -z "$GITEA_USER" ]; then
    echo "GITEA_TOKEN / GITEA_USER が未設定のため Gitea ログイン設定をスキップします"
    exit 0
fi

GITEA_URL="${GITEA_URL:-http://unas-250509.local:3000}"

# tea ログイン設定
tea login add \
    --url "$GITEA_URL" \
    --user "$GITEA_USER" \
    --token "$GITEA_TOKEN" \
    --name gitea 2>/dev/null && echo "tea: Gitea ログイン設定完了" || echo "tea: ログインは既に設定済み"

# git credential store に Gitea トークンを登録
git config --global credential.helper store
GITEA_HOST=$(echo "$GITEA_URL" | sed 's|https\?://||')
CRED_LINE="http://${GITEA_USER}:${GITEA_TOKEN}@${GITEA_HOST}"
if ! grep -qF "$CRED_LINE" ~/.git-credentials 2>/dev/null; then
    echo "$CRED_LINE" >> ~/.git-credentials
    echo "git: Gitea 認証情報を登録しました"
fi
