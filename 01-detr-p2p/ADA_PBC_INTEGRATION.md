# ADA PBC Collator - DETR P2P Integration

## Overview

The ADA (Cardano) PBC collator has been fully integrated with the DETR P2P networking layer, providing secure, encrypted peer-to-peer communication with advanced features including:

- **Public IP Auto-Detection**: Automatically detects public IP via STUN/HTTP for proper NAT traversal
- **PeerId Identity Remapping**: Remaps temporary socket-based PeerIds to cryptographic identities from Announce messages
- **Automatic Reconnection**: Maintains persistent connections with automatic retry logic
- **Full Encryption**: X25519 + ChaCha20-Poly1305 encryption via aecomms CipherSession
- **Background Maintenance**: DHT maintenance, periodic discovery, and connection health monitoring

## Files Modified/Created

### 1. Cargo.toml
**Path**: `/Users/macbook/Desktop/etrid/05-multichain/partition-burst-chains/pbc-node/pbc-collator-nodes/ada-pbc-collator/Cargo.toml`

**Added Dependencies**:
```toml
# DETR P2P Networking
detrp2p = { path = "../../../../../01-detr-p2p/detrp2p" }
etrid-aecomms = { path = "../../../../../01-detr-p2p/aecomms" }
hex = "0.4"
```

### 2. p2p_config.rs (NEW)
**Path**: `/Users/macbook/Desktop/etrid/05-multichain/partition-burst-chains/pbc-node/pbc-collator-nodes/ada-pbc-collator/src/p2p_config.rs`

**Purpose**: Complete P2P configuration and network management module

**Key Components**:

#### P2PConfig Struct
- Manages bind address, announce address, and bootstrap peers
- Auto-detects public IP from environment or STUN/HTTP services
- Handles DETR_P2P_ANNOUNCE_IP environment variable

#### P2PNetworkManager Struct
- Wraps P2PNetwork with high-level management methods
- Handles network lifecycle (start, stop, maintenance)
- Provides message sending/broadcasting capabilities
- Monitors connection health and peer count

#### Helper Functions
- `initialize_p2p_config()`: Sets up P2P configuration from environment and CLI
- `detect_announce_address()`: Auto-detects public IP for NAT traversal
- `parse_env_announce_ip()`: Parses DETR_P2P_ANNOUNCE_IP environment variable

### 3. service.rs (UPDATED)
**Path**: `/Users/macbook/Desktop/etrid/05-multichain/partition-burst-chains/pbc-node/pbc-collator-nodes/ada-pbc-collator/src/service.rs`

**Changes**:
- Added DETR P2P imports
- Integrated P2P network initialization in `start_collator()`
- Spawns dedicated P2P network task with full maintenance
- Periodically logs peer connection status
- Changed log prefixes from "BTC-PBC" to "ADA-PBC"

**Integration Code** (added to `start_collator()`):
```rust
// Generate node ID from keystore
let node_id_bytes = { /* crypto key derivation */ };

// Parse configuration from environment
let p2p_port = std::env::var("DETR_P2P_PORT")...;
let bootstrap_peers = std::env::var("DETR_P2P_BOOTSTRAP_PEERS")...;

// Initialize P2P config with public IP detection
let p2p_config = initialize_p2p_config(node_id_bytes, Some(p2p_port), bootstrap_peers).await?;

// Create P2P network manager
let p2p_manager = P2PNetworkManager::new(p2p_config).await?;

// Start P2P network with all maintenance tasks
task_manager.spawn_essential_handle().spawn_blocking(
    "detr-p2p-network",
    Some("networking"),
    async move {
        p2p_manager.start().await?;
        // Keep alive and log peer count
    },
);
```

### 4. main.rs (UPDATED)
**Path**: `/Users/macbook/Desktop/etrid/05-multichain/partition-burst-chains/pbc-node/pbc-collator-nodes/ada-pbc-collator/src/main.rs`

