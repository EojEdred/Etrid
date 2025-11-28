//
//  Trading.swift
//  EtridWallet
//
//  Trading and market data models
//

import Foundation

struct TradingPair: Identifiable, Codable {
    let id: String
    let pair: String
    let baseToken: String
    let quoteToken: String
    let price: String
    let change24h: String
    let volume24h: String
    let high24h: String?
    let low24h: String?

    var isPositiveChange: Bool {
        change24h.hasPrefix("+")
    }

    var changeColor: String {
        isPositiveChange ? "green" : "red"
    }
}

struct Trade: Identifiable, Codable {
    let id: String
    let pair: String
    let type: TradeType
    let amount: String
    let price: String
    let timestamp: Date
    let status: TradeStatus
    let fee: String?

    enum TradeType: String, Codable {
        case buy
        case sell
        case swap
    }

    enum TradeStatus: String, Codable {
        case pending
        case completed
        case failed
        case cancelled
    }
}

struct MarketStats: Codable {
    let tradingVolume24h: String
    let marketCap: String
    let totalTrades: Int
    let activePairs: Int
}

// MARK: - Mock Data
extension TradingPair {
    static let mockPairs: [TradingPair] = [
        TradingPair(
            id: "1",
            pair: "ÉTR/EDSC",
            baseToken: "ÉTR",
            quoteToken: "EDSC",
            price: "8.00",
            change24h: "+2.5%",
            volume24h: "1.2M",
            high24h: "8.15",
            low24h: "7.85"
        ),
        TradingPair(
            id: "2",
            pair: "ÉTR/DOT",
            baseToken: "ÉTR",
            quoteToken: "DOT",
            price: "1.25",
            change24h: "-1.2%",
            volume24h: "850K",
            high24h: "1.28",
            low24h: "1.22"
        ),
        TradingPair(
            id: "3",
            pair: "EDSC/USDT",
            baseToken: "EDSC",
            quoteToken: "USDT",
            price: "1.00",
            change24h: "+0.1%",
            volume24h: "2.5M",
            high24h: "1.02",
            low24h: "0.98"
        )
    ]
}
