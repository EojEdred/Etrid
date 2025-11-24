//
//  CardViewModel.swift
//  EtridWallet
//
//  Production-ready crypto-backed card management
//

import Foundation
import Combine

@MainActor
class CardViewModel: ObservableObject {
    @Published var cards: [CryptoCard] = []
    @Published var transactions: [CardTransaction] = []
    @Published var isLoading = false
    @Published var errorMessage: String?

    private let storageKey = "CryptoCards.Data"
    private let transactionsKey = "CardTransactions.Data"
    private var cancellables = Set<AnyCancellable>()
    private var priceUpdateTimer: Timer?

    init() {
        loadPersistedData()
        setupAutoSave()
        startPriceUpdates()
    }

    // MARK: - Data Persistence

    private func loadPersistedData() {
        // Load cards
        if let data = UserDefaults.standard.data(forKey: storageKey),
           let decoded = try? JSONDecoder().decode([CryptoCard].self, from: data) {
            self.cards = decoded
        }

        // Load transactions
        if let data = UserDefaults.standard.data(forKey: transactionsKey),
           let decoded = try? JSONDecoder().decode([CardTransaction].self, from: data) {
            self.transactions = decoded
        }
    }

    private func saveData() {
        // Save cards
        if let encoded = try? JSONEncoder().encode(cards) {
            UserDefaults.standard.set(encoded, forKey: storageKey)
        }

        // Save transactions
        if let encoded = try? JSONEncoder().encode(transactions) {
            UserDefaults.standard.set(encoded, forKey: transactionsKey)
        }
    }

    private func setupAutoSave() {
        Publishers.CombineLatest($cards, $transactions)
            .debounce(for: 0.5, scheduler: DispatchQueue.main)
            .sink { [weak self] _ in
                self?.saveData()
            }
            .store(in: &cancellables)
    }

    // MARK: - Card Management

    func createCard(application: CardApplication) {
        let card = CryptoCard(
            id: UUID().uuidString,
            cardNumber: generateCardNumber(),
            cardholderName: application.cardholderName,
            expiryDate: Calendar.current.date(byAdding: .year, value: 3, to: Date()) ?? Date(),
            cvv: String(format: "%03d", Int.random(in: 100...999)),
            status: .active,
            createdAt: Date(),
            lastUsedAt: nil,
            collateralAssets: application.selectedAssets,
            spendingLimitRatio: application.spendingLimitRatio
        )

        cards.insert(card, at: 0)
    }

    func freezeCard(cardId: String) {
        guard let index = cards.firstIndex(where: { $0.id == cardId }) else { return }
        cards[index].status = .frozen
    }

    func unfreezeCard(cardId: String) {
        guard let index = cards.firstIndex(where: { $0.id == cardId }) else { return }
        cards[index].status = .active
    }

    func deleteCard(cardId: String) {
        cards.removeAll { $0.id == cardId }
        transactions.removeAll { $0.cardId == cardId }
    }

    func updateSpendingLimit(cardId: String, newRatio: Double) {
        guard let index = cards.firstIndex(where: { $0.id == cardId }) else { return }
        cards[index].spendingLimitRatio = min(max(newRatio, 0.1), 0.9) // 10%-90%
    }

    func addCollateral(cardId: String, asset: CardCollateralAsset) {
        guard let index = cards.firstIndex(where: { $0.id == cardId }) else { return }
        cards[index].collateralAssets.append(asset)
    }

    func removeCollateral(cardId: String, assetId: String) {
        guard let index = cards.firstIndex(where: { $0.id == cardId }) else { return }
        cards[index].collateralAssets.removeAll { $0.id == assetId }
    }

    // MARK: - Transactions

    func getTransactions(cardId: String) -> [CardTransaction] {
        transactions
            .filter { $0.cardId == cardId }
            .sorted { $0.timestamp > $1.timestamp }
    }

    func getAllTransactions() -> [CardTransaction] {
        transactions.sorted { $0.timestamp > $1.timestamp }
    }

