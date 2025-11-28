# ËTRUST Quick Start Guide

## Build

```bash
cd /Users/macbook/Desktop/etrid
cargo build --release -p etrust
```

Binary location: `target/release/etrust`

## Quick Examples

### Generate Keys

```bash
./target/release/etrust keys generate
```

### Create Account

```bash
./target/release/etrust account create --name "Alice"
```

### Query Chain (requires running node)

```bash
./target/release/etrust query chain-info
```

### Check Balance

```bash
./target/release/etrust query balance 5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY
```

## Available Commands

- `account` - Account management
- `stake` - Staking operations
- `query` - Query blockchain
- `send` - Send transactions
- `consensus` - Governance operations
- `keys` - Key management

Run `etrust --help` or `etrust <command> --help` for more details.

## Notes

This is an MVP implementation. Full transaction signing and pallet integration requires:
- Keystore implementation
- SCALE codec for extrinsic encoding
- Metadata parsing
- Pallet-specific storage keys

See README.md for complete documentation.
