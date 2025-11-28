//
//  BlockchainService.swift
//  EtridWallet
//
//  Polkadot/Substrate blockchain integration service
//

import Foundation
import Combine

// MARK: - Blockchain Models
struct ChainAccount {
    let address: String
    let publicKey: String
    let privateKey: String
}

struct Balance {
    let free: Double
    let reserved: Double
    let frozen: Double

    var total: Double { free + reserved }
    var transferable: Double { free - frozen }
}

struct BlockchainTransaction {
    let hash: String
    let from: String
    let to: String
    let amount: Double
    let timestamp: Date
    let status: BlockchainTransactionStatus
    let blockNumber: Int?
}

enum BlockchainTransactionStatus {
    case pending
    case confirmed
    case failed
}

// MARK: - Blockchain Service
class BlockchainService {
    static let shared = BlockchainService()

    private let networkURL: String
    private let chainId: String

    private init() {
        // TODO: Configure your actual chain endpoint
        self.networkURL = ProcessInfo.processInfo.environment["CHAIN_RPC_URL"] ?? "wss://ws.etrid.org/primearc"
        self.chainId = "etrid"
    }

    // MARK: - Account Management
    func createAccount() async throws -> ChainAccount {
        // TODO: Integrate with sr25519/ed25519 key generation
        // For production, use SubstrateSdk or similar

        // Simulated account creation
        let privateKey = generateRandomHex(length: 64)
        let publicKey = generateRandomHex(length: 64)
        let address = try await deriveAddress(from: publicKey)

        let account = ChainAccount(
            address: address,
            publicKey: publicKey,
            privateKey: privateKey
        )

        // Save to keychain
        _ = KeychainService.shared.savePrivateKey(privateKey, for: address)

        return account
    }

    func importAccount(seedPhrase: String) async throws -> ChainAccount {
        // TODO: Implement BIP39 seed phrase import
        // Use SubstrateSdk for real implementation

        _ = KeychainService.shared.saveSeedPhrase(seedPhrase)

        // Derive account from seed
        return try await createAccount()
    }

    private func deriveAddress(from publicKey: String) async throws -> String {
        // TODO: Implement SS58 address encoding
        // For Substrate chains, use proper SS58 format
        return "5" + publicKey.prefix(47)
    }

    private func generateRandomHex(length: Int) -> String {
        let bytes = (0..<length/2).map { _ in String(format: "%02x", Int.random(in: 0...255)) }
        return bytes.joined()
    }

    // MARK: - Balance Queries
    func getBalance(for address: String) async throws -> Balance {
        struct BalanceResponse: Decodable {
            let free: String
            let reserved: String
            let frozen: String
        }

        do {
            let response: BalanceResponse = try await NetworkService.shared.request(
                endpoint: "/balance/\(address)"
            )

            return Balance(
                free: Double(response.free) ?? 0,
                reserved: Double(response.reserved) ?? 0,
                frozen: Double(response.frozen) ?? 0
            )
        } catch {
            // Fallback to mock data for testing
            return Balance(free: 10000, reserved: 1000, frozen: 500)
        }
    }

    // MARK: - Transactions
    func transfer(
        from: String,
        to: String,
        amount: Double,
        memo: String? = nil
    ) async throws -> BlockchainTransaction {
        guard let privateKey = KeychainService.shared.getPrivateKey(for: from) else {
            throw BlockchainError.noPrivateKey
        }

        struct TransferRequest: Encodable {
            let from: String
            let to: String
            let amount: String
            let memo: String?
        }

        struct TransferResponse: Decodable {
            let hash: String
            let blockNumber: Int?
        }

        let request = TransferRequest(
            from: from,
            to: to,
            amount: String(amount),
            memo: memo
        )

        let response: TransferResponse = try await NetworkService.shared.request(
            endpoint: "/transfer",
            method: .post,
            body: request
        )

        return BlockchainTransaction(
            hash: response.hash,
            from: from,
            to: to,
            amount: amount,
            timestamp: Date(),
            status: .pending,
            blockNumber: response.blockNumber
        )
    }

    func getTransactionHistory(for address: String, limit: Int = 50) async throws -> [BlockchainTransaction] {
        struct TransactionResponse: Decodable {
            let transactions: [TransactionData]
        }

        struct TransactionData: Decodable {
            let hash: String
            let from: String
            let to: String
            let amount: String
            let timestamp: Int
            let status: String
            let blockNumber: Int?
        }

        do {
            let response: TransactionResponse = try await NetworkService.shared.request(
                endpoint: "/transactions/\(address)?limit=\(limit)"
            )

            return response.transactions.map { tx in
                BlockchainTransaction(
                    hash: tx.hash,
                    from: tx.from,
                    to: tx.to,
                    amount: Double(tx.amount) ?? 0,
                    timestamp: Date(timeIntervalSince1970: TimeInterval(tx.timestamp)),
                    status: tx.status == "confirmed" ? .confirmed : .pending,
                    blockNumber: tx.blockNumber
                )
            }
        } catch {
            return []
        }
    }

    // MARK: - Staking
    func stake(address: String, amount: Double, validator: String) async throws {
        struct StakeRequest: Encodable {
            let address: String
            let amount: String
            let validator: String
        }

        let request = StakeRequest(
            address: address,
            amount: String(amount),
            validator: validator
        )

        let _: SharedEmptyResponse = try await NetworkService.shared.request(
            endpoint: "/stake",
            method: .post,
            body: request
        )
    }

    func unstake(address: String, amount: Double) async throws {
        struct UnstakeRequest: Encodable {
            let address: String
            let amount: String
        }

        let request = UnstakeRequest(
            address: address,
            amount: String(amount)
        )

        let _: SharedEmptyResponse = try await NetworkService.shared.request(
            endpoint: "/unstake",
            method: .post,
            body: request
        )
    }

    // MARK: - Chain Info
    func getChainInfo() async throws -> BlockchainChainInfo {
        return try await NetworkService.shared.request(endpoint: "/chain/info")
    }
}

// MARK: - Chain Info Model
struct BlockchainChainInfo: Decodable {
    let name: String
    let version: String
    let blockNumber: Int
    let blockTime: Int
}

// MARK: - Errors
enum BlockchainError: Error {
    case noPrivateKey
    case invalidAddress
    case insufficientBalance
    case transactionFailed(String)
    case networkError

    var localizedDescription: String {
        switch self {
        case .noPrivateKey:
            return "Private key not found"
        case .invalidAddress:
            return "Invalid address format"
        case .insufficientBalance:
            return "Insufficient balance"
        case .transactionFailed(let reason):
            return "Transaction failed: \(reason)"
        case .networkError:
            return "Network connection error"
        }
    }
}

// MARK: - Helper Types
// Using SharedEmptyResponse from SharedComponents.swift
