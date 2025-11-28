# DETR P2P Integration for BTC-PBC Collator

This document describes the complete integration of DETR P2P networking into the BTC-PBC Collator.

## Overview

The BTC-PBC Collator now includes full DETR P2P networking support with:

1. **Public IP Auto-Detection** - Automatically detects the node's public IP via STUN or environment variable
2. **PeerId Identity Remapping** - Properly handles peer identity when receiving Announce messages
3. **Automatic Reconnection** - Maintains connections to known peers with retry logic
4. **DHT Maintenance** - Keeps the Kademlia routing table fresh
5. **Encrypted Communication** - All P2P traffic encrypted via aecomms (X25519 + ChaCha20-Poly1305)
6. **Block Sync Bridge** - Bridges DETR P2P with Substrate's block sync mechanism

## Components

### 1. P2P Configuration Module (`p2p_config.rs`)

Provides configuration structures and initialization logic for DETR P2P:

- `P2PConfig` - Configuration structure with bind/announce addresses
- `P2PNetworkService` - Service wrapper that manages the P2P network lifecycle
- `detect_announce_address()` - Auto-detects public IP (STUN or env var)
- `parse_bootstrap_peers()` - Parses bootstrap peer strings
- `peer_id_from_public_key()` - Creates PeerId from cryptographic keys

### 2. P2P Bridge Module (`p2p_bridge.rs`)

Bridges DETR P2P with Substrate's block sync:

- `P2PBridge` - Main bridge structure
- Block announcement handler - Broadcasts new blocks to P2P network
- Incoming message handler - Processes messages from P2P peers
- Block request/response handling - Serves block requests from peers
- Peer identity remapping - Handles Announce messages correctly

### 3. CLI Updates (`cli.rs`)

New command-line arguments:

```bash
--p2p-enabled <true|false>              # Enable/disable P2P (default: true)
--p2p-bind-address <addr>               # Bind address (default: 0.0.0.0:30333)
--p2p-announce-address <addr>           # Announce address (optional, auto-detected)
--p2p-bootstrap-peers <peers>           # Bootstrap peers (format: peer_id@ip:port)
```

### 4. Service Integration (`service.rs`)

The collator service now:

1. Initializes P2P network with configuration
2. Starts P2P listener on bind address
3. Calls `start_all_maintenance()` to enable:
   - DHT maintenance (every 5 minutes)
   - Periodic peer discovery (adaptive intervals)
   - Automatic reconnection (every 5 minutes)
4. Starts P2P bridge for block sync
5. Connects to bootstrap peers

## Usage Examples

### Basic Usage (Auto-detect Public IP)

```bash
./btc-pbc-collator \
  --pbc-id 0 \
  --p2p-enabled true \
  --p2p-bind-address 0.0.0.0:30333
```

The node will automatically detect its public IP via STUN.

### With Environment Variable

```bash
export DETR_P2P_ANNOUNCE_IP=203.0.113.42
./btc-pbc-collator \
  --pbc-id 0 \
  --p2p-enabled true \
  --p2p-bind-address 0.0.0.0:30333
```

### With Explicit Announce Address

```bash
./btc-pbc-collator \
  --pbc-id 0 \
  --p2p-enabled true \
  --p2p-bind-address 0.0.0.0:30333 \
  --p2p-announce-address 203.0.113.42:30333
```

### With Bootstrap Peers

```bash
./btc-pbc-collator \
  --pbc-id 0 \
  --p2p-enabled true \
  --p2p-bind-address 0.0.0.0:30333 \
  --p2p-bootstrap-peers "0102030405060708090a0b0c0d0e0f101112131415161718191a1b1c1d1e1f20@192.168.1.100:30333,2122232425262728292a2b2c2d2e2f303132333435363738393a3b3c3d3e3f40@192.168.1.101:30333"
```

### Disable P2P Networking

```bash
./btc-pbc-collator \
  --pbc-id 0 \
  --p2p-enabled false
```

## Public IP Detection

The collator uses a three-tier approach to determine the announce address:

1. **Environment Variable** (`DETR_P2P_ANNOUNCE_IP`)
   - Highest priority
   - Set this for consistent behavior in containers/cloud

