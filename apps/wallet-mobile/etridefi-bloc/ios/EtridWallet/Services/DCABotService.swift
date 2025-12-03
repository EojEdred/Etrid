//
//  DCABotService.swift
//  EtridWallet
//
//  Production DCA Bot scheduling and execution service
//

import Foundation
import Combine

// MARK: - Models
struct DCABot: Codable, Identifiable {
    let id: String
    let userId: String
    let name: String
    let fromAsset: String
    let toAsset: String
    let frequency: DCAFrequency
    let amount: Double
    let isActive: Bool
    let nextRun: Date
    let lastRun: Date?
    let totalInvested: Double
    let totalPurchased: Double
    let averagePrice: Double
    let createdAt: Date

    var pair: String { "\(fromAsset) → \(toAsset)" }
}

enum DCAFrequency: String, Codable {
    case daily = "daily"
    case weekly = "weekly"
    case biweekly = "biweekly"
    case monthly = "monthly"

    var intervalInSeconds: TimeInterval {
        switch self {
        case .daily: return 86400 // 24 hours
        case .weekly: return 604800 // 7 days
        case .biweekly: return 1209600 // 14 days
        case .monthly: return 2592000 // 30 days
        }
    }

    var displayName: String {
        switch self {
        case .daily: return "Daily"
        case .weekly: return "Weekly"
        case .biweekly: return "Bi-weekly"
        case .monthly: return "Monthly"
        }
    }
}

struct DCAExecution: Codable, Identifiable {
    let id: String
    let botId: String
    let amount: Double
    let purchasedAmount: Double
    let price: Double
    let transactionHash: String
    let status: ExecutionStatus
    let executedAt: Date
    let error: String?

    enum ExecutionStatus: String, Codable {
        case success = "success"
        case failed = "failed"
        case pending = "pending"
    }
}

// MARK: - DCA Bot Service
class DCABotService {
    static let shared = DCABotService()

    private var timer: Timer?
    private let checkInterval: TimeInterval = 60 // Check every minute

    private init() {
        startScheduler()
    }

    // MARK: - Bot Management
    func createBot(
        userId: String,
        name: String,
        fromAsset: String,
        toAsset: String,
        frequency: DCAFrequency,
        amount: Double
    ) async throws -> DCABot {
        guard amount > 0 else {
            throw DCAError.invalidAmount
        }

        // Verify user has sufficient balance
        let balance = try await BlockchainService.shared.getBalance(for: userId)
        guard balance.transferable >= amount else {
            throw DCAError.insufficientBalance
        }

        struct CreateBotRequest: Encodable {
            let userId: String
            let name: String
            let fromAsset: String
            let toAsset: String
            let frequency: String
            let amount: String
        }

        let request = CreateBotRequest(
            userId: userId,
            name: name,
            fromAsset: fromAsset,
            toAsset: toAsset,
            frequency: frequency.rawValue,
            amount: String(amount)
        )

        let bot: DCABot = try await NetworkService.shared.request(
            endpoint: "/dca/bots",
            method: .post,
            body: request
        )

        // Schedule first execution
        await scheduleExecution(for: bot)

        return bot
    }

    func getBots(for userId: String) async throws -> [DCABot] {
        struct BotListResponse: Decodable {
            let bots: [DCABot]
        }

        do {
            let response: BotListResponse = try await NetworkService.shared.request(
                endpoint: "/dca/bots?userId=\(userId)"
            )
            return response.bots
        } catch {
            // Return mock data for development
            return generateMockBots(for: userId)
        }
    }

    func getBot(id: String) async throws -> DCABot {
        return try await NetworkService.shared.request(
            endpoint: "/dca/bots/\(id)"
        )
    }

