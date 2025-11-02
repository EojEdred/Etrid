#!/bin/bash
# Deploy Node Exporter - Fixed version
set -e

SSH_KEY="$HOME/.ssh/gizzi-validator"

echo "Deploying node exporters to all accessible validators..."
echo ""

# List of validators
VALIDATORS="runtime-dev01@20.224.104.239
compiler-dev01@98.71.91.84
network-dev01@20.169.114.25
sdk-dev01@20.75.92.203
devtools-dev01@20.55.31.30
api-dev01@20.73.34.17
docs-dev01@20.109.102.30
qa-dev01@52.250.61.132
perf-dev01@20.218.66.251
community-dev01@20.109.219.185
analytics-dev01@20.83.208.17
ethics-dev01@172.177.175.132
flarenode16@20.84.231.225
flarenode19@4.175.83.133
flarenode20@52.184.47.99
flarenode21@4.178.181.122"

SUCCESS=0
FAILED=0

for validator in $VALIDATORS; do
    echo "Deploying to $validator..."
    ssh -i "$SSH_KEY" -o StrictHostKeyChecking=no -o ConnectTimeout=10 "$validator" '
if systemctl is-active --quiet node_exporter 2>/dev/null; then
    echo "Already installed"
    exit 0
fi

cd /tmp
wget -q https://github.com/prometheus/node_exporter/releases/download/v1.7.0/node_exporter-1.7.0.linux-amd64.tar.gz || exit 1
tar xzf node_exporter-1.7.0.linux-amd64.tar.gz || exit 1
sudo mv node_exporter-1.7.0.linux-amd64/node_exporter /usr/local/bin/ || exit 1
rm -rf node_exporter-1.7.0.linux-amd64*

sudo tee /etc/systemd/system/node_exporter.service > /dev/null <<EOF
[Unit]
Description=Node Exporter
After=network.target

[Service]
Type=simple
User=nobody
ExecStart=/usr/local/bin/node_exporter --web.listen-address=:9100
Restart=always

[Install]
WantedBy=multi-user.target
EOF

sudo systemctl daemon-reload
sudo systemctl enable node_exporter
sudo systemctl start node_exporter
echo "Installed successfully"
' && {
        echo "✓ Success: $validator"
        SUCCESS=$((SUCCESS + 1))
    } || {
        echo "✗ Failed: $validator"
        FAILED=$((FAILED + 1))
    }
    echo ""
done

echo "========================================"
echo "Deployment Summary:"
echo "Success: $SUCCESS"
echo "Failed: $FAILED"
echo "========================================"
