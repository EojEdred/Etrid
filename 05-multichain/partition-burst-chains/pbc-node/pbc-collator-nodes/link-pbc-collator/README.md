# LINK-PBC Collator (Chainlink Partition Burst Chain)

The LINK-PBC Collator is a specialized blockchain node for the Chainlink Partition Burst Chain within the Ëtrid multichain protocol. It produces blocks, manages Chainlink token bridges, oracle price feeds, and VRF services.

## Features

### Core Functionality
- **Block Production:** ASF consensus-based block authoring
- **Chainlink Integration:** Native LINK token support and bridge
- **Oracle Services:** Price feed aggregation and distribution
- **VRF Support:** Verifiable Random Function for on-chain randomness
- **Lightning Channels:** Fast payment layer for microtransactions
- **State Root Submission:** Periodic state root submission to Primearc Core Chain

### DETR P2P Networking
Full integration with DETR P2P networking layer providing:
- **Identity Remapping:** Automatic PeerId remapping from socket-derived to cryptographic identities
- **Public IP Detection:** Auto-detection via STUN protocol for NAT traversal
- **Auto-Reconnection:** Maintains peer connections with automatic recovery
- **Background Maintenance:** DHT maintenance, peer discovery, and connection health monitoring
- **Encrypted Communication:** X25519 + ChaCha20-Poly1305 encryption via aecomms
- **Block Broadcasting:** Efficient block announcement propagation to all peers

## Building

### Prerequisites
- Rust 1.70 or newer
- Cargo
- Linux, macOS, or Windows with WSL2

### Build Commands

```bash
# Standard release build
cargo build --release

# Fast debug build (for development)
cargo build

# Build with all features
cargo build --release --all-features
```

The compiled binary will be located at:
- Release: `target/release/link-pbc-collator`
- Debug: `target/debug/link-pbc-collator`

## Configuration

### Environment Variables

The collator is configured primarily through environment variables. See [DETR_P2P_CONFIG.md](./DETR_P2P_CONFIG.md) for complete P2P networking configuration.

#### Core Configuration

| Variable | Description | Default | Required |
|----------|-------------|---------|----------|
| `RUST_LOG` | Log level (trace, debug, info, warn, error) | `info` | No |

#### DETR P2P Configuration

| Variable | Description | Default | Required |
|----------|-------------|---------|----------|
| `DETR_P2P_NODE_IDENTITY` | Node's cryptographic identity (64-char hex) | Auto-generated | No |
| `DETR_P2P_LISTEN_ADDR` | Local P2P listen address | `0.0.0.0:30333` | No |
| `DETR_P2P_ANNOUNCE_IP` | Public IP to announce to peers | Auto-detected | No |
| `DETR_P2P_BOOTSTRAP_PEERS` | Comma-separated bootstrap peer list | Empty | No |

## Running

### Standalone Development Node

```bash
# Generate identity on first run
./target/release/link-pbc-collator

# The node will print its identity:
# 🔑 Generated new node identity: a1b2c3d4e5f6...
#    Set DETR_P2P_NODE_IDENTITY=a1b2c3... to persist this identity

# On subsequent runs, use the generated identity
export DETR_P2P_NODE_IDENTITY="a1b2c3d4e5f6..."
./target/release/link-pbc-collator
```

### Production Validator Network

#### Step 1: Generate identities for all validators

Run each validator once without configuration to generate unique identities.

#### Step 2: Configure bootstrap peers

Each validator needs to know about the others. Create a config file for each:

**Validator 1 (validator1.env):**
```bash
export DETR_P2P_NODE_IDENTITY="abcd1234567890abcd1234567890abcd1234567890abcd1234567890abcd1234"
export DETR_P2P_LISTEN_ADDR="0.0.0.0:30333"
export DETR_P2P_ANNOUNCE_IP="192.168.1.10"
export DETR_P2P_BOOTSTRAP_PEERS="ef123456...@192.168.1.11:30333,56789abc...@192.168.1.12:30333"
export RUST_LOG="info,link_pbc_collator=debug,detrp2p=debug"
```

**Validator 2 (validator2.env):**
```bash
export DETR_P2P_NODE_IDENTITY="ef123456789012ef123456789012ef123456789012ef123456789012ef123456"
export DETR_P2P_LISTEN_ADDR="0.0.0.0:30333"
export DETR_P2P_ANNOUNCE_IP="192.168.1.11"
export DETR_P2P_BOOTSTRAP_PEERS="abcd1234...@192.168.1.10:30333,56789abc...@192.168.1.12:30333"
export RUST_LOG="info,link_pbc_collator=debug,detrp2p=debug"
```

