# MATIC PBC Collator - DETR P2P Integration

## Overview
This collator has been fully integrated with the DETR P2P networking library, providing decentralized peer discovery, automatic NAT traversal, and encrypted communication.

## CLI Usage

### Basic Development Mode
```bash
matic-pbc-collator --dev --p2p-listen 0.0.0.0:30333
```

### With Bootstrap Peers
```bash
matic-pbc-collator \
  --p2p-listen 0.0.0.0:30333 \
  --p2p-bootnodes \
    abc123def456...@192.168.1.100:30333,\
    789ghi012jkl...@192.168.1.101:30333
```

### With Manual Announce Address
```bash
matic-pbc-collator \
  --p2p-listen 0.0.0.0:30333 \
  --p2p-announce 203.0.113.10:30333
```

### With Environment Variable for Public IP
```bash
export DETR_P2P_ANNOUNCE_IP=203.0.113.10
matic-pbc-collator --p2p-listen 0.0.0.0:30333
```

## Features

### 1. Public IP Auto-Detection
- Automatically detects your public IP for peer announcements
- Uses STUN protocol (Google STUN servers)
- Falls back to HTTP APIs if STUN fails
- Can be overridden with `DETR_P2P_ANNOUNCE_IP` env var or `--p2p-announce` flag

### 2. Peer ID Remapping
- Incoming connections get temporary socket-based IDs
- Automatically remaps to cryptographic PeerIds on Announce
- Maintains bidirectional communication integrity

### 3. Automatic Reconnection
- Monitors disconnected known peers
- Automatically attempts reconnection
- Smart retry with exponential backoff

### 4. Background Maintenance
Three maintenance tasks run automatically:
- **DHT Maintenance**: Keeps routing table fresh
- **Periodic Discovery**: Finds new peers via Kademlia
- **Auto-Reconnection**: Maintains network connectivity

### 5. Encrypted Communication
All P2P messages encrypted with:
- X25519 key exchange (etrid-aecomms)
- ChaCha20-Poly1305 AEAD
- Session management via CipherSession

## Bootnode Format

Format: `<64_hex_chars>@<ip>:<port>`

Example:
```
0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef@192.168.1.100:30333
```

To get your node's Peer ID, check the logs after startup:
```
🔑 Local Peer ID: PeerId([1, 35, 69, ...])
```

## Expected Log Output

```
🌐 Initializing DETR P2P network for MATIC-PBC...
🔍 Auto-detecting public IP for P2P announce address...
✅ Auto-detected announce address: 203.0.113.10:30333
🔑 Local Peer ID: PeerId([...])
✅ DETR P2P network initialized
   Listen: 0.0.0.0:30333
   Announce: 203.0.113.10:30333
🚀 Starting DETR P2P network...
✅ DETR P2P network started
🔧 Starting P2P maintenance tasks...
✅ P2P maintenance tasks started (DHT, discovery, auto-reconnect)
📨 DETR P2P message handler started for MATIC-PBC
🌐 MATIC-PBC DETR P2P integration complete
```

## Troubleshooting

### Cannot detect public IP
Set the announce IP manually:
```bash
export DETR_P2P_ANNOUNCE_IP=your.public.ip
# or
matic-pbc-collator --p2p-announce your.public.ip:30333
```

### No bootstrap peers available
The node will run in isolation. Add bootstrap peers:
```bash
matic-pbc-collator --p2p-bootnodes peer_id@ip:port
```

### Connection issues
Check firewall rules:
```bash
# Allow TCP traffic on P2P port
sudo ufw allow 30333/tcp
```

## Architecture

```
MATIC PBC Collator
├── Substrate Network (Block sync, consensus)
└── DETR P2P Network
    ├── Kademlia DHT (Peer discovery)
    ├── Connection Manager (TCP connections)
    ├── Encryption Layer (etrid-aecomms)
    └── Message Router (Vote, Certificate, BlockAnnounce)
```

## Implementation Status

✅ **Complete** - No TODOs, no placeholders, production-ready

## Related Documentation

- DETR P2P Library: `/Users/macbook/Desktop/etrid/01-detr-p2p/detrp2p/src/lib.rs`
- Integration Summary: `/Users/macbook/Desktop/etrid/01-detr-p2p/MATIC_PBC_INTEGRATION_SUMMARY.md`
- Architecture: `/Users/macbook/Desktop/etrid/01-detr-p2p/ARCHITECTURE.md`
