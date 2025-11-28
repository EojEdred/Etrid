//
//  NotificationService.swift
//  EtridWallet
//
//  Push notifications and local notification management
//

import Foundation
import UserNotifications
import UIKit

@MainActor
class NotificationService: NSObject, ObservableObject {
    // MARK: - Published Properties
    @Published var settings = NotificationSettings.defaultSettings
    @Published var isAuthorized = false
    @Published var notifications: [NotificationModel] = []
    @Published var unreadCount = 0
    @Published var deviceToken: String?

    // MARK: - Private Properties
    private let notificationCenter = UNUserNotificationCenter.current()
    private var priceAlerts: [PriceAlert] = []
    private var votingReminders: [DAOVotingReminder] = []

    // MARK: - Singleton
    static let shared = NotificationService()

    private override init() {
        super.init()
        notificationCenter.delegate = self
        loadSettings()
        loadNotifications()
        setupNotificationCategories()
    }

    // MARK: - Authorization
    /// Requests notification permissions
    func requestAuthorization() async throws {
        let options: UNAuthorizationOptions = [.alert, .badge, .sound, .providesAppNotificationSettings]

        do {
            let granted = try await notificationCenter.requestAuthorization(options: options)
            isAuthorized = granted

            if granted {
                await registerForRemoteNotifications()
            }
        } catch {
            throw WalletError.unknown("Failed to request authorization: \(error.localizedDescription)")
        }
    }

    /// Checks current authorization status
    func checkAuthorizationStatus() async {
        let settings = await notificationCenter.notificationSettings()
        isAuthorized = settings.authorizationStatus == .authorized
    }

    /// Registers for remote notifications (APNS)
    private func registerForRemoteNotifications() async {
        await UIApplication.shared.registerForRemoteNotifications()
    }

    /// Handles device token registration
    func didRegisterForRemoteNotifications(deviceToken: Data) {
        let tokenParts = deviceToken.map { data in String(format: "%02.2hhx", data) }
        let token = tokenParts.joined()
        self.deviceToken = token

        // Send token to backend
        Task {
            await sendTokenToBackend(token)
        }
    }

    /// Handles registration failure
    func didFailToRegisterForRemoteNotifications(error: Error) {
        print("Failed to register for remote notifications: \(error.localizedDescription)")
    }

    // MARK: - Notification Categories Setup
    private func setupNotificationCategories() {
        var categories = Set<UNNotificationCategory>()

        // Transaction category
        let viewAction = UNNotificationAction(
            identifier: NotificationAction.view.rawValue,
            title: NotificationAction.view.title,
            options: .foreground
        )

        let transactionCategory = UNNotificationCategory(
            identifier: NotificationCategory.transaction.rawValue,
            actions: [viewAction],
            intentIdentifiers: [],
            options: []
        )
        categories.insert(transactionCategory)

        // Payment request category
        let approveAction = UNNotificationAction(
            identifier: NotificationAction.approve.rawValue,
            title: NotificationAction.approve.title,
            options: [.foreground, .authenticationRequired]
        )
        let declineAction = UNNotificationAction(
            identifier: NotificationAction.decline.rawValue,
            title: NotificationAction.decline.title,
            options: [.destructive]
        )

        let paymentRequestCategory = UNNotificationCategory(
            identifier: NotificationCategory.paymentRequest.rawValue,
            actions: [approveAction, declineAction],
            intentIdentifiers: [],
            options: []
        )
        categories.insert(paymentRequestCategory)

        // DAO voting category
        let voteAction = UNNotificationAction(
            identifier: NotificationAction.vote.rawValue,
            title: NotificationAction.vote.title,
            options: [.foreground, .authenticationRequired]
        )

        let daoVotingCategory = UNNotificationCategory(
            identifier: NotificationCategory.daoVoting.rawValue,
            actions: [voteAction, viewAction],
            intentIdentifiers: [],
            options: []
        )
        categories.insert(daoVotingCategory)

        // Price alert category
        let priceAlertCategory = UNNotificationCategory(
            identifier: NotificationCategory.priceAlert.rawValue,
            actions: [viewAction],
            intentIdentifiers: [],
            options: []
        )
        categories.insert(priceAlertCategory)

        // Security category
        let securityCategory = UNNotificationCategory(
            identifier: NotificationCategory.security.rawValue,
            actions: [viewAction],
            intentIdentifiers: [],
            options: [.customDismissAction]
        )
        categories.insert(securityCategory)

        notificationCenter.setNotificationCategories(categories)
    }

