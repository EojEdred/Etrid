import SwiftUI
#if os(iOS)
import UIKit
#elseif os(macOS)
import AppKit
#endif

// MARK: - Main Tab View

struct MainTabView: View {
    @EnvironmentObject var walletManager: EtridWalletManager
    @State private var selectedTab: Tab = .home

    enum Tab: String, CaseIterable {
        case home = "Home"
        case tokens = "Tokens"
        case defiHub = "DeFi"
        case consensus = "Consënsus"
        case settings = "Settings"

        var icon: String {
            switch self {
            case .home: return "house.fill"
            case .tokens: return "bitcoinsign.circle.fill"
            case .defiHub: return "creditcard.and.123"
            case .consensus: return "building.columns.fill"
            case .settings: return "gearshape.fill"
            }
        }
    }

    var body: some View {
        TabView(selection: $selectedTab) {
            HomeView()
                .tabItem {
                    Label(Tab.home.rawValue, systemImage: Tab.home.icon)
                }
                .tag(Tab.home)

            TokensView()
                .tabItem {
                    Label(Tab.tokens.rawValue, systemImage: Tab.tokens.icon)
                }
                .tag(Tab.tokens)

            DeFiHubView()
                .tabItem {
                    Label(Tab.defiHub.rawValue, systemImage: Tab.defiHub.icon)
                }
                .tag(Tab.defiHub)

            GovernanceView()
                .tabItem {
                    Label(Tab.consensus.rawValue, systemImage: Tab.consensus.icon)
                }
                .tag(Tab.consensus)

            SettingsView()
                .tabItem {
                    Label(Tab.settings.rawValue, systemImage: Tab.settings.icon)
                }
                .tag(Tab.settings)
        }
        .tint(EtridColors.primary)
    }
}

// MARK: - Home View

struct HomeView: View {
    @EnvironmentObject var walletManager: EtridWalletManager
    @StateObject private var balanceService = BalanceService.shared
    @State private var showSend = false
    @State private var showReceive = false
    @State private var showBridge = false
    @State private var showSwap = false

    var body: some View {
        NavigationStack {
            ScrollView {
                VStack(spacing: 24) {
                    // Logo and Balance Card
                    BalanceCard()

                    // Quick Actions
                    QuickActionsBar(
                        onSend: { showSend = true },
                        onReceive: { showReceive = true },
                        onSwap: { showSwap = true },
                        onBridge: { showBridge = true }
                    )

                    // Token Holdings
                    TokenHoldingsSection()

                    // Recent Activity
                    RecentActivitySection()
                }
                .padding()
            }
            .background(EtridColors.background)
            .navigationTitle("Ëtridefi Bloc")
            #if os(iOS)
            .navigationBarTitleDisplayMode(.inline)
            #endif
            .toolbar {
                #if os(iOS)
                ToolbarItem(placement: .topBarLeading) {
                    EtridLogo(size: 32)
                }
                ToolbarItem(placement: .topBarTrailing) {
                    Button(action: { Task { await walletManager.refreshBalances() } }) {
                        Image(systemName: "arrow.clockwise")
                    }
                }
                #else
                ToolbarItem(placement: .navigation) {
                    EtridLogo(size: 32)
                }
                ToolbarItem(placement: .automatic) {
                    Button(action: { Task { await walletManager.refreshBalances() } }) {
                        Image(systemName: "arrow.clockwise")
                    }
                }
                #endif
            }
            .sheet(isPresented: $showSend) {
                SendView()
            }
            .sheet(isPresented: $showReceive) {
                ReceiveView()
            }
            .sheet(isPresented: $showBridge) {
                BridgeView()
            }
            .sheet(isPresented: $showSwap) {
                SwapView()
            }
        }
    }
}

// MARK: - Balance Card

struct BalanceCard: View {
    @EnvironmentObject var walletManager: EtridWalletManager

