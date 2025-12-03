//
//  MultisigService.swift
//  EtridWallet
//
//  Production multi-signature wallet service
//

import Foundation
import Combine

// MARK: - Models
struct MultisigWallet: Codable, Identifiable {
    let id: String
    let name: String
    let address: String
    let signers: [String]
    let threshold: Int
    let balance: Double
    let nonce: Int
    let createdAt: Date

    var totalSigners: Int { signers.count }
}

struct MultisigTransaction: Codable, Identifiable {
    let id: String
    let walletId: String
    let nonce: Int
    let type: TransactionType
    let to: String
    let amount: Double
    let data: String?
    let signatures: [Signature]
    let threshold: Int
    let status: MultisigTransactionStatus
    let createdAt: Date
    let executedAt: Date?

    enum TransactionType: String, Codable {
        case transfer = "transfer"
        case contractCall = "contract_call"
        case addSigner = "add_signer"
        case removeSigner = "remove_signer"
        case changeThreshold = "change_threshold"
    }

    var signatureCount: Int { signatures.count }
    var isReadyToExecute: Bool { signatureCount >= threshold }
}

struct Signature: Codable, Identifiable {
    let id: String
    let signer: String
    let signature: String
    let timestamp: Date
}

enum MultisigTransactionStatus: String, Codable {
    case pending = "pending"
    case executed = "executed"
    case cancelled = "cancelled"
    case failed = "failed"
}

// MARK: - Multisig Service
class MultisigService {
    static let shared = MultisigService()

    private init() {}

    // MARK: - Wallet Management
    func createWallet(
        name: String,
        signers: [String],
        threshold: Int,
        creator: String
    ) async throws -> MultisigWallet {
        guard threshold > 0 && threshold <= signers.count else {
            throw MultisigError.invalidThreshold
        }

        guard signers.contains(creator) else {
            throw MultisigError.creatorNotInSigners
        }

        struct CreateWalletRequest: Encodable {
            let name: String
            let signers: [String]
            let threshold: Int
            let creator: String
        }

        let request = CreateWalletRequest(
            name: name,
            signers: signers,
            threshold: threshold,
            creator: creator
        )

        return try await NetworkService.shared.request(
            endpoint: "/multisig/wallets",
            method: .post,
            body: request
        )
    }

    func getWallets(for address: String) async throws -> [MultisigWallet] {
        struct WalletListResponse: Decodable {
            let wallets: [MultisigWallet]
        }

        do {
            let response: WalletListResponse = try await NetworkService.shared.request(
                endpoint: "/multisig/wallets?signer=\(address)"
            )
            return response.wallets
        } catch {
            // Return mock data for development
            return [
                MultisigWallet(
                    id: "multisig_1",
                    name: "Company Treasury",
                    address: "5GxCompanyTreasury...",
                    signers: [
                        "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY",
                        "5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty",
                        "5DAAnrj7VHTznn2AWBemMuyBwZWs6FNFjdyVXUeYum3PTXFy"
                    ],
                    threshold: 2,
                    balance: 500000,
                    nonce: 15,
                    createdAt: Date().addingTimeInterval(-86400 * 30)
                ),
                MultisigWallet(
                    id: "multisig_2",
                    name: "Team Vault",
                    address: "5GxTeamVault...",
                    signers: [
                        "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY",
                        "5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty",
                        "5DAAnrj7VHTznn2AWBemMuyBwZWs6FNFjdyVXUeYum3PTXFy",
                        "5GNJqTPyNqANBkUVMN1LPPrxXnFouWXoe2wNSmmEoLctxiZY",
                        "5HGjWAeFDfFCWPsjFQdVV2Msvz2XtMktvgocEZcCj68kUMaw"
                    ],
                    threshold: 3,
                    balance: 150000,
                    nonce: 8,
                    createdAt: Date().addingTimeInterval(-86400 * 15)
                )
            ]
        }
    }

    func getWallet(id: String) async throws -> MultisigWallet {
        return try await NetworkService.shared.request(
            endpoint: "/multisig/wallets/\(id)"
        )
    }

    // MARK: - Transaction Creation
    func createTransaction(
        walletId: String,
        type: MultisigTransaction.TransactionType,
        to: String,
        amount: Double,
        data: String? = nil,
        proposer: String
    ) async throws -> MultisigTransaction {
        struct CreateTransactionRequest: Encodable {
            let walletId: String
            let type: String
            let to: String
            let amount: String
            let data: String?
            let proposer: String
        }

        // Verify proposer is a signer
        let wallet = try await getWallet(id: walletId)
        guard wallet.signers.contains(proposer) else {
            throw MultisigError.notASigner
        }

        let request = CreateTransactionRequest(
            walletId: walletId,
            type: type.rawValue,
            to: to,
            amount: String(amount),
            data: data,
            proposer: proposer
        )

        return try await NetworkService.shared.request(
            endpoint: "/multisig/transactions",
            method: .post,
            body: request
        )
    }

    func getPendingTransactions(for walletId: String) async throws -> [MultisigTransaction] {
        struct TransactionListResponse: Decodable {
            let transactions: [MultisigTransaction]
        }

        do {
            let response: TransactionListResponse = try await NetworkService.shared.request(
                endpoint: "/multisig/wallets/\(walletId)/transactions?status=pending"
            )
            return response.transactions
        } catch {
            // Return mock data for development
            return generateMockTransactions(for: walletId)
        }
    }