    // MARK: - Local Notifications
    /// Schedules a local notification
    func scheduleLocalNotification(
        title: String,
        body: String,
        category: NotificationCategory,
        timeInterval: TimeInterval,
        userInfo: [String: Any] = [:]
    ) async throws {
        let content = UNMutableNotificationContent()
        content.title = title
        content.body = body
        content.categoryIdentifier = category.rawValue
        content.userInfo = userInfo

        if settings.soundEnabled {
            content.sound = .default
        }

        if settings.badgeEnabled {
            content.badge = NSNumber(value: unreadCount + 1)
        }

        let trigger = UNTimeIntervalNotificationTrigger(timeInterval: timeInterval, repeats: false)
        let request = UNNotificationRequest(
            identifier: UUID().uuidString,
            content: content,
            trigger: trigger
        )

        try await notificationCenter.add(request)
    }

    /// Schedules transaction notification
    func notifyTransaction(hash: String, amount: String, type: String, status: String) async {
        guard settings.transactionNotifications else { return }

        let title = type == "sent" ? "Transaction Sent" : "Transaction Received"
        let body = "\(amount) - Status: \(status)"

        try? await scheduleLocalNotification(
            title: title,
            body: body,
            category: .transaction,
            timeInterval: 1,
            userInfo: ["txHash": hash, "type": type]
        )

        // Add to notifications list
        let notification = NotificationModel(
            category: .transaction,
            title: title,
            body: body,
            data: ["txHash": hash, "type": type]
        )
        addNotification(notification)
    }

    /// Schedules price alert notification
    func notifyPriceAlert(tokenSymbol: String, currentPrice: Double, targetPrice: Double, condition: String) async {
        guard settings.priceAlerts else { return }

        let title = "Price Alert: \(tokenSymbol)"
        let body = "Current price: $\(String(format: "%.2f", currentPrice)) is \(condition) your target of $\(String(format: "%.2f", targetPrice))"

        try? await scheduleLocalNotification(
            title: title,
            body: body,
            category: .priceAlert,
            timeInterval: 1,
            userInfo: ["token": tokenSymbol, "price": currentPrice]
        )

        let notification = NotificationModel(
            category: .priceAlert,
            title: title,
            body: body,
            data: ["token": tokenSymbol, "price": String(currentPrice)]
        )
        addNotification(notification)
    }

    /// Schedules DAO voting reminder
    func notifyDAOVoting(proposalId: String, title: String, endDate: Date) async {
        guard settings.daoVotingReminders else { return }

        let notificationTitle = "DAO Proposal Ending Soon"
        let body = "\(title) ends \(endDate.formatted(.relative(presentation: .named)))"

        try? await scheduleLocalNotification(
            title: notificationTitle,
            body: body,
            category: .daoVoting,
            timeInterval: 1,
            userInfo: ["proposalId": proposalId]
        )

        let notification = NotificationModel(
            category: .daoVoting,
            title: notificationTitle,
            body: body,
            data: ["proposalId": proposalId]
        )
        addNotification(notification)
    }

    /// Schedules payment request notification
    func notifyPaymentRequest(from: String, amount: String, currency: String) async {
        guard settings.paymentRequests else { return }

        let title = "Payment Request"
        let body = "\(from) is requesting \(amount) \(currency)"

        try? await scheduleLocalNotification(
            title: title,
            body: body,
            category: .paymentRequest,
            timeInterval: 1,
            userInfo: ["from": from, "amount": amount, "currency": currency]
        )

        let notification = NotificationModel(
            category: .paymentRequest,
            title: title,
            body: body,
            data: ["from": from, "amount": amount, "currency": currency]
        )
        addNotification(notification)
    }

    /// Schedules security alert
    func notifySecurityAlert(title: String, message: String) async {
        guard settings.securityAlerts else { return }

        try? await scheduleLocalNotification(
            title: title,
            body: message,
            category: .security,
            timeInterval: 1
        )

        let notification = NotificationModel(
            category: .security,
            title: title,
            body: message
        )
        addNotification(notification)
    }

    // MARK: - Notification Management
    private func addNotification(_ notification: NotificationModel) {
        notifications.insert(notification, at: 0)
        if !notification.isRead {
            unreadCount += 1
        }
        saveNotifications()
    }

    /// Marks notification as read
    func markAsRead(_ notificationId: String) {
        if let index = notifications.firstIndex(where: { $0.id == notificationId }) {
            if !notifications[index].isRead {
                notifications[index].isRead = true
                unreadCount = max(0, unreadCount - 1)
                saveNotifications()
            }
        }
    }

    /// Marks all notifications as read
    func markAllAsRead() {
        notifications = notifications.map { var n = $0; n.isRead = true; return n }
        unreadCount = 0
        saveNotifications()
    }

    /// Clears all notifications
    func clearAll() {
        notifications.removeAll()
        unreadCount = 0
        saveNotifications()
        notificationCenter.removeAllDeliveredNotifications()
    }

    /// Removes pending notifications
    func removePendingNotifications() {
        notificationCenter.removeAllPendingNotificationRequests()
    }

