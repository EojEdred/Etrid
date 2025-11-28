//
//  MainTabView.swift
//  EtridWallet
//
//  Main tab navigation for the wallet app
//

import SwiftUI

struct MainTabView: View {
    @State private var selectedTab = 0

    var body: some View {
        TabView(selection: $selectedTab) {
            // Wallet Tab
            WalletView()
                .tabItem {
                    Label("Wallet", systemImage: "wallet.pass")
                }
                .tag(0)

            // Card Tab (Crypto-backed Card)
            CardManagementView()
                .tabItem {
                    Label("Card", systemImage: "creditcard")
                }
                .tag(1)

            // Trade Tab
            TradeView()
                .tabItem {
                    Label("Trade", systemImage: "chart.line.uptrend.xyaxis")
                }
                .tag(2)

            // DCA Bot Tab
            DCABotView()
                .tabItem {
                    Label("DCA", systemImage: "chart.bar.fill")
                }
                .tag(3)

            // Governance Tab
            DAODashboardView()
                .tabItem {
                    Label("Governance", systemImage: "person.3")
                }
                .tag(4)

            // Business Tab
            BusinessDashboardView()
                .tabItem {
                    Label("Business", systemImage: "briefcase")
                }
                .tag(5)

            // Multisig Tab
            MultisigWalletView()
                .tabItem {
                    Label("Multisig", systemImage: "lock.shield")
                }
                .tag(6)

            // Analytics Tab
            AnalyticsDashboardView()
                .tabItem {
                    Label("Analytics", systemImage: "chart.pie")
                }
                .tag(7)

            // DApp Browser Tab
            DAppBrowserView()
                .tabItem {
                    Label("Browser", systemImage: "globe")
                }
                .tag(8)
        }
        .accentColor(.blue)
    }
}

// MARK: - Wallet View (Placeholder)

struct WalletView: View {
    var body: some View {
        NavigationView {
            ZStack {
                LinearGradient(
                    gradient: Gradient(colors: [Color(hex: "0F172A"), Color(hex: "1E293B")]),
                    startPoint: .top,
                    endPoint: .bottom
                )
                .ignoresSafeArea()

                VStack {
                    Text("Wallet View")
                        .font(.largeTitle)
                        .foregroundColor(.white)

                    Text("Main wallet interface goes here")
                        .foregroundColor(.white.opacity(0.6))
                }
            }
            .navigationTitle("Wallet")
        }
    }
}

// MARK: - Trade View (Placeholder)

struct TradeView: View {
    var body: some View {
        NavigationView {
            ZStack {
                LinearGradient(
                    gradient: Gradient(colors: [Color(hex: "0F172A"), Color(hex: "1E293B")]),
                    startPoint: .top,
                    endPoint: .bottom
                )
                .ignoresSafeArea()

                VStack {
                    Text("Trade View")
                        .font(.largeTitle)
                        .foregroundColor(.white)

                    Text("Trading interface goes here")
                        .foregroundColor(.white.opacity(0.6))
                }
            }
            .navigationTitle("Trade")
        }
    }
}