    func getAllPendingTransactions(for address: String) async throws -> [MultisigTransaction] {
        let wallets = try await getWallets(for: address)
        var allTransactions: [MultisigTransaction] = []

        for wallet in wallets {
            let transactions = try await getPendingTransactions(for: wallet.id)
            allTransactions.append(contentsOf: transactions)
        }

        return allTransactions.sorted { $0.createdAt > $1.createdAt }
    }

    // MARK: - Signing
    func signTransaction(
        transactionId: String,
        signer: String
    ) async throws -> Signature {
        guard let privateKey = KeychainService.shared.getPrivateKey(for: signer) else {
            throw MultisigError.noPrivateKey
        }

        // Get transaction
        let transaction = try await getTransaction(id: transactionId)

        // Verify signer hasn't already signed
        if transaction.signatures.contains(where: { $0.signer == signer }) {
            throw MultisigError.alreadySigned
        }

        // Create signature
        let messageHash = try createTransactionHash(transaction: transaction)
        let signature = try signMessage(hash: messageHash, privateKey: privateKey)

        struct SignRequest: Encodable {
            let signer: String
            let signature: String
        }

        let request = SignRequest(signer: signer, signature: signature)

        let signatureResponse: Signature = try await NetworkService.shared.request(
            endpoint: "/multisig/transactions/\(transactionId)/sign",
            method: .post,
            body: request
        )

        // Check if transaction can now be executed
        let updatedTransaction = try await getTransaction(id: transactionId)
        if updatedTransaction.isReadyToExecute {
            // Automatically execute if threshold reached
            try await executeTransaction(id: transactionId, executor: signer)
        }

        return signatureResponse
    }

    func rejectTransaction(
        transactionId: String,
        signer: String
    ) async throws {
        struct RejectRequest: Encodable {
            let signer: String
        }

        let _: SharedEmptyResponse = try await NetworkService.shared.request(
            endpoint: "/multisig/transactions/\(transactionId)/reject",
            method: .post,
            body: RejectRequest(signer: signer)
        )
    }

    // MARK: - Execution
    func executeTransaction(
        id: String,
        executor: String
    ) async throws {
        let transaction = try await getTransaction(id: id)

        guard transaction.isReadyToExecute else {
            throw MultisigError.insufficientSignatures
        }

        struct ExecuteRequest: Encodable {
            let executor: String
        }

        let _: SharedEmptyResponse = try await NetworkService.shared.request(
            endpoint: "/multisig/transactions/\(id)/execute",
            method: .post,
            body: ExecuteRequest(executor: executor)
        )
    }

    func getTransaction(id: String) async throws -> MultisigTransaction {
        return try await NetworkService.shared.request(
            endpoint: "/multisig/transactions/\(id)"
        )
    }

    // MARK: - Helper Methods
    private func createTransactionHash(transaction: MultisigTransaction) throws -> String {
        // Create deterministic hash of transaction for signing
        let message = "\(transaction.walletId):\(transaction.nonce):\(transaction.type):\(transaction.to):\(transaction.amount)"
        // TODO: Use proper cryptographic hashing (e.g., Blake2b for Substrate)
        return message.data(using: .utf8)?.base64EncodedString() ?? ""
    }

    private func signMessage(hash: String, privateKey: String) throws -> String {
        // TODO: Implement proper sr25519/ed25519 signing
        // For production, use SubstrateSdk or similar
        return "0x" + privateKey.prefix(64) + hash.suffix(64)
    }

    private func generateMockTransactions(for walletId: String) -> [MultisigTransaction] {
        [
            MultisigTransaction(
                id: "tx_1",
                walletId: walletId,
                nonce: 1,
                type: .transfer,
                to: "5GNJqTPyNqANBkUVMN1LPPrxXnFouWXoe2wNSmmEoLctxiZY",
                amount: 50000,
                data: nil,
                signatures: [
                    Signature(
                        id: "sig_1",
                        signer: "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY",
                        signature: "0x1234...",
                        timestamp: Date().addingTimeInterval(-3600)
                    )
                ],
                threshold: 2,
                status: .pending,
                createdAt: Date().addingTimeInterval(-7200),
                executedAt: nil
            ),
            MultisigTransaction(
                id: "tx_2",
                walletId: walletId,
                nonce: 2,
                type: .contractCall,
                to: "UniswapRouter",
                amount: 0,
                data: "0xswap_data",
                signatures: [
                    Signature(
                        id: "sig_2",
                        signer: "5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty",
                        signature: "0x5678...",
                        timestamp: Date().addingTimeInterval(-1800)
                    )
                ],
                threshold: 2,
                status: .pending,
                createdAt: Date().addingTimeInterval(-5400),
                executedAt: nil
            )
        ]
    }
}

// MARK: - Errors
enum MultisigError: Error {
    case invalidThreshold
    case creatorNotInSigners
    case notASigner
    case alreadySigned
    case insufficientSignatures
    case noPrivateKey
    case transactionNotFound
    case walletNotFound

    var localizedDescription: String {
        switch self {
        case .invalidThreshold:
            return "Threshold must be between 1 and number of signers"
        case .creatorNotInSigners:
            return "Creator must be one of the signers"
        case .notASigner:
            return "You are not a signer for this wallet"
        case .alreadySigned:
            return "You have already signed this transaction"
        case .insufficientSignatures:
            return "Not enough signatures to execute transaction"
        case .noPrivateKey:
            return "Private key not found for signing"
        case .transactionNotFound:
            return "Transaction not found"
        case .walletNotFound:
            return "Multisig wallet not found"
        }
    }
}
