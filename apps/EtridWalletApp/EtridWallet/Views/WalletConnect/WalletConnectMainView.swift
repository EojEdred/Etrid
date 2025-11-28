//
//  WalletConnectMainView.swift
//  EtridWallet
//
//  Main WalletConnect interface
//

import SwiftUI

struct WalletConnectMainView: View {
    @StateObject private var service = WalletConnectService.shared
    @State private var showScanner = false
    @State private var showProposals = false

    var body: some View {
        NavigationView {
            List {
                // Active Sessions
                Section {
                    if service.sessions.isEmpty {
                        EmptySessionsView {
                            showScanner = true
                        }
                    } else {
                        ForEach(service.sessions) { session in
                            NavigationLink {
                                SessionDetailView(session: session)
                            } label: {
                                SessionRow(session: session)
                            }
                        }
                    }
                } header: {
                    Text("Active Sessions (\(service.sessions.count))")
                }

                // Pending Proposals
                if !service.proposals.isEmpty {
                    Section {
                        ForEach(service.proposals) { proposal in
                            ProposalRow(proposal: proposal)
                        }
                    } header: {
                        Text("Pending Proposals")
                    }
                }

                // Info Section
                Section {
                    InfoRow(
                        icon: "qrcode",
                        title: "How to Connect",
                        subtitle: "Scan QR code from dApp to connect"
                    )
                } header: {
                    Text("Information")
                }
            }
            .navigationTitle("WalletConnect")
            .toolbar {
                ToolbarItem(placement: .navigationBarTrailing) {
                    Button(action: { showScanner = true }) {
                        Image(systemName: "qrcode.viewfinder")
                    }
                }
            }
            .sheet(isPresented: $showScanner) {
                WalletConnectScannerView()
            }
            .sheet(isPresented: $showProposals) {
                ProposalsView()
            }
            .task {
                try? await service.initialize()
                service.cleanExpiredSessions()
                service.cleanExpiredProposals()
            }
        }
    }
}

private struct SessionRow: View {
    let session: WCSession

    var body: some View {
        HStack(spacing: 12) {
            // Icon
            Circle()
                .fill(Color.green.opacity(0.1))
                .frame(width: 50, height: 50)
                .overlay(
                    Image(systemName: "link")
                        .foregroundColor(.green)
                )

            // Info
            VStack(alignment: .leading, spacing: 4) {
                Text(session.dAppName)
                    .font(.headline)

                Text(session.dAppUrl)
                    .font(.caption)
                    .foregroundColor(.secondary)
                    .lineLimit(1)

                HStack(spacing: 4) {
                    Image(systemName: "clock")
                        .font(.caption2)
                    Text("Expires in \(session.daysUntilExpiry) days")
                        .font(.caption2)
                }
                .foregroundColor(.secondary)
            }

            Spacer()

            Image(systemName: "chevron.right")
                .font(.caption)
                .foregroundColor(.secondary)
        }
        .padding(.vertical, 4)
    }
}

private struct EmptySessionsView: View {
    let onScan: () -> Void

    var body: some View {
        VStack(spacing: 16) {
            Image(systemName: "link.badge.plus")
                .font(.system(size: 48))
                .foregroundColor(.secondary)

            Text("No Active Sessions")
                .font(.headline)

            Text("Scan a QR code to connect with a dApp")
                .font(.caption)
                .foregroundColor(.secondary)
                .multilineTextAlignment(.center)

            Button("Scan QR Code") {
                onScan()
            }
            .buttonStyle(.borderedProminent)
        }
        .padding()
        .frame(maxWidth: .infinity)
    }
}

struct ProposalRow: View {
    let proposal: WCProposal
    @StateObject private var service = WalletConnectService.shared
    @State private var showDetail = false

    var body: some View {
        Button(action: { showDetail = true }) {
            HStack(spacing: 12) {
                Circle()
                    .fill(Color.orange.opacity(0.1))
                    .frame(width: 50, height: 50)
                    .overlay(
                        Image(systemName: "exclamationmark")
                            .foregroundColor(.orange)
                    )

                VStack(alignment: .leading, spacing: 4) {
                    Text(proposal.proposer.name)
                        .font(.headline)

                    Text("Wants to connect")
                        .font(.caption)
                        .foregroundColor(.secondary)

                    Text(proposal.permissions.chainsDescription)
                        .font(.caption2)
                        .foregroundColor(.secondary)
                }

                Spacer()
            }
            .padding(.vertical, 4)
        }
        .sheet(isPresented: $showDetail) {
            WCProposalDetailView(proposal: proposal)
        }
    }
}

private struct InfoRow: View {
    let icon: String
    let title: String
    let subtitle: String

    var body: some View {
        HStack(spacing: 12) {
            Image(systemName: icon)
                .foregroundColor(.blue)
                .frame(width: 30)

            VStack(alignment: .leading, spacing: 2) {
                Text(title)
                    .font(.subheadline)
                Text(subtitle)
                    .font(.caption)
                    .foregroundColor(.secondary)
            }
        }
    }
}

#Preview {
    WalletConnectMainView()
}
