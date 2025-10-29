# Ëtrid FlareChain Validator Scripts

## Quick Start (For Anyone)

The `one-command-validator.sh` script is designed to work for **any node operator** who clones this repository.

### Prerequisites

1. Clone the repository:
   ```bash
   git clone https://github.com/EojEdred/Etrid.git
   cd Etrid
   ```

2. Build the node:
   ```bash
   cargo build --release
   ```

That's it! The script will auto-detect everything else.

---

## Usage Examples

### Example 1: Start First Validator (Bootstrap Node)

```bash
# From repo root
./scripts/one-command-validator.sh
```

**What happens:**
- Auto-finds binary at `target/release/flarechain-node`
- Auto-finds chain spec at `infrastructure/chain-specs/flarechain-local-raw.json`
- Creates data directory at `~/.etrid` (or `/var/lib/etrid` if running as root)
- Generates random validator keys
- Inserts AURA, GRANDPA, and ASF keys
- Starts node in bootstrap mode
- Displays your bootnode address to share with others

**Output includes:**
```
📋 Share this bootnode address with other validators:
   /ip4/192.168.1.100/tcp/30333/p2p/12D3KooWABC...XYZ
```

Copy this address - other validators will need it!

---

### Example 2: Join Existing Network

```bash
# Replace with the bootnode address from first validator
./scripts/one-command-validator.sh --bootnode /ip4/192.168.1.100/tcp/30333/p2p/12D3KooWABC...XYZ
```

**What happens:**
- Same auto-detection as Example 1
- Connects to the specified bootnode
- Syncs with the network
- Starts producing blocks

---

### Example 3: Use Specific Validator Keys (Alice/Bob for testing)

```bash
# For Alice
VALIDATOR_KEY="//Alice" ./scripts/one-command-validator.sh

# For Bob (connecting to Alice)
VALIDATOR_KEY="//Bob" ./scripts/one-command-validator.sh --bootnode /ip4/192.168.1.100/tcp/30333/p2p/12D3KooW...
```

---

### Example 4: Custom Configuration

```bash
# Override defaults with environment variables
FLARECHAIN_BINARY=/custom/path/flarechain-node \
CHAIN_SPEC=/custom/chainspec.json \
BASE_PATH=/custom/data \
NODE_NAME="my-validator" \
  ./scripts/one-command-validator.sh
```

Or use command-line options:
```bash
./scripts/one-command-validator.sh \
  --base-path /custom/data \
  --name "my-validator" \
  --rpc-port 9944 \
  --ws-port 9945 \
  --port 30333
```

---

## How It Works

### Auto-Detection

The script automatically finds:

1. **Binary Location**: Checks in order:
   - `target/release/flarechain-node` (after `cargo build --release`)
   - `target/debug/flarechain-node` (after `cargo build`)
   - Current directory
   - `/opt/etrid/` (for production deployments)
   - System PATH

2. **Chain Spec**: Checks in order:
   - `infrastructure/chain-specs/flarechain-local-raw.json` (repo default)
   - `chainspec.json` (repo root)
   - `/opt/etrid/chainspec.json` (production)

3. **Data Directory**:
   - `~/.etrid` for normal users
   - `/var/lib/etrid` for root (production)

4. **IP Address**:
   - Primary network interface IP (for P2P advertising)
   - Falls back to localhost for local testing

---

## What the Script Does

```
[1/4] Network Key Setup
  - Generates P2P network identity key
  - Extracts peer ID
  - Saves backup to keys/ directory

[2/4] Validator Key Setup
  - Uses provided key or generates random one
  - Derives AURA and GRANDPA public keys
  - Saves seed securely

[3/4] Inserting Keys to Keystore
  - AURA key (sr25519) for block production
  - GRANDPA key (ed25519) for finality
  - ASF key (sr25519) for async finality

[4/4] Starting Validator
  - Starts node with proper configuration
  - Bootstrap mode (if no bootnode specified)
  - Joining mode (if bootnode specified)
```

---

## Important Files

After running the script, you'll find:

```
~/.etrid/                              # Data directory
├── keys/
│   ├── validator_seed                 # SECRET - backup this!
│   ├── network_secret                 # P2P identity backup
│   └── node_info.txt                  # All node information
├── chains/flarechain_local/
│   ├── network/
│   │   └── secret_ed25519             # P2P network key (active)
│   ├── keystore/
│   │   ├── 61757261<hex>              # AURA key
│   │   ├── 6772616e<hex>              # GRANDPA key
│   │   └── 6173666b<hex>              # ASF key
│   └── db/                            # Blockchain database
```

**⚠️ CRITICAL**: Backup `validator_seed` securely! This is your validator identity.

---

## Command-Line Options

