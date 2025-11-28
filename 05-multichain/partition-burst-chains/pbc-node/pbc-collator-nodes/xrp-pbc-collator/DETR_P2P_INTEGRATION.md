# DETR P2P Integration for XRP PBC Collator

## Overview

This document describes the complete integration of the DETR P2P networking layer into the XRP Partition Burst Chain (PBC) collator. The integration provides advanced peer-to-peer networking capabilities including:

- **PeerId identity remapping** - Proper handling of peer identities when receiving Announce messages
- **Public IP auto-detection** - Automatic detection of public IP addresses using STUN protocol
- **Automatic reconnection logic** - Resilient network connections with automatic reconnection
- **Maintenance tasks** - DHT maintenance, periodic discovery, and connection health monitoring
- **Proper encryption** - X25519 + ChaCha20-Poly1305 encryption via aecomms CipherSession

## Files Modified/Created

### 1. `/Cargo.toml`
Added dependencies for DETR P2P networking:
```toml
# DETR P2P Networking
detrp2p = { path = "../../../../../01-detr-p2p/detrp2p" }
etrid-aecomms = { path = "../../../../../01-detr-p2p/aecomms" }
hex = "0.4"
rand = "0.8"
```

### 2. `/src/p2p_config.rs` (NEW)
Complete P2P configuration module with:
- `P2PConfig` struct for managing P2P configuration
- Auto-detection of public IP addresses
- Environment variable support for all P2P parameters
- CLI parameter parsing for bootstrap peers
- Peer ID generation (ephemeral or persistent via env var)

Key functions:
- `create_default_p2p_config()` - Creates config from environment variables
- `create_p2p_config_from_cli()` - Creates config from CLI parameters
- `detect_announce_address()` - Auto-detects public IP for announce address
- `parse_bootstrap_peers_from_env()` - Parses bootstrap peers from environment

### 3. `/src/cli.rs`
Added CLI parameters for P2P configuration:
```rust
/// DETR P2P listening port
#[arg(long, default_value = "30333")]
pub p2p_port: u16,

/// DETR P2P public IP address to announce to peers
#[arg(long)]
pub p2p_announce_ip: Option<String>,

/// DETR P2P bootstrap peers (format: peer_id@ip:port)
#[arg(long)]
pub p2p_bootstrap_peer: Vec<String>,

/// DETR P2P bind IP address (default: 0.0.0.0)
#[arg(long, default_value = "0.0.0.0")]
pub p2p_bind_ip: String,
```

### 4. `/src/service.rs`
Integrated DETR P2P network initialization:
- Added `P2PCliParams` struct for passing CLI parameters
- Updated `start_collator()` to accept P2P parameters
- Full P2P network initialization with announce address support
- Spawned P2P network task with all maintenance tasks
- Updated log messages from BTC-PBC to XRP-PBC

Key integration points:
```rust
// Create P2P configuration
let p2p_config = p2p_config::create_p2p_config_from_cli(...).await?;

// Create P2P network with announce address
let p2p_network = Arc::new(P2PNetwork::new_with_announce(
    p2p_config.local_peer_id,
    p2p_config.bind_address,
    Some(announce_addr),
    p2p_config.bootstrap_peers.clone(),
));

// Start P2P network
p2p_network.start().await?;

// Start all maintenance tasks
p2p_network.start_all_maintenance();
```

### 5. `/src/main.rs`
Updated to pass CLI parameters to service:
```rust
let p2p_params = service::P2PCliParams {
    p2p_port: cli.p2p_port,
    p2p_bind_ip: cli.p2p_bind_ip.clone(),
    p2p_announce_ip: cli.p2p_announce_ip.clone(),
    p2p_bootstrap_peers: cli.p2p_bootstrap_peer.clone(),
};

service::start_collator(config, p2p_params).await
```

## Environment Variables

The XRP PBC collator supports the following environment variables for P2P configuration:

### `DETR_P2P_ANNOUNCE_IP`
**Type:** IP address (IPv4 or IPv6)
**Required:** No (auto-detected if not provided)
**Description:** The public IP address to announce to other peers. This should be an externally reachable IP address. If not set, the collator will attempt to auto-detect using STUN.

**Example:**
```bash
export DETR_P2P_ANNOUNCE_IP="203.0.113.5"
```