2. **STUN Detection**
   - Uses Google's STUN servers
   - Fast, no HTTP dependencies
   - Works through most NAT configurations

3. **Fallback to Bind Address**
   - If detection fails, uses bind address
   - Will warn if 0.0.0.0 is used (peers can't connect)

## PeerId Identity Remapping

When a peer connects:

1. Initially assigned a temporary PeerId based on socket address
2. Peer sends `Announce` message with real cryptographic PeerId
3. `remap_peer_id()` is called to update connection mapping
4. All future messages use the real PeerId

This ensures proper bidirectional communication with correct peer identities.

## Maintenance Tasks

The `start_all_maintenance()` method starts three background tasks:

### 1. DHT Maintenance (every 5 minutes)
- Refreshes Kademlia routing table buckets
- Removes stale entries
- Ensures DHT remains healthy

### 2. Periodic Peer Discovery (adaptive)
- Sends FindNode requests to discover new peers
- Adaptive intervals based on peer count:
  - < 5 peers: every 10 seconds (aggressive)
  - 5-15 peers: every 30 seconds (moderate)
  - > 15 peers: every 60 seconds (maintenance)

### 3. Automatic Reconnection (every 5 minutes)
- Identifies disconnected known peers
- Attempts to reconnect with backoff
- Maintains mesh connectivity

## Block Sync Integration

The P2P bridge provides two-way block sync:

### Outbound (Block Announcements)
- Monitors Substrate client for new blocks
- Broadcasts `BlockAnnounce` messages to all connected peers
- Includes block number, hash, parent hash

### Inbound (Block Requests)
- Handles `BlockRequest` messages from peers
- Fetches blocks from local Substrate client
- Sends `BlockResponse` with encoded block data

## Message Handling

The bridge processes various P2P messages:

- `BlockAnnounce` - New block from peer
- `BlockRequest` - Peer requesting a block
- `BlockResponse` - Response to block request
- `Vote` - Consensus vote messages (forwarded to consensus)
- `Certificate` - Consensus certificates (forwarded to consensus)
- `Ping/Pong` - Connection health checks
- `Announce` - Peer identity announcement (triggers remapping)

## Security Features

1. **Encrypted Transport** - All P2P communication encrypted via aecomms
2. **Identity Verification** - PeerId derived from cryptographic keys
3. **Rate Limiting** - Built into DETR P2P connection manager
4. **Peer Reputation** - Connection tracking and management

## Monitoring

The collator logs P2P activity at various levels:

- **INFO**: Network initialization, peer connections, block announcements
- **WARN**: Detection failures, connection issues, missing data
- **ERROR**: Critical failures, network errors
- **DEBUG**: Detailed message flow, block hashes

Example logs:

```
🌐 Initializing DETR P2P Network for BTC-PBC Collator...
  Local Node ID: PeerId([1, 2, 3, ...])
  Bind Address: 0.0.0.0:30333
  Announce Address: Some(203.0.113.42:30333)
  Bootstrap Peers: 2
📢 Auto-detected public IP: 203.0.113.42
🚀 Starting DETR P2P Network Service...
🚀 DETR P2P background maintenance started
✅ P2P network service started with all maintenance tasks
🔗 Connecting to 2 bootstrap peer(s)...
✅ Connected to bootstrap peer: 192.168.1.100:30333
🌉 Starting P2P Bridge...
📢 Block announcement handler started
📥 Incoming message handler started
✅ P2P Bridge started for block announcements and sync
📢 Announced block #42 to 3 peers
```

## Architecture Diagram

```
┌─────────────────────────────────────────────────────────┐
│              BTC-PBC Collator Main Process              │
├─────────────────────────────────────────────────────────┤
│                                                         │
│  ┌───────────────────┐      ┌──────────────────────┐  │
│  │  Substrate Node   │◄────►│   P2P Bridge         │  │
│  │  (Block Producer) │      │   (Block Sync)       │  │
│  └───────────────────┘      └──────────────────────┘  │
│           │                           │                │
│           │                           │                │
│           ▼                           ▼                │
│  ┌───────────────────┐      ┌──────────────────────┐  │
│  │  Consensus (ASF)  │      │  P2P Network Service │  │
│  │  Block Authoring  │      │  (DETR P2P)          │  │
│  └───────────────────┘      └──────────────────────┘  │
│                                       │                │
│                                       │                │
└───────────────────────────────────────┼────────────────┘
                                        │
                                        ▼
                    ┌─────────────────────────────────┐
                    │     DETR P2P Network Layer      │
                    ├─────────────────────────────────┤
                    │  • Kademlia DHT                 │
                    │  • Connection Manager           │
                    │  • Message Router               │
                    │  • Encryption (aecomms)         │
                    │  • Auto-reconnection            │
                    │  • Public IP detection          │
                    └─────────────────────────────────┘
                                        │
                                        ▼
                            ┌───────────────────────┐
                            │    Network Peers      │
                            │  (Other Collators,    │
                            │   Validators, Nodes)  │
                            └───────────────────────┘
```

## Building

To build the collator with P2P support:

```bash
cd /Users/macbook/Desktop/etrid/05-multichain/partition-burst-chains/pbc-node/pbc-collator-nodes/btc-pbc-collator
cargo build --release
```

## Testing

To test P2P connectivity between two nodes:

**Node 1 (Bootstrap):**
```bash
export DETR_P2P_ANNOUNCE_IP=192.168.1.100
./btc-pbc-collator --pbc-id 0 --p2p-bind-address 0.0.0.0:30333
# Note the Local Node ID from logs
```

**Node 2 (Connecting):**
```bash
export DETR_P2P_ANNOUNCE_IP=192.168.1.101
./btc-pbc-collator \
  --pbc-id 1 \
  --p2p-bind-address 0.0.0.0:30333 \
  --p2p-bootstrap-peers "<NODE1_PEER_ID>@192.168.1.100:30333"
```

## Troubleshooting

### Peers can't connect to me

**Problem:** Other peers can't establish connections to your node.

**Solutions:**
1. Set `DETR_P2P_ANNOUNCE_IP` to your public IP
2. Use `--p2p-announce-address` flag
3. Check firewall rules (port 30333 TCP must be open)
4. Verify NAT/port forwarding configuration

### No peers discovered

**Problem:** Node doesn't discover any peers.

**Solutions:**
1. Verify bootstrap peers are correct and reachable
2. Check network connectivity
3. Ensure at least one bootstrap peer is online
4. Wait for DHT to stabilize (can take 1-2 minutes)

### Auto-detection fails

**Problem:** Public IP auto-detection fails with warnings.

**Solutions:**
1. Set `DETR_P2P_ANNOUNCE_IP` environment variable
2. Use `--p2p-announce-address` flag explicitly
3. Check UDP connectivity to STUN servers (19302/udp)
4. Verify DNS resolution works

### High CPU usage

**Problem:** P2P maintenance tasks consuming too much CPU.

**Analysis:** This is usually not an issue. The maintenance tasks run infrequently:
- DHT maintenance: every 5 minutes
- Peer discovery: adaptive (10-60 seconds)
- Auto-reconnection: every 5 minutes

If CPU usage is still high, check for:
1. Too many connections (reduce with connection limits)
2. Network issues causing constant reconnections
3. High message volume (check logs)

## Future Improvements

1. **Enhanced Block Sync** - Full block encoding/decoding in bridge
2. **Peer Scoring** - Reputation system for peer quality
3. **Connection Prioritization** - Prefer validators and collators
4. **Telemetry Integration** - Export P2P metrics to Prometheus
5. **IPv6 Support** - Full dual-stack support
6. **mDNS Discovery** - Local network peer discovery
7. **Custom Protocols** - Application-specific message types

## References

- DETR P2P Library: `/Users/macbook/Desktop/etrid/01-detr-p2p/`
- BTC-PBC Collator: `/Users/macbook/Desktop/etrid/05-multichain/partition-burst-chains/pbc-node/pbc-collator-nodes/btc-pbc-collator/`
- P2P Configuration: `src/p2p_config.rs`
- P2P Bridge: `src/p2p_bridge.rs`
- Service Integration: `src/service.rs`
- CLI Updates: `src/cli.rs`
