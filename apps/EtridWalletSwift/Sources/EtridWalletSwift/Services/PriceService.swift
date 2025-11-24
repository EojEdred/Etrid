//
//  PriceService.swift
//  EtridWalletSwift
//
//  Created by Ëtrid Team
//  Copyright © 2025 Ëtrid. All rights reserved.
//

import Foundation

/// Errors that can occur during price fetching
enum PriceServiceError: Error, LocalizedError {
    case invalidURL
    case networkError(Error)
    case invalidResponse
    case rateLimited
    case apiKeyMissing
    case decodingError(Error)
    case cacheExpired
    case noDataAvailable

    var errorDescription: String? {
        switch self {
        case .invalidURL:
            return "Invalid API URL"
        case .networkError(let error):
            return "Network error: \(error.localizedDescription)"
        case .invalidResponse:
            return "Invalid response from server"
        case .rateLimited:
            return "API rate limit exceeded. Please try again later."
        case .apiKeyMissing:
            return "API key is missing or invalid"
        case .decodingError(let error):
            return "Failed to decode response: \(error.localizedDescription)"
        case .cacheExpired:
            return "Cached data has expired"
        case .noDataAvailable:
            return "No price data available for this token"
        }
    }
}

/// Actor-based price service for thread-safe price fetching from CoinGecko
actor PriceService {

    // MARK: - Properties

    private let baseURL = "https://api.coingecko.com/api/v3"
    private let proBaseURL = "https://pro-api.coingecko.com/api/v3"
    private var apiKey: String?
    private var isPro: Bool = false

    // Cache configuration
    private var priceCache: [String: CachedPrice] = [:]
    private var chartCache: [String: CachedChart] = [:]
    private let priceCacheTTL: TimeInterval = 60 // 1 minute
    private let chartCacheTTL: TimeInterval = 300 // 5 minutes

    // Rate limiting
    private var lastRequestTime: Date?
    private let minRequestInterval: TimeInterval = 1.0 // 1 second between requests
    private var requestCount: Int = 0
    private var requestCountResetTime: Date = Date()
    private let maxRequestsPerMinute: Int = 50

    // MARK: - Initialization

    init(apiKey: String? = nil) {
        self.apiKey = apiKey
        self.isPro = apiKey != nil
    }

    // MARK: - Cached Data Structures

    private struct CachedPrice {
        let data: PriceData
        let timestamp: Date

        func isValid(ttl: TimeInterval) -> Bool {
            return Date().timeIntervalSince(timestamp) < ttl
        }
    }

    private struct CachedChart {
        let data: [ChartDataPoint]
        let timestamp: Date

        func isValid(ttl: TimeInterval) -> Bool {
            return Date().timeIntervalSince(timestamp) < ttl
        }
    }

    // MARK: - Public Methods

    /// Configure API key
    func configure(apiKey: String) {
        self.apiKey = apiKey
        self.isPro = true
    }

    /// Fetch prices for multiple tokens
    /// - Parameters:
    ///   - ids: CoinGecko token IDs (e.g., ["bitcoin", "ethereum"])
    ///   - currency: Target currency (default: "usd")
    ///   - forceRefresh: Skip cache and fetch fresh data
    /// - Returns: Dictionary mapping token ID to price data
    func getTokenPrices(
        ids: [String],
        currency: String = "usd",
        forceRefresh: Bool = false
    ) async throws -> [String: PriceData] {

        // Check cache first unless force refresh
        if !forceRefresh {
            let cached = getCachedPrices(ids: ids)
            if cached.count == ids.count {
                return cached
            }
        }

        // Rate limiting
        try await enforceRateLimit()

        // Build request
        let idsString = ids.joined(separator: ",")
        let endpoint = "\(currentBaseURL)/simple/price"

        var components = URLComponents(string: endpoint)
        components?.queryItems = [
            URLQueryItem(name: "ids", value: idsString),
            URLQueryItem(name: "vs_currencies", value: currency),
            URLQueryItem(name: "include_24hr_change", value: "true"),
            URLQueryItem(name: "include_7d_change", value: "true"),
            URLQueryItem(name: "include_1h_change", value: "true"),
            URLQueryItem(name: "include_market_cap", value: "true"),
            URLQueryItem(name: "include_24hr_vol", value: "true"),
            URLQueryItem(name: "include_last_updated_at", value: "true")
        ]

        guard let url = components?.url else {
            throw PriceServiceError.invalidURL
        }

        var request = URLRequest(url: url)
        addAuthHeader(to: &request)

        // Make request
        let (data, response) = try await URLSession.shared.data(for: request)

        guard let httpResponse = response as? HTTPURLResponse else {
            throw PriceServiceError.invalidResponse
        }

        if httpResponse.statusCode == 429 {
            throw PriceServiceError.rateLimited
        }

        guard httpResponse.statusCode == 200 else {
            throw PriceServiceError.invalidResponse
        }

        // Parse response
        let decoder = JSONDecoder()
        decoder.dateDecodingStrategy = .secondsSince1970

        do {
            let response = try decoder.decode([String: CoinGeckoPriceResponse].self, from: data)

            var prices: [String: PriceData] = [:]
            let now = Date()

            for (id, priceResponse) in response {
                let priceData = PriceData(
                    price: priceResponse.price(for: currency),
                    change24h: priceResponse.change24h ?? 0,
                    change7d: priceResponse.change7d,
                    change1h: priceResponse.change1h,
                    marketCap: priceResponse.marketCap(for: currency),
                    volume24h: priceResponse.volume24h(for: currency),
                    circulatingSupply: nil,
                    totalSupply: nil,
                    rank: nil,
                    lastUpdated: Date(timeIntervalSince1970: priceResponse.lastUpdated ?? now.timeIntervalSince1970)
                )

                prices[id] = priceData

                // Cache the result
                priceCache[id] = CachedPrice(data: priceData, timestamp: now)
            }

            return prices

        } catch {
            throw PriceServiceError.decodingError(error)
        }
    }

    /// Fetch detailed market data for a single token
    func getDetailedMarketData(id: String, currency: String = "usd") async throws -> PriceData {
        try await enforceRateLimit()

        let endpoint = "\(currentBaseURL)/coins/\(id)"

        var components = URLComponents(string: endpoint)
        components?.queryItems = [
            URLQueryItem(name: "localization", value: "false"),
            URLQueryItem(name: "tickers", value: "false"),
            URLQueryItem(name: "market_data", value: "true"),
            URLQueryItem(name: "community_data", value: "false"),
            URLQueryItem(name: "developer_data", value: "false")
        ]

        guard let url = components?.url else {
            throw PriceServiceError.invalidURL
        }

        var request = URLRequest(url: url)
        addAuthHeader(to: &request)

        let (data, response) = try await URLSession.shared.data(for: request)

        guard let httpResponse = response as? HTTPURLResponse, httpResponse.statusCode == 200 else {
            throw PriceServiceError.invalidResponse
        }

        let decoder = JSONDecoder()
        decoder.dateDecodingStrategy = .iso8601

        do {
            let coinData = try decoder.decode(CoinGeckoDetailedResponse.self, from: data)

            guard let marketData = coinData.marketData else {
                throw PriceServiceError.noDataAvailable
            }

            let priceData = PriceData(
                price: marketData.currentPrice[currency] ?? 0,
                change24h: marketData.priceChangePercentage24h ?? 0,
                change7d: marketData.priceChangePercentage7d,
                change1h: marketData.priceChangePercentage1h,
                marketCap: marketData.marketCap[currency],
                volume24h: marketData.totalVolume[currency],
                circulatingSupply: marketData.circulatingSupply,
                totalSupply: marketData.totalSupply,
                rank: coinData.marketCapRank,
                lastUpdated: Date()
            )

            return priceData

        } catch {
            throw PriceServiceError.decodingError(error)
        }
    }

    /// Fetch chart data for a token
    /// - Parameters:
    ///   - id: CoinGecko token ID
    ///   - days: Number of days of historical data
    ///   - currency: Target currency
    /// - Returns: Array of chart data points
    func getChartData(
        id: String,
        days: Int,
        currency: String = "usd",
        forceRefresh: Bool = false
    ) async throws -> [ChartDataPoint] {

        let cacheKey = "\(id)_\(days)_\(currency)"

        // Check cache
        if !forceRefresh, let cached = chartCache[cacheKey], cached.isValid(ttl: chartCacheTTL) {
            return cached.data
        }

        try await enforceRateLimit()

        let endpoint = "\(currentBaseURL)/coins/\(id)/market_chart"

        var components = URLComponents(string: endpoint)
        components?.queryItems = [
            URLQueryItem(name: "vs_currency", value: currency),
            URLQueryItem(name: "days", value: "\(days)"),
            URLQueryItem(name: "interval", value: days <= 1 ? "hourly" : "daily")
        ]

        guard let url = components?.url else {
            throw PriceServiceError.invalidURL
        }

        var request = URLRequest(url: url)
        addAuthHeader(to: &request)

        let (data, response) = try await URLSession.shared.data(for: request)

        guard let httpResponse = response as? HTTPURLResponse, httpResponse.statusCode == 200 else {
            throw PriceServiceError.invalidResponse
        }

        do {
            let chartResponse = try JSONDecoder().decode(CoinGeckoChartResponse.self, from: data)

            let dataPoints = chartResponse.prices.map { pricePoint in
                ChartDataPoint(
                    timestamp: Date(timeIntervalSince1970: pricePoint[0] / 1000),
                    price: pricePoint[1],
                    volume: nil
                )
            }

            // Cache the result
            chartCache[cacheKey] = CachedChart(data: dataPoints, timestamp: Date())

            return dataPoints

        } catch {
            throw PriceServiceError.decodingError(error)
        }
    }

    /// Search for tokens by query
    func searchTokens(query: String) async throws -> [TokenSearchResult] {
        try await enforceRateLimit()

        let endpoint = "\(currentBaseURL)/search"

        var components = URLComponents(string: endpoint)
        components?.queryItems = [
            URLQueryItem(name: "query", value: query)
        ]

        guard let url = components?.url else {
            throw PriceServiceError.invalidURL
        }

        var request = URLRequest(url: url)
        addAuthHeader(to: &request)

        let (data, response) = try await URLSession.shared.data(for: request)

        guard let httpResponse = response as? HTTPURLResponse, httpResponse.statusCode == 200 else {
            throw PriceServiceError.invalidResponse
        }

        let decoder = JSONDecoder()
        let searchResponse = try decoder.decode(CoinGeckoSearchResponse.self, from: data)

        return searchResponse.coins.map { coin in
            TokenSearchResult(
                id: coin.id,
                symbol: coin.symbol,
                name: coin.name,
                thumb: coin.thumb,
                marketCapRank: coin.marketCapRank
            )
        }
    }

    /// Clear all cached data
    func clearCache() {
        priceCache.removeAll()
        chartCache.removeAll()
    }

    // MARK: - Private Methods

    private var currentBaseURL: String {
        isPro ? proBaseURL : baseURL
    }

    private func addAuthHeader(to request: inout URLRequest) {
        if let apiKey = apiKey {
            request.setValue(apiKey, forHTTPHeaderField: "x-cg-pro-api-key")
        }
    }

    private func getCachedPrices(ids: [String]) -> [String: PriceData] {
        var cached: [String: PriceData] = [:]

        for id in ids {
            if let cachedPrice = priceCache[id], cachedPrice.isValid(ttl: priceCacheTTL) {
                cached[id] = cachedPrice.data
            }
        }

        return cached
    }

    private func enforceRateLimit() async throws {
        let now = Date()

        // Reset request count every minute
        if now.timeIntervalSince(requestCountResetTime) > 60 {
            requestCount = 0
            requestCountResetTime = now
        }

        // Check if we've hit the rate limit
        if requestCount >= maxRequestsPerMinute {
            throw PriceServiceError.rateLimited
        }

        // Enforce minimum interval between requests
        if let lastRequest = lastRequestTime {
            let timeSinceLastRequest = now.timeIntervalSince(lastRequest)
            if timeSinceLastRequest < minRequestInterval {
                let delay = minRequestInterval - timeSinceLastRequest
                try await Task.sleep(nanoseconds: UInt64(delay * 1_000_000_000))
            }
        }

        lastRequestTime = Date()
        requestCount += 1
    }
}

