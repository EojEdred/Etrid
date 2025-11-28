//
//  WalletConnectScannerView.swift
//  EtridWallet
//
//  QR code scanner for WalletConnect pairing
//

import SwiftUI

struct WalletConnectScannerView: View {
    @StateObject private var service = WalletConnectService.shared
    @Environment(\.dismiss) private var dismiss
    @State private var scannedCode: String?
    @State private var isPairing = false
    @State private var errorMessage: String?
    @State private var showManualInput = false
    @State private var manualURI = ""

    var body: some View {
        NavigationView {
            VStack(spacing: 24) {
                // QR Scanner
                QRScannerView(onCodeScanned: { code in
                    scannedCode = code
                })
                .frame(height: 300)
                .cornerRadius(12)
                .padding()

                // Instructions
                VStack(spacing: 8) {
                    Text("Scan WalletConnect QR Code")
                        .font(.headline)

                    Text("Position the QR code within the frame to connect")
                        .font(.caption)
                        .foregroundColor(.secondary)
                        .multilineTextAlignment(.center)
                }

                if let error = errorMessage {
                    Text(error)
                        .font(.caption)
                        .foregroundColor(.red)
                        .padding(.horizontal)
                }

                // Manual Input
                Button("Enter Code Manually") {
                    showManualInput = true
                }
                .font(.caption)

                Spacer()
            }
            .navigationTitle("Connect dApp")
            .navigationBarTitleDisplayMode(.inline)
            .toolbar {
                ToolbarItem(placement: .cancellationAction) {
                    Button("Cancel") {
                        dismiss()
                    }
                }
            }
            .overlay {
                if isPairing {
                    ProgressView("Connecting...")
                        .padding()
                        .background(Color(.systemBackground))
                        .cornerRadius(12)
                        .shadow(radius: 10)
                }
            }
            .onChange(of: scannedCode) { newValue in
                if let code = newValue {
                    pairWithCode(code)
                }
            }
            .sheet(isPresented: $showManualInput) {
                ManualURIInputView(uri: $manualURI) {
                    pairWithCode(manualURI)
                    showManualInput = false
                }
            }
        }
    }

    private func pairWithCode(_ code: String) {
        guard !isPairing else { return }

        isPairing = true
        errorMessage = nil

        Task {
            do {
                try await service.pair(uri: code)
                await MainActor.run {
                    isPairing = false
                    dismiss()
                }
            } catch {
                await MainActor.run {
                    isPairing = false
                    errorMessage = error.localizedDescription
                    scannedCode = nil
                }
            }
        }
    }
}

private struct ManualURIInputView: View {
    @Binding var uri: String
    let onConnect: () -> Void
    @Environment(\.dismiss) private var dismiss

    var body: some View {
        NavigationView {
            Form {
                Section {
                    TextField("wc:...", text: $uri)
                        .autocapitalization(.none)
                        .keyboardType(.default)
                } header: {
                    Text("WalletConnect URI")
                } footer: {
                    Text("Paste the WalletConnect URI from the dApp")
                }
            }
            .navigationTitle("Manual Connection")
            .navigationBarTitleDisplayMode(.inline)
            .toolbar {
                ToolbarItem(placement: .cancellationAction) {
                    Button("Cancel") {
                        dismiss()
                    }
                }
                ToolbarItem(placement: .confirmationAction) {
                    Button("Connect") {
                        onConnect()
                        dismiss()
                    }
                    .disabled(!uri.hasPrefix("wc:"))
                }
            }
        }
    }
}

struct ProposalsView: View {
    @StateObject private var service = WalletConnectService.shared
    @Environment(\.dismiss) private var dismiss

    var body: some View {
        NavigationView {
            List {
                if service.proposals.isEmpty {
                    ContentUnavailableView(
                        "No Pending Proposals",
                        systemImage: "doc.badge",
                        description: Text("Connection requests will appear here")
                    )
                } else {
                    ForEach(service.proposals) { proposal in
                        NavigationLink {
                            WCProposalDetailView(proposal: proposal)
                        } label: {
                            ProposalRow(proposal: proposal)
                        }
                    }
                }
            }
            .navigationTitle("Pending Proposals")
            .navigationBarTitleDisplayMode(.inline)
            .toolbar {
                ToolbarItem(placement: .navigationBarTrailing) {
                    Button("Done") {
                        dismiss()
                    }
                }
            }
        }
    }
}

struct WCProposalDetailView: View {
    let proposal: WCProposal
    @StateObject private var service = WalletConnectService.shared
    @Environment(\.dismiss) private var dismiss
    @State private var isProcessing = false

