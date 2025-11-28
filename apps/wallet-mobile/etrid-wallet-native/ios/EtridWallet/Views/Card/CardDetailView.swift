//
//  CardDetailView.swift
//  EtridWallet
//
//  Comprehensive card details with controls and security features
//

import SwiftUI

@MainActor
class CardDetailViewModel: ObservableObject {
    @Published var card: CryptoCard
    @Published var transactions: [CardTransaction] = []
    @Published var controls: CardControls?
    @Published var isLoading = false
    @Published var showCardNumber = false
    @Published var showCVV = false
    @Published var errorMessage: String?
    @Published var showPINChange = false
    @Published var showAddCollateral = false
    @Published var showWithdrawCollateral = false
    @Published var showControls = false

    private let cardService = CardService.shared

    init(card: CryptoCard) {
        self.card = card
    }

    func loadData() async {
        isLoading = true

        do {
            transactions = try await cardService.getTransactions(for: card.id)
            controls = try await cardService.getControls(for: card.id)
        } catch {
            errorMessage = error.localizedDescription
        }

        isLoading = false
    }

    func freezeCard() async {
        do {
            card = try await cardService.freezeCard(cardId: card.id)
        } catch {
            errorMessage = error.localizedDescription
        }
    }

    func unfreezeCard() async {
        do {
            card = try await cardService.unfreezeCard(cardId: card.id)
        } catch {
            errorMessage = error.localizedDescription
        }
    }

    func blockCard() async {
        do {
            card = try await cardService.blockCard(cardId: card.id)
        } catch {
            errorMessage = error.localizedDescription
        }
    }

    func addToAppleWallet() async {
        isLoading = true

        do {
            _ = try await cardService.addToAppleWallet(cardId: card.id)
            // Success feedback
        } catch {
            errorMessage = error.localizedDescription
        }

        isLoading = false
    }
}

struct CardDetailView: View {
    @StateObject private var viewModel: CardDetailViewModel
    @Environment(\.dismiss) var dismiss
    let onUpdate: () -> Void

    init(card: CryptoCard, onUpdate: @escaping () -> Void) {
        _viewModel = StateObject(wrappedValue: CardDetailViewModel(card: card))
        self.onUpdate = onUpdate
    }

    var body: some View {
        NavigationView {
            ZStack {
                // Background
                LinearGradient(
                    gradient: Gradient(colors: [Color(hex: "0F172A"), Color(hex: "1E293B")]),
                    startPoint: .top,
                    endPoint: .bottom
                )
                .ignoresSafeArea()

                ScrollView {
                    VStack(spacing: 24) {
                        // Virtual card
                        VirtualCardView(card: viewModel.card)
                            .padding(.top, 20)

                        // Card information
                        CardInformationSection(
                            card: viewModel.card,
                            showCardNumber: $viewModel.showCardNumber,
                            showCVV: $viewModel.showCVV
                        )
                        .padding(.horizontal)

                        // Security controls
                        SecurityControlsSection(
                            card: viewModel.card,
                            onFreeze: {
                                Task {
                                    if viewModel.card.isFrozen {
                                        await viewModel.unfreezeCard()
                                    } else {
                                        await viewModel.freezeCard()
                                    }
                                    onUpdate()
                                }
                            },
                            onBlock: {
                                Task {
                                    await viewModel.blockCard()
                                    onUpdate()
                                }
                            },
                            onPINChange: {
                                viewModel.showPINChange = true
                            }
                        )
                        .padding(.horizontal)

                        // Collateral management
                        CollateralManagementSection(
                            collateral: viewModel.card.collateral,
                            onAdd: {
                                viewModel.showAddCollateral = true
                            },
                            onWithdraw: {
                                viewModel.showWithdrawCollateral = true
                            }
                        )
                        .padding(.horizontal)

                        // Spending controls
                        SpendingControlsSection(
                            limits: viewModel.card.limits,
                            onUpdate: {
                                viewModel.showControls = true
                            }
                        )
                        .padding(.horizontal)

                        // Apple Wallet
                        AppleWalletSection(
                            onAdd: {
                                Task {
                                    await viewModel.addToAppleWallet()
                                }
                            }
                        )
                        .padding(.horizontal)

                        // Transaction history
                        if !viewModel.transactions.isEmpty {
                            TransactionHistorySection(
                                transactions: viewModel.transactions
                            )
                            .padding(.horizontal)
                        }
                    }
                    .padding(.bottom, 40)
                }
                .refreshable {
                    await viewModel.loadData()
                }
            }
            .navigationTitle("Card Details")
            .navigationBarTitleDisplayMode(.inline)
            .toolbar {
                ToolbarItem(placement: .navigationBarLeading) {
                    Button("Done") {
                        onUpdate()
                        dismiss()
                    }
                    .foregroundColor(.white)
                }
            }
        }
        .task {
            await viewModel.loadData()
        }
        .sheet(isPresented: $viewModel.showPINChange) {
            PINChangeView(cardId: viewModel.card.id)
        }
        .sheet(isPresented: $viewModel.showAddCollateral) {
            AddCollateralView(cardId: viewModel.card.id) {
                Task {
                    await viewModel.loadData()
                    onUpdate()
                }
            }
        }
        .sheet(isPresented: $viewModel.showWithdrawCollateral) {
            WithdrawCollateralView(cardId: viewModel.card.id) {
                Task {
                    await viewModel.loadData()
                    onUpdate()
                }
            }
        }
        .sheet(isPresented: $viewModel.showControls) {
            if let controls = viewModel.controls {
                CardControlsView(cardId: viewModel.card.id, controls: controls) {
                    Task {
                        await viewModel.loadData()
                    }
                }
            }
        }
        .alert("Error", isPresented: .constant(viewModel.errorMessage != nil)) {
            Button("OK") {
                viewModel.errorMessage = nil
            }
        } message: {
            Text(viewModel.errorMessage ?? "")
        }
    }
}

