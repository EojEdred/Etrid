import Foundation
import Combine
import LocalAuthentication

// MARK: - Enhanced Wallet Manager

@MainActor
public class EtridWalletManager: ObservableObject {

    // MARK: - Published Properties

    @Published public var appState: AppState = .loading
    @Published public var currentWallet: Wallet?
    @Published public var accounts: [WalletAccountInfo] = []
    @Published public var selectedAccount: WalletAccountInfo?
    @Published public var totalBalance: Double = 0
    @Published public var isLoading: Bool = false
    @Published public var error: WalletError?

    // MARK: - Services

    private let hdWalletManager = HDWalletManager()
    private let keychainManager = KeychainManager()
    private let storageManager = WalletStorageManager()
    private var bridgeService: BridgeService?
    private var governanceService: GovernanceService?
    private var rpcService: RPCService?
    private var governanceRPCService: GovernanceRPCService?

    // MARK: - Configuration

    private let networkConfig: NetworkConfig

    // MARK: - Initialization

    public init(networkConfig: NetworkConfig = .etridMainnet) {
        self.networkConfig = networkConfig
        setupServices()
        Task {
            await loadWalletState()
        }
    }

    private func setupServices() {
        guard let rpcURL = URL(string: networkConfig.rpcURL) else { return }
        rpcService = RPCService(baseURL: rpcURL)
        if let rpc = rpcService {
            bridgeService = BridgeService(rpcService: rpc)
        }
        // Initialize governance RPC service
        if let govRPC = try? GovernanceRPCService(endpoint: networkConfig.rpcURL) {
            governanceRPCService = govRPC
            governanceService = GovernanceService(rpcService: govRPC)
        }
    }

    // MARK: - Wallet Creation

    /// Create a new wallet with fresh mnemonic
    public func createNewWallet(name: String, passphrase: String = "") async throws -> (wallet: Wallet, mnemonic: String) {
        isLoading = true
        defer { isLoading = false }

        // Generate 12-word mnemonic (128 bits - industry standard)
        let mnemonic = try await hdWalletManager.generateMnemonic(strength: 128)

        // Create wallet from mnemonic
        let wallet = try await createWalletFromMnemonic(mnemonic, name: name, passphrase: passphrase)

        return (wallet, mnemonic)
    }

    /// Import wallet from existing mnemonic
    public func importWallet(mnemonic: String, name: String, passphrase: String = "") async throws -> Wallet {
        isLoading = true
        defer { isLoading = false }

        // Validate mnemonic
        let isValid = await hdWalletManager.validateMnemonic(mnemonic)
        guard isValid else {
            throw WalletError.invalidMnemonic
        }

        return try await createWalletFromMnemonic(mnemonic, name: name, passphrase: passphrase)
    }

    private func createWalletFromMnemonic(_ mnemonic: String, name: String, passphrase: String) async throws -> Wallet {
        // Generate seed
        let seed = try await hdWalletManager.seedFromMnemonic(mnemonic, passphrase: passphrase)

        // Derive master key
        let masterKey = try await hdWalletManager.masterKeyFromSeed(seed)

        // Create ËTRID account (first account)
        let derivationPath = await hdWalletManager.etridDerivationPath(account: 0, index: 0)
        let accountKey = try await hdWalletManager.deriveKey(from: masterKey, path: derivationPath)
        let address = try accountKey.ethereumAddress()

        // Store encrypted mnemonic
        let walletId = UUID()
        try await storeMnemonic(mnemonic, for: walletId)

        // Store private key
        try keychainManager.store(accountKey.privateKey, for: address)

        // Create account info
        let account = WalletAccountInfo(
            address: address,
            chain: .etrid,
            derivationPath: derivationPath,
            balances: [TokenBalance(token: .etrid, amount: 0)]
        )

        // Create wallet
        let wallet = Wallet(
            id: walletId,
            name: name,
            accounts: [account]
        )

        // Save wallet
        try await saveWallet(wallet)

        // Update state
        currentWallet = wallet
        accounts = wallet.accounts
        selectedAccount = accounts.first
        appState = .unlocked

        return wallet
    }

