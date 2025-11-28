//
//  AppConfig.swift
//  EtridWallet
//
//  Created on 2025-01-22.
//  Copyright © 2025 Ëtrid. All rights reserved.
//

import Foundation
import SwiftUI

/// Centralized application configuration with feature flags
@MainActor
class AppConfig: ObservableObject {
    static let shared = AppConfig()

    // MARK: - Environment

    @Published var environment: Environment = .production

    enum Environment: String, CaseIterable {
        case development = "Development"
        case staging = "Staging"
        case production = "Production"

        var baseURL: String {
            switch self {
            case .development:
                return "https://dev-api.etrid.org"
            case .staging:
                return "https://staging-api.etrid.org"
            case .production:
                return "https://api.etrid.org"
            }
        }

        var rpcEndpoint: String {
            switch self {
            case .development:
                return "https://dev-rpc.etrid.org/primearc"
            case .staging:
                return "https://staging-rpc.etrid.org/primearc"
            case .production:
                return "https://rpc.etrid.org/primearc"
            }
        }

        var explorerURL: String {
            switch self {
            case .development:
                return "https://dev-explorer.etrid.org"
            case .staging:
                return "https://staging-explorer.etrid.org"
            case .production:
                return "https://explorer.etrid.org"
            }
        }

        var chainId: Int {
            switch self {
            case .development:
                return 31337 // Local
            case .staging:
                return 1337 // Testnet
            case .production:
                return 137 // Mainnet
            }
        }

        var isProduction: Bool {
            return self == .production
        }
    }

    // MARK: - Feature Flags

    @Published var featureFlags: [String: Bool] = [:]

    // Feature flag keys
    static let enableWalletConnect = "enable_walletconnect"
    static let enablePriceCharts = "enable_price_charts"
    static let enableBiometrics = "enable_biometrics"
    static let enableNFTs = "enable_nfts"
    static let enableStaking = "enable_staking"
    static let enableSwap = "enable_swap"
    static let enableAnalytics = "enable_analytics"
    static let enableCrashReporting = "enable_crash_reporting"
    static let enableBetaFeatures = "enable_beta_features"
    static let enableDebugMenu = "enable_debug_menu"
    static let enablePerformanceMonitoring = "enable_performance_monitoring"
    static let enableMultipleWallets = "enable_multiple_wallets"

    func isFeatureEnabled(_ feature: String) -> Bool {
        return featureFlags[feature] ?? false
    }

    func setFeature(_ feature: String, enabled: Bool) {
        featureFlags[feature] = enabled
        saveFeatureFlags()

        logInfo("Feature '\(feature)' \(enabled ? "enabled" : "disabled")", category: .general)
    }

    func toggleFeature(_ feature: String) {
        let currentValue = isFeatureEnabled(feature)
        setFeature(feature, enabled: !currentValue)
    }

    // MARK: - Network Configuration

    struct NetworkConfig {
        let rpcEndpoint: String
        let chainId: Int
        let chainName: String
        let nativeCurrency: CurrencyInfo
        let blockExplorerURL: String
        let timeout: TimeInterval
        let maxRetries: Int

        struct CurrencyInfo {
            let name: String
            let symbol: String
            let decimals: Int
        }
    }

    func getNetworkConfig() -> NetworkConfig {
        return NetworkConfig(
            rpcEndpoint: environment.rpcEndpoint,
            chainId: environment.chainId,
            chainName: "Ëtrid Network",
            nativeCurrency: NetworkConfig.CurrencyInfo(
                name: "Ëtrid",
                symbol: "ETRD",
                decimals: 18
            ),
            blockExplorerURL: environment.explorerURL,
            timeout: getTimeout(),
            maxRetries: getMaxRetries()
        )
    }

    // MARK: - API Configuration

    struct APIConfig {
        let baseURL: String
        let timeout: TimeInterval
        let maxRetries: Int
        let apiKey: String?

        var headers: [String: String] {
            var headers = [
                "Content-Type": "application/json",
                "User-Agent": "EtridWallet/\(AppConfig.shared.appVersion)"
            ]

            if let apiKey = apiKey {
                headers["X-API-Key"] = apiKey
            }

            return headers
        }
    }

    func getAPIConfig() -> APIConfig {
        return APIConfig(
            baseURL: environment.baseURL,
            timeout: getTimeout(),
            maxRetries: getMaxRetries(),
            apiKey: getAPIKey()
        )
    }

    private func getAPIKey() -> String? {
        // Load from secure storage or environment
        return nil
    }

    // MARK: - App Settings