// MARK: - Response Models

private struct CoinGeckoPriceResponse: Codable {
    let usd: Double?
    let eur: Double?
    let gbp: Double?
    let usdMarketCap: Double?
    let eurMarketCap: Double?
    let gbpMarketCap: Double?
    let usd24hVol: Double?
    let eur24hVol: Double?
    let gbp24hVol: Double?
    let usd24hChange: Double?
    let eur24hChange: Double?
    let gbp24hChange: Double?
    let usd7dChange: Double?
    let eur7dChange: Double?
    let gbp7dChange: Double?
    let usd1hChange: Double?
    let eur1hChange: Double?
    let gbp1hChange: Double?
    let lastUpdated: Double?

    enum CodingKeys: String, CodingKey {
        case usd, eur, gbp
        case usdMarketCap = "usd_market_cap"
        case eurMarketCap = "eur_market_cap"
        case gbpMarketCap = "gbp_market_cap"
        case usd24hVol = "usd_24h_vol"
        case eur24hVol = "eur_24h_vol"
        case gbp24hVol = "gbp_24h_vol"
        case usd24hChange = "usd_24h_change"
        case eur24hChange = "eur_24h_change"
        case gbp24hChange = "gbp_24h_change"
        case usd7dChange = "usd_7d_change"
        case eur7dChange = "eur_7d_change"
        case gbp7dChange = "gbp_7d_change"
        case usd1hChange = "usd_1h_change"
        case eur1hChange = "eur_1h_change"
        case gbp1hChange = "gbp_1h_change"
        case lastUpdated = "last_updated_at"
    }