    var body: some View {
        ScrollView {
            VStack(spacing: 24) {
                // dApp Info
                VStack(spacing: 12) {
                    Circle()
                        .fill(Color.blue.opacity(0.1))
                        .frame(width: 80, height: 80)
                        .overlay(
                            Text(String(proposal.proposer.name.prefix(1)))
                                .font(.largeTitle)
                                .foregroundColor(.blue)
                        )

                    Text(proposal.proposer.name)
                        .font(.title2)
                        .bold()

                    Text(proposal.proposer.description)
                        .font(.subheadline)
                        .foregroundColor(.secondary)
                        .multilineTextAlignment(.center)

                    Text(proposal.proposer.url)
                        .font(.caption)
                        .foregroundColor(.secondary)
                }

                // Permissions
                VStack(alignment: .leading, spacing: 16) {
                    Text("Requested Permissions")
                        .font(.headline)

                    PermissionItem(
                        icon: "link",
                        title: "Networks",
                        value: proposal.permissions.chainsDescription
                    )

                    PermissionItem(
                        icon: "command",
                        title: "Methods",
                        value: proposal.permissions.methodsDescription
                    )
                }
                .padding()
                .background(Color(.systemGray6))
                .cornerRadius(12)

                // Warning
                HStack(spacing: 12) {
                    Image(systemName: "info.circle.fill")
                        .foregroundColor(.blue)
                    Text("Only connect with dApps you trust. This will allow the dApp to see your address and request transactions.")
                        .font(.caption)
                        .foregroundColor(.secondary)
                }
                .padding()
                .background(Color.blue.opacity(0.1))
                .cornerRadius(8)

                // Actions
                HStack(spacing: 12) {
                    Button("Reject") {
                        reject()
                    }
                    .buttonStyle(.bordered)
                    .frame(maxWidth: .infinity)

                    Button("Approve") {
                        approve()
                    }
                    .buttonStyle(.borderedProminent)
                    .frame(maxWidth: .infinity)
                }
                .disabled(isProcessing)
            }
            .padding()
        }
        .navigationTitle("Connection Request")
        .navigationBarTitleDisplayMode(.inline)
    }

    private func approve() {
        isProcessing = true
        Task {
            do {
                _ = try await service.approveSession(proposal)
                await MainActor.run {
                    dismiss()
                }
            } catch {
                print("Approval error: \(error)")
                isProcessing = false
            }
        }
    }

    private func reject() {
        isProcessing = true
        Task {
            try? await service.rejectSession(proposal)
            await MainActor.run {
                dismiss()
            }
        }
    }
}

private struct PermissionItem: View {
    let icon: String
    let title: String
    let value: String

    var body: some View {
        HStack(alignment: .top, spacing: 12) {
            Image(systemName: icon)
                .foregroundColor(.blue)
                .frame(width: 24)

            VStack(alignment: .leading, spacing: 4) {
                Text(title)
                    .font(.subheadline)
                    .foregroundColor(.secondary)
                Text(value)
                    .font(.body)
            }

            Spacer()
        }
    }
}

// MARK: - Helper Views
private struct DetailRow: View {
    let title: String
    let value: String

    var body: some View {
        HStack {
            Text(title)
                .font(.subheadline)
                .foregroundColor(.secondary)

            Spacer()

            Text(value)
                .font(.subheadline)
                .fontWeight(.medium)
        }
    }
}

struct SessionDetailView: View {
    let session: WCSession
    @StateObject private var service = WalletConnectService.shared
    @Environment(\.dismiss) private var dismiss
    @State private var showDisconnectAlert = false

    var body: some View {
        List {
            Section("dApp Information") {
                DetailRow(title: "Name", value: session.dAppName)
                DetailRow(title: "URL", value: session.dAppUrl)
                DetailRow(title: "Topic", value: session.shortTopic)
            }

            Section("Connection Details") {
                DetailRow(title: "Connected", value: session.createdAt.formatted(date: .abbreviated, time: .shortened))
                DetailRow(title: "Expires", value: session.expiresAt.formatted(date: .abbreviated, time: .shortened))
                DetailRow(title: "Days Until Expiry", value: "\(session.daysUntilExpiry)")
            }

            Section("Permissions") {
                ForEach(session.permissions, id: \.self) { permission in
                    Text(permission)
                        .font(.caption)
                }
            }

            Section("Networks") {
                ForEach(session.chains, id: \.self) { chain in
                    Text(chain)
                        .font(.caption)
                }
            }

            Section {
                Button("Disconnect", role: .destructive) {
                    showDisconnectAlert = true
                }
            }
        }
        .navigationTitle("Session Details")
        .navigationBarTitleDisplayMode(.inline)
        .alert("Disconnect Session", isPresented: $showDisconnectAlert) {
            Button("Cancel", role: .cancel) { }
            Button("Disconnect", role: .destructive) {
                disconnect()
            }
        } message: {
            Text("Are you sure you want to disconnect from \(session.dAppName)?")
        }
    }

    private func disconnect() {
        Task {
            try? await service.disconnect(session)
            await MainActor.run {
                dismiss()
            }
        }
    }
}
