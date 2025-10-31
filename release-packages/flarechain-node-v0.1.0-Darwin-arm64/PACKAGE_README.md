# FlareChain Node Binary Package

Version: 0.1.0
Platform: Darwin arm64
Build Date: Thu Oct 30 10:02:04 CDT 2025

## Binary Information

- **File**: flarechain-node
- **Size**:  60M
- **SHA256**: 5e843a05d5be571533267be519e1ffe7cffbe811025601b6c33f3230e264e4b6

## Quick Start

### Installation

Run the installation script:

```bash
./install.sh
```

Or manually copy the binary:

```bash
sudo cp flarechain-node /usr/local/bin/
sudo chmod +x /usr/local/bin/flarechain-node
```

### Running the Node

Start a validator node:

```bash
flarechain-node \
    --chain=chainspec.json \
    --validator \
    --alice \
    --base-path=/tmp/validator \
    --port=30333 \
    --rpc-port=9944
```

For more options:

```bash
flarechain-node --help
```

## System Requirements

- **OS**: Linux, macOS, or Windows (WSL)
- **RAM**: 4GB minimum, 8GB recommended
- **Storage**: 50GB minimum
- **Network**: Stable internet connection

## FlareSwap DEX Integration

This package includes FlareSwap DEX deployment integration for ËtwasmVM:

```bash
cd flareswap
npm install
node deploy-etwasm.js --network=local
```

See `flareswap/ETWASM_DEPLOYMENT_GUIDE.md` for complete instructions.

## Features Included

- ✅ FlareChain validator node binary
- ✅ ASF consensus with PPFA proposer
- ✅ ËtwasmVM (EVM-compatible smart contracts)
- ✅ FlareSwap DEX deployment tools
- ✅ Cross-chain bridge support (15 chains)
- ✅ PBC router for multichain swaps

## Documentation

For complete documentation, visit: https://docs.etrid.org

## Support

- GitHub: https://github.com/etrid/etrid
- Issues: https://github.com/etrid/etrid/issues
- Discord: https://discord.gg/etrid

## License

See LICENSE file for details.