**Changes**:
- Added `mod p2p_config;` declaration

## Environment Variables

### DETR_P2P_ANNOUNCE_IP
**Purpose**: Manually specify the public IP address for peer announcements

**Format**: IP address (IPv4 or IPv6)

**Example**:
```bash
export DETR_P2P_ANNOUNCE_IP="203.0.113.42"
```

**Usage**: Required when automatic IP detection fails or when running behind NAT/firewall. If not set, the system will attempt to auto-detect using STUN and HTTP methods.

### DETR_P2P_PORT
**Purpose**: Specify the P2P network listening port

**Format**: Port number (1-65535)

**Default**: 30333

**Example**:
```bash
export DETR_P2P_PORT=40000
```

### DETR_P2P_BOOTSTRAP_PEERS
**Purpose**: Comma-separated list of bootstrap peers for initial network discovery

**Format**: `peer_id1@ip1:port1,peer_id2@ip2:port2`

**Example**:
```bash
export DETR_P2P_BOOTSTRAP_PEERS="a1b2c3d4...@192.168.1.100:30333,e5f6g7h8...@10.0.0.50:30334"
```

**Note**: Peer IDs must be 64-character hex strings (32 bytes)

## DETR P2P Features Integrated

### 1. Public IP Auto-Detection (`detect_public_ip`)
The ADA collator automatically detects its public IP address through multiple methods:

1. **Environment Variable Override** (`DETR_P2P_ANNOUNCE_IP`)
2. **STUN Protocol**: Queries Google STUN servers via UDP
3. **HTTP Fallback**: Queries public IP detection services
4. **Warning System**: Alerts if detection fails

### 2. PeerId Identity Remapping (`remap_peer_id`)
When receiving an `Announce` message from a peer:

1. Temporary PeerId (derived from socket address) is created for incoming connections
2. Real cryptographic PeerId is received in the Announce message
3. Connection manager remaps the temporary ID to the real ID
4. Stream and encryption session are remapped accordingly
5. Bidirectional communication is established with correct identity

### 3. Automatic Reconnection (`start_auto_reconnection`)
Background task that:

1. Periodically checks for disconnected known peers
2. Attempts reconnection with exponential backoff
3. Re-sends Announce and FindNode messages after reconnection
4. Records failed attempts for routing table eviction
5. Adjusts retry interval based on disconnection count

### 4. Background Maintenance (`start_all_maintenance`)
Convenience method that starts all maintenance tasks:

- **DHT Maintenance**: Bucket refresh, peer health checks
- **Periodic Discovery**: FindNode queries for network mapping
- **Auto-Reconnection**: Connection resilience

**Called After Network Start**:
```rust
p2p_manager.start().await?;
// start_all_maintenance() is called inside start()
```

### 5. Encryption via aecomms
All peer communications are encrypted using:

- **Key Exchange**: X25519 Elliptic Curve Diffie-Hellman
- **Encryption**: ChaCha20-Poly1305 AEAD cipher
- **Session Management**: CipherSession per peer with unique session IDs
- **Handshake**: Automatic handshake during connection establishment

## Running the ADA PBC Collator

### Basic Run
```bash
cd /Users/macbook/Desktop/etrid/05-multichain/partition-burst-chains/pbc-node/pbc-collator-nodes/ada-pbc-collator

# Build the collator
cargo build --release

# Run with default P2P settings
./target/release/ada-pbc-collator \
  --chain=dev \
  --alice
```

### Run with Custom P2P Configuration
```bash
# Set public IP for NAT traversal
export DETR_P2P_ANNOUNCE_IP="203.0.113.42"

# Set custom P2P port
export DETR_P2P_PORT=40000

# Set bootstrap peers for network discovery
export DETR_P2P_BOOTSTRAP_PEERS="a1b2c3d4e5f6...@192.168.1.100:30333"

# Run the collator
./target/release/ada-pbc-collator \
  --chain=dev \
  --alice \
  --pbc-id=0
```

