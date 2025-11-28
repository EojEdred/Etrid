# SC-USDT PBC Collator - Quick Start Guide

## Prerequisites

- Rust toolchain (1.70+)
- Linux or macOS
- Network access for STUN-based IP detection

## Build

```bash
cd /Users/macbook/Desktop/etrid/05-multichain/partition-burst-chains/pbc-node/pbc-collator-nodes/sc-usdt-pbc-collator

# Build in release mode
cargo build --release
```

Binary location: `./target/release/sc-usdt-pbc-collator`

## Run (Development Mode)

### Single Node (Alice)

```bash
./target/release/sc-usdt-pbc-collator \
  --chain dev \
  --validator \
  --alice \
  --tmp \
  --rpc-port 9944 \
  --port 30333
```

P2P will auto-start on port 30336 with auto-detected public IP.

### Two Nodes (Alice + Bob)

**Terminal 1 - Alice (Bootstrap Node):**
```bash
./target/release/sc-usdt-pbc-collator \
  --chain dev \
  --validator \
  --alice \
  --tmp \
  --rpc-port 9944 \
  --port 30333 \
  --p2p-bind-address "0.0.0.0:30336"
```

**Terminal 2 - Bob (Connecting Node):**

First, get Alice's Peer ID from Terminal 1 logs. Look for:
```
Local Node ID: PeerId([...])
```

Then run Bob:
```bash
./target/release/sc-usdt-pbc-collator \
  --chain dev \
  --validator \
  --bob \
  --tmp \
  --rpc-port 9945 \
  --port 30334 \
  --p2p-bind-address "0.0.0.0:30337" \
  --p2p-bootstrap-peers "<alice_peer_id>@127.0.0.1:30336"
```

Replace `<alice_peer_id>` with the hex-encoded peer ID from Terminal 1.

## Environment Variables

### Set Public IP

For nodes behind NAT or with multiple network interfaces:

```bash
export DETR_P2P_ANNOUNCE_IP=203.0.113.42

./target/release/sc-usdt-pbc-collator --chain dev --validator --alice
```

## Verify P2P is Working

Look for these log messages:

```
✅ SC-USDT-PBC: DETR P2P network started successfully
✅ SC-USDT-PBC: Connected to bootstrap peer
🚀 DETR P2P background maintenance started
```

## Disable P2P (Testing)

```bash
./target/release/sc-usdt-pbc-collator \
  --chain dev \
  --validator \
  --alice \
  --p2p-enabled false
```

## Help

```bash
./target/release/sc-usdt-pbc-collator --help
```

Look for P2P options:
- `--p2p-enabled`
- `--p2p-bind-address`
- `--p2p-announce-address`
- `--p2p-bootstrap-peers`

## Troubleshooting

### "Could not auto-detect public IP"

Set `DETR_P2P_ANNOUNCE_IP` environment variable.

### No peer connections

1. Check firewall allows port 30336 (TCP)
2. Verify bootstrap peer address is correct
3. Ensure bootstrap node is running and reachable

### Build errors

1. Update Rust: `rustup update`
2. Clean build: `cargo clean && cargo build --release`
3. Check all dependencies are available

## Next Steps

See full documentation in `SC_USDT_P2P_INTEGRATION_COMPLETE.md`
