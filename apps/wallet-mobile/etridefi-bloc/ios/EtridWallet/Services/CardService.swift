//
//  CardService.swift
//  EtridWallet
//
//  Service for crypto-backed card management and NFC payments
//

import Foundation
import Combine

@MainActor
class CardService: ObservableObject {
    static let shared = CardService()

    @Published var cards: [CryptoCard] = []
    @Published var activeCard: CryptoCard?
    @Published var recentTransactions: [CardTransaction] = []
    @Published var isLoading = false
    @Published var errorMessage: String?

    private let baseURL = "https://api.etrid.com/v1"
    private var cancellables = Set<AnyCancellable>()

    private init() {}

    // MARK: - Card Management

    func getCards(for userId: String) async throws -> [CryptoCard] {
        isLoading = true
        defer { isLoading = false }

        let url = URL(string: "\(baseURL)/cards/user/\(userId)")!
        var request = URLRequest(url: url)
        request.httpMethod = "GET"
        request.addValue("application/json", forHTTPHeaderField: "Content-Type")
        request.addValue("Bearer \(getAuthToken())", forHTTPHeaderField: "Authorization")

        let (data, response) = try await URLSession.shared.data(for: request)

        guard let httpResponse = response as? HTTPURLResponse,
              httpResponse.statusCode == 200 else {
            throw CardError.networkError
        }

        let cards = try JSONDecoder().decode([CryptoCard].self, from: data)
        self.cards = cards
        if let first = cards.first(where: { $0.isActive }) {
            self.activeCard = first
        }
        return cards
    }

    func createCard(userId: String, cardType: CardType, collateralAsset: String, collateralAmount: Double) async throws -> CryptoCard {
        isLoading = true
        defer { isLoading = false }

        let url = URL(string: "\(baseURL)/cards/create")!
        var request = URLRequest(url: url)
        request.httpMethod = "POST"
        request.addValue("application/json", forHTTPHeaderField: "Content-Type")
        request.addValue("Bearer \(getAuthToken())", forHTTPHeaderField: "Authorization")

        let body: [String: Any] = [
            "userId": userId,
            "cardType": cardType.rawValue,
            "collateralAsset": collateralAsset,
            "collateralAmount": collateralAmount
        ]
        request.httpBody = try JSONSerialization.data(withJSONObject: body)

        let (data, response) = try await URLSession.shared.data(for: request)

        guard let httpResponse = response as? HTTPURLResponse,
              httpResponse.statusCode == 201 else {
            throw CardError.cardCreationFailed
        }

        let card = try JSONDecoder().decode(CryptoCard.self, from: data)
        cards.append(card)
        return card
    }

    func freezeCard(cardId: String) async throws -> CryptoCard {
        try await updateCardStatus(cardId: cardId, status: .frozen)
    }

    func unfreezeCard(cardId: String) async throws -> CryptoCard {
        try await updateCardStatus(cardId: cardId, status: .active)
    }

    func blockCard(cardId: String) async throws -> CryptoCard {
        try await updateCardStatus(cardId: cardId, status: .blocked)
    }

    private func updateCardStatus(cardId: String, status: CardStatus) async throws -> CryptoCard {
        isLoading = true
        defer { isLoading = false }

        let url = URL(string: "\(baseURL)/cards/\(cardId)/status")!
        var request = URLRequest(url: url)
        request.httpMethod = "PUT"
        request.addValue("application/json", forHTTPHeaderField: "Content-Type")
        request.addValue("Bearer \(getAuthToken())", forHTTPHeaderField: "Authorization")

        let body: [String: Any] = ["status": status.rawValue]
        request.httpBody = try JSONSerialization.data(withJSONObject: body)

        let (data, response) = try await URLSession.shared.data(for: request)

        guard let httpResponse = response as? HTTPURLResponse,
              httpResponse.statusCode == 200 else {
            throw CardError.statusUpdateFailed
        }

        let updatedCard = try JSONDecoder().decode(CryptoCard.self, from: data)

        // Update local cache
        if let index = cards.firstIndex(where: { $0.id == cardId }) {
            cards[index] = updatedCard
        }
        if activeCard?.id == cardId {
            activeCard = updatedCard
        }

        return updatedCard
    }

    // MARK: - Transaction Management

    func getTransactions(for cardId: String, limit: Int = 50) async throws -> [CardTransaction] {
        isLoading = true
        defer { isLoading = false }

        let url = URL(string: "\(baseURL)/cards/\(cardId)/transactions?limit=\(limit)")!
        var request = URLRequest(url: url)
        request.httpMethod = "GET"
        request.addValue("application/json", forHTTPHeaderField: "Content-Type")
        request.addValue("Bearer \(getAuthToken())", forHTTPHeaderField: "Authorization")

        let (data, response) = try await URLSession.shared.data(for: request)

        guard let httpResponse = response as? HTTPURLResponse,
              httpResponse.statusCode == 200 else {
            throw CardError.networkError
        }

        let transactions = try JSONDecoder().decode([CardTransaction].self, from: data)
        self.recentTransactions = transactions
        return transactions
    }