// MARK: - Card Information

struct CardInformationSection: View {
    let card: CryptoCard
    @Binding var showCardNumber: Bool
    @Binding var showCVV: Bool

    var body: some View {
        VStack(spacing: 12) {
            Text("Card Information")
                .font(.system(size: 18, weight: .bold))
                .foregroundColor(.white)
                .frame(maxWidth: .infinity, alignment: .leading)

            VStack(spacing: 16) {
                // Card number
                HStack {
                    VStack(alignment: .leading, spacing: 4) {
                        Text("Card Number")
                            .font(.system(size: 12))
                            .foregroundColor(.white.opacity(0.6))

                        Text(showCardNumber ? card.cardNumber : card.maskedCardNumber)
                            .font(.system(size: 16, weight: .semibold, design: .monospaced))
                            .foregroundColor(.white)
                    }

                    Spacer()

                    Button(action: { showCardNumber.toggle() }) {
                        Image(systemName: showCardNumber ? "eye.slash.fill" : "eye.fill")
                            .foregroundColor(.blue)
                    }
                }

                Divider()
                    .background(Color.white.opacity(0.1))

                // CVV
                HStack {
                    VStack(alignment: .leading, spacing: 4) {
                        Text("CVV")
                            .font(.system(size: 12))
                            .foregroundColor(.white.opacity(0.6))

                        Text(showCVV ? card.cvv : "•••")
                            .font(.system(size: 16, weight: .semibold, design: .monospaced))
                            .foregroundColor(.white)
                    }

                    Spacer()

                    Button(action: { showCVV.toggle() }) {
                        Image(systemName: showCVV ? "eye.slash.fill" : "eye.fill")
                            .foregroundColor(.blue)
                    }
                }

                Divider()
                    .background(Color.white.opacity(0.1))

                // Expiry
                HStack {
                    Text("Expiry Date")
                        .font(.system(size: 12))
                        .foregroundColor(.white.opacity(0.6))

                    Spacer()

                    Text(card.expiryString)
                        .font(.system(size: 16, weight: .semibold, design: .monospaced))
                        .foregroundColor(.white)
                }

                Divider()
                    .background(Color.white.opacity(0.1))

                // Cardholder
                HStack {
                    Text("Cardholder")
                        .font(.system(size: 12))
                        .foregroundColor(.white.opacity(0.6))

                    Spacer()

                    Text(card.cardholderName)
                        .font(.system(size: 16, weight: .semibold))
                        .foregroundColor(.white)
                }
            }
            .padding()
            .background(Color.white.opacity(0.05))
            .cornerRadius(16)
        }
    }
}

