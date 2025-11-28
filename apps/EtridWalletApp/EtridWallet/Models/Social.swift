//
//  Social.swift
//  EtridWallet
//
//  Social feed and interaction models
//

import Foundation

// MARK: - Activity Type
enum ActivityType: String, Codable {
    case sent = "Sent"
    case received = "Received"
    case billCreated = "Bill Created"
    case billPaid = "Bill Paid"
    case requestSent = "Request Sent"
    case requestReceived = "Request Received"
    case requestPaid = "Request Paid"
    case contactAdded = "Contact Added"
    case friendRequestSent = "Friend Request Sent"
    case friendRequestReceived = "Friend Request Received"
    case friendRequestAccepted = "Friend Request Accepted"

    var icon: String {
        switch self {
        case .sent: return "arrow.up.circle.fill"
        case .received: return "arrow.down.circle.fill"
        case .billCreated: return "doc.text.fill"
        case .billPaid: return "checkmark.circle.fill"
        case .requestSent: return "paperplane.fill"
        case .requestReceived: return "tray.fill"
        case .requestPaid: return "checkmark.seal.fill"
        case .contactAdded: return "person.fill.badge.plus"
        case .friendRequestSent: return "person.wave.2.fill"
        case .friendRequestReceived: return "person.wave.2.fill"
        case .friendRequestAccepted: return "person.2.fill"
        }
    }

    var color: String {
        switch self {
        case .sent: return "red"
        case .received: return "green"
        case .billCreated: return "blue"
        case .billPaid: return "green"
        case .requestSent: return "orange"
        case .requestReceived: return "blue"
        case .requestPaid: return "green"
        case .contactAdded: return "purple"
        case .friendRequestSent: return "blue"
        case .friendRequestReceived: return "orange"
        case .friendRequestAccepted: return "green"
        }
    }
}

// MARK: - Activity Item
struct ActivityItem: Identifiable, Codable {
    let id: UUID
    let type: ActivityType
    var title: String
    var description: String
    var amount: String?
    var tokenSymbol: String?
    var fromAddress: String?
    var toAddress: String?
    var fromName: String?
    var toName: String?
    var timestamp: Date
    var transactionHash: String?
    var billId: UUID?
    var requestId: UUID?
    var contactId: UUID?
    var isRead: Bool

    init(id: UUID = UUID(),
         type: ActivityType,
         title: String,
         description: String,
         amount: String? = nil,
         tokenSymbol: String? = nil,
         fromAddress: String? = nil,
         toAddress: String? = nil,
         fromName: String? = nil,
         toName: String? = nil,
         timestamp: Date = Date(),
         transactionHash: String? = nil,
         billId: UUID? = nil,
         requestId: UUID? = nil,
         contactId: UUID? = nil,
         isRead: Bool = false) {
        self.id = id
        self.type = type
        self.title = title
        self.description = description
        self.amount = amount
        self.tokenSymbol = tokenSymbol
        self.fromAddress = fromAddress
        self.toAddress = toAddress
        self.fromName = fromName
        self.toName = toName
        self.timestamp = timestamp
        self.transactionHash = transactionHash
        self.billId = billId
        self.requestId = requestId
        self.contactId = contactId
        self.isRead = isRead
    }

    var formattedAmount: String? {
        guard let amount = amount else { return nil }
        let value = Double(amount) ?? 0
        let formattedValue = String(format: "%.6f", value / 1e18)
        return tokenSymbol != nil ? "\(formattedValue) \(tokenSymbol!)" : formattedValue
    }

    var timeAgo: String {
        let formatter = RelativeDateTimeFormatter()
        formatter.unitsStyle = .short
        return formatter.localizedString(for: timestamp, relativeTo: Date())
    }
}

// MARK: - Friend Request
struct FriendRequest: Identifiable, Codable {
    let id: UUID
    var fromAddress: String
    var toAddress: String
    var fromName: String?
    var toName: String?
    var message: String?
    var status: FriendRequestStatus
    let createdAt: Date
    var respondedAt: Date?
    var avatar: String?

    init(id: UUID = UUID(),
         fromAddress: String,
         toAddress: String,
         fromName: String? = nil,
         toName: String? = nil,
         message: String? = nil,
         status: FriendRequestStatus = .pending,
         createdAt: Date = Date(),
         respondedAt: Date? = nil,
         avatar: String? = nil) {
        self.id = id
        self.fromAddress = fromAddress
        self.toAddress = toAddress
        self.fromName = fromName
        self.toName = toName
        self.message = message
        self.status = status
        self.createdAt = createdAt
        self.respondedAt = respondedAt
        self.avatar = avatar
    }