    // MARK: - NFC Payment Processing

    func processNFCPayment(request: NFCPaymentRequest) async throws -> NFCPaymentResponse {
        // Validate card status
        guard let card = cards.first(where: { $0.id == request.cardId }) else {
            return NFCPaymentResponse(
                success: false,
                transactionId: nil,
                message: "Card not found",
                newBalance: nil,
                declineReason: PaymentDeclineReason.invalidMerchant.rawValue
            )
        }

        // Check if card is active
        guard card.isActive else {
            return NFCPaymentResponse(
                success: false,
                transactionId: nil,
                message: "Card is frozen or blocked",
                newBalance: nil,
                declineReason: card.isFrozen ? PaymentDeclineReason.cardFrozen.rawValue : PaymentDeclineReason.cardBlocked.rawValue
            )
        }

        // Check spending limits
        guard request.amount <= card.limits.perTransactionLimit else {
            return NFCPaymentResponse(
                success: false,
                transactionId: nil,
                message: "Transaction exceeds per-transaction limit",
                newBalance: nil,
                declineReason: PaymentDeclineReason.limitExceeded.rawValue
            )
        }

        guard request.amount <= card.limits.dailyRemaining else {
            return NFCPaymentResponse(
                success: false,
                transactionId: nil,
                message: "Daily limit exceeded",
                newBalance: nil,
                declineReason: PaymentDeclineReason.limitExceeded.rawValue
            )
        }

        guard request.amount <= card.collateral.availableAmount else {
            return NFCPaymentResponse(
                success: false,
                transactionId: nil,
                message: "Insufficient collateral",
                newBalance: nil,
                declineReason: PaymentDeclineReason.insufficientFunds.rawValue
            )
        }

        // Process payment through backend
        let url = URL(string: "\(baseURL)/cards/\(request.cardId)/pay")!
        var urlRequest = URLRequest(url: url)
        urlRequest.httpMethod = "POST"
        urlRequest.addValue("application/json", forHTTPHeaderField: "Content-Type")
        urlRequest.addValue("Bearer \(getAuthToken())", forHTTPHeaderField: "Authorization")

        let encoder = JSONEncoder()
        encoder.dateEncodingStrategy = .iso8601
        urlRequest.httpBody = try encoder.encode(request)

        let (data, response) = try await URLSession.shared.data(for: urlRequest)

        guard let httpResponse = response as? HTTPURLResponse else {
            throw CardError.networkError
        }

        let decoder = JSONDecoder()
        decoder.dateDecodingStrategy = .iso8601

        if httpResponse.statusCode == 200 {
            let paymentResponse = try decoder.decode(NFCPaymentResponse.self, from: data)

            // Refresh transactions after successful payment
            if paymentResponse.success {
                Task {
                    try? await getTransactions(for: request.cardId)
                    try? await getCards(for: card.userId)
                }
            }

            return paymentResponse
        } else {
            let paymentResponse = try decoder.decode(NFCPaymentResponse.self, from: data)
            return paymentResponse
        }
    }

    // MARK: - Card Controls

    func updateControls(cardId: String, controls: CardControls) async throws -> CardControls {
        let url = URL(string: "\(baseURL)/cards/\(cardId)/controls")!
        var request = URLRequest(url: url)
        request.httpMethod = "PUT"
        request.addValue("application/json", forHTTPHeaderField: "Content-Type")
        request.addValue("Bearer \(getAuthToken())", forHTTPHeaderField: "Authorization")

        request.httpBody = try JSONEncoder().encode(controls)

        let (data, response) = try await URLSession.shared.data(for: request)

        guard let httpResponse = response as? HTTPURLResponse,
              httpResponse.statusCode == 200 else {
            throw CardError.controlUpdateFailed
        }

        return try JSONDecoder().decode(CardControls.self, from: data)
    }

    func getControls(for cardId: String) async throws -> CardControls {
        let url = URL(string: "\(baseURL)/cards/\(cardId)/controls")!
        var request = URLRequest(url: url)
        request.httpMethod = "GET"
        request.addValue("application/json", forHTTPHeaderField: "Content-Type")
        request.addValue("Bearer \(getAuthToken())", forHTTPHeaderField: "Authorization")

        let (data, response) = try await URLSession.shared.data(for: request)

        guard let httpResponse = response as? HTTPURLResponse,
              httpResponse.statusCode == 200 else {
            throw CardError.networkError
        }

        return try JSONDecoder().decode(CardControls.self, from: data)
    }