// MARK: - Security Controls

struct SecurityControlsSection: View {
    let card: CryptoCard
    let onFreeze: () -> Void
    let onBlock: () -> Void
    let onPINChange: () -> Void

    var body: some View {
        VStack(spacing: 12) {
            Text("Security")
                .font(.system(size: 18, weight: .bold))
                .foregroundColor(.white)
                .frame(maxWidth: .infinity, alignment: .leading)

            VStack(spacing: 12) {
                SecurityButton(
                    icon: card.isFrozen ? "flame.fill" : "snowflake",
                    title: card.isFrozen ? "Unfreeze Card" : "Freeze Card",
                    subtitle: card.isFrozen ? "Resume transactions" : "Temporarily block transactions",
                    color: card.isFrozen ? .orange : .cyan,
                    action: onFreeze
                )

                SecurityButton(
                    icon: "lock.fill",
                    title: "Change PIN",
                    subtitle: "Update your card PIN",
                    color: .blue,
                    action: onPINChange
                )

                SecurityButton(
                    icon: "xmark.shield.fill",
                    title: "Block Card",
                    subtitle: "Permanently block this card",
                    color: .red,
                    action: onBlock,
                    isDestructive: true
                )
            }
        }
    }
}

struct SecurityButton: View {
    let icon: String
    let title: String
    let subtitle: String
    let color: Color
    let action: () -> Void
    var isDestructive = false

    var body: some View {
        Button(action: action) {
            HStack(spacing: 12) {
                ZStack {
                    Circle()
                        .fill(color.opacity(0.2))
                        .frame(width: 44, height: 44)

                    Image(systemName: icon)
                        .foregroundColor(color)
                }

                VStack(alignment: .leading, spacing: 2) {
                    Text(title)
                        .font(.system(size: 15, weight: .semibold))
                        .foregroundColor(.white)

                    Text(subtitle)
                        .font(.system(size: 12))
                        .foregroundColor(.white.opacity(0.6))
                }

                Spacer()

                Image(systemName: "chevron.right")
                    .font(.system(size: 14))
                    .foregroundColor(.white.opacity(0.4))
            }
            .padding()
            .background(Color.white.opacity(0.05))
            .cornerRadius(12)
        }
    }
}

// MARK: - Collateral Management

struct CollateralManagementSection: View {
    let collateral: CollateralInfo
    let onAdd: () -> Void
    let onWithdraw: () -> Void

    var body: some View {
        VStack(spacing: 12) {
            Text("Collateral")
                .font(.system(size: 18, weight: .bold))
                .foregroundColor(.white)
                .frame(maxWidth: .infinity, alignment: .leading)

            VStack(spacing: 16) {
                // Collateral stats
                HStack {
                    CollateralStat(
                        title: "Locked",
                        value: String(format: "%.4f %@", collateral.collateralAmount, collateral.cryptoAsset)
                    )

                    Spacer()

                    CollateralStat(
                        title: "Value",
                        value: String(format: "$%.2f", collateral.collateralValueUSD)
                    )
                }

                // LTV and liquidation
                HStack {
                    CollateralStat(
                        title: "LTV Ratio",
                        value: String(format: "%.0f%%", collateral.ltv * 100),
                        color: ltvColor
                    )

                    Spacer()

                    CollateralStat(
                        title: "Liquidation",
                        value: String(format: "$%.2f", collateral.liquidationThreshold),
                        color: .orange
                    )
                }

                // Actions
                HStack(spacing: 12) {
                    Button(action: onAdd) {
                        Label("Add", systemImage: "plus.circle.fill")
                            .font(.system(size: 14, weight: .semibold))
                            .foregroundColor(.white)
                            .frame(maxWidth: .infinity)
                            .padding(.vertical, 12)
                            .background(Color.green.opacity(0.8))
                            .cornerRadius(10)
                    }

                    Button(action: onWithdraw) {
                        Label("Withdraw", systemImage: "minus.circle.fill")
                            .font(.system(size: 14, weight: .semibold))
                            .foregroundColor(.white)
                            .frame(maxWidth: .infinity)
                            .padding(.vertical, 12)
                            .background(Color.orange.opacity(0.8))
                            .cornerRadius(10)
                    }
                }
            }
            .padding()
            .background(Color.white.opacity(0.05))
            .cornerRadius(16)
        }
    }

