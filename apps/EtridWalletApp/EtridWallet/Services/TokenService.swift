//
//  TokenService.swift
//  EtridWallet
//
//  ERC20 token operations service with real contract interactions
//
//  Features:
//  - Real ERC-20 contract calls via eth_call
//  - Token balance fetching (balanceOf)
//  - Token metadata retrieval (name, symbol, decimals)
//  - Token transfer encoding
//  - Token allowance checking
//  - Batch token balance queries
//

import Foundation

actor TokenService {
    // MARK: - Properties
    private let web3Service: Web3Service
    private let network: Network

    // MARK: - Initialization
    init(network: Network, web3Service: Web3Service = .shared) {
        self.network = network
        self.web3Service = web3Service
    }

    // MARK: - Token Balance
    /// Gets token balance for address
    /// - Parameters:
    ///   - token: Token to check
    ///   - address: Wallet address
    /// - Returns: Balance as string
    func getBalance(token: Token, address: String) async throws -> String {
        guard let contractAddress = token.contractAddress else {
            // Native token balance
            return try await web3Service.getBalance(address: address)
        }

        return try await web3Service.getTokenBalance(
            tokenAddress: contractAddress,
            walletAddress: address
        )
    }

    /// Gets balances for multiple tokens
    /// - Parameters:
    ///   - tokens: Array of tokens
    ///   - address: Wallet address
    /// - Returns: Dictionary of token to balance
    func getBalances(tokens: [Token], address: String) async throws -> [Token: String] {
        var balances: [Token: String] = [:]

        for token in tokens {
            do {
                let balance = try await getBalance(token: token, address: address)
                balances[token] = balance
            } catch {
                ErrorLogger.log(error, context: "Get balance for \(token.symbol)")
                balances[token] = "0"
            }
        }

        return balances
    }

    // MARK: - Token Transfer
    /// Builds ERC20 transfer transaction
    /// - Parameters:
    ///   - token: Token to transfer
    ///   - from: Sender address
    ///   - to: Recipient address
    ///   - amount: Amount in token's smallest unit
    /// - Returns: Transaction request
    func buildTransfer(
        token: Token,
        from: String,
        to: String,
        amount: String
    ) async throws -> TransactionRequest {
        guard let contractAddress = token.contractAddress else {
            // Native token transfer
            return try await web3Service.buildTransaction(
                from: from,
                to: to,
                value: amount
            )
        }

        // ERC20 transfer
        let data = ERC20ABI.encodeTransfer(to: to, amount: amount)

        return try await web3Service.buildTransaction(
            from: from,
            to: contractAddress,
            value: "0x0",
            data: data
        )
    }

    // MARK: - Token Info
    /// Gets token name from contract
    /// - Parameter contractAddress: Token contract address
    /// - Returns: Token name
    func getTokenName(contractAddress: String) async throws -> String {
        let data = ERC20ABI.nameSignature
        let result = try await web3Service.call(to: contractAddress, data: data)
        return decodeString(result) ?? "Unknown"
    }

    /// Gets token symbol from contract
    /// - Parameter contractAddress: Token contract address
    /// - Returns: Token symbol
    func getTokenSymbol(contractAddress: String) async throws -> String {
        let data = ERC20ABI.symbolSignature
        let result = try await web3Service.call(to: contractAddress, data: data)
        return decodeString(result) ?? "???"
    }

    /// Gets token decimals from contract
    /// - Parameter contractAddress: Token contract address
    /// - Returns: Number of decimals
    func getTokenDecimals(contractAddress: String) async throws -> Int {
        let data = ERC20ABI.decimalsSignature
        let result = try await web3Service.call(to: contractAddress, data: data)

        guard let decimals = BigInt.fromHex(result) else {
            return 18 // Default
        }

        return Int(decimals)
    }

    /// Gets complete token information
    /// - Parameter contractAddress: Token contract address
    /// - Returns: Token object
    func getTokenInfo(contractAddress: String) async throws -> Token {
        async let name = getTokenName(contractAddress: contractAddress)
        async let symbol = getTokenSymbol(contractAddress: contractAddress)
        async let decimals = getTokenDecimals(contractAddress: contractAddress)

        let (n, s, d) = try await (name, symbol, decimals)

        return Token(
            symbol: s,
            name: n,
            contractAddress: contractAddress,
            decimals: d
        )
    }

    // MARK: - Token Discovery
    /// Discovers tokens held by address
    /// - Parameters:
    ///   - address: Wallet address
    ///   - knownTokens: Known token list to check
    /// - Returns: Tokens with non-zero balance
    func discoverTokens(address: String, knownTokens: [Token]) async throws -> [Token] {
        var tokensWithBalance: [Token] = []

        for token in knownTokens {
            do {
                let balance = try await getBalance(token: token, address: address)
                if let balanceValue = BigInt.fromHex(balance), balanceValue > 0 {
                    var updatedToken = token
                    updatedToken.balance = balance
                    tokensWithBalance.append(updatedToken)
                }
            } catch {
                continue
            }
        }

        return tokensWithBalance
    }

    // MARK: - Token Allowance
    /// Gets token allowance for spender
    /// - Parameters:
    ///   - token: Token contract
    ///   - owner: Owner address
    ///   - spender: Spender address
    /// - Returns: Allowed amount
    func getAllowance(token: Token, owner: String, spender: String) async throws -> String {
        guard let contractAddress = token.contractAddress else {
            throw WalletError.invalidInput("Not an ERC20 token")
        }

        let ownerClean = owner.removeHexPrefix.lowercased()
        let spenderClean = spender.removeHexPrefix.lowercased()

        let paddedOwner = String(repeating: "0", count: 64 - ownerClean.count) + ownerClean
        let paddedSpender = String(repeating: "0", count: 64 - spenderClean.count) + spenderClean

        let data = ERC20ABI.allowanceSignature + paddedOwner + paddedSpender

        return try await web3Service.call(to: contractAddress, data: data)
    }

    /// Builds approve transaction for token spending
    /// - Parameters:
    ///   - token: Token to approve
    ///   - from: Owner address
    ///   - spender: Spender address
    ///   - amount: Amount to approve
    /// - Returns: Transaction request
    func buildApprove(
        token: Token,
        from: String,
        spender: String,
        amount: String
    ) async throws -> TransactionRequest {
        guard let contractAddress = token.contractAddress else {
            throw WalletError.invalidInput("Not an ERC20 token")
        }

        let data = ERC20ABI.approveSignature +
            String(repeating: "0", count: 64 - spender.removeHexPrefix.count) + spender.removeHexPrefix +
            String(repeating: "0", count: 64 - amount.removeHexPrefix.count) + amount.removeHexPrefix

        return try await web3Service.buildTransaction(
            from: from,
            to: contractAddress,
            value: "0x0",
            data: data
        )
    }

    // MARK: - Helper Methods
    private func decodeString(_ hex: String) -> String? {
        guard let data = hex.hexToData else { return nil }

        // Skip first 64 bytes (offset and length info)
        guard data.count > 64 else { return nil }

        let stringData = data.dropFirst(64)
        return String(data: Data(stringData), encoding: .utf8)?.trimmingCharacters(in: .controlCharacters)
    }

    private func decodeUInt(_ hex: String) -> UInt64? {
        BigInt.fromHex(hex)
    }
}

