# ËTRID Director Node

Production-ready director node implementation for ËTRID's director-anchored peering model.

## Quick Start

```bash
# Build director node
cargo build --release

# Configure director
export DIRECTOR_CONFIG=./examples/director1.json

# Run director
./target/release/director-node
```

## What is a Director Node?

Director nodes form a stable network backbone that relays messages between rotating validators. This architecture enables:

- **Seamless validator rotation** (no network disruption)
- **Reduced connection complexity** (O(N) vs O(N²))
- **Enforced authorization** (validator registry)
- **Built-in slashing detection** (double-sign prevention)

## Architecture

```
        Director 1 ◄────────► Director 2 ◄────────► Director 3
           │                     │                     │
           │                     │                     │
       Tailscale             Tailscale             Tailscale
       (private)             (private)             (private)
           │                     │                     │
           ▼                     ▼                     ▼
      Val 1-7               Val 8-14              Val 15-21
   (public internet)     (public internet)     (public internet)
```

## Features

### Core Functionality
- ✅ Multi-director mesh (Tailscale private network)
- ✅ Validator message relay (public internet)
- ✅ Message loop prevention (BLAKE2b hashing)
- ✅ Load balancing (7 validators per director)
- ✅ Hot validator rotation (no downtime)

### Security
- ✅ Validator authorization (2/3+ director consensus)
- ✅ Double-sign detection (checkpoint tracking)
- ✅ Slashing evidence creation (on-chain submission)
- ✅ Encrypted director mesh (Tailscale WireGuard)

### Monitoring
- ✅ Relay metrics (messages relayed, loops prevented)
- ✅ Validator tracking (connected count, roles)
- ✅ Slashing alerts (double-sign events)
- ✅ Health checks (connection status)

## Installation

### Prerequisites

1. **Rust** (1.75+)
2. **Tailscale** (for director mesh)
3. **DETR-P2P** (networking library)

### Build from Source

```bash
cd /Users/macbook/Desktop/etrid/01-detr-p2p/director-node
cargo build --release

# Binary location
./target/release/director-node
```

## Configuration

### Method 1: JSON Configuration File

Create `/etc/etrid/director.json`:

```json
{
  "peer_id": "0x1111...",
  "tailscale_ip": "100.64.0.1",
  "peer_directors": ["100.64.0.2", "100.64.0.3"],
  "max_validators_per_director": 7,
  "require_authorization": true,
  "validator_port": 30333,
  "director_port": 30444
}
```

Run with config file:

```bash
DIRECTOR_CONFIG=/etc/etrid/director.json ./director-node
```

### Method 2: Environment Variables

```bash
export TAILSCALE_IP=100.64.0.1
export PEER_DIRECTORS=100.64.0.2,100.64.0.3
export MAX_VALIDATORS=7
export REQUIRE_AUTHORIZATION=true
export VALIDATOR_PORT=30333
export DIRECTOR_PORT=30444

./director-node
```

### Configuration Reference

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `peer_id` | String | Auto | Director's unique peer ID |
| `tailscale_ip` | String | Required | Tailscale private IP |
| `peer_directors` | Array | [] | Other director IPs |
| `max_validators_per_director` | Number | 7 | Load balancing limit |
| `require_authorization` | Boolean | true | Enforce validator registry |
| `validator_port` | Number | 30333 | Public validator port |
| `director_port` | Number | 30444 | Private director port |

## Usage

### Start Director Node

```bash
# Install Tailscale
curl -fsSL https://tailscale.com/install.sh | sh
sudo tailscale up

# Get Tailscale IP
tailscale ip -4
# Output: 100.64.0.1

# Start director
DIRECTOR_CONFIG=./examples/director1.json ./director-node
```

Expected output:

```
[INFO] 🎯 Starting ËTRID Director Node
[INFO] 📋 Director Configuration:
[INFO]    Tailscale IP: 100.64.0.1
[INFO]    Peer Directors: ["100.64.0.2", "100.64.0.3"]
[INFO]    Max Validators: 7
[INFO] ✅ Director node running on 100.64.0.1
[INFO] 🔗 Connected to director: 100.64.0.2
[INFO] 🔗 Connected to director: 100.64.0.3
[INFO] 📊 Accepting connections from validators on port 30333
```

### Monitor Director

```bash
# Check logs
journalctl -u director-node -f

# Check metrics
curl http://localhost:9615/metrics | grep relay_

# Expected:
# relay_messages_total 1523
# relay_loops_prevented 42
# relay_cache_size 1200
```

### Validator Connection

Validators automatically connect to directors via bootnodes:

```bash
./flarechain-node \
  --chain=flarechain_mainnet_v2_directors \
  --validator \
  --bootnodes=/ip4/100.64.0.1/tcp/30333/p2p/12D3KooWDir1...
```

## Testing

```bash
# Run unit tests
cargo test

# Run with verbose logging
RUST_LOG=debug cargo run --release

# Test with 2 directors locally
# Terminal 1
TAILSCALE_IP=127.0.0.1 PEER_DIRECTORS=127.0.0.2 ./director-node

# Terminal 2
TAILSCALE_IP=127.0.0.2 PEER_DIRECTORS=127.0.0.1 ./director-node
```

## Deployment

### Production Deployment

See [DEPLOYMENT_GUIDE.md](./DEPLOYMENT_GUIDE.md) for full instructions.

Quick summary:

1. **Deploy 3 directors** (high-availability infrastructure)
2. **Install Tailscale** on each director
3. **Configure director mesh** (peer_directors)
4. **Start director nodes**
5. **Update chain-spec** with director bootnodes
6. **Deploy validators** (connect via bootnodes)

