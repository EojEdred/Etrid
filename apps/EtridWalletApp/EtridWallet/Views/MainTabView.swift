//
//  MainTabView.swift
//  EtridWallet
//
//  Main tab bar navigation
//

import SwiftUI

struct MainTabView: View {
    @State private var selectedTab = 0

    var body: some View {
        TabView(selection: $selectedTab) {
            // Home Tab
            HomeView()
                .tabItem {
                    Label("Wallet", systemImage: "wallet.pass")
                }
                .tag(0)

            // NFT Tab
            NFTGalleryView()
                .tabItem {
                    Label("NFT", systemImage: "photo.stack")
                }
                .tag(1)

            // Trading Tab
            TradingView()
                .tabItem {
                    Label("Trade", systemImage: "chart.line.uptrend.xyaxis")
                }
                .tag(2)

            // Card Tab
            CardManagementView()
                .tabItem {
                    Label("Card", systemImage: "creditcard.fill")
                }
                .tag(3)

            // Governance Tab
            ConsensusDayView()
                .tabItem {
                    Label("Governance", systemImage: "building.columns")
                }
                .tag(4)

            // More Tab
            MoreView()
                .tabItem {
                    Label("More", systemImage: "ellipsis.circle")
                }
                .tag(5)
        }
        .accentColor(.etridBlue)
    }
}

// MARK: - More Tab
struct MoreView: View {
    var body: some View {
        NavigationStack {
            List {
                Section("DeFi") {
                    NavigationLink(destination: PlaceholderView(title: "Lending", description: "Lend and borrow crypto assets")) {
                        Label("Lending", systemImage: "dollarsign.circle")
                    }

                    NavigationLink(destination: PlaceholderView(title: "Savings Goals", description: "Set and track savings goals")) {
                        Label("Savings Goals", systemImage: "target")
                    }

                    NavigationLink(destination: PlaceholderView(title: "Fiat Ramp", description: "Buy crypto with fiat currency")) {
                        Label("Buy Crypto", systemImage: "banknote")
                    }
                }

                Section("Web3") {
                    NavigationLink(destination: PlaceholderView(title: "dApp Browser", description: "Browse decentralized applications")) {
                        Label("dApp Browser", systemImage: "safari")
                    }

                    NavigationLink(destination: PlaceholderView(title: "WalletConnect", description: "Connect to dApps")) {
                        Label("WalletConnect", systemImage: "link")
                    }

                    NavigationLink(destination: PlaceholderView(title: "DAO Management", description: "Participate in DAOs")) {
                        Label("DAOs", systemImage: "building.columns")
                    }
                }

                Section("Enterprise") {
                    NavigationLink(destination: PlaceholderView(title: "GPU Marketplace", description: "Rent or provide GPU compute")) {
                        Label("GPU Marketplace", systemImage: "cpu")
                    }

                    NavigationLink(destination: PlaceholderView(title: "Hyperledger Bridge", description: "Bridge to enterprise blockchains")) {
                        Label("Hyperledger", systemImage: "network")
                    }

                    NavigationLink(destination: PlaceholderView(title: "Multi-Signature", description: "Manage multi-sig wallets")) {
                        Label("Multi-Sig", systemImage: "signature")
                    }
                }

                Section("Settings") {
                    NavigationLink(destination: SettingsView()) {
                        Label("Settings", systemImage: "gearshape")
                    }
                }
            }
            .navigationTitle("More")
        }
    }
}

struct PlaceholderView: View {
    let title: String
    let description: String

    var body: some View {
        VStack(spacing: 20) {
            Image(systemName: "hammer.fill")
                .font(.system(size: 60))
                .foregroundColor(.etridBlue)

            Text(title)
                .font(.title)
                .fontWeight(.bold)

            Text(description)
                .font(.subheadline)
                .foregroundColor(.secondary)
                .multilineTextAlignment(.center)
                .padding(.horizontal)

            Text("Under Development")
                .font(.caption)
                .foregroundColor(.etridPurple)
                .padding(.horizontal, 16)
                .padding(.vertical, 8)
                .background(Color.etridPurple.opacity(0.1))
                .cornerRadius(8)
        }
        .padding()
        .navigationTitle(title)
        .navigationBarTitleDisplayMode(.inline)
    }
}

#Preview {
    MainTabView()
}
