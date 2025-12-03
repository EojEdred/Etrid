//
//  MultisigWalletView.swift
//  EtridWallet
//
//  Production-ready multi-signature wallet management
//

import SwiftUI

struct MultisigWalletView: View {
    @StateObject private var viewModel = MultisigWalletViewModel()
    @Environment(\.dismiss) private var dismiss
    @State private var showCreateWallet = false
    @State private var showToast = false
    @State private var toastMessage = ""
    @State private var toastType: ToastType = .success

    var body: some View {
        NavigationView {
            ZStack {
                ScrollView {
                    VStack(spacing: 20) {
                        // Error Banner
                        if let error = viewModel.error {
                            MultisigErrorBanner(message: error) {
                                viewModel.clearError()
                            }
                        }

                        // Loading State
                        if viewModel.isLoading && viewModel.wallets.isEmpty {
                            ProgressView("Loading multisig wallets...")
                                .padding(.top, 50)
                        } else {
                            // Your Multisig Wallets
                            yourWallets

                            // Pending Transactions
                            pendingTransactions
                        }
                    }
                    .padding()
                }
                .refreshable {
                    await viewModel.loadData()
                }

                // Toast Overlay
                if showToast {
                    VStack {
                        Spacer()
                        ToastView(message: toastMessage, type: toastType)
                            .padding()
                            .transition(.move(edge: .bottom).combined(with: .opacity))
                    }
                    .animation(.spring(), value: showToast)
                }
            }
            .navigationTitle("Multisig Wallets")
            .navigationBarTitleDisplayMode(.large)
            .toolbar {
                ToolbarItem(placement: .navigationBarLeading) {
                    Button(action: { dismiss() }) {
                        Image(systemName: "chevron.left")
                            .foregroundColor(.blue)
                    }
                }
                ToolbarItem(placement: .navigationBarTrailing) {
                    Button(action: { showCreateWallet = true }) {
                        Image(systemName: "plus")
                            .font(.title2)
                            .foregroundColor(.blue)
                    }
                }
            }
            .sheet(isPresented: $showCreateWallet) {
                CreateMultisigWalletView(onCreated: {
                    Task {
                        await viewModel.loadData()
                    }
                })
            }
            .task {
                await viewModel.loadData()
            }
            .onChange(of: viewModel.successMessage) { message in
                if let message = message {
                    showToastMessage(message, type: .success)
                    viewModel.successMessage = nil
                }
            }
        }
    }

    // MARK: - Your Wallets
    private var yourWallets: some View {
        VStack(alignment: .leading, spacing: 12) {
            Text("Your Multisig Wallets")
                .font(.headline)
                .fontWeight(.bold)

            if viewModel.wallets.isEmpty {
                MultisigEmptyStateView(
                    icon: "person.2.circle",
                    title: "No Multisig Wallets",
                    message: "Create a multisig wallet to get started"
                )
                .padding(.vertical, 30)
            } else {
                ForEach(viewModel.wallets) { wallet in
                    MultisigWalletCard(
                        wallet: wallet,
                        pendingCount: viewModel.getPendingCount(for: wallet.id)
                    )
                }
            }
        }
    }

    // MARK: - Pending Transactions
    private var pendingTransactions: some View {
        VStack(alignment: .leading, spacing: 12) {
            Text("Pending Transactions")
                .font(.headline)
                .fontWeight(.bold)

            if viewModel.pendingTransactions.isEmpty {
                MultisigEmptyStateView(
                    icon: "checkmark.circle",
                    title: "No Pending Transactions",
                    message: "All transactions are up to date"
                )
                .padding(.vertical, 30)
            } else {
                ForEach(viewModel.pendingTransactions) { tx in
                    PendingTransactionCard(
                        transaction: tx,
                        walletName: viewModel.getWalletName(for: tx.walletId),
                        isProcessing: viewModel.processingTransactionId == tx.id,
                        onSign: {
                            Task {
                                await viewModel.signTransaction(tx.id)
                            }
                        },
                        onReject: {
                            Task {
                                await viewModel.rejectTransaction(tx.id)
                            }
                        }
                    )
                }
            }
        }
    }