    var body: some View {
        VStack(spacing: 16) {
            // ETRID Logo
            EtridLogo(size: 60)

            // Total Balance Label
            Text("Total Balance")
                .font(.subheadline)
                .foregroundColor(.secondary)

            // Balance Amount
            Text(formattedBalance)
                .font(.system(size: 40, weight: .bold, design: .rounded))
                .foregroundColor(EtridColors.primary)

            // USD Value
            Text("$\(String(format: "%.2f", walletManager.totalBalance)) USD")
                .font(.headline)
                .foregroundColor(.secondary)

            // Selected Account Address
            if let account = walletManager.selectedAccount {
                HStack {
                    Text(truncatedAddress(account.address))
                        .font(.caption)
                        .foregroundColor(.secondary)

                    Button(action: copyAddress) {
                        Image(systemName: "doc.on.doc")
                            .font(.caption)
                    }
                }
                .padding(.horizontal, 12)
                .padding(.vertical, 6)
                .background(Color.secondary.opacity(0.1))
                .cornerRadius(8)
            }
        }
        .frame(maxWidth: .infinity)
        .padding(24)
        .background(
            LinearGradient(
                colors: [EtridColors.cardBackground, EtridColors.cardBackground.opacity(0.8)],
                startPoint: .topLeading,
                endPoint: .bottomTrailing
            )
        )
        .cornerRadius(20)
        .shadow(color: EtridColors.primary.opacity(0.1), radius: 10, y: 5)
    }

    private var formattedBalance: String {
        guard let account = walletManager.selectedAccount,
              let etridBalance = account.balances.first(where: { $0.token == .etrid }) else {
            return "0.00 ETR"
        }
        return "\(etridBalance.formattedAmount) ETR"
    }

    private func truncatedAddress(_ address: String) -> String {
        guard address.count > 12 else { return address }
        return "\(address.prefix(6))...\(address.suffix(4))"
    }

    private func copyAddress() {
        guard let address = walletManager.selectedAccount?.address else { return }
        #if os(iOS)
        UIPasteboard.general.string = address
        #elseif os(macOS)
        NSPasteboard.general.clearContents()
        NSPasteboard.general.setString(address, forType: .string)
        #endif
    }
}

// MARK: - Quick Actions Bar

struct QuickActionsBar: View {
    let onSend: () -> Void
    let onReceive: () -> Void
    let onSwap: () -> Void
    let onBridge: () -> Void

    var body: some View {
        HStack(spacing: 12) {
            QuickActionButton(title: "Send", icon: "arrow.up.circle.fill", color: .orange, action: onSend)
            QuickActionButton(title: "Receive", icon: "arrow.down.circle.fill", color: .green, action: onReceive)
            QuickActionButton(title: "Swap", icon: "arrow.triangle.2.circlepath.circle.fill", color: .blue, action: onSwap)
            QuickActionButton(title: "Bridge", icon: "arrow.left.arrow.right.circle.fill", color: EtridColors.primary, action: onBridge)
        }
    }
}

struct QuickActionButton: View {
    let title: String
    let icon: String
    let color: Color
    let action: () -> Void

    var body: some View {
        Button(action: action) {
            VStack(spacing: 8) {
                Image(systemName: icon)
                    .font(.system(size: 28))
                    .foregroundColor(color)

                Text(title)
                    .font(.caption)
                    .fontWeight(.medium)
                    .foregroundColor(.primary)
            }
            .frame(maxWidth: .infinity)
            .padding(.vertical, 16)
            .background(EtridColors.cardBackground)
            .cornerRadius(12)
        }
        .buttonStyle(.plain)
    }
}

// MARK: - Token Holdings Section

struct TokenHoldingsSection: View {
    @EnvironmentObject var walletManager: EtridWalletManager

    var body: some View {
        VStack(alignment: .leading, spacing: 12) {
            HStack {
                Text("Holdings")
                    .font(.headline)
                    .fontWeight(.semibold)

                Spacer()

                NavigationLink(destination: TokensView()) {
                    Text("See All")
                        .font(.caption)
                        .foregroundColor(EtridColors.primary)
                }
            }

            VStack(spacing: 8) {
                ForEach(tokenBalances.prefix(4)) { balance in
                    TokenRowView(balance: balance)
                }
            }
        }
        .padding(16)
        .background(EtridColors.cardBackground)
        .cornerRadius(16)
    }

