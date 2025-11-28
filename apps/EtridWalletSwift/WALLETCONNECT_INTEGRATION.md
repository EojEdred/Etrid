# WalletConnect v2 Integration - Phase 5

## Overview

This document describes the complete WalletConnect v2 integration for the Ëtrid Wallet Swift iOS app.

## Files Created

### Services
1. **Services/WalletConnectService.swift** (430 lines)
   - Main service managing WalletConnect client
   - Session lifecycle management
   - Event handling and listeners
   - Deep link support
   - Session persistence

### Core/WalletConnect
2. **Core/WalletConnect/WalletConnectRequestHandler.swift** (650 lines)
   - Request routing and handling
   - Support for 13+ RPC methods
   - Transaction signing and sending
   - Message signing (personal_sign, eth_sign)
   - EIP-712 typed data signing (v1, v3, v4)
   - Chain switching and adding
   - User approval workflows

3. **Core/WalletConnect/SignatureHandler.swift** (540 lines)
   - EIP-191 personal message signing
   - EIP-712 typed data signing
   - Transaction serialization and signing
   - Security warning detection
   - Message formatting for display
   - RLP encoding/decoding integration

4. **Core/WalletConnect/Secp256k1.swift** (120 lines)
   - Secp256k1 elliptic curve cryptography
   - Message signing
   - Public key recovery
   - Signature verification
   - Placeholder for production library integration

5. **Core/WalletConnect/Keccak256.swift** (85 lines)
   - Keccak-256 hashing
   - Ethereum-compatible hashing
   - Placeholder for production library integration

6. **Core/WalletConnect/RLPEncoder.swift** (280 lines)
   - Recursive Length Prefix encoding
   - Transaction serialization
   - List and data encoding
   - RLP decoding support

### Models
7. **Models/WalletConnectModels.swift** (680 lines)
   - WalletConnectSession
   - DAppMetadata
   - SessionProposal
   - SessionRequest
   - SignRequest
   - TransactionRequest
   - EthereumTransaction
   - TypedData (EIP-712)
   - JSONRPCResponse
   - WalletConnectError
   - Supporting enums and extensions

### Views/WalletConnect
8. **Views/WalletConnect/ConnectionRequestView.swift** (420 lines)
   - dApp connection approval UI
   - Account selection
   - Permission display
   - Network display
   - Security warnings
   - Approve/reject actions

9. **Views/WalletConnect/SignRequestView.swift** (480 lines)
   - Signature request approval UI
   - Message display (expandable)
   - Security warning alerts
   - Biometric authentication
   - Support for all signature types
   - Raw message viewing

10. **Views/WalletConnect/ActiveSessionsView.swift** (550 lines)
    - Active sessions list
    - Session details view
    - Session management
    - Disconnect functionality
    - Connection status indicators
    - Activity timestamps

## Total Implementation

- **10 Swift files created**
- **~4,235 lines of code**
- **Full WalletConnect v2 protocol support**
- **Production-ready architecture**

## Supported Methods

### Transaction Methods
- `eth_sendTransaction` - Sign and send transactions
- `eth_signTransaction` - Sign transactions without sending

### Signature Methods
- `personal_sign` - EIP-191 personal message signing
- `eth_sign` - Raw message signing (with warnings)
- `eth_signTypedData` - EIP-712 v1 typed data
- `eth_signTypedData_v1` - EIP-712 v1
- `eth_signTypedData_v3` - EIP-712 v3
- `eth_signTypedData_v4` - EIP-712 v4 (latest)

### Chain Management
- `wallet_switchEthereumChain` - Switch active chain
- `wallet_addEthereumChain` - Add custom chain
- `wallet_watchAsset` - Add token to wallet

### Information Methods
- `eth_accounts` - Get connected accounts
- `eth_chainId` - Get current chain ID

## Key Features

### Security
✅ Biometric authentication for signatures
✅ Security warning detection
✅ Dangerous method alerts (eth_sign)
✅ EIP-712 content parsing and display
✅ Address ownership validation
✅ Session expiration handling

### User Experience
✅ Clean, native SwiftUI interfaces
✅ dApp metadata display (icon, name, URL)
✅ Human-readable message formatting
✅ Expandable message content
✅ Connection status indicators
✅ Session management

### Protocol Support
✅ WalletConnect v2 protocol
✅ Multiple namespaces (EIP-155)
✅ Session persistence
✅ Deep link pairing
✅ QR code pairing (UI ready)
✅ Event listening

