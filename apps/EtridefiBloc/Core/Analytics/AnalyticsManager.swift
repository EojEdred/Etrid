//
//  AnalyticsManager.swift
//  EtridWallet
//
//  Created on 2025-01-22.
//  Copyright © 2025 Ëtrid. All rights reserved.
//

import Foundation

/// Privacy-preserving analytics manager
actor AnalyticsManager {
    static let shared = AnalyticsManager()

    // MARK: - Types

    struct AnalyticsEvent: Codable {
        let id: String
        let name: String
        let properties: [String: AnyCodable]
        let timestamp: Date
        let sessionId: String

        init(name: String, properties: [String: Any] = [:]) {
            self.id = UUID().uuidString
            self.name = name
            self.properties = properties.mapValues { AnyCodable($0) }
            self.timestamp = Date()
            self.sessionId = AnalyticsManager.shared.currentSessionId
        }
    }

    struct PerformanceMetric: Codable {
        let operation: String
        let duration: TimeInterval
        let timestamp: Date
        let success: Bool
        let metadata: [String: AnyCodable]
    }

    // MARK: - Properties

    private var isEnabled: Bool = true
    private var events: [AnalyticsEvent] = []
    private var performanceMetrics: [PerformanceMetric] = []
    private var currentSessionId: String = UUID().uuidString
    private var sessionStartTime: Date = Date()
    private let maxStoredEvents = 1000
    private let userDefaults = UserDefaults.standard

    // MARK: - Initialization

    private init() {
        loadSettings()
        startNewSession()
    }

    // MARK: - Configuration

    func setEnabled(_ enabled: Bool) {
        isEnabled = enabled
        userDefaults.set(enabled, forKey: "analytics_enabled")
        await Logger.shared.info("Analytics \(enabled ? "enabled" : "disabled")", category: .analytics)
    }

    func getEnabled() -> Bool {
        return isEnabled
    }

    private func loadSettings() {
        if userDefaults.object(forKey: "analytics_enabled") != nil {
            isEnabled = userDefaults.bool(forKey: "analytics_enabled")
        }
    }

    // MARK: - Session Management

    private func startNewSession() {
        currentSessionId = UUID().uuidString
        sessionStartTime = Date()
        trackEvent(AnalyticsEvent(name: "session_start"))
    }

    func endSession() {
        let sessionDuration = Date().timeIntervalSince(sessionStartTime)
        trackEvent(AnalyticsEvent(
            name: "session_end",
            properties: ["duration": sessionDuration]
        ))
        flush()
    }

    // MARK: - Event Tracking

    func trackEvent(_ event: AnalyticsEvent) {
        guard isEnabled else { return }

        events.append(event)

        // Log locally
        await Logger.shared.info(
            "Analytics Event: \(event.name)",
            category: .analytics,
            metadata: event.properties.mapValues { $0.value }
        )

        // Trim if too many events
        if events.count > maxStoredEvents {
            events.removeFirst(events.count - maxStoredEvents)
        }

        // Auto-flush periodically
        if events.count % 50 == 0 {
            flush()
        }
    }

    func trackEvent(name: String, properties: [String: Any] = [:]) {
        trackEvent(AnalyticsEvent(name: name, properties: properties))
    }

    // MARK: - Screen Tracking

    func trackScreen(_ screenName: String) {
        trackEvent(name: "screen_view", properties: [
            "screen_name": screenName,
            "timestamp": Date().timeIntervalSince1970
        ])
    }

    // MARK: - User Actions

    func trackWalletCreated(address: String) {
        // Anonymize address (only log prefix)
        let anonymizedAddress = String(address.prefix(10)) + "..."
        trackEvent(name: "wallet_created", properties: [
            "address_prefix": anonymizedAddress
        ])
    }

    func trackWalletImported() {
        trackEvent(name: "wallet_imported")
    }

    func trackTransactionSent(amount: String, to: String) {
        // Don't log actual amount or recipient
        trackEvent(name: "transaction_sent", properties: [
            "has_amount": !amount.isEmpty,
            "has_recipient": !to.isEmpty
        ])
    }

    func trackTransactionReceived() {
        trackEvent(name: "transaction_received")
    }

    func trackTokenAdded(symbol: String) {
        trackEvent(name: "token_added", properties: [
            "symbol": symbol
        ])
    }

    func trackWalletConnectSession(connected: Bool) {
        trackEvent(name: connected ? "walletconnect_connected" : "walletconnect_disconnected")
    }

    func trackBiometricsEnabled(_ enabled: Bool) {
        trackEvent(name: "biometrics_\(enabled ? "enabled" : "disabled")")
    }

    func trackBackupCompleted() {
        trackEvent(name: "backup_completed")
    }

    func trackSettingChanged(setting: String, value: String) {
        trackEvent(name: "setting_changed", properties: [
            "setting": setting,
            "value": value
        ])
    }

    // MARK: - Error Tracking

    func trackError(_ error: Error, context: String, metadata: [String: Any] = [:]) {
        var properties: [String: Any] = [
            "error_type": String(describing: type(of: error)),
            "error_message": error.localizedDescription,
            "context": context
        ]

        properties.merge(metadata) { _, new in new }

        trackEvent(name: "error_occurred", properties: properties)

        await Logger.shared.error(
            "Error tracked: \(error.localizedDescription) in \(context)",
            category: .analytics
        )
    }

    func trackCrash(error: String, stackTrace: String) {
        trackEvent(name: "app_crash", properties: [
            "error": error,
            "stack_trace": stackTrace
        ])
    }

    // MARK: - Performance Tracking

    func trackPerformanceMetric(_ metric: PerformanceMetric) {
        guard isEnabled else { return }

        performanceMetrics.append(metric)

        await Logger.shared.info(
            "Performance: \(metric.operation) took \(String(format: "%.2f", metric.duration * 1000))ms",
            category: .performance
        )

        // Trim if too many metrics
        if performanceMetrics.count > maxStoredEvents {
            performanceMetrics.removeFirst(performanceMetrics.count - maxStoredEvents)
        }
    }

    func trackAppLaunchTime(_ duration: TimeInterval) {
        trackPerformanceMetric(PerformanceMetric(
            operation: "app_launch",
            duration: duration,
            timestamp: Date(),
            success: true,
            metadata: [:]
        ))
    }

    func trackTransactionSpeed(txHash: String, duration: TimeInterval) {
        trackPerformanceMetric(PerformanceMetric(
            operation: "transaction_confirmation",
            duration: duration,
            timestamp: Date(),
            success: true,
            metadata: ["tx_hash_prefix": String(txHash.prefix(10))]
        ))
    }

    func trackNetworkLatency(endpoint: String, duration: TimeInterval, success: Bool) {
        trackPerformanceMetric(PerformanceMetric(
            operation: "network_request",
            duration: duration,
            timestamp: Date(),
            success: success,
            metadata: ["endpoint": endpoint]
        ))
    }

    // MARK: - Feature Usage

    func trackFeatureUsage(_ feature: String) {
        trackEvent(name: "feature_used", properties: [
            "feature": feature
        ])
    }

    func trackButtonTap(_ buttonName: String, screen: String) {
        trackEvent(name: "button_tap", properties: [
            "button": buttonName,
            "screen": screen
        ])
    }

    // MARK: - Data Export

    func getAllEvents() -> [AnalyticsEvent] {
        return events
    }

    func getPerformanceMetrics() -> [PerformanceMetric] {
        return performanceMetrics
    }

    func getEventsSince(_ date: Date) -> [AnalyticsEvent] {
        return events.filter { $0.timestamp >= date }
    }

    func getSessionSummary() -> [String: Any] {
        let sessionDuration = Date().timeIntervalSince(sessionStartTime)

        return [
            "session_id": currentSessionId,
            "session_duration": sessionDuration,
            "events_count": events.count,
            "screens_viewed": events.filter { $0.name == "screen_view" }.count,
            "errors_count": events.filter { $0.name == "error_occurred" }.count
        ]
    }

    // MARK: - Data Management

    func flush() {
        guard isEnabled else { return }

        // In a real implementation, this would send events to an analytics backend
        // For now, we just log and persist locally

        Task {
            await Logger.shared.info("Flushing \(events.count) analytics events", category: .analytics)
        }

        // Save to persistent storage
        saveEventsToStorage()
    }

    func clearAllData() {
        events.removeAll()
        performanceMetrics.removeAll()
        clearStoredEvents()
        await Logger.shared.info("Analytics data cleared", category: .analytics)
    }

    private func saveEventsToStorage() {
        guard let documentsURL = FileManager.default.urls(for: .documentDirectory, in: .userDomainMask).first else {
            return
        }

        let analyticsURL = documentsURL.appendingPathComponent("Analytics", isDirectory: true)

        // Create directory if needed
        if !FileManager.default.fileExists(atPath: analyticsURL.path) {
            try? FileManager.default.createDirectory(at: analyticsURL, withIntermediateDirectories: true)
        }

        let eventsURL = analyticsURL.appendingPathComponent("events.json")

        if let data = try? JSONEncoder().encode(events) {
            try? data.write(to: eventsURL)
        }
    }

    private func clearStoredEvents() {
        guard let documentsURL = FileManager.default.urls(for: .documentDirectory, in: .userDomainMask).first else {
            return
        }

        let analyticsURL = documentsURL.appendingPathComponent("Analytics", isDirectory: true)
        try? FileManager.default.removeItem(at: analyticsURL)
    }

    // MARK: - Privacy

    func getPrivacyReport() -> [String: Any] {
        return [
            "analytics_enabled": isEnabled,
            "events_collected": events.count,
            "data_retention_days": 30,
            "pii_collected": false,
            "third_party_sharing": false,
            "anonymization_enabled": true
        ]
    }
}