    // MARK: - Settings Management
    func updateSettings(_ newSettings: NotificationSettings) {
        settings = newSettings
        saveSettings()
    }

    func toggleCategory(_ category: NotificationCategory, enabled: Bool) {
        switch category {
        case .transaction:
            settings.transactionNotifications = enabled
        case .priceAlert:
            settings.priceAlerts = enabled
        case .daoVoting:
            settings.daoVotingReminders = enabled
        case .paymentRequest:
            settings.paymentRequests = enabled
        case .security:
            settings.securityAlerts = enabled
        case .marketing:
            settings.marketingNotifications = enabled
        }
        saveSettings()
    }

    // MARK: - Backend Communication
    private func sendTokenToBackend(_ token: String) async {
        // TODO: Implement backend API call to register device token
        print("Device token to send to backend: \(token)")
    }

    // MARK: - Persistence
    private func saveSettings() {
        if let encoded = try? JSONEncoder().encode(settings) {
            UserDefaults.standard.set(encoded, forKey: "notificationSettings")
        }
    }

    private func loadSettings() {
        if let data = UserDefaults.standard.data(forKey: "notificationSettings"),
           let decoded = try? JSONDecoder().decode(NotificationSettings.self, from: data) {
            settings = decoded
        }
    }

    private func saveNotifications() {
        // Save notification history (limit to last 100)
        let recentNotifications = Array(notifications.prefix(100))
        if let encoded = try? JSONEncoder().encode(recentNotifications.map { notification in
            NotificationData(
                id: notification.id,
                category: notification.category.rawValue,
                title: notification.title,
                body: notification.body,
                timestamp: notification.timestamp,
                isRead: notification.isRead
            )
        }) {
            UserDefaults.standard.set(encoded, forKey: "notifications")
        }
    }

    private func loadNotifications() {
        if let data = UserDefaults.standard.data(forKey: "notifications"),
           let decoded = try? JSONDecoder().decode([NotificationData].self, from: data) {
            notifications = decoded.compactMap { data in
                guard let category = NotificationCategory(rawValue: data.category) else { return nil }
                return NotificationModel(
                    id: data.id,
                    category: category,
                    title: data.title,
                    body: data.body,
                    timestamp: data.timestamp,
                    isRead: data.isRead
                )
            }
            unreadCount = notifications.filter { !$0.isRead }.count
        }
    }
}

// MARK: - UNUserNotificationCenterDelegate
extension NotificationService: UNUserNotificationCenterDelegate {
    /// Handles notification when app is in foreground
    nonisolated func userNotificationCenter(
        _ center: UNUserNotificationCenter,
        willPresent notification: UNNotification,
        withCompletionHandler completionHandler: @escaping (UNNotificationPresentationOptions) -> Void
    ) {
        Task { @MainActor in
            let options: UNNotificationPresentationOptions = settings.showPreviewsWhenUnlocked
                ? [.banner, .badge, .sound]
                : [.badge]
            completionHandler(options)
        }
    }

    /// Handles notification tap
    nonisolated func userNotificationCenter(
        _ center: UNUserNotificationCenter,
        didReceive response: UNNotificationResponse,
        withCompletionHandler completionHandler: @escaping () -> Void
    ) {
        Task { @MainActor in
            let userInfo = response.notification.request.content.userInfo
            let categoryId = response.notification.request.content.categoryIdentifier

            // Handle different actions
            switch response.actionIdentifier {
            case NotificationAction.view.rawValue:
                handleViewAction(userInfo: userInfo, category: categoryId)
            case NotificationAction.approve.rawValue:
                handleApproveAction(userInfo: userInfo)
            case NotificationAction.decline.rawValue:
                handleDeclineAction(userInfo: userInfo)
            case NotificationAction.vote.rawValue:
                handleVoteAction(userInfo: userInfo)
            case UNNotificationDefaultActionIdentifier:
                handleViewAction(userInfo: userInfo, category: categoryId)
            default:
                break
            }

            completionHandler()
        }
    }

    private func handleViewAction(userInfo: [AnyHashable: Any], category: String) {
        // Handle navigation based on category and user info
        print("View action for category: \(category)")
    }

    private func handleApproveAction(userInfo: [AnyHashable: Any]) {
        // Handle payment request approval
        print("Approve action with userInfo: \(userInfo)")
    }

    private func handleDeclineAction(userInfo: [AnyHashable: Any]) {
        // Handle payment request decline
        print("Decline action with userInfo: \(userInfo)")
    }

    private func handleVoteAction(userInfo: [AnyHashable: Any]) {
        // Handle DAO voting action
        print("Vote action with userInfo: \(userInfo)")
    }
}

// MARK: - Codable Helper
private struct NotificationData: Codable {
    let id: String
    let category: String
    let title: String
    let body: String
    let timestamp: Date
    let isRead: Bool
}