    // MARK: - Wallet Management

    /// Load wallet state on app launch
    private func loadWalletState() async {
        do {
            if let savedWallet = try await storageManager.loadWallet() {
                currentWallet = savedWallet
                accounts = savedWallet.accounts
                selectedAccount = accounts.first
                appState = .locked
            } else {
                appState = .onboarding
            }
        } catch {
            appState = .onboarding
        }
    }

    /// Save wallet to storage
    private func saveWallet(_ wallet: Wallet) async throws {
        try await storageManager.saveWallet(wallet)
    }

    /// Store mnemonic securely
    private func storeMnemonic(_ mnemonic: String, for walletId: UUID) async throws {
        guard let data = mnemonic.data(using: .utf8) else {
            throw WalletError.encodingError
        }
        try keychainManager.store(data, for: "mnemonic_\(walletId.uuidString)")
    }

    /// Retrieve mnemonic (requires authentication)
    public func getMnemonic(for walletId: UUID) async throws -> String {
        let data = try keychainManager.retrieve(for: "mnemonic_\(walletId.uuidString)")
        guard let mnemonic = String(data: data, encoding: .utf8) else {
            throw WalletError.decodingError
        }
        return mnemonic
    }

    /// Delete wallet
    public func deleteWallet() async throws {
        guard let wallet = currentWallet else { return }

        // Delete mnemonic
        try? keychainManager.delete(for: "mnemonic_\(wallet.id.uuidString)")

        // Delete private keys
        for account in wallet.accounts {
            try? keychainManager.delete(for: account.address)
        }

        // Delete wallet data
        try await storageManager.deleteWallet()

        // Reset state
        currentWallet = nil
        accounts = []
        selectedAccount = nil
        appState = .onboarding
    }

    // MARK: - Authentication

    /// Unlock wallet with biometrics
    public func unlockWithBiometrics() async throws {
        let context = LAContext()
        var error: NSError?

        guard context.canEvaluatePolicy(.deviceOwnerAuthenticationWithBiometrics, error: &error) else {
            throw WalletError.biometricsNotAvailable
        }

        let reason = "Unlock Ëtridefi Bloc"
        let success = try await context.evaluatePolicy(
            .deviceOwnerAuthenticationWithBiometrics,
            localizedReason: reason
        )

        if success {
            appState = .unlocked
        } else {
            throw WalletError.authenticationFailed
        }
    }

    /// Unlock wallet with passcode
    public func unlockWithPasscode(_ passcode: String) async throws {
        // Verify passcode against stored hash
        guard let storedHash = try? await storageManager.getPasscodeHash() else {
            throw WalletError.passcodeNotSet
        }

        let inputHash = hashPasscode(passcode)
        guard inputHash == storedHash else {
            throw WalletError.incorrectPasscode
        }

        appState = .unlocked
    }

    /// Set passcode
    public func setPasscode(_ passcode: String) async throws {
        guard passcode.count >= 6 else {
            throw WalletError.passcodeTooShort
        }

        let hash = hashPasscode(passcode)
        try await storageManager.savePasscodeHash(hash)
    }

    private func hashPasscode(_ passcode: String) -> String {
        // Use SHA256 for passcode hashing
        guard let data = passcode.data(using: .utf8) else { return "" }
        let hash = SHA256.hash(data: data)
        return hash.map { String(format: "%02x", $0) }.joined()
    }

    /// Lock wallet
    public func lock() {
        appState = .locked
    }

    // MARK: - Account Management

