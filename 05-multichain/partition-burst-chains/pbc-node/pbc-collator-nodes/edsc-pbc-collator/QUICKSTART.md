# EDSC-PBC Collator Quick Start Guide

Quick reference for starting the EDSC collator with DETR P2P networking.

## Prerequisites

- Rust toolchain installed
- EDSC-PBC collator built (`cargo build --release`)
- Public IP address (for production deployments)
- Bootstrap peer information (optional)

## Quick Start Commands

### 1. Development Mode (Single Node)

```bash
export DETR_P2P_ANNOUNCE_IP=127.0.0.1

./target/release/edsc-pbc-collator \
  --dev \
  --p2p-bind-address 0.0.0.0:30333
```

### 2. Production Mode with Auto-Detection

```bash
./start-edsc-collator.sh
```

### 3. Production Mode with Manual Configuration

```bash
export DETR_P2P_ANNOUNCE_IP=203.0.113.42
export BOOTSTRAP_PEERS="a1b2c3d4...@203.0.113.100:30333,e5f6g7h8...@203.0.113.101:30333"

./target/release/edsc-pbc-collator \
  --name my-edsc-collator \
  --base-path ./data \
  --chain local \
  --pbc-id 12 \
  --relay-chain-rpc ws://relay-node:9944 \
  --p2p-bind-address 0.0.0.0:30333 \
  --p2p-enable-maintenance true \
  --validator
```

## Environment Variables

| Variable | Description | Example |
|----------|-------------|---------|
| `DETR_P2P_ANNOUNCE_IP` | Your public IP address | `203.0.113.42` |
| `DETR_P2P_BIND_ADDRESS` | Address to bind to | `0.0.0.0:30333` |
| `DETR_P2P_BOOTSTRAP_PEERS` | Bootstrap peer list | `abc@1.2.3.4:30333,def@5.6.7.8:30333` |
| `VALIDATOR_PUBLIC_KEY` | Validator public key | `5GrwvaEF5zXb...` |

## CLI Options

| Option | Default | Description |
|--------|---------|-------------|
| `--p2p-bind-address` | `0.0.0.0:30333` | Address to listen on |
| `--p2p-announce-address` | Auto-detected | Public address to announce |
| `--p2p-bootstrap-peers` | None | Comma-separated bootstrap peers |
| `--p2p-enable-maintenance` | `true` | Enable P2P maintenance tasks |
| `--pbc-id` | `12` | PBC ID for EDSC |
| `--relay-chain-rpc` | `ws://127.0.0.1:9944` | Relay chain RPC endpoint |

## Verification

After starting, look for these log messages:

```
✅ P2P configuration validated successfully
  🔌 Bind address: 0.0.0.0:30333
  📢 Announce address: 203.0.113.42:30333
  👥 Bootstrap peers: 2
🚀 Starting DETR P2P network listener...
✅ DETR P2P network started successfully
✅ P2P maintenance tasks started (DHT, auto-reconnect, discovery)
🌐 DETR P2P network fully initialized and operational
```

## Getting Your Node ID

Your node ID will be printed during startup:

```
🔑 Generated P2P node ID from seed: edsc-pbc-12-hostname
🆔 Local Node ID: PeerId([...])
```

To share with others for bootstrap peers:

```
<your_node_id>@<your_public_ip>:30333
```

## Troubleshooting

### Issue: "Announce address is 0.0.0.0"

**Solution:**
```bash
export DETR_P2P_ANNOUNCE_IP=your.public.ip.address
```

### Issue: "Could not auto-detect public IP"

**Solution:** Manually set announce IP via environment variable or CLI

### Issue: Node not discovering peers

**Check:**
1. Bootstrap peers are correct and reachable
2. Firewall allows TCP port 30333
3. Public IP is correctly configured

## Multi-Node Setup

### Node 1 (Bootstrap Node)

```bash
export DETR_P2P_ANNOUNCE_IP=203.0.113.100

./target/release/edsc-pbc-collator \
  --name edsc-node-1 \
  --base-path ./data/node1 \
  --p2p-bind-address 0.0.0.0:30333
```

Save the node ID from logs, then:

### Node 2 (Connect to Bootstrap)

```bash
export DETR_P2P_ANNOUNCE_IP=203.0.113.101
export BOOTSTRAP_PEERS="<node1_id>@203.0.113.100:30333"

./target/release/edsc-pbc-collator \
  --name edsc-node-2 \
  --base-path ./data/node2 \
  --p2p-bind-address 0.0.0.0:30333
```

## Production Deployment

1. Set environment variables in your system/container
2. Use systemd service or Docker container
3. Monitor logs for P2P health
4. Ensure firewall rules allow P2P port
5. Use static public IP or set DETR_P2P_ANNOUNCE_IP

### Example systemd Service

```ini
[Unit]
Description=EDSC-PBC Collator
After=network.target

[Service]
Type=simple
User=etrid
Environment="DETR_P2P_ANNOUNCE_IP=203.0.113.42"
Environment="BOOTSTRAP_PEERS=abc@1.2.3.4:30333"
ExecStart=/opt/etrid/edsc-pbc-collator \
  --name edsc-prod-1 \
  --base-path /var/lib/etrid/edsc \
  --validator \
  --p2p-bind-address 0.0.0.0:30333
Restart=always
RestartSec=10

[Install]
WantedBy=multi-user.target
```

## Documentation

- **Full Integration Guide:** [DETR_P2P_INTEGRATION.md](./DETR_P2P_INTEGRATION.md)
- **DETR P2P Library:** `/Users/macbook/Desktop/etrid/01-detr-p2p/detrp2p`
- **Architecture:** See integration documentation

## Support

- Issues: GitHub repository
- Community: Ëtrid Discord server
- Email: dev@etrid.org
