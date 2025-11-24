//
//  BridgeHistoryView.swift
//  EtridWallet
//
//  Bridge Transaction History View
//

import SwiftUI

struct BridgeHistoryView: View {
    @StateObject private var service = HyperledgerBridgeService()
    @State private var transactions: [BridgeTransaction] = []
    @State private var selectedDirection: BridgeDirection?
    @State private var selectedStatus: BridgeStatus?
    @State private var selectedTransaction: BridgeTransaction?

    var body: some View {
        NavigationView {
            VStack(spacing: 0) {
                // Filters
                ScrollView(.horizontal, showsIndicators: false) {
                    HStack(spacing: 12) {
                        FilterButton(title: "All", isSelected: selectedDirection == nil && selectedStatus == nil) {
                            selectedDirection = nil
                            selectedStatus = nil
                            loadHistory()
                        }

                        FilterButton(title: "To Fabric", isSelected: selectedDirection == .toFabric) {
                            selectedDirection = .toFabric
                            selectedStatus = nil
                            loadHistory()
                        }

                        FilterButton(title: "From Fabric", isSelected: selectedDirection == .fromFabric) {
                            selectedDirection = .fromFabric
                            selectedStatus = nil
                            loadHistory()
                        }

                        Divider()
                            .frame(height: 20)

                        FilterButton(title: "Completed", isSelected: selectedStatus == .completed) {
                            selectedDirection = nil
                            selectedStatus = .completed
                            loadHistory()
                        }

                        FilterButton(title: "Failed", isSelected: selectedStatus == .failed) {
                            selectedDirection = nil
                            selectedStatus = .failed
                            loadHistory()
                        }
                    }
                    .padding()
                }

                // Transaction List
                if service.isLoading {
                    Spacer()
                    ProgressView("Loading history...")
                    Spacer()
                } else if transactions.isEmpty {
                    Spacer()
                    VStack(spacing: 16) {
                        Image(systemName: "tray")
                            .font(.system(size: 60))
                            .foregroundColor(.gray)
                        Text("No Transactions Found")
                            .font(.title2)
                            .foregroundColor(.gray)
                        Text("Your bridge history will appear here")
                            .font(.subheadline)
                            .foregroundColor(.gray)
                    }
                    Spacer()
                } else {
                    ScrollView {
                        LazyVStack(spacing: 12) {
                            ForEach(transactions) { transaction in
                                BridgeTransactionCardView(transaction: transaction)
                                    .onTapGesture {
                                        selectedTransaction = transaction
                                    }
                            }
                        }
                        .padding()
                    }
                }
            }
            .navigationTitle("Bridge History")
            .navigationBarTitleDisplayMode(.inline)
            .sheet(item: $selectedTransaction) { transaction in
                BridgeTransactionDetailView(transaction: transaction)
            }
            .onAppear(perform: loadHistory)
        }
    }

    private func loadHistory() {
        Task {
            do {
                // Replace with actual wallet address
                let address = "0x1234567890123456789012345678901234567890"
                let response = try await service.getBridgeHistory(
                    address: address,
                    direction: selectedDirection,
                    status: selectedStatus
                )
                transactions = response.transactions
            } catch {
                print("Error loading history: \(error)")
            }
        }
    }
}

// MARK: - Filter Button
private struct FilterButton: View {
    let title: String
    let isSelected: Bool
    let action: () -> Void

    var body: some View {
        Button(action: action) {
            Text(title)
                .font(.subheadline)
                .fontWeight(.medium)
                .foregroundColor(isSelected ? .white : .blue)
                .padding(.horizontal, 16)
                .padding(.vertical, 8)
                .background(isSelected ? Color.blue : Color.blue.opacity(0.1))
                .cornerRadius(20)
        }
    }
}

// MARK: - Bridge Transaction Card View
private struct BridgeTransactionCardView: View {
    let transaction: BridgeTransaction

    var statusColor: Color {
        Color(hex: transaction.status.color) ?? .gray
    }

