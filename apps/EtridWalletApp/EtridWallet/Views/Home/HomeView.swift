//
//  HomeView.swift
//  EtridWallet
//
//  Main home/wallet screen
//

import SwiftUI

struct HomeView: View {
    @StateObject private var viewModel = HomeViewModel()
    @State private var showNetworkPicker = false
    @State private var showAccountPicker = false

    var body: some View {
        NavigationStack {
            ScrollView {
                VStack(spacing: 20) {
                    // Header - Account & Network
                    VStack(spacing: 12) {
                        // Account selector
                        Button(action: { showAccountPicker = true }) {
                            HStack {
                                Image(systemName: "person.circle.fill")
                                    .font(.title3)

                                VStack(alignment: .leading, spacing: 2) {
                                    Text(viewModel.currentAccount?.name ?? "No Account")
                                        .font(.headline)

                                    if let address = viewModel.currentAccount?.address {
                                        Text(address.formatAddress())
                                            .font(.caption)
                                            .foregroundColor(.secondary)
                                    }
                                }

                                Spacer()

                                Image(systemName: "chevron.down")
                                    .font(.caption)
                                    .foregroundColor(.secondary)
                            }
                            .padding()
                            .background(Color(.systemGray6))
                            .cornerRadius(12)
                        }
                        .foregroundColor(.primary)

                        // Network selector
                        Button(action: { showNetworkPicker = true }) {
                            HStack {
                                Circle()
                                    .fill(Color.etridBlue)
                                    .frame(width: 10, height: 10)

                                Text(viewModel.currentNetwork.name)
                                    .font(.subheadline)
                                    .fontWeight(.medium)

                                Spacer()

                                Image(systemName: "chevron.down")
                                    .font(.caption)
                                    .foregroundColor(.secondary)
                            }
                            .padding(.horizontal)
                            .padding(.vertical, 8)
                            .background(Color(.systemGray6))
                            .cornerRadius(8)
                        }
                        .foregroundColor(.primary)
                    }
                    .padding(.horizontal)

                    // Balance Card
                    VStack(spacing: 16) {
                        Text("Total Balance")
                            .font(.subheadline)
                            .foregroundColor(.secondary)

                        if viewModel.isLoadingBalance {
                            ProgressView()
                        } else {
                            Text(viewModel.formattedBalance)
                                .font(.system(size: 42, weight: .bold))

                            if let usdValue = viewModel.balanceUSD {
                                Text(usdValue)
                                    .font(.title3)
                                    .foregroundColor(.secondary)
                            }
                        }
                    }
                    .frame(maxWidth: .infinity)
                    .padding(.vertical, 30)
                    .background(
                        LinearGradient(
                            colors: [Color.etridBlue, Color.etridPurple],
                            startPoint: .topLeading,
                            endPoint: .bottomTrailing
                        )
                    )
                    .foregroundColor(.white)
                    .cornerRadius(20)
                    .padding(.horizontal)

                    // Action Buttons
                    HStack(spacing: 16) {
                        ActionButton(icon: "arrow.up", title: "Send", color: .etridBlue) {
                            viewModel.showSendView = true
                        }

                        ActionButton(icon: "arrow.down", title: "Receive", color: .etridGreen) {
                            viewModel.showReceiveView = true
                        }

                        ActionButton(icon: "clock.arrow.circlepath", title: "Activity", color: .etridOrange) {
                            viewModel.showActivityView = true
                        }
                    }
                    .padding(.horizontal)

                    // Tokens List
                    VStack(alignment: .leading, spacing: 12) {
                        HStack {
                            Text("Tokens")
                                .font(.headline)

                            Spacer()

                            Button(action: { viewModel.refreshBalances() }) {
                                Image(systemName: "arrow.clockwise")
                                    .font(.subheadline)
                            }
                        }

                        if viewModel.tokens.isEmpty {
                            VStack(spacing: 12) {
                                Image(systemName: "tray")
                                    .font(.largeTitle)
                                    .foregroundColor(.secondary)

                                Text("No tokens yet")
                                    .font(.subheadline)
                                    .foregroundColor(.secondary)
                            }
                            .frame(maxWidth: .infinity)
                            .padding(.vertical, 40)
                        } else {
                            ForEach(viewModel.tokens) { token in
                                TokenRow(token: token)
                            }
                        }
                    }
                    .padding()
                    .background(Color(.systemBackground))
                    .cornerRadius(16)
                    .padding(.horizontal)
                }
                .padding(.vertical)
            }
            .navigationTitle("Wallet")
            .navigationBarTitleDisplayMode(.inline)
            .toolbar {
                ToolbarItem(placement: .navigationBarTrailing) {
                    NavigationLink(destination: SettingsView()) {
                        Image(systemName: "gearshape.fill")
                    }
                }
            }
            .sheet(isPresented: $showNetworkPicker) {
                NetworkPickerView(selectedNetwork: $viewModel.currentNetwork)
            }
            .sheet(isPresented: $showAccountPicker) {
                AccountPickerView()
            }
            .sheet(isPresented: $viewModel.showSendView) {
                SendView()
            }
            .sheet(isPresented: $viewModel.showReceiveView) {
                ReceiveView(address: viewModel.currentAccount?.address ?? "", network: viewModel.currentNetwork)
            }
            .sheet(isPresented: $viewModel.showActivityView) {
                ActivityView()
            }
            .refreshable {
                await viewModel.loadData()
            }
        }
        .task {
            await viewModel.loadData()
        }
    }
}

