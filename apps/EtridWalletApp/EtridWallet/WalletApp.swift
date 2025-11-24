//
//  WalletApp.swift
//  EtridWallet
//
//  Main app entry point with native iOS features
//

import SwiftUI
import UserNotifications

@main
struct EtridWalletApp: App {
    // MARK: - App Delegate
    @UIApplicationDelegateAdaptor(AppDelegate.self) var appDelegate

    // MARK: - State Objects
    @StateObject private var networkManager = NetworkManager.shared
    @StateObject private var securityManager = SecurityManager.shared
    @StateObject private var notificationService = NotificationService.shared
    @StateObject private var deepLinkService = DeepLinkService.shared
    @StateObject private var biometricService = BiometricService.shared
    @StateObject private var navigationCoordinator = DeepLinkNavigationCoordinator()

    // MARK: - Scene
    var body: some Scene {
        WindowGroup {
            MainTabView()
                .environmentObject(networkManager)
                .environmentObject(securityManager)
                .environmentObject(notificationService)
                .environmentObject(deepLinkService)
                .environmentObject(biometricService)
                .environmentObject(navigationCoordinator)
                .onOpenURL { url in
                    handleDeepLink(url)
                }
                .onContinueUserActivity(NSUserActivityTypeBrowsingWeb) { userActivity in
                    handleUniversalLink(userActivity)
                }
                .task {
                    await setupApp()
                }
        }
    }

    // MARK: - App Setup
    private func setupApp() async {
        // Request notification permissions
        if !notificationService.isAuthorized {
            try? await notificationService.requestAuthorization()
        }

        // Check authorization status
        await notificationService.checkAuthorizationStatus()
    }

    // MARK: - Deep Link Handling
    private func handleDeepLink(_ url: URL) {
        _ = deepLinkService.handleURL(url)
        HapticService.shared.tap()
    }

    private func handleUniversalLink(_ userActivity: NSUserActivity) {
        _ = deepLinkService.handleUniversalLink(userActivity)
        HapticService.shared.tap()
    }
}

// MARK: - App Delegate
class AppDelegate: NSObject, UIApplicationDelegate {
    func application(
        _ application: UIApplication,
        didFinishLaunchingWithOptions launchOptions: [UIApplication.LaunchOptionsKey: Any]? = nil
    ) -> Bool {
        // Configure notification categories
        configureNotifications()

        return true
    }

    // MARK: - Remote Notifications
    func application(
        _ application: UIApplication,
        didRegisterForRemoteNotificationsWithDeviceToken deviceToken: Data
    ) {
        Task { @MainActor in
            NotificationService.shared.didRegisterForRemoteNotifications(deviceToken: deviceToken)
        }
    }

    func application(
        _ application: UIApplication,
        didFailToRegisterForRemoteNotificationsWithError error: Error
    ) {
        Task { @MainActor in
            NotificationService.shared.didFailToRegisterForRemoteNotifications(error: error)
        }
    }

    // MARK: - Configuration
    private func configureNotifications() {
        UNUserNotificationCenter.current().delegate = NotificationService.shared
    }
}