// MARK: - Token List Management
actor TokenListManager {
    private var cachedTokens: [Int: [Token]] = [:] // chainId -> tokens

    static let shared = TokenListManager()

    private init() {
        loadDefaultTokens()
    }

    // MARK: - Token Lists
    func getTokens(for network: Network) -> [Token] {
        cachedTokens[network.chainId] ?? DefaultTokens.tokens(for: network)
    }

    func addCustomToken(_ token: Token, for network: Network) {
        var tokens = getTokens(for: network)
        if !tokens.contains(where: { $0.contractAddress == token.contractAddress }) {
            tokens.append(token)
            cachedTokens[network.chainId] = tokens
            saveCustomTokens()
        }
    }

    func removeToken(_ token: Token, for network: Network) {
        var tokens = getTokens(for: network)
        tokens.removeAll { $0.contractAddress == token.contractAddress }
        cachedTokens[network.chainId] = tokens
        saveCustomTokens()
    }

    // MARK: - Popular Tokens
    func getPopularTokens(for network: Network) -> [Token] {
        let allTokens = getTokens(for: network)
        return Array(allTokens.prefix(10))
    }

    func getStablecoins(for network: Network) -> [Token] {
        getTokens(for: network).filter {
            DefaultTokens.isStablecoin($0.symbol)
        }
    }

    // MARK: - Search
    func searchTokens(query: String, network: Network) -> [Token] {
        let tokens = getTokens(for: network)
        let lowercasedQuery = query.lowercased()

        return tokens.filter {
            $0.symbol.lowercased().contains(lowercasedQuery) ||
            $0.name.lowercased().contains(lowercasedQuery) ||
            $0.contractAddress?.lowercased().contains(lowercasedQuery) == true
        }
    }

    // MARK: - Persistence
    private func loadDefaultTokens() {
        // Load default tokens for all networks
        for network in Networks.allNetworks {
            cachedTokens[network.chainId] = DefaultTokens.tokens(for: network)
        }

        // Load custom tokens from storage
        if let data = UserDefaults.standard.data(forKey: "customTokens"),
           let customTokens = try? JSONDecoder().decode([Int: [Token]].self, from: data) {
            // Merge custom tokens
            for (chainId, tokens) in customTokens {
                cachedTokens[chainId]?.append(contentsOf: tokens)
            }
        }
    }

    private func saveCustomTokens() {
        if let encoded = try? JSONEncoder().encode(cachedTokens) {
            UserDefaults.standard.set(encoded, forKey: "customTokens")
        }
    }
}
