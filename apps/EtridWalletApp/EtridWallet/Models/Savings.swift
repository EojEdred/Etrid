//
//  Savings.swift
//  EtridWallet
//
//  Models for savings goals and auto-save functionality
//

import Foundation

// MARK: - Savings Goal
struct SavingsGoal: Identifiable, Codable, Equatable {
    let id: UUID
    var name: String
    var targetAmount: Double
    var currentAmount: Double
    var token: Token
    var deadline: Date?
    var category: GoalCategory
    var imageIcon: String
    var autoSaveRules: [AutoSaveRule]
    let createdAt: Date
    var lastUpdated: Date

    init(id: UUID = UUID(),
         name: String,
         targetAmount: Double,
         currentAmount: Double = 0,
         token: Token,
         deadline: Date? = nil,
         category: GoalCategory,
         imageIcon: String,
         autoSaveRules: [AutoSaveRule] = [],
         createdAt: Date = Date(),
         lastUpdated: Date = Date()) {
        self.id = id
        self.name = name
        self.targetAmount = targetAmount
        self.currentAmount = currentAmount
        self.token = token
        self.deadline = deadline
        self.category = category
        self.imageIcon = imageIcon
        self.autoSaveRules = autoSaveRules
        self.createdAt = createdAt
        self.lastUpdated = lastUpdated
    }

    var progress: GoalProgress {
        GoalProgress(
            currentAmount: currentAmount,
            targetAmount: targetAmount,
            percentage: progressPercentage,
            remainingAmount: remainingAmount,
            daysRemaining: daysRemaining,
            isCompleted: isCompleted
        )
    }

    var progressPercentage: Double {
        guard targetAmount > 0 else { return 0 }
        return min((currentAmount / targetAmount) * 100, 100)
    }

    var remainingAmount: Double {
        max(targetAmount - currentAmount, 0)
    }

    var daysRemaining: Int? {
        guard let deadline = deadline else { return nil }
        let calendar = Calendar.current
        let components = calendar.dateComponents([.day], from: Date(), to: deadline)
        return components.day
    }

    var isCompleted: Bool {
        currentAmount >= targetAmount
    }

    var isOverdue: Bool {
        guard let deadline = deadline else { return false }
        return Date() > deadline && !isCompleted
    }

    var formattedTarget: String {
        String(format: "%.4f %@", targetAmount, token.symbol)
    }

    var formattedCurrent: String {
        String(format: "%.4f %@", currentAmount, token.symbol)
    }

    var formattedRemaining: String {
        String(format: "%.4f %@", remainingAmount, token.symbol)
    }

    var estimatedCompletionDate: Date? {
        guard !autoSaveRules.isEmpty else { return nil }

        let totalMonthlyDeposit = autoSaveRules.reduce(0.0) { total, rule in
            total + rule.estimatedMonthlyAmount
        }

        guard totalMonthlyDeposit > 0 else { return nil }

        let monthsToComplete = remainingAmount / totalMonthlyDeposit
        return Calendar.current.date(byAdding: .month, value: Int(ceil(monthsToComplete)), to: Date())
    }
}

// MARK: - Auto Save Rule
struct AutoSaveRule: Identifiable, Codable, Equatable {
    let id: UUID
    var type: AutoSaveType
    var amount: Double
    var frequency: SaveFrequency
    var isActive: Bool
    let createdAt: Date

    init(id: UUID = UUID(),
         type: AutoSaveType,
         amount: Double,
         frequency: SaveFrequency,
         isActive: Bool = true,
         createdAt: Date = Date()) {
        self.id = id
        self.type = type
        self.amount = amount
        self.frequency = frequency
        self.isActive = isActive
        self.createdAt = createdAt
    }

    var formattedAmount: String {
        String(format: "%.4f", amount)
    }

    var description: String {
        switch type {
        case .fixed:
            return "Save \(formattedAmount) every \(frequency.rawValue.lowercased())"
        case .percentage:
            return "Save \(String(format: "%.1f", amount))% of deposits \(frequency.rawValue.lowercased())"
        case .roundUp:
            return "Round up transactions to nearest \(formattedAmount)"
        case .milestone:
            return "Save \(formattedAmount) when milestone reached"
        }
    }

    var estimatedMonthlyAmount: Double {
        switch frequency {
        case .daily:
            return amount * 30
        case .weekly:
            return amount * 4
        case .biweekly:
            return amount * 2
        case .monthly:
            return amount
        case .onDeposit:
            return amount * 4 // Estimate 4 deposits per month
        }
    }
}

// MARK: - Goal Progress
struct GoalProgress: Codable, Equatable {
    let currentAmount: Double
    let targetAmount: Double
    let percentage: Double
    let remainingAmount: Double
    let daysRemaining: Int?
    let isCompleted: Bool

    var progressText: String {
        String(format: "%.1f%% Complete", percentage)
    }

    var statusText: String {
        if isCompleted {
            return "Goal Achieved!"
        } else if let days = daysRemaining {
            if days > 0 {
                return "\(days) days remaining"
            } else {
                return "Overdue"
            }
        } else {
            return "In Progress"
        }
    }
}