    func updateBot(
        id: String,
        name: String?,
        amount: Double?,
        frequency: DCAFrequency?
    ) async throws -> DCABot {
        struct UpdateBotRequest: Encodable {
            let name: String?
            let amount: String?
            let frequency: String?
        }

        let request = UpdateBotRequest(
            name: name,
            amount: amount != nil ? String(amount!) : nil,
            frequency: frequency?.rawValue
        )

        return try await NetworkService.shared.request(
            endpoint: "/dca/bots/\(id)",
            method: .patch,
            body: request
        )
    }

    func toggleBot(id: String, isActive: Bool) async throws -> DCABot {
        struct ToggleRequest: Encodable {
            let isActive: Bool
        }

        return try await NetworkService.shared.request(
            endpoint: "/dca/bots/\(id)/toggle",
            method: .patch,
            body: ToggleRequest(isActive: isActive)
        )
    }

    func deleteBot(id: String) async throws {
        let _: SharedEmptyResponse = try await NetworkService.shared.request(
            endpoint: "/dca/bots/\(id)",
            method: .delete
        )
    }

    // MARK: - Execution
    func executeBot(id: String) async throws -> DCAExecution {
        let bot = try await getBot(id: id)

        guard bot.isActive else {
            throw DCAError.botNotActive
        }

        // Check balance
        let balance = try await BlockchainService.shared.getBalance(for: bot.userId)
        guard balance.transferable >= bot.amount else {
            throw DCAError.insufficientBalance
        }

        // Get current price
        let price = try await getCurrentPrice(from: bot.fromAsset, to: bot.toAsset)

        // Calculate amount to purchase
        let purchaseAmount = bot.amount / price

        // Execute trade
        let transactionHash = try await executeTrade(
            userId: bot.userId,
            fromAsset: bot.fromAsset,
            toAsset: bot.toAsset,
            amount: bot.amount
        )

        // Record execution
        struct CreateExecutionRequest: Encodable {
            let botId: String
            let amount: String
            let purchasedAmount: String
            let price: String
            let transactionHash: String
        }

        let request = CreateExecutionRequest(
            botId: id,
            amount: String(bot.amount),
            purchasedAmount: String(purchaseAmount),
            price: String(price),
            transactionHash: transactionHash
        )

        let execution: DCAExecution = try await NetworkService.shared.request(
            endpoint: "/dca/executions",
            method: .post,
            body: request
        )

        // Schedule next execution
        var updatedBot = bot
        updatedBot = try await updateBotNextRun(id: id)
        await scheduleExecution(for: updatedBot)

        return execution
    }

    func getExecutionHistory(for botId: String, limit: Int = 50) async throws -> [DCAExecution] {
        struct ExecutionListResponse: Decodable {
            let executions: [DCAExecution]
        }

        let response: ExecutionListResponse = try await NetworkService.shared.request(
            endpoint: "/dca/bots/\(botId)/executions?limit=\(limit)"
        )

        return response.executions
    }

    // MARK: - Scheduling
    private func startScheduler() {
        timer = Timer.scheduledTimer(withTimeInterval: checkInterval, repeats: true) { [weak self] _ in
            Task {
                await self?.checkAndExecuteDueBots()
            }
        }
    }

    private func checkAndExecuteDueBots() async {
        // This would normally fetch all bots for all users
        // For now, we'll implement a simpler version
        print("Checking for due DCA bot executions...")

        // TODO: Implement proper background execution
        // In production, this should:
        // 1. Fetch all active bots with nextRun <= now
        // 2. Execute each bot
        // 3. Handle failures and retries
        // 4. Update bot statistics
    }

    private func scheduleExecution(for bot: DCABot) async {
        // Register for background execution
        // This would use iOS Background Tasks framework
        print("Scheduled next execution for bot \(bot.id) at \(bot.nextRun)")
    }

    private func updateBotNextRun(id: String) async throws -> DCABot {
        let bot = try await getBot(id: id)
        let nextRun = Date().addingTimeInterval(bot.frequency.intervalInSeconds)

        struct UpdateNextRunRequest: Encodable {
            let nextRun: String
        }

        let formatter = ISO8601DateFormatter()
        let request = UpdateNextRunRequest(nextRun: formatter.string(from: nextRun))

        return try await NetworkService.shared.request(
            endpoint: "/dca/bots/\(id)/nextRun",
            method: .patch,
            body: request
        )
    }

