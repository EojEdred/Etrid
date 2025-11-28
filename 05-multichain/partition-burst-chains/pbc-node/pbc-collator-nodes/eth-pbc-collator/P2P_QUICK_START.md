# ETH PBC Collator - P2P Quick Start Guide

## Quick Start

### 1. Set Your Public IP (Recommended)
```bash
export DETR_P2P_ANNOUNCE_IP=YOUR_PUBLIC_IP
```

### 2. Run the Collator
```bash
./eth-pbc-collator \
  --enable-p2p \
  --p2p-listen=0.0.0.0:30333 \
  --bootnodes=/ip4/BOOTSTRAP_IP/tcp/30333/p2p/BOOTSTRAP_PEER_ID
```

## Configuration Options

### P2P Enable/Disable
```bash
--enable-p2p=true   # Enable P2P (default)
--enable-p2p=false  # Disable P2P
```

### Listen Address
```bash
--p2p-listen=0.0.0.0:30333      # Listen on all interfaces (default)
--p2p-listen=127.0.0.1:30333    # Listen on localhost only
--p2p-listen=192.168.1.10:30333 # Listen on specific interface
```

### Announce Address

**Option 1: Environment Variable (Recommended)**
```bash
export DETR_P2P_ANNOUNCE_IP=203.0.113.5
./eth-pbc-collator --enable-p2p
```

**Option 2: CLI Argument**
```bash
./eth-pbc-collator --p2p-announce=203.0.113.5:30333
```

**Option 3: Auto-Detection (Automatic)**
```bash
./eth-pbc-collator --enable-p2p
# Will use STUN to detect public IP automatically
```

### Bootstrap Peers

**Single Bootstrap Peer:**
```bash
--bootnodes=/ip4/192.168.1.100/tcp/30333/p2p/12D3KooWEyoppNCUx8Yx66oV9fJnriXwCcXwDDUA2kj6vnc6iDEp
```

**Multiple Bootstrap Peers:**
```bash
--bootnodes=/ip4/192.168.1.100/tcp/30333/p2p/12D3KooW...,/ip4/192.168.1.101/tcp/30333/p2p/12D3KooWX...
```

## Common Scenarios

### Behind NAT/Firewall
```bash
# Set public IP explicitly
export DETR_P2P_ANNOUNCE_IP=203.0.113.5

# Forward port 30333 TCP on your router to this machine

# Run collator
./eth-pbc-collator \
  --enable-p2p \
  --p2p-listen=0.0.0.0:30333 \
  --bootnodes=/ip4/BOOTSTRAP_IP/tcp/30333/p2p/PEER_ID
```

### Local Development
```bash
# No need to set announce IP for local testing
./eth-pbc-collator \
  --enable-p2p \
  --p2p-listen=127.0.0.1:30333 \
  --bootnodes=/ip4/127.0.0.1/tcp/30334/p2p/PEER_ID
```

### Cloud/VPS Deployment
```bash
# Auto-detection usually works
./eth-pbc-collator \
  --enable-p2p \
  --p2p-listen=0.0.0.0:30333 \
  --bootnodes=/ip4/BOOTSTRAP_IP/tcp/30333/p2p/PEER_ID

# Or set explicitly if auto-detection fails
export DETR_P2P_ANNOUNCE_IP=$(curl -s ifconfig.me)
./eth-pbc-collator --enable-p2p
```

## Monitoring

### Log Levels
```bash
# Default logging
RUST_LOG=info ./eth-pbc-collator

# Verbose P2P logging
RUST_LOG=detrp2p=debug,eth_pbc_collator=info ./eth-pbc-collator

# Trace-level P2P logging
RUST_LOG=detrp2p=trace,eth_pbc_collator=info ./eth-pbc-collator
```

### Network Statistics
Watch for periodic stats logs (every 60 seconds):
```
📊 Network Statistics:
   Connected peers: 5
   Bootstrap peers: 2
   Listen address: 0.0.0.0:30333
   Announce address: 203.0.113.5:30333
   Node ID: [0xab, 0xcd, 0xef, ...]
```

## Troubleshooting

### No Peers Connected
1. Check bootstrap peer addresses are correct
2. Verify port 30333 is open (firewall/router)
3. Ensure DETR_P2P_ANNOUNCE_IP is set correctly
4. Check logs for connection errors

### Auto-Detection Fails
```bash
# Fallback: Set IP manually
export DETR_P2P_ANNOUNCE_IP=$(curl -s ifconfig.me)
./eth-pbc-collator --enable-p2p
```

### Port Already in Use
```bash
# Use different port
./eth-pbc-collator --p2p-listen=0.0.0.0:30334
```

### Connection Timeout
```bash
# Check network connectivity
ping BOOTSTRAP_IP

# Check port is open
nc -zv BOOTSTRAP_IP 30333

# Verify announce IP is reachable from peers
```

## Features Enabled

✅ **Automatic Public IP Detection** - STUN-based with HTTP fallback
✅ **PeerId Identity Remapping** - Correct cryptographic identities
✅ **Automatic Reconnection** - Handles transient network failures
✅ **Encryption** - X25519 + ChaCha20-Poly1305 via aecomms
✅ **DHT Maintenance** - Automatic peer discovery and routing
✅ **Block Sync** - Announce and request blocks over P2P

## Environment Variables

```bash
DETR_P2P_ANNOUNCE_IP    # Your public IP address
RUST_LOG                # Logging configuration
```

## Default Values

- P2P Enable: `true`
- Listen Address: `0.0.0.0:30333`
- Announce Address: Auto-detected or from DETR_P2P_ANNOUNCE_IP
- Bootstrap Peers: None (must be provided)

## Security Notes

- All P2P traffic is encrypted with ChaCha20-Poly1305
- PeerIDs are cryptographically verified
- Automatic session key rotation per connection
- No plaintext message transmission

## Support

For issues or questions:
1. Check logs with `RUST_LOG=debug`
2. Verify network configuration
3. Consult full documentation: `ETH_PBC_P2P_INTEGRATION_COMPLETE.md`
