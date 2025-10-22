# Ëtrid SDK for Swift

Swift library for interacting with the Ëtrid Protocol blockchain.

## Features

- ✅ Account management (create, import, sign)
- ✅ Async/await support
- ✅ WebSocket RPC client
- ✅ iOS 15+ and macOS 12+ support
- 🔨 Transaction building (in progress)
- 🔨 SwiftUI views (planned)

## Installation

### Swift Package Manager

Add to your `Package.swift`:

```swift
dependencies: [
    .package(url: "https://github.com/etrid/etrid-sdk-swift.git", from: "0.1.0")
]
```

Or add in Xcode: File > Add Package Dependencies

## Quick Start

```swift
import EtridSDK

// Connect to node
let client = EtridClient(endpoint: "ws://localhost:9944")
try await client.connect()

// Create account
let account = Account.generate()
print("Address: \(account.address)")

// Query balance
let balance = try await client.query.balance(account.address)
print("Balance: \(balance.free) ETR")

await client.disconnect()
```

## SwiftUI Example

```swift
import SwiftUI
import EtridSDK

struct WalletView: View {
    @State private var balance: Balance?
    let client = EtridClient(endpoint: "ws://localhost:9944")
    let account = Account.generate()

    var body: some View {
        VStack {
            Text("Address: \(account.address)")
            if let balance = balance {
                Text("Balance: \(balance.free) ETR")
            }
        }
        .task {
            try? await client.connect()
            balance = try? await client.query.balance(account.address)
        }
    }
}
```

## Documentation

See inline documentation for full API reference.

## Status

**Development Status**: Basic implementation complete, full features in progress.

## Requirements

- iOS 15.0+ / macOS 12.0+
- Swift 5.9+
- Xcode 15.0+
