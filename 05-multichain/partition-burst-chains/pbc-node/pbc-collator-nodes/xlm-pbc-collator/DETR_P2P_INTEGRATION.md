# DETR P2P Integration for XLM-PBC Collator

This document describes the complete DETR P2P networking integration for the Stellar (XLM) Partition Burst Chain collator.

## Overview

The XLM-PBC collator now includes full DETR P2P networking support with the following features:

1. **Public IP Auto-Detection** - Automatically detects the node's public IP address for proper peer announcements
2. **PeerId Identity Remapping** - Properly handles peer identity transitions from temporary to permanent IDs
3. **Automatic Reconnection** - Automatically reconnects to known peers that disconnect
4. **DHT Maintenance** - Maintains the distributed hash table for peer discovery
5. **Encrypted Communication** - Uses aecomms (X25519 + ChaCha20-Poly1305) for secure peer-to-peer communication
6. **Block Sync** - Broadcasts new blocks to P2P peers and handles incoming block announcements

## Architecture

### Components

#### 1. P2P Configuration (`src/p2p_config.rs`)
- `P2PConfig` - Configuration structure for P2P networking
- `P2PNetworkService` - Service wrapper that manages the P2P network lifecycle
- `peer_id_from_public_key()` - Helper to derive PeerId from validator keys
- `parse_bootstrap_peers()` - Parses bootstrap peer strings

#### 2. P2P Bridge (`src/p2p_bridge.rs`)
- `P2PBridge` - Bridges DETR P2P with Substrate's block sync
- Block announcement handler - Broadcasts new blocks to peers
- Message handler - Processes incoming P2P messages

#### 3. Service Integration (`src/service.rs`)
- `start_collator_with_p2p()` - Main entry point with P2P support
- Initializes P2P network after consensus is running
- Calls `start_all_maintenance()` to enable background tasks

#### 4. CLI Interface (`src/cli.rs`)
- `--p2p-enabled` - Enable/disable P2P networking (default: true)
- `--p2p-bind-address` - Address to bind listener (default: 0.0.0.0:30333)
- `--p2p-announce-address` - Public address to announce to peers (auto-detected if not set)
- `--p2p-bootstrap-peers` - Comma-separated list of bootstrap peers

## Usage

### Basic Usage

Start the collator with default P2P settings:

```bash
./xlm-pbc-collator \
  --chain dev \
  --alice
```

The collator will:
- Bind to `0.0.0.0:30333`
- Auto-detect its public IP using STUN
- Enable all P2P maintenance tasks

### Manual Public IP Configuration

If auto-detection fails or you're behind NAT, set your public IP:

```bash
export DETR_P2P_ANNOUNCE_IP=203.0.113.42

./xlm-pbc-collator \
  --chain dev \
  --alice
```

Or use the CLI flag:

```bash
./xlm-pbc-collator \
  --chain dev \
  --alice \
  --p2p-announce-address "203.0.113.42:30333"
```

### Connect to Bootstrap Peers

Connect to existing peers on startup:

```bash
./xlm-pbc-collator \
  --chain dev \
  --alice \
  --p2p-bootstrap-peers "0000000000000000000000000000000000000000000000000000000000000001@192.168.1.100:30333,0000000000000000000000000000000000000000000000000000000000000002@192.168.1.101:30333"
```

### Custom Bind Address

Bind to a specific interface or port:

```bash
./xlm-pbc-collator \
  --chain dev \
  --alice \
  --p2p-bind-address "192.168.1.50:40000"
```

### Disable P2P Networking

Run without P2P networking:

```bash
./xlm-pbc-collator \
  --chain dev \
  --alice \
  --p2p-enabled=false
```

## How It Works

### Initialization Flow

1. **CLI Parsing** - Command-line arguments are parsed including P2P options
2. **Node ID Generation** - A deterministic PeerId is generated from the node's data directory
3. **P2P Config Creation** - `P2PConfig` is built with bind/announce addresses and bootstrap peers
4. **Network Initialization** - `P2PNetwork::new_with_announce()` creates the P2P network instance
5. **Listener Start** - Network starts listening for incoming connections
6. **Maintenance Tasks** - `start_all_maintenance()` launches background tasks:
   - DHT bucket refresh
   - Periodic peer discovery
   - Automatic reconnection to disconnected peers
7. **Bootstrap Connection** - Connects to configured bootstrap peers
8. **Bridge Start** - P2PBridge starts handling block announcements and messages

### Public IP Detection

The collator uses a multi-method approach to detect its public IP:

1. **Environment Variable** - Checks `DETR_P2P_ANNOUNCE_IP` first
2. **STUN Protocol** - Uses Google's STUN servers to detect external IP via UDP
3. **Fallback** - Uses bind address if detection fails (with warning)

### PeerId Remapping

When peers connect:

1. Initial connection uses temporary PeerId derived from socket address
2. Peer sends `Announce` message with real cryptographic identity
3. `remap_peer_id()` updates the routing table with the real identity
4. Future messages use the permanent PeerId

### Automatic Reconnection

The auto-reconnection task:

