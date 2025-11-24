//
//  MarketModels.swift
//  EtridWalletSwift
//
//  Created by Ëtrid Team
//  Copyright © 2025 Ëtrid. All rights reserved.
//

import Foundation

// MARK: - Price Data

/// Real-time price information for a token
struct PriceData: Codable, Sendable {
    let price: Double
    let change24h: Double
    let change7d: Double?
    let change1h: Double?
    let marketCap: Double?
    let volume24h: Double?
    let circulatingSupply: Double?
    let totalSupply: Double?
    let rank: Int?
    let lastUpdated: Date

    enum CodingKeys: String, CodingKey {
        case price
        case change24h = "price_change_percentage_24h"
        case change7d = "price_change_percentage_7d"
        case change1h = "price_change_percentage_1h"
        case marketCap = "market_cap"
        case volume24h = "total_volume"
        case circulatingSupply = "circulating_supply"
        case totalSupply = "total_supply"
        case rank = "market_cap_rank"
        case lastUpdated = "last_updated"
    }

    /// Formatted price string with currency symbol
    func formattedPrice(currency: String = "USD") -> String {
        let formatter = NumberFormatter()
        formatter.numberStyle = .currency
        formatter.currencyCode = currency
        formatter.maximumFractionDigits = price < 1 ? 6 : 2
        return formatter.string(from: NSNumber(value: price)) ?? "$0.00"
    }

    /// Formatted 24h change percentage
    var formattedChange24h: String {
        let sign = change24h >= 0 ? "+" : ""
        return String(format: "%@%.2f%%", sign, change24h)
    }

    /// Whether the price is trending up
    var isPositive: Bool {
        return change24h >= 0
    }
}

// MARK: - Token Metadata

/// Comprehensive token metadata from multiple sources
struct TokenMetadata: Codable, Sendable {
    let contractAddress: String
    let chainId: Int
    let name: String
    let symbol: String
    let decimals: Int
    let logoURL: String?
    let coingeckoId: String?
    let description: String?
    let website: String?
    let twitter: String?
    let telegram: String?
    let discord: String?
    let verified: Bool
    let isSpam: Bool
    let tags: [String]
    let createdAt: Date

    enum CodingKeys: String, CodingKey {
        case contractAddress = "contract_address"
        case chainId = "chain_id"
        case name, symbol, decimals
        case logoURL = "logo_url"
        case coingeckoId = "coingecko_id"
        case description, website, twitter, telegram, discord
        case verified
        case isSpam = "is_spam"
        case tags
        case createdAt = "created_at"
    }

    init(
        contractAddress: String,
        chainId: Int,
        name: String,
        symbol: String,
        decimals: Int,
        logoURL: String? = nil,
        coingeckoId: String? = nil,
        description: String? = nil,
        website: String? = nil,
        twitter: String? = nil,
        telegram: String? = nil,
        discord: String? = nil,
        verified: Bool = false,
        isSpam: Bool = false,
        tags: [String] = [],
        createdAt: Date = Date()
    ) {
        self.contractAddress = contractAddress
        self.chainId = chainId
        self.name = name
        self.symbol = symbol
        self.decimals = decimals
        self.logoURL = logoURL
        self.coingeckoId = coingeckoId
        self.description = description
        self.website = website
        self.twitter = twitter
        self.telegram = telegram
        self.discord = discord
        self.verified = verified
        self.isSpam = isSpam
        self.tags = tags
        self.createdAt = createdAt
    }
}

// MARK: - Chart Data

/// Single data point for price charts
struct ChartDataPoint: Codable, Identifiable, Sendable {
    let id: UUID
    let timestamp: Date
    let price: Double
    let volume: Double?

    init(timestamp: Date, price: Double, volume: Double? = nil) {
        self.id = UUID()
        self.timestamp = timestamp
        self.price = price
        self.volume = volume
    }

    enum CodingKeys: String, CodingKey {
        case timestamp, price, volume
    }

    init(from decoder: Decoder) throws {
        let container = try decoder.container(keyedBy: CodingKeys.self)
        self.id = UUID()
        self.timestamp = try container.decode(Date.self, forKey: .timestamp)
        self.price = try container.decode(Double.self, forKey: .price)
        self.volume = try container.decodeIfPresent(Double.self, forKey: .volume)
    }
}

