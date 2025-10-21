# etrcpp - ËTRID C++ CLI

**etrcpp** is ËTRID's official C++ command-line interface, inspired by Bitcoin Core's bitcoin-cli architecture. It provides a robust, high-performance CLI for interacting with ËTRID nodes.

## Features

- **Account Management**: Create, import, and manage ËTRID accounts
- **Staking Operations**: Stake/unstake tokens and manage validator status
- **Query Operations**: Query balances, blocks, transactions, and network status
- **Transaction Management**: Send transactions and broadcast raw transactions
- **Consensus Operations**: Monitor consensus days and submit validator votes
- **JSON-RPC Client**: Full-featured HTTP client with authentication support
- **Bitcoin Core Inspired**: Command structure follows bitcoin-cli patterns

## Prerequisites

### Required Dependencies

- **C++ Compiler**: GCC 7+, Clang 5+, or MSVC 2017+
- **CMake**: Version 3.15 or higher
- **libcurl**: For HTTP requests
- **nlohmann_json**: For JSON parsing (auto-downloaded if not installed)

### Installing Dependencies

#### macOS (with Homebrew)
```bash
brew install cmake curl nlohmann-json
```

#### Ubuntu/Debian
```bash
sudo apt-get update
sudo apt-get install cmake libcurl4-openssl-dev nlohmann-json3-dev build-essential
```

#### Fedora/RHEL
```bash
sudo dnf install cmake libcurl-devel json-devel gcc-c++
```

#### Windows (with vcpkg)
```powershell
vcpkg install curl nlohmann-json
```

## Building from Source

### Standard Build

```bash
# Navigate to etrcpp directory
cd /Users/macbook/Desktop/etrid/13-clients/etrcpp

# Create build directory
mkdir build && cd build

# Configure with CMake
cmake ..

# Build
cmake --build .

# Optional: Install system-wide
sudo cmake --install .
```

### Build with Specific Options

```bash
# Debug build
cmake -DCMAKE_BUILD_TYPE=Debug ..

# Release build with optimizations
cmake -DCMAKE_BUILD_TYPE=Release ..

# Specify custom install prefix
cmake -DCMAKE_INSTALL_PREFIX=/usr/local ..
```

### Building on macOS with Xcode

```bash
cmake -G Xcode ..
open etrcpp.xcodeproj
```

## Usage

### Basic Syntax

```bash
etrcpp [options] <command> [parameters]
```

### Connection Options

```bash
-rpcconnect=<ip>      Connect to ËTRID node on <ip> (default: 127.0.0.1)
-rpcport=<port>       Connect to ËTRID node on <port> (default: 9944)
-rpcuser=<user>       Username for RPC authentication
-rpcpassword=<pw>     Password for RPC authentication
-timeout=<n>          Connection timeout in seconds (default: 30)
```

### Account Commands

```bash
# Create a new account
etrcpp account create [name]

# List all accounts
etrcpp account list

# Get account information
etrcpp account info <address>

# Import account from private key
etrcpp account import <private_key> [name]
```

### Staking Commands

```bash
# Stake tokens
etrcpp stake <address> <amount>

# Unstake tokens (0 = unstake all)
etrcpp unstake <address> [amount]

# Get stake information
etrcpp stakeinfo <address>

# List all validators
etrcpp validators
```

### Query Commands

```bash
# Query account balance
etrcpp balance <address>

# Query block by height or hash
etrcpp block <height|hash>

# Query transaction by hash
etrcpp transaction <hash>

# Get blockchain information
etrcpp blockchaininfo

# Get network information
etrcpp networkinfo
```

### Transaction Commands

```bash
# Send transaction
etrcpp send <from> <to> <amount> [fee]

# Send raw transaction
etrcpp sendraw <hex>
```

### Consensus Commands

```bash
# Get current consensus day
etrcpp consensusday

# Get consensus day information
etrcpp consensusdayinfo <day_number>

# Submit consensus vote
etrcpp vote <validator_address> <proposal_id> <yes|no>
```

## Examples

### Create and Fund an Account

```bash
# Create account
etrcpp account create alice

# Check balance
etrcpp balance 0x1234567890123456789012345678901234567890

# Send funds
etrcpp send 0xfrom... 0xto... 1000000 1000
```

### Staking Operations

```bash
# Stake 10,000,000 tokens
etrcpp stake 0x1234567890123456789012345678901234567890 10000000

# Check stake info
etrcpp stakeinfo 0x1234567890123456789012345678901234567890

# List all validators
etrcpp validators

# Unstake all tokens
etrcpp unstake 0x1234567890123456789012345678901234567890 0
```

### Query Blockchain Data

```bash
# Get latest block
etrcpp block latest

# Get block by height
etrcpp block 12345

# Get block by hash
etrcpp block 0xabcdef...

# Get transaction details
etrcpp transaction 0xtxhash...

# Get blockchain info
etrcpp blockchaininfo
```

### Consensus Operations

```bash
# Get current consensus day
etrcpp consensusday

# Get consensus day 100 details
etrcpp consensusdayinfo 100

# Submit vote as validator
etrcpp vote 0xvalidator... proposal-123 yes
```

