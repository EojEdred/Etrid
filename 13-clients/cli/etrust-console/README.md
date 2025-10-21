# ËTRUST - ËTRID Rust CLI

Professional command-line interface for ËTRID Protocol, inspired by Ethereum's Reth/Lighthouse CLI patterns.

## Features

- **Account Management**: Create, import, export, and manage accounts
- **Staking Operations**: Stake, unstake, nominate validators
- **Blockchain Queries**: Query blocks, transactions, balances, and network state
- **Transaction Submission**: Send transfers, deploy contracts, call smart contracts
- **Consensus Operations**: Submit proposals, vote, check consensus day status
- **Key Management**: Generate keys, derive child keys, inspect keys

## Installation

### Build from Source

```bash
cd /Users/macbook/Desktop/etrid/13-clients/etrust
cargo build --release
```

The binary will be available at `target/release/etrust`.

### Add to PATH (Optional)

```bash
# Add to your shell profile (~/.zshrc, ~/.bashrc, etc.)
export PATH="/Users/macbook/Desktop/etrid/13-clients/etrust/target/release:$PATH"
```

## Usage

### Global Options

```bash
# Specify RPC endpoint
etrust --endpoint ws://localhost:9944 <command>

# Or use environment variable
export ETRID_RPC_ENDPOINT=ws://localhost:9944
etrust <command>
```

## Commands

### Account Management

#### Create a New Account

```bash
etrust account create
etrust account create --name "Alice"
etrust account create --name "Bob" --password "secure123"
```

#### Import an Account

```bash
# From mnemonic phrase
etrust account import --secret "word1 word2 ... word12" --name "Alice"

# From hex private key
etrust account import --secret "0x1234..." --name "Bob"
```

#### List Accounts

```bash
etrust account list
```

#### Export Account

```bash
etrust account export Alice --format json
etrust account export Bob --format seed
```

#### Check Balance

```bash
etrust account balance 5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY
```

### Staking Operations

#### Stake Tokens

```bash
# Self-staking
etrust stake stake 1000 --from 5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY

# Nominating a validator
etrust stake stake 1000 --from <YOUR_ADDRESS> --validator <VALIDATOR_ADDRESS>
```

#### Unstake Tokens

```bash
etrust stake unstake 500 --from 5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY
```

#### Query Staking Info

```bash
etrust stake info 5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY
```

#### List Validators

```bash
etrust stake validators
```

#### Nominate Validator

```bash
etrust stake nominate <VALIDATOR_ADDRESS> 1000 --from <YOUR_ADDRESS>
```

### Blockchain Queries

#### Get Block Information

```bash
# By block number
etrust query block 100

# By block hash
etrust query block 0x1234...

# Latest block
etrust query block latest
```

#### Get Transaction

```bash
etrust query transaction 0xabcd1234...
```

#### Get Balance

```bash
etrust query balance 5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY
```

#### Get Chain Info

```bash
etrust query chain-info
```

#### Get Network Peers

```bash
etrust query peers
```

#### Get Current Block Number

```bash
etrust query block-number
```

### Send Transactions

#### Transfer Tokens

```bash
# Transfer ETR
etrust send transfer <TO_ADDRESS> 100 --from <FROM_ADDRESS>

# Transfer ETD (stablecoin)
etrust send transfer <TO_ADDRESS> 50 --from <FROM_ADDRESS> --token ETD

# Transfer VMW (gas token)
etrust send transfer <TO_ADDRESS> 10 --from <FROM_ADDRESS> --token VMW
```

#### Deploy Smart Contract

```bash
etrust send deploy ./contract.wasm --from <YOUR_ADDRESS>
etrust send deploy ./contract.wasm --from <YOUR_ADDRESS> --args '{"arg1": "value1"}'
```

#### Call Smart Contract

```bash
etrust send call <CONTRACT_ADDRESS> method_name --from <YOUR_ADDRESS>
etrust send call <CONTRACT_ADDRESS> transfer --args '{"to": "0x...", "amount": 100}' --from <YOUR_ADDRESS>
```

### Consensus Operations

#### Submit Governance Proposal

```bash
etrust consensus propose-submit "Proposal Title" "Detailed description" --from <YOUR_ADDRESS>
```

#### List Active Proposals

```bash
etrust consensus propose-list
```

#### Vote on Proposal

```bash
etrust consensus vote 1 yes --from <YOUR_ADDRESS>
etrust consensus vote 2 no --from <YOUR_ADDRESS>
etrust consensus vote 3 abstain --from <YOUR_ADDRESS>
```

#### Get Proposal Details

```bash
etrust consensus proposal-info 1
```

#### Check Consensus Day Status

```bash
etrust consensus status
```

#### View Distribution Schedule

```bash
etrust consensus distribution
```

### Key Management

#### Generate Keypair

