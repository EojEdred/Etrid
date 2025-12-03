import Foundation
import Combine

// MARK: - Price Service

/// Service for fetching and caching token prices
@MainActor
public class PriceService: ObservableObject {
    public static let shared = PriceService()

    // MARK: - Published Properties

    @Published public var prices: [String: TokenPrice] = [:]
    @Published public var lastUpdated: Date?
    @Published public var isLoading = false

    // MARK: - Private Properties

    private let session: URLSession
    private var updateTimer: Timer?
    private let cacheKey = "tokenPricesCache"

    // Price update interval (60 seconds)
    private let updateInterval: TimeInterval = 60

    // CoinGecko API (free tier)
    private let coingeckoBaseURL = "https://api.coingecko.com/api/v3"

    // Token ID mapping for CoinGecko
    private let tokenIdMapping: [String: String] = [
        "ETR": "etrid",           // ËTRID native token (not listed yet, use mock)
        "wETR": "etrid",          // Wrapped ËTRID
        "EDSC": "usd-coin",       // ËTRID stablecoin pegged to USD
        "BTC": "bitcoin",
        "ETH": "ethereum",
        "BNB": "binancecoin",
        "SOL": "solana",
        "XRP": "ripple",
        "ADA": "cardano",
        "DOGE": "dogecoin",
        "TRX": "tron",
        "XLM": "stellar",
        "USDT": "tether",
        "USDC": "usd-coin",
    ]

    private init() {
        let config = URLSessionConfiguration.default
        config.timeoutIntervalForRequest = 15
        self.session = URLSession(configuration: config)

        loadCachedPrices()
        startPriceUpdates()
    }

    deinit {
        updateTimer?.invalidate()
    }

    // MARK: - Public Methods

    /// Get current price for a token
    public func getPrice(for symbol: String) -> Decimal? {
        return prices[symbol.uppercased()]?.price
    }

    /// Get price change (24h) for a token
    public func getPriceChange(for symbol: String) -> Double? {
        return prices[symbol.uppercased()]?.change24h
    }

    /// Calculate value of token amount in USD
    public func calculateValue(amount: Decimal, symbol: String) -> Decimal? {
        guard let price = getPrice(for: symbol) else { return nil }
        return amount * price
    }

    /// Refresh all prices
    public func refreshPrices() async {
        isLoading = true
        defer { isLoading = false }

        do {
            try await fetchPricesFromAPI()
            lastUpdated = Date()
            savePricesToCache()
        } catch {
            print("Failed to fetch prices: \(error)")
            // Keep using cached prices
        }
    }

    /// Get formatted price string
    public func formattedPrice(for symbol: String) -> String {
        guard let price = getPrice(for: symbol) else { return "$--" }

        let formatter = NumberFormatter()
        formatter.numberStyle = .currency
        formatter.currencyCode = "USD"

        if price < 0.01 {
            formatter.minimumFractionDigits = 6
            formatter.maximumFractionDigits = 6
        } else if price < 1 {
            formatter.minimumFractionDigits = 4
            formatter.maximumFractionDigits = 4
        } else {
            formatter.minimumFractionDigits = 2
            formatter.maximumFractionDigits = 2
        }

        return formatter.string(from: price as NSNumber) ?? "$--"
    }

    /// Get ETR price (native ËTRID token)
    public var etrPrice: Decimal {
        // Until ETR is listed on exchanges, use a mock price
        // In production, this would come from PrimeSwap DEX or oracle
        return prices["ETR"]?.price ?? Decimal(string: "0.15")!
    }

    /// Get EDSC price (should always be ~$1.00)
    public var edscPrice: Decimal {
        return prices["EDSC"]?.price ?? Decimal(string: "1.00")!
    }

    // MARK: - Private Methods

    private func startPriceUpdates() {
        // Initial fetch
        Task {
            await refreshPrices()
        }

        // Schedule periodic updates
        updateTimer = Timer.scheduledTimer(withTimeInterval: updateInterval, repeats: true) { [weak self] _ in
            Task { @MainActor in
                await self?.refreshPrices()
            }
        }
    }

