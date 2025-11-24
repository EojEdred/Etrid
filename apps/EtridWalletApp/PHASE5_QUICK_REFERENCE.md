# Phase 5: Native iOS Features - Quick Reference Guide

## File Structure

```
EtridWallet/
├── Models/
│   ├── Notification.swift           # Notification models and settings
│   └── DeepLink.swift               # Deep link routing and parsing
├── Services/
│   ├── BiometricService.swift       # Enhanced biometric auth
│   ├── NotificationService.swift    # Push & local notifications
│   ├── DeepLinkService.swift        # URL handling & navigation
│   └── AppClipService.swift         # App Clips, Widgets, Haptics
├── Views/Settings/
│   ├── BiometricSettingsView.swift  # Biometric settings UI
│   ├── NotificationSettingsView.swift # Notification settings UI
│   └── SecuritySettingsView.swift   # Security overview UI
├── Info.plist                       # Updated with permissions & URL schemes
├── EtridWallet.entitlements         # App capabilities
└── WalletApp.swift                  # Updated with services integration
```

## Quick Usage Examples

### 1. Biometric Authentication

```swift
import SwiftUI

// In your view
@StateObject private var biometricService = BiometricService.shared

// Authenticate for transaction
try await biometricService.authenticateForTransaction(
    amount: "1.5 ETH",
    to: "0x1234...5678"
)

// Authenticate with custom reason
try await biometricService.authenticate(reason: "Confirm action")

// Secure transaction signing
let result = try await biometricService.verifyAndSignTransaction(
    amount: "1.5 ETH",
    to: recipientAddress
) {
    // Your signing logic here
    return try await wallet.signTransaction(transaction)
}

// Access sensitive data
let seedPhrase = try await biometricService.secureKeychainAccess(
    dataType: "seed phrase"
) {
    return try keychainManager.getSeedPhrase()
}
```

### 2. Push Notifications

```swift
import SwiftUI

// In your view
@StateObject private var notificationService = NotificationService.shared

// Request permission (do this once)
try await notificationService.requestAuthorization()

// Send transaction notification
await notificationService.notifyTransaction(
    hash: "0xabcd...1234",
    amount: "1.5 ETH",
    type: "sent",
    status: "Confirmed"
)

// Send price alert
await notificationService.notifyPriceAlert(
    tokenSymbol: "ETH",
    currentPrice: 2000.0,
    targetPrice: 1950.0,
    condition: "below"
)

// Send DAO voting reminder
await notificationService.notifyDAOVoting(
    proposalId: "proposal-123",
    title: "Treasury Allocation Proposal",
    endDate: Date().addingTimeInterval(86400) // 24 hours
)

// Send payment request
await notificationService.notifyPaymentRequest(
    from: "Alice",
    amount: "10.0",
    currency: "ETH"
)
```

### 3. Deep Linking

```swift
import SwiftUI

// In your view
@StateObject private var deepLinkService = DeepLinkService.shared

// Generate payment link
let paymentLink = deepLinkService.generatePaymentLink(
    address: "0x1234...5678",
    amount: "1.5",
    token: "ETH",
    chainId: 1,
    message: "Coffee payment"
)

// Generate QR code for payment
let qrCodeData = deepLinkService.generatePaymentQRCode(
    address: wallet.address,
    amount: "1.5",
    token: "ETH"
)

// Share payment request
let shareItems = deepLinkService.createShareItems(for: paymentRequest)
let activityVC = UIActivityViewController(
    activityItems: shareItems,
    applicationActivities: nil
)
present(activityVC, animated: true)
```

### 4. Haptic Feedback

```swift
// Import is automatic - HapticService is available globally

// Button press
Button("Send") {
    HapticService.shared.tap()
    // Your action
}

// Transaction feedback
HapticService.shared.transactionSent()      // Sent
HapticService.shared.transactionConfirmed() // Confirmed
HapticService.shared.transactionFailed()    // Failed

// General feedback
HapticService.shared.success()   // Success
HapticService.shared.error()     // Error
HapticService.shared.warning()   // Warning
HapticService.shared.selection() // Selection changed

// QR code scanned
HapticService.shared.qrCodeScanned()
```

### 5. Widget Updates

```swift
import SwiftUI

// Update widget balance
WidgetService.shared.updateWidgetBalance(
    balance: "1.234 ETH",
    balanceUSD: "$2,456.78",
    priceChange: 2.5,
    token: "ETH"
)

// Reload specific widget
WidgetService.shared.reloadWidget(kind: "BalanceWidget")
```

### 6. App Clips

```swift
import SwiftUI

@StateObject private var appClipService = AppClipService.shared

// Check if running as App Clip
if appClipService.isAppClip {
    // Show simplified UI
    QuickPaymentView(request: appClipService.appClipPaymentRequest)
} else {
    // Show full app UI
    MainTabView()
}

// Prompt upgrade to full app
if appClipService.shouldPromptUpgrade {
    appClipService.promptFullAppDownload()
}
```

## Environment Object Usage

Add to your views:

```swift
struct YourView: View {
    @EnvironmentObject var biometricService: BiometricService
    @EnvironmentObject var notificationService: NotificationService
    @EnvironmentObject var deepLinkService: DeepLinkService
    @EnvironmentObject var navigationCoordinator: DeepLinkNavigationCoordinator

    var body: some View {
        // Your view code
    }
}
```

## URL Scheme Examples

### Custom URL Schemes

