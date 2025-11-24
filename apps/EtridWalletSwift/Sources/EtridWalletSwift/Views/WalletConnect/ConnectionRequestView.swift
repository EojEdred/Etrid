//
//  ConnectionRequestView.swift
//  EtridWalletSwift
//
//  Created by Etrid Team on 2025-11-22.
//  Copyright © 2025 Etrid. All rights reserved.
//

import SwiftUI

/// View for displaying WalletConnect connection requests
struct ConnectionRequestView: View {
    // MARK: - Properties

    let proposal: SessionProposal
    @ObservedObject var walletConnectService: WalletConnectService
    @ObservedObject var accountManager: AccountManager

    @Environment(\.dismiss) private var dismiss
    @State private var isApproving = false
    @State private var isRejecting = false
    @State private var selectedAccounts: Set<String> = []
    @State private var showError = false
    @State private var errorMessage = ""

    // MARK: - Body

    var body: some View {
        NavigationView {
            ScrollView {
                VStack(spacing: 24) {
                    // Header
                    headerSection

                    // dApp Information
                    dAppInfoSection

                    // Requested Permissions
                    permissionsSection

                    // Requested Chains
                    chainsSection

                    // Account Selection
                    accountSelectionSection

                    // Warning Notice
                    warningSection

                    // Action Buttons
                    actionButtons
                }
                .padding()
            }
            .navigationTitle("Connection Request")
            .navigationBarTitleDisplayMode(.inline)
            .toolbar {
                ToolbarItem(placement: .navigationBarLeading) {
                    Button("Cancel") {
                        Task {
                            await rejectConnection()
                        }
                    }
                    .disabled(isApproving || isRejecting)
                }
            }
            .alert("Error", isPresented: $showError) {
                Button("OK", role: .cancel) {}
            } message: {
                Text(errorMessage)
            }
        }
        .onAppear {
            // Pre-select first account
            if let firstAccount = accountManager.accounts.first {
                selectedAccounts.insert(firstAccount.address)
            }
        }
    }

    // MARK: - View Components

    private var headerSection: some View {
        VStack(spacing: 16) {
            // dApp Icon
            AsyncImage(url: proposal.proposer.iconURL) { phase in
                switch phase {
                case .empty:
                    ProgressView()
                        .frame(width: 80, height: 80)
                case .success(let image):
                    image
                        .resizable()
                        .aspectRatio(contentMode: .fit)
                case .failure:
                    Image(systemName: "app.badge")
                        .font(.system(size: 40))
                        .foregroundColor(.secondary)
                @unknown default:
                    EmptyView()
                }
            }
            .frame(width: 80, height: 80)
            .clipShape(RoundedRectangle(cornerRadius: 16))
            .shadow(color: .black.opacity(0.1), radius: 8, y: 4)

            Text("wants to connect to your wallet")
                .font(.headline)
                .foregroundColor(.secondary)
        }
    }

    private var dAppInfoSection: some View {
        VStack(spacing: 12) {
            Text(proposal.proposer.name)
                .font(.title2)
                .fontWeight(.bold)

            HStack {
                Image(systemName: "globe")
                    .font(.caption)
                    .foregroundColor(.secondary)

                Text(proposal.proposer.displayURL)
                    .font(.subheadline)
                    .foregroundColor(.secondary)
            }

            if !proposal.proposer.description.isEmpty {
                Text(proposal.proposer.description)
                    .font(.callout)
                    .foregroundColor(.secondary)
                    .multilineTextAlignment(.center)
                    .padding(.horizontal)
            }
        }
        .padding()
        .background(
            RoundedRectangle(cornerRadius: 12)
                .fill(Color(.systemBackground))
                .shadow(color: .black.opacity(0.05), radius: 4, y: 2)
        )
    }

