//
//  BackgroundTaskService.swift
//  EtridWallet
//
//  Created on 2025-01-22.
//  Copyright © 2025 Ëtrid. All rights reserved.
//

import Foundation
import BackgroundTasks
import UIKit

/// Background task management for wallet updates and monitoring
@MainActor
class BackgroundTaskService: ObservableObject {
    static let shared = BackgroundTaskService()

    // MARK: - Task Identifiers

    private enum TaskIdentifier {
        static let balanceRefresh = "com.etrid.wallet.balance-refresh"
        static let transactionMonitor = "com.etrid.wallet.transaction-monitor"
        static let priceUpdate = "com.etrid.wallet.price-update"
        static let notificationCheck = "com.etrid.wallet.notification-check"
    }

    // MARK: - Properties

    @Published var isBackgroundRefreshEnabled = false
    @Published var lastRefreshDate: Date?
    @Published var backgroundRefreshStatus: UIBackgroundRefreshStatus = .available

    private var balanceRefreshTask: BGAppRefreshTask?
    private var transactionMonitorTask: BGAppRefreshTask?

    // MARK: - Initialization

    private init() {
        checkBackgroundRefreshStatus()
    }

    // MARK: - Registration

    func registerBackgroundTasks() {
        // Register all background task identifiers
        BGTaskScheduler.shared.register(
            forTaskWithIdentifier: TaskIdentifier.balanceRefresh,
            using: nil
        ) { [weak self] task in
            guard let self = self else { return }
            Task { await self.handleBalanceRefresh(task as! BGAppRefreshTask) }
        }

        BGTaskScheduler.shared.register(
            forTaskWithIdentifier: TaskIdentifier.transactionMonitor,
            using: nil
        ) { [weak self] task in
            guard let self = self else { return }
            Task { await self.handleTransactionMonitor(task as! BGAppRefreshTask) }
        }

        BGTaskScheduler.shared.register(
            forTaskWithIdentifier: TaskIdentifier.priceUpdate,
            using: nil
        ) { [weak self] task in
            guard let self = self else { return }
            Task { await self.handlePriceUpdate(task as! BGAppRefreshTask) }
        }

        BGTaskScheduler.shared.register(
            forTaskWithIdentifier: TaskIdentifier.notificationCheck,
            using: nil
        ) { [weak self] task in
            guard let self = self else { return }
            Task { await self.handleNotificationCheck(task as! BGAppRefreshTask) }
        }

        logInfo("Background tasks registered", category: .general)
    }

    // MARK: - Scheduling

    func scheduleBackgroundRefresh() {
        scheduleBalanceRefresh()
        scheduleTransactionMonitor()
        schedulePriceUpdate()
        scheduleNotificationCheck()

        logInfo("Background refresh tasks scheduled", category: .general)
    }

    private func scheduleBalanceRefresh() {
        let request = BGAppRefreshTaskRequest(identifier: TaskIdentifier.balanceRefresh)
        request.earliestBeginDate = Date(timeIntervalSinceNow: 15 * 60) // 15 minutes

        do {
            try BGTaskScheduler.shared.submit(request)
            logDebug("Balance refresh scheduled", category: .general)
        } catch {
            logError("Failed to schedule balance refresh: \(error.localizedDescription)", category: .general)
        }
    }

    private func scheduleTransactionMonitor() {
        let request = BGAppRefreshTaskRequest(identifier: TaskIdentifier.transactionMonitor)
        request.earliestBeginDate = Date(timeIntervalSinceNow: 10 * 60) // 10 minutes

        do {
            try BGTaskScheduler.shared.submit(request)
            logDebug("Transaction monitor scheduled", category: .transaction)
        } catch {
            logError("Failed to schedule transaction monitor: \(error.localizedDescription)", category: .transaction)
        }
    }

