//
//  BalanceService.swift
//  EtridWallet
//
//  Balance fetching and caching service
//

import Foundation
import Combine

@MainActor
class BalanceService: ObservableObject {
    // MARK: - Published Properties
    @Published var balances: [String: AccountBalance] = [:] // address -> balance
    @Published var isLoading = false
    @Published var lastUpdate: Date?

    // MARK: - Private Properties
    private let web3Service: Web3Service
    private let tokenService: TokenService
    private var refreshTimer: Timer?
    private var cancellables = Set<AnyCancellable>()

    // MARK: - Configuration
    private let refreshInterval: TimeInterval = 15 // 15 seconds
    private let cacheTimeout: TimeInterval = 30 // 30 seconds

    // MARK: - Singleton
    static let shared = BalanceService()

    init(web3Service: Web3Service = .shared, network: Network = .ethereum) {
        self.web3Service = web3Service
        self.tokenService = TokenService(network: network)
        setupAutoRefresh()
    }

    // MARK: - Balance Fetching
    /// Fetches balance for single address
    /// - Parameters:
    ///   - address: Wallet address
    ///   - tokens: Tokens to fetch balances for
    ///   - forceRefresh: Skip cache
    /// - Returns: Account balance
    func fetchBalance(
        for address: String,
        tokens: [Token] = [],
        forceRefresh: Bool = false
    ) async throws -> AccountBalance {
        // Check cache
        if !forceRefresh,
           let cached = balances[address],
           let lastUpdate = lastUpdate,
           Date().timeIntervalSince(lastUpdate) < cacheTimeout {
            return cached
        }

        isLoading = true
        defer { isLoading = false }

        // Fetch native balance
        let nativeBalance = try await web3Service.getBalance(address: address)

        // Fetch token balances
        var tokenBalances: [Token] = []
        for var token in tokens {
            do {
                let balance = try await tokenService.getBalance(token: token, address: address)
                token.balance = balance
                tokenBalances.append(token)
            } catch {
                ErrorLogger.log(error, context: "Fetch token balance: \(token.symbol)")
            }
        }

        let accountBalance = AccountBalance(
            address: address,
            nativeBalance: nativeBalance,
            tokens: tokenBalances,
            lastUpdated: Date()
        )

        balances[address] = accountBalance
        lastUpdate = Date()

        return accountBalance
    }

    /// Fetches balances for multiple addresses
    /// - Parameters:
    ///   - addresses: Array of wallet addresses
    ///   - tokens: Tokens to fetch
    /// - Returns: Dictionary of address to balance
    func fetchBalances(
        for addresses: [String],
        tokens: [Token] = []
    ) async throws -> [String: AccountBalance] {
        isLoading = true
        defer { isLoading = false }

        var results: [String: AccountBalance] = [:]

        for address in addresses {
            do {
                let balance = try await fetchBalance(for: address, tokens: tokens, forceRefresh: true)
                results[address] = balance
            } catch {
                ErrorLogger.log(error, context: "Fetch balance for \(address)")
            }
        }

        return results
    }

    /// Gets cached balance or fetches if needed
    /// - Parameter address: Wallet address
    /// - Returns: Account balance
    func getBalance(for address: String) async throws -> AccountBalance {
        if let cached = balances[address],
           let lastUpdate = lastUpdate,
           Date().timeIntervalSince(lastUpdate) < cacheTimeout {
            return cached
        }

        return try await fetchBalance(for: address)
    }

    // MARK: - Auto Refresh
    private func setupAutoRefresh() {
        refreshTimer = Timer.scheduledTimer(withTimeInterval: refreshInterval, repeats: true) { [weak self] _ in
            Task { @MainActor [weak self] in
                await self?.refreshAllBalances()
            }
        }
    }

    func startAutoRefresh() {
        setupAutoRefresh()
    }

    func stopAutoRefresh() {
        refreshTimer?.invalidate()
        refreshTimer = nil
    }

    private func refreshAllBalances() async {
        guard !balances.isEmpty else { return }

        for address in balances.keys {
            do {
                _ = try await fetchBalance(for: address, forceRefresh: true)
            } catch {
                ErrorLogger.log(error, context: "Auto refresh balance")
            }
        }
    }