    private var permissionsSection: some View {
        VStack(alignment: .leading, spacing: 16) {
            Text("Requested Permissions")
                .font(.headline)
                .fontWeight(.semibold)

            VStack(alignment: .leading, spacing: 12) {
                ForEach(proposal.permissions, id: \.self) { permission in
                    HStack(spacing: 12) {
                        Image(systemName: permissionIcon(for: permission))
                            .font(.system(size: 16))
                            .foregroundColor(.blue)
                            .frame(width: 24)

                        Text(permission)
                            .font(.subheadline)

                        Spacer()
                    }
                    .padding(.vertical, 8)
                }
            }
            .padding()
            .background(
                RoundedRectangle(cornerRadius: 12)
                    .fill(Color(.secondarySystemBackground))
            )
        }
    }

    private var chainsSection: some View {
        VStack(alignment: .leading, spacing: 16) {
            Text("Supported Networks")
                .font(.headline)
                .fontWeight(.semibold)

            VStack(alignment: .leading, spacing: 8) {
                ForEach(proposal.requestedChains, id: \.self) { chain in
                    HStack {
                        Image(systemName: "link")
                            .font(.caption)
                            .foregroundColor(.blue)

                        Text(formatChainId(chain))
                            .font(.subheadline)

                        Spacer()
                    }
                    .padding(.vertical, 4)
                }
            }
            .padding()
            .background(
                RoundedRectangle(cornerRadius: 12)
                    .fill(Color(.secondarySystemBackground))
            )
        }
    }

    private var accountSelectionSection: some View {
        VStack(alignment: .leading, spacing: 16) {
            Text("Select Accounts to Connect")
                .font(.headline)
                .fontWeight(.semibold)

            VStack(spacing: 12) {
                ForEach(accountManager.accounts) { account in
                    AccountSelectionRow(
                        account: account,
                        isSelected: selectedAccounts.contains(account.address)
                    ) {
                        toggleAccountSelection(account)
                    }
                }
            }
        }
    }

    private var warningSection: some View {
        HStack(spacing: 12) {
            Image(systemName: "exclamationmark.triangle.fill")
                .foregroundColor(.orange)

            VStack(alignment: .leading, spacing: 4) {
                Text("Only connect to websites you trust")
                    .font(.subheadline)
                    .fontWeight(.semibold)

                Text("This dApp will be able to view your account balance and request transaction approvals")
                    .font(.caption)
                    .foregroundColor(.secondary)
            }

            Spacer()
        }
        .padding()
        .background(
            RoundedRectangle(cornerRadius: 12)
                .fill(Color.orange.opacity(0.1))
        )
    }

    private var actionButtons: some View {
        VStack(spacing: 12) {
            Button {
                Task {
                    await approveConnection()
                }
            } label: {
                HStack {
                    if isApproving {
                        ProgressView()
                            .progressViewStyle(CircularProgressViewStyle(tint: .white))
                    } else {
                        Text("Connect")
                            .fontWeight(.semibold)
                    }
                }
                .frame(maxWidth: .infinity)
                .padding()
                .background(
                    RoundedRectangle(cornerRadius: 12)
                        .fill(selectedAccounts.isEmpty ? Color.gray : Color.blue)
                )
                .foregroundColor(.white)
            }
            .disabled(isApproving || isRejecting || selectedAccounts.isEmpty)

            Button {
                Task {
                    await rejectConnection()
                }
            } label: {
                HStack {
                    if isRejecting {
                        ProgressView()
                    } else {
                        Text("Reject")
                            .fontWeight(.medium)
                    }
                }
                .frame(maxWidth: .infinity)
                .padding()
                .background(
                    RoundedRectangle(cornerRadius: 12)
                        .stroke(Color.red, lineWidth: 2)
                )
                .foregroundColor(.red)
            }
            .disabled(isApproving || isRejecting)
        }
        .padding(.top, 8)
    }

    // MARK: - Actions

    private func approveConnection() async {
        guard !selectedAccounts.isEmpty else { return }

        isApproving = true

        do {
            let accounts = accountManager.accounts.filter { selectedAccounts.contains($0.address) }

            try await walletConnectService.approveSession(proposal, accounts: accounts)

            await MainActor.run {
                dismiss()
            }

        } catch {
            await MainActor.run {
                errorMessage = error.localizedDescription
                showError = true
                isApproving = false
            }
        }
    }