```
./scripts/one-command-validator.sh [OPTIONS]

Options:
  --bootnode <addr>       Connect to existing node (multiaddr format)
  --validator-key <key>   Use specific validator key (e.g. //Alice)
  --base-path <path>      Data directory (default: auto-detected)
  --name <name>           Node name (default: etrid-<user>-<hostname>)
  --rpc-port <port>       RPC port (default: 9944)
  --ws-port <port>        WebSocket port (default: 9945)
  --port <port>           P2P port (default: 30333)
  --help, -h              Show help

Environment Variables:
  FLARECHAIN_BINARY       Path to binary
  CHAIN_SPEC              Path to chain spec
  BASE_PATH               Data directory
  NODE_NAME               Node name
  VALIDATOR_KEY           Validator key
  BOOTNODE                Bootnode address
```

---

## Typical Workflow

### Two Friends Starting a Network

**Alice (First Validator):**
```bash
# On Alice's computer
cd ~/Etrid
cargo build --release
./scripts/one-command-validator.sh

# Alice sees output:
# Peer ID: 12D3KooWABC123...
# Share this: /ip4/203.0.113.10/tcp/30333/p2p/12D3KooWABC123...
```

Alice sends Bob the bootnode address.

**Bob (Second Validator):**
```bash
# On Bob's computer
cd ~/Etrid
cargo build --release
./scripts/one-command-validator.sh --bootnode /ip4/203.0.113.10/tcp/30333/p2p/12D3KooWABC123...

# Bob's node connects to Alice
# Both nodes show: "Idle (1 peers)"
```

---

## Production Deployment

For production servers:

```bash
# Run as root for proper permissions
sudo su

# Clone and build
cd /opt
git clone https://github.com/EojEdred/Etrid.git etrid
cd etrid
cargo build --release

# Run validator (will use /var/lib/etrid automatically)
./scripts/one-command-validator.sh --name "my-production-validator"
```

---

## Troubleshooting

### "Could not find flarechain-node binary"
**Solution**: Build the binary first:
```bash
cargo build --release
```

### "Could not find chain spec file"
**Solution**: Make sure you're in the repository root, or specify the path:
```bash
CHAIN_SPEC=/path/to/chainspec.json ./scripts/one-command-validator.sh
```

### "0 peers" after connecting
**Checklist**:
1. Verify bootnode address is correct
2. Check firewall allows port 30333
3. Ensure both nodes using same chain spec
4. Check logs for "Different genesis" errors

### Restart with fresh state
```bash
# Stop node (Ctrl+C)
# Remove data directory
rm -rf ~/.etrid    # or /var/lib/etrid if root

# Run script again
./scripts/one-command-validator.sh
```

---

## Comparison: Old vs New

| Task | Old Method | New Method |
|------|-----------|------------|
| Find binary | Manual path | Auto-detected |
| Generate network key | Manual command | Automatic |
| Extract peer ID | Manual command | Automatic |
| Generate validator keys | Manual command | Automatic |
| Insert AURA key | Manual command | Automatic |
| Insert GRANDPA key | Manual command | Automatic |
| Insert ASF key | Manual command | Automatic |
| Start node | Manual command | Automatic |
| **Total commands** | **10+** | **1** |

---

## Advanced: Using with Systemd

Create `/etc/systemd/system/etrid-validator.service`:

```ini
[Unit]
Description=Ëtrid FlareChain Validator
After=network.target

[Service]
Type=simple
User=etrid
WorkingDirectory=/opt/etrid
Environment="VALIDATOR_KEY=<your-secret-key>"
Environment="BOOTNODE=<bootnode-if-needed>"
ExecStart=/opt/etrid/scripts/one-command-validator.sh
Restart=on-failure
RestartSec=10

[Install]
WantedBy=multi-user.target
```

Enable and start:
```bash
sudo systemctl daemon-reload
sudo systemctl enable etrid-validator
sudo systemctl start etrid-validator
sudo systemctl status etrid-validator
```

---

## Security Notes

1. **Backup validator_seed**: This is your validator identity. Store offline securely.
2. **File permissions**: Keys are automatically set to 600 (owner read/write only)
3. **Firewall**: Only expose necessary ports (30333 for P2P, optionally 9944/9945 for RPC/WS)
4. **Never share**: Your `validator_seed` or private keys

---

## Support

For issues or questions:
- GitHub: https://github.com/EojEdred/Etrid
- Documentation: See repository README

---

## What About VM-Specific Scripts?

The `/tmp/VM1_DEPLOY_SIMPLE.sh` and `/tmp/VM2_DEPLOY_SIMPLE.sh` scripts are **examples** of how to use the generic script for specific deployments. They:

1. Set environment variables for VM-specific paths
2. Call the generic `one-command-validator.sh`
3. Work for the specific Azure VM setup

**Anyone can create similar wrapper scripts** for their own environment!

```bash
#!/bin/bash
# my-deployment.sh - Custom deployment wrapper

export VALIDATOR_KEY="//MyKey"
export NODE_NAME="my-company-validator"
export BOOTNODE="/ip4/10.0.0.5/tcp/30333/p2p/12D3Koo..."

exec /path/to/one-command-validator.sh
```

The core `one-command-validator.sh` remains generic and works for everyone! 🚀