    /// Manually refresh balance for address
    /// - Parameter address: Wallet address
    func refresh(address: String) async {
        do {
            _ = try await fetchBalance(for: address, forceRefresh: true)
        } catch {
            ErrorLogger.log(error, context: "Refresh balance")
        }
    }

    // MARK: - Cache Management
    func clearCache() {
        balances.removeAll()
        lastUpdate = nil
    }

    func clearCache(for address: String) {
        balances.removeValue(forKey: address)
    }

    // MARK: - Balance Calculations
    /// Calculates total balance in USD
    /// - Parameters:
    ///   - accountBalance: Account balance
    ///   - prices: Price data for tokens
    /// - Returns: Total value in USD
    func calculateTotalValue(
        accountBalance: AccountBalance,
        prices: [String: PriceData]
    ) -> Double {
        var total: Double = 0

        // Native balance
        if let ethPrice = prices["ETH"] {
            let ethBalance = WeiConverter.weiToEther(accountBalance.nativeBalance)
            total += ethBalance * ethPrice.price
        }

        // Token balances
        for token in accountBalance.tokens {
            if let price = prices[token.symbol] {
                let balance = Double(token.balance) ?? 0
                let divisor = pow(10.0, Double(token.decimals))
                total += (balance / divisor) * price.price
            }
        }

        return total
    }

    /// Gets balance for specific token
    /// - Parameters:
    ///   - token: Token to find
    ///   - accountBalance: Account balance
    /// - Returns: Token balance or nil
    func getTokenBalance(
        token: Token,
        from accountBalance: AccountBalance
    ) -> Token? {
        accountBalance.tokens.first {
            $0.contractAddress == token.contractAddress
        }
    }
}

// MARK: - Account Balance Model
struct AccountBalance {
    let address: String
    let nativeBalance: String
    let tokens: [Token]
    let lastUpdated: Date

    var formattedNativeBalance: String {
        WeiConverter.formatWeiAsEther(nativeBalance)
    }

    var totalTokenCount: Int {
        tokens.count
    }

    var tokensWithBalance: [Token] {
        tokens.filter { token in
            guard let balance = BigInt.fromHex(token.balance) else { return false }
            return balance > 0
        }
    }
}

// MARK: - Price Service (Mock)
actor PriceService {
    static let shared = PriceService()

    private var cachedPrices: [String: PriceData] = [:]
    private var lastUpdate: Date?

    private init() {}

    /// Fetches current prices for symbols
    /// - Parameter symbols: Array of token symbols
    /// - Returns: Dictionary of symbol to price data
    func fetchPrices(for symbols: [String]) async throws -> [String: PriceData] {
        // Mock implementation - in production, call CoinGecko/CoinMarketCap API
        var prices: [String: PriceData] = [:]

        for symbol in symbols {
            let mockPrice = generateMockPrice(for: symbol)
            prices[symbol] = mockPrice
            cachedPrices[symbol] = mockPrice
        }

        lastUpdate = Date()
        return prices
    }

    /// Gets cached price or fetches if needed
    /// - Parameter symbol: Token symbol
    /// - Returns: Price data
    func getPrice(for symbol: String) async throws -> PriceData {
        if let cached = cachedPrices[symbol],
           let lastUpdate = lastUpdate,
           Date().timeIntervalSince(lastUpdate) < 60 {
            return cached
        }

        let prices = try await fetchPrices(for: [symbol])
        return prices[symbol] ?? generateMockPrice(for: symbol)
    }

    private func generateMockPrice(for symbol: String) -> PriceData {
        let prices: [String: Double] = [
            "ETH": 2000.00,
            "BTC": 45000.00,
            "USDT": 1.00,
            "USDC": 1.00,
            "DAI": 1.00,
            "BNB": 300.00,
            "MATIC": 0.80,
            "AVAX": 35.00
        ]

        let price = prices[symbol] ?? Double.random(in: 0.1...100)
        let change = Double.random(in: -10...10)

        return PriceData(
            symbol: symbol,
            price: price,
            change24h: change,
            lastUpdated: Date()
        )
    }
}
