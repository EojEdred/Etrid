# TRX PBC Collator - DETR P2P Integration Guide

## Overview

The TRX (Tron) PBC Collator has been fully integrated with the DETR P2P networking layer, providing enterprise-grade peer-to-peer communication with the following features:

### Key Features

1. **PeerId Identity Remapping**: Automatic remapping from temporary socket-derived peer IDs to cryptographic peer identities upon receiving Announce messages
2. **Public IP Auto-Detection**: Three-tier detection strategy:
   - STUN-based UDP detection (fastest, no external HTTP dependencies)
   - HTTP API fallback (api.ipify.org, ifconfig.me, icanhazip.com)
   - Environment variable override (DETR_P2P_ANNOUNCE_IP)
3. **Automatic Reconnection Logic**: Maintains connections to known peers with exponential backoff
4. **Background Maintenance Tasks**:
   - DHT bucket refresh and storage cleanup
   - Periodic peer discovery via Kademlia lookups
   - Connection health monitoring
5. **Encrypted Communications**: X25519 + ChaCha20-Poly1305 via Etrid's aecomms library

## Architecture

```
TRX PBC Collator
├── src/main.rs          - Entry point, initializes P2P from CLI args
├── src/cli.rs           - CLI arguments for P2P configuration
├── src/p2p.rs           - P2P manager and configuration
├── src/service.rs       - Service integration, broadcasts blocks
└── Cargo.toml           - Dependencies: detrp2p, etrid-aecomms

DETR P2P Library (/Users/macbook/Desktop/etrid/01-detr-p2p/detrp2p)
├── Kademlia DHT         - Peer discovery and routing
├── Connection Manager   - TCP connection lifecycle
├── Message Router       - Bidirectional message handling
├── Encryption Manager   - Session key negotiation
└── Reputation System    - Peer scoring and ban logic
```

## Configuration

### Command-Line Arguments

```bash
# Start with default P2P settings (listens on 0.0.0.0:30333)
./trx-pbc-collator

# Specify listen and announce addresses
./trx-pbc-collator \
  --p2p-listen "0.0.0.0:30333" \
  --p2p-announce "203.0.113.42:30333"

# Connect to bootstrap peers
./trx-pbc-collator \
  --p2p-bootstrap "1.2.3.4:30333,5.6.7.8:30333"

# Advanced multiaddr format (with explicit peer IDs)
./trx-pbc-collator \
  --p2p-bootstrap "/ip4/1.2.3.4/tcp/30333/p2p/12D3KooWAbCd..."

# Disable P2P networking
./trx-pbc-collator --enable-p2p false
```

### Environment Variables

```bash
# Override public IP detection
export DETR_P2P_ANNOUNCE_IP="203.0.113.42"

# Then start the collator
./trx-pbc-collator --p2p-listen "0.0.0.0:30333"
```

### Configuration Strategies

#### Strategy 1: Local Development (Single Machine)
```bash
# Node 1
./trx-pbc-collator --p2p-listen "127.0.0.1:30333"

# Node 2
./trx-pbc-collator --p2p-listen "127.0.0.1:30334" \
  --p2p-bootstrap "127.0.0.1:30333"
```

#### Strategy 2: Cloud Deployment (Public IP)
```bash
# Automatic IP detection via STUN
./trx-pbc-collator --p2p-listen "0.0.0.0:30333"

# Or explicit configuration
export DETR_P2P_ANNOUNCE_IP="203.0.113.42"
./trx-pbc-collator --p2p-listen "0.0.0.0:30333"
```

#### Strategy 3: Behind NAT/Firewall
```bash
# Port forward 30333 -> your internal IP, then:
export DETR_P2P_ANNOUNCE_IP="<your-public-ip>"
./trx-pbc-collator --p2p-listen "0.0.0.0:30333"
```

## Implementation Details

### 1. PeerId Identity Remapping

When a peer connects, we initially assign a temporary peer ID derived from their socket address:

```rust
// Initial connection (temporary ID)
let temp_peer_id = PeerId::from_socket_addr(peer_addr);

// After receiving Announce message with real cryptographic identity
connection_manager.remap_peer_id(temp_peer_id, real_peer_id, real_address).await;
```

This ensures that:
- Connections are tracked immediately
- Cryptographic identities are used for all routing decisions
- DHT operations use stable, authenticated peer IDs