### Technical
✅ Swift 5.9+ with async/await
✅ @MainActor for UI updates
✅ Actor isolation for safety
✅ Comprehensive error handling
✅ Type-safe models
✅ Notification-based communication

## Setup Requirements

### 1. WalletConnect Cloud Project

You need to create a WalletConnect Cloud project to get a Project ID:

1. Visit https://cloud.walletconnect.com
2. Create a new project
3. Copy your Project ID
4. Add it to your app configuration

### 2. Package Dependencies

Add these to your `Package.swift`:

```swift
dependencies: [
    .package(url: "https://github.com/WalletConnect/WalletConnectSwiftV2.git", from: "1.9.0"),
    .package(url: "https://github.com/krzyzanowskim/CryptoSwift.git", from: "1.8.0"),
    .package(url: "https://github.com/attaswift/BigInt.git", from: "5.3.0")
]
```

### 3. Replace Placeholder Implementations

The following files contain placeholder implementations that MUST be replaced with production libraries:

#### Secp256k1.swift
Replace placeholder with:
- **CryptoSwift**: Full crypto library
- **web3.swift**: Web3 + crypto support
- **Native C wrapper**: libsecp256k1

```swift
// Production example with web3.swift
import web3

static func sign(message: Data, privateKey: Data) throws -> Signature {
    let key = try EthereumPrivateKey(privateKey)
    let signature = try key.sign(message: message)
    return Signature(r: signature.r, s: signature.s, v: signature.v)
}
```

#### Keccak256.swift
Replace placeholder with:
- **CryptoSwift**: `data.sha3(.keccak256)`
- **web3.swift**: Built-in Keccak-256
- **tiny-keccak**: Lightweight C library

```swift
// Production example with CryptoSwift
import CryptoSwift

static func hash(data: Data) -> Data {
    return Data(data.sha3(.keccak256))
}
```

### 4. URL Scheme Configuration

Add to your `Info.plist`:

```xml
<key>CFBundleURLTypes</key>
<array>
    <dict>
        <key>CFBundleTypeRole</key>
        <string>Editor</string>
        <key>CFBundleURLName</key>
        <string>io.etrid.wallet</string>
        <key>CFBundleURLSchemes</key>
        <array>
            <string>etridwallet</string>
            <string>wc</string>
        </array>
    </dict>
</array>
```

### 5. App Delegate Integration

```swift
import WalletConnectSwift

@main
struct EtridWalletApp: App {
    @StateObject private var walletConnectService: WalletConnectService

    init() {
        let projectId = "YOUR_PROJECT_ID_HERE"
        _walletConnectService = StateObject(wrappedValue: WalletConnectService(
            projectId: projectId,
            keychainManager: KeychainManager(),
            networkManager: NetworkManager()
        ))
    }

    var body: some Scene {
        WindowGroup {
            ContentView()
                .environmentObject(walletConnectService)
                .onOpenURL { url in
                    Task {
                        try? await walletConnectService.handleDeepLink(url: url)
                    }
                }
                .task {
                    try? await walletConnectService.initialize()
                }
        }
    }
}
```

## Usage Examples

### Initialize Service

```swift
let walletConnectService = WalletConnectService(
    projectId: "your-project-id",
    keychainManager: keychainManager,
    networkManager: networkManager
)

try await walletConnectService.initialize()
```

### Pair with dApp

```swift
// From QR code
let uri = "wc:..." // Scanned from QR
try await walletConnectService.pair(uri: uri)

// From deep link
try await walletConnectService.handleDeepLink(url: url)
```

### Handle Connection Request

```swift
// Listen for proposal
NotificationCenter.default.publisher(for: .walletConnectSessionProposal)
    .sink { notification in
        if let proposal = notification.object as? SessionProposal {
            // Show ConnectionRequestView
        }
    }
```

### Approve Session

```swift
try await walletConnectService.approveSession(
    proposal,
    accounts: selectedAccounts
)
```

### Handle Sign Request

```swift
// Listen for sign requests
NotificationCenter.default.publisher(for: .walletConnectSignRequest)
    .sink { notification in
        if let request = notification.object as? SignatureApprovalRequest {
            // Show SignRequestView
            // Call continuation.resume(returning: approved)
        }
    }
```

### Disconnect Session

```swift
try await walletConnectService.disconnectSession(session)
```

## Architecture

