//
//  AppClipService.swift
//  EtridWallet
//
//  App Clips and Widgets support for quick payments
//

import Foundation
import WidgetKit
import SwiftUI

// MARK: - App Clip Service
@MainActor
class AppClipService: ObservableObject {
    // MARK: - Published Properties
    @Published var isAppClip = false
    @Published var appClipDeepLinkPaymentRequest: DeepLinkPaymentRequest?

    // MARK: - Singleton
    static let shared = AppClipService()

    private init() {
        checkIfAppClip()
    }

    // MARK: - App Clip Detection
    private func checkIfAppClip() {
        #if APPCLIP
        isAppClip = true
        #else
        isAppClip = false
        #endif
    }

    // MARK: - App Clip Invocation
    /// Handles App Clip invocation URL
    func handleAppClipInvocation(_ url: URL) {
        guard isAppClip else { return }

        if let route = DeepLinkParser.parse(url) {
            switch route {
            case .paymentRequest(let request):
                appClipDeepLinkPaymentRequest = request
            case .send(let address, let amount, let token):
                if let addr = address {
                    appClipDeepLinkPaymentRequest = DeepLinkPaymentRequest(
                        address: addr,
                        amount: amount,
                        token: token
                    )
                }
            default:
                break
            }
        }
    }

    // MARK: - App Clip to Full App Upgrade
    /// Prompts user to download full app
    func promptFullAppDownload() {
        // In a real implementation, this would use StoreKit's SKOverlay
        // to show the App Store download sheet
        #if APPCLIP
        print("Prompting user to download full app")
        #endif
    }

    /// Checks if user should be prompted to upgrade
    var shouldPromptUpgrade: Bool {
        guard isAppClip else { return false }

        // Prompt after successful payment or after 3 uses
        let usageCount = UserDefaults.standard.integer(forKey: "appClipUsageCount")
        return usageCount >= 3
    }

    /// Increments usage count
    func incrementUsageCount() {
        let current = UserDefaults.standard.integer(forKey: "appClipUsageCount")
        UserDefaults.standard.set(current + 1, forKey: "appClipUsageCount")
    }
}

// MARK: - Widget Service
@MainActor
class WidgetService: ObservableObject {
    // MARK: - Widget Data
    struct WidgetData: Codable {
        let balance: String
        let balanceUSD: String
        let priceChange24h: Double
        let lastUpdated: Date
        let tokenSymbol: String

        init(balance: String = "0.00",
             balanceUSD: String = "$0.00",
             priceChange24h: Double = 0.0,
             lastUpdated: Date = Date(),
             tokenSymbol: String = "ETH") {
            self.balance = balance
            self.balanceUSD = balanceUSD
            self.priceChange24h = priceChange24h
            self.lastUpdated = lastUpdated
            self.tokenSymbol = tokenSymbol
        }
    }

    // MARK: - Singleton
    static let shared = WidgetService()

    private init() {}

    // MARK: - Widget Update Methods
    /// Updates widget with latest balance
    func updateWidgetBalance(balance: String, balanceUSD: String, priceChange: Double, token: String = "ETH") {
        let widgetData = WidgetData(
            balance: balance,
            balanceUSD: balanceUSD,
            priceChange24h: priceChange,
            lastUpdated: Date(),
            tokenSymbol: token
        )

        saveWidgetData(widgetData)
        reloadAllWidgets()
    }

    /// Saves widget data to shared container
    private func saveWidgetData(_ data: WidgetData) {
        guard let sharedDefaults = UserDefaults(suiteName: "group.com.etrid.wallet") else {
            return
        }

        if let encoded = try? JSONEncoder().encode(data) {
            sharedDefaults.set(encoded, forKey: "widgetData")
        }
    }

    /// Loads widget data from shared container
    static func loadWidgetData() -> WidgetData {
        guard let sharedDefaults = UserDefaults(suiteName: "group.com.etrid.wallet"),
              let data = sharedDefaults.data(forKey: "widgetData"),
              let decoded = try? JSONDecoder().decode(WidgetData.self, from: data) else {
            return WidgetData()
        }

        return decoded
    }

    /// Reloads all widgets
    private func reloadAllWidgets() {
        WidgetCenter.shared.reloadAllTimelines()
    }

    /// Reloads specific widget
    func reloadWidget(kind: String) {
        WidgetCenter.shared.reloadTimelines(ofKind: kind)
    }
}

// MARK: - Siri Shortcuts Service
@MainActor
class SiriShortcutsService: ObservableObject {
    // MARK: - Singleton
    static let shared = SiriShortcutsService()

    private init() {}

    // MARK: - Shortcut Types
    enum ShortcutType: String, CaseIterable {
        case viewBalance = "view_balance"
        case sendPayment = "send_payment"
        case receivePayment = "receive_payment"
        case checkPrice = "check_price"

        var title: String {
            switch self {
            case .viewBalance:
                return "View Balance"
            case .sendPayment:
                return "Send Payment"
            case .receivePayment:
                return "Receive Payment"
            case .checkPrice:
                return "Check Price"
            }
        }

        var systemImageName: String {
            switch self {
            case .viewBalance:
                return "dollarsign.circle"
            case .sendPayment:
                return "arrow.up.circle"
            case .receivePayment:
                return "arrow.down.circle"
            case .checkPrice:
                return "chart.line.uptrend.xyaxis"
            }
        }
    }

    // MARK: - Shortcut Donation
    /// Donates shortcut to Siri
    func donateShortcut(type: ShortcutType) {
        #if os(iOS)
        // In a real implementation, this would use Intents framework
        // to donate shortcuts to Siri
        print("Donating shortcut: \(type.title)")
        #endif
    }

    // MARK: - Shortcut Handling
    /// Handles incoming shortcut
    func handleShortcut(type: ShortcutType) {
        switch type {
        case .viewBalance:
            handleViewBalance()
        case .sendPayment:
            handleSendPayment()
        case .receivePayment:
            handleReceivePayment()
        case .checkPrice:
            handleCheckPrice()
        }
    }

    private func handleViewBalance() {
        // Navigate to balance view
        DeepLinkService.shared.handleURL(URL(string: "etrid://home")!)
    }

    private func handleSendPayment() {
        // Navigate to send view
        DeepLinkService.shared.handleURL(URL(string: "etrid://send")!)
    }

    private func handleReceivePayment() {
        // Navigate to receive view
        DeepLinkService.shared.handleURL(URL(string: "etrid://receive")!)
    }

    private func handleCheckPrice() {
        // Navigate to price chart
        DeepLinkService.shared.handleURL(URL(string: "etrid://home")!)
    }
}
