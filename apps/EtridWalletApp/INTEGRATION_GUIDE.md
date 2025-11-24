# Phase 6 Integration Guide

## Quick Start: Adding Enterprise Features to Main App

### 1. Update ContentView.swift

Add navigation links to the enterprise features:

```swift
import SwiftUI

struct ContentView: View {
    var body: some View {
        TabView {
            // Existing tabs...

            // GPU Marketplace Tab
            GPUMarketplaceView()
                .tabItem {
                    Label("GPU", systemImage: "cpu")
                }

            // Hyperledger Bridge Tab
            HyperledgerBridgeView()
                .tabItem {
                    Label("Bridge", systemImage: "arrow.left.arrow.right")
                }

            // ETH PBC Tab
            ETHPBCView()
                .tabItem {
                    Label("PBC", systemImage: "bolt.fill")
                }
        }
    }
}
```

### 2. Alternative: Dashboard Integration

Add enterprise features to an existing dashboard:

```swift
struct DashboardView: View {
    var body: some View {
        ScrollView {
            VStack(spacing: 20) {
                // Existing dashboard content...

                // Enterprise Features Section
                VStack(alignment: .leading, spacing: 12) {
                    Text("Enterprise Features")
                        .font(.headline)

                    HStack(spacing: 12) {
                        NavigationLink(destination: GPUMarketplaceView()) {
                            FeatureCard(
                                icon: "cpu",
                                title: "GPU Marketplace",
                                color: .blue
                            )
                        }

                        NavigationLink(destination: HyperledgerBridgeView()) {
                            FeatureCard(
                                icon: "arrow.left.arrow.right",
                                title: "Hyperledger Bridge",
                                color: .purple
                            )
                        }
                    }

                    NavigationLink(destination: ETHPBCView()) {
                        FeatureCard(
                            icon: "bolt.fill",
                            title: "ETH PBC",
                            color: .orange,
                            wide: true
                        )
                    }
                }
                .padding()
            }
        }
    }
}

struct FeatureCard: View {
    let icon: String
    let title: String
    let color: Color
    var wide: Bool = false

    var body: some View {
        HStack {
            Image(systemName: icon)
                .font(.title)
                .foregroundColor(color)
            Text(title)
                .font(.headline)
            Spacer()
        }
        .padding()
        .frame(maxWidth: wide ? .infinity : nil)
        .background(color.opacity(0.1))
        .cornerRadius(12)
    }
}
```

### 3. Environment Configuration

Create a configuration file for API endpoints:

```swift
// Config/APIConfig.swift
struct APIConfig {
    static let shared = APIConfig()

    #if DEBUG
    let gpuBaseURL = "https://dev-api.etrid.io/v1/gpu"
    let hyperledgerBaseURL = "https://dev-api.etrid.io/v1/hyperledger"
    let ethpbcBaseURL = "https://dev-api.etrid.io/v1/ethpbc"
    let ethpbcRPCURL = "https://dev-rpc.ethpbc.etrid.io"
    #else
    let gpuBaseURL = "https://api.etrid.io/v1/gpu"
    let hyperledgerBaseURL = "https://api.etrid.io/v1/hyperledger"
    let ethpbcBaseURL = "https://api.etrid.io/v1/ethpbc"
    let ethpbcRPCURL = "https://rpc.ethpbc.etrid.io"
    #endif
}
```

Update services to use config:

```swift
// In GPUMarketplaceService.swift
private let baseURL = APIConfig.shared.gpuBaseURL

// In HyperledgerBridgeService.swift
private let baseURL = APIConfig.shared.hyperledgerBaseURL

// In ETHPBCService.swift
private let baseURL = APIConfig.shared.ethpbcBaseURL
private let rpcURL = APIConfig.shared.ethpbcRPCURL
```

### 4. Wallet Integration

Connect services to actual wallet:

```swift
// Utils/WalletProvider.swift
class WalletProvider: ObservableObject {
    @Published var address: String = ""
    @Published var balance: String = "0"

    static let shared = WalletProvider()

    func getCurrentAddress() -> String {
        return address
    }

    func getBalance() async throws -> String {
        // Implement actual balance fetch
        return balance
    }
}
```