```bash
# Send payment
etrid://send?address=0x1234&amount=1.5&token=ETH

# Receive
etrid://receive

# Transaction details
etrid://transaction/0xabcd1234

# Token details
etrid://token/0x1234

# NFT details
etrid://nft/0xcontract/tokenId

# WalletConnect
etrid://wc?uri=wc:a13aef...

# Governance
etrid://governance/proposal-123

# Settings
etrid://settings
```

### Ethereum Payment Requests (ERC-681)

```bash
ethereum:0x1234?value=1.5&chainId=1
```

### Universal Links

```bash
https://etrid.app/send/0x1234?amount=1.5
https://etrid.app/transaction/0xabcd
https://etrid.app/governance/proposal-123
```

## Testing Commands

### Deep Link Testing (Simulator)

```bash
# Payment request
xcrun simctl openurl booted "etrid://send?address=0x742d35Cc6634C0532925a3b844Bc9e7595f0bEb&amount=1.5&token=ETH"

# Receive
xcrun simctl openurl booted "etrid://receive"

# Transaction
xcrun simctl openurl booted "etrid://transaction/0xabcd1234"

# WalletConnect
xcrun simctl openurl booted "etrid://wc?uri=wc:a13aef..."

# Universal Link
xcrun simctl openurl booted "https://etrid.app/send/0x1234"
```

### Notification Testing

```swift
// In NotificationSettingsView, use the "Send Test Notification" button

// Or programmatically:
await NotificationService.shared.notifyTransaction(
    hash: "0x1234",
    amount: "Test",
    type: "sent",
    status: "Test"
)
```

## Common Integration Patterns

### Send Transaction Flow

```swift
func sendTransaction() async {
    do {
        // 1. Authenticate with biometrics
        try await biometricService.authenticateForTransaction(
            amount: amount,
            to: recipientAddress
        )

        // 2. Haptic feedback
        HapticService.shared.tap()

        // 3. Sign and send transaction
        let txHash = try await wallet.sendTransaction(...)

        // 4. Success haptic
        HapticService.shared.transactionSent()

        // 5. Send notification
        await notificationService.notifyTransaction(
            hash: txHash,
            amount: amount,
            type: "sent",
            status: "Pending"
        )

        // 6. Update widgets
        await updateBalance()

    } catch {
        // Error haptic
        HapticService.shared.error()
        showError(error)
    }
}
```

### Receive Payment Flow

```swift
func sharePaymentRequest() {
    // 1. Generate payment link
    guard let link = deepLinkService.generatePaymentLink(
        address: wallet.address,
        amount: requestAmount,
        token: selectedToken
    ) else { return }

    // 2. Generate QR code
    let qrCode = deepLinkService.generatePaymentQRCode(
        address: wallet.address,
        amount: requestAmount,
        token: selectedToken
    )

    // 3. Create payment request
    let request = PaymentRequest(
        address: wallet.address,
        amount: requestAmount,
        token: selectedToken
    )

    // 4. Share
    let items = deepLinkService.createShareItems(for: request)
    let activityVC = UIActivityViewController(
        activityItems: items,
        applicationActivities: nil
    )

    // 5. Haptic
    HapticService.shared.tap()

    // Present share sheet
    present(activityVC)
}
```

### Handle Incoming Deep Link

```swift
// Automatically handled in WalletApp.swift
// To add custom handling:

deepLinkService.setNavigationHandler { route in
    switch route {
    case .send(let address, let amount, let token):
        // Navigate to send screen with pre-filled data
        navigationCoordinator.navigate(to: .send(
            address: address,
            amount: amount,
            token: token
        ))

    case .paymentRequest(let request):
        // Show payment confirmation
        showPaymentConfirmation(request)

    case .transaction(let hash):
        // Show transaction details
        showTransactionDetails(hash)

    default:
        break
    }
}
```

## Settings Navigation

```swift
// Navigate to security settings
NavigationLink {
    SecuritySettingsView()
} label: {
    Text("Security")
}

// Navigate to biometric settings
NavigationLink {
    BiometricSettingsView()
} label: {
    Text("Biometric Authentication")
}

// Navigate to notification settings
NavigationLink {
    NotificationSettingsView()
} label: {
    Text("Notifications")
}
```

## Troubleshooting

### Biometrics Not Working
1. Check `NSFaceIDUsageDescription` in Info.plist
2. Verify device has biometrics enrolled
3. Test on physical device (simulator may have limitations)

### Notifications Not Appearing
1. Check authorization status
2. Verify `NSUserNotificationsUsageDescription` in Info.plist
3. Check notification settings in iOS Settings app
4. Test on physical device

### Deep Links Not Working
1. Verify URL schemes in Info.plist
2. Check entitlements file
3. Verify Universal Links configuration
4. Test with `xcrun simctl openurl` command

### Widgets Not Updating
1. Check App Group is configured
2. Verify shared UserDefaults suite name
3. Call `WidgetCenter.shared.reloadAllTimelines()`
4. Check widget extension target settings

## Production Checklist

- [ ] Configure App Groups in Apple Developer Portal
- [ ] Set up APNs certificates
- [ ] Configure Universal Links (apple-app-site-association)
- [ ] Test on physical devices
- [ ] Enable push notifications in Xcode capabilities
- [ ] Add associated domains in Xcode capabilities
- [ ] Test all URL schemes
- [ ] Test biometric authentication
- [ ] Test notification delivery
- [ ] Test widget updates
- [ ] Verify entitlements are correct
- [ ] Test App Clip (if implemented)

## Support

For issues or questions:
1. Check the full implementation report: `PHASE5_IMPLEMENTATION_REPORT.md`
2. Review service code comments
3. Test with provided examples
4. Check iOS documentation for native features

All services are singleton instances accessible via `.shared` property.
