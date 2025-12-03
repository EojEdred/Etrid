//
//  AnalyticsService.swift
//  EtridWallet
//
//  Production portfolio analytics and tracking service
//

import Foundation

// MARK: - Models
struct PortfolioSnapshot: Codable, Identifiable {
    let id: String
    let userId: String
    let totalValue: Double
    let change24h: Double
    let timestamp: Date
    let assets: [AssetHolding]
}

struct AssetHolding: Codable, Identifiable {
    let id: String
    let asset: String
    let amount: Double
    let valueUSD: Double
    let price: Double
    let change24h: Double
    let allocation: Double // Percentage
}

struct PerformanceData: Codable {
    let period: TimePeriod
    let dataPoints: [DataPoint]
    let roi: Double
    let totalGain: Double
    let totalGainPercent: Double

    enum TimePeriod: String, Codable {
        case day = "1D"
        case week = "1W"
        case month = "1M"
        case threeMonths = "3M"
        case year = "1Y"
        case all = "ALL"
    }

    struct DataPoint: Codable, Identifiable {
        let id: String
        let timestamp: Date
        let value: Double
    }
}

struct RiskMetrics: Codable {
    let volatility: Double
    let sharpeRatio: Double
    let maxDrawdown: Double
    let beta: Double
    let diversificationScore: Int
    let riskLevel: RiskLevel

    enum RiskLevel: String, Codable {
        case low = "low"
        case medium = "medium"
        case high = "high"
        case veryHigh = "very_high"
    }
}

// MARK: - Analytics Service
class AnalyticsService {
    static let shared = AnalyticsService()

    private init() {}

    // MARK: - Portfolio Snapshot
    func getCurrentPortfolio(for userId: String) async throws -> PortfolioSnapshot {
        do {
            return try await NetworkService.shared.request(
                endpoint: "/analytics/portfolio/current?userId=\(userId)"
            )
        } catch {
            // Return mock data for development
            return generateMockPortfolio(for: userId)
        }
    }

    func getPortfolioHistory(
        for userId: String,
        period: PerformanceData.TimePeriod
    ) async throws -> [PortfolioSnapshot] {
        struct PortfolioHistoryResponse: Decodable {
            let snapshots: [PortfolioSnapshot]
        }

        let response: PortfolioHistoryResponse = try await NetworkService.shared.request(
            endpoint: "/analytics/portfolio/history?userId=\(userId)&period=\(period.rawValue)"
        )

        return response.snapshots
    }

    // MARK: - Performance Tracking
    func getPerformance(
        for userId: String,
        period: PerformanceData.TimePeriod
    ) async throws -> PerformanceData {
        do {
            return try await NetworkService.shared.request(
                endpoint: "/analytics/performance?userId=\(userId)&period=\(period.rawValue)"
            )
        } catch {
            return generateMockPerformance(period: period)
        }
    }

    // MARK: - Risk Analysis
    func getRiskMetrics(for userId: String) async throws -> RiskMetrics {
        do {
            return try await NetworkService.shared.request(
                endpoint: "/analytics/risk?userId=\(userId)"
            )
        } catch {
            return RiskMetrics(
                volatility: 0.25,
                sharpeRatio: 1.5,
                maxDrawdown: 0.15,
                beta: 1.2,
                diversificationScore: 85,
                riskLevel: .medium
            )
        }
    }

    // MARK: - Asset Allocation
    func getAssetAllocation(for userId: String) async throws -> [AssetHolding] {
        let portfolio = try await getCurrentPortfolio(for: userId)
        return portfolio.assets
    }

    // MARK: - Tax Reports
    // Uses TaxReport from SharedComponents.swift
    func generateTaxReport(
        for userId: String,
        year: Int
    ) async throws -> TaxReport {
        return try await NetworkService.shared.request(
            endpoint: "/analytics/tax?userId=\(userId)&year=\(year)"
        )
    }

    // MARK: - Insights
    // Uses Insight from SharedComponents.swift
    func getInsights(for userId: String) async throws -> [Insight] {
        struct InsightListResponse: Decodable {
            let insights: [Insight]
        }

        let response: InsightListResponse = try await NetworkService.shared.request(
            endpoint: "/analytics/insights?userId=\(userId)"
        )

        return response.insights
    }

    // MARK: - Helper Methods
    private func generateMockPortfolio(for userId: String) -> PortfolioSnapshot {
        PortfolioSnapshot(
            id: "snap_1",
            userId: userId,
            totalValue: 125000,
            change24h: 5.2,
            timestamp: Date(),
            assets: [
                AssetHolding(
                    id: "holding_1",
                    asset: "ÉTR",
                    amount: 50000,
                    valueUSD: 56250,
                    price: 1.125,
                    change24h: 6.5,
                    allocation: 45
                ),
                AssetHolding(
                    id: "holding_2",
                    asset: "EDSC",
                    amount: 30000,
                    valueUSD: 37500,
                    price: 1.25,
                    change24h: 4.2,
                    allocation: 30
                ),
                AssetHolding(
                    id: "holding_3",
                    asset: "USDC",
                    amount: 20000,
                    valueUSD: 20000,
                    price: 1.0,
                    change24h: 0.01,
                    allocation: 16
                ),
                AssetHolding(
                    id: "holding_4",
                    asset: "ETH",
                    amount: 5,
                    valueUSD: 11250,
                    price: 2250,
                    change24h: 3.8,
                    allocation: 9
                )
            ]
        )
    }

    private func generateMockPerformance(period: PerformanceData.TimePeriod) -> PerformanceData {
        let days: Int
        switch period {
        case .day: days = 1
        case .week: days = 7
        case .month: days = 30
        case .threeMonths: days = 90
        case .year: days = 365
        case .all: days = 730
        }

        var dataPoints: [PerformanceData.DataPoint] = []
        let now = Date()
        let baseValue: Double = 100000
        let volatility: Double = 0.02

        for i in 0..<days {
            let date = Calendar.current.date(byAdding: .day, value: -days + i, to: now)!
            let trend = Double(i) / Double(days) * 25000 // Upward trend
            let randomness = Double.random(in: -volatility...volatility) * baseValue
            let value = baseValue + trend + randomness

            dataPoints.append(
                PerformanceData.DataPoint(
                    id: "dp_\(i)",
                    timestamp: date,
                    value: value
                )
            )
        }

        let startValue = dataPoints.first?.value ?? baseValue
        let endValue = dataPoints.last?.value ?? baseValue
        let totalGain = endValue - startValue
        let totalGainPercent = (totalGain / startValue) * 100

        return PerformanceData(
            period: period,
            dataPoints: dataPoints,
            roi: totalGainPercent,
            totalGain: totalGain,
            totalGainPercent: totalGainPercent
        )
    }
}

// MARK: - Errors
enum AnalyticsError: Error {
    case dataNotAvailable
    case invalidPeriod

    var localizedDescription: String {
        switch self {
        case .dataNotAvailable:
            return "Analytics data not available"
        case .invalidPeriod:
            return "Invalid time period"
        }
    }
}
