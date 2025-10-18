# etrcpp Quick Reference Card

## Build & Install

```bash
# Build
./build.sh

# Or use Make
make

# Install system-wide
sudo install -m 0755 etrcpp /usr/local/bin/
```

## Basic Usage

```bash
etrcpp [options] <command> [parameters]
```

## Connection Options

```bash
-rpcconnect=<ip>      # Node IP (default: 127.0.0.1)
-rpcport=<port>       # RPC port (default: 9944)
-rpcuser=<user>       # RPC username
-rpcpassword=<pw>     # RPC password
-timeout=<n>          # Timeout in seconds (default: 30)
```

## Commands Cheat Sheet

### Account
```bash
etrcpp account create [name]         # Create account
etrcpp account list                  # List accounts
etrcpp account info <address>        # Account info
etrcpp account import <key> [name]   # Import account
```

### Stake
```bash
etrcpp stake <address> <amount>      # Stake tokens
etrcpp unstake <address> [amount]    # Unstake (0=all)
etrcpp stakeinfo <address>           # Stake info
etrcpp validators                    # List validators
```

### Query
```bash
etrcpp balance <address>             # Get balance
etrcpp block <height|hash>           # Get block
etrcpp transaction <hash>            # Get transaction
etrcpp blockchaininfo                # Blockchain info
etrcpp networkinfo                   # Network info
```

### Transaction
```bash
etrcpp send <from> <to> <amount> [fee]  # Send tx
etrcpp sendraw <hex>                     # Send raw tx
```

### Consensus
```bash
etrcpp consensusday                  # Current day
etrcpp consensusdayinfo <day>        # Day info
etrcpp vote <validator> <proposal> <yes|no>  # Vote
```

## Common Examples

```bash
# Check version
etrcpp -version

# Help
etrcpp -h

# Create and fund account
etrcpp account create alice
etrcpp send 0xfrom... 0xto... 1000000

# Stake operations
etrcpp stake 0x1234... 10000000
etrcpp validators
etrcpp stakeinfo 0x1234...

# Query operations
etrcpp balance 0x1234...
etrcpp block 12345
etrcpp blockchaininfo

# Remote node
etrcpp -rpcconnect=node.etrid.io balance 0x...
```

## Address Formats

- **Ethereum:** `0x` + 40 hex chars (42 total)
- **ËTRID:** `etr` + identifier (10+ chars)

## Error Codes

- `0` = Success
- `1` = Command error
- `-1` = RPC error

## Tips

- Use `-timeout=60` for slow connections
- Omit `[amount]` in unstake to unstake all
- Use block hash or height for queries
- Vote with "yes", "no", "true", "false", "1", or "0"