    private var ltvColor: Color {
        if collateral.ltv < 0.5 {
            return .green
        } else if collateral.ltv < 0.7 {
            return .yellow
        } else {
            return .red
        }
    }
}

struct CollateralStat: View {
    let title: String
    let value: String
    var color: Color = .white

    var body: some View {
        VStack(alignment: .leading, spacing: 4) {
            Text(title)
                .font(.system(size: 12))
                .foregroundColor(.white.opacity(0.6))

            Text(value)
                .font(.system(size: 16, weight: .bold))
                .foregroundColor(color)
        }
    }
}

// MARK: - Spending Controls

struct SpendingControlsSection: View {
    let limits: SpendingLimits
    let onUpdate: () -> Void

    var body: some View {
        VStack(spacing: 12) {
            HStack {
                Text("Spending Limits")
                    .font(.system(size: 18, weight: .bold))
                    .foregroundColor(.white)

                Spacer()

                Button("Edit") {
                    onUpdate()
                }
                .font(.system(size: 14, weight: .semibold))
                .foregroundColor(.blue)
            }

            VStack(spacing: 12) {
                LimitRow(
                    title: "Daily Limit",
                    spent: limits.dailySpent,
                    limit: limits.dailyLimit
                )

                LimitRow(
                    title: "Monthly Limit",
                    spent: limits.monthlySpent,
                    limit: limits.monthlyLimit
                )

                LimitRow(
                    title: "Per Transaction",
                    spent: 0,
                    limit: limits.perTransactionLimit
                )
            }
            .padding()
            .background(Color.white.opacity(0.05))
            .cornerRadius(16)
        }
    }
}

struct LimitRow: View {
    let title: String
    let spent: Double
    let limit: Double

    var body: some View {
        VStack(alignment: .leading, spacing: 8) {
            HStack {
                Text(title)
                    .font(.system(size: 14))
                    .foregroundColor(.white.opacity(0.6))

                Spacer()

                if spent > 0 {
                    Text("$\(spent, specifier: "%.0f") / $\(limit, specifier: "%.0f")")
                        .font(.system(size: 14, weight: .semibold))
                        .foregroundColor(.white)
                } else {
                    Text("$\(limit, specifier: "%.0f")")
                        .font(.system(size: 14, weight: .semibold))
                        .foregroundColor(.white)
                }
            }

            if spent > 0 {
                GeometryReader { geometry in
                    ZStack(alignment: .leading) {
                        Rectangle()
                            .fill(Color.white.opacity(0.1))
                            .frame(height: 4)
                            .cornerRadius(2)

                        Rectangle()
                            .fill(progressColor)
                            .frame(width: geometry.size.width * min(spent / limit, 1.0), height: 4)
                            .cornerRadius(2)
                    }
                }
                .frame(height: 4)
            }
        }
    }

    private var progressColor: Color {
        let ratio = spent / limit
        if ratio < 0.5 {
            return .green
        } else if ratio < 0.8 {
            return .yellow
        } else {
            return .red
        }
    }
}

// MARK: - Apple Wallet

struct AppleWalletSection: View {
    let onAdd: () -> Void

    var body: some View {
        Button(action: onAdd) {
            HStack {
                Image(systemName: "apple.logo")
                    .font(.system(size: 20))

                Text("Add to Apple Wallet")
                    .font(.system(size: 16, weight: .semibold))

                Spacer()

                Image(systemName: "chevron.right")
                    .font(.system(size: 14))
                    .foregroundColor(.white.opacity(0.4))
            }
            .foregroundColor(.white)
            .padding()
            .background(Color.white.opacity(0.1))
            .cornerRadius(12)
        }
    }
}