    /// Add new account to wallet
    public func addAccount(name: String? = nil) async throws -> WalletAccountInfo {
        guard var wallet = currentWallet else {
            throw WalletError.noWalletFound
        }

        let accountIndex = wallet.accounts.count

        // Retrieve mnemonic and derive new key
        let mnemonic = try await getMnemonic(for: wallet.id)
        let seed = try await hdWalletManager.seedFromMnemonic(mnemonic)
        let masterKey = try await hdWalletManager.masterKeyFromSeed(seed)

        let derivationPath = await hdWalletManager.etridDerivationPath(account: 0, index: accountIndex)
        let accountKey = try await hdWalletManager.deriveKey(from: masterKey, path: derivationPath)
        let address = try accountKey.ethereumAddress()

        // Store private key
        try keychainManager.store(accountKey.privateKey, for: address)

        // Create account
        let account = WalletAccountInfo(
            address: address,
            chain: .etrid,
            derivationPath: derivationPath,
            balances: [TokenBalance(token: .etrid, amount: 0)]
        )

        wallet.accounts.append(account)
        try await saveWallet(wallet)

        currentWallet = wallet
        accounts = wallet.accounts

        return account
    }

    /// Select active account
    public func selectAccount(_ account: WalletAccountInfo) {
        selectedAccount = account
    }

    // MARK: - Balance & Transactions

    /// Refresh balances for all accounts
    public func refreshBalances() async {
        isLoading = true
        defer { isLoading = false }

        guard let rpc = rpcService else { return }

        for (index, account) in accounts.enumerated() {
            do {
                let result = try await rpc.call(
                    method: "eth_getBalance",
                    params: ["address": account.address, "block": "latest"]
                )

                if let balanceHex = result["balance"] as? String {
                    let balance = parseHexBalance(balanceHex)
                    var updatedAccount = account
                    if let etridIndex = updatedAccount.balances.firstIndex(where: { $0.token == .etrid }) {
                        updatedAccount.balances[etridIndex] = TokenBalance(
                            token: .etrid,
                            amount: balance,
                            lastUpdated: Date()
                        )
                    }
                    accounts[index] = updatedAccount
                }
            } catch {
                print("Failed to fetch balance for \(account.address): \(error)")
            }
        }

        // Update total balance
        totalBalance = accounts.reduce(0) { $0 + $1.totalUSDValue }

        // Sync to selected account
        if let selected = selectedAccount,
           let updated = accounts.first(where: { $0.id == selected.id }) {
            selectedAccount = updated
        }
    }

    private func parseHexBalance(_ hex: String) -> Decimal {
        let cleanHex = hex.hasPrefix("0x") ? String(hex.dropFirst(2)) : hex
        guard let value = UInt64(cleanHex, radix: 16) else { return 0 }
        return Decimal(value) / Decimal(pow(10, 18))
    }

    /// Get transaction history
    public func getTransactionHistory(for address: String, limit: Int = 50) async throws -> [Transaction] {
        guard let rpc = rpcService else {
            throw WalletError.serviceNotAvailable
        }

        let result = try await rpc.call(
            method: "eth_getTransactions",
            params: ["address": address, "limit": limit]
        )

        // Parse transactions
        guard let txData = result["transactions"] as? [[String: Any]] else {
            return []
        }

        return txData.compactMap { data -> Transaction? in
            guard let hash = data["hash"] as? String,
                  let from = data["from"] as? String,
                  let to = data["to"] as? String else {
                return nil
            }

            let type: TransactionType = from.lowercased() == address.lowercased() ? .send : .receive

            return Transaction(
                hash: hash,
                type: type,
                status: .confirmed,
                from: from,
                to: to,
                amount: Decimal(string: data["value"] as? String ?? "0") ?? 0,
                token: .etrid,
                fee: Decimal(string: data["fee"] as? String ?? "0") ?? 0,
                feeToken: .etrid,
                timestamp: Date(timeIntervalSince1970: data["timestamp"] as? Double ?? 0),
                blockNumber: data["blockNumber"] as? UInt64,
                confirmations: data["confirmations"] as? Int ?? 0
            )
        }
    }

    // MARK: - Sending Transactions

    /// Send ETR tokens
    public func sendETR(
        to recipient: String,
        amount: Decimal,
        from account: WalletAccountInfo? = nil
    ) async throws -> String {
        let fromAccount = account ?? selectedAccount
        guard let sender = fromAccount else {
            throw WalletError.noAccountSelected
        }

        guard let rpc = rpcService else {
            throw WalletError.serviceNotAvailable
        }

        // Get private key
        let privateKey = try keychainManager.retrieve(for: sender.address)

        // Build transaction
        let result = try await rpc.call(
            method: "eth_sendTransaction",
            params: [
                "from": sender.address,
                "to": recipient,
                "value": amount.description,
                "privateKey": privateKey.base64EncodedString()
            ]
        )

        guard let txHash = result["txHash"] as? String else {
            throw WalletError.transactionFailed("No transaction hash returned")
        }

        return txHash
    }