### Docker Deployment
```bash
# Build Docker image
docker build -t ada-pbc-collator .

# Run with environment variables
docker run -d \
  --name ada-pbc \
  -e DETR_P2P_ANNOUNCE_IP="203.0.113.42" \
  -e DETR_P2P_PORT=30333 \
  -e DETR_P2P_BOOTSTRAP_PEERS="..." \
  -p 30333:30333 \
  -p 9944:9944 \
  ada-pbc-collator \
  --chain=dev \
  --alice
```

## Monitoring P2P Network

### Log Output
The ADA collator logs detailed P2P network information:

```
🌐 Initializing DETR P2P network for ADA PBC...
🔧 Initializing DETR P2P configuration for ADA PBC...
🔍 Detecting public announce address for DETR P2P...
📢 Using DETR P2P_ANNOUNCE_IP: 203.0.113.42:30333
✅ Public announce address: 203.0.113.42:30333
🚀 Initializing DETR P2P network for ADA PBC...
   Node ID: PeerId([...])
   Bind Address: 0.0.0.0:30333
   Announce Address: Some(203.0.113.42:30333)
   Bootstrap Peers: 3
🔌 Starting DETR P2P network for ADA PBC...
✅ DETR P2P network started successfully
🔧 Starting background maintenance tasks...
🚀 DETR P2P background maintenance started
✅ DETR P2P network running with full maintenance
📊 DETR P2P: 5 connected peers
```

### Peer Count Monitoring
Every 30 seconds, the collator logs the current peer count:
```
📊 DETR P2P: 5 connected peers
```

### Connection Events
```
🔗 Incoming connection from 192.168.1.100:54321
📢 Announced ourselves to 192.168.1.100:54321
🔑 Remapped connection: temp PeerId([...]) → real PeerId([...])
🔐 Secure session established with peer PeerId([...])
```

## Testing P2P Integration

### Unit Tests
Run the p2p_config module tests:
```bash
cd /Users/macbook/Desktop/etrid/05-multichain/partition-burst-chains/pbc-node/pbc-collator-nodes/ada-pbc-collator

cargo test p2p_config --lib
```

### Integration Tests
1. Start first ADA collator (bootstrap node):
```bash
export DETR_P2P_PORT=30333
./target/release/ada-pbc-collator --chain=dev --alice
```

2. Note the Node ID from logs and start second collator:
```bash
export DETR_P2P_PORT=30334
export DETR_P2P_BOOTSTRAP_PEERS="<node_id_from_first>@127.0.0.1:30333"
./target/release/ada-pbc-collator --chain=dev --bob
```

3. Check logs for successful peer connection:
```
📊 DETR P2P: 1 connected peers
```

## Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                    ADA PBC Collator                         │
├─────────────────────────────────────────────────────────────┤
│                                                             │
│  ┌──────────────┐      ┌──────────────────────────────┐   │
│  │   service.rs │──────│   p2p_config.rs              │   │
│  │              │      │                               │   │
│  │ - Substrate  │      │ - P2PConfig                   │   │
│  │ - ASF        │      │ - P2PNetworkManager           │   │
│  │ - Runtime    │      │ - initialize_p2p_config()     │   │
│  └──────────────┘      └──────────────────────────────┘   │
│         │                           │                       │
│         └───────────────────────────┘                       │
│                         │                                   │
└─────────────────────────┼───────────────────────────────────┘
                          │
                          ▼
┌─────────────────────────────────────────────────────────────┐
│              DETR P2P Library (detrp2p)                     │
├─────────────────────────────────────────────────────────────┤
│                                                             │
│  ┌───────────────┐   ┌─────────────────┐   ┌────────────┐ │
│  │  P2PNetwork   │   │ ConnectionMgr   │   │  Kademlia  │ │
│  │               │   │                 │   │            │ │
│  │ - start()     │   │ - connect()     │   │ - DHT      │ │
│  │ - maintenance │   │ - remap_peer_id │   │ - routing  │ │
│  │ - unicast     │   │ - send_message  │   │ - lookup   │ │
│  └───────────────┘   └─────────────────┘   └────────────┘ │
│         │                     │                    │        │
│         └─────────────────────┴────────────────────┘        │
│                               │                             │
└───────────────────────────────┼─────────────────────────────┘
                                │
                                ▼