    private func schedulePriceUpdate() {
        let request = BGAppRefreshTaskRequest(identifier: TaskIdentifier.priceUpdate)
        request.earliestBeginDate = Date(timeIntervalSinceNow: 30 * 60) // 30 minutes

        do {
            try BGTaskScheduler.shared.submit(request)
            logDebug("Price update scheduled", category: .general)
        } catch {
            logError("Failed to schedule price update: \(error.localizedDescription)", category: .general)
        }
    }

    private func scheduleNotificationCheck() {
        let request = BGAppRefreshTaskRequest(identifier: TaskIdentifier.notificationCheck)
        request.earliestBeginDate = Date(timeIntervalSinceNow: 20 * 60) // 20 minutes

        do {
            try BGTaskScheduler.shared.submit(request)
            logDebug("Notification check scheduled", category: .general)
        } catch {
            logError("Failed to schedule notification check: \(error.localizedDescription)", category: .general)
        }
    }

    // MARK: - Task Handlers

    private func handleBalanceRefresh(_ task: BGAppRefreshTask) async {
        logInfo("Background balance refresh started", category: .general)

        // Schedule next refresh
        scheduleBalanceRefresh()

        // Create task to prevent early termination
        let taskWork = Task {
            do {
                // Refresh balance for all wallets
                // This would integrate with WalletManager
                try await refreshAllBalances()

                await MainActor.run {
                    self.lastRefreshDate = Date()
                }

                logInfo("Background balance refresh completed", category: .general)
                task.setTaskCompleted(success: true)

            } catch {
                logError("Background balance refresh failed: \(error.localizedDescription)", category: .general)
                task.setTaskCompleted(success: false)
            }
        }

        // Handle expiration
        task.expirationHandler = {
            logWarning("Background balance refresh expired", category: .general)
            taskWork.cancel()
        }

        await taskWork.value
    }

    private func handleTransactionMonitor(_ task: BGAppRefreshTask) async {
        logInfo("Background transaction monitor started", category: .transaction)

        // Schedule next check
        scheduleTransactionMonitor()

        let taskWork = Task {
            do {
                // Check for pending transactions
                try await checkPendingTransactions()

                logInfo("Background transaction monitor completed", category: .transaction)
                task.setTaskCompleted(success: true)

            } catch {
                logError("Background transaction monitor failed: \(error.localizedDescription)", category: .transaction)
                task.setTaskCompleted(success: false)
            }
        }

        task.expirationHandler = {
            logWarning("Background transaction monitor expired", category: .transaction)
            taskWork.cancel()
        }

        await taskWork.value
    }

    private func handlePriceUpdate(_ task: BGAppRefreshTask) async {
        logInfo("Background price update started", category: .general)

        // Schedule next update
        schedulePriceUpdate()

        let taskWork = Task {
            do {
                // Update token prices
                try await updateTokenPrices()

                logInfo("Background price update completed", category: .general)
                task.setTaskCompleted(success: true)

            } catch {
                logError("Background price update failed: \(error.localizedDescription)", category: .general)
                task.setTaskCompleted(success: false)
            }
        }

        task.expirationHandler = {
            logWarning("Background price update expired", category: .general)
            taskWork.cancel()
        }

        await taskWork.value
    }

    private func handleNotificationCheck(_ task: BGAppRefreshTask) async {
        logInfo("Background notification check started", category: .general)

        // Schedule next check
        scheduleNotificationCheck()

        let taskWork = Task {
            do {
                // Check for notifications (new transactions, price alerts, etc.)
                try await checkNotifications()

                logInfo("Background notification check completed", category: .general)
                task.setTaskCompleted(success: true)

            } catch {
                logError("Background notification check failed: \(error.localizedDescription)", category: .general)
                task.setTaskCompleted(success: false)
            }
        }

        task.expirationHandler = {
            logWarning("Background notification check expired", category: .general)
            taskWork.cancel()
        }

        await taskWork.value
    }

    // MARK: - Background Work

    private func refreshAllBalances() async throws {
        logDebug("Refreshing all wallet balances in background", category: .general)

        // This would integrate with your WalletManager
        // For now, simulate work
        try await Task.sleep(nanoseconds: 2_000_000_000) // 2 seconds

        // Update balances
        // await WalletManager.shared.refreshBalances()

        logDebug("All balances refreshed", category: .general)
    }

