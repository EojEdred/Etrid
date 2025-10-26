/// Common types used throughout the SDK

import Foundation

/// Account balance information
public struct Balance: Codable {
    /// Free balance
    public let free: UInt128
    /// Reserved balance
    public let reserved: UInt128
    /// Frozen balance
    public let frozen: UInt128

    public init(free: UInt128, reserved: UInt128, frozen: UInt128) {
        self.free = free
        self.reserved = reserved
        self.frozen = frozen
    }
}

/// Block information
public struct Block: Codable {
    /// Block number
    public let number: UInt64
    /// Block hash
    public let hash: String
    /// Parent hash
    public let parentHash: String
    /// State root
    public let stateRoot: String
}

/// Transaction hash
public typealias TxHash = String

/// Account address (SS58 encoded)
public typealias Address = String

/// SDK errors
public enum EtridError: Error {
    case notConnected
    case invalidResponse
    case cryptoError(String)
    case networkError(String)
}

/// UInt128 helper (Swift doesn't have native UInt128)
public struct UInt128: Codable, Comparable {
    public let high: UInt64
    public let low: UInt64

    public init(_ value: UInt64) {
        self.high = 0
        self.low = value
    }

    public init(high: UInt64, low: UInt64) {
        self.high = high
        self.low = low
    }

    public static func < (lhs: UInt128, rhs: UInt128) -> Bool {
        if lhs.high == rhs.high {
            return lhs.low < rhs.low
        }
        return lhs.high < rhs.high
    }

    public static func == (lhs: UInt128, rhs: UInt128) -> Bool {
        lhs.high == rhs.high && lhs.low == rhs.low
    }
}