    private func fetchPricesFromAPI() async throws {
        // Get all token IDs we need to fetch
        let ids = Set(tokenIdMapping.values).joined(separator: ",")

        guard let url = URL(string: "\(coingeckoBaseURL)/simple/price?ids=\(ids)&vs_currencies=usd&include_24hr_change=true") else {
            throw PriceError.invalidURL
        }

        var request = URLRequest(url: url)
        request.addValue("application/json", forHTTPHeaderField: "Accept")

        let (data, response) = try await session.data(for: request)

        guard let httpResponse = response as? HTTPURLResponse,
              httpResponse.statusCode == 200 else {
            throw PriceError.apiError
        }

        // Parse response
        guard let json = try JSONSerialization.jsonObject(with: data) as? [String: [String: Double]] else {
            throw PriceError.parseError
        }

        // Map back to our token symbols
        var newPrices: [String: TokenPrice] = [:]

        for (symbol, coingeckoId) in tokenIdMapping {
            if let priceData = json[coingeckoId] {
                let price = Decimal(priceData["usd"] ?? 0)
                let change24h = priceData["usd_24h_change"] ?? 0

                newPrices[symbol] = TokenPrice(
                    symbol: symbol,
                    price: price,
                    change24h: change24h,
                    lastUpdated: Date()
                )
            }
        }

        // Add mock ETR price (not on CoinGecko yet)
        if newPrices["ETR"] == nil {
            newPrices["ETR"] = TokenPrice(
                symbol: "ETR",
                price: Decimal(string: "0.15")!,
                change24h: 2.5,
                lastUpdated: Date()
            )
            newPrices["wETR"] = newPrices["ETR"]
        }

        // EDSC is a stablecoin, always $1
        newPrices["EDSC"] = TokenPrice(
            symbol: "EDSC",
            price: Decimal(string: "1.00")!,
            change24h: 0.0,
            lastUpdated: Date()
        )

        prices = newPrices
    }

    private func loadCachedPrices() {
        guard let data = UserDefaults.standard.data(forKey: cacheKey),
              let cached = try? JSONDecoder().decode([String: TokenPrice].self, from: data) else {
            return
        }

        prices = cached
    }

    private func savePricesToCache() {
        guard let data = try? JSONEncoder().encode(prices) else { return }
        UserDefaults.standard.set(data, forKey: cacheKey)
    }
}

// MARK: - Supporting Types

public struct TokenPrice: Codable, Equatable {
    public let symbol: String
    public let price: Decimal
    public let change24h: Double
    public let lastUpdated: Date

    public var isPositiveChange: Bool {
        change24h >= 0
    }

    public var formattedChange: String {
        let sign = change24h >= 0 ? "+" : ""
        return "\(sign)\(String(format: "%.2f", change24h))%"
    }

    public var changeColor: String {
        if change24h > 0 {
            return "green"
        } else if change24h < 0 {
            return "red"
        } else {
            return "gray"
        }
    }
}

public enum PriceError: LocalizedError {
    case invalidURL
    case apiError
    case parseError
    case rateLimited

    public var errorDescription: String? {
        switch self {
        case .invalidURL:
            return "Invalid price API URL"
        case .apiError:
            return "Failed to fetch prices"
        case .parseError:
            return "Failed to parse price data"
        case .rateLimited:
            return "Rate limited, try again later"
        }
    }
}

// MARK: - Portfolio Value Calculator

public extension PriceService {

    /// Calculate total portfolio value from token balances
    func calculatePortfolioValue(balances: [String: Decimal]) -> Decimal {
        var total: Decimal = 0

        for (symbol, amount) in balances {
            if let value = calculateValue(amount: amount, symbol: symbol) {
                total += value
            }
        }

        return total
    }

    /// Get price history for charting (mock data for now)
    func getPriceHistory(for symbol: String, days: Int = 7) async -> [PricePoint] {
        // In production, fetch from CoinGecko /coins/{id}/market_chart
        // For now, generate mock data
        var history: [PricePoint] = []
        let basePrice = Double(truncating: (getPrice(for: symbol) ?? 1) as NSNumber)

        let calendar = Calendar.current
        for i in (0..<days * 24).reversed() {
            let date = calendar.date(byAdding: .hour, value: -i, to: Date())!
            let variance = Double.random(in: -0.05...0.05)
            let price = Decimal(basePrice * (1 + variance))

            history.append(PricePoint(date: date, price: price))
        }

        return history
    }
}

public struct PricePoint: Identifiable {
    public let id = UUID()
    public let date: Date
    public let price: Decimal
}