    var body: some View {
        VStack(alignment: .leading, spacing: 12) {
            // Header
            HStack {
                VStack(alignment: .leading, spacing: 4) {
                    Text(transaction.direction.displayName)
                        .font(.headline)
                    Text(transaction.amountDisplay)
                        .font(.title3)
                        .fontWeight(.bold)
                        .foregroundColor(.blue)
                }

                Spacer()

                Text(transaction.status.rawValue.replacingOccurrences(of: "_", with: " ").capitalized)
                    .font(.caption)
                    .fontWeight(.medium)
                    .foregroundColor(.white)
                    .padding(.horizontal, 10)
                    .padding(.vertical, 4)
                    .background(statusColor)
                    .cornerRadius(12)
            }

            Divider()

            // Transaction Details
            VStack(spacing: 8) {
                HStack {
                    Text("From:")
                        .font(.caption)
                        .foregroundColor(.gray)
                    Text(transaction.sourceChain)
                        .font(.subheadline)
                        .fontWeight(.medium)
                    Spacer()
                    Image(systemName: "arrow.right")
                        .foregroundColor(.gray)
                        .font(.caption)
                    Spacer()
                    Text("To:")
                        .font(.caption)
                        .foregroundColor(.gray)
                    Text(transaction.destinationChain)
                        .font(.subheadline)
                        .fontWeight(.medium)
                }

                HStack {
                    Text("Tx Hash:")
                        .font(.caption)
                        .foregroundColor(.gray)
                    Text(transaction.txHashDisplay)
                        .font(.system(.caption, design: .monospaced))
                    Spacer()
                }
            }

            // Progress Bar (if in progress)
            if transaction.isInProgress {
                VStack(alignment: .leading, spacing: 4) {
                    Text("Step \(transaction.status.stepNumber) of 5")
                        .font(.caption)
                        .foregroundColor(.gray)
                    ProgressView(value: Double(transaction.status.stepNumber), total: 5)
                        .progressViewStyle(LinearProgressViewStyle(tint: .blue))
                }
            }

            // Endorsements (if available)
            if let endorsements = transaction.endorsements, !endorsements.isEmpty {
                HStack {
                    Image(systemName: "checkmark.seal.fill")
                        .foregroundColor(.blue)
                        .font(.caption)
                    Text("\(transaction.validEndorsementCount)/\(transaction.endorsementCount) endorsements")
                        .font(.caption)
                        .foregroundColor(.gray)
                }
            }

            // Time
            HStack {
                Image(systemName: "clock")
                    .font(.caption)
                    .foregroundColor(.gray)
                Text(transaction.createdAt, style: .relative)
                    .font(.caption)
                    .foregroundColor(.gray)

                if transaction.isComplete {
                    Text("•")
                        .foregroundColor(.gray)
                    Text("Duration: \(transaction.duration)")
                        .font(.caption)
                        .foregroundColor(.gray)
                }
            }
        }
        .padding()
        .background(Color(.systemBackground))
        .cornerRadius(12)
        .shadow(color: Color.black.opacity(0.1), radius: 5, x: 0, y: 2)
    }
}

// MARK: - Bridge Transaction Detail View
private struct BridgeTransactionDetailView: View {
    let transaction: BridgeTransaction
    @Environment(\.presentationMode) var presentationMode

    var statusColor: Color {
        Color(hex: transaction.status.color) ?? .gray
    }

