//
//  Contact.swift
//  EtridWallet
//
//  Contact models for social and payments features
//

import Foundation

// MARK: - Contact
struct Contact: Identifiable, Codable, Equatable, Hashable {
    let id: UUID
    var name: String
    var address: String
    var email: String?
    var phone: String?
    var avatar: String? // URL or emoji
    var isFavorite: Bool
    var tags: [String]
    let createdAt: Date
    var lastInteraction: Date?
    var notes: String?
    var network: String? // Preferred network

    init(id: UUID = UUID(),
         name: String,
         address: String,
         email: String? = nil,
         phone: String? = nil,
         avatar: String? = nil,
         isFavorite: Bool = false,
         tags: [String] = [],
         createdAt: Date = Date(),
         lastInteraction: Date? = nil,
         notes: String? = nil,
         network: String? = nil) {
        self.id = id
        self.name = name
        self.address = address
        self.email = email
        self.phone = phone
        self.avatar = avatar
        self.isFavorite = isFavorite
        self.tags = tags
        self.createdAt = createdAt
        self.lastInteraction = lastInteraction
        self.notes = notes
        self.network = network
    }

    var initials: String {
        let components = name.components(separatedBy: " ")
        let firstInitial = components.first?.prefix(1).uppercased() ?? ""
        let lastInitial = components.count > 1 ? components.last?.prefix(1).uppercased() ?? "" : ""
        return firstInitial + lastInitial
    }

    var shortAddress: String {
        guard address.count > 10 else { return address }
        return "\(address.prefix(6))...\(address.suffix(4))"
    }

    func hash(into hasher: inout Hasher) {
        hasher.combine(id)
    }

    static func == (lhs: Contact, rhs: Contact) -> Bool {
        lhs.id == rhs.id
    }
}

// MARK: - Recent Contact
struct RecentContact: Identifiable, Codable, Equatable {
    let id: UUID
    let contactId: UUID?
    let address: String
    let name: String?
    let lastTransactionDate: Date
    let transactionCount: Int
    let totalAmountSent: String
    let totalAmountReceived: String

    init(id: UUID = UUID(),
         contactId: UUID? = nil,
         address: String,
         name: String? = nil,
         lastTransactionDate: Date = Date(),
         transactionCount: Int = 1,
         totalAmountSent: String = "0",
         totalAmountReceived: String = "0") {
        self.id = id
        self.contactId = contactId
        self.address = address
        self.name = name
        self.lastTransactionDate = lastTransactionDate
        self.transactionCount = transactionCount
        self.totalAmountSent = totalAmountSent
        self.totalAmountReceived = totalAmountReceived
    }

    var displayName: String {
        name ?? "\(address.prefix(6))...\(address.suffix(4))"
    }
}

// MARK: - Contact Transaction
struct ContactTransaction: Identifiable, Codable {
    let id: UUID
    let contactId: UUID
    let transactionHash: String
    let type: TransactionDirection
    let amount: String
    let tokenSymbol: String
    let timestamp: Date
    let status: TransactionStatus

    enum TransactionDirection: String, Codable {
        case sent
        case received
    }

    var formattedAmount: String {
        let value = Double(amount) ?? 0
        let divisor = pow(10.0, 18.0)
        let amountValue = value / divisor
        let sign = type == .sent ? "-" : "+"
        return "\(sign)\(String(format: "%.6f", amountValue)) \(tokenSymbol)"
    }
}

// MARK: - Contact Group
struct ContactGroup: Identifiable, Codable {
    let id: UUID
    var name: String
    var contactIds: [UUID]
    var color: String
    let createdAt: Date

    init(id: UUID = UUID(),
         name: String,
         contactIds: [UUID] = [],
         color: String = "blue",
         createdAt: Date = Date()) {
        self.id = id
        self.name = name
        self.contactIds = contactIds
        self.color = color
        self.createdAt = createdAt
    }
}
