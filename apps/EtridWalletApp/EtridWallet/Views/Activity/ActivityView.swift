//
//  ActivityView.swift
//  EtridWallet
//
//  Transaction history screen
//

import SwiftUI

struct ActivityView: View {
    @Environment(\.dismiss) private var dismiss
    @StateObject private var viewModel = ActivityViewModel()

    var body: some View {
        NavigationStack {
            Group {
                if viewModel.isLoading {
                    ProgressView("Loading transactions...")
                } else if viewModel.transactions.isEmpty {
                    EmptyActivityView()
                } else {
                    List {
                        ForEach(viewModel.groupedTransactions.keys.sorted(by: >), id: \.self) { date in
                            Section(header: Text(formatDate(date))) {
                                ForEach(viewModel.groupedTransactions[date] ?? []) { transaction in
                                    TransactionRowView(transaction: transaction)
                                        .onTapGesture {
                                            viewModel.selectedTransaction = transaction
                                        }
                                }
                            }
                        }
                    }
                    .listStyle(.insetGrouped)
                }
            }
            .navigationTitle("Activity")
            .navigationBarTitleDisplayMode(.inline)
            .toolbar {
                ToolbarItem(placement: .cancellationAction) {
                    Button("Done") {
                        dismiss()
                    }
                }

                ToolbarItem(placement: .navigationBarTrailing) {
                    Button(action: {
                        Task {
                            await viewModel.loadTransactions()
                        }
                    }) {
                        Image(systemName: "arrow.clockwise")
                    }
                }
            }
            .sheet(item: $viewModel.selectedTransaction) { transaction in
                TransactionDetailView(transaction: transaction)
            }
            .refreshable {
                await viewModel.loadTransactions()
            }
        }
        .task {
            await viewModel.loadTransactions()
        }
    }

    func formatDate(_ date: Date) -> String {
        let calendar = Calendar.current
        if calendar.isDateInToday(date) {
            return "Today"
        } else if calendar.isDateInYesterday(date) {
            return "Yesterday"
        } else {
            let formatter = DateFormatter()
            formatter.dateStyle = .medium
            return formatter.string(from: date)
        }
    }
}

private struct EmptyActivityView: View {
    var body: some View {
        VStack(spacing: 20) {
            Image(systemName: "clock.arrow.circlepath")
                .font(.system(size: 60))
                .foregroundColor(.secondary)

            Text("No transactions yet")
                .font(.headline)

            Text("Your transaction history will appear here")
                .font(.subheadline)
                .foregroundColor(.secondary)
                .multilineTextAlignment(.center)
        }
        .padding()
    }
}

private struct TransactionRowView: View {
    let transaction: Transaction

    var body: some View {
        HStack(spacing: 12) {
            // Icon
            Circle()
                .fill(iconColor.opacity(0.2))
                .frame(width: 40, height: 40)
                .overlay(
                    Image(systemName: transaction.type.iconName)
                        .foregroundColor(iconColor)
                )

            // Details
            VStack(alignment: .leading, spacing: 4) {
                Text(transaction.type.rawValue)
                    .font(.headline)

                Text(transaction.shortHash)
                    .font(.caption)
                    .foregroundColor(.secondary)

                Text(transaction.timestamp.relativeTime)
                    .font(.caption2)
                    .foregroundColor(.secondary)
            }

            Spacer()

            // Amount & Status
            VStack(alignment: .trailing, spacing: 4) {
                Text(transaction.formattedValue)
                    .font(.headline)

                HStack(spacing: 4) {
                    Circle()
                        .fill(statusColor)
                        .frame(width: 6, height: 6)

                    Text(transaction.status.rawValue)
                        .font(.caption)
                        .foregroundColor(statusColor)
                }
            }
        }
        .padding(.vertical, 4)
    }

    var iconColor: Color {
        switch transaction.type {
        case .send:
            return .etridOrange
        case .receive:
            return .etridGreen
        default:
            return .etridBlue
        }
    }

    var statusColor: Color {
        switch transaction.status {
        case .confirmed:
            return .etridGreen
        case .pending:
            return .orange
        case .failed:
            return .etridRed
        default:
            return .secondary
        }
    }
}

private struct TransactionDetailView: View {
    @Environment(\.dismiss) private var dismiss
    let transaction: Transaction

    var body: some View {
        NavigationStack {
            List {
                Section("Details") {
                    DetailRow(label: "Status", value: transaction.status.rawValue)
                    DetailRow(label: "Type", value: transaction.type.rawValue)
                    DetailRow(label: "Amount", value: transaction.formattedValue + " ETH")
                    DetailRow(label: "Network Fee", value: transaction.formattedGasFee + " ETH")
                }

                Section("Addresses") {
                    DetailRow(label: "From", value: transaction.from.formatAddress())
                    DetailRow(label: "To", value: transaction.to.formatAddress())
                }

                Section("Transaction") {
                    DetailRow(label: "Hash", value: transaction.hash.formatAddress())
                    DetailRow(label: "Network", value: transaction.network)
                    DetailRow(label: "Timestamp", value: transaction.timestamp.fullDateTime)

                    if let blockNumber = transaction.blockNumber {
                        DetailRow(label: "Block", value: "\(blockNumber)")
                    }
                }

                Button(action: {
                    // Open in explorer
                }) {
                    Label("View on Explorer", systemImage: "arrow.up.right.square")
                }
            }
            .navigationTitle("Transaction Details")
            .navigationBarTitleDisplayMode(.inline)
            .toolbar {
                ToolbarItem(placement: .cancellationAction) {
                    Button("Done") {
                        dismiss()
                    }
                }
            }
        }
    }
}

private struct DetailRow: View {
    let label: String
    let value: String

    var body: some View {
        HStack {
            Text(label)
                .foregroundColor(.secondary)

            Spacer()

            Text(value)
                .font(.system(.body, design: .monospaced))
                .lineLimit(1)
                .truncationMode(.middle)
        }
    }
}

@MainActor
class ActivityViewModel: ObservableObject {
    @Published var transactions: [Transaction] = []
    @Published var isLoading = false
    @Published var selectedTransaction: Transaction?

    var groupedTransactions: [Date: [Transaction]] {
        Dictionary(grouping: transactions) { transaction in
            Calendar.current.startOfDay(for: transaction.timestamp)
        }
    }

    func loadTransactions() async {
        isLoading = true

        do {
            guard let account = try await WalletStorage.shared.getPrimaryAccount() else {
                return
            }

            transactions = try await WalletStorage.shared.getTransactions(for: account.address)
        } catch {
            ErrorLogger.log(error, context: "Load transactions")
        }

        isLoading = false
    }
}

#Preview {
    ActivityView()
}
