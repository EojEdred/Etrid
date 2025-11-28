# TRX PBC Collator - Quick Start Guide

## Build

```bash
cd /Users/macbook/Desktop/etrid/05-multichain/partition-burst-chains/pbc-node/pbc-collator-nodes/trx-pbc-collator

# Verify dependencies
./verify_build.sh

# Build release binary
cargo build --release

# Binary will be at:
# ./target/release/trx-pbc-collator
```

## Run Locally (Development)

### Single Node

```bash
./target/release/trx-pbc-collator --dev
```

### Multi-Node Local Network

**Terminal 1 - Bootstrap Node:**
```bash
./target/release/trx-pbc-collator \
  --dev \
  --p2p-listen "127.0.0.1:30333"
```

**Terminal 2 - Second Node:**
```bash
./target/release/trx-pbc-collator \
  --dev \
  --p2p-listen "127.0.0.1:30334" \
  --p2p-bootstrap "127.0.0.1:30333"
```

**Terminal 3 - Third Node:**
```bash
./target/release/trx-pbc-collator \
  --dev \
  --p2p-listen "127.0.0.1:30335" \
  --p2p-bootstrap "127.0.0.1:30333,127.0.0.1:30334"
```

## Run on Server (Production)

### Option 1: Auto-detect Public IP

```bash
# DETR P2P will automatically detect your public IP via STUN
./target/release/trx-pbc-collator \
  --p2p-listen "0.0.0.0:30333" \
  --p2p-bootstrap "boot1.example.com:30333,boot2.example.com:30333"
```

### Option 2: Explicit Public IP

```bash
export DETR_P2P_ANNOUNCE_IP="203.0.113.42"

./target/release/trx-pbc-collator \
  --p2p-listen "0.0.0.0:30333" \
  --p2p-bootstrap "boot1.example.com:30333"
```

### Option 3: CLI Flag

```bash
./target/release/trx-pbc-collator \
  --p2p-listen "0.0.0.0:30333" \
  --p2p-announce "203.0.113.42:30333" \
  --p2p-bootstrap "boot1.example.com:30333"
```

## Verify P2P is Working

Look for these log messages:

```
✅ Successful Startup:
🚀 Starting DETR P2P network for TRX PBC Collator...
📢 Detected public IP via STUN: 203.0.113.42
✅ DETR P2P network started successfully
🔧 DETR P2P maintenance tasks started:
   ✓ DHT maintenance (bucket refresh, storage cleanup)
   ✓ Periodic peer discovery
   ✓ Automatic reconnection to known peers
🔌 Connecting to 2 bootstrap peers...
  ✅ Connected to bootstrap peer: 203.0.113.10:30333
📊 DETR P2P connected to 2 peers
```

```
✅ Block Production & Broadcasting:
🔗 TRX-PBC: Block #123 produced with state root: 0x1234...
📡 TRX-PBC: Broadcasting block #123 to 5 peers
```

```
✅ Peer Discovery:
📢 Received Announce from PeerId([...]) - listening at 203.0.113.20:30333
  🔑 Remapping temp PeerId → real cryptographic PeerId
  ✅ Added peer 203.0.113.20:30333 to routing table
```

## Troubleshooting

### Issue: "Could not detect public IP"

**Fix:**
```bash
# Check your public IP
curl https://api.ipify.org

# Then set it explicitly
export DETR_P2P_ANNOUNCE_IP="$(curl -s https://api.ipify.org)"
./target/release/trx-pbc-collator --p2p-listen "0.0.0.0:30333"
```

### Issue: "No peers connecting"

**Checklist:**

1. **Is port open?**
   ```bash
   sudo ufw allow 30333/tcp
   # Or for firewalld:
   sudo firewall-cmd --permanent --add-port=30333/tcp
   sudo firewall-cmd --reload
   ```

2. **Is port reachable externally?**
   ```bash
   # From another machine:
   nc -zv YOUR_IP 30333
   ```

3. **Are bootstrap peers correct?**
   ```bash
   # Test bootstrap connectivity:
   nc -zv boot1.example.com 30333
   ```

### Issue: "Peers disconnect frequently"

**Possible causes:**
- Unstable network connection
- Firewall blocking connections
- NAT timeout too short
- System resource limits

**Fix:**
```bash
# Increase file descriptor limit
ulimit -n 65536

# Or set in /etc/security/limits.conf:
etrid soft nofile 65536
etrid hard nofile 65536
```

## CLI Options Reference

### P2P Options

| Option | Default | Description |
|--------|---------|-------------|
| `--p2p-listen` | `0.0.0.0:30333` | Address to listen for P2P connections |
| `--p2p-announce` | Auto-detect | Public address to announce to peers |
| `--p2p-bootstrap` | None | Comma-separated bootstrap peers |
| `--enable-p2p` | `true` | Enable/disable DETR P2P networking |

### Collator Options

| Option | Default | Description |
|--------|---------|-------------|
| `--pbc-id` | `0` | PBC chain ID (0-11) |
| `--relay-chain-rpc` | `ws://127.0.0.1:9944` | Relay chain RPC endpoint |

### Standard Substrate Options

| Option | Default | Description |
|--------|---------|-------------|
| `--dev` | Off | Development mode (implies --chain=dev) |
| `--chain` | `local` | Chain specification |
| `--validator` | Off | Run as validator |
| `--port` | `30333` | Substrate P2P port (separate from DETR P2P) |
| `--rpc-port` | `9933` | RPC HTTP server port |
| `--ws-port` | `9944` | RPC WebSocket server port |
| `--base-path` | Temp | Base path for blockchain data |
| `--name` | Random | Node name for telemetry |

## Environment Variables

| Variable | Description | Example |
|----------|-------------|---------|
| `DETR_P2P_ANNOUNCE_IP` | Override public IP detection | `203.0.113.42` |
| `RUST_LOG` | Logging level | `info`, `debug`, `trace` |

## Logging

### Enable Debug Logging

```bash
RUST_LOG=debug ./target/release/trx-pbc-collator ...
```

### P2P-Only Debug Logging

```bash
RUST_LOG=detrp2p=debug,trx_pbc_collator=info ./target/release/trx-pbc-collator ...
```

### Full Trace Logging

```bash
RUST_LOG=trace ./target/release/trx-pbc-collator ...
```

## Performance Tuning

### Recommended System Requirements

**Minimum:**
- CPU: 2 cores
- RAM: 4 GB
- Disk: 100 GB SSD
- Network: 10 Mbps

**Recommended:**
- CPU: 4+ cores
- RAM: 8+ GB
- Disk: 500 GB NVMe SSD
- Network: 100 Mbps

### Optimize for Production

```bash
./target/release/trx-pbc-collator \
  --chain mainnet \
  --validator \
  --p2p-listen "0.0.0.0:30333" \
  --p2p-bootstrap "boot1.etrid.org:30333,boot2.etrid.org:30333" \
  --relay-chain-rpc "wss://relay.etrid.org:443" \
  --execution native \
  --state-cache-size 1073741824 \
  --db-cache 4096 \
  --prometheus-external \
  --prometheus-port 9615
```

## Next Steps

1. **Review Full Documentation**: See `README_DETR_P2P_INTEGRATION.md`
2. **Example Configurations**: See `example-configs.sh`
3. **Join the Network**: Contact Etrid team for bootstrap peer addresses
4. **Monitor Your Node**: Set up Prometheus + Grafana monitoring
5. **Report Issues**: https://github.com/etrid/etrid/issues

## Support

- Documentation: https://docs.etrid.org
- Discord: https://discord.gg/etrid
- GitHub: https://github.com/etrid/etrid
- Email: support@etrid.org
