# Quick Start Guide - Phase 4: Price & Token Discovery

Get up and running with market data features in 5 minutes.

## Step 1: Verify Installation

Ensure all Phase 4 files are present:

```bash
# Check files exist
ls -la Sources/EtridWalletSwift/Models/MarketModels.swift
ls -la Sources/EtridWalletSwift/Services/PriceService.swift
ls -la Sources/EtridWalletSwift/Services/TokenMetadataService.swift
ls -la Sources/EtridWalletSwift/Services/TokenDiscoveryService.swift
ls -la Sources/EtridWalletSwift/Core/Market/MarketDataManager.swift
```

All files should exist. ✅

## Step 2: Initialize Services

Add this to your app initialization:

```swift
import Foundation

// Create services
let priceService = PriceService()
let metadataService = TokenMetadataService()
let discoveryService = TokenDiscoveryService(metadataService: metadataService)

// Create market manager
let marketManager = MarketDataManager(
    priceService: priceService,
    metadataService: metadataService,
    discoveryService: discoveryService
)
```

## Step 3: Basic Usage

### Get Token Prices

```swift
Task {
    do {
        // Fetch Bitcoin and Ethereum prices
        let prices = try await priceService.getTokenPrices(
            ids: ["bitcoin", "ethereum"],
            currency: "usd"
        )

        for (id, price) in prices {
            print("\(id): \(price.formattedPrice())")
            print("24h Change: \(price.formattedChange24h)")
        }
    } catch {
        print("Error: \(error)")
    }
}
```

**Expected Output:**
```
bitcoin: $43,256.78
24h Change: +2.45%
ethereum: $2,234.56
24h Change: -1.23%
```

### Calculate Portfolio Value

```swift
Task {
    // Define your tokens
    let tokens = [
        Token(
            contractAddress: "0xdac17f958d2ee523a2206206994597c13d831ec7",
            chainId: 1,
            symbol: "USDT",
            name: "Tether",
            decimals: 6,
            balance: "1000000000", // 1,000 USDT
            coingeckoId: "tether"
        )
    ]

    let accounts = [
        Account(address: "0x123...", name: "Main", chainId: 1)
    ]

    // Calculate portfolio
    let portfolio = try await marketManager.calculatePortfolioValue(
        accounts: accounts,
        tokens: tokens
    )

    print("Portfolio Value: \(portfolio.formattedTotalValue)")
    print("24h Change: \(portfolio.formattedChange24h)")
}
```

### Add to Watchlist

```swift
Task {
    // Add Bitcoin to watchlist
    try await marketManager.addToWatchlist(
        tokenId: "bitcoin",
        symbol: "BTC",
        name: "Bitcoin"
    )

    // Get watchlist with prices
    let watchlist = try await marketManager.getWatchlist()

    for (item, price) in watchlist {
        print("\(item.symbol): \(price.formattedPrice())")
    }

    // Save to persistence
    await marketManager.saveWatchlist()
}
```

### Create Price Alert

```swift
Task {
    // Alert when ETH goes below $2,000
    await marketManager.createPriceAlert(
        tokenId: "ethereum",
        tokenSymbol: "ETH",
        targetPrice: 2000.0,
        condition: .below
    )

    // Check if alert triggered
    let triggered = try await marketManager.checkPriceAlerts()

    for alert in triggered {
        print("Alert: \(alert.tokenSymbol) is \(alert.condition.symbol) $\(alert.targetPrice)")
    }

    // Save to persistence
    await marketManager.savePriceAlerts()
}
```

## Step 4: Enable Auto-Refresh

```swift
Task {
    // Enable auto-refresh every 30 seconds
    await marketManager.setAutoRefresh(enabled: true)
    await marketManager.setRefreshInterval(30)

    // Now price alerts will be checked automatically
    print("Auto-refresh enabled")
}
```

## Step 5: Discover Tokens

```swift
Task {
    let account = Account(
        address: "0x742d35Cc6634C0532925a3b844Bc9e7595f0bEb", // Example
        name: "Main Wallet",
        chainId: 1
    )

    // Discover tokens held by this wallet
    let tokens = try await marketManager.getAccountTokens(
        account: account,
        includeZeroBalance: false
    )

    print("Found \(tokens.count) tokens:")
    for token in tokens.prefix(5) {
        print("  \(token.formattedBalance) = \(token.formattedValue)")
    }
}
```

## Complete Example

Here's a complete working example:

