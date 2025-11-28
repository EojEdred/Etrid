//
//  MultiSigWalletDetailView.swift
//  EtridWallet
//
//  Detailed multi-sig wallet view with pending transactions
//

import SwiftUI

struct MultiSigWalletDetailView: View {
    let wallet: MultiSigWallet
    @StateObject private var service = MultiSigService.shared
    @State private var selectedTab = 0
    @State private var showProposeTransaction = false

    var body: some View {
        VStack(spacing: 0) {
            // Header
            walletHeader

            // Tabs
            Picker("View", selection: $selectedTab) {
                Text("Pending").tag(0)
                Text("Owners").tag(1)
                Text("History").tag(2)
            }
            .pickerStyle(.segmented)
            .padding()

            // Content
            TabView(selection: $selectedTab) {
                MultiSigPendingTransactionsTab(wallet: wallet).tag(0)
                OwnersTab(wallet: wallet).tag(1)
                HistoryTab(wallet: wallet).tag(2)
            }
            .tabViewStyle(.page(indexDisplayMode: .never))
        }
        .navigationTitle(wallet.name)
        .navigationBarTitleDisplayMode(.inline)
        .toolbar {
            ToolbarItem(placement: .navigationBarTrailing) {
                if wallet.isUserOwner {
                    Button(action: { showProposeTransaction = true }) {
                        Image(systemName: "plus")
                    }
                }
            }
        }
        .sheet(isPresented: $showProposeTransaction) {
            ProposeTransactionView(wallet: wallet)
        }
    }

    private var walletHeader: some View {
        VStack(spacing: 12) {
            // Address
            HStack {
                Text(wallet.address)
                    .font(.caption)
                    .lineLimit(1)
                Button(action: { copyAddress() }) {
                    Image(systemName: "doc.on.doc")
                        .font(.caption)
                }
            }

            // Stats
            HStack(spacing: 24) {
                StatItem(value: wallet.balance, label: "Balance")
                StatItem(value: wallet.thresholdDescription, label: "Threshold")
                StatItem(value: "\(wallet.pendingTransactionCount)", label: "Pending")
            }
        }
        .padding()
        .background(Color(.systemGray6))
    }

    private func copyAddress() {
        UIPasteboard.general.string = wallet.address
    }
}

struct MultiSigPendingTransactionsTab: View {
    let wallet: MultiSigWallet
    @StateObject private var service = MultiSigService.shared

    var body: some View {
        List {
            if pendingTransactions.isEmpty {
                ContentUnavailableView(
                    "No Pending Transactions",
                    systemImage: "tray",
                    description: Text("Propose a transaction to get started")
                )
            } else {
                ForEach(pendingTransactions) { transaction in
                    NavigationLink {
                        MultiSigPendingTransactionDetailView(transaction: transaction, wallet: wallet)
                    } label: {
                        MultiSigPendingTransactionRow(transaction: transaction)
                    }
                }
            }
        }
    }

    private var pendingTransactions: [MultiSigPendingTransaction] {
        service.getPendingTransactions(walletId: wallet.id)
    }
}

struct MultiSigPendingTransactionRow: View {
    let transaction: MultiSigPendingTransaction

    var body: some View {
        VStack(alignment: .leading, spacing: 8) {
            // To and Value
            HStack {
                Text("To: \(transaction.to)")
                    .font(.headline)
                    .lineLimit(1)

                Spacer()

                Text(transaction.valueInEth + " ETH")
                    .font(.subheadline)
                    .foregroundColor(.secondary)
            }

            // Signature Progress
            VStack(alignment: .leading, spacing: 4) {
                HStack {
                    Text("\(transaction.signatures.count) of \(transaction.requiredSignatures) signatures")
                        .font(.caption)
                        .foregroundColor(.secondary)

                    Spacer()

                    if transaction.isReadyToExecute {
                        Text("Ready to Execute")
                            .font(.caption2)
                            .padding(.horizontal, 6)
                            .padding(.vertical, 2)
                            .background(Color.green.opacity(0.2))
                            .foregroundColor(.green)
                            .cornerRadius(4)
                    }
                }

                ProgressView(value: transaction.signatureProgress)
            }

            // Proposer
            HStack {
                Image(systemName: "person.circle")
                    .font(.caption2)
                Text("Proposed by \(transaction.proposerName ?? "Unknown")")
                    .font(.caption2)
                    .foregroundColor(.secondary)
            }
        }
        .padding(.vertical, 4)
    }
}

