//
//  NotificationService.swift
//  EtridWallet
//
//  Created on 2025-01-22.
//  Copyright © 2025 Ëtrid. All rights reserved.
//

import Foundation
import UserNotifications
import UIKit

/// Local notification management for wallet events
@MainActor
class NotificationService: NSObject, ObservableObject {
    static let shared = NotificationService()

    // MARK: - Types

    enum NotificationCategory: String {
        case transaction = "TRANSACTION"
        case security = "SECURITY"
        case priceAlert = "PRICE_ALERT"
        case reminder = "REMINDER"
        case general = "GENERAL"
    }

    enum NotificationAction: String {
        case view = "VIEW"
        case dismiss = "DISMISS"
        case reply = "REPLY"
    }

    // MARK: - Properties

    @Published var isAuthorized = false
    @Published var authorizationStatus: UNAuthorizationStatus = .notDetermined
    @Published var notificationSettings: UNNotificationSettings?

    private let notificationCenter = UNUserNotificationCenter.current()

    // MARK: - Initialization

    private override init() {
        super.init()
        notificationCenter.delegate = self
        checkAuthorizationStatus()
    }

    // MARK: - Authorization

    func requestAuthorization() async throws {
        let options: UNAuthorizationOptions = [.alert, .badge, .sound, .providesAppNotificationSettings]

        do {
            let granted = try await notificationCenter.requestAuthorization(options: options)
            isAuthorized = granted

            if granted {
                registerNotificationCategories()
                logInfo("Notification authorization granted", category: .general)
            } else {
                logWarning("Notification authorization denied", category: .general)
            }

            await checkAuthorizationStatus()

        } catch {
            logError("Failed to request notification authorization: \(error.localizedDescription)", category: .general)
            throw error
        }
    }

    func checkAuthorizationStatus() async {
        let settings = await notificationCenter.notificationSettings()
        authorizationStatus = settings.authorizationStatus
        isAuthorized = settings.authorizationStatus == .authorized
        notificationSettings = settings

        logDebug("Notification authorization status: \(authorizationStatus.rawValue)", category: .general)
    }

    private func checkAuthorizationStatus() {
        Task {
            await checkAuthorizationStatus()
        }
    }

    // MARK: - Category Registration

    private func registerNotificationCategories() {
        let categories: Set<UNNotificationCategory> = [
            createTransactionCategory(),
            createSecurityCategory(),
            createPriceAlertCategory(),
            createReminderCategory(),
            createGeneralCategory()
        ]

        notificationCenter.setNotificationCategories(categories)
        logDebug("Notification categories registered", category: .general)
    }

    private func createTransactionCategory() -> UNNotificationCategory {
        let viewAction = UNNotificationAction(
            identifier: NotificationAction.view.rawValue,
            title: "View",
            options: .foreground
        )

        let dismissAction = UNNotificationAction(
            identifier: NotificationAction.dismiss.rawValue,
            title: "Dismiss",
            options: .destructive
        )

        return UNNotificationCategory(
            identifier: NotificationCategory.transaction.rawValue,
            actions: [viewAction, dismissAction],
            intentIdentifiers: [],
            options: .customDismissAction
        )
    }

    private func createSecurityCategory() -> UNNotificationCategory {
        let viewAction = UNNotificationAction(
            identifier: NotificationAction.view.rawValue,
            title: "View",
            options: [.foreground, .authenticationRequired]
        )

        return UNNotificationCategory(
            identifier: NotificationCategory.security.rawValue,
            actions: [viewAction],
            intentIdentifiers: [],
            options: []
        )
    }

    private func createPriceAlertCategory() -> UNNotificationCategory {
        let viewAction = UNNotificationAction(
            identifier: NotificationAction.view.rawValue,
            title: "View Chart",
            options: .foreground
        )

        let dismissAction = UNNotificationAction(
            identifier: NotificationAction.dismiss.rawValue,
            title: "Dismiss",
            options: .destructive
        )

        return UNNotificationCategory(
            identifier: NotificationCategory.priceAlert.rawValue,
            actions: [viewAction, dismissAction],
            intentIdentifiers: [],
            options: .customDismissAction
        )
    }

