//
//  TokenDiscoveryService.swift
//  EtridWalletSwift
//
//  Created by Ëtrid Team
//  Copyright © 2025 Ëtrid. All rights reserved.
//

import Foundation

/// Errors that can occur during token discovery
enum TokenDiscoveryError: Error, LocalizedError {
    case invalidAddress
    case networkError(Error)
    case invalidResponse
    case decodingError(Error)
    case unsupportedChain
    case rpcError(String)

    var errorDescription: String? {
        switch self {
        case .invalidAddress:
            return "Invalid wallet address"
        case .networkError(let error):
            return "Network error: \(error.localizedDescription)"
        case .invalidResponse:
            return "Invalid response from server"
        case .decodingError(let error):
            return "Failed to decode response: \(error.localizedDescription)"
        case .unsupportedChain:
            return "Chain not supported for token discovery"
        case .rpcError(let message):
            return "RPC error: \(message)"
        }
    }
}

/// Network configuration for token discovery
struct Network: Sendable {
    let chainId: Int
    let name: String
    let rpcURL: String
    let explorerURL: String?
    let nativeCurrency: String

    static let ethereum = Network(
        chainId: 1,
        name: "Ethereum",
        rpcURL: "https://eth.llamarpc.com",
        explorerURL: "https://etherscan.io",
        nativeCurrency: "ETH"
    )

    static let bsc = Network(
        chainId: 56,
        name: "BNB Smart Chain",
        rpcURL: "https://bsc-dataseed.binance.org",
        explorerURL: "https://bscscan.com",
        nativeCurrency: "BNB"
    )

    static let polygon = Network(
        chainId: 137,
        name: "Polygon",
        rpcURL: "https://polygon-rpc.com",
        explorerURL: "https://polygonscan.com",
        nativeCurrency: "MATIC"
    )
}

/// Discovered token with balance
struct DiscoveredToken: Codable, Identifiable, Sendable {
    let id: UUID
    let metadata: TokenMetadata
    let balance: String
    let discoveredAt: Date
    let transactionCount: Int

    init(
        id: UUID = UUID(),
        metadata: TokenMetadata,
        balance: String,
        discoveredAt: Date = Date(),
        transactionCount: Int = 0
    ) {
        self.id = id
        self.metadata = metadata
        self.balance = balance
        self.discoveredAt = discoveredAt
        self.transactionCount = transactionCount
    }
}

