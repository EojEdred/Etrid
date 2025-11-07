# FlareChain Node Binaries

Pre-compiled binaries for the Ëtrid FlareChain mainnet node.

## Download the Correct Binary

### For Linux x86_64 (Ubuntu, Debian, CentOS, etc.)
**Use this for your validator VMs:**
```bash
cd linux-x86_64
./flarechain-node --version
```

**Requirements:**
- Linux x86_64 (64-bit Intel/AMD processors)
- Ubuntu 20.04+ / Debian 11+ / CentOS 8+ or compatible
- GLIBC 2.31 or newer

### For macOS ARM64 (Apple Silicon)
**Use this for local testing on M1/M2/M3 Macs:**
```bash
cd macos-arm64
./flarechain-node --version
```

**Requirements:**
- macOS 11.0 (Big Sur) or newer
- Apple Silicon (M1, M2, M3, M4)

## Quick Start

### On Linux VM (Validators):
```bash
# Download Linux binary
wget https://github.com/EojEdred/Etrid/releases/latest/download/flarechain-node-linux-x86_64
chmod +x flarechain-node-linux-x86_64
mv flarechain-node-linux-x86_64 flarechain-node

# Start validator
./flarechain-node \
  --validator \
  --chain flarechain \
  --name "Your-Validator-Name" \
  --rpc-port 9944 \
  --port 30333
```

### On macOS (Local Development):
```bash
# Download macOS binary
wget https://github.com/EojEdred/Etrid/releases/latest/download/flarechain-node-macos-arm64
chmod +x flarechain-node-macos-arm64
mv flarechain-node-macos-arm64 flarechain-node

# Start local node
./flarechain-node --dev
```

## Binary Details

### linux-x86_64/flarechain-node
- **Platform**: Linux x86_64
- **Target**: x86_64-unknown-linux-gnu
- **Size**: ~60MB
- **Compiled on**: November 1, 2025
- **Genesis**: 51 easter eggs embedded
- **IPFS Hashes**: 11 whitepapers included

### macos-arm64/flarechain-node
- **Platform**: macOS ARM64
- **Target**: aarch64-apple-darwin
- **Size**: ~58MB
- **Compiled on**: November 1, 2025
- **Genesis**: 51 easter eggs embedded
- **IPFS Hashes**: 11 whitepapers included

## Verification

### Check Binary Architecture:
```bash
# On Linux:
file flarechain-node
# Expected: ELF 64-bit LSB executable, x86-64

# On macOS:
file flarechain-node
# Expected: Mach-O 64-bit executable arm64
```

### Verify Version:
```bash
./flarechain-node --version
# Should show: flarechain-node 1.0.0-mainnet
```

### Test Connection:
```bash
# Start node
./flarechain-node --dev

# In another terminal, check RPC
curl -H "Content-Type: application/json" \
  -d '{"id":1, "jsonrpc":"2.0", "method":"system_health"}' \
  http://localhost:9944
```

## Deployment to 21 Validator VMs

```bash
# Transfer Linux binary to all VMs
for i in {01..21}; do
  scp linux-x86_64/flarechain-node validator-$i:~/
  ssh validator-$i "chmod +x ~/flarechain-node"
done

# Verify installation
for i in {01..21}; do
  echo "=== Validator $i ==="
  ssh validator-$i "./flarechain-node --version"
done
```

## Session Keys

After starting the node, insert session keys:

```bash
# Generate session keys
curl -H "Content-Type: application/json" \
  -d '{"id":1, "jsonrpc":"2.0", "method":"author_rotateKeys"}' \
  http://localhost:9944

# Insert AURA key
curl -H "Content-Type: application/json" \
  -d '{"id":1, "jsonrpc":"2.0", "method":"author_insertKey", "params":["aura","YOUR_SEED","YOUR_AURA_KEY"]}' \
  http://localhost:9944

# Insert GRANDPA key
curl -H "Content-Type: application/json" \
  -d '{"id":1, "jsonrpc":"2.0", "method":"author_insertKey", "params":["gran","YOUR_SEED","YOUR_GRANDPA_KEY"]}' \
  http://localhost:9944

# Insert ASF key
curl -H "Content-Type: application/json" \
  -d '{"id":1, "jsonrpc":"2.0", "method":"author_insertKey", "params":["asf!","YOUR_SEED","YOUR_ASF_KEY"]}' \
  http://localhost:9944
```

All session keys are available in `secrets/.env.mainnet` (not in repository).

## Network Configuration

### Mainnet:
- **Chain ID**: flarechain_mainnet
- **Total Supply**: 2.521 Billion ETR
- **EDSC Supply**: 1 Billion EDSC
- **Validators**: 21 (Gizzi + EojEdred + 19 others)
- **Consensus**: Adaptive Security Framework (ASF)

### RPC Endpoints:
- Default: `http://localhost:9944`
- WebSocket: `ws://localhost:9944`

### P2P Port:
- Default: `30333`

## Troubleshooting

### Linux: "cannot execute binary file"
- You may have downloaded the macOS binary by mistake
- Use the `linux-x86_64` folder

### macOS: "cannot be opened because the developer cannot be verified"
```bash
xattr -d com.apple.quarantine flarechain-node
```

### "GLIBC version too old"
- Update your Linux system to Ubuntu 20.04+ or equivalent
- Or compile from source on your system

### Docker not found (when using cross)
- Install Docker: `sudo apt install docker.io`
- Or download pre-built binaries from releases

## Building from Source

If you need to compile for other platforms:

```bash
# Clone repository
git clone https://github.com/EojEdred/Etrid.git
cd Etrid/05-multichain/flare-chain/node

# Build for your platform
cargo build --release --bin flarechain-node

# Cross-compile for Linux from macOS
cross build --release --bin flarechain-node --target x86_64-unknown-linux-gnu
```

## Support

- **GitHub**: https://github.com/EojEdred/Etrid
- **Documentation**: https://etrid.com/docs
- **Whitepaper**: https://etrid.com/whitepaper

---

**Built with**: Substrate Framework + Polkadot SDK
**License**: GPLv3
**Maintainer**: Eoj Edred
**Network Launch**: November 1, 2025

🔥 Keep the flame burning! 🔥
