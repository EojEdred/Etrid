//
//  ContentView.swift
//  EtridWallet
//
//  Main app entry point - routes to Welcome or Home based on wallet state
//

import SwiftUI

struct ContentView: View {
    @StateObject private var securityManager = SecurityManager.shared
    @State private var isWalletInitialized = false
    @State private var isLoading = true
    @State private var refreshID = UUID()

    var body: some View {
        Group {
            if isLoading {
                LoadingView(message: "Loading wallet...")
            } else if !isWalletInitialized {
                WelcomeView()
                    .onReceive(NotificationCenter.default.publisher(for: .walletCreated)) { _ in
                        Task {
                            await refreshWalletState()
                        }
                    }
            } else if securityManager.isLocked {
                LockedView()
            } else {
                HomeView()
            }
        }
        .id(refreshID)
        .task {
            await checkWalletState()
        }
    }

    func checkWalletState() async {
        // Check if wallet is initialized
        isWalletInitialized = await WalletStorage.shared.isWalletInitialized()

        // Small delay for smooth transition
        try? await Task.sleep(nanoseconds: 500_000_000)

        isLoading = false
    }

    func refreshWalletState() async {
        isLoading = true
        await checkWalletState()
        refreshID = UUID()
    }
}

struct LockedView: View {
    @StateObject private var securityManager = SecurityManager.shared

    var body: some View {
        VStack(spacing: 30) {
            Spacer()

            Image(systemName: securityManager.biometricType.iconName)
                .font(.system(size: 80))
                .foregroundColor(.etridBlue)

            Text("Wallet Locked")
                .font(.title)
                .fontWeight(.bold)

            Text("Authenticate to unlock your wallet")
                .font(.subheadline)
                .foregroundColor(.secondary)
                .multilineTextAlignment(.center)

            Button(action: {
                Task {
                    try? await securityManager.unlock()
                }
            }) {
                Label("Unlock", systemImage: "lock.open.fill")
                    .font(.headline)
                    .foregroundColor(.white)
                    .frame(maxWidth: .infinity)
                    .padding()
                    .background(Color.etridBlue)
                    .cornerRadius(16)
            }
            .padding(.horizontal, 40)

            Spacer()
        }
        .padding()
    }
}

#Preview {
    ContentView()
}