    @Published var timeoutSeconds: TimeInterval = 30.0
    @Published var maxRetries: Int = 3
    @Published var enableLogging: Bool = true
    @Published var logLevel: Logger.LogLevel = .info
    @Published var enableHaptics: Bool = true
    @Published var enableNotifications: Bool = true

    func getTimeout() -> TimeInterval {
        return timeoutSeconds
    }

    func getMaxRetries() -> Int {
        return maxRetries
    }

    // MARK: - App Info

    var appVersion: String {
        Bundle.main.infoDictionary?["CFBundleShortVersionString"] as? String ?? "1.0.0"
    }

    var buildNumber: String {
        Bundle.main.infoDictionary?["CFBundleVersion"] as? String ?? "1"
    }

    var bundleIdentifier: String {
        Bundle.main.bundleIdentifier ?? "com.etrid.wallet"
    }

    var fullVersion: String {
        "\(appVersion) (\(buildNumber))"
    }

    // MARK: - Security Settings

    struct SecurityConfig {
        var biometricAuthEnabled: Bool = false
        var autoLockEnabled: Bool = true
        var autoLockTimeout: TimeInterval = 300 // 5 minutes
        var requireAuthForTransactions: Bool = true
        var showBalanceOnLaunch: Bool = false
        var allowScreenshots: Bool = false
    }

    @Published var securityConfig = SecurityConfig()

    // MARK: - UI Settings

    struct UIConfig {
        var theme: Theme = .system
        var primaryColor: String = "#6366F1" // Indigo
        var fontSize: FontSize = .medium
        var showTestnetWarning: Bool = true

        enum Theme: String, CaseIterable {
            case light = "Light"
            case dark = "Dark"
            case system = "System"
        }

        enum FontSize: String, CaseIterable {
            case small = "Small"
            case medium = "Medium"
            case large = "Large"
            case extraLarge = "Extra Large"

            var scale: CGFloat {
                switch self {
                case .small: return 0.9
                case .medium: return 1.0
                case .large: return 1.1
                case .extraLarge: return 1.2
                }
            }
        }
    }

    @Published var uiConfig = UIConfig()

    // MARK: - Initialization

    private init() {
        loadConfiguration()
        setupDefaultFeatureFlags()
    }

    private func setupDefaultFeatureFlags() {
        // Set default feature flags based on environment
        let defaults: [String: Bool] = [
            Self.enableWalletConnect: true,
            Self.enablePriceCharts: true,
            Self.enableBiometrics: true,
            Self.enableNFTs: environment != .production, // Beta feature
            Self.enableStaking: true,
            Self.enableSwap: true,
            Self.enableAnalytics: environment.isProduction,
            Self.enableCrashReporting: environment.isProduction,
            Self.enableBetaFeatures: environment != .production,
            Self.enableDebugMenu: environment == .development,
            Self.enablePerformanceMonitoring: true,
            Self.enableMultipleWallets: environment != .production
        ]

        // Only set if not already configured
        for (key, value) in defaults {
            if featureFlags[key] == nil {
                featureFlags[key] = value
            }
        }
    }

    // MARK: - Persistence

    private func loadConfiguration() {
        let defaults = UserDefaults.standard

        // Load environment
        if let envString = defaults.string(forKey: "app_environment"),
           let env = Environment(rawValue: envString) {
            environment = env
        }

        // Load feature flags
        if let flagsData = defaults.data(forKey: "feature_flags"),
           let flags = try? JSONDecoder().decode([String: Bool].self, from: flagsData) {
            featureFlags = flags
        }

        // Load settings
        timeoutSeconds = defaults.double(forKey: "timeout_seconds")
        if timeoutSeconds == 0 { timeoutSeconds = 30.0 }

        maxRetries = defaults.integer(forKey: "max_retries")
        if maxRetries == 0 { maxRetries = 3 }

        enableLogging = defaults.bool(forKey: "enable_logging")
        if defaults.object(forKey: "enable_logging") == nil {
            enableLogging = true
        }

        enableHaptics = defaults.bool(forKey: "enable_haptics")
        if defaults.object(forKey: "enable_haptics") == nil {
            enableHaptics = true
        }

        enableNotifications = defaults.bool(forKey: "enable_notifications")
        if defaults.object(forKey: "enable_notifications") == nil {
            enableNotifications = true
        }

        // Load security config
        loadSecurityConfig()

        // Load UI config
        loadUIConfig()
    }