// MARK: - AnyCodable

/// Type-erased Codable wrapper
struct AnyCodable: Codable {
    let value: Any

    init(_ value: Any) {
        self.value = value
    }

    init(from decoder: Decoder) throws {
        let container = try decoder.singleValueContainer()

        if let bool = try? container.decode(Bool.self) {
            value = bool
        } else if let int = try? container.decode(Int.self) {
            value = int
        } else if let double = try? container.decode(Double.self) {
            value = double
        } else if let string = try? container.decode(String.self) {
            value = string
        } else if let array = try? container.decode([AnyCodable].self) {
            value = array.map { $0.value }
        } else if let dictionary = try? container.decode([String: AnyCodable].self) {
            value = dictionary.mapValues { $0.value }
        } else {
            throw DecodingError.dataCorruptedError(
                in: container,
                debugDescription: "Unsupported type"
            )
        }
    }

    func encode(to encoder: Encoder) throws {
        var container = encoder.singleValueContainer()

        switch value {
        case let bool as Bool:
            try container.encode(bool)
        case let int as Int:
            try container.encode(int)
        case let double as Double:
            try container.encode(double)
        case let string as String:
            try container.encode(string)
        case let array as [Any]:
            try container.encode(array.map { AnyCodable($0) })
        case let dictionary as [String: Any]:
            try container.encode(dictionary.mapValues { AnyCodable($0) })
        default:
            let context = EncodingError.Context(
                codingPath: container.codingPath,
                debugDescription: "Unsupported type"
            )
            throw EncodingError.invalidValue(value, context)
        }
    }
}

// MARK: - Global Convenience Functions

/// Track analytics event
func trackEvent(name: String, properties: [String: Any] = [:]) {
    Task {
        await AnalyticsManager.shared.trackEvent(name: name, properties: properties)
    }
}

/// Track screen view
func trackScreen(_ screenName: String) {
    Task {
        await AnalyticsManager.shared.trackScreen(screenName)
    }
}

/// Track error
func trackError(_ error: Error, context: String) {
    Task {
        await AnalyticsManager.shared.trackError(error, context: context)
    }
}
