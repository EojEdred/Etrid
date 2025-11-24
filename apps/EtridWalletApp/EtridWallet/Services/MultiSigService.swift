//
//  MultiSigService.swift
//  EtridWallet
//
//  Service for multi-signature wallet operations
//

import Foundation
import Combine

@MainActor
class MultiSigService: ObservableObject {
    static let shared = MultiSigService()

    @Published var wallets: [MultiSigWallet] = []
    @Published var pendingTransactions: [String: [MultiSigPendingTransaction]] = [:] // walletId -> transactions
    @Published var isLoading = false
    @Published var error: Error?

    private let storageKey = "multisig_wallets"
    private let transactionsKey = "multisig_transactions"

    private init() {
        loadWallets()
        loadTransactions()
    }

    // MARK: - Wallet Management

    func createWallet(_ params: CreateMultiSigParams) async throws -> MultiSigWallet {
        guard params.isValid else {
            throw WalletError.invalidInput(params.validationError ?? "Invalid parameters")
        }

        isLoading = true
        defer { isLoading = false }

        // Generate wallet address (would deploy actual contract)
        let address = generateMultiSigAddress()

        let wallet = MultiSigWallet(
            id: UUID().uuidString,
            address: address,
            name: params.name,
            description: params.description,
            owners: params.owners,
            threshold: params.threshold,
            chainId: params.chainId,
            balance: "0",
            pendingTransactionCount: 0,
            createdAt: Date(),
            userRole: .creator
        )

        wallets.append(wallet)
        saveWallets()

        // TODO: Deploy multi-sig contract
        // let contractAddress = try await deployMultiSigContract(params)

        return wallet
    }

    func getWallets() -> [MultiSigWallet] {
        return wallets
    }

    func getWallet(id: String) -> MultiSigWallet? {
        wallets.first { $0.id == id }
    }

    func getWallet(address: String) -> MultiSigWallet? {
        wallets.first { $0.address.lowercased() == address.lowercased() }
    }

    func updateBalance(walletId: String) async throws {
        guard var wallet = getWallet(id: walletId) else {
            throw WalletError.unknown("Wallet not found")
        }

        // TODO: Fetch actual balance from blockchain
        let balance = "0"

        wallet.balance = balance
        updateWallet(wallet)
    }

    func deleteWallet(id: String) {
        wallets.removeAll { $0.id == id }
        pendingTransactions.removeValue(forKey: id)
        saveWallets()
        saveTransactions()
    }

    // MARK: - Transaction Management

    func proposeTransaction(_ params: ProposeTransactionParams) async throws -> MultiSigPendingTransaction {
        guard var wallet = getWallet(id: params.walletId) else {
            throw WalletError.unknown("Wallet not found")
        }

        guard wallet.isUserOwner else {
            throw WalletError.unknown("Only owners can propose transactions")
        }

        isLoading = true
        defer { isLoading = false }

        let nonce = (pendingTransactions[params.walletId]?.count ?? 0)

        let transaction = MultiSigPendingTransaction(
            id: UUID().uuidString,
            walletId: params.walletId,
            walletAddress: wallet.address,
            nonce: nonce,
            to: params.to,
            value: params.value,
            data: params.data,
            operation: params.operation,
            proposer: try await getCurrentAddress(),
            proposerName: "You",
            signatures: [],
            requiredSignatures: wallet.threshold,
            createdAt: Date(),
            expiresAt: params.expiryHours.map { Date().addingTimeInterval(TimeInterval($0 * 3600)) },
            status: .pending
        )

        var walletTransactions = pendingTransactions[params.walletId] ?? []
        walletTransactions.append(transaction)
        pendingTransactions[params.walletId] = walletTransactions

        wallet.pendingTransactionCount += 1
        updateWallet(wallet)

        saveTransactions()

        // TODO: Submit transaction proposal to smart contract

        return transaction
    }

    func getPendingTransactions(walletId: String) -> [MultiSigPendingTransaction] {
        pendingTransactions[walletId]?.filter { $0.status == .pending || $0.status == .readyToExecute } ?? []
    }