private struct ActionButton: View {
    let icon: String
    let title: String
    let color: Color
    let action: () -> Void

    var body: some View {
        Button(action: action) {
            VStack(spacing: 8) {
                Circle()
                    .fill(color)
                    .frame(width: 50, height: 50)
                    .overlay(
                        Image(systemName: icon)
                            .foregroundColor(.white)
                    )

                Text(title)
                    .font(.caption)
                    .foregroundColor(.primary)
            }
            .frame(maxWidth: .infinity)
        }
    }
}

struct TokenRow: View {
    let token: Token

    var body: some View {
        HStack {
            // Token icon
            Circle()
                .fill(Color.etridBlue.opacity(0.2))
                .frame(width: 40, height: 40)
                .overlay(
                    Text(token.symbol.prefix(1))
                        .font(.headline)
                        .foregroundColor(.etridBlue)
                )

            VStack(alignment: .leading, spacing: 4) {
                Text(token.symbol)
                    .font(.headline)

                Text(token.name)
                    .font(.caption)
                    .foregroundColor(.secondary)
            }

            Spacer()

            VStack(alignment: .trailing, spacing: 4) {
                Text(token.displayBalance)
                    .font(.headline)

                Text(token.symbol)
                    .font(.caption)
                    .foregroundColor(.secondary)
            }
        }
        .padding()
        .background(Color(.systemGray6))
        .cornerRadius(12)
    }
}

@MainActor
class HomeViewModel: ObservableObject {
    @Published var currentAccount: WalletAccount?
    @Published var currentNetwork: Network = .ethereum {
        didSet {
            if oldValue.chainId != currentNetwork.chainId {
                Task {
                    await loadBalance()
                    await loadTokens()
                }
            }
        }
    }
    @Published var balance: String = "0"
    @Published var balanceUSD: String?
    @Published var tokens: [Token] = []
    @Published var isLoadingBalance = false
    @Published var showSendView = false
    @Published var showReceiveView = false
    @Published var showActivityView = false

    var formattedBalance: String {
        WeiConverter.formatWeiAsEther(balance) + " " + currentNetwork.symbol
    }

    func loadData() async {
        await loadAccount()
        await loadBalance()
        await loadTokens()
    }

    func loadAccount() async {
        do {
            currentAccount = try await WalletStorage.shared.getPrimaryAccount()
        } catch {
            ErrorLogger.log(error, context: "Load account")
        }
    }

    func loadBalance() async {
        guard let address = currentAccount?.address else { return }

        isLoadingBalance = true

        do {
            let web3 = Web3Service(network: currentNetwork)
            balance = try await web3.getBalance(address: address)

            // Get USD value
            let ethBalance = WeiConverter.weiToEther(balance)
            let price = try await PriceService.shared.getPrice(for: currentNetwork.symbol)
            let usd = ethBalance * price.price
            balanceUSD = String(format: "$%.2f", usd)
        } catch {
            ErrorLogger.log(error, context: "Load balance")
        }

        isLoadingBalance = false
    }

    func loadTokens() async {
        guard let address = currentAccount?.address else { return }

        do {
            let defaultTokens = await TokenListManager.shared.getTokens(for: currentNetwork)
            let tokenService = TokenService(network: currentNetwork)

            var loadedTokens: [Token] = []

            for var token in defaultTokens.prefix(5) {
                do {
                    let balance = try await tokenService.getBalance(token: token, address: address)
                    token.balance = balance

                    if let balanceValue = BigInt.fromHex(balance), balanceValue > 0 {
                        loadedTokens.append(token)
                    }
                } catch {
                    continue
                }
            }

            tokens = loadedTokens
        } catch {
            ErrorLogger.log(error, context: "Load tokens")
        }
    }

    func refreshBalances() {
        Task {
            await loadBalance()
            await loadTokens()
        }
    }
}

private struct NetworkPickerView: View {
    @Environment(\.dismiss) private var dismiss
    @Binding var selectedNetwork: Network

    var body: some View {
        NavigationStack {
            List(Networks.allMainnets) { network in
                Button(action: {
                    selectedNetwork = network
                    dismiss()
                }) {
                    HStack {
                        Circle()
                            .fill(Color.etridBlue)
                            .frame(width: 10, height: 10)

                        Text(network.name)

                        Spacer()

                        if network.chainId == selectedNetwork.chainId {
                            Image(systemName: "checkmark")
                                .foregroundColor(.etridBlue)
                        }
                    }
                }
            }
            .navigationTitle("Select Network")
            .navigationBarTitleDisplayMode(.inline)
            .toolbar {
                ToolbarItem(placement: .cancellationAction) {
                    Button("Cancel") {
                        dismiss()
                    }
                }
            }
        }
    }
}

private struct AccountPickerView: View {
    @Environment(\.dismiss) private var dismiss

    var body: some View {
        NavigationStack {
            List {
                Text("Account management coming soon")
                    .foregroundColor(.secondary)
            }
            .navigationTitle("Select Account")
            .navigationBarTitleDisplayMode(.inline)
            .toolbar {
                ToolbarItem(placement: .cancellationAction) {
                    Button("Cancel") {
                        dismiss()
                    }
                }
            }
        }
    }
}

#Preview {
    HomeView()
}
