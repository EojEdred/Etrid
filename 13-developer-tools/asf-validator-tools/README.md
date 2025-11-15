# ËTRID ASF Validator Tools

Comprehensive tooling suite for ËTRID ASF (Ascending Scale of Finality) validators.

## Overview

This repository contains four essential tools for managing and monitoring ËTRID validators:

1. **asf-keygen** - Cryptographic key generation and management
2. **asf-monitor** - Real-time validator monitoring dashboard
3. **asf-health** - Validator health checking utility
4. **asf-stake** - Staking management CLI

## Quick Start

### Prerequisites

- Rust 1.75+ and Cargo
- An ËTRID node running with RPC enabled
- At least 4GB RAM for building

### Installation

```bash
# Clone the repository
git clone https://github.com/etrid/asf-validator-tools.git
cd asf-validator-tools

# Build all tools
make build

# Install to ~/.local/bin (recommended)
make install-local

# Or install to /usr/local/bin (requires sudo)
sudo make install
```

### Quick Test

```bash
# Generate a new validator key
asf-keygen generate --scheme sr25519 --output validator.key

# Check validator health
asf-health --rpc http://localhost:9944 --validator <address>

# Monitor validator in real-time
asf-monitor --rpc http://localhost:9944 --validator <address>
```

## Tools Overview

### 1. asf-keygen - Key Management CLI

Generate, inspect, sign, and manage cryptographic keys for validators.

**Features:**
- Generate sr25519 and ed25519 keys
- Sign and verify messages
- Import/export keys in multiple formats
- Generate complete session keys for validators

**Quick Examples:**

```bash
# Generate a new sr25519 key
asf-keygen generate --scheme sr25519 --output validator.key

# Generate ed25519 key with custom password
asf-keygen generate --scheme ed25519 --output grandpa.key --password "secure123"

# Inspect a key file
asf-keygen inspect --keyfile validator.key

# Sign a message
asf-keygen sign --keyfile validator.key --message "0x48656c6c6f"

# Verify a signature
asf-keygen verify --pubkey <hex> --message <hex> --signature <hex> --scheme sr25519

# Export key as JSON
asf-keygen export --keyfile validator.key --format json --output exported.json

# Import from mnemonic
asf-keygen import --data "word1 word2 ... word24" --format mnemonic --output imported.key

# Generate complete session keys (AURA + GRANDPA)
asf-keygen generate-session --output-dir ./session-keys --name validator-1
```

### 2. asf-monitor - Real-time TUI Dashboard

Terminal-based real-time monitoring dashboard for validators.

**Features:**
- Live network status (chain, block height, peers)
- Validator status (active, committee membership, votes)
- Performance metrics (uptime, blocks signed, health score)
- Certificate issuance tracking
- Slashing status indicators
- Activity log with scrolling
- Real-time block production sparkline

**Usage:**

```bash
# Monitor a specific validator
asf-monitor --rpc http://localhost:9944 --validator <address>

# Monitor with custom update interval (5 seconds)
asf-monitor --rpc http://localhost:9944 --validator <address> --interval 5

# Monitor all validators (network overview)
asf-monitor --rpc http://localhost:9944
```

**Keyboard Controls:**
- `q` - Quit
- `r` - Force refresh
- `c` - Clear activity log
- `↑/↓` - Scroll activity log

### 3. asf-health - Health Check Utility

Comprehensive health checking for validator nodes.

**Features:**
- Node connectivity testing
- Key accessibility verification
- Stake validation
- Slashing status check
- P2P peer count monitoring
- Sync status verification
- System resource checks (CPU, memory, disk)
- JSON and text output formats
- Exit codes for automation

**Usage:**

```bash
# Run full health check
asf-health --rpc http://localhost:9944 --validator <address>

# Verbose output
asf-health --rpc http://localhost:9944 --validator <address> --verbose

# Export to JSON
asf-health --rpc http://localhost:9944 --validator <address> --format json --output health.json

# Exit with error code on failures (useful for scripts)
asf-health --rpc http://localhost:9944 --validator <address> --exit-on-failure
```

**Health Checks Performed:**
1. ✓ Node Connectivity - Tests RPC connection
2. ✓ Key Accessibility - Validates key format
3. ✓ Stake Verification - Checks minimum stake requirement
4. ✓ Slashing Status - Ensures validator is not slashed
5. ✓ P2P Peer Count - Verifies sufficient peer connections
6. ✓ Sync Status - Confirms node is fully synced
7. ✓ System Resources - Monitors CPU, memory, and disk usage

### 4. asf-stake - Staking Management CLI

Manage validator staking operations.

**Features:**
- Bond tokens to validators
- Unbond tokens (with unbonding period)
- Check staking status
- View reward history
- Claim pending rewards
- List all validators with sorting

**Usage:**

