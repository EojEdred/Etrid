//
//  NotificationSettingsView.swift
//  EtridWallet
//
//  Notification settings and preferences
//

import SwiftUI

struct NotificationSettingsView: View {
    @StateObject private var notificationService = NotificationService.shared
    @State private var showPermissionAlert = false
    @State private var showClearConfirmation = false

    var body: some View {
        List {
            // Authorization Status
            Section {
                HStack {
                    Image(systemName: notificationService.isAuthorized ? "checkmark.circle.fill" : "xmark.circle.fill")
                        .foregroundColor(notificationService.isAuthorized ? .green : .red)

                    Text(notificationService.isAuthorized ? "Notifications Enabled" : "Notifications Disabled")

                    Spacer()

                    if !notificationService.isAuthorized {
                        Button("Enable") {
                            Task {
                                await requestPermissions()
                            }
                        }
                        .buttonStyle(.borderedProminent)
                        .controlSize(.small)
                    }
                }
            } header: {
                Text("Status")
            } footer: {
                if !notificationService.isAuthorized {
                    Text("Enable notifications to receive important updates about your wallet.")
                }
            }

            // Notification Categories
            if notificationService.isAuthorized {
                Section {
                    ForEach(NotificationCategory.allCases, id: \.self) { category in
                        NotificationCategoryRow(
                            category: category,
                            isEnabled: isEnabled(for: category)
                        ) { enabled in
                            notificationService.toggleCategory(category, enabled: enabled)
                        }
                    }
                } header: {
                    Text("Notification Types")
                } footer: {
                    Text("Choose which types of notifications you want to receive.")
                }

                // Notification Preferences
                Section {
                    Toggle("Sound", isOn: Binding(
                        get: { notificationService.settings.soundEnabled },
                        set: { newValue in
                            var settings = notificationService.settings
                            settings.soundEnabled = newValue
                            notificationService.updateSettings(settings)
                        }
                    ))

                    Toggle("Badge", isOn: Binding(
                        get: { notificationService.settings.badgeEnabled },
                        set: { newValue in
                            var settings = notificationService.settings
                            settings.badgeEnabled = newValue
                            notificationService.updateSettings(settings)
                        }
                    ))

                    Toggle("Show Previews", isOn: Binding(
                        get: { notificationService.settings.showPreviewsWhenUnlocked },
                        set: { newValue in
                            var settings = notificationService.settings
                            settings.showPreviewsWhenUnlocked = newValue
                            notificationService.updateSettings(settings)
                        }
                    ))
                } header: {
                    Text("Preferences")
                }

                // Price Alert Settings
                if notificationService.settings.priceAlerts {
                    Section {
                        Stepper(
                            "Alert Threshold: \(Int(notificationService.settings.priceAlertThresholdPercentage))%",
                            value: Binding(
                                get: { notificationService.settings.priceAlertThresholdPercentage },
                                set: { newValue in
                                    var settings = notificationService.settings
                                    settings.priceAlertThresholdPercentage = newValue
                                    notificationService.updateSettings(settings)
                                }
                            ),
                            in: 1...50,
                            step: 1
                        )

                        Picker("Cooldown Period", selection: Binding(
                            get: { notificationService.settings.priceAlertCooldownMinutes },
                            set: { newValue in
                                var settings = notificationService.settings
                                settings.priceAlertCooldownMinutes = newValue
                                notificationService.updateSettings(settings)
                            }
                        )) {
                            Text("15 minutes").tag(15)
                            Text("30 minutes").tag(30)
                            Text("1 hour").tag(60)
                            Text("2 hours").tag(120)
                            Text("6 hours").tag(360)
                            Text("24 hours").tag(1440)
                        }
                    } header: {
                        Text("Price Alert Settings")
                    } footer: {
                        Text("Get notified when prices change by more than \(Int(notificationService.settings.priceAlertThresholdPercentage))%.")
                    }
                }

                // Notification History
                Section {
                    HStack {
                        Text("Unread Notifications")
                        Spacer()
                        Text("\(notificationService.unreadCount)")
                            .foregroundColor(.secondary)
                    }

                    if notificationService.unreadCount > 0 {
                        Button("Mark All as Read") {
                            notificationService.markAllAsRead()
                            HapticService.shared.success()
                        }
                    }

                    if !notificationService.notifications.isEmpty {
                        Button("Clear All", role: .destructive) {
                            showClearConfirmation = true
                        }
                    }
                } header: {
                    Text("History")
                }

                // Test Notification
                Section {
                    Button {
                        Task {
                            await sendTestNotification()
                        }
                    } label: {
                        HStack {
                            Image(systemName: "bell.badge")
                            Text("Send Test Notification")
                        }
                    }
                } header: {
                    Text("Test")
                }
            }

            // Device Token (Debug)
            #if DEBUG
            if let token = notificationService.deviceToken {
                Section {
                    Text(token)
                        .font(.caption)
                        .textSelection(.enabled)
                } header: {
                    Text("Device Token")
                }
            }
            #endif
        }
        .navigationTitle("Notifications")
        .navigationBarTitleDisplayMode(.inline)
        .alert("Notifications Disabled", isPresented: $showPermissionAlert) {
            Button("Settings", role: .cancel) {
                openSettings()
            }
            Button("Cancel", role: .cancel) {}
        } message: {
            Text("Please enable notifications in Settings to receive important updates.")
        }
        .confirmationDialog("Clear All Notifications", isPresented: $showClearConfirmation) {
            Button("Clear All", role: .destructive) {
                notificationService.clearAll()
                HapticService.shared.success()
            }
            Button("Cancel", role: .cancel) {}
        } message: {
            Text("This will clear all notification history.")
        }
        .task {
            await notificationService.checkAuthorizationStatus()
        }
    }