### `DETR_P2P_PORT`
**Type:** Port number (1-65535)
**Default:** 30333
**Description:** The port to listen on for P2P connections.

**Example:**
```bash
export DETR_P2P_PORT=30333
```

### `DETR_P2P_BIND_IP`
**Type:** IP address
**Default:** 0.0.0.0
**Description:** The IP address to bind the P2P listener to. Use 0.0.0.0 to listen on all interfaces.

**Example:**
```bash
export DETR_P2P_BIND_IP="0.0.0.0"
```

### `DETR_P2P_BOOTSTRAP_PEERS`
**Type:** Comma-separated list
**Format:** `peer_id@ip:port,peer_id@ip:port,...`
**Description:** List of bootstrap peers to connect to on startup. Peer IDs should be 32-byte hex strings.

**Example:**
```bash
export DETR_P2P_BOOTSTRAP_PEERS="a1b2c3d4e5f6...@192.0.2.10:30333,f6e5d4c3b2a1...@192.0.2.11:30333"
```

### `DETR_P2P_PEER_ID`
**Type:** 64-character hex string (32 bytes)
**Required:** No (ephemeral ID generated if not provided)
**Description:** A persistent peer ID for this node. If not set, a new ephemeral ID is generated on each restart.

**Example:**
```bash
export DETR_P2P_PEER_ID="a1b2c3d4e5f6789012345678901234567890123456789012345678901234"
```

## CLI Usage

### Basic Usage
```bash
./xrp-pbc-collator \
  --chain dev \
  --p2p-port 30333 \
  --p2p-announce-ip 203.0.113.5
```

### With Bootstrap Peers
```bash
./xrp-pbc-collator \
  --chain dev \
  --p2p-port 30333 \
  --p2p-announce-ip 203.0.113.5 \
  --p2p-bootstrap-peer "a1b2c3d4e5f6789012345678901234567890123456789012345678901234@192.0.2.10:30333" \
  --p2p-bootstrap-peer "f6e5d4c3b2a1987654321098765432109876543210987654321098765432@192.0.2.11:30333"
```

### Custom Bind Address
```bash
./xrp-pbc-collator \
  --chain dev \
  --p2p-port 30333 \
  --p2p-bind-ip 10.0.0.5 \
  --p2p-announce-ip 203.0.113.5
```

## How It Works

### 1. Startup Sequence

1. **Configuration Loading**
   - CLI parameters are parsed
   - If CLI parameters are provided, they take precedence
   - Otherwise, configuration is loaded from environment variables
   - Defaults are used for any unspecified values

2. **Public IP Detection**
   - If `--p2p-announce-ip` or `DETR_P2P_ANNOUNCE_IP` is set, that value is used
   - If bind IP is not 0.0.0.0, it's used as the announce address
   - Otherwise, public IP is auto-detected using STUN protocol (Google STUN servers)
   - If auto-detection fails, a warning is logged

3. **Peer ID Generation**
   - If `DETR_P2P_PEER_ID` is set, that ID is used
   - Otherwise, an ephemeral ID is generated from timestamp + random entropy + blake2_256 hash
   - The ID is logged so it can be shared with other nodes

4. **P2P Network Initialization**
   - P2PNetwork instance is created with the announce address
   - Network binds to the configured bind address
   - Bootstrap peers are loaded and connections are established
   - Announce messages are sent to bootstrap peers
   - FindNode requests are sent to discover more peers

5. **Maintenance Tasks**
   - `start_all_maintenance()` is called, which starts:
     - DHT maintenance (bucket refresh, republishing)
     - Periodic discovery (finding new peers)
     - Auto-reconnection (reconnecting to dropped peers)

### 2. Peer Identity Remapping

When a peer connects:
1. Initial PeerId is created from the socket address (temporary)
2. When Announce message is received, the temporary PeerId is remapped to the real cryptographic PeerId
3. Connection is updated with the real PeerId and listening address
4. Peer is added to the Kademlia routing table with correct identity

### 3. Encryption

All P2P connections use:
- **X25519** key exchange for establishing shared secrets
- **ChaCha20-Poly1305** authenticated encryption for all messages
- **CipherSession** from etrid-aecomms handles the encryption/decryption

### 4. Network Maintenance

The P2P network runs continuous maintenance:
- **DHT Maintenance:** Refreshes routing table buckets, republishes stored values
- **Periodic Discovery:** Actively searches for new peers
- **Auto-reconnection:** Automatically reconnects to disconnected bootstrap peers

