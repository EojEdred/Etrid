//
//  Notification.swift
//  EtridWallet
//
//  Notification models and settings
//

import Foundation
import UserNotifications

// MARK: - Notification Settings
struct NotificationSettings: Codable {
    var transactionNotifications: Bool = true
    var priceAlerts: Bool = true
    var daoVotingReminders: Bool = true
    var paymentRequests: Bool = true
    var securityAlerts: Bool = true
    var marketingNotifications: Bool = false

    // Price alert thresholds
    var priceAlertThresholdPercentage: Double = 5.0
    var priceAlertCooldownMinutes: Int = 60

    // Sound and badge settings
    var soundEnabled: Bool = true
    var badgeEnabled: Bool = true
    var showPreviewsWhenUnlocked: Bool = true

    static let defaultSettings = NotificationSettings()
}

// MARK: - Notification Category
enum NotificationCategory: String, CaseIterable {
    case transaction = "TRANSACTION"
    case priceAlert = "PRICE_ALERT"
    case daoVoting = "DAO_VOTING"
    case paymentRequest = "PAYMENT_REQUEST"
    case security = "SECURITY"
    case marketing = "MARKETING"

    var displayName: String {
        switch self {
        case .transaction:
            return "Transactions"
        case .priceAlert:
            return "Price Alerts"
        case .daoVoting:
            return "DAO Voting"
        case .paymentRequest:
            return "Payment Requests"
        case .security:
            return "Security Alerts"
        case .marketing:
            return "Marketing"
        }
    }

    var iconName: String {
        switch self {
        case .transaction:
            return "arrow.left.arrow.right"
        case .priceAlert:
            return "chart.line.uptrend.xyaxis"
        case .daoVoting:
            return "person.3.fill"
        case .paymentRequest:
            return "dollarsign.circle.fill"
        case .security:
            return "shield.fill"
        case .marketing:
            return "megaphone.fill"
        }
    }

    var description: String {
        switch self {
        case .transaction:
            return "Notifications for incoming and outgoing transactions"
        case .priceAlert:
            return "Alerts when token prices change significantly"
        case .daoVoting:
            return "Reminders for active DAO proposals"
        case .paymentRequest:
            return "Notifications for payment requests"
        case .security:
            return "Important security alerts and warnings"
        case .marketing:
            return "News, updates, and promotional content"
        }
    }
}

// MARK: - Notification Action
enum NotificationAction: String {
    case view = "VIEW_ACTION"
    case approve = "APPROVE_ACTION"
    case decline = "DECLINE_ACTION"
    case vote = "VOTE_ACTION"
    case pay = "PAY_ACTION"

    var title: String {
        switch self {
        case .view:
            return "View"
        case .approve:
            return "Approve"
        case .decline:
            return "Decline"
        case .vote:
            return "Vote"
        case .pay:
            return "Pay"
        }
    }
}

// MARK: - Notification Content
struct NotificationModel: Identifiable {
    let id: String
    let category: NotificationCategory
    let title: String
    let body: String
    let timestamp: Date
    var isRead: Bool = false
    var data: [String: Any]?

    init(id: String = UUID().uuidString,
         category: NotificationCategory,
         title: String,
         body: String,
         timestamp: Date = Date(),
         isRead: Bool = false,
         data: [String: Any]? = nil) {
        self.id = id
        self.category = category
        self.title = title
        self.body = body
        self.timestamp = timestamp
        self.isRead = isRead
        self.data = data
    }
}

// MARK: - Push Notification Payload
struct PushNotificationPayload: Codable {
    let title: String
    let body: String
    let category: String
    let data: [String: String]?
    let badge: Int?
    let sound: String?

    enum CodingKeys: String, CodingKey {
        case title
        case body
        case category
        case data
        case badge
        case sound
    }
}

// MARK: - Price Alert
struct PriceAlert: Identifiable, Codable {
    let id: String
    let tokenSymbol: String
    let tokenAddress: String
    let targetPrice: Double
    let condition: PriceAlertCondition
    let isEnabled: Bool
    let createdAt: Date
    var lastTriggered: Date?

    init(id: String = UUID().uuidString,
         tokenSymbol: String,
         tokenAddress: String,
         targetPrice: Double,
         condition: PriceAlertCondition,
         isEnabled: Bool = true,
         createdAt: Date = Date(),
         lastTriggered: Date? = nil) {
        self.id = id
        self.tokenSymbol = tokenSymbol
        self.tokenAddress = tokenAddress
        self.targetPrice = targetPrice
        self.condition = condition
        self.isEnabled = isEnabled
        self.createdAt = createdAt
        self.lastTriggered = lastTriggered
    }
}

enum PriceAlertCondition: String, Codable {
    case above
    case below
    case percentageChange

    var displayName: String {
        switch self {
        case .above:
            return "Above"
        case .below:
            return "Below"
        case .percentageChange:
            return "% Change"
        }
    }
}

// MARK: - DAO Voting Reminder
struct DAOVotingReminder: Identifiable, Codable {
    let id: String
    let proposalId: String
    let proposalTitle: String
    let endDate: Date
    let reminderDate: Date
    var isCompleted: Bool

    init(id: String = UUID().uuidString,
         proposalId: String,
         proposalTitle: String,
         endDate: Date,
         reminderDate: Date,
         isCompleted: Bool = false) {
        self.id = id
        self.proposalId = proposalId
        self.proposalTitle = proposalTitle
        self.endDate = endDate
        self.reminderDate = reminderDate
        self.isCompleted = isCompleted
    }
}
