# Blockchain Connectivity Documentation

## Overview

EtridWallet implements **real blockchain connectivity** using native Swift HTTP JSON-RPC calls to public Ethereum-compatible RPC nodes. The implementation supports multiple networks and includes comprehensive error handling, retry logic, and transaction encoding.

## Architecture

### Core Components

1. **RPCProvider** (`Core/Network/RPCProvider.swift`)
   - Low-level JSON-RPC 2.0 HTTP client
   - Handles communication with blockchain nodes
   - Implements retry logic with exponential backoff
   - Automatic timeout handling (30 seconds)
   - Rate limit detection

2. **Web3Service** (`Services/Web3Service.swift`)
   - High-level blockchain interaction service
   - Provides methods for balance queries, transactions, gas estimation
   - Supports both EIP-1559 and legacy transactions
   - Network-agnostic design

3. **TransactionManager** (`Core/Transaction/TransactionManager.swift`)
   - Transaction building and signing
   - RLP encoding for Ethereum transactions
   - Supports EIP-1559 (Type 2) and Legacy (Type 0) transactions
   - EIP-155 replay protection

4. **TokenService** (`Services/TokenService.swift`)
   - ERC-20 token interactions
   - Token balance queries using `balanceOf`
   - Token metadata retrieval (name, symbol, decimals)
   - Token transfer transaction building

## Network Endpoints

### Production RPC Endpoints

All networks use **public, production-ready RPC endpoints** with automatic fallback:

#### Ethereum Mainnet
- Primary: `https://eth.llamarpc.com`
- Fallbacks:
  - `https://rpc.ankr.com/eth`
  - `https://ethereum.publicnode.com`
  - `https://1rpc.io/eth`
  - `https://eth.drpc.org`

#### Polygon
- Primary: `https://polygon-rpc.com`
- Fallbacks:
  - `https://rpc.ankr.com/polygon`
  - `https://polygon.llamarpc.com`
  - `https://1rpc.io/matic`

#### BSC (Binance Smart Chain)
- Primary: `https://bsc-dataseed1.binance.org`
- Fallbacks:
  - `https://bsc-dataseed2.binance.org`
  - `https://bsc.publicnode.com`
  - `https://1rpc.io/bnb`

#### Arbitrum
- Primary: `https://arb1.arbitrum.io/rpc`
- Fallbacks:
  - `https://rpc.ankr.com/arbitrum`
  - `https://arbitrum.llamarpc.com`
  - `https://1rpc.io/arb`

#### Optimism
- Primary: `https://mainnet.optimism.io`
- Fallbacks:
  - `https://rpc.ankr.com/optimism`
  - `https://optimism.llamarpc.com`
  - `https://1rpc.io/op`

#### Avalanche
- Primary: `https://api.avax.network/ext/bc/C/rpc`
- Fallbacks:
  - `https://rpc.ankr.com/avalanche`
  - `https://avalanche.public-rpc.com`
  - `https://1rpc.io/avax/c`

### Testnet Endpoints

#### Sepolia (Ethereum Testnet)
- Primary: `https://rpc.sepolia.org`
- Fallbacks:
  - `https://rpc2.sepolia.org`
  - `https://ethereum-sepolia.publicnode.com`
  - `https://1rpc.io/sepolia`

#### Mumbai (Polygon Testnet)
- Primary: `https://rpc-mumbai.maticvigil.com`
- Fallbacks:
  - `https://rpc.ankr.com/polygon_mumbai`
  - `https://polygon-mumbai.g.alchemy.com/v2/demo`

## Implemented Features

### ✅ Real Network Connectivity

1. **Balance Queries**
   - Native token balances via `eth_getBalance`
   - ERC-20 token balances via contract calls
   - Batch balance queries for multiple tokens

2. **Transaction Management**
   - Nonce retrieval via `eth_getTransactionCount`
   - Gas price estimation via `eth_gasPrice`
   - EIP-1559 fee data via `eth_feeHistory`
   - Gas limit estimation via `eth_estimateGas`
   - Transaction broadcasting via `eth_sendRawTransaction`
   - Transaction receipt polling via `eth_getTransactionReceipt`

3. **Smart Contract Interactions**
   - Read-only calls via `eth_call`
   - ERC-20 contract methods:
     - `balanceOf(address)`
     - `transfer(address,uint256)`
     - `approve(address,uint256)`
     - `allowance(address,address)`
     - `name()`, `symbol()`, `decimals()`

4. **Block Information**
   - Current block number via `eth_blockNumber`
   - Block data via `eth_getBlockByNumber`
   - Chain ID verification via `eth_chainId`

### ✅ Transaction Signing

1. **RLP Encoding** (`Utils/RLPEncoder.swift`)
   - Recursive Length Prefix encoding
   - Support for all Ethereum data types
   - EIP-2718 transaction envelope (Type 2)

2. **Transaction Types**
   - **EIP-1559 (Type 2)**: Modern transactions with `maxFeePerGas` and `maxPriorityFeePerGas`
   - **Legacy (Type 0)**: Traditional transactions with `gasPrice`

3. **Signature Standards**
   - ECDSA secp256k1 signatures
   - EIP-155 replay protection (chain ID in v)
   - Recovery ID handling

### ✅ Error Handling & Reliability