1. Runs every 30-120 seconds (adaptive interval based on disconnection count)
2. Scans routing table for known peers that are disconnected
3. Attempts to reconnect to up to 5 peers per cycle
4. Sends `Announce` and `FindNode` messages on successful reconnection
5. Records failed pings so routing table can evict unresponsive peers

### Block Sync

The P2P bridge:

1. Monitors the local chain for new blocks
2. Broadcasts `BlockAnnounce` messages to all connected peers
3. Processes incoming `BlockRequest` messages and returns full encoded blocks
4. Handles consensus votes and certificates

## Implementation Details

### Key DETR P2P Features Used

#### 1. `P2PNetwork::new_with_announce()`
Creates a network with separate bind and announce addresses:
```rust
let network = P2PNetwork::new_with_announce(
    local_node_id,      // Cryptographic identity
    bind_address,       // Local listener (can be 0.0.0.0)
    announce_address,   // Public address for peers
    bootstrap_peers,    // Initial peers
);
```

#### 2. `start_all_maintenance()`
Launches all background maintenance tasks:
```rust
network.start_all_maintenance();
// Starts:
// - start_dht_maintenance() - Bucket refresh every 10 minutes
// - start_periodic_discovery() - FindNode queries every 30 seconds
// - start_auto_reconnection() - Reconnection every 30-120 seconds
```

#### 3. `detect_public_ip()`
Auto-detects public IP using STUN:
```rust
if let Some(ip) = detect_public_ip().await {
    let announce_addr = SocketAddr::new(ip, bind_port);
    // Use detected IP for announcements
}
```

#### 4. `remap_peer_id()`
Updates peer identity after `Announce`:
```rust
// On receiving Announce message with real identity:
conn_manager.remap_peer_id(temp_peer_id, real_peer_id, real_address).await?;
```

### Message Flow

#### Outgoing Block Announcement
```
Local Chain → P2PBridge.start_block_announcements()
           → Message::BlockAnnounce
           → P2PNetwork.broadcast_message()
           → All connected peers
```

#### Incoming Message
```
Remote Peer → TCP connection
           → DETR P2P decoding
           → MessageRouter.route_message()
           → P2PBridge.handle_message()
           → Process based on message type
```

## Environment Variables

- `DETR_P2P_ANNOUNCE_IP` - Override auto-detected public IP address

## Logging

P2P-related log messages use the following prefixes:

- `🌐` - Network initialization and configuration
- `🚀` - Service startup
- `✅` - Successful operations
- `❌` - Errors
- `⚠️` - Warnings
- `🔗` - Connection events
- `📢` - Block announcements
- `📥` - Incoming messages
- `🔄` - Reconnection events
- `🌉` - P2P bridge events

## Testing

### Local Network Test

Run two collators on the same machine:

**Node 1:**
```bash
./xlm-pbc-collator \
  --chain dev \
  --alice \
  --port 30333 \
  --p2p-bind-address "0.0.0.0:30333"
```

**Node 2:**
```bash
./xlm-pbc-collator \
  --chain dev \
  --bob \
  --port 30334 \
  --p2p-bind-address "0.0.0.0:30334" \
  --p2p-bootstrap-peers "<node1_peer_id>@127.0.0.1:30333"
```

Monitor logs for connection and block announcement messages.

### Multi-Machine Test

On separate machines in the same network or over the internet:

**Node 1 (Public IP: 203.0.113.42):**
```bash
export DETR_P2P_ANNOUNCE_IP=203.0.113.42

./xlm-pbc-collator \
  --chain dev \
  --alice
```

**Node 2:**
```bash
./xlm-pbc-collator \
  --chain dev \
  --bob \
  --p2p-bootstrap-peers "<node1_peer_id>@203.0.113.42:30333"
```

## Troubleshooting

### Public IP Not Detected

**Problem:** Log shows "Could not detect public IP"

**Solutions:**
1. Set `DETR_P2P_ANNOUNCE_IP` environment variable
2. Use `--p2p-announce-address` CLI flag
3. Check firewall rules (UDP port 19302 for STUN)

### Peers Not Connecting

**Problem:** No incoming connections

**Solutions:**
1. Verify port forwarding (TCP port 30333 by default)
2. Check firewall allows incoming connections
3. Verify announce address is correct: `--p2p-announce-address`

### Bootstrap Peer Connection Failed

**Problem:** "Failed to connect to bootstrap peer"

**Solutions:**
1. Verify peer ID format (64 hex characters)
2. Verify peer is online and reachable
3. Check network connectivity: `telnet <peer_ip> <peer_port>`

## Future Enhancements

- [ ] Full block request/response implementation
- [ ] Block state sync using P2P
- [ ] Consensus vote propagation over P2P
- [ ] Certificate distribution via P2P
- [ ] Peer reputation scoring
- [ ] Bandwidth management

## References

- [DETR P2P Library](/Users/macbook/Desktop/etrid/01-detr-p2p/detrp2p/src/lib.rs)
- [BTC-PBC P2P Integration](/Users/macbook/Desktop/etrid/05-multichain/partition-burst-chains/pbc-node/pbc-collator-nodes/btc-pbc-collator/P2P_INTEGRATION.md)
- [Substrate Networking](https://docs.substrate.io/fundamentals/networking/)