    func price(for currency: String) -> Double {
        switch currency.lowercased() {
        case "usd": return usd ?? 0
        case "eur": return eur ?? 0
        case "gbp": return gbp ?? 0
        default: return 0
        }
    }

    func marketCap(for currency: String) -> Double? {
        switch currency.lowercased() {
        case "usd": return usdMarketCap
        case "eur": return eurMarketCap
        case "gbp": return gbpMarketCap
        default: return nil
        }
    }

    func volume24h(for currency: String) -> Double? {
        switch currency.lowercased() {
        case "usd": return usd24hVol
        case "eur": return eur24hVol
        case "gbp": return gbp24hVol
        default: return nil
        }
    }

    var change24h: Double? {
        usd24hChange
    }

    var change7d: Double? {
        usd7dChange
    }

    var change1h: Double? {
        usd1hChange
    }
}

private struct CoinGeckoDetailedResponse: Codable {
    let id: String
    let symbol: String
    let name: String
    let marketCapRank: Int?
    let marketData: MarketData?

    struct MarketData: Codable {
        let currentPrice: [String: Double]
        let marketCap: [String: Double]
        let totalVolume: [String: Double]
        let circulatingSupply: Double?
        let totalSupply: Double?
        let priceChangePercentage1h: Double?
        let priceChangePercentage24h: Double?
        let priceChangePercentage7d: Double?

        enum CodingKeys: String, CodingKey {
            case currentPrice = "current_price"
            case marketCap = "market_cap"
            case totalVolume = "total_volume"
            case circulatingSupply = "circulating_supply"
            case totalSupply = "total_supply"
            case priceChangePercentage1h = "price_change_percentage_1h_in_currency"
            case priceChangePercentage24h = "price_change_percentage_24h"
            case priceChangePercentage7d = "price_change_percentage_7d"
        }
    }

    enum CodingKeys: String, CodingKey {
        case id, symbol, name
        case marketCapRank = "market_cap_rank"
        case marketData = "market_data"
    }
}

private struct CoinGeckoChartResponse: Codable {
    let prices: [[Double]]
}

private struct CoinGeckoSearchResponse: Codable {
    let coins: [CoinSearchItem]

    struct CoinSearchItem: Codable {
        let id: String
        let symbol: String
        let name: String
        let thumb: String
        let marketCapRank: Int?

        enum CodingKeys: String, CodingKey {
            case id, symbol, name, thumb
            case marketCapRank = "market_cap_rank"
        }
    }
}

struct TokenSearchResult: Codable, Identifiable, Sendable {
    let id: String
    let symbol: String
    let name: String
    let thumb: String
    let marketCapRank: Int?
}
