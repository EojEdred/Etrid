# Ëtrid Wallet Swift - Configuration Guide

## Phase 4: Price & Token Discovery Configuration

### API Keys (Optional)

#### CoinGecko API Key
To use CoinGecko Pro API for higher rate limits:

```swift
// In your app initialization
let priceService = PriceService(apiKey: "YOUR_COINGECKO_API_KEY")
```

**Free Tier:**
- No API key needed
- 50 calls/minute
- Public data only

**Pro Tier:**
- API key required
- Higher rate limits
- Sign up at: https://www.coingecko.com/en/api/pricing

---

### RPC Endpoints

Default public RPC endpoints are pre-configured:

```swift
// Ethereum
"https://eth.llamarpc.com"

// BNB Smart Chain
"https://bsc-dataseed.binance.org"

// Polygon
"https://polygon-rpc.com"

// Avalanche
"https://api.avax.network/ext/bc/C/rpc"

// Fantom
"https://rpc.ftm.tools"

// Arbitrum
"https://arb1.arbitrum.io/rpc"

// Optimism
"https://mainnet.optimism.io"
```

**Custom RPC Endpoints:**

To use your own RPC endpoints (e.g., Infura, Alchemy):

```swift
let customNetwork = Network(
    chainId: 1,
    name: "Ethereum (Infura)",
    rpcURL: "https://mainnet.infura.io/v3/YOUR_PROJECT_ID",
    explorerURL: "https://etherscan.io",
    nativeCurrency: "ETH"
)
```

---

### Cache Configuration

Default cache settings can be customized in each service:

#### Price Service Cache
```swift
// In PriceService.swift
private let priceCacheTTL: TimeInterval = 60 // 1 minute
private let chartCacheTTL: TimeInterval = 300 // 5 minutes
```

#### Metadata Service Cache
```swift
// In TokenMetadataService.swift
func isValid(ttl: TimeInterval = 3600) -> Bool // 1 hour
```

#### Discovery Service Cache
```swift
// In TokenDiscoveryService.swift
private let cacheTTL: TimeInterval = 300 // 5 minutes
```

---

### Rate Limiting

Configure rate limits in PriceService:

```swift
// In PriceService.swift
private let minRequestInterval: TimeInterval = 1.0 // 1 second between requests
private let maxRequestsPerMinute: Int = 50 // 50 requests per minute
```

---

### Auto-Refresh Settings

Configure auto-refresh behavior:

```swift
// Enable auto-refresh with 30-second interval
await marketManager.setAutoRefresh(enabled: true)
await marketManager.setRefreshInterval(30) // seconds
```

**Recommended Intervals:**
- **Active Trading:** 10-15 seconds
- **Normal Use:** 30-60 seconds
- **Background:** 5-10 minutes
- **Battery Saving:** Disable auto-refresh

---

### Persistence

Data is automatically persisted to UserDefaults:

```swift
// Keys used
let watchlistKey = "watchlist"
let priceAlertsKey = "priceAlerts"
```

**Manual Persistence:**
```swift
// Save watchlist
await marketManager.saveWatchlist()

// Load watchlist
await marketManager.loadWatchlist()

// Save price alerts
await marketManager.savePriceAlerts()

// Load price alerts
await marketManager.loadPriceAlerts()
```

---

### Currency Preference

Set preferred display currency:

```swift
// Set currency (default: "usd")
await marketManager.setPreferredCurrency("usd") // or "eur", "gbp"
```

**Supported Currencies:**
- USD (default)
- EUR
- GBP

---

### Watchlist Limits

```swift
private let maxWatchlistItems = 50
```

To change the limit, modify in MarketDataManager.swift.

---

### Network Timeouts

Default URLSession timeout:

```swift
// Configure custom timeout
var request = URLRequest(url: url)
request.timeoutInterval = 30.0 // 30 seconds
```

---

### Error Handling

Configure retry logic:

```swift
// Example: Retry on network errors
do {
    let prices = try await priceService.getTokenPrices(ids: tokenIds)
} catch PriceServiceError.networkError {
    // Retry after delay
    try await Task.sleep(nanoseconds: 2_000_000_000) // 2 seconds
    let prices = try await priceService.getTokenPrices(ids: tokenIds)
}
```

---

### Memory Management

NSCache limits can be configured:

```swift
// In TokenMetadataService.swift
init() {
    self.cache = NSCache<NSString, CachedMetadata>()
    self.cache.countLimit = 1000 // Store up to 1000 token metadata entries
}
```

---

### Debug Mode

Enable logging for debugging:

```swift
// Add to your services
#if DEBUG
print("🔍 [PriceService] Fetching prices for: \(ids)")
#endif
```

---

### Environment-Specific Configuration

#### Development
```swift
let priceService = PriceService() // Free tier, no API key
let refreshInterval: TimeInterval = 60 // 1 minute
```

#### Production
```swift
let priceService = PriceService(apiKey: Config.coingeckoAPIKey)
let refreshInterval: TimeInterval = 30 // 30 seconds
```