    // MARK: - Helper Methods
    private func showToastMessage(_ message: String, type: ToastType) {
        toastMessage = message
        toastType = type
        withAnimation {
            showToast = true
        }

        DispatchQueue.main.asyncAfter(deadline: .now() + 3) {
            withAnimation {
                showToast = false
            }
        }
    }
}

// MARK: - Error Banner
struct MultisigErrorBanner: View {
    let message: String
    let onDismiss: () -> Void

    var body: some View {
        HStack {
            Image(systemName: "exclamationmark.triangle.fill")
                .foregroundColor(.white)

            Text(message)
                .font(.subheadline)
                .foregroundColor(.white)
                .multilineTextAlignment(.leading)

            Spacer()

            Button(action: onDismiss) {
                Image(systemName: "xmark.circle.fill")
                    .foregroundColor(.white.opacity(0.8))
            }
        }
        .padding()
        .background(Color.red)
        .cornerRadius(12)
    }
}

// MARK: - Empty State View
struct MultisigEmptyStateView: View {
    let icon: String
    let title: String
    let message: String

    var body: some View {
        VStack(spacing: 12) {
            Image(systemName: icon)
                .font(.system(size: 50))
                .foregroundColor(.secondary)

            Text(title)
                .font(.headline)
                .foregroundColor(.secondary)

            Text(message)
                .font(.subheadline)
                .foregroundColor(.secondary)
                .multilineTextAlignment(.center)
        }
        .frame(maxWidth: .infinity)
    }
}

// MARK: - Toast View
enum ToastType {
    case success
    case error
    case info

    var color: Color {
        switch self {
        case .success: return .green
        case .error: return .red
        case .info: return .blue
        }
    }

    var icon: String {
        switch self {
        case .success: return "checkmark.circle.fill"
        case .error: return "xmark.circle.fill"
        case .info: return "info.circle.fill"
        }
    }
}

struct ToastView: View {
    let message: String
    let type: ToastType

    var body: some View {
        HStack(spacing: 12) {
            Image(systemName: type.icon)
                .foregroundColor(.white)
                .font(.title3)

            Text(message)
                .font(.subheadline)
                .foregroundColor(.white)
                .multilineTextAlignment(.leading)
        }
        .padding()
        .background(type.color)
        .cornerRadius(12)
        .shadow(radius: 10)
    }
}

// MARK: - Multisig Wallet Card
struct MultisigWalletCard: View {
    let wallet: MultisigWallet
    let pendingCount: Int

    var body: some View {
        VStack(alignment: .leading, spacing: 12) {
            // Header
            HStack {
                Text(wallet.name)
                    .font(.title3)
                    .fontWeight(.bold)

                Spacer()

                if pendingCount > 0 {
                    Text("\(pendingCount) pending")
                        .font(.caption)
                        .fontWeight(.semibold)
                        .padding(.horizontal, 10)
                        .padding(.vertical, 4)
                        .background(Color.orange.opacity(0.2))
                        .foregroundColor(.orange)
                        .cornerRadius(12)
                }
            }

            // Balance
            Text("$\(wallet.balance.formatted())")
                .font(.system(size: 28, weight: .bold))

            // Address
            HStack {
                Text(wallet.address.prefix(10) + "..." + wallet.address.suffix(6))
                    .font(.caption)
                    .foregroundColor(.secondary)
                    .fontDesign(.monospaced)

                Button(action: {
                    UIPasteboard.general.string = wallet.address
                }) {
                    Image(systemName: "doc.on.doc")
                        .font(.caption)
                        .foregroundColor(.blue)
                }
            }

            // Info
            HStack {
                Image(systemName: "person.2")
                    .foregroundColor(.secondary)
                Text("\(wallet.threshold)/\(wallet.totalSigners) signatures required")
                    .font(.subheadline)
                    .foregroundColor(.secondary)
            }

            // Signers
            HStack(spacing: -8) {
                ForEach(0..<min(wallet.totalSigners, 5), id: \.self) { _ in
                    Circle()
                        .fill(
                            LinearGradient(
                                gradient: Gradient(colors: [Color.blue, Color.purple]),
                                startPoint: .topLeading,
                                endPoint: .bottomTrailing
                            )
                        )
                        .frame(width: 32, height: 32)
                        .overlay(
                            Circle()
                                .stroke(Color(.systemBackground), lineWidth: 2)
                        )
                }

                if wallet.totalSigners > 5 {
                    Text("+\(wallet.totalSigners - 5)")
                        .font(.caption)
                        .fontWeight(.semibold)
                        .foregroundColor(.secondary)
                        .padding(.leading, 8)
                }
            }
        }
        .padding()
        .background(Color(.systemGray6))
        .cornerRadius(16)
        .overlay(
            RoundedRectangle(cornerRadius: 16)
                .stroke(Color(.systemGray4), lineWidth: 1)
        )
    }
}