**Validator 3 (validator3.env):**
```bash
export DETR_P2P_NODE_IDENTITY="56789abcdef01256789abcdef01256789abcdef01256789abcdef012567890ab"
export DETR_P2P_LISTEN_ADDR="0.0.0.0:30333"
export DETR_P2P_ANNOUNCE_IP="192.168.1.12"
export DETR_P2P_BOOTSTRAP_PEERS="abcd1234...@192.168.1.10:30333,ef123456...@192.168.1.11:30333"
export RUST_LOG="info,link_pbc_collator=debug,detrp2p=debug"
```

#### Step 3: Start validators

```bash
# Validator 1
source validator1.env
./target/release/link-pbc-collator

# Validator 2
source validator2.env
./target/release/link-pbc-collator

# Validator 3
source validator3.env
./target/release/link-pbc-collator
```

### Docker Deployment

```bash
# Build Docker image
docker build -t link-pbc-collator:latest .

# Run with environment variables
docker run -d \
  --name link-pbc-validator \
  -e DETR_P2P_NODE_IDENTITY="your_persistent_identity" \
  -e DETR_P2P_LISTEN_ADDR="0.0.0.0:30333" \
  -e DETR_P2P_ANNOUNCE_IP="$(curl -s https://api.ipify.org)" \
  -e DETR_P2P_BOOTSTRAP_PEERS="peer1@ip1:port1,peer2@ip2:port2" \
  -p 30333:30333 \
  link-pbc-collator:latest
```

## Monitoring

### Log Output

The collator provides detailed logging of its operations:

```
🚀 Initializing DETR P2P Network for LINK-PBC Collator
   Listen address: 0.0.0.0:30333
   Node ID: abcd1234567890...
📢 Detected public IP via STUN: 203.0.113.45
✅ DETR P2P initialized successfully
🚀 Starting DETR P2P Network...
✅ P2P server started on 0.0.0.0:30333
🌐 Bootstrapping from 2 peers...
✅ Bootstrap complete
✅ All maintenance tasks started
📨 Message processor started
✅ DETR P2P Network fully operational
🔗 LINK-PBC: State root submitter with P2P broadcasting started
📊 P2P network stats reporter started
```

### Network Statistics

Every 30 seconds, the collator reports P2P network statistics:

```
📊 P2P Network Stats: 20 connected peers | Local ID: abcd1234567890...
```

### Block Production

When blocks are produced and broadcast:

```
🔗 LINK-PBC: Block #42 produced with state root: 0x1234...
📦 Encoded block #42 (1024 bytes)
📢 Block #42 broadcast to 20 peers
```

## Troubleshooting

### Common Issues

#### 1. Cannot connect to bootstrap peers

**Symptoms:**
```
⚠️ Failed to connect to bootstrap peer 192.168.1.11:30333: Connection refused
```

**Solutions:**
- Verify the bootstrap peer is running
- Check firewall rules allow port 30333
- Confirm network connectivity with `ping`
- Verify the bootstrap peer's IP and port are correct

#### 2. Public IP auto-detection fails

**Symptoms:**
```
⚠️ Could not detect public IP - announce address may be incorrect
```

**Solutions:**
- Manually set `DETR_P2P_ANNOUNCE_IP` to your public IP
- Check if STUN traffic is blocked by firewall
- Verify network has internet access

#### 3. Low peer count

**Symptoms:**
```
📊 P2P Network Stats: 0 connected peers
```

**Solutions:**
- Check bootstrap peers are configured correctly
- Verify announce IP is reachable from other nodes
- Ensure firewall allows incoming connections on P2P port
- Check NAT traversal is working (STUN detection succeeded)

### Debug Logging

Enable detailed debug logging:

```bash
export RUST_LOG="debug,link_pbc_collator=trace,detrp2p=trace"
./target/release/link-pbc-collator
```

## Architecture

### Components