// MARK: - Transaction History

struct TransactionHistorySection: View {
    let transactions: [CardTransaction]

    var body: some View {
        VStack(alignment: .leading, spacing: 12) {
            Text("All Transactions")
                .font(.system(size: 18, weight: .bold))
                .foregroundColor(.white)

            VStack(spacing: 12) {
                ForEach(transactions) { transaction in
                    TransactionDetailRow(transaction: transaction)
                }
            }
        }
    }
}

struct TransactionDetailRow: View {
    let transaction: CardTransaction

    var body: some View {
        VStack(spacing: 12) {
            HStack(spacing: 12) {
                // Icon
                ZStack {
                    Circle()
                        .fill(iconColor.opacity(0.3))
                        .frame(width: 44, height: 44)

                    Image(systemName: transaction.icon)
                        .font(.system(size: 18))
                        .foregroundColor(iconColor)
                }

                // Details
                VStack(alignment: .leading, spacing: 4) {
                    Text(transaction.merchantName)
                        .font(.system(size: 15, weight: .semibold))
                        .foregroundColor(.white)

                    HStack(spacing: 8) {
                        if transaction.nfcTransaction {
                            HStack(spacing: 2) {
                                Image(systemName: "wave.3.right")
                                    .font(.system(size: 10))
                                Text("NFC")
                                    .font(.system(size: 11, weight: .medium))
                            }
                            .foregroundColor(.blue)
                            .padding(.horizontal, 6)
                            .padding(.vertical, 2)
                            .background(Color.blue.opacity(0.2))
                            .cornerRadius(4)
                        }

                        Text(transaction.merchantCategory)
                            .font(.system(size: 12))
                            .foregroundColor(.white.opacity(0.6))
                    }
                }

                Spacer()

                // Amount
                VStack(alignment: .trailing, spacing: 4) {
                    Text(transaction.amountString)
                        .font(.system(size: 16, weight: .bold))
                        .foregroundColor(amountColor)

                    StatusBadge(status: transaction.status)
                }
            }

            // Additional info
            HStack {
                Text(transaction.timestamp.formatted(date: .abbreviated, time: .shortened))
                    .font(.system(size: 12))
                    .foregroundColor(.white.opacity(0.5))

                Spacer()

                if let location = transaction.location, let city = location.city {
                    HStack(spacing: 4) {
                        Image(systemName: "mappin.circle.fill")
                        Text(city)
                    }
                    .font(.system(size: 12))
                    .foregroundColor(.white.opacity(0.5))
                }
            }
        }
        .padding()
        .background(Color.white.opacity(0.05))
        .cornerRadius(12)
    }

    private var iconColor: Color {
        switch transaction.status {
        case .completed: return .blue
        case .pending: return .yellow
        case .declined: return .red
        case .refunded: return .green
        }
    }

    private var amountColor: Color {
        switch transaction.status {
        case .completed: return .white
        case .pending: return .yellow
        case .declined: return .red
        case .refunded: return .green
        }
    }
}

// MARK: - PIN Change View

struct PINChangeView: View {
    let cardId: String
    @Environment(\.dismiss) var dismiss
    @State private var currentPIN = ""
    @State private var newPIN = ""
    @State private var confirmPIN = ""
    @State private var isLoading = false
    @State private var errorMessage: String?

    var body: some View {
        NavigationView {
            Form {
                Section("Current PIN") {
                    SecureField("Enter current PIN", text: $currentPIN)
                        .keyboardType(.numberPad)
                }

                Section("New PIN") {
                    SecureField("Enter new PIN", text: $newPIN)
                        .keyboardType(.numberPad)

                    SecureField("Confirm new PIN", text: $confirmPIN)
                        .keyboardType(.numberPad)
                }

                Section {
                    Button("Change PIN") {
                        Task {
                            await changePIN()
                        }
                    }
                    .disabled(!isValid || isLoading)
                }
            }
            .navigationTitle("Change PIN")
            .navigationBarTitleDisplayMode(.inline)
            .toolbar {
                ToolbarItem(placement: .navigationBarLeading) {
                    Button("Cancel") {
                        dismiss()
                    }
                }
            }
            .alert("Error", isPresented: .constant(errorMessage != nil)) {
                Button("OK") {
                    errorMessage = nil
                }
            } message: {
                Text(errorMessage ?? "")
            }
        }
    }