```swift
import Foundation

@MainActor
class MarketDataManager {
    private let marketManager: MarketDataManager

    init() {
        // Initialize services
        let priceService = PriceService()
        let metadataService = TokenMetadataService()
        let discoveryService = TokenDiscoveryService(metadataService: metadataService)

        self.marketManager = MarketDataManager(
            priceService: priceService,
            metadataService: metadataService,
            discoveryService: discoveryService
        )
    }

    func start() async {
        // Load persisted data
        await marketManager.loadWatchlist()
        await marketManager.loadPriceAlerts()

        // Enable auto-refresh
        await marketManager.setAutoRefresh(enabled: true)
        await marketManager.setRefreshInterval(30)

        // Add some tokens to watchlist
        try? await marketManager.addToWatchlist(
            tokenId: "bitcoin",
            symbol: "BTC",
            name: "Bitcoin"
        )

        try? await marketManager.addToWatchlist(
            tokenId: "ethereum",
            symbol: "ETH",
            name: "Ethereum"
        )

        // Get prices
        let watchlist = try? await marketManager.getWatchlist()
        for (item, price) in watchlist ?? [] {
            print("\(item.symbol): \(price.formattedPrice()) (\(price.formattedChange24h))")
        }

        // Save state
        await marketManager.saveWatchlist()
    }
}

// Usage
Task {
    let manager = MarketDataManager()
    await manager.start()
}
```

## SwiftUI Integration

### Create ViewModel

```swift
import SwiftUI

@MainActor
class MarketViewModel: ObservableObject {
    @Published var portfolio: PortfolioSummary?
    @Published var watchlist: [(WatchlistItem, PriceData)] = []
    @Published var isLoading = false
    @Published var error: Error?

    private let marketManager: MarketDataManager

    init() {
        let priceService = PriceService()
        let metadataService = TokenMetadataService()
        let discoveryService = TokenDiscoveryService(metadataService: metadataService)

        self.marketManager = MarketDataManager(
            priceService: priceService,
            metadataService: metadataService,
            discoveryService: discoveryService
        )
    }

    func loadData() async {
        isLoading = true
        defer { isLoading = false }

        do {
            // Load watchlist
            watchlist = try await marketManager.getWatchlist()

            // Calculate portfolio
            // portfolio = try await marketManager.calculatePortfolioValue(...)

        } catch {
            self.error = error
        }
    }

    func addToWatchlist(tokenId: String, symbol: String, name: String) async {
        do {
            try await marketManager.addToWatchlist(
                tokenId: tokenId,
                symbol: symbol,
                name: name
            )
            await loadData()
        } catch {
            self.error = error
        }
    }
}
```

### Create View

```swift
struct WatchlistView: View {
    @StateObject private var viewModel = MarketViewModel()

    var body: some View {
        NavigationView {
            List {
                ForEach(viewModel.watchlist, id: \.0.id) { item, price in
                    VStack(alignment: .leading) {
                        Text(item.symbol)
                            .font(.headline)
                        HStack {
                            Text(price.formattedPrice())
                            Spacer()
                            Text(price.formattedChange24h)
                                .foregroundColor(price.isPositive ? .green : .red)
                        }
                        .font(.subheadline)
                    }
                }
            }
            .navigationTitle("Watchlist")
            .task {
                await viewModel.loadData()
            }
            .overlay {
                if viewModel.isLoading {
                    ProgressView()
                }
            }
        }
    }
}
```

## Testing Your Implementation

### Run the Examples

```swift
import Foundation

Task {
    let examples = MarketDataExample()
    await examples.runAllExamples()
}
```

This will run all 8 examples demonstrating Phase 4 features.

### Verify API Connection

```swift
Task {
    let service = PriceService()
    let prices = try? await service.getTokenPrices(ids: ["bitcoin"])

    if prices != nil {
        print("✅ API connection successful")
    } else {
        print("❌ API connection failed")
    }
}
```

## Common Issues

### Issue: Rate Limited
**Error:** `PriceServiceError.rateLimited`
**Solution:** Wait 60 seconds or enable caching

### Issue: No Internet
**Error:** `PriceServiceError.networkError`
**Solution:** Check internet connection, use cached data

### Issue: Token Not Found
**Error:** `PriceServiceError.noDataAvailable`
**Solution:** Verify token ID is correct on CoinGecko

### Issue: Invalid Address
**Error:** `TokenDiscoveryError.invalidAddress`
**Solution:** Check wallet address format (must be 0x...)

## Next Steps

1. **Read Full Documentation:** See `PHASE4_IMPLEMENTATION.md`
2. **Review Configuration:** See `CONFIGURATION.md`
3. **Explore Examples:** Check `MarketDataExample.swift`
4. **Integrate with UI:** Create SwiftUI views
5. **Add Notifications:** Implement price alert notifications
6. **Customize:** Adjust cache TTL, refresh intervals, etc.

## API Keys (Optional)

To use CoinGecko Pro for higher rate limits:

1. Sign up at https://www.coingecko.com/en/api/pricing
2. Get your API key
3. Initialize with key:

```swift
let priceService = PriceService(apiKey: "YOUR_API_KEY")
```

## Resources

- **Full Documentation:** `PHASE4_IMPLEMENTATION.md`
- **Configuration Guide:** `CONFIGURATION.md`
- **Example Code:** `Examples/MarketDataExample.swift`
- **Summary:** `PHASE4_SUMMARY.md`

## Support

For questions or issues:
- Check inline code documentation
- Review example implementations
- Contact Ëtrid development team

---

**You're all set!** Start building your market data features. 🚀