    private func rejectConnection() async {
        isRejecting = true

        do {
            try await walletConnectService.rejectSession(proposal)

            await MainActor.run {
                dismiss()
            }

        } catch {
            await MainActor.run {
                errorMessage = error.localizedDescription
                showError = true
                isRejecting = false
            }
        }
    }

    private func toggleAccountSelection(_ account: Account) {
        if selectedAccounts.contains(account.address) {
            selectedAccounts.remove(account.address)
        } else {
            selectedAccounts.insert(account.address)
        }
    }

    // MARK: - Helper Methods

    private func permissionIcon(for permission: String) -> String {
        if permission.contains("transaction") {
            return "arrow.up.arrow.down.circle.fill"
        } else if permission.contains("sign") || permission.contains("Sign") {
            return "signature"
        } else if permission.contains("balance") {
            return "dollarsign.circle.fill"
        } else if permission.contains("address") {
            return "person.crop.circle.fill"
        }
        return "checkmark.circle.fill"
    }

    private func formatChainId(_ chainId: String) -> String {
        if chainId == "eip155:1" {
            return "Ethereum Mainnet"
        } else if chainId == "eip155:5" {
            return "Ethereum Goerli"
        } else if chainId == "eip155:137" {
            return "Polygon"
        } else if chainId == "eip155:56" {
            return "Binance Smart Chain"
        } else if chainId == "eip155:43114" {
            return "Avalanche C-Chain"
        } else if chainId.contains("eip155:") {
            let id = chainId.replacingOccurrences(of: "eip155:", with: "")
            return "Chain ID: \(id)"
        }
        return chainId
    }
}

// MARK: - Account Selection Row

struct AccountSelectionRow: View {
    let account: Account
    let isSelected: Bool
    let onTap: () -> Void

    var body: some View {
        Button(action: onTap) {
            HStack(spacing: 12) {
                // Selection indicator
                Image(systemName: isSelected ? "checkmark.circle.fill" : "circle")
                    .foregroundColor(isSelected ? .blue : .gray)
                    .font(.title3)

                // Account info
                VStack(alignment: .leading, spacing: 4) {
                    Text(account.name)
                        .font(.subheadline)
                        .fontWeight(.medium)

                    Text(account.address.prefix(10) + "..." + account.address.suffix(8))
                        .font(.caption)
                        .foregroundColor(.secondary)
                }

                Spacer()

                // Balance
                if let balance = account.balance {
                    Text(String(format: "%.4f ETH", balance))
                        .font(.caption)
                        .foregroundColor(.secondary)
                }
            }
            .padding()
            .background(
                RoundedRectangle(cornerRadius: 12)
                    .fill(isSelected ? Color.blue.opacity(0.1) : Color(.secondarySystemBackground))
            )
        }
        .buttonStyle(PlainButtonStyle())
    }
}

// MARK: - Preview

struct ConnectionRequestView_Previews: PreviewProvider {
    static var previews: some View {
        ConnectionRequestView(
            proposal: SessionProposal(
                id: "test-id",
                proposer: DAppMetadata(
                    name: "Uniswap",
                    description: "Swap, earn, and build on the leading decentralized crypto trading protocol",
                    url: "https://app.uniswap.org",
                    icons: ["https://app.uniswap.org/favicon.ico"]
                ),
                requiredNamespaces: [
                    "eip155": ProposalNamespace(
                        chains: ["eip155:1"],
                        methods: ["eth_sendTransaction", "personal_sign"],
                        events: ["chainChanged", "accountsChanged"]
                    )
                ],
                optionalNamespaces: [:],
                createdAt: Date()
            ),
            walletConnectService: WalletConnectService(
                projectId: "test",
                keychainManager: KeychainManager(),
                networkManager: NetworkManager()
            ),
            accountManager: AccountManager()
        )
    }
}