    /// Send bridged tokens
    public func sendBridgedToken(
        token: Token,
        to recipient: String,
        amount: Decimal,
        from account: WalletAccountInfo? = nil
    ) async throws -> String {
        let fromAccount = account ?? selectedAccount
        guard let sender = fromAccount else {
            throw WalletError.noAccountSelected
        }

        guard let rpc = rpcService else {
            throw WalletError.serviceNotAvailable
        }

        let result = try await rpc.call(
            method: "token_transfer",
            params: [
                "token": token.id,
                "from": sender.address,
                "to": recipient,
                "amount": amount.description
            ]
        )

        guard let txHash = result["txHash"] as? String else {
            throw WalletError.transactionFailed("No transaction hash returned")
        }

        return txHash
    }

    // MARK: - Bridge Access

    public func getBridgeService() -> BridgeService? {
        return bridgeService
    }

    // MARK: - Governance Access

    public func getGovernanceService() -> GovernanceService? {
        return governanceService
    }

    /// Get governance dashboard for current account
    public func getGovernanceDashboard() async throws -> GovernanceDashboard? {
        guard let governance = governanceService,
              let account = selectedAccount else {
            return nil
        }

        return try await governance.getDashboard(address: account.address)
    }
}

// MARK: - Wallet Storage Manager

actor WalletStorageManager {
    private let walletKey = "etrid_wallet"
    private let passcodeKey = "etrid_passcode_hash"

    func saveWallet(_ wallet: Wallet) async throws {
        let encoder = JSONEncoder()
        let data = try encoder.encode(wallet)
        UserDefaults.standard.set(data, forKey: walletKey)
    }

    func loadWallet() async throws -> Wallet? {
        guard let data = UserDefaults.standard.data(forKey: walletKey) else {
            return nil
        }
        let decoder = JSONDecoder()
        return try decoder.decode(Wallet.self, from: data)
    }

    func deleteWallet() async throws {
        UserDefaults.standard.removeObject(forKey: walletKey)
    }

    func savePasscodeHash(_ hash: String) async throws {
        UserDefaults.standard.set(hash, forKey: passcodeKey)
    }

    func getPasscodeHash() async throws -> String? {
        return UserDefaults.standard.string(forKey: passcodeKey)
    }
}

// MARK: - Enhanced Wallet Errors

import CryptoKit

public enum WalletError: LocalizedError {
    case invalidMnemonic
    case encodingError
    case decodingError
    case biometricsNotAvailable
    case authenticationFailed
    case passcodeNotSet
    case incorrectPasscode
    case passcodeTooShort
    case noWalletFound
    case noAccountSelected
    case serviceNotAvailable
    case keyGenerationFailed
    case transactionFailed(String)
    case insufficientBalance

    public var errorDescription: String? {
        switch self {
        case .invalidMnemonic:
            return "Invalid recovery phrase"
        case .encodingError:
            return "Failed to encode data"
        case .decodingError:
            return "Failed to decode data"
        case .biometricsNotAvailable:
            return "Biometric authentication is not available"
        case .authenticationFailed:
            return "Authentication failed"
        case .passcodeNotSet:
            return "Passcode not set"
        case .incorrectPasscode:
            return "Incorrect passcode"
        case .passcodeTooShort:
            return "Passcode must be at least 6 characters"
        case .noWalletFound:
            return "No wallet found"
        case .noAccountSelected:
            return "No account selected"
        case .serviceNotAvailable:
            return "Service not available"
        case .keyGenerationFailed:
            return "Failed to generate keys"
        case .transactionFailed(let message):
            return "Transaction failed: \(message)"
        case .insufficientBalance:
            return "Insufficient balance"
        }
    }
}