    private var isValid: Bool {
        !currentPIN.isEmpty &&
        newPIN.count == 4 &&
        newPIN == confirmPIN
    }

    private func changePIN() async {
        isLoading = true

        do {
            _ = try await CardService.shared.changePIN(cardId: cardId, currentPIN: currentPIN, newPIN: newPIN)
            dismiss()
        } catch {
            errorMessage = error.localizedDescription
        }

        isLoading = false
    }
}

// MARK: - Add Collateral View

struct AddCollateralView: View {
    let cardId: String
    let onComplete: () -> Void
    @Environment(\.dismiss) var dismiss
    @State private var amount = ""

    var body: some View {
        NavigationView {
            Form {
                Section("Amount") {
                    TextField("Enter amount", text: $amount)
                        .keyboardType(.decimalPad)
                }

                Section {
                    Button("Add Collateral") {
                        Task {
                            if let value = Double(amount), value > 0 {
                                do {
                                    _ = try await CardService.shared.addCollateral(cardId: cardId, amount: value)
                                    onComplete()
                                    dismiss()
                                } catch {
                                    // Handle error
                                }
                            }
                        }
                    }
                    .disabled(amount.isEmpty)
                }
            }
            .navigationTitle("Add Collateral")
            .navigationBarTitleDisplayMode(.inline)
            .toolbar {
                ToolbarItem(placement: .navigationBarLeading) {
                    Button("Cancel") {
                        dismiss()
                    }
                }
            }
        }
    }
}

// MARK: - Withdraw Collateral View

struct WithdrawCollateralView: View {
    let cardId: String
    let onComplete: () -> Void
    @Environment(\.dismiss) var dismiss
    @State private var amount = ""

    var body: some View {
        NavigationView {
            Form {
                Section("Amount") {
                    TextField("Enter amount", text: $amount)
                        .keyboardType(.decimalPad)
                }

                Section {
                    Button("Withdraw Collateral") {
                        Task {
                            if let value = Double(amount), value > 0 {
                                do {
                                    _ = try await CardService.shared.withdrawCollateral(cardId: cardId, amount: value)
                                    onComplete()
                                    dismiss()
                                } catch {
                                    // Handle error
                                }
                            }
                        }
                    }
                    .disabled(amount.isEmpty)
                }
            }
            .navigationTitle("Withdraw Collateral")
            .navigationBarTitleDisplayMode(.inline)
            .toolbar {
                ToolbarItem(placement: .navigationBarLeading) {
                    Button("Cancel") {
                        dismiss()
                    }
                }
            }
        }
    }
}

// MARK: - Card Controls View

struct CardControlsView: View {
    let cardId: String
    @State var controls: CardControls
    let onUpdate: () -> Void
    @Environment(\.dismiss) var dismiss

    var body: some View {
        NavigationView {
            Form {
                Section("Transaction Types") {
                    Toggle("Online Transactions", isOn: $controls.onlineTransactionsEnabled)
                    Toggle("Contactless (NFC)", isOn: $controls.contactlessEnabled)
                    Toggle("ATM Withdrawals", isOn: $controls.atmWithdrawalsEnabled)
                    Toggle("Foreign Transactions", isOn: $controls.foreignTransactionsEnabled)
                }

                Section {
                    Button("Save Changes") {
                        Task {
                            do {
                                _ = try await CardService.shared.updateControls(cardId: cardId, controls: controls)
                                onUpdate()
                                dismiss()
                            } catch {
                                // Handle error
                            }
                        }
                    }
                }
            }
            .navigationTitle("Card Controls")
            .navigationBarTitleDisplayMode(.inline)
            .toolbar {
                ToolbarItem(placement: .navigationBarLeading) {
                    Button("Cancel") {
                        dismiss()
                    }
                }
            }
        }
    }
}
