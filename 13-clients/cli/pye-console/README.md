# pyE - ËTRID's Python Command-Line Interface

**pyE** is the official Python CLI for interacting with the ËTRID blockchain network. Built with modern Python best practices and inspired by Ethereum's Ape framework, pyE provides a clean, intuitive interface for developers and users.

## Features

- **Account Management**: Create, import, export, and manage ËTRID accounts
- **Token Operations**: Send ETR tokens, check balances, query transactions
- **Staking**: Stake/unstake tokens, manage validator operations, claim rewards
- **Consensus**: Participate in consensus day voting, register as validity node
- **Blockchain Queries**: Query blocks, transactions, state, and chain info
- **Rich Terminal UI**: Beautiful output with tables, colors, and status indicators
- **WebSocket & HTTP**: Automatic fallback from WebSocket to HTTP
- **Secure Keystore**: Ed25519 keypair generation with encrypted storage

## Installation

### From Source (Development)

```bash
cd /Users/macbook/Desktop/etrid/13-clients/pye
pip install -e .
```

### From PyPI (Future)

```bash
pip install pye
```

## Quick Start

### 1. Check Version

```bash
pye version
```

### 2. Create an Account

```bash
pye account create alice
```

### 3. Check Network Info

```bash
pye info
```

### 4. Query Your Balance

```bash
pye query balance alice
```

### 5. Send Tokens

```bash
pye send 0x123... 100 -f alice
```

## Command Reference

### Account Management

```bash
# Create new account
pye account create [NAME]

# List all accounts
pye account list

# Show account details
pye account show [NAME]

# Export account (public key only)
pye account export [NAME] -o output.json

# Delete account
pye account delete [NAME]
```

### Query Commands

```bash
# Query latest block
pye query block

# Query block by number
pye query block -n 1000

# Query account balance
pye query balance [ADDRESS]

# Query transaction
pye query transaction [TX_HASH]

# Query account info
pye query account [ADDRESS]

# Query chain info
pye query chain

# Query node health
pye query health

# Query state
pye query state [STORAGE_KEY]
```

### Send Transactions

```bash
# Send ETR tokens
pye send [TO_ADDRESS] [AMOUNT] -f [FROM_ADDRESS]

# Send with auto-confirmation
pye send 0x123... 100 -f alice -y
```

### Staking Operations

```bash
# Stake tokens
pye stake deposit [AMOUNT] -a [ACCOUNT]

# Unstake tokens
pye stake withdraw [AMOUNT] -a [ACCOUNT]

# Query staking info
pye stake info -a [ACCOUNT]

# Query rewards
pye stake rewards -a [ACCOUNT]

# Claim rewards
pye stake claim -a [ACCOUNT]

# List validators
pye stake validators
```

### Consensus Operations

```bash
# Get consensus status
pye consensus status

# Register as validity node
pye consensus register -a [ACCOUNT] -s [STAKE_AMOUNT]

# Submit vote on proposal
pye consensus vote [PROPOSAL_ID] [yes|no|abstain] -a [ACCOUNT]

# List active proposals
pye consensus proposals

# Get proposal details
pye consensus proposal [PROPOSAL_ID]

# List validity nodes
pye consensus validators
```

## Configuration

### Environment Variables

- `ETRID_NODE_URL`: Node WebSocket URL (default: `ws://localhost:9944`)
- `ETRID_KEYSTORE`: Custom keystore directory (default: `~/.etrid/keystore`)

### Node URL

You can specify a custom node URL:

```bash
# Via environment variable
export ETRID_NODE_URL=ws://mainnet.etrid.io:9944
pye query chain

# Via command option
pye query chain --node ws://ember.etrid.io:9944
```

### Keystore Location

By default, accounts are stored in `~/.etrid/keystore`. You can customize this:

```bash
export ETRID_KEYSTORE=/path/to/custom/keystore
pye account create alice
```

## Examples

### Complete Workflow Example

```bash
# 1. Create account
pye account create alice

# 2. Check network status
pye info

# 3. Query balance
pye query balance alice

# 4. Stake tokens
pye stake deposit 1000 -a alice

# 5. Check staking info
pye stake info -a alice

# 6. Register as validator
pye consensus register -a alice -s 10000

# 7. Vote on proposal
pye consensus vote PROP-001 yes -a alice

# 8. Check rewards
pye stake rewards -a alice

# 9. Claim rewards
pye stake claim -a alice
```

### Scripting with pyE

```bash
#!/bin/bash

# Check if node is healthy
pye query health || exit 1

# Get latest block
BLOCK=$(pye query block)

# Send transaction
pye send 0x456... 100 -f alice -y

# Query transaction status
pye query tx $TX_HASH
```

## Python API Usage

You can also use pyE as a Python library:

```python
from pye import EtridClient

# Create client
with EtridClient(ws_url="ws://localhost:9944") as client:
    # Get latest block
    block = client.get_block()
    print(f"Latest block: {block}")

    # Check balance
    balance = client.get_balance("0x123...")
    print(f"Balance: {balance}")

    # Send transaction
    tx_hash = client.send_transaction(
        from_addr="0x123...",
        to_addr="0x456...",
        amount=100
    )
    print(f"Transaction: {tx_hash}")
```

## Security

### Private Key Storage

- Private keys are stored in `~/.etrid/keystore` with `0600` permissions
- Supports password encryption using industry-standard algorithms
- Never share your private keys or keystore files

### Best Practices

1. **Always use passwords** for production accounts
2. **Backup your keystore** regularly
3. **Never commit** keystore files to version control
4. **Use hardware wallets** for large amounts (future support)
5. **Verify addresses** before sending transactions

## Troubleshooting

### Connection Issues

```bash
# Check node health
pye query health

# Try different node URL
pye info --node ws://localhost:9944
```

### Account Not Found

```bash
# List all accounts
pye account list

# Check keystore location
echo $ETRID_KEYSTORE
```

### Transaction Failures

```bash
# Check balance
pye query balance alice

# Check node sync status
pye query chain

# Verify transaction hash
pye query tx [TX_HASH]
```

## Development

### Running Tests

```bash
pip install -e ".[dev]"
pytest tests/
```

### Code Formatting

```bash
black pye/
ruff check pye/
```

### Type Checking

```bash
mypy pye/
```

## Architecture

### Project Structure

```
pye/
├── pyproject.toml          # Project configuration
├── README.md               # This file
├── pye/
│   ├── __init__.py         # Package init
│   ├── cli.py              # Main CLI entry point
│   ├── client.py           # RPC client
│   └── commands/           # Command modules
│       ├── __init__.py
│       ├── account.py      # Account management
│       ├── stake.py        # Staking operations
│       ├── query.py        # Query commands
│       ├── send.py         # Send transactions
│       └── consensus.py    # Consensus operations
```

### Technology Stack

- **Click**: Modern CLI framework
- **Rich**: Beautiful terminal output
- **Pydantic**: Data validation
- **WebSocket-Client**: WebSocket connectivity
- **Cryptography**: Secure key management

## Contributing

Contributions are welcome! Please see [CONTRIBUTING.md](../../CONTRIBUTING.md) for guidelines.

## License

MIT License - see [LICENSE](../../LICENSE) for details.

## Links

- **Website**: https://etrid.io
- **Documentation**: https://docs.etrid.io
- **GitHub**: https://github.com/etrid/etrid
- **Discord**: https://discord.gg/etrid

## Support

For help and support:

- Documentation: https://docs.etrid.io/pye
- Discord: https://discord.gg/etrid
- GitHub Issues: https://github.com/etrid/etrid/issues

---

Built with ❤️ by the ËTRID Foundation
