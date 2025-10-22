/// RPC client for connecting to Ëtrid blockchain nodes

import Foundation
import Starscream

/// Ëtrid blockchain client
public actor EtridClient {
    private let endpoint: String
    private var socket: WebSocket?
    private var isConnected = false

    /// Create a new client instance
    /// - Parameter endpoint: WebSocket endpoint (e.g., "ws://localhost:9944")
    public init(endpoint: String) {
        self.endpoint = endpoint
    }

    /// Connect to the blockchain
    public func connect() async throws {
        var request = URLRequest(url: URL(string: endpoint)!)
        request.timeoutInterval = 5
        socket = WebSocket(request: request)

        // In a real implementation, would handle connection events
        socket?.connect()
        isConnected = true
    }

    /// Disconnect from the blockchain
    public func disconnect() {
        socket?.disconnect()
        isConnected = false
    }

    /// Get the current block number
    public func getBlockNumber() async throws -> UInt64 {
        guard isConnected else {
            throw EtridError.notConnected
        }
        // Placeholder implementation
        return 0
    }

    /// Get account balance
    /// - Parameter address: Account address (SS58 format)
    /// - Returns: Balance information
    public func getBalance(_ address: String) async throws -> Balance {
        guard isConnected else {
            throw EtridError.notConnected
        }
        // Placeholder implementation
        return Balance(free: 0, reserved: 0, frozen: 0)
    }

    /// Get the chain name
    public func getChainName() async throws -> String {
        guard isConnected else {
            throw EtridError.notConnected
        }
        // Placeholder implementation
        return "Ëtrid"
    }

    /// Query interface
    public var query: QueryAPI {
        QueryAPI(client: self)
    }
}

/// Query API helper
public struct QueryAPI {
    let client: EtridClient

    /// Get account balance
    public func balance(_ address: String) async throws -> Balance {
        try await client.getBalance(address)
    }

    /// Get current block number
    public func blockNumber() async throws -> UInt64 {
        try await client.getBlockNumber()
    }
}
