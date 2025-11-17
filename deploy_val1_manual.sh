#!/bin/bash
# Manual deployment to val-1 (146.190.136.56)
# Run these commands one by one

set -e

echo "═══════════════════════════════════════════════════════════"
echo "Step 1: Upload files to val-1"
echo "═══════════════════════════════════════════════════════════"

# Upload binary
scp ./target/release/etrid ubuntu@146.190.136.56:/tmp/etrid
echo "✓ Binary uploaded"

# Upload chainspec
scp ./flarechain_production_raw.json ubuntu@146.190.136.56:/tmp/chainspec.json
echo "✓ Chainspec uploaded"

echo ""
echo "═══════════════════════════════════════════════════════════"
echo "Step 2: SSH into val-1 and run deployment"
echo "═══════════════════════════════════════════════════════════"

ssh ubuntu@146.190.136.56 << 'ENDSSH'
    set -e

    echo "Stopping old flarechain-node..."
    sudo systemctl stop flarechain-node 2>/dev/null || true
    sudo pkill -9 flarechain-node 2>/dev/null || true
    sleep 2

    echo "Installing new etrid binary..."
    sudo mv /tmp/etrid /usr/local/bin/etrid
    sudo chmod +x /usr/local/bin/etrid
    /usr/local/bin/etrid --version

    echo "Installing chainspec..."
    sudo mkdir -p /etc/etrid
    sudo mv /tmp/chainspec.json /etc/etrid/flarechain_production.json
    ls -lh /etc/etrid/flarechain_production.json

    echo "Generating node key..."
    if [ ! -f /etc/etrid/node-key.secret ]; then
        openssl rand -hex 32 | sudo tee /etc/etrid/node-key.secret
        sudo chmod 600 /etc/etrid/node-key.secret
    fi

    echo "Node key: $(sudo cat /etc/etrid/node-key.secret | cut -c1-16)..."

    echo "Backing up old database..."
    if [ -d /var/lib/etrid/chains ]; then
        sudo mv /var/lib/etrid/chains /var/lib/etrid/chains.backup.$(date +%Y%m%d_%H%M%S)
    fi

    echo "Creating systemd service..."
    sudo tee /etc/systemd/system/etrid-validator.service > /dev/null <<EOF
[Unit]
Description=Etrid FlareChain Validator (Pure ASF v108)
After=network.target
Wants=network-online.target

[Service]
Type=simple
User=ubuntu
WorkingDirectory=/var/lib/etrid
ExecStart=/usr/local/bin/etrid \\
  --chain /etc/etrid/flarechain_production.json \\
  --validator \\
  --base-path /var/lib/etrid \\
  --port 30333 \\
  --rpc-port 9944 \\
  --prometheus-port 9615 \\
  --prometheus-external \\
  --node-key \$(cat /etc/etrid/node-key.secret) \\
  --name "FlareChain-Val-1" \\
  --rpc-cors all \\
  --rpc-methods=unsafe \\
  --unsafe-rpc-external

Restart=always
RestartSec=10
LimitNOFILE=65536

StandardOutput=journal
StandardError=journal

[Install]
WantedBy=multi-user.target
EOF

    echo "Starting validator..."
    sudo systemctl daemon-reload
    sudo systemctl enable etrid-validator
    sudo systemctl start etrid-validator

    echo "Waiting 5 seconds for startup..."
    sleep 5

    echo "Checking status..."
    if sudo systemctl is-active --quiet etrid-validator; then
        echo "✓ Validator is RUNNING"
    else
        echo "✗ Validator FAILED to start"
        sudo journalctl -u etrid-validator -n 50 --no-pager
        exit 1
    fi

    echo ""
    echo "Last 30 log lines:"
    sudo journalctl -u etrid-validator -n 30 --no-pager
ENDSSH

echo ""
echo "═══════════════════════════════════════════════════════════"
echo "Step 3: Monitor the validator"
echo "═══════════════════════════════════════════════════════════"
echo ""
echo "Check logs:"
echo "  ssh ubuntu@146.190.136.56 'sudo journalctl -u etrid-validator -f'"
echo ""
echo "Check block production:"
echo "  curl -s -X POST http://146.190.136.56:9944 -H 'Content-Type: application/json' \\"
echo "    -d '{\"id\":1,\"jsonrpc\":\"2.0\",\"method\":\"chain_getHeader\",\"params\":[]}' | jq"
echo ""
echo "Check system health:"
echo "  curl -s -X POST http://146.190.136.56:9944 -H 'Content-Type: application/json' \\"
echo "    -d '{\"id\":1,\"jsonrpc\":\"2.0\",\"method\":\"system_health\",\"params\":[]}' | jq"
echo ""
