//
//  TokenMetadataService.swift
//  EtridWalletSwift
//
//  Created by Ëtrid Team
//  Copyright © 2025 Ëtrid. All rights reserved.
//

import Foundation

/// Errors that can occur during token metadata fetching
enum TokenMetadataError: Error, LocalizedError {
    case invalidAddress
    case networkError(Error)
    case invalidResponse
    case decodingError(Error)
    case notERC20Compliant
    case noMetadataFound
    case unsupportedChain

    var errorDescription: String? {
        switch self {
        case .invalidAddress:
            return "Invalid token contract address"
        case .networkError(let error):
            return "Network error: \(error.localizedDescription)"
        case .invalidResponse:
            return "Invalid response from server"
        case .decodingError(let error):
            return "Failed to decode response: \(error.localizedDescription)"
        case .notERC20Compliant:
            return "Token is not ERC20 compliant"
        case .noMetadataFound:
            return "No metadata found for this token"
        case .unsupportedChain:
            return "Chain not supported for metadata lookup"
        }
    }
}

/// Actor-based token metadata service for fetching token information from multiple sources
actor TokenMetadataService {

    // MARK: - Properties

    private let cache: NSCache<NSString, CachedMetadata>
    private let trustWalletBaseURL = "https://raw.githubusercontent.com/trustwallet/assets/master/blockchains"

    // MARK: - Initialization

    init() {
        self.cache = NSCache<NSString, CachedMetadata>()
        self.cache.countLimit = 1000 // Store up to 1000 token metadata entries
    }

    // MARK: - Cached Metadata

    private class CachedMetadata {
        let metadata: TokenMetadata
        let timestamp: Date

        init(metadata: TokenMetadata, timestamp: Date = Date()) {
            self.metadata = metadata
            self.timestamp = timestamp
        }

        func isValid(ttl: TimeInterval = 3600) -> Bool { // 1 hour default TTL
            return Date().timeIntervalSince(timestamp) < ttl
        }
    }

    // MARK: - Public Methods

    /// Get token metadata from multiple sources
    /// - Parameters:
    ///   - contractAddress: Token contract address
    ///   - chainId: Chain ID (1 = Ethereum, 56 = BSC, etc.)
    ///   - forceRefresh: Skip cache and fetch fresh data
    /// - Returns: Token metadata
    func getMetadata(
        contractAddress: String,
        chainId: Int,
        forceRefresh: Bool = false
    ) async throws -> TokenMetadata {

        let normalizedAddress = contractAddress.lowercased()
        let cacheKey = "\(chainId)_\(normalizedAddress)" as NSString

        // Check cache first
        if !forceRefresh, let cached = cache.object(forKey: cacheKey), cached.isValid() {
            return cached.metadata
        }

        // Fetch from multiple sources in parallel
        async let trustWalletData = fetchFromTrustWallet(address: normalizedAddress, chainId: chainId)
        async let onChainData = fetchFromContract(address: normalizedAddress, chainId: chainId)
        async let coingeckoData = fetchFromCoinGecko(address: normalizedAddress, chainId: chainId)

        // Merge data from all sources
        let metadata = try await mergeMetadata(
            address: normalizedAddress,
            chainId: chainId,
            trustWallet: try? trustWalletData,
            onChain: try? onChainData,
            coingecko: try? coingeckoData
        )

        // Cache the result
        let cached = CachedMetadata(metadata: metadata)
        cache.setObject(cached, forKey: cacheKey)

        return metadata
    }

    /// Batch fetch metadata for multiple tokens
    func getMetadataBatch(
        tokens: [(address: String, chainId: Int)]
    ) async throws -> [String: TokenMetadata] {

        var results: [String: TokenMetadata] = [:]

        // Process in batches to avoid overwhelming the system
        let batchSize = 10
        let batches = tokens.chunked(into: batchSize)

        for batch in batches {
            await withTaskGroup(of: (String, TokenMetadata?).self) { group in
                for (address, chainId) in batch {
                    group.addTask {
                        let metadata = try? await self.getMetadata(
                            contractAddress: address,
                            chainId: chainId
                        )
                        return (address.lowercased(), metadata)
                    }
                }

                for await (address, metadata) in group {
                    if let metadata = metadata {
                        results[address] = metadata
                    }
                }
            }
        }

        return results
    }

    /// Verify if a contract is ERC20 compliant
    func isERC20Compliant(contractAddress: String, chainId: Int) async -> Bool {
        do {
            // Try to fetch basic ERC20 functions
            let _ = try await fetchFromContract(address: contractAddress.lowercased(), chainId: chainId)
            return true
        } catch {
            return false
        }
    }

    /// Clear cache for a specific token or all tokens
    func clearCache(contractAddress: String? = nil, chainId: Int? = nil) {
        if let address = contractAddress, let chain = chainId {
            let cacheKey = "\(chain)_\(address.lowercased())" as NSString
            cache.removeObject(forKey: cacheKey)
        } else {
            cache.removeAllObjects()
        }
    }

    // MARK: - Private Methods - Trust Wallet Assets

    private func fetchFromTrustWallet(
        address: String,
        chainId: Int
    ) async throws -> TrustWalletMetadata {

        guard let chainName = getChainName(for: chainId) else {
            throw TokenMetadataError.unsupportedChain
        }

        let checksumAddress = toChecksumAddress(address)
        let infoURL = "\(trustWalletBaseURL)/\(chainName)/assets/\(checksumAddress)/info.json"

        guard let url = URL(string: infoURL) else {
            throw TokenMetadataError.invalidAddress
        }

        let (data, response) = try await URLSession.shared.data(from: url)

        guard let httpResponse = response as? HTTPURLResponse, httpResponse.statusCode == 200 else {
            throw TokenMetadataError.invalidResponse
        }

        let decoder = JSONDecoder()
        return try decoder.decode(TrustWalletMetadata.self, from: data)
    }

    // MARK: - Private Methods - On-Chain Data

    private func fetchFromContract(
        address: String,
        chainId: Int
    ) async throws -> OnChainMetadata {

        // Get RPC endpoint for the chain
        guard let rpcURL = getRPCEndpoint(for: chainId) else {
            throw TokenMetadataError.unsupportedChain
        }

        // Fetch name, symbol, and decimals in parallel
        async let name = callContract(
            rpcURL: rpcURL,
            contractAddress: address,
            functionSignature: "0x06fdde03" // name()
        )

        async let symbol = callContract(
            rpcURL: rpcURL,
            contractAddress: address,
            functionSignature: "0x95d89b41" // symbol()
        )

        async let decimals = callContract(
            rpcURL: rpcURL,
            contractAddress: address,
            functionSignature: "0x313ce567" // decimals()
        )

        let nameResult = try await name
        let symbolResult = try await symbol
        let decimalsResult = try await decimals

        return OnChainMetadata(
            name: decodeString(nameResult),
            symbol: decodeString(symbolResult),
            decimals: decodeUInt8(decimalsResult)
        )
    }

    private func callContract(
        rpcURL: String,
        contractAddress: String,
        functionSignature: String
    ) async throws -> String {

        guard let url = URL(string: rpcURL) else {
            throw TokenMetadataError.invalidAddress
        }

        let requestBody: [String: Any] = [
            "jsonrpc": "2.0",
            "method": "eth_call",
            "params": [
                [
                    "to": contractAddress,
                    "data": functionSignature
                ],
                "latest"
            ],
            "id": 1
        ]

        var request = URLRequest(url: url)
        request.httpMethod = "POST"
        request.setValue("application/json", forHTTPHeaderField: "Content-Type")
        request.httpBody = try JSONSerialization.data(withJSONObject: requestBody)

        let (data, response) = try await URLSession.shared.data(for: request)

        guard let httpResponse = response as? HTTPURLResponse, httpResponse.statusCode == 200 else {
            throw TokenMetadataError.invalidResponse
        }

        let json = try JSONSerialization.jsonObject(with: data) as? [String: Any]
        guard let result = json?["result"] as? String else {
            throw TokenMetadataError.invalidResponse
        }

        return result
    }

    // MARK: - Private Methods - CoinGecko

    private func fetchFromCoinGecko(
        address: String,
        chainId: Int
    ) async throws -> CoinGeckoMetadata {

        guard let platform = getCoinGeckoPlatform(for: chainId) else {
            throw TokenMetadataError.unsupportedChain
        }

        let endpoint = "https://api.coingecko.com/api/v3/coins/\(platform)/contract/\(address)"

        guard let url = URL(string: endpoint) else {
            throw TokenMetadataError.invalidAddress
        }

        let (data, response) = try await URLSession.shared.data(from: url)

        guard let httpResponse = response as? HTTPURLResponse, httpResponse.statusCode == 200 else {
            throw TokenMetadataError.invalidResponse
        }

        let decoder = JSONDecoder()
        decoder.keyDecodingStrategy = .convertFromSnakeCase
        return try decoder.decode(CoinGeckoMetadata.self, from: data)
    }

    // MARK: - Private Methods - Data Merging

    private func mergeMetadata(
        address: String,
        chainId: Int,
        trustWallet: TrustWalletMetadata?,
        onChain: OnChainMetadata?,
        coingecko: CoinGeckoMetadata?
    ) throws -> TokenMetadata {

        // Priority: TrustWallet > CoinGecko > OnChain for most fields
        // OnChain is most reliable for name, symbol, decimals

        guard let onChain = onChain else {
            throw TokenMetadataError.notERC20Compliant
        }

        let name = trustWallet?.name ?? coingecko?.name ?? onChain.name
        let symbol = trustWallet?.symbol ?? coingecko?.symbol ?? onChain.symbol
        let decimals = onChain.decimals // Always use on-chain decimals

        // Get logo URL
        var logoURL: String?
        if let trustWalletLogo = trustWallet?.logoURI {
            logoURL = trustWalletLogo
        } else if let coingeckoLogo = coingecko?.image?.large {
            logoURL = coingeckoLogo
        } else if let chainName = getChainName(for: chainId) {
            let checksumAddress = toChecksumAddress(address)
            logoURL = "\(trustWalletBaseURL)/\(chainName)/assets/\(checksumAddress)/logo.png"
        }

        return TokenMetadata(
            contractAddress: address,
            chainId: chainId,
            name: name,
            symbol: symbol,
            decimals: decimals,
            logoURL: logoURL,
            coingeckoId: coingecko?.id,
            description: coingecko?.description?.en,
            website: coingecko?.links?.homepage?.first,
            twitter: coingecko?.links?.twitterScreenName,
            telegram: coingecko?.links?.telegramChannelIdentifier,
            discord: nil,
            verified: trustWallet != nil || coingecko != nil,
            isSpam: false,
            tags: coingecko?.categories ?? [],
            createdAt: Date()
        )
    }

    // MARK: - Private Methods - Helpers

    private func decodeString(_ hexString: String) -> String {
        guard hexString.hasPrefix("0x") else { return "" }

        let hex = String(hexString.dropFirst(2))
        guard hex.count >= 128 else { return "" }

        // Skip offset (first 32 bytes) and length (next 32 bytes)
        let lengthHex = String(hex.dropFirst(64).prefix(64))
        guard let length = Int(lengthHex, radix: 16) else { return "" }

        // Get the actual string data
        let dataHex = String(hex.dropFirst(128).prefix(length * 2))

        var result = ""
        var index = dataHex.startIndex
        while index < dataHex.endIndex {
            let nextIndex = dataHex.index(index, offsetBy: 2)
            let byteString = String(dataHex[index..<nextIndex])
            if let byte = UInt8(byteString, radix: 16),
               byte != 0 {
                result.append(Character(UnicodeScalar(byte)))
            }
            index = nextIndex
        }

        return result
    }

    private func decodeUInt8(_ hexString: String) -> Int {
        guard hexString.hasPrefix("0x") else { return 18 }

        let hex = String(hexString.dropFirst(2))
        return Int(hex, radix: 16) ?? 18
    }

    private func toChecksumAddress(_ address: String) -> String {
        // Simplified checksum address conversion
        // In production, use proper EIP-55 checksumming
        let lowercased = address.lowercased().replacingOccurrences(of: "0x", with: "")
        var result = "0x"

        for (index, char) in lowercased.enumerated() {
            if let _ = Int(String(char), radix: 16), index % 2 == 0 {
                result.append(char.uppercased())
            } else {
                result.append(char)
            }
        }

        return result
    }

    private func getChainName(for chainId: Int) -> String? {
        switch chainId {
        case 1: return "ethereum"
        case 56: return "smartchain"
        case 137: return "polygon"
        case 43114: return "avalanchec"
        case 250: return "fantom"
        case 42161: return "arbitrum"
        case 10: return "optimism"
        default: return nil
        }
    }

    private func getRPCEndpoint(for chainId: Int) -> String? {
        switch chainId {
        case 1: return "https://eth.llamarpc.com"
        case 56: return "https://bsc-dataseed.binance.org"
        case 137: return "https://polygon-rpc.com"
        case 43114: return "https://api.avax.network/ext/bc/C/rpc"
        case 250: return "https://rpc.ftm.tools"
        case 42161: return "https://arb1.arbitrum.io/rpc"
        case 10: return "https://mainnet.optimism.io"
        default: return nil
        }
    }

    private func getCoinGeckoPlatform(for chainId: Int) -> String? {
        switch chainId {
        case 1: return "ethereum"
        case 56: return "binance-smart-chain"
        case 137: return "polygon-pos"
        case 43114: return "avalanche"
        case 250: return "fantom"
        case 42161: return "arbitrum-one"
        case 10: return "optimistic-ethereum"
        default: return nil
        }
    }
}