// MARK: - Goal Category
enum GoalCategory: String, Codable, CaseIterable {
    case emergency = "Emergency Fund"
    case vacation = "Vacation"
    case investment = "Investment"
    case purchase = "Big Purchase"
    case education = "Education"
    case retirement = "Retirement"
    case custom = "Custom"

    var iconName: String {
        switch self {
        case .emergency:
            return "shield.fill"
        case .vacation:
            return "airplane"
        case .investment:
            return "chart.line.uptrend.xyaxis"
        case .purchase:
            return "cart.fill"
        case .education:
            return "graduationcap.fill"
        case .retirement:
            return "house.fill"
        case .custom:
            return "star.fill"
        }
    }

    var color: String {
        switch self {
        case .emergency:
            return "red"
        case .vacation:
            return "blue"
        case .investment:
            return "green"
        case .purchase:
            return "purple"
        case .education:
            return "orange"
        case .retirement:
            return "indigo"
        case .custom:
            return "gray"
        }
    }
}

// MARK: - Auto Save Type
enum AutoSaveType: String, Codable {
    case fixed = "Fixed Amount"
    case percentage = "Percentage"
    case roundUp = "Round Up"
    case milestone = "Milestone"

    var description: String {
        switch self {
        case .fixed:
            return "Save a fixed amount regularly"
        case .percentage:
            return "Save a percentage of your deposits"
        case .roundUp:
            return "Round up transactions and save the difference"
        case .milestone:
            return "Save when you reach specific milestones"
        }
    }
}

// MARK: - Save Frequency
enum SaveFrequency: String, Codable, CaseIterable {
    case daily = "Daily"
    case weekly = "Weekly"
    case biweekly = "Bi-weekly"
    case monthly = "Monthly"
    case onDeposit = "On Deposit"
}

// MARK: - Savings Deposit
struct SavingsDeposit: Identifiable, Codable, Equatable {
    let id: UUID
    let goalId: UUID
    let amount: Double
    let source: DepositSource
    let timestamp: Date
    var transactionHash: String?

    init(id: UUID = UUID(),
         goalId: UUID,
         amount: Double,
         source: DepositSource,
         timestamp: Date = Date(),
         transactionHash: String? = nil) {
        self.id = id
        self.goalId = goalId
        self.amount = amount
        self.source = source
        self.timestamp = timestamp
        self.transactionHash = transactionHash
    }

    var formattedAmount: String {
        String(format: "%.4f", amount)
    }
}

// MARK: - Deposit Source
enum DepositSource: String, Codable {
    case manual = "Manual"
    case autoSave = "Auto-Save"
    case roundUp = "Round-Up"
    case milestone = "Milestone"
}

// MARK: - Goal Achievement
struct GoalAchievement: Identifiable, Codable, Equatable {
    let id: UUID
    let goalId: UUID
    let goalName: String
    let targetAmount: Double
    let achievedAt: Date
    let daysToComplete: Int
    let totalDeposits: Int

    init(id: UUID = UUID(),
         goalId: UUID,
         goalName: String,
         targetAmount: Double,
         achievedAt: Date = Date(),
         daysToComplete: Int,
         totalDeposits: Int) {
        self.id = id
        self.goalId = goalId
        self.goalName = goalName
        self.targetAmount = targetAmount
        self.achievedAt = achievedAt
        self.daysToComplete = daysToComplete
        self.totalDeposits = totalDeposits
    }

    var formattedTarget: String {
        String(format: "%.2f", targetAmount)
    }
}

// MARK: - Mock Data
extension SavingsGoal {
    static let mockGoals: [SavingsGoal] = [
        SavingsGoal(
            name: "Emergency Fund",
            targetAmount: 10.0,
            currentAmount: 6.5,
            token: Token(symbol: "ETH", name: "Ethereum", decimals: 18, isNative: true),
            deadline: Calendar.current.date(byAdding: .month, value: 6, to: Date()),
            category: .emergency,
            imageIcon: "shield.fill",
            autoSaveRules: [
                AutoSaveRule(type: .fixed, amount: 0.5, frequency: .weekly),
                AutoSaveRule(type: .percentage, amount: 10, frequency: .onDeposit)
            ]
        ),
        SavingsGoal(
            name: "Summer Vacation",
            targetAmount: 5000,
            currentAmount: 2800,
            token: Token(symbol: "USDC", name: "USD Coin", contractAddress: "0xA0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48", decimals: 6),
            deadline: Calendar.current.date(byAdding: .month, value: 4, to: Date()),
            category: .vacation,
            imageIcon: "airplane",
            autoSaveRules: [
                AutoSaveRule(type: .fixed, amount: 500, frequency: .monthly)
            ]
        ),
        SavingsGoal(
            name: "Investment Portfolio",
            targetAmount: 50000,
            currentAmount: 18500,
            token: Token(symbol: "DAI", name: "Dai Stablecoin", contractAddress: "0x6B175474E89094C44Da98b954EedeAC495271d0F", decimals: 18),
            deadline: nil,
            category: .investment,
            imageIcon: "chart.line.uptrend.xyaxis",
            autoSaveRules: [
                AutoSaveRule(type: .fixed, amount: 1000, frequency: .monthly),
                AutoSaveRule(type: .roundUp, amount: 1, frequency: .daily)
            ]
        )
    ]
}
