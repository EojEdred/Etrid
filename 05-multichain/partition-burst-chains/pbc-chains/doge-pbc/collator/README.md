# DOGE-PBC Collator

Dogecoin Partition Burst Chain collator with full DETR P2P integration.

## Features

- **DETR P2P Networking**: Custom P2P layer with auto-reconnection, peer discovery, and DHT
- **Dogecoin Bridge**: 20-confirmation bridge for DOGE ↔ ETR swaps
- **Lightning Channels**: Instant payment channels for low-latency transactions
- **ASF Consensus**: Asynchronous Semi-Finality consensus for efficient block production
- **Public IP Detection**: Automatic public IP detection for NAT traversal
- **Peer Identity Remapping**: Cryptographic identity verification after initial connection

## Building

```bash
# Build the collator
cargo build --release -p doge-pbc-collator

# The binary will be at:
# target/release/doge-pbc-collator
```

## Running

### Development Mode

```bash
./target/release/doge-pbc-collator --dev
```

### With DETR P2P Configuration

```bash
# Set environment variables
export DETR_P2P_LISTEN_ADDR="0.0.0.0:30333"
export DETR_P2P_ANNOUNCE_IP="203.0.113.42"  # Your public IP
export DETR_P2P_BOOTSTRAP_PEERS="198.51.100.10:30333,203.0.113.50:30333"
export DETR_P2P_MAX_CONNECTIONS="100"

# Run the collator
./target/release/doge-pbc-collator \
  --chain doge-pbc \
  --base-path /tmp/doge-pbc \
  --port 30334 \
  --rpc-port 9945 \
  --rpc-cors all
```

### Using CLI Arguments

```bash
./target/release/doge-pbc-collator \
  --chain doge-pbc \
  --base-path /tmp/doge-pbc \
  --p2p-listen 0.0.0.0:30333 \
  --p2p-announce-ip 203.0.113.42 \
  --p2p-bootstrap "198.51.100.10:30333,203.0.113.50:30333" \
  --p2p-max-connections 100 \
  --port 30334 \
  --rpc-port 9945
```

## DETR P2P Configuration

### Environment Variables

| Variable | Description | Default | Example |
|----------|-------------|---------|---------|
| `DETR_P2P_LISTEN_ADDR` | Local address to listen on | `0.0.0.0:30333` | `0.0.0.0:30333` |
| `DETR_P2P_ANNOUNCE_IP` | Public IP to announce to peers | Auto-detected | `203.0.113.42` |
| `DETR_P2P_BOOTSTRAP_PEERS` | Comma-separated bootstrap peers | None | `198.51.100.10:30333,203.0.113.50:30333` |
| `DETR_P2P_MAX_CONNECTIONS` | Maximum peer connections | `100` | `200` |

### NAT Traversal

If your node is behind NAT, set `DETR_P2P_ANNOUNCE_IP` to your public IP:

```bash
# Auto-detect will try STUN first, then HTTP APIs
# But explicit setting is more reliable:
export DETR_P2P_ANNOUNCE_IP="$(curl -s https://api.ipify.org)"
```

### Bootstrap Peers

Bootstrap peers help your node find other nodes in the network. Format:

```bash
export DETR_P2P_BOOTSTRAP_PEERS="ip1:port1,ip2:port2,ip3:port3"
```

## DETR P2P Features

### 1. Auto-Reconnection

The collator automatically reconnects to peers that disconnect:

- Monitors connection health every 30 seconds
- Reconnects to peers that have been disconnected for > 5 minutes
- Exponential backoff for failed reconnection attempts

### 2. Peer Identity Remapping

- Initial connections use temporary IP-based peer IDs
- After receiving `Announce` message, peer ID is remapped to cryptographic identity
- Ensures proper routing and prevents peer ID conflicts

### 3. Public IP Detection

The collator automatically detects your public IP using:

1. **STUN Protocol**: Fast UDP-based detection (Google STUN servers)
2. **HTTP APIs**: Fallback to ipify.org and similar services
3. **Environment Variable**: Manual override with `DETR_P2P_ANNOUNCE_IP`

