# DETR P2P Integration for SOL-PBC Collator

## Overview

The SOL (Solana) PBC collator has been fully integrated with the DETR P2P networking layer, providing:

1. **Automatic Public IP Detection** - Uses STUN protocol and HTTP fallback
2. **PeerId Remapping** - Properly handles temporary socket-based IDs and real cryptographic IDs
3. **Automatic Reconnection** - Maintains connections to known peers
4. **DHT Maintenance** - Keeps the routing table healthy
5. **Periodic Discovery** - Continuously discovers new peers
6. **Encrypted Communication** - Uses aecomms CipherSession with X25519 + ChaCha20-Poly1305

## Files Modified/Created

### New Files
- `src/p2p_config.rs` - P2P configuration module with announce address support

### Modified Files
- `Cargo.toml` - Added detrp2p and etrid-aecomms dependencies
- `src/main.rs` - Added p2p_config module and CLI parameter passing
- `src/cli.rs` - Added P2P CLI parameters
- `src/service.rs` - Integrated P2P service with block announcement

## Configuration

### Environment Variables

```bash
# Public IP for P2P announcements (auto-detected if not set)
export DETR_P2P_ANNOUNCE_IP="203.0.113.10"

# Listen port (can also be set via CLI)
export DETR_P2P_PORT="30333"

# Bootstrap peers (comma-separated)
export DETR_P2P_BOOTSTRAP="203.0.113.11:30333,203.0.113.12:30333"
```

### CLI Parameters

```bash
# Listen port
--p2p-port 30333

# Announce IP (overrides auto-detection)
--p2p-announce-ip 203.0.113.10

# Bootstrap peers
--p2p-bootstrap "203.0.113.11:30333,203.0.113.12:30333"

# Disable specific maintenance tasks
--p2p-disable-auto-reconnect
--p2p-disable-dht-maintenance
--p2p-disable-periodic-discovery
```

## Usage Examples

### Development (Local Testing)

```bash
# Node 1
./target/release/sol-pbc-collator \
  --dev \
  --p2p-port 30333

# Node 2 (connects to Node 1)
./target/release/sol-pbc-collator \
  --dev \
  --p2p-port 30334 \
  --p2p-bootstrap "127.0.0.1:30333"
```

### Production Deployment

```bash
# Set public IP via environment variable
export DETR_P2P_ANNOUNCE_IP="203.0.113.10"

# Run with bootstrap peers
./target/release/sol-pbc-collator \
  --chain mainnet \
  --p2p-port 30333 \
  --p2p-bootstrap "bootstrap1.etrid.org:30333,bootstrap2.etrid.org:30333"
```

### Docker Deployment

```bash
docker run -d \
  -e DETR_P2P_ANNOUNCE_IP="203.0.113.10" \
  -p 30333:30333 \
  etrid/sol-pbc-collator:latest \
  --p2p-port 30333 \
  --p2p-bootstrap "bootstrap.etrid.org:30333"
```

## Features Implemented

### 1. Public IP Auto-Detection

The collator automatically detects its public IP using:
- STUN protocol (Google STUN servers)
- HTTP API fallback (ipify, ifconfig.me, icanhazip)
- Environment variable override (`DETR_P2P_ANNOUNCE_IP`)

### 2. PeerId Remapping

When receiving `Announce` messages, the collator properly remaps temporary socket-derived PeerIds to real cryptographic PeerIds from the message.

### 3. Automatic Reconnection

The `start_auto_reconnection()` task:
- Runs every 30-120 seconds (adaptive based on disconnected peer count)
- Attempts to reconnect to known peers from routing table
- Limits to 5 reconnection attempts per cycle to avoid storms
- Re-announces identity and refreshes peer lists on reconnection

### 4. DHT Maintenance

The `start_dht_maintenance()` task:
- Runs every 30-60 seconds (adaptive based on routing table health)
- Pings peers to verify liveness
- Evicts failed peers from routing table
- Ensures routing table stays populated

### 5. Periodic Discovery

The `start_periodic_discovery()` task:
- Runs every 30-60 seconds (adaptive based on connection count)
- Performs random lookups to discover new peers
- Maintains healthy connection count

### 6. Block Announcement

Every new block produced is:
- Announced to all connected peers via P2P network
- Includes block number, hash, parent hash, and encoded header
- Uses DETR P2P's `BlockAnnounce` message type

## Architecture

```
┌─────────────────────────────────────────────────┐
│         SOL-PBC Collator Service                │
├─────────────────────────────────────────────────┤
│  ┌───────────────┐      ┌──────────────────┐   │
│  │ Block Author  │─────▶│ State Root Sub   │   │
│  │  (ASF)        │      │  + P2P Announce  │   │
│  └───────────────┘      └────────┬─────────┘   │
│                                   │             │
│  ┌────────────────────────────────▼──────────┐  │
│  │         DETR P2P Service                  │  │
│  ├───────────────────────────────────────────┤  │
│  │  • Auto IP Detection                      │  │
│  │  • PeerId Remapping                       │  │
│  │  • Auto Reconnection                      │  │
│  │  • DHT Maintenance                        │  │
│  │  • Periodic Discovery                     │  │
│  │  • Encrypted Communication (aecomms)      │  │
│  └───────────────────────────────────────────┘  │
└─────────────────────────────────────────────────┘
```

## Maintenance Tasks

All maintenance tasks are started via `start_all_maintenance()` which calls:
1. `start_dht_maintenance()` - Keeps routing table healthy
2. `start_periodic_discovery()` - Discovers new peers
3. `start_auto_reconnection()` - Reconnects to known peers

These tasks run in background tokio tasks and adapt their intervals based on network conditions.

## Health Monitoring

A dedicated P2P health monitor task runs every 30 seconds and:
- Reports connected peer count
- Warns if no peers are connected
- Logs changes in peer count

## Security

- All P2P communication is encrypted using aecomms CipherSession
- X25519 key exchange for perfect forward secrecy
- ChaCha20-Poly1305 for authenticated encryption
- PeerIds are cryptographically derived from public keys

## Building

```bash
cd /Users/macbook/Desktop/etrid/05-multichain/partition-burst-chains/pbc-node/pbc-collator-nodes/sol-pbc-collator
cargo build --release
```

## Testing

```bash
# Run tests
cargo test

# Start a development node
cargo run -- --dev --p2p-port 30333
```

## Troubleshooting

### No Peers Connected

1. Check firewall rules allow incoming connections on P2P port
2. Verify public IP is correctly detected/set
3. Check bootstrap peers are reachable
4. Review logs for connection errors

### Public IP Detection Fails

Set the announce IP manually:
```bash
export DETR_P2P_ANNOUNCE_IP="your.public.ip"
# or use CLI
--p2p-announce-ip "your.public.ip"
```

### High Reconnection Activity

If you see excessive reconnection attempts:
- Check network stability
- Verify bootstrap peers are stable
- Consider increasing reconnection intervals (code modification needed)

## Next Steps

Potential enhancements:
1. Add metrics for P2P network health
2. Implement peer scoring and reputation
3. Add configurable reconnection intervals
4. Implement block sync request/response handling
5. Add peer discovery via DNS seeds