    var body: some View {
        NavigationView {
            ScrollView {
                VStack(alignment: .leading, spacing: 20) {
                    // Header
                    VStack(alignment: .leading, spacing: 12) {
                        Text(transaction.direction.displayName)
                            .font(.title2)
                            .fontWeight(.bold)

                        HStack {
                            Text(transaction.amountDisplay)
                                .font(.title)
                                .fontWeight(.bold)
                                .foregroundColor(.blue)
                            Spacer()
                            Text(transaction.status.rawValue.replacingOccurrences(of: "_", with: " ").capitalized)
                                .font(.subheadline)
                                .fontWeight(.medium)
                                .foregroundColor(.white)
                                .padding(.horizontal, 12)
                                .padding(.vertical, 6)
                                .background(statusColor)
                                .cornerRadius(12)
                        }
                    }
                    .padding()
                    .background(Color(.systemGray6))
                    .cornerRadius(12)

                    // Bridge Process Steps
                    VStack(alignment: .leading, spacing: 12) {
                        Text("Bridge Process")
                            .font(.headline)

                        VStack(spacing: 16) {
                            StepDetailView(step: 1, title: "Lock/Burn", status: transaction.status, txHash: transaction.lockTxHash ?? transaction.burnTxHash)
                            StepDetailView(step: 2, title: "Generate Proof", status: transaction.status, txHash: nil)
                            StepDetailView(step: 3, title: "Verify", status: transaction.status, txHash: nil)
                            StepDetailView(step: 4, title: "Complete", status: transaction.status, txHash: transaction.mintTxHash ?? transaction.unlockTxHash)
                        }
                        .padding()
                        .background(Color(.systemGray6))
                        .cornerRadius(12)
                    }

                    // Transaction Details
                    VStack(alignment: .leading, spacing: 12) {
                        Text("Transaction Details")
                            .font(.headline)

                        VStack(spacing: 12) {
                            DetailRow(label: "Tx Hash", value: transaction.txHash)
                            DetailRow(label: "Source Chain", value: transaction.sourceChain)
                            DetailRow(label: "Destination Chain", value: transaction.destinationChain)
                            DetailRow(label: "Sender", value: transaction.senderDisplay)
                            DetailRow(label: "Recipient", value: transaction.recipientDisplay)
                            DetailRow(label: "Created", value: transaction.createdAt.formatted())

                            if transaction.isComplete, let completed = transaction.completedAt {
                                DetailRow(label: "Completed", value: completed.formatted())
                                DetailRow(label: "Duration", value: transaction.duration)
                            }
                        }
                        .padding()
                        .background(Color(.systemGray6))
                        .cornerRadius(12)
                    }

                    // Endorsements
                    if let endorsements = transaction.endorsements, !endorsements.isEmpty {
                        VStack(alignment: .leading, spacing: 12) {
                            Text("Endorsements (\(transaction.validEndorsementCount)/\(transaction.endorsementCount))")
                                .font(.headline)

                            VStack(spacing: 8) {
                                ForEach(endorsements) { endorsement in
                                    HStack {
                                        Image(systemName: endorsement.statusIcon)
                                            .foregroundColor(Color(hex: endorsement.statusColor) ?? .gray)
                                        Text(endorsement.mspId)
                                            .font(.subheadline)
                                            .fontWeight(.medium)
                                        Spacer()
                                        Text(endorsement.timestamp, style: .relative)
                                            .font(.caption)
                                            .foregroundColor(.gray)
                                    }
                                    .padding()
                                    .background(Color(.systemGray6))
                                    .cornerRadius(8)
                                }
                            }
                        }
                    }

                    // Error Message
                    if let error = transaction.errorMessage {
                        VStack(alignment: .leading, spacing: 8) {
                            Text("Error")
                                .font(.headline)
                                .foregroundColor(.red)

                            Text(error)
                                .font(.subheadline)
                                .foregroundColor(.red)
                                .padding()
                                .background(Color.red.opacity(0.1))
                                .cornerRadius(8)
                        }
                    }
                }
                .padding()
            }
            .navigationTitle("Transaction Details")
            .navigationBarTitleDisplayMode(.inline)
            .toolbar {
                ToolbarItem(placement: .navigationBarTrailing) {
                    Button("Done") {
                        presentationMode.wrappedValue.dismiss()
                    }
                }
            }
        }
    }
}

// MARK: - Step Detail View
private struct StepDetailView: View {
    let step: Int
    let title: String
    let status: BridgeStatus
    let txHash: String?

    var isComplete: Bool {
        status.stepNumber >= step
    }

    var body: some View {
        HStack(spacing: 12) {
            ZStack {
                Circle()
                    .fill(isComplete ? Color.blue : Color.gray.opacity(0.3))
                    .frame(width: 32, height: 32)

                if isComplete {
                    Image(systemName: "checkmark")
                        .font(.subheadline)
                        .fontWeight(.bold)
                        .foregroundColor(.white)
                } else {
                    Text("\(step)")
                        .font(.subheadline)
                        .fontWeight(.bold)
                        .foregroundColor(.white)
                }
            }

            VStack(alignment: .leading, spacing: 4) {
                Text(title)
                    .font(.subheadline)
                    .fontWeight(.medium)

                if let hash = txHash {
                    Text(hash.prefix(16) + "...")
                        .font(.system(.caption, design: .monospaced))
                        .foregroundColor(.gray)
                }
            }

            Spacer()
        }
    }
}

// MARK: - Detail Row
private struct DetailRow: View {
    let label: String
    let value: String

    var body: some View {
        HStack {
            Text(label)
                .font(.subheadline)
                .foregroundColor(.gray)
            Spacer()
            Text(value)
                .font(.subheadline)
                .fontWeight(.medium)
        }
    }
}