    var displayFromName: String {
        fromName ?? "\(fromAddress.prefix(6))...\(fromAddress.suffix(4))"
    }

    var displayToName: String {
        toName ?? "\(toAddress.prefix(6))...\(toAddress.suffix(4))"
    }
}

enum FriendRequestStatus: String, Codable {
    case pending = "Pending"
    case accepted = "Accepted"
    case declined = "Declined"
    case cancelled = "Cancelled"

    var color: String {
        switch self {
        case .pending: return "orange"
        case .accepted: return "green"
        case .declined: return "red"
        case .cancelled: return "gray"
        }
    }
}

// MARK: - Social Interaction
struct SocialInteraction: Identifiable, Codable {
    let id: UUID
    let type: InteractionType
    var fromAddress: String
    var toAddress: String
    var content: String?
    var timestamp: Date
    var metadata: [String: String]?

    init(id: UUID = UUID(),
         type: InteractionType,
         fromAddress: String,
         toAddress: String,
         content: String? = nil,
         timestamp: Date = Date(),
         metadata: [String: String]? = nil) {
        self.id = id
        self.type = type
        self.fromAddress = fromAddress
        self.toAddress = toAddress
        self.content = content
        self.timestamp = timestamp
        self.metadata = metadata
    }
}

enum InteractionType: String, Codable {
    case like
    case comment
    case share
    case mention
    case reaction
}

// MARK: - Notification
struct WalletNotification: Identifiable, Codable {
    let id: UUID
    let type: NotificationType
    var title: String
    var body: String
    var timestamp: Date
    var isRead: Bool
    var actionURL: String?
    var metadata: [String: String]?
    var priority: NotificationPriority

    init(id: UUID = UUID(),
         type: NotificationType,
         title: String,
         body: String,
         timestamp: Date = Date(),
         isRead: Bool = false,
         actionURL: String? = nil,
         metadata: [String: String]? = nil,
         priority: NotificationPriority = .normal) {
        self.id = id
        self.type = type
        self.title = title
        self.body = body
        self.timestamp = timestamp
        self.isRead = isRead
        self.actionURL = actionURL
        self.metadata = metadata
        self.priority = priority
    }

    var icon: String {
        type.icon
    }

    var color: String {
        type.color
    }
}

enum NotificationType: String, Codable {
    case transaction
    case paymentRequest
    case billSplit
    case friendRequest
    case system
    case security

    var icon: String {
        switch self {
        case .transaction: return "arrow.left.arrow.right"
        case .paymentRequest: return "paperplane"
        case .billSplit: return "doc.text"
        case .friendRequest: return "person.wave.2"
        case .system: return "bell"
        case .security: return "shield"
        }
    }

    var color: String {
        switch self {
        case .transaction: return "blue"
        case .paymentRequest: return "orange"
        case .billSplit: return "purple"
        case .friendRequest: return "green"
        case .system: return "gray"
        case .security: return "red"
        }
    }
}

enum NotificationPriority: String, Codable {
    case low
    case normal
    case high
    case urgent
}

// MARK: - Social Stats
struct SocialStats: Codable {
    var totalFriends: Int
    var totalTransactions: Int
    var totalAmountSent: String
    var totalAmountReceived: String
    var activeBills: Int
    var pendingRequests: Int

    init(totalFriends: Int = 0,
         totalTransactions: Int = 0,
         totalAmountSent: String = "0",
         totalAmountReceived: String = "0",
         activeBills: Int = 0,
         pendingRequests: Int = 0) {
        self.totalFriends = totalFriends
        self.totalTransactions = totalTransactions
        self.totalAmountSent = totalAmountSent
        self.totalAmountReceived = totalAmountReceived
        self.activeBills = activeBills
        self.pendingRequests = pendingRequests
    }

    var formattedAmountSent: String {
        let value = Double(totalAmountSent) ?? 0
        return String(format: "%.6f", value / 1e18)
    }

    var formattedAmountReceived: String {
        let value = Double(totalAmountReceived) ?? 0
        return String(format: "%.6f", value / 1e18)
    }
}
