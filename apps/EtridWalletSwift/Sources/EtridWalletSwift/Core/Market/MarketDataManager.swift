//
//  MarketDataManager.swift
//  EtridWalletSwift
//
//  Created by Ëtrid Team
//  Copyright © 2025 Ëtrid. All rights reserved.
//

import Foundation
import Combine

/// Errors that can occur in market data operations
enum MarketDataError: Error, LocalizedError {
    case serviceFailed(String)
    case invalidData
    case calculationError
    case alertNotFound
    case watchlistFull

    var errorDescription: String? {
        switch self {
        case .serviceFailed(let message):
            return "Service failed: \(message)"
        case .invalidData:
            return "Invalid market data"
        case .calculationError:
            return "Error calculating portfolio value"
        case .alertNotFound:
            return "Price alert not found"
        case .watchlistFull:
            return "Watchlist is full (maximum 50 items)"
        }
    }
}

/// Account information for portfolio calculation
struct Account: Codable, Identifiable, Sendable {
    let id: UUID
    let address: String
    let name: String
    let chainId: Int

    init(id: UUID = UUID(), address: String, name: String, chainId: Int) {
        self.id = id
        self.address = address
        self.name = name
        self.chainId = chainId
    }
}

/// Token with balance information
struct Token: Codable, Identifiable, Sendable {
    let id: UUID
    let contractAddress: String
    let chainId: Int
    let symbol: String
    let name: String
    let decimals: Int
    let balance: String
    let coingeckoId: String?

    init(
        id: UUID = UUID(),
        contractAddress: String,
        chainId: Int,
        symbol: String,
        name: String,
        decimals: Int,
        balance: String,
        coingeckoId: String? = nil
    ) {
        self.id = id
        self.contractAddress = contractAddress
        self.chainId = chainId
        self.symbol = symbol
        self.name = name
        self.decimals = decimals
        self.balance = balance
        self.coingeckoId = coingeckoId
    }
}