// MARK: - Pending Transaction Card
struct PendingTransactionCard: View {
    let transaction: MultisigTransaction
    let walletName: String
    let isProcessing: Bool
    let onSign: () -> Void
    let onReject: () -> Void

    var body: some View {
        VStack(alignment: .leading, spacing: 12) {
            // Header
            HStack {
                VStack(alignment: .leading, spacing: 4) {
                    Text(transactionTypeDisplay)
                        .font(.body)
                        .fontWeight(.bold)

                    Text(walletName)
                        .font(.caption)
                        .foregroundColor(.secondary)
                }

                Spacer()

                // Status Badge
                if transaction.isReadyToExecute {
                    Text("Ready")
                        .font(.caption2)
                        .fontWeight(.semibold)
                        .padding(.horizontal, 8)
                        .padding(.vertical, 4)
                        .background(Color.green.opacity(0.2))
                        .foregroundColor(.green)
                        .cornerRadius(8)
                }
            }

            // Amount
            if transaction.amount > 0 {
                Text("$\(transaction.amount.formatted())")
                    .font(.title2)
                    .fontWeight(.bold)
            }

            // To Address
            HStack {
                Text("To:")
                    .font(.caption)
                    .foregroundColor(.secondary)

                Text(formatAddress(transaction.to))
                    .font(.caption)
                    .fontWeight(.medium)
                    .foregroundColor(.primary)
                    .fontDesign(.monospaced)
            }

            // Signature Progress
            VStack(alignment: .leading, spacing: 6) {
                GeometryReader { geometry in
                    ZStack(alignment: .leading) {
                        Rectangle()
                            .fill(Color(.systemGray4))
                            .frame(height: 8)
                            .cornerRadius(4)

                        Rectangle()
                            .fill(transaction.isReadyToExecute ? Color.green : Color.blue)
                            .frame(
                                width: geometry.size.width * CGFloat(transaction.signatureCount) / CGFloat(transaction.threshold),
                                height: 8
                            )
                            .cornerRadius(4)
                            .animation(.easeInOut, value: transaction.signatureCount)
                    }
                }
                .frame(height: 8)

                HStack {
                    Text("\(transaction.signatureCount)/\(transaction.threshold) signatures")
                        .font(.caption)
                        .foregroundColor(.secondary)

                    Spacer()

                    Text(formatDate(transaction.createdAt))
                        .font(.caption)
                        .foregroundColor(.secondary)
                }
            }

            // Signatures List
            if !transaction.signatures.isEmpty {
                VStack(alignment: .leading, spacing: 4) {
                    Text("Signatures:")
                        .font(.caption)
                        .foregroundColor(.secondary)

                    ForEach(transaction.signatures) { signature in
                        HStack(spacing: 4) {
                            Image(systemName: "checkmark.circle.fill")
                                .font(.caption2)
                                .foregroundColor(.green)
                            Text(formatAddress(signature.signer))
                                .font(.caption2)
                                .fontDesign(.monospaced)
                                .foregroundColor(.secondary)
                        }
                    }
                }
                .padding(.top, 4)
            }

            // Action Buttons
            if !isProcessing {
                HStack(spacing: 12) {
                    Button(action: onSign) {
                        HStack {
                            Image(systemName: "checkmark")
                            Text("Sign")
                        }
                        .font(.subheadline)
                        .fontWeight(.semibold)
                        .foregroundColor(.white)
                        .frame(maxWidth: .infinity)
                        .padding(.vertical, 12)
                        .background(Color.green)
                        .cornerRadius(8)
                    }

                    Button(action: onReject) {
                        HStack {
                            Image(systemName: "xmark")
                            Text("Reject")
                        }
                        .font(.subheadline)
                        .fontWeight(.semibold)
                        .foregroundColor(.white)
                        .frame(maxWidth: .infinity)
                        .padding(.vertical, 12)
                        .background(Color.red)
                        .cornerRadius(8)
                    }
                }
            } else {
                HStack {
                    ProgressView()
                        .progressViewStyle(CircularProgressViewStyle())
                    Text("Processing...")
                        .font(.subheadline)
                        .foregroundColor(.secondary)
                }
                .frame(maxWidth: .infinity)
                .padding(.vertical, 12)
            }
        }
        .padding()
        .background(Color(.systemGray6))
        .cornerRadius(16)
        .overlay(
            RoundedRectangle(cornerRadius: 16)
                .stroke(Color(.systemGray4), lineWidth: 1)
        )
        .opacity(isProcessing ? 0.7 : 1.0)
    }

