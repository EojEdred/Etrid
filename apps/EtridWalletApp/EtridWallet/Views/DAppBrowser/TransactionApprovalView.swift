//
//  TransactionApprovalView.swift
//  EtridWallet
//
//  Transaction approval UI for dApp requests
//

import SwiftUI

struct TransactionApprovalView: View {
    let request: Web3Request
    @StateObject private var service = DAppBrowserService.shared
    @Environment(\.dismiss) private var dismiss
    @State private var isApproving = false

    var body: some View {
        NavigationView {
            ScrollView {
                VStack(spacing: 24) {
                    // dApp Info
                    VStack(spacing: 8) {
                        Circle()
                            .fill(Color.blue.opacity(0.1))
                            .frame(width: 60, height: 60)
                            .overlay(
                                Text(String(request.dAppName.prefix(1)))
                                    .font(.title)
                                    .foregroundColor(.blue)
                            )

                        Text(request.dAppName)
                            .font(.headline)

                        Text(request.dAppUrl)
                            .font(.caption)
                            .foregroundColor(.secondary)
                    }
                    .padding()
                    .frame(maxWidth: .infinity)
                    .background(Color(.systemGray6))
                    .cornerRadius(12)

                    // Request Details
                    VStack(alignment: .leading, spacing: 12) {
                        Text("Request")
                            .font(.headline)

                        DetailRow(title: "Method", value: request.methodDescription)

                        if request.method == "eth_sendTransaction" {
                            transactionDetails
                        } else if request.method == "personal_sign" {
                            messageDetails
                        }
                    }
                    .padding()
                    .background(Color(.systemGray6))
                    .cornerRadius(12)

                    // Warning
                    if request.method != "eth_requestAccounts" {
                        WarningBox(
                            message: "Only approve if you trust this dApp. This action cannot be undone."
                        )
                    }

                    Spacer()

                    // Action Buttons
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
                        .disabled(isApproving)
                    }
                }
                .padding()
            }
            .navigationTitle(request.methodDescription)
            .navigationBarTitleDisplayMode(.inline)
        }
    }

    @ViewBuilder
    private var transactionDetails: some View {
        if let txData = request.params.first as? [String: Any] {
            if let to = txData["to"] as? String {
                DetailRow(title: "To", value: to)
            }
            if let value = txData["value"] as? String {
                DetailRow(title: "Value", value: formatValue(value))
            }
            if let gas = txData["gas"] as? String {
                DetailRow(title: "Gas Limit", value: gas)
            }
        }
    }

    @ViewBuilder
    private var messageDetails: some View {
        if let message = request.params.first as? String {
            VStack(alignment: .leading, spacing: 8) {
                Text("Message")
                    .font(.caption)
                    .foregroundColor(.secondary)

                Text(message)
                    .font(.body)
                    .padding()
                    .frame(maxWidth: .infinity, alignment: .leading)
                    .background(Color(.systemBackground))
                    .cornerRadius(8)
            }
        }
    }

    private func approve() {
        isApproving = true
        Task {
            do {
                let result = try await service.handleWeb3Request(request)
                await MainActor.run {
                    dismiss()
                }
            } catch {
                print("Approval error: \(error)")
                isApproving = false
            }
        }
    }

    private func reject() {
        service.pendingRequest = nil
        dismiss()
    }

    private func formatValue(_ value: String) -> String {
        // Convert hex value to readable format
        guard let valueInt = Int(value.replacingOccurrences(of: "0x", with: ""), radix: 16) else {
            return value
        }
        let eth = Double(valueInt) / 1_000_000_000_000_000_000
        return String(format: "%.6f ETH", eth)
    }
}

private struct DetailRow: View {
    let title: String
    let value: String

    var body: some View {
        VStack(alignment: .leading, spacing: 4) {
            Text(title)
                .font(.caption)
                .foregroundColor(.secondary)
            Text(value)
                .font(.body)
                .textSelection(.enabled)
        }
    }
}

private struct WarningBox: View {
    let message: String

    var body: some View {
        HStack(spacing: 12) {
            Image(systemName: "exclamationmark.triangle.fill")
                .foregroundColor(.orange)

            Text(message)
                .font(.caption)
                .foregroundColor(.secondary)
        }
        .padding()
        .frame(maxWidth: .infinity)
        .background(Color.orange.opacity(0.1))
        .cornerRadius(8)
    }
}

struct ConnectedDAppsView: View {
    @StateObject private var service = DAppBrowserService.shared
    @Environment(\.dismiss) private var dismiss

    var body: some View {
        NavigationView {
            List {
                if service.connectedDApps.isEmpty {
                    ContentUnavailableView(
                        "No Connected dApps",
                        systemImage: "link.slash",
                        description: Text("Connected dApps will appear here")
                    )
                } else {
                    ForEach(service.connectedDApps) { dapp in
                        ConnectedDAppRow(dapp: dapp) {
                            service.disconnectDApp(dapp)
                        }
                    }
                }
            }
            .navigationTitle("Connected dApps")
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

private struct ConnectedDAppRow: View {
    let dapp: ConnectedDApp
    let onDisconnect: () -> Void

    var body: some View {
        HStack {
            VStack(alignment: .leading, spacing: 4) {
                Text(dapp.dAppName)
                    .font(.headline)
                Text(dapp.dAppUrl)
                    .font(.caption)
                    .foregroundColor(.secondary)
                Text("Connected \(dapp.connectedAt.formatted(date: .abbreviated, time: .omitted))")
                    .font(.caption2)
                    .foregroundColor(.secondary)
            }

            Spacer()

            Button("Disconnect") {
                onDisconnect()
            }
            .font(.caption)
            .foregroundColor(.red)
        }
    }
}