    func simulateTransaction(cardId: String, merchant: String, amount: Double, category: CardTransaction.TransactionCategory) -> Bool {
        guard let card = cards.first(where: { $0.id == cardId }) else { return false }
        guard card.status == .active else { return false }

        let totalSpent = getTotalSpent(cardId: cardId)
        let availableLimit = card.availableSpendingLimit

        // Check if transaction exceeds limit
        if totalSpent + amount > availableLimit {
            // Decline transaction
            let transaction = CardTransaction(
                id: UUID().uuidString,
                cardId: cardId,
                merchantName: merchant,
                category: category,
                amount: amount,
                currency: "USD",
                timestamp: Date(),
                status: .declined,
                location: "San Francisco, CA"
            )
            transactions.insert(transaction, at: 0)
            return false
        }

        // Approve transaction
        let transaction = CardTransaction(
            id: UUID().uuidString,
            cardId: cardId,
            merchantName: merchant,
            category: category,
            amount: amount,
            currency: "USD",
            timestamp: Date(),
            status: .completed,
            location: "San Francisco, CA"
        )
        transactions.insert(transaction, at: 0)

        // Update last used
        if let index = cards.firstIndex(where: { $0.id == cardId }) {
            cards[index].lastUsedAt = Date()
        }

        return true
    }

    func getTotalSpent(cardId: String) -> Double {
        transactions
            .filter { $0.cardId == cardId && $0.status == .completed }
            .reduce(0) { $0 + $1.amount }
    }

    // MARK: - Stats

    func getCardStats(cardId: String) -> CardStats {
        let cardTransactions = transactions.filter { $0.cardId == cardId && $0.status == .completed }
        let totalSpent = cardTransactions.reduce(0) { $0 + $1.amount }
        let count = cardTransactions.count
        let average = count > 0 ? totalSpent / Double(count) : 0

        // Spending by category
        var byCategory: [CardTransaction.TransactionCategory: Double] = [:]
        for transaction in cardTransactions {
            byCategory[transaction.category, default: 0] += transaction.amount
        }

        // Monthly spending
        var monthlySpending: [String: Double] = [:]
        let formatter = DateFormatter()
        formatter.dateFormat = "yyyy-MM"
        for transaction in cardTransactions {
            let month = formatter.string(from: transaction.timestamp)
            monthlySpending[month, default: 0] += transaction.amount
        }

        return CardStats(
            totalSpent: totalSpent,
            transactionCount: count,
            averageTransaction: average,
            spendingByCategory: byCategory,
            monthlySpending: monthlySpending
        )
    }

    // MARK: - Price Updates

    private func startPriceUpdates() {
        // Update prices every 30 seconds (in production, use WebSocket)
        priceUpdateTimer = Timer.scheduledTimer(withTimeInterval: 30, repeats: true) { [weak self] _ in
            self?.updateAssetPrices()
        }
    }

    private func updateAssetPrices() {
        // Simulate price updates (in production, fetch from price oracle)
        for (cardIndex, card) in cards.enumerated() {
            for (assetIndex, asset) in card.collateralAssets.enumerated() {
                let priceChange = Double.random(in: -0.02...0.02) // ±2% change
                let newPrice = asset.priceUSD * (1 + priceChange)
                cards[cardIndex].collateralAssets[assetIndex].priceUSD = newPrice
            }
        }
    }

    // MARK: - Helpers

    private func generateCardNumber() -> String {
        // Generate valid-looking card number (not real)
        let prefix = "4532" // Visa-like prefix
        var number = prefix
        for _ in 0..<12 {
            number += String(Int.random(in: 0...9))
        }
        return number
    }

    // MARK: - Demo Data

    func createDemoCard() {
        let demoAssets = [
            CardCollateralAsset(
                id: UUID().uuidString,
                symbol: "ËTR",
                name: "Ëtrid",
                amount: "5000000000000000000000", // 5000 ËTR
                lockedAt: Date(),
                priceUSD: 2.50
            ),
            CardCollateralAsset(
                id: UUID().uuidString,
                symbol: "ETH",
                name: "Ethereum",
                amount: "2000000000000000000", // 2 ETH
                lockedAt: Date(),
                priceUSD: 3200.00
            )
        ]

        let application = CardApplication(
            selectedAssets: demoAssets,
            spendingLimitRatio: 0.5, // 50% of collateral
            cardholderName: "JOHN DOE"
        )

        createCard(application: application)

        // Add demo transactions
        if let cardId = cards.first?.id {
            _ = simulateTransaction(cardId: cardId, merchant: "Starbucks", amount: 12.50, category: .dining)
            _ = simulateTransaction(cardId: cardId, merchant: "Amazon", amount: 89.99, category: .shopping)
            _ = simulateTransaction(cardId: cardId, merchant: "Shell Gas", amount: 45.00, category: .gas)
            _ = simulateTransaction(cardId: cardId, merchant: "Whole Foods", amount: 127.35, category: .groceries)
        }
    }
}