    private var transactionTypeDisplay: String {
        switch transaction.type {
        case .transfer:
            return "Transfer"
        case .contractCall:
            return "Contract Call"
        case .addSigner:
            return "Add Signer"
        case .removeSigner:
            return "Remove Signer"
        case .changeThreshold:
            return "Change Threshold"
        }
    }

    private func formatAddress(_ address: String) -> String {
        if address.count > 20 {
            return address.prefix(10) + "..." + address.suffix(6)
        }
        return address
    }

    private func formatDate(_ date: Date) -> String {
        let formatter = RelativeDateTimeFormatter()
        formatter.unitsStyle = .abbreviated
        return formatter.localizedString(for: date, relativeTo: Date())
    }
}

// MARK: - Create Multisig Wallet View
struct CreateMultisigWalletView: View {
    @Environment(\.dismiss) private var dismiss
    @StateObject private var viewModel = CreateMultisigViewModel()
    let onCreated: () -> Void

    var body: some View {
        NavigationView {
            Form {
                Section(header: Text("Wallet Details")) {
                    TextField("Wallet Name", text: $viewModel.walletName)

                    Stepper("Required Signatures: \(viewModel.threshold)", value: $viewModel.threshold, in: 1...viewModel.signers.filter({ !$0.isEmpty }).count)
                }

                Section(header: Text("Signers")) {
                    ForEach(viewModel.signers.indices, id: \.self) { index in
                        TextField("Signer \(index + 1) Address", text: $viewModel.signers[index])
                            .autocapitalization(.none)
                            .disableAutocorrection(true)
                            .fontDesign(.monospaced)
                    }

                    Button(action: {
                        viewModel.signers.append("")
                    }) {
                        HStack {
                            Image(systemName: "plus.circle.fill")
                            Text("Add Signer")
                        }
                    }
                }

                Section {
                    Text("A multisig wallet requires multiple signatures to execute transactions. This provides enhanced security for your funds.")
                        .font(.caption)
                        .foregroundColor(.secondary)
                }

                if let error = viewModel.error {
                    Section {
                        Text(error)
                            .font(.caption)
                            .foregroundColor(.red)
                    }
                }
            }
            .navigationTitle("Create Multisig Wallet")
            .navigationBarTitleDisplayMode(.inline)
            .toolbar {
                ToolbarItem(placement: .navigationBarLeading) {
                    Button("Cancel") { dismiss() }
                        .disabled(viewModel.isCreating)
                }
                ToolbarItem(placement: .navigationBarTrailing) {
                    if viewModel.isCreating {
                        ProgressView()
                    } else {
                        Button("Create") {
                            Task {
                                if await viewModel.createWallet() {
                                    onCreated()
                                    dismiss()
                                }
                            }
                        }
                        .disabled(!viewModel.isValid)
                    }
                }
            }
        }
    }
}

