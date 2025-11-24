//
//  ActiveSessionsView.swift
//  EtridWalletSwift
//
//  Created by Etrid Team on 2025-11-22.
//  Copyright © 2025 Etrid. All rights reserved.
//

import SwiftUI

/// View for managing active WalletConnect sessions
struct ActiveSessionsView: View {
    // MARK: - Properties

    @ObservedObject var walletConnectService: WalletConnectService
    @State private var showingDisconnectAlert = false
    @State private var sessionToDisconnect: WalletConnectSession?
    @State private var showError = false
    @State private var errorMessage = ""
    @State private var selectedSession: WalletConnectSession?
    @State private var showSessionDetails = false

    // MARK: - Body

    var body: some View {
        NavigationView {
            Group {
                if walletConnectService.activeSessions.isEmpty {
                    emptyStateView
                } else {
                    sessionsList
                }
            }
            .navigationTitle("Connected dApps")
            .navigationBarTitleDisplayMode(.large)
            .toolbar {
                ToolbarItem(placement: .navigationBarTrailing) {
                    Button {
                        // Show QR scanner
                    } label: {
                        Image(systemName: "qrcode.viewfinder")
                    }
                }
            }
            .alert("Disconnect Session", isPresented: $showingDisconnectAlert, presenting: sessionToDisconnect) { session in
                Button("Cancel", role: .cancel) {}
                Button("Disconnect", role: .destructive) {
                    Task {
                        await disconnect(session)
                    }
                }
            } message: { session in
                Text("Are you sure you want to disconnect from \(session.dAppMetadata.name)?")
            }
            .alert("Error", isPresented: $showError) {
                Button("OK", role: .cancel) {}
            } message: {
                Text(errorMessage)
            }
            .sheet(item: $selectedSession) { session in
                SessionDetailsView(session: session, walletConnectService: walletConnectService)
            }
        }
    }

    // MARK: - View Components

    private var emptyStateView: some View {
        VStack(spacing: 24) {
            Image(systemName: "link.circle")
                .font(.system(size: 80))
                .foregroundColor(.secondary)

            VStack(spacing: 8) {
                Text("No Connected dApps")
                    .font(.title2)
                    .fontWeight(.bold)

                Text("Scan a QR code to connect to a dApp")
                    .font(.subheadline)
                    .foregroundColor(.secondary)
                    .multilineTextAlignment(.center)
            }

            Button {
                // Show QR scanner
            } label: {
                HStack {
                    Image(systemName: "qrcode.viewfinder")
                    Text("Scan QR Code")
                }
                .fontWeight(.semibold)
                .foregroundColor(.white)
                .padding()
                .background(
                    RoundedRectangle(cornerRadius: 12)
                        .fill(Color.blue)
                )
            }
        }
        .padding()
    }

    private var sessionsList: some View {
        ScrollView {
            VStack(spacing: 16) {
                // Active sessions header
                HStack {
                    Text("Active Connections")
                        .font(.headline)
                        .foregroundColor(.secondary)

                    Spacer()

                    Text("\(walletConnectService.activeSessions.count)")
                        .font(.subheadline)
                        .foregroundColor(.secondary)
                }
                .padding(.horizontal)
                .padding(.top)

                // Sessions list
                ForEach(walletConnectService.activeSessions) { session in
                    SessionRow(
                        session: session,
                        onTap: {
                            selectedSession = session
                            showSessionDetails = true
                        },
                        onDisconnect: {
                            sessionToDisconnect = session
                            showingDisconnectAlert = true
                        }
                    )
                }
                .padding(.horizontal)
            }
        }
    }

    // MARK: - Actions

    private func disconnect(_ session: WalletConnectSession) async {
        do {
            try await walletConnectService.disconnectSession(session)
        } catch {
            await MainActor.run {
                errorMessage = error.localizedDescription
                showError = true
            }
        }
    }
}

// MARK: - Session Row

struct SessionRow: View {
    let session: WalletConnectSession
    let onTap: () -> Void
    let onDisconnect: () -> Void

    @State private var showingActions = false