```bash
# Bond 1000 ETR to a validator
asf-stake bond --amount 1000 --validator <address> --keyfile staker.key

# Unbond 500 ETR
asf-stake unbond --amount 500 --validator <address> --keyfile staker.key

# Check staking status
asf-stake status --validator <address>

# Detailed staking status
asf-stake status --validator <address> --detailed

# View reward history (last 20 epochs)
asf-stake rewards --validator <address> --epochs 20

# Claim pending rewards
asf-stake claim --validator <address> --keyfile validator.key

# List all validators sorted by stake
asf-stake list --sort stake --limit 20

# List validators sorted by reputation
asf-stake list --sort reputation --limit 10
```

## Building from Source

### Build All Tools

```bash
make build
```

### Build Individual Tools

```bash
make build-keygen
make build-monitor
make build-health
make build-stake
```

### Development Mode

```bash
# Run in development mode with cargo
make dev-keygen
make dev-monitor
make dev-health
make dev-stake
```

### Testing

```bash
# Run all tests
make test

# Test individual tools
make test-keygen
make test-monitor
make test-health
make test-stake

# Run clippy linter
make clippy

# Format code
make fmt
```

## Configuration

### RPC Endpoint

All tools support the `--rpc` flag to specify the node endpoint:

```bash
# Local node
asf-health --rpc http://localhost:9944

# Remote node
asf-health --rpc https://rpc.etrid.network:9944

# Custom port
asf-health --rpc http://validator.local:9933
```

### Environment Variables

You can set default values using environment variables:

```bash
export ASF_RPC_ENDPOINT="http://localhost:9944"
export ASF_VALIDATOR_ADDRESS="5GrwvaEF..."
```

## Advanced Usage

### Automated Health Monitoring

Create a cron job to check validator health:

```bash
# Add to crontab (every 5 minutes)
*/5 * * * * /usr/local/bin/asf-health --rpc http://localhost:9944 --validator <address> --format json --output /var/log/validator-health.json --exit-on-failure || /usr/local/bin/alert-script.sh
```

### Key Rotation Workflow

```bash
# 1. Generate new session keys
asf-keygen generate-session --output-dir ./new-keys --name validator-1

# 2. Inspect the new keys
asf-keygen inspect --keyfile ./new-keys/validator-1-aura.key
asf-keygen inspect --keyfile ./new-keys/validator-1-grandpa.key

# 3. Update on-chain session keys (using asf-stake or custom script)
# 4. Restart validator with new keys
# 5. Verify with asf-monitor
```

### Monitoring Multiple Validators

Create a simple script to monitor multiple validators:

```bash
#!/bin/bash
# monitor-all.sh

VALIDATORS=(
    "5GrwvaEF..."
    "5FHneW46..."
    "5CiPPseX..."
)

for validator in "${VALIDATORS[@]}"; do
    echo "Checking $validator..."
    asf-health --rpc http://localhost:9944 --validator "$validator" --exit-on-failure
    if [ $? -ne 0 ]; then
        echo "Health check failed for $validator" | mail -s "Validator Alert" admin@example.com
    fi
done
```

## Troubleshooting

### Common Issues

**1. Connection refused to RPC**
```bash
# Check if node is running
systemctl status etrid-node

# Verify RPC is enabled in node config
grep -i "rpc" /etc/etrid/config.toml

# Test connection
curl -H "Content-Type: application/json" -d '{"id":1, "jsonrpc":"2.0", "method": "system_health"}' http://localhost:9944
```

**2. Key file decryption failed**
```bash
# Wrong password - try again
asf-keygen inspect --keyfile validator.key

# Corrupted file - restore from backup
cp validator.key.backup validator.key
```

**3. Monitor display issues**
```bash
# Set correct terminal
export TERM=xterm-256color

# Resize terminal to at least 100x30
resize
```

## Security Best Practices

1. **Key Storage**
   - Store key files in encrypted directories
   - Use strong passwords for key encryption
   - Never commit keys to version control
   - Keep offline backups of mnemonics

2. **RPC Access**
   - Use TLS for remote RPC connections
   - Restrict RPC access with firewall rules
   - Use authentication when available
   - Monitor RPC access logs

3. **Monitoring**
   - Run health checks regularly
   - Set up alerts for critical issues
   - Monitor validator uptime and performance
   - Track slashing events

4. **Staking Operations**
   - Verify amounts before bonding
   - Understand unbonding periods
   - Claim rewards regularly
   - Monitor delegation changes

## Contributing

We welcome contributions! Please see [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.

## License

Apache-2.0

## Support

- Documentation: https://docs.etrid.network
- Discord: https://discord.gg/etrid
- Forum: https://forum.etrid.network
- Email: support@etrid.network

## Acknowledgments

Built with:
- [Substrate](https://substrate.io/) - Blockchain framework
- [Ratatui](https://github.com/ratatui-org/ratatui) - Terminal UI
- [Clap](https://github.com/clap-rs/clap) - Command-line parsing
- [Tokio](https://tokio.rs/) - Async runtime