/// Actor-based market data manager for coordinating price and market operations
actor MarketDataManager {

    // MARK: - Properties

    private let priceService: PriceService
    private let metadataService: TokenMetadataService
    private let discoveryService: TokenDiscoveryService

    // Watchlist storage
    private var watchlist: [WatchlistItem] = []
    private let maxWatchlistItems = 50

    // Price alerts storage
    private var priceAlerts: [PriceAlert] = []

    // Auto-refresh settings
    private var autoRefreshEnabled = true
    private var refreshInterval: TimeInterval = 30 // 30 seconds
    private var refreshTask: Task<Void, Never>?

    // Current currency preference
    private var preferredCurrency: String = "usd"

    // MARK: - Initialization

    init(
        priceService: PriceService,
        metadataService: TokenMetadataService,
        discoveryService: TokenDiscoveryService
    ) {
        self.priceService = priceService
        self.metadataService = metadataService
        self.discoveryService = discoveryService
    }

    // MARK: - Portfolio Management

    /// Calculate total portfolio value across all accounts and tokens
    func calculatePortfolioValue(
        accounts: [Account],
        tokens: [Token]
    ) async throws -> PortfolioSummary {

        guard !tokens.isEmpty else {
            return PortfolioSummary(tokens: [])
        }

        // Get unique CoinGecko IDs
        let tokenIds = Set(tokens.compactMap { $0.coingeckoId })

        // Fetch prices in batch
        let prices = try await priceService.getTokenPrices(
            ids: Array(tokenIds),
            currency: preferredCurrency
        )

        // Build token balances with price data
        var tokenBalances: [TokenBalance] = []

        for token in tokens {
            guard let coingeckoId = token.coingeckoId,
                  let priceData = prices[coingeckoId] else {
                continue
            }

            // Get metadata for logo and additional info
            let metadata = try? await metadataService.getMetadata(
                contractAddress: token.contractAddress,
                chainId: token.chainId
            )

            let tokenMetadata = metadata ?? TokenMetadata(
                contractAddress: token.contractAddress,
                chainId: token.chainId,
                name: token.name,
                symbol: token.symbol,
                decimals: token.decimals,
                coingeckoId: coingeckoId
            )

            let balance = TokenBalance(
                token: tokenMetadata,
                balance: token.balance,
                priceData: priceData
            )

            tokenBalances.append(balance)
        }

        return PortfolioSummary(tokens: tokenBalances)
    }

    /// Get token balances for a specific account
    func getAccountTokens(
        account: Account,
        includeZeroBalance: Bool = false
    ) async throws -> [TokenBalance] {

        let network = Network(
            chainId: account.chainId,
            name: "Chain \(account.chainId)",
            rpcURL: getRPCURL(for: account.chainId),
            explorerURL: nil,
            nativeCurrency: "ETH"
        )

        let discoveredTokens = try await discoveryService.discoverTokens(
            for: account.address,
            network: network,
            includeZeroBalance: includeZeroBalance
        )

        // Get prices for discovered tokens
        let tokenIds = discoveredTokens.compactMap { $0.metadata.coingeckoId }

        var prices: [String: PriceData] = [:]
        if !tokenIds.isEmpty {
            prices = try await priceService.getTokenPrices(ids: tokenIds)
        }

        // Map to TokenBalance
        return discoveredTokens.map { discovered in
            let priceData = discovered.metadata.coingeckoId.flatMap { prices[$0] }

            return TokenBalance(
                token: discovered.metadata,
                balance: discovered.balance,
                priceData: priceData
            )
        }
    }

    // MARK: - Watchlist Management

    /// Add token to watchlist
    func addToWatchlist(tokenId: String, symbol: String, name: String) throws {
        guard watchlist.count < maxWatchlistItems else {
            throw MarketDataError.watchlistFull
        }

        // Check if already in watchlist
        guard !watchlist.contains(where: { $0.tokenId == tokenId }) else {
            return
        }

        let item = WatchlistItem(
            tokenId: tokenId,
            symbol: symbol,
            name: name
        )

        watchlist.append(item)
    }

    /// Remove token from watchlist
    func removeFromWatchlist(tokenId: String) {
        watchlist.removeAll { $0.tokenId == tokenId }
    }

    /// Get all watchlist items with current prices
    func getWatchlist() async throws -> [(item: WatchlistItem, price: PriceData)] {
        let tokenIds = watchlist.map { $0.tokenId }

        guard !tokenIds.isEmpty else {
            return []
        }

        let prices = try await priceService.getTokenPrices(ids: tokenIds)

        return watchlist.compactMap { item in
            guard let price = prices[item.tokenId] else { return nil }
            return (item, price)
        }
    }

    /// Check if token is in watchlist
    func isInWatchlist(tokenId: String) -> Bool {
        return watchlist.contains { $0.tokenId == tokenId }
    }

    // MARK: - Price Alerts

    /// Create a new price alert
    func createPriceAlert(
        tokenId: String,
        tokenSymbol: String,
        targetPrice: Double,
        condition: PriceAlert.AlertCondition
    ) {
        let alert = PriceAlert(
            tokenId: tokenId,
            tokenSymbol: tokenSymbol,
            targetPrice: targetPrice,
            condition: condition
        )

        priceAlerts.append(alert)
    }

    /// Remove a price alert
    func removePriceAlert(id: UUID) throws {
        guard let index = priceAlerts.firstIndex(where: { $0.id == id }) else {
            throw MarketDataError.alertNotFound
        }

        priceAlerts.remove(at: index)
    }

    /// Toggle price alert enabled state
    func togglePriceAlert(id: UUID) throws {
        guard let index = priceAlerts.firstIndex(where: { $0.id == id }) else {
            throw MarketDataError.alertNotFound
        }

        let alert = priceAlerts[index]
        priceAlerts[index] = PriceAlert(
            id: alert.id,
            tokenId: alert.tokenId,
            tokenSymbol: alert.tokenSymbol,
            targetPrice: alert.targetPrice,
            condition: alert.condition,
            isEnabled: !alert.isEnabled,
            isTriggered: alert.isTriggered,
            createdAt: alert.createdAt,
            triggeredAt: alert.triggeredAt
        )
    }

    /// Get all price alerts
    func getPriceAlerts() -> [PriceAlert] {
        return priceAlerts
    }

    /// Check price alerts and return triggered ones
    func checkPriceAlerts() async throws -> [PriceAlert] {
        let tokenIds = Set(priceAlerts.map { $0.tokenId })

        guard !tokenIds.isEmpty else {
            return []
        }

        let prices = try await priceService.getTokenPrices(ids: Array(tokenIds))

        var triggeredAlerts: [PriceAlert] = []

        for (index, alert) in priceAlerts.enumerated() {
            guard let priceData = prices[alert.tokenId] else { continue }

            if alert.shouldTrigger(currentPrice: priceData.price) {
                // Mark as triggered
                let triggeredAlert = PriceAlert(
                    id: alert.id,
                    tokenId: alert.tokenId,
                    tokenSymbol: alert.tokenSymbol,
                    targetPrice: alert.targetPrice,
                    condition: alert.condition,
                    isEnabled: alert.isEnabled,
                    isTriggered: true,
                    createdAt: alert.createdAt,
                    triggeredAt: Date()
                )

                priceAlerts[index] = triggeredAlert
                triggeredAlerts.append(triggeredAlert)
            }
        }

        return triggeredAlerts
    }

    // MARK: - Market Data

    /// Get detailed market data for a token
    func getDetailedMarketData(tokenId: String) async throws -> (price: PriceData, chart: [ChartDataPoint]) {
        async let price = priceService.getDetailedMarketData(id: tokenId, currency: preferredCurrency)
        async let chart = priceService.getChartData(id: tokenId, days: 7, currency: preferredCurrency)

        return try await (price, chart)
    }

    /// Get chart data for a specific timeframe
    func getChartData(
        tokenId: String,
        timeframe: ChartData.ChartTimeframe
    ) async throws -> ChartData {

        let dataPoints = try await priceService.getChartData(
            id: tokenId,
            days: timeframe.days,
            currency: preferredCurrency
        )

        return ChartData(
            tokenId: tokenId,
            currency: preferredCurrency,
            dataPoints: dataPoints,
            timeframe: timeframe,
            lastUpdated: Date()
        )
    }

    /// Search for tokens
    func searchTokens(query: String) async throws -> [TokenSearchResult] {
        return try await priceService.searchTokens(query: query)
    }

    // MARK: - Settings

    /// Set preferred currency
    func setPreferredCurrency(_ currency: String) {
        self.preferredCurrency = currency
    }

    /// Get preferred currency
    func getPreferredCurrency() -> String {
        return preferredCurrency
    }

    /// Enable/disable auto-refresh
    func setAutoRefresh(enabled: Bool) {
        autoRefreshEnabled = enabled

        if enabled {
            startAutoRefresh()
        } else {
            stopAutoRefresh()
        }
    }

    /// Set refresh interval
    func setRefreshInterval(_ interval: TimeInterval) {
        refreshInterval = interval

        if autoRefreshEnabled {
            stopAutoRefresh()
            startAutoRefresh()
        }
    }

    // MARK: - Auto-Refresh

    private func startAutoRefresh() {
        refreshTask?.cancel()

        refreshTask = Task {
            while !Task.isCancelled {
                try? await Task.sleep(nanoseconds: UInt64(refreshInterval * 1_000_000_000))

                if !Task.isCancelled {
                    // Check price alerts
                    _ = try? await checkPriceAlerts()
                }
            }
        }
    }

    private func stopAutoRefresh() {
        refreshTask?.cancel()
        refreshTask = nil
    }

    // MARK: - Cleanup

    /// Clear all caches
    func clearCaches() async {
        await priceService.clearCache()
        await metadataService.clearCache()
    }

    /// Cleanup resources
    func cleanup() {
        stopAutoRefresh()
    }

    // MARK: - Private Helpers

    private func getRPCURL(for chainId: Int) -> String {
        switch chainId {
        case 1: return "https://eth.llamarpc.com"
        case 56: return "https://bsc-dataseed.binance.org"
        case 137: return "https://polygon-rpc.com"
        case 43114: return "https://api.avax.network/ext/bc/C/rpc"
        case 250: return "https://rpc.ftm.tools"
        case 42161: return "https://arb1.arbitrum.io/rpc"
        case 10: return "https://mainnet.optimism.io"
        default: return ""
        }
    }
}

// MARK: - Persistence Extension

extension MarketDataManager {

    /// Save watchlist to UserDefaults
    func saveWatchlist() {
        if let encoded = try? JSONEncoder().encode(watchlist) {
            UserDefaults.standard.set(encoded, forKey: "watchlist")
        }
    }

    /// Load watchlist from UserDefaults
    func loadWatchlist() {
        if let data = UserDefaults.standard.data(forKey: "watchlist"),
           let decoded = try? JSONDecoder().decode([WatchlistItem].self, from: data) {
            watchlist = decoded
        }
    }

    /// Save price alerts to UserDefaults
    func savePriceAlerts() {
        if let encoded = try? JSONEncoder().encode(priceAlerts) {
            UserDefaults.standard.set(encoded, forKey: "priceAlerts")
        }
    }

    /// Load price alerts from UserDefaults
    func loadPriceAlerts() {
        if let data = UserDefaults.standard.data(forKey: "priceAlerts"),
           let decoded = try? JSONDecoder().decode([PriceAlert].self, from: data) {
            priceAlerts = decoded
        }
    }
}