    // MARK: - PIN Management

    func changePIN(cardId: String, currentPIN: String, newPIN: String) async throws -> Bool {
        let url = URL(string: "\(baseURL)/cards/\(cardId)/pin")!
        var request = URLRequest(url: url)
        request.httpMethod = "PUT"
        request.addValue("application/json", forHTTPHeaderField: "Content-Type")
        request.addValue("Bearer \(getAuthToken())", forHTTPHeaderField: "Authorization")

        let pinRequest = PINChangeRequest(cardId: cardId, currentPIN: currentPIN, newPIN: newPIN)
        request.httpBody = try JSONEncoder().encode(pinRequest)

        let (_, response) = try await URLSession.shared.data(for: request)

        guard let httpResponse = response as? HTTPURLResponse,
              httpResponse.statusCode == 200 else {
            throw CardError.pinChangeFailed
        }

        return true
    }

    // MARK: - Collateral Management

    func addCollateral(cardId: String, amount: Double) async throws -> CollateralInfo {
        let url = URL(string: "\(baseURL)/cards/\(cardId)/collateral/add")!
        var request = URLRequest(url: url)
        request.httpMethod = "POST"
        request.addValue("application/json", forHTTPHeaderField: "Content-Type")
        request.addValue("Bearer \(getAuthToken())", forHTTPHeaderField: "Authorization")

        let body: [String: Any] = ["amount": amount]
        request.httpBody = try JSONSerialization.data(withJSONObject: body)

        let (data, response) = try await URLSession.shared.data(for: request)

        guard let httpResponse = response as? HTTPURLResponse,
              httpResponse.statusCode == 200 else {
            throw CardError.collateralUpdateFailed
        }

        return try JSONDecoder().decode(CollateralInfo.self, from: data)
    }

    func withdrawCollateral(cardId: String, amount: Double) async throws -> CollateralInfo {
        let url = URL(string: "\(baseURL)/cards/\(cardId)/collateral/withdraw")!
        var request = URLRequest(url: url)
        request.httpMethod = "POST"
        request.addValue("application/json", forHTTPHeaderField: "Content-Type")
        request.addValue("Bearer \(getAuthToken())", forHTTPHeaderField: "Authorization")

        let body: [String: Any] = ["amount": amount]
        request.httpBody = try JSONSerialization.data(withJSONObject: body)

        let (data, response) = try await URLSession.shared.data(for: request)

        guard let httpResponse = response as? HTTPURLResponse,
              httpResponse.statusCode == 200 else {
            throw CardError.collateralUpdateFailed
        }

        return try JSONDecoder().decode(CollateralInfo.self, from: data)
    }

    // MARK: - Apple Wallet Integration

    func addToAppleWallet(cardId: String) async throws -> AppleWalletPass {
        let url = URL(string: "\(baseURL)/cards/\(cardId)/wallet/provision")!
        var request = URLRequest(url: url)
        request.httpMethod = "POST"
        request.addValue("application/json", forHTTPHeaderField: "Content-Type")
        request.addValue("Bearer \(getAuthToken())", forHTTPHeaderField: "Authorization")

        let (data, response) = try await URLSession.shared.data(for: request)

        guard let httpResponse = response as? HTTPURLResponse,
              httpResponse.statusCode == 200 else {
            throw CardError.walletProvisioningFailed
        }

        return try JSONDecoder().decode(AppleWalletPass.self, from: data)
    }

    // MARK: - Helper Methods

    private func getAuthToken() -> String {
        // TODO: Get from secure storage
        return UserDefaults.standard.string(forKey: "authToken") ?? ""
    }
}

// MARK: - Error Handling

enum CardError: LocalizedError {
    case networkError
    case cardCreationFailed
    case statusUpdateFailed
    case controlUpdateFailed
    case pinChangeFailed
    case collateralUpdateFailed
    case walletProvisioningFailed
    case invalidCard
    case insufficientCollateral

    var errorDescription: String? {
        switch self {
        case .networkError:
            return "Network connection failed. Please try again."
        case .cardCreationFailed:
            return "Failed to create card. Please check your collateral and try again."
        case .statusUpdateFailed:
            return "Failed to update card status."
        case .controlUpdateFailed:
            return "Failed to update card controls."
        case .pinChangeFailed:
            return "Failed to change PIN. Please check your current PIN."
        case .collateralUpdateFailed:
            return "Failed to update collateral."
        case .walletProvisioningFailed:
            return "Failed to add card to Apple Wallet."
        case .invalidCard:
            return "Invalid card."
        case .insufficientCollateral:
            return "Insufficient collateral for this operation."
        }
    }
}