/// Complete chart data with multiple timeframes
struct ChartData: Codable, Sendable {
    let tokenId: String
    let currency: String
    let dataPoints: [ChartDataPoint]
    let timeframe: ChartTimeframe
    let lastUpdated: Date

    enum ChartTimeframe: String, Codable, CaseIterable, Sendable {
        case oneHour = "1h"
        case oneDay = "24h"
        case sevenDays = "7d"
        case thirtyDays = "30d"
        case ninetyDays = "90d"
        case oneYear = "1y"
        case all = "all"

        var days: Int {
            switch self {
            case .oneHour: return 0
            case .oneDay: return 1
            case .sevenDays: return 7
            case .thirtyDays: return 30
            case .ninetyDays: return 90
            case .oneYear: return 365
            case .all: return 999999
            }
        }

        var displayName: String {
            switch self {
            case .oneHour: return "1H"
            case .oneDay: return "1D"
            case .sevenDays: return "7D"
            case .thirtyDays: return "1M"
            case .ninetyDays: return "3M"
            case .oneYear: return "1Y"
            case .all: return "All"
            }
        }
    }
}

// MARK: - Market Stats

/// Aggregated market statistics
struct MarketStats: Codable, Sendable {
    let totalMarketCap: Double
    let total24hVolume: Double
    let btcDominance: Double
    let ethDominance: Double
    let marketCapChangePercentage24h: Double
    let activeCryptocurrencies: Int
    let lastUpdated: Date

    enum CodingKeys: String, CodingKey {
        case totalMarketCap = "total_market_cap"
        case total24hVolume = "total_volume"
        case btcDominance = "market_cap_percentage_btc"
        case ethDominance = "market_cap_percentage_eth"
        case marketCapChangePercentage24h = "market_cap_change_percentage_24h"
        case activeCryptocurrencies = "active_cryptocurrencies"
        case lastUpdated = "last_updated"
    }
}

// MARK: - Price Alert

/// User-configured price alert
struct PriceAlert: Codable, Identifiable, Sendable {
    let id: UUID
    let tokenId: String
    let tokenSymbol: String
    let targetPrice: Double
    let condition: AlertCondition
    let isEnabled: Bool
    let isTriggered: Bool
    let createdAt: Date
    let triggeredAt: Date?

    enum AlertCondition: String, Codable, Sendable {
        case above = "above"
        case below = "below"

        func isMet(currentPrice: Double, targetPrice: Double) -> Bool {
            switch self {
            case .above: return currentPrice >= targetPrice
            case .below: return currentPrice <= targetPrice
            }
        }

        var symbol: String {
            switch self {
            case .above: return "≥"
            case .below: return "≤"
            }
        }
    }

    init(
        id: UUID = UUID(),
        tokenId: String,
        tokenSymbol: String,
        targetPrice: Double,
        condition: AlertCondition,
        isEnabled: Bool = true,
        isTriggered: Bool = false,
        createdAt: Date = Date(),
        triggeredAt: Date? = nil
    ) {
        self.id = id
        self.tokenId = tokenId
        self.tokenSymbol = tokenSymbol
        self.targetPrice = targetPrice
        self.condition = condition
        self.isEnabled = isEnabled
        self.isTriggered = isTriggered
        self.createdAt = createdAt
        self.triggeredAt = triggeredAt
    }

    /// Check if alert should trigger
    func shouldTrigger(currentPrice: Double) -> Bool {
        return isEnabled && !isTriggered && condition.isMet(currentPrice: currentPrice, targetPrice: targetPrice)
    }
}

// MARK: - Token Balance

/// Token balance with market value
struct TokenBalance: Codable, Identifiable, Sendable {
    let id: UUID
    let token: TokenMetadata
    let balance: String // Wei/smallest unit as string
    let balanceFormatted: Double // Human-readable balance
    let priceData: PriceData?
    let valueUSD: Double?
    let lastUpdated: Date