    func getTransaction(id: String, walletId: String) -> MultiSigPendingTransaction? {
        pendingTransactions[walletId]?.first { $0.id == id }
    }

    func signTransaction(transactionId: String, walletId: String) async throws {
        guard var transaction = getTransaction(id: transactionId, walletId: walletId) else {
            throw WalletError.unknown("Transaction not found")
        }

        guard transaction.status == .pending || transaction.status == .readyToExecute else {
            throw WalletError.invalidInput("Transaction cannot be signed")
        }

        let userAddress = try await getCurrentAddress()

        // Check if already signed
        guard !transaction.signatures.contains(where: { $0.signer.lowercased() == userAddress.lowercased() }) else {
            throw WalletError.invalidInput("Already signed this transaction")
        }

        isLoading = true
        defer { isLoading = false }

        // TODO: Generate actual signature using CryptoManager
        let signatureData = "0x" + String(repeating: "0", count: 130)

        let signature = TransactionSignature(
            id: UUID().uuidString,
            transactionId: transactionId,
            signer: userAddress,
            signerName: "You",
            signature: signatureData,
            signedAt: Date()
        )

        transaction.signatures.append(signature)

        // Update status if threshold reached
        if transaction.signatures.count >= transaction.requiredSignatures {
            transaction.status = .readyToExecute
        }

        updateTransaction(transaction, walletId: walletId)

        // TODO: Submit signature to smart contract
    }

    func executeTransaction(transactionId: String, walletId: String) async throws {
        guard var transaction = getTransaction(id: transactionId, walletId: walletId) else {
            throw WalletError.unknown("Transaction not found")
        }

        guard transaction.isReadyToExecute else {
            throw WalletError.invalidInput("Transaction not ready to execute")
        }

        isLoading = true
        defer { isLoading = false }

        // TODO: Execute transaction on-chain
        let txHash = "0x" + UUID().uuidString.replacingOccurrences(of: "-", with: "")

        transaction.status = .executed
        transaction.executedAt = Date()
        transaction.executionTxHash = txHash

        updateTransaction(transaction, walletId: walletId)

        // Update wallet pending count
        if var wallet = getWallet(id: walletId) {
            wallet.pendingTransactionCount = max(0, wallet.pendingTransactionCount - 1)
            updateWallet(wallet)
        }
    }

    func rejectTransaction(transactionId: String, walletId: String) async throws {
        guard var transaction = getTransaction(id: transactionId, walletId: walletId) else {
            throw WalletError.unknown("Transaction not found")
        }

        let userAddress = try await getCurrentAddress()
        guard var wallet = getWallet(id: walletId),
              wallet.owners.contains(where: { $0.lowercased() == userAddress.lowercased() }) else {
            throw WalletError.unknown("Only owners can reject transactions")
        }

        transaction.status = .rejected
        updateTransaction(transaction, walletId: walletId)

        wallet.pendingTransactionCount = max(0, wallet.pendingTransactionCount - 1)
        updateWallet(wallet)
    }

    // MARK: - Owner Management

    func getOwners(walletId: String) async throws -> [MultiSigOwner] {
        guard let wallet = getWallet(id: walletId) else {
            throw WalletError.unknown("Wallet not found")
        }

        // TODO: Fetch owner details from blockchain
        return wallet.owners.enumerated().map { index, address in
            MultiSigOwner(
                id: UUID().uuidString,
                address: address,
                name: index == 0 ? "You" : nil,
                addedAt: wallet.createdAt,
                transactionsSigned: 0,
                transactionsProposed: 0
            )
        }
    }

    func addOwner(walletId: String, ownerAddress: String, newThreshold: Int?) async throws {
        guard var wallet = getWallet(id: walletId) else {
            throw WalletError.unknown("Wallet not found")
        }

        guard wallet.userRole == .creator else {
            throw WalletError.unknown("Only creator can add owners")
        }

        guard !wallet.owners.contains(where: { $0.lowercased() == ownerAddress.lowercased() }) else {
            throw WalletError.invalidInput("Address is already an owner")
        }

        wallet.owners.append(ownerAddress)
        if let threshold = newThreshold {
            wallet.threshold = min(threshold, wallet.owners.count)
        }

        updateWallet(wallet)

        // TODO: Submit owner addition to smart contract
    }

