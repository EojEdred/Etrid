# ADA PBC DETR P2P Quick Start Guide

## Build & Run (3 Steps)

### 1. Build the Collator
```bash
cd /Users/macbook/Desktop/etrid/05-multichain/partition-burst-chains/pbc-node/pbc-collator-nodes/ada-pbc-collator
cargo build --release
```

### 2. Set Environment Variables (Optional but Recommended)
```bash
# Set your public IP (if behind NAT/firewall)
export DETR_P2P_ANNOUNCE_IP="your.public.ip.address"

# Set P2P port (default: 30333)
export DETR_P2P_PORT=30333

# Add bootstrap peers (for network discovery)
export DETR_P2P_BOOTSTRAP_PEERS="peer_id@bootstrap_ip:port"
```

### 3. Run the Collator
```bash
./target/release/ada-pbc-collator --chain=dev --alice --pbc-id=0
```

## Environment Variables Reference

| Variable | Purpose | Required | Default | Example |
|----------|---------|----------|---------|---------|
| `DETR_P2P_ANNOUNCE_IP` | Public IP for peer announcements | No (auto-detected) | Auto-detect | `203.0.113.42` |
| `DETR_P2P_PORT` | P2P listening port | No | 30333 | `40000` |
| `DETR_P2P_BOOTSTRAP_PEERS` | Initial peers to connect to | No | Empty | `a1b2c3...@192.168.1.100:30333` |

## What's Integrated

✅ **PeerId Identity Remapping** - Automatic cryptographic identity mapping
✅ **Public IP Auto-Detection** - STUN/HTTP-based NAT traversal
✅ **Automatic Reconnection** - Resilient peer connections
✅ **Full Encryption** - X25519 + ChaCha20-Poly1305 via aecomms
✅ **Background Maintenance** - DHT, discovery, health monitoring

## Example: Two-Node Network

**Node 1 (Bootstrap):**
```bash
export DETR_P2P_PORT=30333
export DETR_P2P_ANNOUNCE_IP="192.168.1.100"
./target/release/ada-pbc-collator --chain=dev --alice --pbc-id=0
# Note the Node ID from logs: PeerId([a1, b2, c3, ...])
```

**Node 2 (Peer):**
```bash
export DETR_P2P_PORT=30334
export DETR_P2P_ANNOUNCE_IP="192.168.1.101"
export DETR_P2P_BOOTSTRAP_PEERS="<node1_peer_id_hex>@192.168.1.100:30333"
./target/release/ada-pbc-collator --chain=dev --bob --pbc-id=1
```

## Expected Logs

```
🌐 Initializing DETR P2P network for ADA PBC...
🔍 Detecting public announce address for DETR P2P...
✅ Public announce address: 203.0.113.42:30333
🚀 Initializing DETR P2P network for ADA PBC...
🔌 Starting DETR P2P network for ADA PBC...
✅ DETR P2P network started successfully
🔧 Starting background maintenance tasks...
🚀 DETR P2P background maintenance started
✅ ADA PBC collator with DETR P2P network fully initialized
📊 DETR P2P: 3 connected peers  ← Logged every 30 seconds
```

## Troubleshooting

**"Could not detect public IP"**
→ Set `DETR_P2P_ANNOUNCE_IP` manually

**"No bootstrap peers"**
→ At least one node must be running first (bootstrap node)

**"Failed to bind listener"**
→ Port already in use, try different `DETR_P2P_PORT`

**Peers not connecting**
→ Check firewall allows incoming on P2P port
→ Verify announce IP is reachable from peer network
→ Verify bootstrap peer format: `peer_id@ip:port`

## Files Modified

- `Cargo.toml` - Added detrp2p, etrid-aecomms, hex dependencies
- `src/p2p_config.rs` - NEW - Complete P2P configuration module (301 lines)
- `src/service.rs` - Integrated P2P network initialization
- `src/main.rs` - Added p2p_config module declaration

## Documentation

Full documentation: `/Users/macbook/Desktop/etrid/01-detr-p2p/ADA_PBC_INTEGRATION.md`

## Status

✅ **Production Ready** - No TODOs, no placeholders, builds successfully