// MARK: - Response Models

private struct TrustWalletMetadata: Codable {
    let name: String
    let symbol: String
    let type: String
    let decimals: Int
    let description: String?
    let website: String?
    let explorer: String?
    let status: String?
    let id: String?
    let logoURI: String?

    enum CodingKeys: String, CodingKey {
        case name, symbol, type, decimals, description, website, explorer, status, id
        case logoURI = "logoURI"
    }
}

private struct OnChainMetadata {
    let name: String
    let symbol: String
    let decimals: Int
}

private struct CoinGeckoMetadata: Codable {
    let id: String
    let symbol: String
    let name: String
    let description: Description?
    let image: Image?
    let links: Links?
    let categories: [String]?

    struct Description: Codable {
        let en: String?
    }

    struct Image: Codable {
        let thumb: String?
        let small: String?
        let large: String?
    }

    struct Links: Codable {
        let homepage: [String]?
        let twitterScreenName: String?
        let telegramChannelIdentifier: String?

        enum CodingKeys: String, CodingKey {
            case homepage
            case twitterScreenName = "twitter_screen_name"
            case telegramChannelIdentifier = "telegram_channel_identifier"
        }
    }
}

// MARK: - Array Extension

private extension Array {
    func chunked(into size: Int) -> [[Element]] {
        return stride(from: 0, to: count, by: size).map {
            Array(self[$0 ..< Swift.min($0 + size, count)])
        }
    }
}
