# XLM-PBC Collator Quick Start Guide

Get your Stellar (XLM) Partition Burst Chain collator running with DETR P2P networking in minutes.

## Prerequisites

- Rust toolchain (stable)
- etrid repository cloned
- DETR P2P libraries compiled

## Build

```bash
cd /Users/macbook/Desktop/etrid/05-multichain/partition-burst-chains/pbc-node/pbc-collator-nodes/xlm-pbc-collator
cargo build --release
```

The binary will be at: `target/release/xlm-pbc-collator`

## Quick Start Commands

### 1. Single Node (Development)

```bash
./target/release/xlm-pbc-collator \
  --chain dev \
  --alice \
  --tmp
```

This starts a development node with:
- Auto-detected public IP
- P2P networking enabled on port 30333
- Temporary database

### 2. Connect Two Nodes Locally

**Terminal 1 - Node Alice:**
```bash
./target/release/xlm-pbc-collator \
  --chain dev \
  --alice \
  --tmp \
  --port 30333 \
  --p2p-bind-address "0.0.0.0:30333"
```

Copy the PeerId from the logs (looks like: `0000...0001`)

**Terminal 2 - Node Bob:**
```bash
./target/release/xlm-pbc-collator \
  --chain dev \
  --bob \
  --tmp \
  --port 30334 \
  --p2p-bind-address "0.0.0.0:30334" \
  --p2p-bootstrap-peers "<ALICE_PEER_ID>@127.0.0.1:30333"
```

Replace `<ALICE_PEER_ID>` with the PeerId from Node Alice.

### 3. Production Node with Public IP

```bash
export DETR_P2P_ANNOUNCE_IP=<YOUR_PUBLIC_IP>

./target/release/xlm-pbc-collator \
  --chain mainnet \
  --validator \
  --name "XLM-Collator-01" \
  --base-path /var/lib/xlm-pbc \
  --p2p-bootstrap-peers "<PEER1_ID>@<PEER1_IP>:30333,<PEER2_ID>@<PEER2_IP>:30333"
```

## Configuration Options

### Essential Flags

| Flag | Description | Default |
|------|-------------|---------|
| `--chain` | Chain specification (dev/local/mainnet) | - |
| `--validator` | Enable validator mode | false |
| `--name` | Node name for telemetry | - |
| `--base-path` | Database directory | `~/.local/share/xlm-pbc-collator` |

### P2P Networking Flags

| Flag | Description | Default |
|------|-------------|---------|
| `--p2p-enabled` | Enable DETR P2P networking | true |
| `--p2p-bind-address` | Address to bind listener | 0.0.0.0:30333 |
| `--p2p-announce-address` | Public address for peers | Auto-detected |
| `--p2p-bootstrap-peers` | Bootstrap peers (comma-separated) | "" |

### Substrate Flags

| Flag | Description | Default |
|------|-------------|---------|
| `--port` | Substrate p2p port | 30333 |
| `--rpc-port` | HTTP RPC port | 9933 |
| `--ws-port` | WebSocket RPC port | 9944 |
| `--tmp` | Use temporary database | false |

## Check Node Status

### View Logs

P2P events are logged with emoji prefixes:

```
🌐 Initializing DETR P2P Network for XLM-PBC
✅ P2P network initialized successfully
🚀 Starting DETR P2P Network Service
🔗 Connecting to bootstrap peers
📢 Announced XLM-PBC block #42 to 5 peers
```

### RPC Queries

Check node info:
```bash
curl -H "Content-Type: application/json" \
  -d '{"jsonrpc":"2.0","method":"system_health","params":[],"id":1}' \
  http://localhost:9933
```

## Common Issues

### Issue: "Could not detect public IP"

**Solution:** Set your public IP manually:
```bash
export DETR_P2P_ANNOUNCE_IP=203.0.113.42
```

### Issue: "Failed to connect to bootstrap peer"

**Solutions:**
1. Verify peer is online: `telnet <peer_ip> <peer_port>`
2. Check peer ID format (64 hex characters)
3. Verify firewall allows outbound connections

### Issue: "No incoming connections"

**Solutions:**
1. Enable port forwarding on your router (TCP 30333)
2. Check firewall: `sudo ufw allow 30333/tcp`
3. Verify announce address: `--p2p-announce-address`

## Example Configurations

### Behind NAT with Port Forwarding

```bash
# Router forwards external 30333 -> internal 192.168.1.50:30333
export DETR_P2P_ANNOUNCE_IP=<EXTERNAL_IP>

./target/release/xlm-pbc-collator \
  --chain mainnet \
  --validator \
  --p2p-bind-address "192.168.1.50:30333"
```

### Custom Port

```bash
./target/release/xlm-pbc-collator \
  --chain dev \
  --alice \
  --p2p-bind-address "0.0.0.0:40000" \
  --p2p-announce-address "<PUBLIC_IP>:40000"
```

### Disable P2P for Testing

```bash
./target/release/xlm-pbc-collator \
  --chain dev \
  --alice \
  --p2p-enabled=false
```

## Bootstrap Peer Format

Bootstrap peers use the format: `peer_id@ip:port`

**Example:**
```
0000000000000000000000000000000000000000000000000000000000000001@192.168.1.100:30333
```

**Multiple peers (comma-separated):**
```bash
--p2p-bootstrap-peers "PEER1@IP1:PORT1,PEER2@IP2:PORT2,PEER3@IP3:PORT3"
```

## Getting Your Peer ID

Your PeerId is shown in the logs on startup:

```
🌐 P2P Configuration:
  Node ID: PeerId(0x0123456789abcdef...)
```

Or query via RPC (if exposed):
```bash
# Future enhancement - RPC endpoint for peer info
```

## Next Steps

- Review [DETR_P2P_INTEGRATION.md](./DETR_P2P_INTEGRATION.md) for detailed architecture
- Configure monitoring and telemetry
- Set up systemd service for production deployment
- Join the etrid validator network

## Support

- Documentation: `/Users/macbook/Desktop/etrid/docs/`
- Issues: GitHub repository
- Community: etrid Discord/Telegram

---

**Built with DETR P2P v1.0 - Production-ready distributed networking**