## Testing

### Check Compilation
```bash
cd /Users/macbook/Desktop/etrid/05-multichain/partition-burst-chains/pbc-node/pbc-collator-nodes/xrp-pbc-collator
cargo check --bin xrp-pbc-collator
```

### Build Release Binary
```bash
cargo build --bin xrp-pbc-collator --release
```

### Run with Debug Logging
```bash
RUST_LOG=debug ./target/release/xrp-pbc-collator --chain dev
```

## Architecture

```
┌─────────────────────────────────────────┐
│      XRP PBC Collator Service           │
├─────────────────────────────────────────┤
│                                         │
│  ┌─────────────────────────────────┐   │
│  │   Substrate Services            │   │
│  │   - Block Authoring (ASF)       │   │
│  │   - State Root Submission       │   │
│  │   - Consensus                   │   │
│  └─────────────────────────────────┘   │
│                                         │
│  ┌─────────────────────────────────┐   │
│  │   DETR P2P Network              │   │
│  │                                 │   │
│  │   ┌───────────────────────┐     │   │
│  │   │ ConnectionManager     │     │   │
│  │   │ - TCP connections     │     │   │
│  │   │ - Encryption          │     │   │
│  │   │ - PeerId remapping    │     │   │
│  │   └───────────────────────┘     │   │
│  │                                 │   │
│  │   ┌───────────────────────┐     │   │
│  │   │ KademliaNetwork       │     │   │
│  │   │ - DHT routing         │     │   │
│  │   │ - Peer discovery      │     │   │
│  │   │ - Value storage       │     │   │
│  │   └───────────────────────┘     │   │
│  │                                 │   │
│  │   ┌───────────────────────┐     │   │
│  │   │ MessageRouter         │     │   │
│  │   │ - Message dispatch    │     │   │
│  │   │ - Protocol handling   │     │   │
│  │   └───────────────────────┘     │   │
│  │                                 │   │
│  │   ┌───────────────────────┐     │   │
│  │   │ Maintenance Tasks     │     │   │
│  │   │ - DHT refresh         │     │   │
│  │   │ - Auto-reconnection   │     │   │
│  │   │ - Peer discovery      │     │   │
│  │   └───────────────────────┘     │   │
│  └─────────────────────────────────┘   │
│                                         │
└─────────────────────────────────────────┘
```

## Future Enhancements

1. **Persistent Peer Store** - Store discovered peers to disk for faster startup
2. **Peer Reputation** - Track peer behavior and adjust connection priorities
3. **NAT Traversal** - Add support for NAT hole-punching
4. **Metrics** - Expose Prometheus metrics for P2P network health
5. **Rate Limiting** - Add rate limiting for incoming messages
6. **Block Sync Integration** - Use DETR P2P for block synchronization

## Troubleshooting

### Problem: "Could not detect public IP"
**Solution:** Set `DETR_P2P_ANNOUNCE_IP` environment variable or use `--p2p-announce-ip` CLI flag.

### Problem: "No bootstrap peers configured"
**Solution:** Add bootstrap peers via `DETR_P2P_BOOTSTRAP_PEERS` environment variable or `--p2p-bootstrap-peer` CLI flags.

### Problem: "Failed to connect to bootstrap peer"
**Solution:** Check that:
- Bootstrap peer addresses are correct
- Bootstrap peer nodes are running
- Firewall allows outbound connections on the P2P port
- Network connectivity is available

### Problem: "Peer ID changes on restart"
**Solution:** Set `DETR_P2P_PEER_ID` environment variable with a persistent 64-character hex string.

## Security Considerations

1. **Encryption** - All P2P messages are encrypted with ChaCha20-Poly1305
2. **Authentication** - Peer identities are cryptographically verified
3. **Rate Limiting** - Consider implementing rate limiting for production
4. **Firewall** - Ensure P2P port is properly firewalled (allow only necessary incoming connections)
5. **Peer ID Privacy** - Peer IDs are derived from public keys but don't expose private information

## References

- [DETR P2P Library](../../../../../01-detr-p2p/detrp2p/src/lib.rs)
- [aecomms Encryption](../../../../../01-detr-p2p/aecomms/src/lib.rs)
- [Substrate Service Documentation](https://paritytech.github.io/polkadot-sdk/master/sc_service/index.html)