```bash
# SR25519 (default, recommended for Substrate)
etrust keys generate
etrust keys generate --key-type sr25519

# ED25519
etrust keys generate --key-type ed25519

# ECDSA
etrust keys generate --key-type ecdsa
```

#### Derive Child Key

```bash
etrust keys derive "word1 word2 ... word12" "//Alice"
etrust keys derive "word1 word2 ... word12" "//Alice//stash"
```

#### Inspect Key

```bash
# From mnemonic
etrust keys inspect "word1 word2 ... word12"

# From hex seed
etrust keys inspect 0x1234567890abcdef...

# With derivation path
etrust keys inspect "word1 word2 ... word12//Alice"
```

#### Generate Mnemonic

```bash
etrust keys mnemonic
```

## Configuration

### RPC Endpoint

Default: `ws://localhost:9944`

Set via:
- Command line flag: `--endpoint ws://your-node:9944`
- Environment variable: `ETRID_RPC_ENDPOINT=ws://your-node:9944`

### Logging

Set log level via `RUST_LOG` environment variable:

```bash
# Info level (default)
export RUST_LOG=info
etrust <command>

# Debug level
export RUST_LOG=debug
etrust <command>

# Trace level (very verbose)
export RUST_LOG=trace
etrust <command>
```

## Examples

### Complete Workflow

```bash
# 1. Generate a new account
etrust keys generate

# 2. Check your balance
etrust query balance <YOUR_ADDRESS>

# 3. Stake tokens to become a validator
etrust stake stake 10000 --from <YOUR_ADDRESS>

# 4. Check staking info
etrust stake info <YOUR_ADDRESS>

# 5. Submit a governance proposal
etrust consensus propose-submit "Increase validator count" "We should increase..." --from <YOUR_ADDRESS>

# 6. Vote on proposals
etrust consensus vote 1 yes --from <YOUR_ADDRESS>

# 7. Check consensus day status
etrust consensus status
```

### Scripting Examples

```bash
# Monitor block production
watch -n 5 'etrust query block-number'

# Check balances for multiple accounts
for addr in addr1 addr2 addr3; do
    echo "Balance for $addr:"
    etrust query balance $addr
    echo
done

# Query chain info every minute
while true; do
    etrust query chain-info
    sleep 60
done
```

## Development Status

This is an MVP (Minimum Viable Product) implementation for mainnet launch. Current capabilities:

✅ **Implemented:**
- Complete CLI structure following Reth/Lighthouse patterns
- Account generation and key management
- RPC client with WebSocket connection
- Basic query operations (blocks, balance, chain info)
- Command routing and error handling

⏳ **Requires Full Node Integration:**
- Transaction signing and submission (requires keystore)
- Contract deployment (requires ETWASM VM integration)
- Staking operations (requires pallet-staking integration)
- Governance operations (requires pallet-governance integration)

## Architecture

```
etrust/
├── src/
│   ├── main.rs           # Entry point
│   ├── cli.rs            # Command definitions (clap)
│   ├── rpc_client.rs     # JSON-RPC WebSocket client
│   └── commands/
│       ├── mod.rs        # Command module exports
│       ├── account.rs    # Account management
│       ├── stake.rs      # Staking operations
│       ├── query.rs      # Blockchain queries
│       ├── send.rs       # Transaction submission
│       ├── consensus.rs  # Governance & consensus day
│       └── keys.rs       # Key management
├── Cargo.toml            # Dependencies
└── README.md             # This file
```

## Dependencies

Key dependencies:
- `clap 4.5` - CLI argument parsing
- `tokio 1.38` - Async runtime
- `jsonrpsee 0.24` - JSON-RPC client
- `sp-core`, `sp-runtime` - Substrate types (from workspace)
- `serde`, `serde_json` - Serialization
- `colored` - Terminal colors

## Troubleshooting

### Connection Failed

```
Error: Failed to connect to RPC endpoint: ws://localhost:9944
```

**Solution:** Ensure your ËTRID node is running and accessible.

```bash
# Check if node is running
curl -H "Content-Type: application/json" -d '{"id":1, "jsonrpc":"2.0", "method":"system_chain"}' http://localhost:9944
```

### Invalid Mnemonic

```
Error: Invalid mnemonic phrase
```

**Solution:** Ensure your mnemonic has 12 or 24 words, properly formatted.

### Build Errors

```
error: failed to resolve dependencies
```

**Solution:** Ensure you're in the workspace root:

```bash
cd /Users/macbook/Desktop/etrid
cargo build --release -p etrust
```

## Contributing

This CLI is part of the ËTRID Protocol. For contribution guidelines, see the main repository.

## License

Apache-2.0

## Links

- ËTRID Website: https://etrid.io
- GitHub: https://github.com/etrid/etrid
- Documentation: https://docs.etrid.io

---

**Built with ❤️ by the ËTRID Foundation**