┌─────────────────────────────────────────────────────────────┐
│            Encryption Layer (etrid-aecomms)                 │
├─────────────────────────────────────────────────────────────┤
│                                                             │
│  ┌────────────────────────────────────────────────────┐    │
│  │  CipherSession                                     │    │
│  │                                                    │    │
│  │  - X25519 Key Exchange                             │    │
│  │  - ChaCha20-Poly1305 AEAD Encryption               │    │
│  │  - Session Management                              │    │
│  └────────────────────────────────────────────────────┘    │
│                                                             │
└─────────────────────────────────────────────────────────────┘
```

## Troubleshooting

### Issue: "Could not detect public IP"
**Solution**: Set `DETR_P2P_ANNOUNCE_IP` environment variable to your public IP

### Issue: "Max connections reached"
**Solution**: Increase `max_connections` in ConnectionManager initialization (currently set to 100)

### Issue: "No bootstrap peers"
**Solution**: Set `DETR_P2P_BOOTSTRAP_PEERS` environment variable or ensure at least one bootstrap peer is configured

### Issue: "Failed to bind listener"
**Solution**:
- Check if port is already in use: `lsof -i :30333`
- Try a different port: `export DETR_P2P_PORT=40000`
- Check firewall settings

### Issue: Peers not connecting
**Solution**:
1. Verify announce IP is reachable from peer network
2. Check firewall allows incoming connections on P2P port
3. Verify bootstrap peer format is correct
4. Check logs for "Connection closed" or "Failed to connect" messages

## Security Considerations

1. **Encryption**: All peer communication is encrypted end-to-end with X25519 + ChaCha20-Poly1305
2. **Authentication**: PeerIds are derived from cryptographic keys, not socket addresses
3. **Session Management**: Each peer has a unique encryption session with automatic key rotation
4. **DoS Protection**: Message size limits (10MB) prevent oversized message attacks
5. **Idle Timeout**: Inactive connections are automatically closed after 5 minutes

## Performance Notes

- **Connection Limit**: 100 concurrent connections (configurable)
- **Message Size**: 10MB maximum per message
- **Idle Timeout**: 5 minutes
- **Reconnection Interval**: 30-120 seconds (adaptive based on peer count)
- **DHT Maintenance**: Periodic bucket refresh and peer health checks
- **Discovery Interval**: Regular FindNode queries for network mapping

## Future Enhancements

1. **Dynamic Port Allocation**: Automatic port selection if default is in use
2. **Peer Reputation System**: Track peer behavior and ban misbehaving nodes
3. **Connection Prioritization**: Prefer validators and collators over regular nodes
4. **Bandwidth Monitoring**: Track upload/download per peer
5. **Advanced NAT Traversal**: UPnP/NAT-PMP support for automatic port forwarding
6. **IPv6 Support**: Full dual-stack IPv4/IPv6 support
7. **Metrics Export**: Prometheus metrics for network health monitoring

## References

- [DETR P2P Library](../01-detr-p2p/detrp2p/src/lib.rs)
- [Ëtrid AEComms Encryption](../01-detr-p2p/aecomms/src/lib.rs)
- [ADA PBC Runtime](../../pbc-chains/ada-pbc/runtime/src/lib.rs)
- [ASF Consensus](../../../../09-consensus/client/consensus-asf/)

## Contact

For issues or questions about DETR P2P integration:
- Open an issue in the Ëtrid repository
- Contact the Ëtrid team at support@etrid.org
- Check the community forums at https://forum.etrid.org