### 2. Public IP Auto-Detection Flow

```
1. Check CLI --p2p-announce flag
   ↓ (if not set)
2. Check DETR_P2P_ANNOUNCE_IP environment variable
   ↓ (if not set)
3. Try STUN-based UDP detection (stun.l.google.com:19302)
   ↓ (if fails)
4. Try HTTP API fallback (api.ipify.org, etc.)
   ↓ (if fails)
5. Fallback to listen address (with warning)
```

### 3. Automatic Reconnection Logic

The `start_auto_reconnection()` task:
- Runs every 30-120 seconds (adaptive based on disconnected peer count)
- Identifies known peers from DHT routing table that are disconnected
- Attempts to reconnect up to 5 peers per cycle
- Sends Announce + FindNode after successful reconnection
- Records failed pings for eventual peer eviction

### 4. Background Maintenance Tasks

Called via `network.start_all_maintenance()`:

```rust
// DHT Maintenance (every 60s)
- Clean expired storage entries
- Refresh stale buckets (>1 hour since last refresh)
- Republish stored values

// Periodic Discovery (every 10-60s, adaptive)
- Generate random peer ID in sparse buckets
- Perform Kademlia lookup to discover new peers
- More aggressive when peer count < 15

// Auto-Reconnection (every 30-120s)
- Reconnect to known but disconnected peers
- Maintain stable peer graph
```

### 5. Block Broadcasting

Every time a new block is produced:

```rust
// In submit_state_roots()
let block_announce = detrp2p::Message::BlockAnnounce {
    block_number: best_number as u64,
    block_hash: best_hash.into(),
    parent_hash: *header.parent_hash(),
    encoded_block: Vec::new(),
};

manager.broadcast(block_announce).await;
```

This announces new blocks to all connected peers via the P2P network.

## Module Structure

### src/p2p.rs

**Key Components:**

- `P2PConfig`: Configuration struct (listen/announce addresses, bootstrap peers)
- `P2PManager`: Main manager (network lifecycle, message sending, peer queries)
- `parse_bootstrap_peers()`: Parses CLI peer strings into `Vec<PeerAddr>`
- Global instance via `once_cell` for thread-safe access

**Methods:**

- `new(config)`: Create manager with configuration
- `start()`: Initialize network, detect IP, connect to bootstrap peers, start maintenance
- `determine_announce_address()`: Multi-strategy IP detection
- `send_to_peer(peer_id, msg)`: Send to specific peer
- `broadcast(msg)`: Send to all connected peers
- `peer_count()`: Get current connection count
- `connected_peers()`: Get list of peer IDs

### src/service.rs

**Integration Points:**

- `start_collator()`: Spawns `start_p2p_network()` task
- `start_p2p_network()`: Initializes P2P manager, logs status every 60s
- `submit_state_roots()`: Broadcasts block announcements when new blocks are produced

### src/main.rs

**Initialization:**

```rust
// Parse CLI arguments
let cli = cli::Cli::parse();

// Build P2P config from CLI args
let p2p_config = p2p::P2PConfig {
    listen_address: cli.p2p_listen.parse()?,
    announce_address: cli.p2p_announce.as_ref().map(|a| a.parse()).transpose()?,
    bootstrap_peers: parse_bootstrap_peers(&cli.p2p_bootstrap)?,
    enabled: cli.enable_p2p,
};

// Initialize global manager
p2p::init_p2p_manager(p2p_config);
```

## Testing

### Local Network Test

```bash
# Terminal 1: Bootstrap node
./trx-pbc-collator --p2p-listen "127.0.0.1:30333" --dev

# Terminal 2: Connect to bootstrap
./trx-pbc-collator --p2p-listen "127.0.0.1:30334" \
  --p2p-bootstrap "127.0.0.1:30333" --dev

# Terminal 3: Third node
./trx-pbc-collator --p2p-listen "127.0.0.1:30335" \
  --p2p-bootstrap "127.0.0.1:30333,127.0.0.1:30334" --dev
```

### Expected Log Output