/// Actor-based token discovery service for automatically detecting tokens
actor TokenDiscoveryService {

    // MARK: - Properties

    private let metadataService: TokenMetadataService
    private var discoveryCache: [String: [DiscoveredToken]] = [:]
    private let cacheTTL: TimeInterval = 300 // 5 minutes

    // ERC20 Transfer event signature
    private let transferEventSignature = "0xddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef"

    // MARK: - Initialization

    init(metadataService: TokenMetadataService) {
        self.metadataService = metadataService
    }

    // MARK: - Public Methods

    /// Discover all tokens held by a wallet address
    /// - Parameters:
    ///   - address: Wallet address to scan
    ///   - network: Network to scan on
    ///   - includeZeroBalance: Include tokens with zero balance
    /// - Returns: Array of discovered tokens with balances
    func discoverTokens(
        for address: String,
        network: Network,
        includeZeroBalance: Bool = false
    ) async throws -> [DiscoveredToken] {

        let normalizedAddress = address.lowercased()

        // Check cache first
        let cacheKey = "\(network.chainId)_\(normalizedAddress)"
        if let cached = discoveryCache[cacheKey] {
            return cached
        }

        // Get transaction history
        let transactions = try await getTransactionHistory(
            address: normalizedAddress,
            network: network
        )

        // Extract unique token addresses from Transfer events
        let tokenAddresses = await extractTokenAddresses(
            from: transactions,
            walletAddress: normalizedAddress
        )

        // Fetch metadata and balances for discovered tokens
        var discoveredTokens: [DiscoveredToken] = []

        await withTaskGroup(of: DiscoveredToken?.self) { group in
            for tokenAddress in tokenAddresses {
                group.addTask {
                    return await self.processToken(
                        contractAddress: tokenAddress,
                        walletAddress: normalizedAddress,
                        network: network,
                        includeZeroBalance: includeZeroBalance
                    )
                }
            }

            for await token in group {
                if let token = token {
                    discoveredTokens.append(token)
                }
            }
        }

        // Sort by balance value (if price data available)
        discoveredTokens.sort { token1, token2 in
            let balance1 = Double(token1.balance) ?? 0
            let balance2 = Double(token2.balance) ?? 0
            return balance1 > balance2
        }

        // Cache the results
        discoveryCache[cacheKey] = discoveredTokens

        return discoveredTokens
    }

    /// Scan for new tokens by analyzing recent transactions
    /// - Parameters:
    ///   - address: Wallet address to monitor
    ///   - network: Network to scan on
    ///   - knownTokens: Set of already known token addresses
    /// - Returns: Array of newly discovered tokens
    func scanForNewTokens(
        for address: String,
        network: Network,
        knownTokens: Set<String>
    ) async throws -> [DiscoveredToken] {

        let allTokens = try await discoverTokens(for: address, network: network)

        return allTokens.filter { token in
            !knownTokens.contains(token.metadata.contractAddress.lowercased())
        }
    }

    /// Get token balance for a specific contract
    func getTokenBalance(
        contractAddress: String,
        walletAddress: String,
        network: Network
    ) async throws -> String {

        let balanceOfSignature = "0x70a08231" // balanceOf(address)
        let paddedAddress = walletAddress.lowercased().replacingOccurrences(of: "0x", with: "")
            .padding(toLength: 64, withPad: "0", startingAt: 0)
        let data = balanceOfSignature + paddedAddress

        let result = try await callContract(
            rpcURL: network.rpcURL,
            contractAddress: contractAddress,
            data: data
        )

        // Remove 0x prefix and convert to decimal
        let hex = result.replacingOccurrences(of: "0x", with: "")
        guard let balance = UInt64(hex, radix: 16) else {
            return "0"
        }

        return String(balance)
    }

    /// Verify if a contract is a valid ERC20 token
    func verifyERC20(
        contractAddress: String,
        network: Network
    ) async -> Bool {

        // Try to call name(), symbol(), and decimals()
        let nameSignature = "0x06fdde03"
        let symbolSignature = "0x95d89b41"
        let decimalsSignature = "0x313ce567"

        do {
            async let nameCall = callContract(
                rpcURL: network.rpcURL,
                contractAddress: contractAddress,
                data: nameSignature
            )
            async let symbolCall = callContract(
                rpcURL: network.rpcURL,
                contractAddress: contractAddress,
                data: symbolSignature
            )
            async let decimalsCall = callContract(
                rpcURL: network.rpcURL,
                contractAddress: contractAddress,
                data: decimalsSignature
            )

            let _ = try await nameCall
            let _ = try await symbolCall
            let _ = try await decimalsCall

            return true
        } catch {
            return false
        }
    }

    /// Clear discovery cache
    func clearCache(address: String? = nil, network: Network? = nil) {
        if let address = address, let network = network {
            let cacheKey = "\(network.chainId)_\(address.lowercased())"
            discoveryCache.removeValue(forKey: cacheKey)
        } else {
            discoveryCache.removeAll()
        }
    }

    // MARK: - Private Methods

    private func processToken(
        contractAddress: String,
        walletAddress: String,
        network: Network,
        includeZeroBalance: Bool
    ) async -> DiscoveredToken? {

        do {
            // Verify it's an ERC20 token
            guard await verifyERC20(contractAddress: contractAddress, network: network) else {
                return nil
            }

            // Get token balance
            let balance = try await getTokenBalance(
                contractAddress: contractAddress,
                walletAddress: walletAddress,
                network: network
            )

            // Skip if zero balance and not including zero balances
            if !includeZeroBalance && (balance == "0" || balance.isEmpty) {
                return nil
            }

            // Fetch metadata
            let metadata = try await metadataService.getMetadata(
                contractAddress: contractAddress,
                chainId: network.chainId
            )

            // Skip if marked as spam
            if metadata.isSpam {
                return nil
            }

            return DiscoveredToken(
                metadata: metadata,
                balance: balance,
                discoveredAt: Date(),
                transactionCount: 0
            )

        } catch {
            // Silent failure for individual tokens
            return nil
        }
    }

    private func getTransactionHistory(
        address: String,
        network: Network
    ) async throws -> [Transaction] {

        // Use block explorer API if available
        if let explorerURL = network.explorerURL {
            return try await fetchFromExplorer(address: address, explorerURL: explorerURL)
        }

        // Fallback to scanning recent blocks (less efficient)
        return try await scanRecentBlocks(address: address, network: network)
    }

    private func fetchFromExplorer(
        address: String,
        explorerURL: String
    ) async throws -> [Transaction] {

        // This is a simplified version
        // In production, use proper block explorer APIs (Etherscan, BSCScan, etc.)

        var transactions: [Transaction] = []

        // For now, return empty array
        // TODO: Implement proper explorer API integration

        return transactions
    }

    private func scanRecentBlocks(
        address: String,
        network: Network,
        blockCount: Int = 1000
    ) async throws -> [Transaction] {

        // Get current block number
        let currentBlock = try await getCurrentBlockNumber(rpcURL: network.rpcURL)

        // Get logs for Transfer events involving this address
        let fromBlock = max(0, currentBlock - blockCount)

        let logs = try await getLogs(
            rpcURL: network.rpcURL,
            fromBlock: fromBlock,
            toBlock: currentBlock,
            address: address
        )

        return logs.map { log in
            Transaction(
                hash: log.transactionHash,
                from: log.address,
                to: address,
                value: "0",
                input: "",
                blockNumber: log.blockNumber,
                logs: [log]
            )
        }
    }

    private func extractTokenAddresses(
        from transactions: [Transaction],
        walletAddress: String
    ) async -> Set<String> {

        var tokenAddresses = Set<String>()

        for transaction in transactions {
            for log in transaction.logs {
                // Check if it's a Transfer event
                guard log.topics.first?.lowercased() == transferEventSignature.lowercased() else {
                    continue
                }

                // Check if this wallet is involved (sender or receiver)
                let addressInTopics = log.topics.contains { topic in
                    topic.lowercased().contains(walletAddress.replacingOccurrences(of: "0x", with: ""))
                }

                if addressInTopics {
                    tokenAddresses.insert(log.address.lowercased())
                }
            }
        }

        return tokenAddresses
    }

    // MARK: - RPC Methods

    private func callContract(
        rpcURL: String,
        contractAddress: String,
        data: String
    ) async throws -> String {

        guard let url = URL(string: rpcURL) else {
            throw TokenDiscoveryError.invalidAddress
        }

        let requestBody: [String: Any] = [
            "jsonrpc": "2.0",
            "method": "eth_call",
            "params": [
                [
                    "to": contractAddress,
                    "data": data
                ],
                "latest"
            ],
            "id": 1
        ]

        var request = URLRequest(url: url)
        request.httpMethod = "POST"
        request.setValue("application/json", forHTTPHeaderField: "Content-Type")
        request.httpBody = try JSONSerialization.data(withJSONObject: requestBody)

        let (responseData, response) = try await URLSession.shared.data(for: request)

        guard let httpResponse = response as? HTTPURLResponse, httpResponse.statusCode == 200 else {
            throw TokenDiscoveryError.invalidResponse
        }

        let json = try JSONSerialization.jsonObject(with: responseData) as? [String: Any]

        if let error = json?["error"] as? [String: Any],
           let message = error["message"] as? String {
            throw TokenDiscoveryError.rpcError(message)
        }

        guard let result = json?["result"] as? String else {
            throw TokenDiscoveryError.invalidResponse
        }

        return result
    }

    private func getCurrentBlockNumber(rpcURL: String) async throws -> Int {
        guard let url = URL(string: rpcURL) else {
            throw TokenDiscoveryError.invalidAddress
        }

        let requestBody: [String: Any] = [
            "jsonrpc": "2.0",
            "method": "eth_blockNumber",
            "params": [],
            "id": 1
        ]

        var request = URLRequest(url: url)
        request.httpMethod = "POST"
        request.setValue("application/json", forHTTPHeaderField: "Content-Type")
        request.httpBody = try JSONSerialization.data(withJSONObject: requestBody)

        let (data, response) = try await URLSession.shared.data(for: request)

        guard let httpResponse = response as? HTTPURLResponse, httpResponse.statusCode == 200 else {
            throw TokenDiscoveryError.invalidResponse
        }

        let json = try JSONSerialization.jsonObject(with: data) as? [String: Any]
        guard let result = json?["result"] as? String else {
            throw TokenDiscoveryError.invalidResponse
        }

        let hex = result.replacingOccurrences(of: "0x", with: "")
        return Int(hex, radix: 16) ?? 0
    }

    private func getLogs(
        rpcURL: String,
        fromBlock: Int,
        toBlock: Int,
        address: String
    ) async throws -> [TransactionLog] {

        guard let url = URL(string: rpcURL) else {
            throw TokenDiscoveryError.invalidAddress
        }

        let fromBlockHex = String(format: "0x%x", fromBlock)
        let toBlockHex = String(format: "0x%x", toBlock)

        // Remove 0x prefix and pad address
        let paddedAddress = address.lowercased().replacingOccurrences(of: "0x", with: "")
            .padding(toLength: 64, withPad: "0", startingAt: 0)

        let requestBody: [String: Any] = [
            "jsonrpc": "2.0",
            "method": "eth_getLogs",
            "params": [
                [
                    "fromBlock": fromBlockHex,
                    "toBlock": toBlockHex,
                    "topics": [
                        transferEventSignature,
                        nil,
                        "0x" + paddedAddress
                    ]
                ]
            ],
            "id": 1
        ]

        var request = URLRequest(url: url)
        request.httpMethod = "POST"
        request.setValue("application/json", forHTTPHeaderField: "Content-Type")
        request.httpBody = try JSONSerialization.data(withJSONObject: requestBody)

        let (data, response) = try await URLSession.shared.data(for: request)

        guard let httpResponse = response as? HTTPURLResponse, httpResponse.statusCode == 200 else {
            throw TokenDiscoveryError.invalidResponse
        }

        let json = try JSONSerialization.jsonObject(with: data) as? [String: Any]
        guard let result = json?["result"] as? [[String: Any]] else {
            throw TokenDiscoveryError.invalidResponse
        }

        return result.compactMap { logData in
            guard let address = logData["address"] as? String,
                  let topics = logData["topics"] as? [String],
                  let data = logData["data"] as? String,
                  let transactionHash = logData["transactionHash"] as? String,
                  let blockNumberHex = logData["blockNumber"] as? String,
                  let blockNumber = Int(blockNumberHex.replacingOccurrences(of: "0x", with: ""), radix: 16)
            else {
                return nil
            }

            return TransactionLog(
                address: address,
                topics: topics,
                data: data,
                blockNumber: blockNumber,
                transactionHash: transactionHash
            )
        }
    }
}

// MARK: - Supporting Types

struct Transaction: Codable, Sendable {
    let hash: String
    let from: String
    let to: String
    let value: String
    let input: String
    let blockNumber: Int
    let logs: [TransactionLog]
}

struct TransactionLog: Codable, Sendable {
    let address: String
    let topics: [String]
    let data: String
    let blockNumber: Int
    let transactionHash: String
}