    init(
        id: UUID = UUID(),
        token: TokenMetadata,
        balance: String,
        priceData: PriceData? = nil,
        lastUpdated: Date = Date()
    ) {
        self.id = id
        self.token = token
        self.balance = balance

        // Calculate formatted balance
        if let balanceInt = Double(balance) {
            self.balanceFormatted = balanceInt / pow(10, Double(token.decimals))
        } else {
            self.balanceFormatted = 0
        }

        self.priceData = priceData

        // Calculate USD value
        if let price = priceData?.price {
            self.valueUSD = balanceFormatted * price
        } else {
            self.valueUSD = nil
        }

        self.lastUpdated = lastUpdated
    }

    /// Formatted balance string with symbol
    var formattedBalance: String {
        let formatter = NumberFormatter()
        formatter.numberStyle = .decimal
        formatter.maximumFractionDigits = 6
        formatter.minimumFractionDigits = 2
        let balanceStr = formatter.string(from: NSNumber(value: balanceFormatted)) ?? "0.00"
        return "\(balanceStr) \(token.symbol)"
    }

    /// Formatted USD value
    var formattedValue: String {
        guard let value = valueUSD else { return "$0.00" }
        let formatter = NumberFormatter()
        formatter.numberStyle = .currency
        formatter.currencyCode = "USD"
        formatter.maximumFractionDigits = 2
        return formatter.string(from: NSNumber(value: value)) ?? "$0.00"
    }
}

// MARK: - Portfolio Summary

/// Complete portfolio overview
struct PortfolioSummary: Codable, Sendable {
    let totalValueUSD: Double
    let change24h: Double
    let change24hPercent: Double
    let tokens: [TokenBalance]
    let topGainers: [TokenBalance]
    let topLosers: [TokenBalance]
    let lastUpdated: Date

    init(tokens: [TokenBalance]) {
        self.tokens = tokens
        self.lastUpdated = Date()

        // Calculate total value
        self.totalValueUSD = tokens.compactMap { $0.valueUSD }.reduce(0, +)

        // Calculate 24h change
        var totalChange: Double = 0
        for token in tokens {
            if let price = token.priceData?.price,
               let change = token.priceData?.change24h {
                let previousValue = token.balanceFormatted * price / (1 + change / 100)
                let changeValue = token.balanceFormatted * price - previousValue
                totalChange += changeValue
            }
        }
        self.change24h = totalChange
        self.change24hPercent = totalValueUSD > 0 ? (totalChange / totalValueUSD) * 100 : 0

        // Find top gainers and losers
        let sorted = tokens
            .filter { $0.priceData != nil }
            .sorted { ($0.priceData?.change24h ?? 0) > ($1.priceData?.change24h ?? 0) }

        self.topGainers = Array(sorted.prefix(5))
        self.topLosers = Array(sorted.suffix(5).reversed())
    }

    /// Formatted total value
    var formattedTotalValue: String {
        let formatter = NumberFormatter()
        formatter.numberStyle = .currency
        formatter.currencyCode = "USD"
        formatter.maximumFractionDigits = 2
        return formatter.string(from: NSNumber(value: totalValueUSD)) ?? "$0.00"
    }

    /// Formatted 24h change
    var formattedChange24h: String {
        let sign = change24h >= 0 ? "+" : ""
        let formatter = NumberFormatter()
        formatter.numberStyle = .currency
        formatter.currencyCode = "USD"
        formatter.maximumFractionDigits = 2
        let changeStr = formatter.string(from: NSNumber(value: abs(change24h))) ?? "$0.00"
        return "\(sign)\(changeStr) (\(String(format: "%.2f%%", change24hPercent)))"
    }
}

// MARK: - Watchlist Item

/// Token in user's watchlist
struct WatchlistItem: Codable, Identifiable, Sendable {
    let id: UUID
    let tokenId: String
    let symbol: String
    let name: String
    let addedAt: Date

    init(id: UUID = UUID(), tokenId: String, symbol: String, name: String, addedAt: Date = Date()) {
        self.id = id
        self.tokenId = tokenId
        self.symbol = symbol
        self.name = name
        self.addedAt = addedAt
    }
}