    func removeOwner(walletId: String, ownerAddress: String, newThreshold: Int?) async throws {
        guard var wallet = getWallet(id: walletId) else {
            throw WalletError.unknown("Wallet not found")
        }

        guard wallet.userRole == .creator else {
            throw WalletError.unknown("Only creator can remove owners")
        }

        guard wallet.owners.count > wallet.threshold else {
            throw WalletError.invalidInput("Cannot remove owner: would fall below threshold")
        }

        wallet.owners.removeAll { $0.lowercased() == ownerAddress.lowercased() }
        if let threshold = newThreshold {
            wallet.threshold = min(threshold, wallet.owners.count)
        }

        updateWallet(wallet)

        // TODO: Submit owner removal to smart contract
    }

    func changeThreshold(walletId: String, newThreshold: Int) async throws {
        guard var wallet = getWallet(id: walletId) else {
            throw WalletError.unknown("Wallet not found")
        }

        guard wallet.userRole == .creator else {
            throw WalletError.unknown("Only creator can change threshold")
        }

        guard newThreshold > 0 && newThreshold <= wallet.owners.count else {
            throw WalletError.invalidInput("Invalid threshold")
        }

        wallet.threshold = newThreshold
        updateWallet(wallet)

        // TODO: Submit threshold change to smart contract
    }

    // MARK: - Stats

    func getStats(walletId: String) async throws -> MultiSigStats {
        let transactions = pendingTransactions[walletId] ?? []
        let total = transactions.count
        let executed = transactions.filter { $0.status == .executed }.count
        let pending = transactions.filter { $0.status == .pending || $0.status == .readyToExecute }.count
        let rejected = transactions.filter { $0.status == .rejected }.count

        return MultiSigStats(
            totalTransactions: total,
            executedTransactions: executed,
            pendingTransactions: pending,
            rejectedTransactions: rejected,
            totalValue: "0",
            averageSigningTime: 0
        )
    }

    // MARK: - Helpers

    private func updateWallet(_ wallet: MultiSigWallet) {
        if let index = wallets.firstIndex(where: { $0.id == wallet.id }) {
            wallets[index] = wallet
            saveWallets()
        }
    }

    private func updateTransaction(_ transaction: MultiSigPendingTransaction, walletId: String) {
        if var walletTransactions = pendingTransactions[walletId],
           let index = walletTransactions.firstIndex(where: { $0.id == transaction.id }) {
            walletTransactions[index] = transaction
            pendingTransactions[walletId] = walletTransactions
            saveTransactions()
        }
    }

    private func generateMultiSigAddress() -> String {
        "0x" + UUID().uuidString.replacingOccurrences(of: "-", with: "").prefix(40)
    }

    private func getCurrentAddress() async throws -> String {
        // TODO: Get from WalletManager
        return "0x742d35Cc6634C0532925a3b844Bc454e4438f44e"
    }

    // MARK: - Storage

    private func saveWallets() {
        if let encoded = try? JSONEncoder().encode(wallets) {
            UserDefaults.standard.set(encoded, forKey: storageKey)
        }
    }

    private func loadWallets() {
        if let data = UserDefaults.standard.data(forKey: storageKey),
           let decoded = try? JSONDecoder().decode([MultiSigWallet].self, from: data) {
            wallets = decoded
        }
    }

    private func saveTransactions() {
        if let encoded = try? JSONEncoder().encode(pendingTransactions) {
            UserDefaults.standard.set(encoded, forKey: transactionsKey)
        }
    }

    private func loadTransactions() {
        if let data = UserDefaults.standard.data(forKey: transactionsKey),
           let decoded = try? JSONDecoder().decode([String: [MultiSigPendingTransaction]].self, from: data) {
            pendingTransactions = decoded
        }
    }
}