    private var tokenBalances: [TokenBalance] {
        walletManager.selectedAccount?.balances ?? []
    }
}

struct TokenRowView: View {
    let balance: TokenBalance

    var body: some View {
        HStack {
            // Token Icon
            Circle()
                .fill(tokenColor)
                .frame(width: 40, height: 40)
                .overlay(
                    Text(balance.token.symbol.prefix(1))
                        .font(.headline)
                        .fontWeight(.bold)
                        .foregroundColor(.white)
                )

            VStack(alignment: .leading, spacing: 2) {
                Text(balance.token.name)
                    .font(.subheadline)
                    .fontWeight(.medium)

                Text(balance.token.symbol)
                    .font(.caption)
                    .foregroundColor(.secondary)
            }

            Spacer()

            VStack(alignment: .trailing, spacing: 2) {
                Text(balance.formattedAmount)
                    .font(.subheadline)
                    .fontWeight(.medium)

                Text(balance.formattedUSDValue)
                    .font(.caption)
                    .foregroundColor(.secondary)
            }
        }
        .padding(.vertical, 8)
    }

    private var tokenColor: Color {
        switch balance.token.symbol {
        case "ETR": return EtridColors.primary
        case "bSOL": return .purple
        case "bBNB": return .yellow
        case "bXRP": return .blue
        case "bTRX": return .red
        default: return .gray
        }
    }
}

// MARK: - Recent Activity Section

struct RecentActivitySection: View {
    @EnvironmentObject var walletManager: EtridWalletManager
    @State private var transactions: [Transaction] = []

    var body: some View {
        VStack(alignment: .leading, spacing: 12) {
            HStack {
                Text("Recent Activity")
                    .font(.headline)
                    .fontWeight(.semibold)

                Spacer()

                NavigationLink(destination: ActivityView()) {
                    Text("See All")
                        .font(.caption)
                        .foregroundColor(EtridColors.primary)
                }
            }

            if transactions.isEmpty {
                EmptyActivityView()
            } else {
                VStack(spacing: 8) {
                    ForEach(transactions.prefix(5)) { tx in
                        TransactionRowView(transaction: tx)
                    }
                }
            }
        }
        .padding(16)
        .background(EtridColors.cardBackground)
        .cornerRadius(16)
        .task {
            await loadTransactions()
        }
    }

    private func loadTransactions() async {
        guard let address = walletManager.selectedAccount?.address else { return }
        do {
            transactions = try await walletManager.getTransactionHistory(for: address, limit: 5)
        } catch {
            print("Failed to load transactions: \(error)")
        }
    }
}

struct EmptyActivityView: View {
    var body: some View {
        VStack(spacing: 12) {
            Image(systemName: "clock.arrow.circlepath")
                .font(.system(size: 40))
                .foregroundColor(.secondary)

            Text("No transactions yet")
                .font(.subheadline)
                .foregroundColor(.secondary)
        }
        .frame(maxWidth: .infinity)
        .padding(24)
    }
}

struct TransactionRowView: View {
    let transaction: Transaction

    var body: some View {
        HStack {
            // Icon
            Image(systemName: transaction.type.icon)
                .font(.title2)
                .foregroundColor(iconColor)
                .frame(width: 40)

            VStack(alignment: .leading, spacing: 2) {
                Text(transaction.type.rawValue)
                    .font(.subheadline)
                    .fontWeight(.medium)

                Text(formattedDate)
                    .font(.caption)
                    .foregroundColor(.secondary)
            }

            Spacer()

            VStack(alignment: .trailing, spacing: 2) {
                Text(amountText)
                    .font(.subheadline)
                    .fontWeight(.medium)
                    .foregroundColor(amountColor)

                Text(transaction.status.rawValue)
                    .font(.caption)
                    .foregroundColor(statusColor)
            }
        }
        .padding(.vertical, 8)
    }

