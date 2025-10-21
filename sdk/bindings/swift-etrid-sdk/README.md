# Swift Ëtrid SDK

**Status:** 📋 Planned for v1.3 (Post-Mainnet)

## Overview

Swift bindings for the Ëtrid blockchain SDK for iOS and macOS development.

## Planned Features

- **Native iOS/macOS**: Swift Package Manager support
- **Async/Await**: Modern Swift concurrency
- **SwiftUI Ready**: Combine integration
- **Type-Safe**: Leverages Swift's type system
- **Keychain Integration**: Secure key storage

## Installation (When Available)

### Swift Package Manager

```swift
dependencies: [
    .package(url: "https://github.com/etrid/etrid-swift-sdk.git", from: "1.3.0")
]
```

### CocoaPods

```ruby
pod 'EtridSDK', '~> 1.3'
```

## Usage Example (Planned API)

```swift
import EtridSDK

// Connect to FlareChain
let client = try await EtridClient.connect(endpoint: "wss://flarechain.etrid.io")

// Create or import wallet
let wallet = try Wallet(mnemonic: "your twelve word mnemonic...")

// Get balance
let balance = try await client.getBalance(address: wallet.address)
print("Balance: \\(balance) ETR")

// Send transaction
let tx = try await client.transfer(
    from: wallet,
    to: "5Gx...",
    amount: "1000000000000", // 1 ETR
    chain: .flarechain
)

// Multichain operations
let btcPbc = try await client.connectChain(.btcPbc)
let btcBalance = try await btcPbc.getBridgedBalance(address: wallet.address)
```

## SwiftUI Integration

```swift
import SwiftUI
import EtridSDK

struct WalletView: View {
    @StateObject private var wallet = EtridWallet()

    var body: some View {
        VStack {
            Text("Balance: \\(wallet.balance) ETR")

            Button("Send") {
                Task {
                    try await wallet.send(
                        to: recipientAddress,
                        amount: amount
                    )
                }
            }
        }
        .task {
            await wallet.connect()
        }
    }
}
```

## Roadmap

- **v1.3.0**: Initial release with core wallet functionality
- **v1.3.1**: Keychain integration
- **v1.4.0**: Face ID/Touch ID support
- **v1.5.0**: WalletConnect protocol

## Development

This SDK will be implemented after mainnet deployment.

**Target Timeline:** Q3 2026 (post-mainnet)

## Temporary Alternative

The current Flutter mobile wallet already works on iOS:

See: `apps/wallet-mobile/etrid-wallet/`

The Swift SDK will provide a native alternative for iOS-only projects.