// MARK: - Create Multisig View Model
class CreateMultisigViewModel: ObservableObject {
    @Published var walletName = ""
    @Published var threshold = 2
    @Published var signers: [String] = ["", ""]
    @Published var isCreating = false
    @Published var error: String?

    var isValid: Bool {
        !walletName.isEmpty &&
        signers.filter({ !$0.isEmpty }).count >= 2 &&
        threshold >= 1 &&
        threshold <= signers.filter({ !$0.isEmpty }).count
    }

    func createWallet() async -> Bool {
        guard isValid else { return false }

        isCreating = true
        error = nil

        do {
            // Get current user address (assumes WalletService has current address)
            // TODO: Replace with actual current address retrieval
            let currentAddress = "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY"

            let validSigners = signers.filter({ !$0.isEmpty })

            let _ = try await MultisigService.shared.createWallet(
                name: walletName,
                signers: validSigners,
                threshold: threshold,
                creator: currentAddress
            )

            isCreating = false
            return true
        } catch let multisigError as MultisigError {
            self.error = multisigError.localizedDescription
            isCreating = false
            return false
        } catch {
            self.error = error.localizedDescription
            isCreating = false
            return false
        }
    }
}

// MARK: - Multisig Wallet View Model
class MultisigWalletViewModel: ObservableObject {
    @Published var wallets: [MultisigWallet] = []
    @Published var pendingTransactions: [MultisigTransaction] = []
    @Published var isLoading = false
    @Published var error: String?
    @Published var processingTransactionId: String?
    @Published var successMessage: String?

    private let multisigService = MultisigService.shared

    // Get current user address - TODO: Replace with actual wallet service
    private var currentAddress: String {
        // This should come from WalletService or KeychainService
        return "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY"
    }

    // MARK: - Data Loading
    @MainActor
    func loadData() async {
        isLoading = true
        error = nil

        do {
            async let walletsTask = multisigService.getWallets(for: currentAddress)
            async let transactionsTask = multisigService.getAllPendingTransactions(for: currentAddress)

            let (loadedWallets, loadedTransactions) = try await (walletsTask, transactionsTask)

            wallets = loadedWallets
            pendingTransactions = loadedTransactions
        } catch let multisigError as MultisigError {
            error = multisigError.localizedDescription
        } catch let caughtError {
            self.error = "Failed to load data: \(caughtError.localizedDescription)"
        }

        isLoading = false
    }

    // MARK: - Transaction Actions
    @MainActor
    func signTransaction(_ id: String) async {
        processingTransactionId = id
        error = nil

        do {
            let _ = try await multisigService.signTransaction(
                transactionId: id,
                signer: currentAddress
            )

            successMessage = "Transaction signed successfully"

            // Auto-refresh after signing
            await loadData()
        } catch let multisigError as MultisigError {
            error = multisigError.localizedDescription
        } catch let caughtError {
            self.error = "Failed to sign transaction: \(caughtError.localizedDescription)"
        }

        processingTransactionId = nil
    }

    @MainActor
    func rejectTransaction(_ id: String) async {
        processingTransactionId = id
        error = nil

        do {
            try await multisigService.rejectTransaction(
                transactionId: id,
                signer: currentAddress
            )

            successMessage = "Transaction rejected"

            // Auto-refresh after rejection
            await loadData()
        } catch let multisigError as MultisigError {
            error = multisigError.localizedDescription
        } catch let caughtError {
            self.error = "Failed to reject transaction: \(caughtError.localizedDescription)"
        }

        processingTransactionId = nil
    }

    // MARK: - Helper Methods
    func getPendingCount(for walletId: String) -> Int {
        pendingTransactions.filter { $0.walletId == walletId }.count
    }

    func getWalletName(for walletId: String) -> String {
        wallets.first(where: { $0.id == walletId })?.name ?? "Unknown Wallet"
    }

    func clearError() {
        error = nil
    }
}

// MARK: - Preview
struct MultisigWalletView_Previews: PreviewProvider {
    static var previews: some View {
        MultisigWalletView()
    }
}