    private func loadSecurityConfig() {
        let defaults = UserDefaults.standard

        securityConfig.biometricAuthEnabled = defaults.bool(forKey: "biometric_auth_enabled")
        securityConfig.autoLockEnabled = defaults.bool(forKey: "auto_lock_enabled")
        if defaults.object(forKey: "auto_lock_enabled") == nil {
            securityConfig.autoLockEnabled = true
        }

        securityConfig.autoLockTimeout = defaults.double(forKey: "auto_lock_timeout")
        if securityConfig.autoLockTimeout == 0 {
            securityConfig.autoLockTimeout = 300
        }

        securityConfig.requireAuthForTransactions = defaults.bool(forKey: "require_auth_transactions")
        if defaults.object(forKey: "require_auth_transactions") == nil {
            securityConfig.requireAuthForTransactions = true
        }

        securityConfig.showBalanceOnLaunch = defaults.bool(forKey: "show_balance_launch")
        securityConfig.allowScreenshots = defaults.bool(forKey: "allow_screenshots")
    }

    private func loadUIConfig() {
        let defaults = UserDefaults.standard

        if let themeString = defaults.string(forKey: "ui_theme"),
           let theme = UIConfig.Theme(rawValue: themeString) {
            uiConfig.theme = theme
        }

        if let color = defaults.string(forKey: "ui_primary_color") {
            uiConfig.primaryColor = color
        }

        if let fontString = defaults.string(forKey: "ui_font_size"),
           let fontSize = UIConfig.FontSize(rawValue: fontString) {
            uiConfig.fontSize = fontSize
        }

        uiConfig.showTestnetWarning = defaults.bool(forKey: "show_testnet_warning")
        if defaults.object(forKey: "show_testnet_warning") == nil {
            uiConfig.showTestnetWarning = true
        }
    }

    func saveConfiguration() {
        let defaults = UserDefaults.standard

        defaults.set(environment.rawValue, forKey: "app_environment")
        defaults.set(timeoutSeconds, forKey: "timeout_seconds")
        defaults.set(maxRetries, forKey: "max_retries")
        defaults.set(enableLogging, forKey: "enable_logging")
        defaults.set(enableHaptics, forKey: "enable_haptics")
        defaults.set(enableNotifications, forKey: "enable_notifications")

        saveFeatureFlags()
        saveSecurityConfig()
        saveUIConfig()
    }

    private func saveFeatureFlags() {
        if let data = try? JSONEncoder().encode(featureFlags) {
            UserDefaults.standard.set(data, forKey: "feature_flags")
        }
    }

    private func saveSecurityConfig() {
        let defaults = UserDefaults.standard

        defaults.set(securityConfig.biometricAuthEnabled, forKey: "biometric_auth_enabled")
        defaults.set(securityConfig.autoLockEnabled, forKey: "auto_lock_enabled")
        defaults.set(securityConfig.autoLockTimeout, forKey: "auto_lock_timeout")
        defaults.set(securityConfig.requireAuthForTransactions, forKey: "require_auth_transactions")
        defaults.set(securityConfig.showBalanceOnLaunch, forKey: "show_balance_launch")
        defaults.set(securityConfig.allowScreenshots, forKey: "allow_screenshots")
    }

    private func saveUIConfig() {
        let defaults = UserDefaults.standard

        defaults.set(uiConfig.theme.rawValue, forKey: "ui_theme")
        defaults.set(uiConfig.primaryColor, forKey: "ui_primary_color")
        defaults.set(uiConfig.fontSize.rawValue, forKey: "ui_font_size")
        defaults.set(uiConfig.showTestnetWarning, forKey: "show_testnet_warning")
    }

    // MARK: - Reset

    func resetToDefaults() {
        environment = .production
        featureFlags.removeAll()
        setupDefaultFeatureFlags()

        timeoutSeconds = 30.0
        maxRetries = 3
        enableLogging = true
        logLevel = .info
        enableHaptics = true
        enableNotifications = true

        securityConfig = SecurityConfig()
        uiConfig = UIConfig()

        saveConfiguration()

        logInfo("Configuration reset to defaults", category: .general)
    }

    // MARK: - Debug

    func printConfiguration() {
        print("=== App Configuration ===")
        print("Environment: \(environment.rawValue)")
        print("Version: \(fullVersion)")
        print("Chain ID: \(environment.chainId)")
        print("RPC Endpoint: \(environment.rpcEndpoint)")
        print("\nFeature Flags:")
        for (key, value) in featureFlags.sorted(by: { $0.key < $1.key }) {
            print("  \(key): \(value)")
        }
        print("\nSettings:")
        print("  Timeout: \(timeoutSeconds)s")
        print("  Max Retries: \(maxRetries)")
        print("  Logging: \(enableLogging)")
        print("  Haptics: \(enableHaptics)")
        print("========================")
    }
}