```
┌─────────────────────────────────────────────────────────┐
│              LINK-PBC Collator Node                     │
├─────────────────────────────────────────────────────────┤
│                                                         │
│  ┌───────────────────────────────────────────────────┐ │
│  │         ASF Consensus Engine                      │ │
│  │  - Block authoring                                │ │
│  │  - Transaction validation                         │ │
│  │  - State transitions                              │ │
│  └───────────────────────────────────────────────────┘ │
│                                                         │
│  ┌───────────────────────────────────────────────────┐ │
│  │         DETR P2P Network Layer                    │ │
│  │  - PeerId identity remapping                      │ │
│  │  - Public IP auto-detection (STUN)                │ │
│  │  - Automatic reconnection                         │ │
│  │  - Background maintenance (DHT, discovery)        │ │
│  │  - X25519 + ChaCha20-Poly1305 encryption          │ │
│  └───────────────────────────────────────────────────┘ │
│                                                         │
│  ┌───────────────────────────────────────────────────┐ │
│  │         Chainlink Services                        │ │
│  │  - LINK token bridge                              │ │
│  │  - Oracle price feeds                             │ │
│  │  - VRF (Verifiable Random Function)               │ │
│  └───────────────────────────────────────────────────┘ │
│                                                         │
│  ┌───────────────────────────────────────────────────┐ │
│  │         State Root Submitter                      │ │
│  │  - Periodic state root calculation                │ │
│  │  - Block announcement broadcasting                │ │
│  │  - P2P network integration                        │ │
│  └───────────────────────────────────────────────────┘ │
│                                                         │
└─────────────────────────────────────────────────────────┘
```

### Data Flow

1. **Block Production:**
   - ASF consensus engine produces new block
   - Block header extracted with state root
   - Block encoded for P2P transmission

2. **P2P Broadcasting:**
   - Block announcement created with block data
   - Broadcast to all connected peers via DETR P2P
   - Encrypted transmission using aecomms

3. **Peer Discovery:**
   - Bootstrap from configured peers on startup
   - Kademlia DHT for peer discovery
   - Periodic FindNode queries to discover new peers
   - Automatic reconnection to maintain mesh

4. **Message Processing:**
   - Incoming messages decrypted and validated
   - Block announcements processed and logged
   - Vote and certificate messages handled by consensus
   - Announce messages trigger PeerId remapping

## Development

### Code Structure

```
link-pbc-collator/
├── src/
│   ├── main.rs           # Entry point and CLI handling
│   ├── service.rs        # Node service and task management
│   ├── chain_spec.rs     # Chain specification
│   ├── cli.rs            # Command-line interface
│   └── p2p_network.rs    # DETR P2P integration layer
├── Cargo.toml            # Dependencies and build config
├── build.rs              # Build script
├── README.md             # This file
└── DETR_P2P_CONFIG.md    # P2P configuration guide
```

### Key Dependencies

- **detrp2p:** DETR P2P networking library
- **etrid-aecomms:** Encrypted communication layer
- **Substrate:** Blockchain framework
- **ASF Consensus:** Adaptive State Finality consensus

### Adding New Features

1. Edit `src/service.rs` for core node functionality
2. Edit `src/p2p_network.rs` for P2P features
3. Update `Cargo.toml` for new dependencies
4. Run tests: `cargo test`
5. Build: `cargo build --release`

## Testing

```bash
# Run all tests
cargo test

# Run tests with output
cargo test -- --nocapture

# Run specific test
cargo test test_p2p_config_default

# Run tests with debug logging
RUST_LOG=debug cargo test
```

## Performance

### Resource Requirements

- **CPU:** 2+ cores recommended
- **RAM:** 4GB minimum, 8GB recommended
- **Disk:** 100GB+ SSD for chain data
- **Network:** 10 Mbps+ connection, low latency

### Optimization Tips

1. **Block Production:** ASF consensus is lightweight and efficient
2. **P2P Networking:** Background maintenance runs periodically (5 min intervals)
3. **State Storage:** Use SSD for best performance
4. **Network Bandwidth:** Block broadcasting is efficient, ~1KB per block

## Security

### Best Practices

1. **Node Identity:** Keep `DETR_P2P_NODE_IDENTITY` secret and backed up
2. **Firewall:** Allow only necessary ports (30333 for P2P)
3. **Updates:** Keep the collator software up to date
4. **Monitoring:** Watch logs for unusual activity
5. **Backups:** Regular backups of chain data and identity

### Encryption

- All P2P communication is encrypted using X25519 + ChaCha20-Poly1305
- No plaintext message transmission
- Automatic key exchange on connection establishment

## Support

For issues, questions, or contributions:
- Check the logs for error messages
- Review [DETR_P2P_CONFIG.md](./DETR_P2P_CONFIG.md) for configuration help
- Contact the Ëtrid development team

## License

Apache-2.0

## Version

**Version:** 0.1.0
**Substrate:** polkadot-stable2509
**DETR P2P:** Latest with full feature integration