struct MultiSigPendingTransactionDetailView: View {
    let transaction: MultiSigPendingTransaction
    let wallet: MultiSigWallet
    @StateObject private var service = MultiSigService.shared
    @Environment(\.dismiss) private var dismiss

    @State private var showSignAlert = false
    @State private var showExecuteAlert = false
    @State private var isSigning = false
    @State private var isExecuting = false

    var body: some View {
        List {
            Section("Transaction Details") {
                DetailRow(title: "To", value: transaction.to)
                DetailRow(title: "Value", value: transaction.valueInEth + " ETH")
                DetailRow(title: "Operation", value: transaction.operation.rawValue)
                DetailRow(title: "Nonce", value: "\(transaction.nonce)")
            }

            Section("Proposer") {
                DetailRow(title: "Address", value: transaction.proposer)
                DetailRow(title: "Proposed", value: transaction.createdAt.formatted(date: .abbreviated, time: .shortened))
            }

            Section("Signatures (\(transaction.signatures.count)/\(transaction.requiredSignatures))") {
                ForEach(transaction.signatures) { signature in
                    VStack(alignment: .leading, spacing: 4) {
                        Text(signature.signerName ?? signature.signer)
                            .font(.subheadline)
                        Text(signature.signedAt.formatted(date: .abbreviated, time: .shortened))
                            .font(.caption)
                            .foregroundColor(.secondary)
                    }
                }
            }

            Section {
                if !hasUserSigned && wallet.isUserOwner {
                    Button("Sign Transaction") {
                        showSignAlert = true
                    }
                    .disabled(isSigning)
                }

                if transaction.isReadyToExecute && wallet.isUserOwner {
                    Button("Execute Transaction") {
                        showExecuteAlert = true
                    }
                    .disabled(isExecuting)
                    .foregroundColor(.green)
                }

                if wallet.isUserOwner {
                    Button("Reject Transaction", role: .destructive) {
                        rejectTransaction()
                    }
                }
            }
        }
        .navigationTitle("Transaction #\(transaction.nonce)")
        .navigationBarTitleDisplayMode(.inline)
        .alert("Sign Transaction", isPresented: $showSignAlert) {
            Button("Cancel", role: .cancel) { }
            Button("Sign") {
                signTransaction()
            }
        } message: {
            Text("Are you sure you want to sign this transaction?")
        }
        .alert("Execute Transaction", isPresented: $showExecuteAlert) {
            Button("Cancel", role: .cancel) { }
            Button("Execute") {
                executeTransaction()
            }
        } message: {
            Text("This will execute the transaction on-chain. This action cannot be undone.")
        }
    }

    private var hasUserSigned: Bool {
        // TODO: Check if current user has signed
        false
    }

    private func signTransaction() {
        isSigning = true
        Task {
            do {
                try await service.signTransaction(transactionId: transaction.id, walletId: wallet.id)
                await MainActor.run {
                    isSigning = false
                }
            } catch {
                print("Sign error: \(error)")
                isSigning = false
            }
        }
    }

    private func executeTransaction() {
        isExecuting = true
        Task {
            do {
                try await service.executeTransaction(transactionId: transaction.id, walletId: wallet.id)
                await MainActor.run {
                    dismiss()
                }
            } catch {
                print("Execute error: \(error)")
                isExecuting = false
            }
        }
    }