---

### Info.plist Configuration

Add to your Info.plist for network access:

```xml
<key>NSAppTransportSecurity</key>
<dict>
    <key>NSAllowsArbitraryLoads</key>
    <true/>
</dict>
```

Or more securely, allow specific domains:

```xml
<key>NSAppTransportSecurity</key>
<dict>
    <key>NSExceptionDomains</key>
    <dict>
        <key>coingecko.com</key>
        <dict>
            <key>NSIncludesSubdomains</key>
            <true/>
            <key>NSTemporaryExceptionAllowsInsecureHTTPLoads</key>
            <true/>
        </dict>
    </dict>
</dict>
```

---

### Background Fetch (Optional)

To enable background price updates:

1. Add Background Modes capability
2. Enable "Background fetch"
3. Implement background task:

```swift
import BackgroundTasks

func registerBackgroundTasks() {
    BGTaskScheduler.shared.register(
        forTaskWithIdentifier: "com.etrid.wallet.refresh",
        using: nil
    ) { task in
        self.handlePriceRefresh(task: task as! BGAppRefreshTask)
    }
}

func handlePriceRefresh(task: BGAppRefreshTask) {
    Task {
        // Check price alerts
        let triggered = try? await marketManager.checkPriceAlerts()

        // Send notifications for triggered alerts
        for alert in triggered ?? [] {
            sendNotification(for: alert)
        }

        task.setTaskCompleted(success: true)
    }
}
```

---

### Push Notifications (Optional)

To notify users of price alerts:

1. Request notification permissions
2. Configure notifications:

```swift
import UserNotifications

func sendNotification(for alert: PriceAlert) {
    let content = UNMutableNotificationContent()
    content.title = "Price Alert"
    content.body = "\(alert.tokenSymbol) is \(alert.condition.symbol) $\(alert.targetPrice)"
    content.sound = .default

    let request = UNNotificationRequest(
        identifier: alert.id.uuidString,
        content: content,
        trigger: nil
    )

    UNUserNotificationCenter.current().add(request)
}
```

---

### Recommended App Initialization

```swift
@main
struct EtridWalletApp: App {
    @StateObject private var marketViewModel = MarketViewModel()

    var body: some Scene {
        WindowGroup {
            ContentView()
                .environmentObject(marketViewModel)
                .task {
                    await marketViewModel.initialize()
                }
        }
    }
}

@MainActor
class MarketViewModel: ObservableObject {
    private let marketManager: MarketDataManager

    init() {
        let priceService = PriceService(apiKey: Config.coingeckoAPIKey)
        let metadataService = TokenMetadataService()
        let discoveryService = TokenDiscoveryService(metadataService: metadataService)

        self.marketManager = MarketDataManager(
            priceService: priceService,
            metadataService: metadataService,
            discoveryService: discoveryService
        )
    }

    func initialize() async {
        // Load persisted data
        await marketManager.loadWatchlist()
        await marketManager.loadPriceAlerts()

        // Enable auto-refresh
        await marketManager.setAutoRefresh(enabled: true)
        await marketManager.setRefreshInterval(30)

        // Set preferred currency
        await marketManager.setPreferredCurrency("usd")
    }
}
```

---

### Testing Configuration

For unit tests, use mock services:

```swift
// Create mock price service
class MockPriceService: PriceService {
    override func getTokenPrices(ids: [String]) async throws -> [String: PriceData] {
        return [
            "bitcoin": PriceData(
                price: 50000,
                change24h: 5.0,
                change7d: 10.0,
                change1h: 1.0,
                marketCap: 1_000_000_000_000,
                volume24h: 50_000_000_000,
                circulatingSupply: 19_000_000,
                totalSupply: 21_000_000,
                rank: 1,
                lastUpdated: Date()
            )
        ]
    }
}
```

---

### Performance Optimization Tips

1. **Batch Requests:** Always fetch prices in batches
2. **Cache First:** Check cache before making API calls
3. **Debounce:** Don't refresh too frequently
4. **Lazy Loading:** Load metadata only when needed
5. **Background Tasks:** Use for non-urgent updates

---

### Security Considerations

1. **API Keys:** Store in Keychain, not in code
2. **HTTPS:** Always use HTTPS for API calls
3. **Input Validation:** Validate all addresses and IDs
4. **Rate Limiting:** Respect API rate limits
5. **Error Handling:** Don't expose sensitive errors to users

---

### Monitoring & Analytics

Track important metrics:

```swift
// Example: Track API performance
let startTime = Date()
let prices = try await priceService.getTokenPrices(ids: tokenIds)
let duration = Date().timeIntervalSince(startTime)

Analytics.track("price_fetch_success", properties: [
    "duration": duration,
    "token_count": tokenIds.count
])
```

---

## Need Help?

- Check PHASE4_IMPLEMENTATION.md for detailed documentation
- See MarketDataExample.swift for working examples
- Review inline code comments for specific functions

---

**Configuration Last Updated:** November 22, 2025