    private func createReminderCategory() -> UNNotificationCategory {
        let viewAction = UNNotificationAction(
            identifier: NotificationAction.view.rawValue,
            title: "Open",
            options: .foreground
        )

        return UNNotificationCategory(
            identifier: NotificationCategory.reminder.rawValue,
            actions: [viewAction],
            intentIdentifiers: [],
            options: []
        )
    }

    private func createGeneralCategory() -> UNNotificationCategory {
        let viewAction = UNNotificationAction(
            identifier: NotificationAction.view.rawValue,
            title: "View",
            options: .foreground
        )

        return UNNotificationCategory(
            identifier: NotificationCategory.general.rawValue,
            actions: [viewAction],
            intentIdentifiers: [],
            options: []
        )
    }

    // MARK: - Transaction Notifications

    func sendTransactionConfirmed(txHash: String, amount: String, token: String) async {
        let content = UNMutableNotificationContent()
        content.title = "Transaction Confirmed"
        content.body = "Your transaction of \(amount) \(token) has been confirmed"
        content.categoryIdentifier = NotificationCategory.transaction.rawValue
        content.sound = .default
        content.badge = 1

        content.userInfo = [
            "type": "transaction_confirmed",
            "txHash": txHash,
            "amount": amount,
            "token": token
        ]

        await sendNotification(identifier: "tx_confirmed_\(txHash)", content: content)

        logInfo("Transaction confirmed notification sent for \(txHash)", category: .transaction)
    }

    func sendTransactionReceived(amount: String, token: String, from: String) async {
        let content = UNMutableNotificationContent()
        content.title = "Tokens Received"
        content.body = "You received \(amount) \(token)"
        content.categoryIdentifier = NotificationCategory.transaction.rawValue
        content.sound = .default
        content.badge = 1

        content.userInfo = [
            "type": "transaction_received",
            "amount": amount,
            "token": token,
            "from": from
        ]

        await sendNotification(identifier: "tx_received_\(UUID().uuidString)", content: content)

        logInfo("Transaction received notification sent", category: .transaction)
    }

    func sendTransactionFailed(txHash: String, reason: String) async {
        let content = UNMutableNotificationContent()
        content.title = "Transaction Failed"
        content.body = reason
        content.categoryIdentifier = NotificationCategory.transaction.rawValue
        content.sound = .default

        content.userInfo = [
            "type": "transaction_failed",
            "txHash": txHash,
            "reason": reason
        ]

        await sendNotification(identifier: "tx_failed_\(txHash)", content: content)

        logWarning("Transaction failed notification sent for \(txHash)", category: .transaction)
    }

    // MARK: - Security Notifications

    func sendSecurityAlert(title: String, message: String) async {
        let content = UNMutableNotificationContent()
        content.title = title
        content.body = message
        content.categoryIdentifier = NotificationCategory.security.rawValue
        content.sound = .defaultCritical
        content.interruptionLevel = .critical

        content.userInfo = [
            "type": "security_alert",
            "message": message
        ]

        await sendNotification(identifier: "security_\(UUID().uuidString)", content: content)

        logWarning("Security alert sent: \(title)", category: .security)
    }

    func sendLoginDetected(deviceName: String) async {
        await sendSecurityAlert(
            title: "New Login Detected",
            message: "Your wallet was accessed from \(deviceName)"
        )
    }

    func sendSuspiciousActivity(description: String) async {
        await sendSecurityAlert(
            title: "Suspicious Activity Detected",
            message: description
        )
    }

    // MARK: - Price Alerts