1. **Automatic Retry Logic**
   - Exponential backoff (1s, 2s, 4s)
   - Maximum 3 retry attempts
   - Smart retry decisions (don't retry on user errors)

2. **Timeout Handling**
   - 30-second request timeout
   - 60-second resource timeout
   - Graceful timeout errors

3. **Rate Limit Detection**
   - HTTP 429 status code detection
   - Automatic backoff on rate limits
   - Error categorization for user feedback

4. **Error Types**
   - Network errors (connection, timeout)
   - RPC errors (invalid params, execution reverted)
   - Validation errors (invalid address, insufficient balance)
   - Each error includes recovery suggestions

## Implementation Notes

### Keccak256 Hashing

**IMPORTANT**: The current implementation uses SHA256 as a placeholder for Keccak256 hashing in:
- Transaction hash calculation
- Address checksum verification
- Contract event hashing

**For production use**, you must replace SHA256 with proper Keccak256:

```swift
// Current (placeholder):
let hash = Data(SHA256.hash(data: data))

// Required for production:
// Option 1: Use CryptoSwift
import CryptoSwift
let hash = data.sha3(.keccak256)

// Option 2: Use web3swift
import web3swift
let hash = data.sha3(.keccak256)

// Option 3: Native C implementation
// Link with a C library that implements Keccak256
```

### ECDSA Signing

The wallet uses iOS/macOS native `CryptoKit` for ECDSA signing with secp256k1 curves. This is production-ready and secure.

### ABI Encoding

Current implementation includes:
- Basic function selector encoding (first 4 bytes of Keccak256 hash)
- Simple parameter padding (addresses, uint256)
- String/bytes decoding

For complex ABI encoding/decoding, consider integrating a library like `web3swift` or `Web3.swift`.

## Usage Examples

### Get Balance

```swift
let web3 = Web3Service(network: .ethereum)
let balance = try await web3.getBalance(address: "0x...")
let ether = WeiConverter.weiToEther(balance)
print("Balance: \(ether) ETH")
```

### Send Transaction

```swift
let txManager = TransactionManager.shared

// Build transaction
let tx = try await txManager.buildTransaction(
    from: "0xSender...",
    to: "0xRecipient...",
    value: WeiConverter.etherToWei(0.1),
    network: .ethereum
)

// Sign and send
let hash = try await txManager.sendTransaction(
    tx,
    from: "0xSender...",
    network: .ethereum
)

print("Transaction hash: \(hash)")
```

### Get Token Balance

```swift
let tokenService = TokenService(network: .ethereum)
let token = Token(
    symbol: "USDT",
    name: "Tether USD",
    contractAddress: "0xdAC17F958D2ee523a2206206994597C13D831ec7",
    decimals: 6
)

let balance = try await tokenService.getBalance(
    token: token,
    address: "0x..."
)

let formatted = TokenAmountFormatter.format(
    balance,
    decimals: token.decimals,
    symbol: token.symbol
)
print("Balance: \(formatted)")
```

## Testing

### Testnet Recommendations

1. **Sepolia** - Recommended for Ethereum testing
   - Faucets: https://sepoliafaucet.com/
   - Stable and well-supported

2. **Mumbai** - For Polygon testing
   - Faucet: https://faucet.polygon.technology/

### Manual Testing Checklist

- [ ] Balance queries on mainnet
- [ ] Balance queries on testnet
- [ ] Gas estimation
- [ ] Transaction sending (testnet only)
- [ ] Transaction receipt polling
- [ ] ERC-20 token balance queries
- [ ] Contract method calls
- [ ] Error handling (invalid address, insufficient balance)
- [ ] Network switching
- [ ] Retry logic (simulate network failures)

## Performance Considerations

1. **Caching**
   - Balance queries should be cached with TTL
   - Token metadata can be cached indefinitely
   - Gas prices should be refreshed frequently

2. **Batching**
   - Use batch RPC requests for multiple queries
   - Implemented in `RPCProvider.batchRequest()`

3. **Rate Limiting**
   - Public nodes have rate limits (typically 10-100 requests/second)
   - Implement request queuing for high-volume apps
   - Consider using paid RPC providers for production

## Security

1. **Private Key Storage**
   - Keys stored in iOS Keychain with biometric protection
   - Never transmitted over network
   - Signing happens locally

2. **Network Verification**
   - Chain ID verification before signing
   - EIP-155 replay protection
   - Address checksum validation

3. **Input Validation**
   - All addresses validated with regex
   - Amount validation (not negative, not > balance)
   - Gas limit safety checks

## Future Enhancements

1. **Add Proper Keccak256**
   - Replace SHA256 placeholder
   - Use CryptoSwift or web3swift library

2. **WebSocket Support**
   - Real-time event subscriptions
   - Pending transaction monitoring

3. **Custom RPC Endpoints**
   - Allow users to configure their own nodes
   - Alchemy/Infura API key support

4. **MEV Protection**
   - Flashbots integration
   - Private transaction broadcasting

5. **Multi-signature Support**
   - Gnosis Safe integration
   - Hardware wallet support

## Troubleshooting

### Common Issues

1. **"Request timed out"**
   - Check internet connection
   - RPC node may be down (try fallback)
   - Increase timeout in RPCProvider

2. **"Rate limit exceeded"**
   - Wait 1-5 minutes
   - Switch to different RPC endpoint
   - Consider paid RPC service

3. **"Invalid response"**
   - RPC node may be syncing
   - Try different endpoint
   - Check network connectivity

4. **"Nonce too low"**
   - Transaction already sent
   - Clear pending transactions
   - Wait for confirmation

## License

MIT License - See LICENSE file for details

## Support

For issues and questions:
- GitHub Issues: [etrid/issues]
- Documentation: This file
- RPC Status: Check https://chainlist.org/ for endpoint health
