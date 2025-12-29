#!/usr/bin/env bash
export AGENT_MAIL_PROJECT='/Users/macbook/Desktop/etrid-workspace/etrid'
export AGENT_MAIL_AGENT='RedLake'
export AGENT_MAIL_URL='http://127.0.0.1:8765/mcp/'
export AGENT_MAIL_INTERVAL='120'
exec '/Users/macbook/Desktop/etrid-workspace/etrid/.codex/hooks/notify_inbox.sh' "$@"