    func sendPriceAlert(token: String, currentPrice: String, targetPrice: String, direction: String) async {
        let content = UNMutableNotificationContent()
        content.title = "\(token) Price Alert"
        content.body = "Price has \(direction) \(targetPrice) (current: \(currentPrice))"
        content.categoryIdentifier = NotificationCategory.priceAlert.rawValue
        content.sound = .default

        content.userInfo = [
            "type": "price_alert",
            "token": token,
            "current_price": currentPrice,
            "target_price": targetPrice,
            "direction": direction
        ]

        await sendNotification(identifier: "price_\(token)_\(UUID().uuidString)", content: content)

        logInfo("Price alert sent for \(token)", category: .general)
    }

    // MARK: - Reminders

    func scheduleBackupReminder(afterDays days: Int = 7) async {
        let content = UNMutableNotificationContent()
        content.title = "Backup Your Wallet"
        content.body = "It's been a while since you backed up your recovery phrase. Make sure it's stored safely."
        content.categoryIdentifier = NotificationCategory.reminder.rawValue
        content.sound = .default

        content.userInfo = [
            "type": "backup_reminder"
        ]

        let trigger = UNTimeIntervalNotificationTrigger(
            timeInterval: TimeInterval(days * 24 * 60 * 60),
            repeats: false
        )

        await sendNotification(
            identifier: "backup_reminder",
            content: content,
            trigger: trigger
        )

        logInfo("Backup reminder scheduled for \(days) days", category: .general)
    }

    func scheduleUpdateReminder(version: String) async {
        let content = UNMutableNotificationContent()
        content.title = "Update Available"
        content.body = "A new version (\(version)) of Ëtrid Wallet is available"
        content.categoryIdentifier = NotificationCategory.reminder.rawValue
        content.sound = .default

        content.userInfo = [
            "type": "update_reminder",
            "version": version
        ]

        await sendNotification(identifier: "update_reminder", content: content)

        logInfo("Update reminder sent for version \(version)", category: .general)
    }

    // MARK: - General Notifications

    func sendGeneralNotification(title: String, body: String, data: [String: Any] = [:]) async {
        let content = UNMutableNotificationContent()
        content.title = title
        content.body = body
        content.categoryIdentifier = NotificationCategory.general.rawValue
        content.sound = .default

        var userInfo = data
        userInfo["type"] = "general"
        content.userInfo = userInfo

        await sendNotification(identifier: "general_\(UUID().uuidString)", content: content)

        logInfo("General notification sent: \(title)", category: .general)
    }

    // MARK: - Core Notification Methods

    private func sendNotification(
        identifier: String,
        content: UNNotificationContent,
        trigger: UNNotificationTrigger? = nil
    ) async {
        guard isAuthorized else {
            logWarning("Cannot send notification - not authorized", category: .general)
            return
        }

        let request = UNNotificationRequest(
            identifier: identifier,
            content: content,
            trigger: trigger
        )

        do {
            try await notificationCenter.add(request)
            logDebug("Notification added: \(identifier)", category: .general)
        } catch {
            logError("Failed to add notification: \(error.localizedDescription)", category: .general)
        }
    }

    // MARK: - Badge Management

    func updateBadgeCount(_ count: Int) {
        UIApplication.shared.applicationIconBadgeNumber = count
        logDebug("Badge count updated to \(count)", category: .general)
    }

    func clearBadge() {
        updateBadgeCount(0)
    }

    func incrementBadge() {
        let current = UIApplication.shared.applicationIconBadgeNumber
        updateBadgeCount(current + 1)
    }

    // MARK: - Notification Management

    func getPendingNotifications() async -> [UNNotificationRequest] {
        return await notificationCenter.pendingNotificationRequests()
    }

    func getDeliveredNotifications() async -> [UNNotification] {
        return await notificationCenter.deliveredNotifications()
    }

    func removeNotification(identifier: String) {
        notificationCenter.removeDeliveredNotifications(withIdentifiers: [identifier])
        logDebug("Removed notification: \(identifier)", category: .general)
    }

    func removeAllNotifications() {
        notificationCenter.removeAllDeliveredNotifications()
        notificationCenter.removeAllPendingNotificationRequests()
        clearBadge()
        logInfo("All notifications removed", category: .general)
    }