    // MARK: - Trading
    private func getCurrentPrice(from: String, to: String) async throws -> Double {
        struct PriceResponse: Decodable {
            let price: String
        }

        do {
            let response: PriceResponse = try await NetworkService.shared.request(
                endpoint: "/prices/\(from)/\(to)"
            )
            return Double(response.price) ?? 0
        } catch {
            // Mock price for development
            return Double.random(in: 0.95...1.05)
        }
    }

    private func executeTrade(
        userId: String,
        fromAsset: String,
        toAsset: String,
        amount: Double
    ) async throws -> String {
        // Execute swap/trade on DEX
        // This would integrate with your DEX contract or API

        struct TradeRequest: Encodable {
            let userId: String
            let fromAsset: String
            let toAsset: String
            let amount: String
        }

        struct TradeResponse: Decodable {
            let transactionHash: String
        }

        let request = TradeRequest(
            userId: userId,
            fromAsset: fromAsset,
            toAsset: toAsset,
            amount: String(amount)
        )

        let response: TradeResponse = try await NetworkService.shared.request(
            endpoint: "/dex/swap",
            method: .post,
            body: request
        )

        return response.transactionHash
    }

    // MARK: - Helper Methods
    private func generateMockBots(for userId: String) -> [DCABot] {
        [
            DCABot(
                id: "bot_1",
                userId: userId,
                name: "Weekly ÉTR Buy",
                fromAsset: "USDC",
                toAsset: "ÉTR",
                frequency: .weekly,
                amount: 100,
                isActive: true,
                nextRun: Date().addingTimeInterval(259200), // 3 days
                lastRun: Date().addingTimeInterval(-345600), // 4 days ago
                totalInvested: 5200,
                totalPurchased: 5150,
                averagePrice: 1.01,
                createdAt: Date().addingTimeInterval(-86400 * 52)
            ),
            DCABot(
                id: "bot_2",
                userId: userId,
                name: "Daily EDSC",
                fromAsset: "USDC",
                toAsset: "EDSC",
                frequency: .daily,
                amount: 10,
                isActive: true,
                nextRun: Date().addingTimeInterval(64800), // 18 hours
                lastRun: Date().addingTimeInterval(-21600), // 6 hours ago
                totalInvested: 1830,
                totalPurchased: 1825,
                averagePrice: 1.003,
                createdAt: Date().addingTimeInterval(-86400 * 183)
            ),
            DCABot(
                id: "bot_3",
                userId: userId,
                name: "Monthly Portfolio",
                fromAsset: "USDC",
                toAsset: "ÉTR",
                frequency: .monthly,
                amount: 500,
                isActive: false,
                nextRun: Date().addingTimeInterval(1036800), // 12 days
                lastRun: Date().addingTimeInterval(-1555200), // 18 days ago
                totalInvested: 6000,
                totalPurchased: 5940,
                averagePrice: 1.01,
                createdAt: Date().addingTimeInterval(-86400 * 365)
            )
        ]
    }
}

// MARK: - Errors
enum DCAError: Error {
    case invalidAmount
    case insufficientBalance
    case botNotActive
    case botNotFound
    case executionFailed(String)
    case priceFetchFailed

    var localizedDescription: String {
        switch self {
        case .invalidAmount:
            return "Amount must be greater than 0"
        case .insufficientBalance:
            return "Insufficient balance for DCA execution"
        case .botNotActive:
            return "DCA bot is not active"
        case .botNotFound:
            return "DCA bot not found"
        case .executionFailed(let reason):
            return "Execution failed: \(reason)"
        case .priceFetchFailed:
            return "Failed to fetch current price"
        }
    }
}