```
🚀 Starting DETR P2P network for TRX PBC Collator...
🔍 Auto-detecting public IP address...
📢 Detected public IP via STUN: 203.0.113.42
📡 DETR P2P Configuration:
   Listen Address:   0.0.0.0:30333
   Announce Address: 203.0.113.42:30333
   Local Peer ID:    PeerId([...])
   Bootstrap Peers:  2
✅ DETR P2P network started successfully
🔧 DETR P2P maintenance tasks started:
   ✓ DHT maintenance (bucket refresh, storage cleanup)
   ✓ Periodic peer discovery
   ✓ Automatic reconnection to known peers
🔌 Connecting to 2 bootstrap peers...
  ✅ Connected to bootstrap peer: 1.2.3.4:30333
  📢 Announced ourselves to 1.2.3.4:30333
  📤 Sent FindNode to bootstrap peer 1.2.3.4:30333
📊 DETR P2P connected to 2 peers
```

## Troubleshooting

### Issue: "Could not detect public IP"

**Solution:**
```bash
export DETR_P2P_ANNOUNCE_IP="<your-public-ip>"
./trx-pbc-collator --p2p-listen "0.0.0.0:30333"
```

### Issue: "Peers can't connect to me"

**Checklist:**
1. Is port 30333 open in firewall?
   ```bash
   sudo ufw allow 30333/tcp
   ```
2. Is NAT/port forwarding configured?
3. Is announce IP correct?
   ```bash
   curl https://api.ipify.org
   ```

### Issue: "No peers connecting after bootstrap"

**Debug:**
```bash
# Check if bootstrap peer is reachable
nc -zv <bootstrap-ip> 30333

# Enable debug logging
RUST_LOG=debug ./trx-pbc-collator ...
```

## Performance Considerations

### Resource Usage

- **Memory**: ~50-100 MB for P2P layer (scales with peer count)
- **CPU**: <5% (idle), ~10-15% (under heavy message load)
- **Network**: ~1-5 KB/s per peer (heartbeat + DHT maintenance)

### Scaling

- **Max Connections**: 100 (configurable in `ConnectionManager::new()`)
- **Recommended Peers**: 20-50 for optimal DHT coverage
- **Bootstrap Peers**: 3-5 recommended

### Tuning

Edit `src/p2p.rs` to adjust:

```rust
// In P2PNetwork::new_with_announce()
let connection_manager = Arc::new(ConnectionManager::new(
    100,                              // max_connections
    Duration::from_secs(10),          // connection_timeout
    Duration::from_secs(300),         // idle_timeout
));
```

## Security

### Encryption

- All peer-to-peer messages are encrypted via X25519 key exchange + ChaCha20-Poly1305 AEAD
- Session keys are ephemeral and rotated per connection
- No plaintext metadata leakage

### Peer Reputation

- Invalid messages decrease peer score
- Peers with score < -20 are banned
- Connection failures are tracked and penalized
- Automatic eviction of bad peers from routing table

### DoS Protection

- Message size limit: 10 MB
- Connection timeout: 10 seconds
- Idle timeout: 5 minutes
- Rate limiting via peer scoring

## Future Enhancements

1. **Persistent Peer Storage**: Save known peers to disk for faster restarts
2. **NAT Traversal**: Implement hole punching for peers behind symmetric NAT
3. **Relay Support**: Allow trusted peers to relay traffic for unreachable nodes
4. **Metrics Export**: Prometheus metrics for monitoring
5. **Block Sync Protocol**: Full block synchronization via P2P (currently only announces)

## Dependencies

- `detrp2p`: Core P2P networking library
- `etrid-aecomms`: Authenticated encryption communications
- `once_cell`: Thread-safe global state
- `tokio`: Async runtime
- `log`: Logging facade

## References

- DETR P2P Library: `/Users/macbook/Desktop/etrid/01-detr-p2p/detrp2p/`
- Kademlia DHT Paper: [Maymounkov & Mazières 2002](https://pdos.csail.mit.edu/~petar/papers/maymounkov-kademlia-lncs.pdf)
- STUN Protocol: [RFC 5389](https://datatracker.ietf.org/doc/html/rfc5389)
- X25519: [RFC 7748](https://datatracker.ietf.org/doc/html/rfc7748)
- ChaCha20-Poly1305: [RFC 8439](https://datatracker.ietf.org/doc/html/rfc8439)

## Support

For issues or questions:
- GitHub: https://github.com/etrid/etrid
- Documentation: https://docs.etrid.org
- Discord: https://discord.gg/etrid