    // MARK: - Helper Methods
    private func isEnabled(for category: NotificationCategory) -> Bool {
        switch category {
        case .transaction:
            return notificationService.settings.transactionNotifications
        case .priceAlert:
            return notificationService.settings.priceAlerts
        case .daoVoting:
            return notificationService.settings.daoVotingReminders
        case .paymentRequest:
            return notificationService.settings.paymentRequests
        case .security:
            return notificationService.settings.securityAlerts
        case .marketing:
            return notificationService.settings.marketingNotifications
        }
    }

    private func requestPermissions() async {
        do {
            try await notificationService.requestAuthorization()
            HapticService.shared.success()
        } catch {
            showPermissionAlert = true
            HapticService.shared.error()
        }
    }

    private func sendTestNotification() async {
        await notificationService.notifyTransaction(
            hash: "0x1234...5678",
            amount: "0.1 ETH",
            type: "received",
            status: "Confirmed"
        )
        HapticService.shared.success()
    }

    private func openSettings() {
        if let url = URL(string: UIApplication.openSettingsURLString) {
            UIApplication.shared.open(url)
        }
    }
}

// MARK: - Notification Category Row
struct NotificationCategoryRow: View {
    let category: NotificationCategory
    let isEnabled: Bool
    let onToggle: (Bool) -> Void

    var body: some View {
        HStack {
            Image(systemName: category.iconName)
                .foregroundColor(.blue)
                .frame(width: 30)

            VStack(alignment: .leading, spacing: 2) {
                Text(category.displayName)
                    .font(.body)

                Text(category.description)
                    .font(.caption)
                    .foregroundColor(.secondary)
            }

            Spacer()

            Toggle("", isOn: Binding(
                get: { isEnabled },
                set: { newValue in
                    onToggle(newValue)
                    HapticService.shared.tap()
                }
            ))
            .labelsHidden()
        }
    }
}

#Preview {
    NavigationStack {
        NotificationSettingsView()
    }
}