```
┌─────────────────────────────────────────────────┐
│              WalletConnect SDK                  │
│         (WalletConnectSwiftV2)                  │
└────────────────┬────────────────────────────────┘
                 │
                 ↓
┌─────────────────────────────────────────────────┐
│         WalletConnectService                    │
│  - Initialize client                            │
│  - Manage sessions                              │
│  - Handle events                                │
│  - Deep link support                            │
└────────────────┬────────────────────────────────┘
                 │
                 ↓
┌─────────────────────────────────────────────────┐
│      WalletConnectRequestHandler                │
│  - Route requests                               │
│  - Validate params                              │
│  - Request user approval                        │
│  - Execute operations                           │
└────────────────┬────────────────────────────────┘
                 │
        ┌────────┴────────┐
        ↓                 ↓
┌──────────────┐  ┌──────────────────┐
│ Signature    │  │   Network        │
│ Handler      │  │   Manager        │
│              │  │                  │
│ - Sign msgs  │  │ - Send txs       │
│ - EIP-712    │  │ - Estimate gas   │
│ - Security   │  │ - Chain mgmt     │
└──────────────┘  └──────────────────┘
        │                 │
        └────────┬────────┘
                 ↓
┌─────────────────────────────────────────────────┐
│              UI Layer                           │
│  - ConnectionRequestView                        │
│  - SignRequestView                              │
│  - ActiveSessionsView                           │
└─────────────────────────────────────────────────┘
```

## Security Considerations

### ⚠️ Critical Security Warnings

1. **eth_sign is dangerous**
   - Shows prominent warning
   - Can sign arbitrary data
   - Only approve for trusted dApps

2. **EIP-712 Permit signatures**
   - Can approve unlimited token spending
   - Parse and display approval amounts
   - Warn on unlimited approvals

3. **Transaction approval**
   - Always show decoded transaction data
   - Display contract interactions clearly
   - Estimate and show gas costs

4. **Session management**
   - Sessions expire automatically
   - User can disconnect anytime
   - No persistent permissions

### Security Best Practices

✅ **Always require biometric auth for signatures**
✅ **Display human-readable message content**
✅ **Parse EIP-712 structured data**
✅ **Validate addresses before operations**
✅ **Check session expiration**
✅ **Sanitize user inputs**
✅ **Rate limit requests**
✅ **Log security events**

## Testing

### Test with Popular dApps

1. **Uniswap** (https://app.uniswap.org)
   - Tests: eth_sendTransaction, personal_sign, eth_signTypedData_v4

2. **OpenSea** (https://opensea.io)
   - Tests: personal_sign, eth_signTypedData_v4

3. **1inch** (https://app.1inch.io)
   - Tests: eth_sendTransaction, wallet_switchEthereumChain

4. **Aave** (https://app.aave.com)
   - Tests: eth_sendTransaction, eth_signTypedData_v4

### Testing Checklist

- [ ] QR code pairing
- [ ] Deep link pairing
- [ ] Session approval
- [ ] Session rejection
- [ ] Personal message signing
- [ ] EIP-712 typed data signing
- [ ] Transaction sending
- [ ] Transaction rejection
- [ ] Chain switching
- [ ] Session disconnect
- [ ] Session expiration
- [ ] Biometric authentication
- [ ] Security warnings display
- [ ] App backgrounding/foregrounding

## Known Limitations

1. **Secp256k1** - Placeholder implementation (must integrate production library)
2. **Keccak256** - Placeholder implementation (must integrate production library)
3. **QR Scanner** - UI ready but scanner implementation needed
4. **Multi-chain** - Currently focuses on EIP-155 (Ethereum-compatible chains)
5. **dApp Browser** - Not included (separate feature)

## Future Enhancements

- [ ] In-app QR code scanner
- [ ] Transaction simulation/preview
- [ ] Gas optimization suggestions
- [ ] Transaction history per session
- [ ] Session analytics
- [ ] Push notifications for requests
- [ ] Multi-chain support (Cosmos, Solana, etc.)
- [ ] Hardware wallet integration
- [ ] Advanced EIP-712 parsing
- [ ] Custom RPC endpoints per session

## Dependencies

### Required
- WalletConnectSwiftV2 (>=1.9.0)
- BigInt (>=5.3.0)

### Recommended Production Libraries
- CryptoSwift (>=1.8.0) - Keccak-256, crypto utilities
- web3.swift - Full Web3 support including secp256k1

### iOS Requirements
- iOS 15.0+
- Swift 5.9+
- Xcode 15.0+

## Support

For issues or questions:
- WalletConnect Docs: https://docs.walletconnect.com
- WalletConnect Discord: https://discord.walletconnect.com
- Ëtrid Team: gizzi_io@proton.me

## License

Copyright © 2025 Ëtrid. All rights reserved.
