//
//  WalletStorage.swift
//  EtridWallet
//
//  Wallet data persistence
//

import Foundation

actor WalletStorage {
    // MARK: - Singleton
    static let shared = WalletStorage()

    private let storage = StorageManager.shared
    private let keychain = KeychainManager()

    // Filenames
    private let accountsFilename = "accounts.json"
    private let configFilename = "config.json"
    private let transactionsFilename = "transactions.json"
    private let addressBookFilename = "addressBook.json"

    private init() {}

    // MARK: - Accounts
    /// Saves wallet accounts
    /// - Parameter accounts: Accounts to save
    func saveAccounts(_ accounts: [WalletAccount]) async throws {
        try await storage.save(accounts, filename: accountsFilename)
    }

    /// Loads wallet accounts
    /// - Returns: Array of accounts
    func loadAccounts() async throws -> [WalletAccount] {
        try await storage.load([WalletAccount].self, filename: accountsFilename) ?? []
    }

    /// Adds new account
    /// - Parameter account: Account to add
    func addAccount(_ account: WalletAccount) async throws {
        var accounts = try await loadAccounts()
        accounts.append(account)
        try await saveAccounts(accounts)
    }

    /// Updates existing account
    /// - Parameter account: Updated account
    func updateAccount(_ account: WalletAccount) async throws {
        var accounts = try await loadAccounts()
        if let index = accounts.firstIndex(where: { $0.id == account.id }) {
            accounts[index] = account
            try await saveAccounts(accounts)
        }
    }

    /// Deletes account
    /// - Parameter accountId: Account ID to delete
    func deleteAccount(_ accountId: UUID) async throws {
        var accounts = try await loadAccounts()
        accounts.removeAll { $0.id == accountId }
        try await saveAccounts(accounts)
    }

    // MARK: - Private Keys
    /// Saves private key securely in keychain
    /// - Parameters:
    ///   - privateKey: Private key data
    ///   - address: Associated address
    func savePrivateKey(_ privateKey: Data, for address: String) throws {
        try keychain.store(privateKey, for: address)
    }

    /// Loads private key from keychain
    /// - Parameter address: Address
    /// - Returns: Private key data
    func loadPrivateKey(for address: String) throws -> Data? {
        try keychain.retrieve(for: address)
    }

    /// Deletes private key
    /// - Parameter address: Address
    func deletePrivateKey(for address: String) throws {
        try keychain.delete(for: address)
    }

    // MARK: - Mnemonic
    /// Saves mnemonic phrase securely
    /// - Parameter mnemonic: Mnemonic phrase
    func saveMnemonic(_ mnemonic: String) throws {
        guard let data = mnemonic.data(using: .utf8) else {
            throw WalletError.encodingFailed
        }
        try keychain.store(data, for: "mnemonic")
    }

    /// Loads mnemonic phrase
    /// - Returns: Mnemonic phrase or nil
    func loadMnemonic() -> String? {
        do {
            let data = try keychain.retrieve(for: "mnemonic")
            return String(data: data, encoding: .utf8)
        } catch {
            // Mnemonic doesn't exist
            return nil
        }
    }

    /// Deletes mnemonic phrase
    func deleteMnemonic() throws {
        try keychain.delete(for: "mnemonic")
    }

    /// Checks if mnemonic exists
    /// - Returns: True if mnemonic is stored
    func hasMnemonic() -> Bool {
        (try? loadMnemonic()) != nil
    }

    // MARK: - Configuration
    /// Saves wallet configuration
    /// - Parameter config: Configuration
    func saveConfiguration(_ config: WalletConfiguration) async throws {
        try await storage.save(config, filename: configFilename)
    }

    /// Loads wallet configuration
    /// - Returns: Configuration or default
    func loadConfiguration() async throws -> WalletConfiguration {
        try await storage.load(WalletConfiguration.self, filename: configFilename) ?? .default
    }

    // MARK: - Transactions
    /// Saves transaction history
    /// - Parameter transactions: Transactions to save
    func saveTransactions(_ transactions: [Transaction]) async throws {
        try await storage.save(transactions, filename: transactionsFilename)
    }

    /// Loads transaction history
    /// - Returns: Array of transactions
    func loadTransactions() async throws -> [Transaction] {
        try await storage.load([Transaction].self, filename: transactionsFilename) ?? []
    }

    /// Adds new transaction
    /// - Parameter transaction: Transaction to add
    func addTransaction(_ transaction: Transaction) async throws {
        var transactions = try await loadTransactions()
        transactions.insert(transaction, at: 0) // Add to beginning
        try await saveTransactions(transactions)
    }

    /// Updates transaction status
    /// - Parameters:
    ///   - hash: Transaction hash
    ///   - status: New status
    func updateTransactionStatus(_ hash: String, status: TransactionStatus) async throws {
        var transactions = try await loadTransactions()
        if let index = transactions.firstIndex(where: { $0.hash == hash }) {
            var transaction = transactions[index]
            transactions[index] = Transaction(
                id: transaction.id,
                hash: transaction.hash,
                from: transaction.from,
                to: transaction.to,
                value: transaction.value,
                data: transaction.data,
                gasLimit: transaction.gasLimit,
                gasPrice: transaction.gasPrice,
                nonce: transaction.nonce,
                timestamp: transaction.timestamp,
                status: status,
                network: transaction.network,
                type: transaction.type,
                blockNumber: transaction.blockNumber,
                confirmations: transaction.confirmations
            )
            try await saveTransactions(transactions)
        }
    }

    /// Gets transactions for address
    /// - Parameters:
    ///   - address: Wallet address
    ///   - limit: Maximum number to return
    /// - Returns: Filtered transactions
    func getTransactions(for address: String, limit: Int? = nil) async throws -> [Transaction] {
        let transactions = try await loadTransactions()
        let filtered = transactions.filter {
            $0.from.lowercased() == address.lowercased() ||
            $0.to.lowercased() == address.lowercased()
        }

        if let limit = limit {
            return Array(filtered.prefix(limit))
        }

        return filtered
    }

    // MARK: - Address Book
    /// Saves address book
    /// - Parameter entries: Address book entries
    func saveAddressBook(_ entries: [AddressBookEntry]) async throws {
        try await storage.save(entries, filename: addressBookFilename)
    }

    /// Loads address book
    /// - Returns: Address book entries
    func loadAddressBook() async throws -> [AddressBookEntry] {
        try await storage.load([AddressBookEntry].self, filename: addressBookFilename) ?? []
    }

    /// Adds address book entry
    /// - Parameter entry: Entry to add
    func addAddressBookEntry(_ entry: AddressBookEntry) async throws {
        var entries = try await loadAddressBook()
        entries.append(entry)
        try await saveAddressBook(entries)
    }

    /// Deletes address book entry
    /// - Parameter entryId: Entry ID to delete
    func deleteAddressBookEntry(_ entryId: UUID) async throws {
        var entries = try await loadAddressBook()
        entries.removeAll { $0.id == entryId }
        try await saveAddressBook(entries)
    }

    /// Searches address book
    /// - Parameter query: Search query
    /// - Returns: Matching entries
    func searchAddressBook(_ query: String) async throws -> [AddressBookEntry] {
        let entries = try await loadAddressBook()
        let lowercasedQuery = query.lowercased()

        return entries.filter {
            $0.name.lowercased().contains(lowercasedQuery) ||
            $0.address.lowercased().contains(lowercasedQuery)
        }
    }

    // MARK: - Wallet State
    /// Checks if wallet is initialized
    /// - Returns: True if wallet exists
    func isWalletInitialized() async -> Bool {
        do {
            let accounts = try await loadAccounts()
            return !accounts.isEmpty
        } catch {
            return false
        }
    }

    /// Gets first account (primary account)
    /// - Returns: Primary account or nil
    func getPrimaryAccount() async throws -> WalletAccount? {
        let accounts = try await loadAccounts()
        return accounts.first
    }

    // MARK: - Data Export/Import
    /// Exports all wallet data (encrypted)
    /// - Parameter password: Encryption password
    /// - Returns: Encrypted data
    func exportWalletData(password: String) async throws -> Data {
        let accounts = try await loadAccounts()
        let transactions = try await loadTransactions()
        let addressBook = try await loadAddressBook()
        let config = try await loadConfiguration()

        let exportData = WalletExportData(
            accounts: accounts,
            transactions: transactions,
            addressBook: addressBook,
            configuration: config
        )

        let jsonData = try JSONEncoder().encode(exportData)
        return try await CryptoManager.shared.encrypt(jsonData, password: password)
    }

    /// Imports wallet data from encrypted backup
    /// - Parameters:
    ///   - data: Encrypted backup data
    ///   - password: Decryption password
    func importWalletData(_ data: Data, password: String) async throws {
        let decrypted = try await CryptoManager.shared.decrypt(data, password: password)
        let exportData = try JSONDecoder().decode(WalletExportData.self, from: decrypted)

        try await saveAccounts(exportData.accounts)
        try await saveTransactions(exportData.transactions)
        try await saveAddressBook(exportData.addressBook)
        try await saveConfiguration(exportData.configuration)
    }

    // MARK: - Clear All Data
    /// Removes all wallet data (danger!)
    func clearAllData() async throws {
        // Delete files
        try? await storage.delete(filename: accountsFilename)
        try? await storage.delete(filename: configFilename)
        try? await storage.delete(filename: transactionsFilename)
        try? await storage.delete(filename: addressBookFilename)

        // Delete keychain items
        let accounts = try? await loadAccounts()
        accounts?.forEach { account in
            try? deletePrivateKey(for: account.address)
        }
        try? deleteMnemonic()
    }
}

// MARK: - Export Data Model
struct WalletExportData: Codable {
    let accounts: [WalletAccount]
    let transactions: [Transaction]
    let addressBook: [AddressBookEntry]
    let configuration: WalletConfiguration
}