### 4. Encrypted Communication

All peer-to-peer communication is encrypted using:

- **X25519**: Elliptic curve key exchange
- **ChaCha20-Poly1305**: Authenticated encryption
- Via `etrid-aecomms` crate

### 5. DHT and Discovery

- Distributed Hash Table for peer routing
- Periodic peer discovery broadcasts
- Kademlia-based distance metrics for efficient routing

## Monitoring

The collator logs P2P status every 30 seconds:

```
📊 DETR P2P Status:
   Connected peers: 15
   Active peer addresses:
     - 198.51.100.10:30333
     - 203.0.113.50:30333
     ... and 13 more
```

## Dogecoin Bridge

The DOGE bridge enables trustless swaps between DOGE and ETR:

- **Conversion Rate**: 1 DOGE = 0.001 ETR (configurable)
- **Confirmations**: 20 Dogecoin blocks required
- **Bridge Fee**: 1% (configurable)
- **Min Amount**: 0.001 ETR
- **Max Amount**: 1,000,000 ETR

## Lightning Channels

Lightning channels enable instant, low-fee payments:

- **Min Capacity**: 0.001 ETR
- **Max Capacity**: 1000 ETR
- **Timeout**: 24 hours
- Bidirectional payment channels
- State channel updates without on-chain transactions

## Troubleshooting

### No peers connecting

1. Check firewall allows inbound on P2P port (default 30333)
2. Verify `DETR_P2P_ANNOUNCE_IP` is set to your public IP
3. Ensure bootstrap peers are reachable
4. Check logs for connection errors

### Public IP not detected

Set manually:

```bash
export DETR_P2P_ANNOUNCE_IP="$(curl -s https://api.ipify.org)"
```

### Build errors

Ensure you have the latest dependencies:

```bash
cargo update
cargo clean
cargo build --release -p doge-pbc-collator
```

## Architecture

```
┌─────────────────────────────────────────────────────┐
│           DOGE-PBC Collator Architecture            │
├─────────────────────────────────────────────────────┤
│                                                     │
│  ┌─────────────────────────────────────────────┐   │
│  │         Substrate Service Layer             │   │
│  │  • Block Production (ASF Consensus)         │   │
│  │  • Transaction Pool                         │   │
│  │  • RPC Server                               │   │
│  └─────────────────────────────────────────────┘   │
│                      │                              │
│                      ▼                              │
│  ┌─────────────────────────────────────────────┐   │
│  │           DETR P2P Network Layer            │   │
│  │  • Connection Manager                       │   │
│  │  • Auto-Reconnection                        │   │
│  │  • Peer Discovery (DHT)                     │   │
│  │  • Identity Remapping                       │   │
│  │  • Public IP Detection                      │   │
│  │  • Encrypted Sessions (aecomms)             │   │
│  └─────────────────────────────────────────────┘   │
│                      │                              │
│                      ▼                              │
│  ┌─────────────────────────────────────────────┐   │
│  │           Runtime (doge-pbc-runtime)        │   │
│  │  • Dogecoin Bridge (20 confirmations)      │   │
│  │  • Lightning Channels                       │   │
│  │  • ETR Lock (for bridge operations)        │   │
│  │  • ASF Consensus Pallet                    │   │
│  └─────────────────────────────────────────────┘   │
│                                                     │
└─────────────────────────────────────────────────────┘
```

## Integration Points

### With FlareChain Relay

The collator submits state roots to FlareChain every 6 seconds for:

- Cross-chain message verification
- Partition state finality
- Security guarantees from relay chain

### With Other PBCs

Communicates via DETR P2P with:

- BTC-PBC (Bitcoin bridge)
- ETH-PBC (Ethereum compatibility)
- Other partition chains

### With External Networks

- **Dogecoin Network**: Monitors DOGE confirmations
- **Lightning Network**: Opens/closes payment channels

## License

Apache 2.0

## Support

For issues and questions, visit: https://github.com/etrid/etrid