    private func rejectTransaction() {
        Task {
            try? await service.rejectTransaction(transactionId: transaction.id, walletId: wallet.id)
            await MainActor.run {
                dismiss()
            }
        }
    }
}

struct OwnersTab: View {
    let wallet: MultiSigWallet
    @StateObject private var service = MultiSigService.shared
    @State private var owners: [MultiSigOwner] = []

    var body: some View {
        List {
            ForEach(owners) { owner in
                OwnerRow(owner: owner)
            }
        }
        .task {
            do {
                owners = try await service.getOwners(walletId: wallet.id)
            } catch {
                print("Error loading owners: \(error)")
            }
        }
    }
}

struct OwnerRow: View {
    let owner: MultiSigOwner

    var body: some View {
        VStack(alignment: .leading, spacing: 4) {
            Text(owner.displayName)
                .font(.headline)

            Text(owner.address)
                .font(.caption)
                .foregroundColor(.secondary)

            HStack(spacing: 12) {
                Label("\(owner.transactionsProposed)", systemImage: "doc.badge.plus")
                Label("\(owner.transactionsSigned)", systemImage: "signature")
            }
            .font(.caption2)
            .foregroundColor(.secondary)
        }
    }
}

struct HistoryTab: View {
    let wallet: MultiSigWallet

    var body: some View {
        List {
            ContentUnavailableView(
                "No History",
                systemImage: "clock",
                description: Text("Executed transactions will appear here")
            )
        }
    }
}

// MARK: - Helper Views

private struct StatItem: View {
    let value: String
    let label: String

    var body: some View {
        VStack(spacing: 4) {
            Text(value)
                .font(.headline)
            Text(label)
                .font(.caption)
                .foregroundColor(.secondary)
        }
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
        }
    }
}

struct ProposeTransactionView: View {
    let wallet: MultiSigWallet
    @StateObject private var service = MultiSigService.shared
    @Environment(\.dismiss) private var dismiss

    @State private var toAddress = ""
    @State private var valueEth = ""
    @State private var description = ""
    @State private var operation = MultiSigPendingTransaction.OperationType.call

    @State private var isProposing = false

    var body: some View {
        NavigationView {
            Form {
                Section("Transaction Details") {
                    TextField("To Address", text: $toAddress)
                        .autocapitalization(.none)
                    TextField("Value (ETH)", text: $valueEth)
                        .keyboardType(.decimalPad)
                    Picker("Operation", selection: $operation) {
                        Text("Call").tag(MultiSigPendingTransaction.OperationType.call)
                        Text("Delegate Call").tag(MultiSigPendingTransaction.OperationType.delegateCall)
                    }
                }

                Section("Description") {
                    TextField("Optional description", text: $description, axis: .vertical)
                        .lineLimit(3...6)
                }
            }
            .navigationTitle("Propose Transaction")
            .navigationBarTitleDisplayMode(.inline)
            .toolbar {
                ToolbarItem(placement: .cancellationAction) {
                    Button("Cancel") {
                        dismiss()
                    }
                }
                ToolbarItem(placement: .confirmationAction) {
                    Button("Propose") {
                        proposeTransaction()
                    }
                    .disabled(!isValid || isProposing)
                }
            }
        }
    }

    private var isValid: Bool {
        !toAddress.isEmpty && !valueEth.isEmpty
    }

    private func proposeTransaction() {
        guard let ethValue = Double(valueEth) else { return }
        let weiValue = String(Int(ethValue * 1_000_000_000_000_000_000))

        let params = ProposeTransactionParams(
            walletId: wallet.id,
            to: toAddress,
            value: weiValue,
            data: nil,
            operation: operation,
            description: description.isEmpty ? nil : description,
            expiryHours: nil
        )

        isProposing = true

        Task {
            do {
                _ = try await service.proposeTransaction(params)
                await MainActor.run {
                    dismiss()
                }
            } catch {
                print("Propose error: \(error)")
                isProposing = false
            }
        }
    }
}