    private var iconColor: Color {
        switch transaction.type {
        case .send: return .orange
        case .receive: return .green
        case .swap: return .blue
        case .bridgeDeposit, .bridgeWithdraw: return EtridColors.primary
        default: return .gray
        }
    }

    private var amountText: String {
        let prefix = transaction.type == .send ? "-" : "+"
        return "\(prefix)\((transaction.amount as NSDecimalNumber).doubleValue) \(transaction.token.symbol)"
    }

    private var amountColor: Color {
        transaction.type == .send ? .red : .green
    }

    private var statusColor: Color {
        switch transaction.status {
        case .confirmed: return .green
        case .pending: return .orange
        case .failed: return .red
        case .cancelled: return .gray
        }
    }

    private var formattedDate: String {
        let formatter = RelativeDateTimeFormatter()
        formatter.unitsStyle = .abbreviated
        return formatter.localizedString(for: transaction.timestamp, relativeTo: Date())
    }
}

// MARK: - Activity View

struct ActivityView: View {
    @EnvironmentObject var walletManager: EtridWalletManager
    @State private var transactions: [Transaction] = []
    @State private var isLoading = false

    var body: some View {
        List {
            if isLoading {
                ProgressView()
                    .frame(maxWidth: .infinity)
            } else if transactions.isEmpty {
                EmptyActivityView()
            } else {
                ForEach(transactions) { tx in
                    TransactionRowView(transaction: tx)
                }
            }
        }
        .navigationTitle("Activity")
        .task {
            await loadTransactions()
        }
        .refreshable {
            await loadTransactions()
        }
    }

    private func loadTransactions() async {
        guard let address = walletManager.selectedAccount?.address else { return }
        isLoading = true
        do {
            transactions = try await walletManager.getTransactionHistory(for: address, limit: 50)
        } catch {
            print("Failed to load transactions: \(error)")
        }
        isLoading = false
    }
}

// MARK: - ETRID Logo Component

struct EtridLogo: View {
    let size: CGFloat

    var body: some View {
        // Try to load the bundled logo image, fallback to text if not found
        #if os(iOS)
        if let uiImage = UIImage(named: "etrid_logo") {
            Image(uiImage: uiImage)
                .resizable()
                .aspectRatio(contentMode: .fit)
                .frame(width: size, height: size)
                .clipShape(Circle())
        } else {
            fallbackLogo
        }
        #else
        if let nsImage = NSImage(named: "etrid_logo") {
            Image(nsImage: nsImage)
                .resizable()
                .aspectRatio(contentMode: .fit)
                .frame(width: size, height: size)
                .clipShape(Circle())
        } else {
            fallbackLogo
        }
        #endif
    }

    private var fallbackLogo: some View {
        ZStack {
            Circle()
                .fill(
                    LinearGradient(
                        colors: [EtridColors.primary, EtridColors.primary.opacity(0.7)],
                        startPoint: .topLeading,
                        endPoint: .bottomTrailing
                    )
                )
                .frame(width: size, height: size)

            Text("Ë")
                .font(.system(size: size * 0.5, weight: .bold, design: .rounded))
                .foregroundColor(.white)
        }
    }
}

// MARK: - Color Theme

struct EtridColors {
    static let primary = Color(red: 0.4, green: 0.85, blue: 0.9) // Cyan/Turquoise from logo
    static let secondary = Color(red: 0.3, green: 0.7, blue: 0.8)
    #if os(iOS)
    static let background = Color(UIColor.systemBackground)
    static let cardBackground = Color(UIColor.secondarySystemBackground)
    #else
    static let background = Color(nsColor: NSColor.windowBackgroundColor)
    static let cardBackground = Color(nsColor: NSColor.controlBackgroundColor)
    #endif
    static let success = Color.green
    static let warning = Color.orange
    static let error = Color.red
}
