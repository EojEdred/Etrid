/// Ëtrid SDK for Swift
///
/// Provides a comprehensive interface to interact with Ëtrid Protocol blockchain.
///
/// Example:
/// ```swift
/// let client = EtridClient(endpoint: "ws://localhost:9944")
/// await client.connect()
///
/// let account = Account.generate()
/// let balance = try await client.query.balance(account.address)
/// print("Balance: \(balance.free) ETR")
/// ```

import Foundation

/// SDK version
public let version = "0.1.0"

/// Re-export main types
public typealias Client = EtridClient