    private func checkPendingTransactions() async throws {
        logDebug("Checking pending transactions in background", category: .transaction)

        // This would check blockchain for transaction status
        try await Task.sleep(nanoseconds: 1_500_000_000) // 1.5 seconds

        // Check transaction status
        // let pendingTxs = await TransactionManager.shared.getPendingTransactions()
        // for tx in pendingTxs {
        //     let status = try await checkTransactionStatus(tx.hash)
        //     if status.confirmed {
        //         await NotificationService.shared.sendTransactionConfirmed(tx)
        //     }
        // }

        logDebug("Pending transactions checked", category: .transaction)
    }

    private func updateTokenPrices() async throws {
        logDebug("Updating token prices in background", category: .general)

        // This would fetch latest prices from API
        try await Task.sleep(nanoseconds: 1_000_000_000) // 1 second

        // Update prices
        // await PriceService.shared.updatePrices()

        logDebug("Token prices updated", category: .general)
    }

    private func checkNotifications() async throws {
        logDebug("Checking for notifications in background", category: .general)

        // Check for events that require notifications
        try await Task.sleep(nanoseconds: 1_000_000_000) // 1 second

        // Send notifications if needed
        // await NotificationService.shared.checkAndSendNotifications()

        logDebug("Notifications checked", category: .general)
    }

    // MARK: - Status

    private func checkBackgroundRefreshStatus() {
        backgroundRefreshStatus = UIApplication.shared.backgroundRefreshStatus

        switch backgroundRefreshStatus {
        case .available:
            isBackgroundRefreshEnabled = true
            logInfo("Background refresh is available", category: .general)
        case .denied:
            isBackgroundRefreshEnabled = false
            logWarning("Background refresh is denied by user", category: .general)
        case .restricted:
            isBackgroundRefreshEnabled = false
            logWarning("Background refresh is restricted", category: .general)
        @unknown default:
            isBackgroundRefreshEnabled = false
            logWarning("Background refresh status unknown", category: .general)
        }
    }

    func refreshBackgroundStatus() {
        checkBackgroundRefreshStatus()
    }

    // MARK: - Manual Trigger (for testing)

    func triggerManualRefresh() async {
        logInfo("Manual background refresh triggered", category: .general)

        do {
            try await refreshAllBalances()
            try await checkPendingTransactions()
            try await updateTokenPrices()

            lastRefreshDate = Date()

            logInfo("Manual refresh completed successfully", category: .general)
        } catch {
            logError("Manual refresh failed: \(error.localizedDescription)", category: .general)
        }
    }

    // MARK: - Lifecycle

    func handleAppWillResignActive() {
        scheduleBackgroundRefresh()
        logDebug("Background tasks scheduled on app resign active", category: .general)
    }

    func handleAppDidBecomeActive() {
        refreshBackgroundStatus()
        logDebug("Background refresh status updated on app become active", category: .general)
    }

    // MARK: - Cancel

    func cancelAllBackgroundTasks() {
        BGTaskScheduler.shared.cancel(taskRequestWithIdentifier: TaskIdentifier.balanceRefresh)
        BGTaskScheduler.shared.cancel(taskRequestWithIdentifier: TaskIdentifier.transactionMonitor)
        BGTaskScheduler.shared.cancel(taskRequestWithIdentifier: TaskIdentifier.priceUpdate)
        BGTaskScheduler.shared.cancel(taskRequestWithIdentifier: TaskIdentifier.notificationCheck)

        logInfo("All background tasks cancelled", category: .general)
    }

    // MARK: - Diagnostics

    func getBackgroundTaskStatus() -> [String: Any] {
        return [
            "background_refresh_enabled": isBackgroundRefreshEnabled,
            "background_refresh_status": String(describing: backgroundRefreshStatus),
            "last_refresh_date": lastRefreshDate?.timeIntervalSince1970 ?? 0,
            "tasks_registered": 4
        ]
    }
}
