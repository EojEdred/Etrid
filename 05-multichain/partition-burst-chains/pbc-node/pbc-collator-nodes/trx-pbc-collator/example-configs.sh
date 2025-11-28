#!/bin/bash
# Example configurations for TRX PBC Collator with DETR P2P

# =============================================================================
# EXAMPLE 1: Local Development (Single Machine)
# =============================================================================

# Node 1 (Bootstrap)
./trx-pbc-collator \
  --dev \
  --p2p-listen "127.0.0.1:30333" \
  --pbc-id 0

# Node 2 (Connect to bootstrap)
./trx-pbc-collator \
  --dev \
  --p2p-listen "127.0.0.1:30334" \
  --p2p-bootstrap "127.0.0.1:30333" \
  --pbc-id 1

# Node 3 (Multi-bootstrap)
./trx-pbc-collator \
  --dev \
  --p2p-listen "127.0.0.1:30335" \
  --p2p-bootstrap "127.0.0.1:30333,127.0.0.1:30334" \
  --pbc-id 2

# =============================================================================
# EXAMPLE 2: Cloud Deployment (Auto IP Detection)
# =============================================================================

# Automatic public IP detection via STUN
./trx-pbc-collator \
  --p2p-listen "0.0.0.0:30333" \
  --p2p-bootstrap "203.0.113.10:30333,203.0.113.20:30333" \
  --relay-chain-rpc "ws://primearc-relay:9944"

# =============================================================================
# EXAMPLE 3: Cloud Deployment (Explicit Announce IP)
# =============================================================================

# Using environment variable
export DETR_P2P_ANNOUNCE_IP="203.0.113.42"
./trx-pbc-collator \
  --p2p-listen "0.0.0.0:30333" \
  --p2p-bootstrap "203.0.113.10:30333"

# Or using CLI flag
./trx-pbc-collator \
  --p2p-listen "0.0.0.0:30333" \
  --p2p-announce "203.0.113.42:30333" \
  --p2p-bootstrap "203.0.113.10:30333"

# =============================================================================
# EXAMPLE 4: Behind NAT/Firewall
# =============================================================================

# Port forward external:30333 -> internal:30333, then:
export DETR_P2P_ANNOUNCE_IP="<YOUR_PUBLIC_IP>"
./trx-pbc-collator \
  --p2p-listen "0.0.0.0:30333" \
  --p2p-bootstrap "203.0.113.10:30333"

# =============================================================================
# EXAMPLE 5: Testnet Deployment
# =============================================================================

./trx-pbc-collator \
  --chain local \
  --p2p-listen "0.0.0.0:30333" \
  --p2p-announce "$(curl -s https://api.ipify.org):30333" \
  --p2p-bootstrap "testnet-boot1.etrid.org:30333,testnet-boot2.etrid.org:30333" \
  --relay-chain-rpc "ws://testnet-relay.etrid.org:9944" \
  --rpc-cors all \
  --rpc-methods=unsafe

# =============================================================================
# EXAMPLE 6: Production Validator
# =============================================================================

export DETR_P2P_ANNOUNCE_IP="<YOUR_PUBLIC_IP>"

./trx-pbc-collator \
  --chain mainnet \
  --validator \
  --p2p-listen "0.0.0.0:30333" \
  --p2p-bootstrap "boot1.etrid.org:30333,boot2.etrid.org:30333,boot3.etrid.org:30333" \
  --relay-chain-rpc "wss://relay.etrid.org:443" \
  --rpc-port 9933 \
  --ws-port 9944 \
  --prometheus-external \
  --prometheus-port 9615 \
  --name "TRX-Validator-$(hostname)" \
  --telemetry-url 'wss://telemetry.etrid.org/submit 0'

# =============================================================================
# EXAMPLE 7: Disable P2P (Substrate networking only)
# =============================================================================

./trx-pbc-collator \
  --dev \
  --enable-p2p false

# =============================================================================
# EXAMPLE 8: Custom Port Configuration
# =============================================================================

./trx-pbc-collator \
  --p2p-listen "0.0.0.0:40000" \
  --p2p-announce "203.0.113.42:40000" \
  --p2p-bootstrap "203.0.113.10:40000" \
  --port 30334 \
  --rpc-port 9934 \
  --ws-port 9945

# =============================================================================
# EXAMPLE 9: Docker Deployment
# =============================================================================

docker run -d \
  --name trx-pbc-collator \
  -p 30333:30333 \
  -p 9933:9933 \
  -p 9944:9944 \
  -e DETR_P2P_ANNOUNCE_IP="$(curl -s https://api.ipify.org)" \
  etrid/trx-pbc-collator:latest \
  --p2p-listen "0.0.0.0:30333" \
  --p2p-bootstrap "boot1.etrid.org:30333" \
  --relay-chain-rpc "wss://relay.etrid.org:443"

# =============================================================================
# EXAMPLE 10: Systemd Service
# =============================================================================

# Create /etc/systemd/system/trx-pbc-collator.service:
cat > /tmp/trx-pbc-collator.service << 'EOF'
[Unit]
Description=TRX PBC Collator
After=network.target

[Service]
Type=simple
User=etrid
WorkingDirectory=/opt/etrid
Environment="DETR_P2P_ANNOUNCE_IP=YOUR_PUBLIC_IP"
ExecStart=/opt/etrid/trx-pbc-collator \
  --chain mainnet \
  --p2p-listen "0.0.0.0:30333" \
  --p2p-bootstrap "boot1.etrid.org:30333,boot2.etrid.org:30333" \
  --relay-chain-rpc "wss://relay.etrid.org:443" \
  --base-path /var/lib/etrid/trx-pbc
Restart=always
RestartSec=10
LimitNOFILE=65536

[Install]
WantedBy=multi-user.target
EOF

# Then:
# sudo mv /tmp/trx-pbc-collator.service /etc/systemd/system/
# sudo systemctl daemon-reload
# sudo systemctl enable trx-pbc-collator
# sudo systemctl start trx-pbc-collator
# sudo journalctl -u trx-pbc-collator -f

# =============================================================================
# TROUBLESHOOTING COMMANDS
# =============================================================================

# Check if P2P port is open
nc -zv <your-ip> 30333

# Test STUN connectivity
nc -u -v stun.l.google.com 19302

# Get your public IP
curl https://api.ipify.org

# Monitor P2P logs
tail -f /var/log/trx-pbc-collator.log | grep -E "DETR|P2P|peer"

# Check connected peers (if RPC is enabled)
curl -H "Content-Type: application/json" \
  -d '{"id":1, "jsonrpc":"2.0", "method":"system_peers"}' \
  http://localhost:9933/