    func removePendingNotifications(identifiers: [String]) {
        notificationCenter.removePendingNotificationRequests(withIdentifiers: identifiers)
        logDebug("Removed \(identifiers.count) pending notifications", category: .general)
    }

    // MARK: - Settings

    func openNotificationSettings() {
        if let url = URL(string: UIApplication.openSettingsURLString) {
            UIApplication.shared.open(url)
        }
    }

    // MARK: - Diagnostics

    func getNotificationStatus() async -> [String: Any] {
        let pending = await getPendingNotifications()
        let delivered = await getDeliveredNotifications()

        return [
            "authorized": isAuthorized,
            "authorization_status": authorizationStatus.rawValue,
            "pending_count": pending.count,
            "delivered_count": delivered.count,
            "badge_count": UIApplication.shared.applicationIconBadgeNumber
        ]
    }
}

// MARK: - UNUserNotificationCenterDelegate

extension NotificationService: UNUserNotificationCenterDelegate {
    nonisolated func userNotificationCenter(
        _ center: UNUserNotificationCenter,
        willPresent notification: UNNotification,
        withCompletionHandler completionHandler: @escaping (UNNotificationPresentationOptions) -> Void
    ) {
        // Show notification even when app is in foreground
        completionHandler([.banner, .sound, .badge])

        Task { @MainActor in
            logDebug("Will present notification: \(notification.request.identifier)", category: .general)
        }
    }

    nonisolated func userNotificationCenter(
        _ center: UNUserNotificationCenter,
        didReceive response: UNNotificationResponse,
        withCompletionHandler completionHandler: @escaping () -> Void
    ) {
        Task { @MainActor in
            await handleNotificationResponse(response)
            completionHandler()
        }
    }

    private func handleNotificationResponse(_ response: UNNotificationResponse) async {
        let userInfo = response.notification.request.content.userInfo
        let actionIdentifier = response.actionIdentifier

        logInfo("Notification action: \(actionIdentifier)", category: .general)

        // Handle different actions
        switch actionIdentifier {
        case NotificationAction.view.rawValue:
            await handleViewAction(userInfo: userInfo)

        case NotificationAction.dismiss.rawValue:
            logDebug("Notification dismissed", category: .general)

        case UNNotificationDefaultActionIdentifier:
            // User tapped the notification
            await handleViewAction(userInfo: userInfo)

        case UNNotificationDismissActionIdentifier:
            // User dismissed the notification
            logDebug("Notification dismissed", category: .general)

        default:
            break
        }
    }

    private func handleViewAction(userInfo: [AnyHashable: Any]) async {
        guard let type = userInfo["type"] as? String else { return }

        logInfo("Handling view action for type: \(type)", category: .general)

        // Navigate to appropriate screen based on notification type
        switch type {
        case "transaction_confirmed", "transaction_received", "transaction_failed":
            if let txHash = userInfo["txHash"] as? String {
                // Navigate to transaction detail
                logDebug("Navigate to transaction: \(txHash)", category: .transaction)
            }

        case "price_alert":
            if let token = userInfo["token"] as? String {
                // Navigate to token detail/chart
                logDebug("Navigate to token: \(token)", category: .general)
            }

        case "backup_reminder":
            // Navigate to backup screen
            logDebug("Navigate to backup screen", category: .general)

        case "security_alert":
            // Navigate to security settings
            logDebug("Navigate to security settings", category: .security)

        default:
            logDebug("Unknown notification type: \(type)", category: .general)
        }
    }
}

// MARK: - Global Convenience Functions

/// Send transaction confirmed notification
func notifyTransactionConfirmed(txHash: String, amount: String, token: String) {
    Task { @MainActor in
        await NotificationService.shared.sendTransactionConfirmed(
            txHash: txHash,
            amount: amount,
            token: token
        )
    }
}

/// Send security alert
func notifySecurityAlert(title: String, message: String) {
    Task { @MainActor in
        await NotificationService.shared.sendSecurityAlert(title: title, message: message)
    }
}
