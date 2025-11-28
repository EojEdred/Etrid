//
//  BillSplit.swift
//  EtridWallet
//
//  Bill splitting and payment request models
//

import Foundation

// MARK: - Split Type
enum SplitType: String, Codable, CaseIterable {
    case even = "Split Evenly"
    case custom = "Custom Amounts"
    case percentage = "Percentage"
    case shares = "Shares"

    var icon: String {
        switch self {
        case .even: return "equal.square"
        case .custom: return "slider.horizontal.3"
        case .percentage: return "percent"
        case .shares: return "chart.pie"
        }
    }
}

// MARK: - Payment Status
enum PaymentStatus: String, Codable {
    case pending = "Pending"
    case paid = "Paid"
    case declined = "Declined"
    case expired = "Expired"

    var color: String {
        switch self {
        case .pending: return "orange"
        case .paid: return "green"
        case .declined: return "red"
        case .expired: return "gray"
        }
    }
}

// MARK: - Participant
struct Participant: Identifiable, Codable, Equatable {
    let id: UUID
    var contactId: UUID?
    var name: String
    var address: String
    var amount: String // Amount in wei
    var percentage: Double?
    var shares: Int?
    var status: PaymentStatus
    var paidAt: Date?
    var transactionHash: String?

    init(id: UUID = UUID(),
         contactId: UUID? = nil,
         name: String,
         address: String,
         amount: String = "0",
         percentage: Double? = nil,
         shares: Int? = nil,
         status: PaymentStatus = .pending,
         paidAt: Date? = nil,
         transactionHash: String? = nil) {
        self.id = id
        self.contactId = contactId
        self.name = name
        self.address = address
        self.amount = amount
        self.percentage = percentage
        self.shares = shares
        self.status = status
        self.paidAt = paidAt
        self.transactionHash = transactionHash
    }

    var formattedAmount: String {
        let value = Double(amount) ?? 0
        return String(format: "%.6f", value / 1e18)
    }

    var shortAddress: String {
        guard address.count > 10 else { return address }
        return "\(address.prefix(6))...\(address.suffix(4))"
    }
}

// MARK: - Split Bill
struct SplitBill: Identifiable, Codable {
    let id: UUID
    var title: String
    var description: String?
    var totalAmount: String // Amount in wei
    var tokenSymbol: String
    var tokenAddress: String?
    var splitType: SplitType
    var participants: [Participant]
    var creatorAddress: String
    let createdAt: Date
    var expiresAt: Date?
    var isActive: Bool
    var category: BillCategory?
    var imageURL: String?

    init(id: UUID = UUID(),
         title: String,
         description: String? = nil,
         totalAmount: String,
         tokenSymbol: String = "ETR",
         tokenAddress: String? = nil,
         splitType: SplitType = .even,
         participants: [Participant] = [],
         creatorAddress: String,
         createdAt: Date = Date(),
         expiresAt: Date? = nil,
         isActive: Bool = true,
         category: BillCategory? = nil,
         imageURL: String? = nil) {
        self.id = id
        self.title = title
        self.description = description
        self.totalAmount = totalAmount
        self.tokenSymbol = tokenSymbol
        self.tokenAddress = tokenAddress
        self.splitType = splitType
        self.participants = participants
        self.creatorAddress = creatorAddress
        self.createdAt = createdAt
        self.expiresAt = expiresAt
        self.isActive = isActive
        self.category = category
        self.imageURL = imageURL
    }

    var formattedTotalAmount: String {
        let value = Double(totalAmount) ?? 0
        return String(format: "%.6f", value / 1e18)
    }

    var paidCount: Int {
        participants.filter { $0.status == .paid }.count
    }

    var totalPaid: String {
        let total = participants
            .filter { $0.status == .paid }
            .compactMap { Double($0.amount) }
            .reduce(0, +)
        return String(UInt64(total))
    }

    var totalPending: String {
        let total = participants
            .filter { $0.status == .pending }
            .compactMap { Double($0.amount) }
            .reduce(0, +)
        return String(UInt64(total))
    }

    var completionPercentage: Double {
        guard participants.count > 0 else { return 0 }
        return Double(paidCount) / Double(participants.count) * 100
    }

    var isCompleted: Bool {
        paidCount == participants.count
    }

    var isExpired: Bool {
        guard let expiresAt = expiresAt else { return false }
        return Date() > expiresAt
    }
}

// MARK: - Bill Category
enum BillCategory: String, Codable, CaseIterable {
    case food = "Food & Dining"
    case entertainment = "Entertainment"
    case travel = "Travel"
    case utilities = "Utilities"
    case shopping = "Shopping"
    case rent = "Rent"
    case other = "Other"

    var icon: String {
        switch self {
        case .food: return "fork.knife"
        case .entertainment: return "ticket"
        case .travel: return "airplane"
        case .utilities: return "bolt"
        case .shopping: return "bag"
        case .rent: return "house"
        case .other: return "ellipsis.circle"
        }
    }

    var color: String {
        switch self {
        case .food: return "orange"
        case .entertainment: return "purple"
        case .travel: return "blue"
        case .utilities: return "yellow"
        case .shopping: return "pink"
        case .rent: return "green"
        case .other: return "gray"
        }
    }
}

// MARK: - Payment Request
struct PaymentRequest: Identifiable, Codable {
    let id: UUID
    var fromAddress: String
    var toAddress: String
    var fromName: String?
    var toName: String?
    var amount: String
    var tokenSymbol: String
    var tokenAddress: String?
    var message: String?
    var status: PaymentStatus
    let createdAt: Date
    var expiresAt: Date?
    var paidAt: Date?
    var transactionHash: String?
    var billId: UUID? // Reference to split bill if part of one

    init(id: UUID = UUID(),
         fromAddress: String,
         toAddress: String,
         fromName: String? = nil,
         toName: String? = nil,
         amount: String,
         tokenSymbol: String = "ETR",
         tokenAddress: String? = nil,
         message: String? = nil,
         status: PaymentStatus = .pending,
         createdAt: Date = Date(),
         expiresAt: Date? = nil,
         paidAt: Date? = nil,
         transactionHash: String? = nil,
         billId: UUID? = nil) {
        self.id = id
        self.fromAddress = fromAddress
        self.toAddress = toAddress
        self.fromName = fromName
        self.toName = toName
        self.amount = amount
        self.tokenSymbol = tokenSymbol
        self.tokenAddress = tokenAddress
        self.message = message
        self.status = status
        self.createdAt = createdAt
        self.expiresAt = expiresAt
        self.paidAt = paidAt
        self.transactionHash = transactionHash
        self.billId = billId
    }

    var formattedAmount: String {
        let value = Double(amount) ?? 0
        return String(format: "%.6f", value / 1e18)
    }

    var isExpired: Bool {
        guard let expiresAt = expiresAt else { return false }
        return Date() > expiresAt
    }

    var displayFromName: String {
        fromName ?? "\(fromAddress.prefix(6))...\(fromAddress.suffix(4))"
    }

    var displayToName: String {
        toName ?? "\(toAddress.prefix(6))...\(toAddress.suffix(4))"
    }
}

// MARK: - Settlement
struct Settlement: Identifiable, Codable {
    let id: UUID
    let billId: UUID
    let participantId: UUID
    let amount: String
    let transactionHash: String
    let timestamp: Date
    var confirmed: Bool

    var formattedAmount: String {
        let value = Double(amount) ?? 0
        return String(format: "%.6f", value / 1e18)
    }
}