    var body: some View {
        Button(action: onTap) {
            HStack(spacing: 12) {
                // dApp Icon
                AsyncImage(url: session.dAppMetadata.iconURL) { phase in
                    switch phase {
                    case .empty:
                        ProgressView()
                    case .success(let image):
                        image
                            .resizable()
                            .aspectRatio(contentMode: .fit)
                    case .failure:
                        Image(systemName: "app.badge")
                            .foregroundColor(.secondary)
                    @unknown default:
                        EmptyView()
                    }
                }
                .frame(width: 50, height: 50)
                .clipShape(RoundedRectangle(cornerRadius: 10))

                // Session info
                VStack(alignment: .leading, spacing: 4) {
                    Text(session.dAppMetadata.name)
                        .font(.headline)
                        .foregroundColor(.primary)

                    Text(session.dAppMetadata.displayURL)
                        .font(.caption)
                        .foregroundColor(.secondary)

                    // Connection status
                    HStack(spacing: 4) {
                        Circle()
                            .fill(session.isExpired ? Color.red : Color.green)
                            .frame(width: 6, height: 6)

                        Text(session.isExpired ? "Expired" : "Connected")
                            .font(.caption2)
                            .foregroundColor(.secondary)

                        Text("•")
                            .font(.caption2)
                            .foregroundColor(.secondary)

                        Text(timeAgo(from: session.createdAt))
                            .font(.caption2)
                            .foregroundColor(.secondary)
                    }
                }

                Spacer()

                // More button
                Button(action: { showingActions = true }) {
                    Image(systemName: "ellipsis")
                        .foregroundColor(.secondary)
                        .frame(width: 30, height: 30)
                        .background(
                            Circle()
                                .fill(Color(.tertiarySystemBackground))
                        )
                }
                .buttonStyle(PlainButtonStyle())
            }
            .padding()
            .background(
                RoundedRectangle(cornerRadius: 12)
                    .fill(Color(.secondarySystemBackground))
            )
        }
        .buttonStyle(PlainButtonStyle())
        .confirmationDialog("Session Actions", isPresented: $showingActions) {
            Button("View Details") {
                onTap()
            }

            Button("Disconnect", role: .destructive) {
                onDisconnect()
            }

            Button("Cancel", role: .cancel) {}
        }
    }

    private func timeAgo(from date: Date) -> String {
        let interval = Date().timeIntervalSince(date)

        if interval < 60 {
            return "Just now"
        } else if interval < 3600 {
            let minutes = Int(interval / 60)
            return "\(minutes)m ago"
        } else if interval < 86400 {
            let hours = Int(interval / 3600)
            return "\(hours)h ago"
        } else {
            let days = Int(interval / 86400)
            return "\(days)d ago"
        }
    }
}

// MARK: - Session Details View

struct SessionDetailsView: View {
    let session: WalletConnectSession
    @ObservedObject var walletConnectService: WalletConnectService

    @Environment(\.dismiss) private var dismiss
    @State private var showingDisconnectAlert = false

    var body: some View {
        NavigationView {
            ScrollView {
                VStack(spacing: 24) {
                    // dApp Header
                    dAppHeaderSection

                    // Connection Info
                    connectionInfoSection

                    // Connected Accounts
                    connectedAccountsSection

                    // Supported Networks
                    supportedNetworksSection

                    // Permissions
                    permissionsSection

                    // Disconnect Button
                    disconnectButton
                }
                .padding()
            }
            .navigationTitle("Session Details")
            .navigationBarTitleDisplayMode(.inline)
            .toolbar {
                ToolbarItem(placement: .navigationBarTrailing) {
                    Button("Done") {
                        dismiss()
                    }
                }
            }
            .alert("Disconnect Session", isPresented: $showingDisconnectAlert) {
                Button("Cancel", role: .cancel) {}
                Button("Disconnect", role: .destructive) {
                    Task {
                        try? await walletConnectService.disconnectSession(session)
                        dismiss()
                    }
                }
            } message: {
                Text("Are you sure you want to disconnect from \(session.dAppMetadata.name)?")
            }
        }
    }

