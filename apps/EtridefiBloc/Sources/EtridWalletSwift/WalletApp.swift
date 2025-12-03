import SwiftUI

// Note: @main is in the App wrapper (EtridWalletAppMain.swift)

// MARK: - Root View (App State Router)

public struct RootView: View {
    @EnvironmentObject var walletManager: EtridWalletManager

    public init() {}

    public var body: some View {
        Group {
            switch walletManager.appState {
            case .loading:
                LoadingView()

            case .onboarding:
                OnboardingView()

            case .locked:
                LockScreenView()

            case .unlocked:
                MainTabView()
            }
        }
        .animation(.easeInOut, value: walletManager.appState)
    }
}

// MARK: - Loading View

public struct LoadingView: View {
    public init() {}

    public var body: some View {
        VStack(spacing: 24) {
            EtridLogo(size: 80)

            ProgressView()
                .scaleEffect(1.2)

            Text("Loading...")
                .font(.subheadline)
                .foregroundColor(.secondary)
        }
    }
}