### Remote Node Connection

```bash
# Connect to remote node
etrcpp -rpcconnect=node.etrid.io -rpcport=9944 balance 0x...

# With authentication
etrcpp -rpcuser=admin -rpcpassword=secret balance 0x...

# Custom timeout (60 seconds)
etrcpp -timeout=60 blockchaininfo
```

## Configuration

### Environment Variables

You can set default connection parameters using environment variables:

```bash
export ETRCPP_RPC_HOST=127.0.0.1
export ETRCPP_RPC_PORT=9944
export ETRCPP_RPC_USER=admin
export ETRCPP_RPC_PASSWORD=secret
```

### Configuration File (Future Enhancement)

A configuration file `~/.etrcpp/etrcpp.conf` will be supported in future versions.

## Architecture

### Project Structure

```
etrcpp/
├── CMakeLists.txt          # Build configuration
├── README.md               # This file
├── include/
│   ├── types.h            # ËTRID data structures
│   ├── rpc_client.h       # JSON-RPC client interface
│   └── commands.h         # Command handlers
└── src/
    ├── etrcpp.cpp         # Main entry point
    ├── rpc_client.cpp     # JSON-RPC client implementation
    └── commands.cpp       # Command implementations
```

### Design Patterns

- **PIMPL Pattern**: Used in `RPCClient` for implementation hiding
- **Command Pattern**: Each CLI command maps to a handler method
- **RAII**: Smart pointers for resource management
- **Modern C++17**: Range-based loops, structured bindings, etc.

## Troubleshooting

### Build Issues

**Problem**: `nlohmann/json.hpp` not found
```bash
# Solution: Let CMake download it automatically, or install manually
brew install nlohmann-json  # macOS
sudo apt-get install nlohmann-json3-dev  # Ubuntu
```

**Problem**: `curl/curl.h` not found
```bash
# Solution: Install libcurl
brew install curl  # macOS
sudo apt-get install libcurl4-openssl-dev  # Ubuntu
```

### Runtime Issues

**Problem**: Connection refused
```bash
# Solution: Ensure ËTRID node is running
# Check node status and RPC port (default 9944)
```

**Problem**: Authentication error
```bash
# Solution: Provide correct credentials
etrcpp -rpcuser=admin -rpcpassword=secret balance 0x...
```

**Problem**: Timeout error
```bash
# Solution: Increase timeout
etrcpp -timeout=60 blockchaininfo
```

## Development

### Adding New Commands

1. Add command handler to `include/commands.h`
2. Implement handler in `src/commands.cpp`
3. Add command parsing in `src/etrcpp.cpp` `executeCommand()` function
4. Update help text in `printHelp()` function
5. Add tests and documentation

### Code Style

- Follow C++ Core Guidelines
- Use clang-format for formatting
- Prefer STL containers over raw arrays
- Use smart pointers for ownership
- Add comments for complex logic

## Testing

### Manual Testing

```bash
# Test connection
etrcpp networkinfo

# Test account creation
etrcpp account create test-account

# Test balance query
etrcpp balance 0x0000000000000000000000000000000000000000
```

### Automated Testing (Future Enhancement)

Unit tests and integration tests will be added in future versions.

## Performance

- **Async Operations**: Future versions will support async RPC calls
- **Connection Pooling**: Reuse connections for multiple commands
- **Caching**: Cache frequently accessed data (blocks, accounts)

## Security

- **HTTPS Support**: Use `-rpcconnect=https://...` for encrypted connections
- **Credential Storage**: Never store credentials in plain text
- **Input Validation**: All inputs are validated before RPC calls
- **Error Handling**: Comprehensive error handling prevents crashes

## Compatibility

- **ËTRID Node**: Compatible with ËTRID mainnet and testnet nodes
- **RPC Protocol**: JSON-RPC 2.0
- **Address Formats**: Supports both Ethereum-style (0x...) and ËTRID (etr...) addresses

## Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Test thoroughly
5. Submit a pull request

## License

Copyright (c) 2025 ËTRID Foundation
Licensed under MIT License

## Support

- **Documentation**: https://docs.etrid.io
- **GitHub Issues**: https://github.com/etrid/etrid
- **Community**: https://discord.gg/etrid

## Roadmap

### Version 1.1
- [ ] Configuration file support
- [ ] Bash completion
- [ ] Transaction signing
- [ ] Batch RPC calls

### Version 1.2
- [ ] WebSocket support
- [ ] Watch mode for monitoring
- [ ] Hardware wallet support
- [ ] Multi-signature support

### Version 2.0
- [ ] Full wallet functionality
- [ ] Smart contract deployment
- [ ] Advanced consensus features
- [ ] Performance optimizations

## Version History

- **v1.0.0** (2025-10-16): Initial MVP release for mainnet launch
  - Account management
  - Staking operations
  - Query commands
  - Transaction sending
  - Consensus day monitoring
  - Bitcoin Core inspired architecture

---

Built with care for the ËTRID ecosystem.