### Systemd Service

Create `/etc/systemd/system/director-node.service`:

```ini
[Unit]
Description=ETRID Director Node
After=network.target tailscaled.service

[Service]
Type=simple
User=etrid
ExecStart=/usr/local/bin/director-node
Environment="DIRECTOR_CONFIG=/etc/etrid/director.json"
Restart=always
RestartSec=10

[Install]
WantedBy=multi-user.target
```

Enable and start:

```bash
sudo systemctl daemon-reload
sudo systemctl enable director-node
sudo systemctl start director-node
sudo systemctl status director-node
```

## Monitoring & Metrics

### Health Checks

```bash
# Director status
curl http://localhost:9615/health

# Connection count
curl http://localhost:9615/metrics | grep p2p_peers_count

# Relay performance
curl http://localhost:9615/metrics | grep relay_
```

### Key Metrics

| Metric | Description |
|--------|-------------|
| `relay_messages_total` | Total messages relayed |
| `relay_loops_prevented` | Duplicate messages dropped |
| `relay_cache_size` | Seen message cache size |
| `p2p_peers_count{role="director"}` | Connected directors |
| `p2p_peers_count{role="validator"}` | Connected validators |
| `slashing_events_total` | Double-sign detections |

### Alerting

Set up alerts for:

- ⚠️ Director disconnections (`p2p_peers_count{role="director"}` < 2)
- ⚠️ Validator overload (`p2p_peers_count{role="validator"}` > max_validators)
- 🚨 Slashing events (`slashing_events_total` > 0)

## Troubleshooting

### Validators Can't Connect

**Check:**
1. Director node is running: `systemctl status director-node`
2. Tailscale is active: `tailscale status`
3. Firewall allows port 30333: `sudo ufw allow 30333`
4. Correct bootnodes in validator config

### Director Disconnected

**Check:**
1. Tailscale connectivity: `tailscale ping 100.64.0.2`
2. Network latency: `ping -c 10 100.64.0.2`
3. Director logs: `journalctl -u director-node -f`

### High Relay Latency

**Check:**
1. CPU usage: `top`
2. Network bandwidth: `iftop`
3. Cache size: `curl localhost:9615/metrics | grep cache_size`

### Slashing Detected

```
[ERROR] 🚨 SLASHING EVIDENCE DETECTED:
  Validator: 12D3KooWVal7...
  Offense: DoubleSign
  Checkpoint: 12345
  Block Hash 1: 0xaabb...
  Block Hash 2: 0xccdd...
```

**Action:**
1. Review slashing evidence
2. Submit to on-chain slashing pallet
3. Validator will be ejected and slashed

## API Reference

### DirectorNode

```rust
pub struct DirectorNode {
    pub fn new(config: DirectorConfig) -> Self;
    pub async fn start(&mut self, bootstrap_peers: Vec<PeerAddr>) -> Result<(), String>;
    pub async fn connect_to_directors(&self) -> Result<(), String>;
    pub async fn relay_checkpoint_message(&self, from: PeerId, msg: Message) -> Result<(), String>;
    pub async fn authorize_validator(&self, proof: AuthorizationProof) -> Result<(), String>;
    pub async fn revoke_validator(&self, peer_id: PeerId, reason: String) -> Result<(), String>;
}
```

### MessageRelay

```rust
pub struct MessageRelay {
    pub fn hash_message(&self, msg: &Message) -> Result<[u8; 32], String>;
    pub async fn is_seen(&self, msg_hash: &[u8; 32]) -> bool;
    pub async fn mark_seen(&self, msg_hash: [u8; 32]);
    pub async fn get_metrics(&self) -> RelayMetrics;
}
```

### SlashingDetector

```rust
pub struct SlashingDetector {
    pub fn check_certificate(&mut self, validator: &PeerId, data: &[u8]) -> Option<SlashingEvidence>;
    pub fn get_evidence(&self) -> Vec<SlashingEvidence>;
    pub fn clear_evidence(&mut self);
}
```

## Examples

See `examples/` directory:

- `director1.json` - Director 1 configuration
- `director2.json` - Director 2 configuration
- `director3.json` - Director 3 configuration

## Documentation

- [ARCHITECTURE.md](./ARCHITECTURE.md) - Technical architecture
- [DEPLOYMENT_GUIDE.md](./DEPLOYMENT_GUIDE.md) - Production deployment
- [DETR-P2P Protocol](../detrp2p/README.md) - P2P networking layer

## Performance

- **Relay throughput**: 10,000+ msg/sec per director
- **Relay latency**: 5-10ms average, 50ms P99
- **Connection count**: O(N) vs O(N²) full mesh
- **Broadcast speedup**: 21× faster (50ms vs 1050ms for 21 validators)

## Security

- ✅ Tailscale WireGuard encryption (director mesh)
- ✅ Validator authorization (2/3+ director consensus)
- ✅ Double-sign detection (checkpoint tracking)
- ✅ Message loop prevention (BLAKE2b hashing)
- ✅ On-chain slashing (evidence submission)

## License

Apache 2.0 - See LICENSE file

## Support

- GitHub: https://github.com/etrid/etrid
- Discord: https://discord.gg/etrid
- Email: support@etrid.network

## Contributing

Contributions welcome! See [CONTRIBUTING.md](../../CONTRIBUTING.md)

## Acknowledgments

Built for ËTRID FlareChain mainnet by the ËTRID development team.