    private var dAppHeaderSection: some View {
        VStack(spacing: 16) {
            AsyncImage(url: session.dAppMetadata.iconURL) { phase in
                switch phase {
                case .empty:
                    ProgressView()
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

            Text(session.dAppMetadata.name)
                .font(.title2)
                .fontWeight(.bold)

            HStack {
                Image(systemName: "globe")
                Text(session.dAppMetadata.displayURL)
            }
            .font(.subheadline)
            .foregroundColor(.secondary)
        }
    }

    private var connectionInfoSection: some View {
        VStack(alignment: .leading, spacing: 12) {
            Text("Connection Info")
                .font(.headline)

            InfoRow(
                label: "Status",
                value: session.isExpired ? "Expired" : "Active",
                valueColor: session.isExpired ? .red : .green
            )

            InfoRow(
                label: "Connected",
                value: formatDate(session.createdAt)
            )

            InfoRow(
                label: "Expires",
                value: formatDate(session.expiresAt)
            )

            InfoRow(
                label: "Session ID",
                value: String(session.id.prefix(16)) + "...",
                monospaced: true
            )
        }
        .padding()
        .background(
            RoundedRectangle(cornerRadius: 12)
                .fill(Color(.secondarySystemBackground))
        )
    }

    private var connectedAccountsSection: some View {
        VStack(alignment: .leading, spacing: 12) {
            Text("Connected Accounts")
                .font(.headline)

            ForEach(session.accounts, id: \.self) { address in
                HStack {
                    Image(systemName: "person.circle.fill")
                        .foregroundColor(.blue)

                    Text(formatAddress(address))
                        .font(.system(.subheadline, design: .monospaced))

                    Spacer()

                    Button {
                        UIPasteboard.general.string = address
                    } label: {
                        Image(systemName: "doc.on.doc")
                            .foregroundColor(.blue)
                    }
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

    private var supportedNetworksSection: some View {
        VStack(alignment: .leading, spacing: 12) {
            Text("Supported Networks")
                .font(.headline)

            ForEach(session.chains, id: \.self) { chain in
                HStack {
                    Image(systemName: "link.circle.fill")
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

    private var permissionsSection: some View {
        VStack(alignment: .leading, spacing: 12) {
            Text("Granted Permissions")
                .font(.headline)

            ForEach(session.supportedMethods.prefix(10), id: \.self) { method in
                HStack {
                    Image(systemName: "checkmark.circle.fill")
                        .foregroundColor(.green)
                        .font(.caption)

                    Text(formatMethodName(method))
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

    private var disconnectButton: some View {
        Button {
            showingDisconnectAlert = true
        } label: {
            Text("Disconnect Session")
                .fontWeight(.semibold)
                .foregroundColor(.white)
                .frame(maxWidth: .infinity)
                .padding()
                .background(
                    RoundedRectangle(cornerRadius: 12)
                        .fill(Color.red)
                )
        }
    }

    // MARK: - Helper Methods

    private func formatDate(_ date: Date) -> String {
        let formatter = DateFormatter()
        formatter.dateStyle = .medium
        formatter.timeStyle = .short
        return formatter.string(from: date)
    }

    private func formatAddress(_ address: String) -> String {
        guard address.count >= 42 else { return address }
        return String(address.prefix(10)) + "..." + String(address.suffix(8))
    }

    private func formatChainId(_ chainId: String) -> String {
        if chainId == "eip155:1" {
            return "Ethereum Mainnet"
        } else if chainId == "eip155:137" {
            return "Polygon"
        } else if chainId == "eip155:56" {
            return "Binance Smart Chain"
        }
        return chainId
    }

    private func formatMethodName(_ method: String) -> String {
        switch method {
        case "eth_sendTransaction":
            return "Send Transactions"
        case "personal_sign":
            return "Sign Messages"
        case "eth_signTypedData_v4":
            return "Sign Typed Data"
        case "wallet_switchEthereumChain":
            return "Switch Networks"
        default:
            return method.replacingOccurrences(of: "_", with: " ").capitalized
        }
    }
}

// MARK: - Info Row

struct InfoRow: View {
    let label: String
    let value: String
    var valueColor: Color = .primary
    var monospaced: Bool = false

    var body: some View {
        HStack {
            Text(label)
                .font(.subheadline)
                .foregroundColor(.secondary)

            Spacer()

            Text(value)
                .font(monospaced ? .system(.subheadline, design: .monospaced) : .subheadline)
                .foregroundColor(valueColor)
        }
    }
}

// MARK: - Preview

struct ActiveSessionsView_Previews: PreviewProvider {
    static var previews: some View {
        ActiveSessionsView(
            walletConnectService: WalletConnectService(
                projectId: "test",
                keychainManager: KeychainManager(),
                networkManager: NetworkManager()
            )
        )
    }
}