Update services to use WalletProvider:

```swift
// In service methods, replace hardcoded addresses:
let address = WalletProvider.shared.getCurrentAddress()
```

### 5. State Management

Create a shared app state:

```swift
// Core/AppState.swift
class AppState: ObservableObject {
    @Published var isConnected = false
    @Published var currentNetwork = "mainnet"
    @Published var walletAddress = ""

    static let shared = AppState()

    // GPU Marketplace State
    @Published var activeRentals: [Rental] = []

    // Hyperledger State
    @Published var connectedFabricNetwork: FabricNetwork?
    @Published var pendingBridges: [BridgeTransaction] = []

    // ETH PBC State
    @Published var stakingInfo: StakingInfo?
    @Published var activeProposals: [GovernanceProposal] = []
}
```

Inject into views:

```swift
struct GPUMarketplaceView: View {
    @StateObject private var service = GPUMarketplaceService()
    @ObservedObject var appState = AppState.shared

    // Use appState for global data
}
```

### 6. Deep Linking

Support deep links to enterprise features:

```swift
// In WalletApp.swift
@main
struct WalletApp: App {
    var body: some Scene {
        WindowGroup {
            ContentView()
                .onOpenURL { url in
                    handleDeepLink(url)
                }
        }
    }

    func handleDeepLink(_ url: URL) {
        // etrid://gpu/marketplace
        // etrid://hyperledger/bridge
        // etrid://ethpbc/staking

        guard let scheme = url.scheme, scheme == "etrid" else { return }

        switch url.host {
        case "gpu":
            // Navigate to GPU marketplace
            break
        case "hyperledger":
            // Navigate to bridge
            break
        case "ethpbc":
            // Navigate to precompile service
            break
        default:
            break
        }
    }
}
```

### 7. Notifications

Add push notification support for rentals and bridges:

```swift
// Utils/NotificationManager.swift
class NotificationManager {
    static let shared = NotificationManager()

    func scheduleRentalExpiry(rental: Rental) {
        let content = UNMutableNotificationContent()
        content.title = "GPU Rental Expiring"
        content.body = "Your rental of \(rental.gpu?.specs.displayName ?? "GPU") expires in 1 hour"
        content.sound = .default

        let trigger = UNTimeIntervalNotificationTrigger(
            timeInterval: rental.endTime.timeIntervalSinceNow - 3600,
            repeats: false
        )

        let request = UNNotificationRequest(
            identifier: rental.id,
            content: content,
            trigger: trigger
        )

        UNUserNotificationCenter.current().add(request)
    }

    func notifyBridgeComplete(transaction: BridgeTransaction) {
        let content = UNMutableNotificationContent()
        content.title = "Bridge Complete"
        content.body = "Successfully bridged \(transaction.amountDisplay)"
        content.sound = .default

        let request = UNNotificationRequest(
            identifier: transaction.id,
            content: content,
            trigger: nil
        )

        UNUserNotificationCenter.current().add(request)
    }
}
```

### 8. Analytics Integration

Track user interactions:

```swift
// Utils/AnalyticsManager.swift
class AnalyticsManager {
    static let shared = AnalyticsManager()

    func trackGPURental(gpu: GPU, duration: Int, cost: Double) {
        // Send to analytics service
        print("Rental: \(gpu.id), Duration: \(duration)h, Cost: \(cost) ETR")
    }

    func trackBridgeTransaction(amount: String, direction: BridgeDirection) {
        // Send to analytics service
        print("Bridge: \(amount), Direction: \(direction.displayName)")
    }

    func trackPrecompileCall(precompile: PrecompileAddress, method: String) {
        // Send to analytics service
        print("Precompile: \(precompile.name), Method: \(method)")
    }
}
```

Add to service methods:

```swift
// In GPUMarketplaceService.rentGPU
AnalyticsManager.shared.trackGPURental(gpu: gpu, duration: durationHours, cost: totalCost)

// In HyperledgerBridgeService.bridgeToFabric
AnalyticsManager.shared.trackBridgeTransaction(amount: amount, direction: .toFabric)

// In ETHPBCService.callPrecompile
AnalyticsManager.shared.trackPrecompileCall(precompile: address, method: method)
```

### 9. Error Tracking

Integrate error reporting:

```swift
// Utils/ErrorReporter.swift
class ErrorReporter {
    static let shared = ErrorReporter()

    func report(_ error: Error, context: [String: Any] = [:]) {
        // Send to error tracking service (Sentry, Firebase, etc.)
        print("Error: \(error.localizedDescription)")
        print("Context: \(context)")

        // You can also show user-friendly error alerts
        DispatchQueue.main.async {
            // Show alert to user
        }
    }
}
```

Add to service catch blocks:

```swift
catch {
    ErrorReporter.shared.report(error, context: [
        "service": "GPUMarketplace",
        "method": "rentGPU",
        "gpuId": gpuId
    ])
    throw error
}
```

### 10. Testing Integration

Create mock services for testing:

```swift
// Mocks/MockGPUService.swift
class MockGPUMarketplaceService: GPUMarketplaceService {
    override func searchGPUs(...) async throws -> GPUSearchResponse {
        return GPUSearchResponse(
            gpus: [
                GPU(id: "1", providerId: "p1", specs: mockSpecs, ...)
            ],
            total: 1,
            page: 1,
            totalPages: 1
        )
    }
}
```

Use in previews:

```swift
struct GPUMarketplaceView_Previews: PreviewProvider {
    static var previews: some View {
        GPUMarketplaceView()
            .environmentObject(MockGPUMarketplaceService())
    }
}
```

---

## Quick Test Checklist

After integration, test these flows:

### GPU Marketplace
- [ ] Navigate to GPU marketplace
- [ ] Search and filter GPUs
- [ ] View GPU details
- [ ] Rent a GPU
- [ ] View active rentals
- [ ] Access SSH credentials
- [ ] View usage metrics
- [ ] Register as provider

### Hyperledger Bridge
- [ ] Navigate to bridge
- [ ] Select Fabric network
- [ ] Bridge to Fabric
- [ ] Bridge from Fabric
- [ ] View transaction history
- [ ] Verify transaction
- [ ] Check endorsements

### ETH PBC
- [ ] Navigate to PBC dashboard
- [ ] Check oracle prices
- [ ] View governance proposals
- [ ] Cast a vote
- [ ] Stake tokens
- [ ] Wrap/Unwrap ETH
- [ ] View transaction history

---

## Common Issues & Solutions

### Issue: Views not appearing in navigation
**Solution:** Ensure all view files are added to Xcode target membership

### Issue: Services returning errors
**Solution:** Check API endpoints in APIConfig and network connectivity

### Issue: Models not encoding/decoding properly
**Solution:** Verify JSON structure matches model properties

### Issue: State not updating in UI
**Solution:** Use @StateObject for service instances, @ObservedObject for shared state

---

## Performance Optimization

### Lazy Loading
```swift
// In list views, use LazyVStack instead of VStack
LazyVStack {
    ForEach(items) { item in
        ItemView(item: item)
    }
}
```

### Caching
```swift
// Cache expensive operations
class CacheManager {
    static let shared = CacheManager()
    private var cache: [String: Any] = [:]

    func get<T>(_ key: String) -> T? {
        return cache[key] as? T
    }

    func set(_ key: String, value: Any) {
        cache[key] = value
    }
}
```

### Debouncing
```swift
// For search fields
.onChange(of: searchText) { newValue in
    NSObject.cancelPreviousPerformRequests(withTarget: self)
    self.perform(#selector(search), with: nil, afterDelay: 0.5)
}
```

---

## Next Steps

1. Update Info.plist with required permissions
2. Add network security exceptions if testing on HTTP
3. Configure Firebase/Analytics
4. Set up push notification certificates
5. Test on physical device
6. Submit to TestFlight

---

Generated: November 22, 2025
