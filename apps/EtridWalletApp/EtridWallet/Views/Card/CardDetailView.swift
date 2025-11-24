//
//  CardDetailView.swift
//  EtridWallet
//
//  Detailed card view with transactions and collateral
//

import SwiftUI

struct CardDetailView: View {
    let card: CryptoCard
    @ObservedObject var viewModel: CardViewModel
    @Environment(\.dismiss) private var dismiss
    @State private var selectedTab = 0
    @State private var showFullCardNumber = false
    @State private var showSimulateTransaction = false

    var body: some View {
        NavigationView {
            ZStack {
                // Background
                LinearGradient(
                    colors: [Color(red: 0.1, green: 0.05, blue: 0.2), Color(red: 0.2, green: 0.1, blue: 0.3)],
                    startPoint: .topLeading,
                    endPoint: .bottomTrailing
                )
                .ignoresSafeArea()

                ScrollView {
                    VStack(spacing: 24) {
                        // Card Display
                        cardDisplay

                        // Quick Actions
                        quickActions

                        // Tabs
                        tabSelector

                        // Tab Content
                        if selectedTab == 0 {
                            transactionsTab
                        } else if selectedTab == 1 {
                            collateralTab
                        } else {
                            settingsTab
                        }
                    }
                    .padding()
                }
            }
            .navigationTitle("Card Details")
            .navigationBarTitleDisplayMode(.inline)
            .toolbar {
                ToolbarItem(placement: .navigationBarLeading) {
                    Button("Close") {
                        dismiss()
                    }
                }

                ToolbarItem(placement: .navigationBarTrailing) {
                    Menu {
                        if card.status == .active {
                            Button(role: .destructive) {
                                viewModel.freezeCard(cardId: card.id)
                                dismiss()
                            } label: {
                                Label("Freeze Card", systemImage: "snowflake")
                            }
                        } else if card.status == .frozen {
                            Button {
                                viewModel.unfreezeCard(cardId: card.id)
                                dismiss()
                            } label: {
                                Label("Unfreeze Card", systemImage: "sun.max")
                            }
                        }

                        Button(role: .destructive) {
                            viewModel.deleteCard(cardId: card.id)
                            dismiss()
                        } label: {
                            Label("Delete Card", systemImage: "trash")
                        }
                    } label: {
                        Image(systemName: "ellipsis.circle")
                            .foregroundColor(.white)
                    }
                }
            }
            .sheet(isPresented: $showSimulateTransaction) {
                SimulateTransactionView(card: card, viewModel: viewModel)
            }
        }
    }

    private var cardDisplay: some View {
        VStack(spacing: 16) {
            // Spending limit progress
            VStack(alignment: .leading, spacing: 8) {
                HStack {
                    Text("Available Balance")
                        .font(.subheadline)
                        .foregroundColor(.gray)

                    Spacer()

                    let totalSpent = viewModel.getTotalSpent(cardId: card.id)
                    let remaining = card.availableSpendingLimit - totalSpent

                    Text(String(format: "$%.2f / $%.2f", remaining, card.availableSpendingLimit))
                        .font(.caption)
                        .foregroundColor(.gray)
                }

                GeometryReader { geometry in
                    ZStack(alignment: .leading) {
                        Rectangle()
                            .fill(Color.white.opacity(0.1))
                            .frame(height: 8)

                        let totalSpent = viewModel.getTotalSpent(cardId: card.id)
                        let percentage = min(totalSpent / card.availableSpendingLimit, 1.0)

                        Rectangle()
                            .fill(
                                LinearGradient(
                                    colors: percentage > 0.8 ? [.red, .orange] : [.green, .blue],
                                    startPoint: .leading,
                                    endPoint: .trailing
                                )
                            )
                            .frame(width: geometry.size.width * percentage, height: 8)
                    }
                    .cornerRadius(4)
                }
                .frame(height: 8)
            }
            .padding()
            .background(Color.white.opacity(0.05))
            .cornerRadius(12)

            // Card number reveal
            VStack(spacing: 12) {
                Button {
                    showFullCardNumber.toggle()
                } label: {
                    HStack {
                        Image(systemName: showFullCardNumber ? "eye.slash" : "eye")
                        Text(showFullCardNumber ? card.formattedCardNumber : card.maskedCardNumber)
                            .font(.title3)
                            .fontWeight(.medium)
                            .tracking(2)
                    }
                    .foregroundColor(.white)
                }

                HStack(spacing: 40) {
                    VStack(spacing: 4) {
                        Text("EXPIRES")
                            .font(.caption2)
                            .foregroundColor(.gray)
                        Text(card.expiryFormatted)
                            .font(.subheadline)
                            .foregroundColor(.white)
                    }

                    VStack(spacing: 4) {
                        Text("CVV")
                            .font(.caption2)
                            .foregroundColor(.gray)
                        Text(showFullCardNumber ? card.cvv : "•••")
                            .font(.subheadline)
                            .foregroundColor(.white)
                    }
                }
            }
            .padding()
            .background(Color.white.opacity(0.05))
            .cornerRadius(12)
        }
    }

    private var quickActions: some View {
        HStack(spacing: 12) {
            Button {
                showSimulateTransaction = true
            } label: {
                VStack(spacing: 8) {
                    Image(systemName: "dollarsign.circle.fill")
                        .font(.title2)
                    Text("Simulate")
                        .font(.caption)
                }
                .frame(maxWidth: .infinity)
                .padding()
                .background(Color.blue.opacity(0.2))
                .foregroundColor(.blue)
                .cornerRadius(12)
            }

            Button {
                // TODO: Add collateral
            } label: {
                VStack(spacing: 8) {
                    Image(systemName: "plus.circle.fill")
                        .font(.title2)
                    Text("Add Funds")
                        .font(.caption)
                }
                .frame(maxWidth: .infinity)
                .padding()
                .background(Color.green.opacity(0.2))
                .foregroundColor(.green)
                .cornerRadius(12)
            }
        }
    }

    private var tabSelector: some View {
        HStack(spacing: 0) {
            TabButton(title: "Transactions", isSelected: selectedTab == 0) {
                selectedTab = 0
            }

            TabButton(title: "Collateral", isSelected: selectedTab == 1) {
                selectedTab = 1
            }

            TabButton(title: "Settings", isSelected: selectedTab == 2) {
                selectedTab = 2
            }
        }
        .background(Color.white.opacity(0.05))
        .cornerRadius(12)
    }

    private var transactionsTab: some View {
        VStack(alignment: .leading, spacing: 16) {
            Text("Recent Transactions")
                .font(.headline)
                .foregroundColor(.white)

            let transactions = viewModel.getTransactions(cardId: card.id)

            if transactions.isEmpty {
                Text("No transactions yet")
                    .font(.subheadline)
                    .foregroundColor(.gray)
                    .frame(maxWidth: .infinity)
                    .padding(40)
            } else {
                ForEach(transactions) { transaction in
                    TransactionRow(transaction: transaction)
                }
            }
        }
    }

    private var collateralTab: some View {
        VStack(alignment: .leading, spacing: 16) {
            Text("Collateral Assets")
                .font(.headline)
                .foregroundColor(.white)

            VStack(spacing: 12) {
                ForEach(card.collateralAssets) { asset in
                    CardCollateralAssetRow(asset: asset)
                }

                HStack {
                    Text("Total Value")
                        .font(.headline)
                        .foregroundColor(.white)

                    Spacer()

                    Text(String(format: "$%.2f", card.totalCollateralValue))
                        .font(.title3)
                        .bold()
                        .foregroundColor(.green)
                }
                .padding()
                .background(Color.white.opacity(0.05))
                .cornerRadius(12)
            }
        }
    }

    private var settingsTab: some View {
        VStack(alignment: .leading, spacing: 16) {
            Text("Card Settings")
                .font(.headline)
                .foregroundColor(.white)

            VStack(spacing: 0) {
                SettingRow(icon: "percent", title: "Spending Limit", value: "\(Int(card.spendingLimitRatio * 100))%")

                Divider()
                    .background(Color.white.opacity(0.1))

                SettingRow(icon: "calendar", title: "Created", value: card.createdAt.formatted(date: .abbreviated, time: .omitted))

                if let lastUsed = card.lastUsedAt {
                    Divider()
                        .background(Color.white.opacity(0.1))

                    SettingRow(icon: "clock", title: "Last Used", value: lastUsed.formatted(date: .abbreviated, time: .shortened))
                }
            }
            .background(Color.white.opacity(0.05))
            .cornerRadius(12)
        }
    }
}

// MARK: - Tab Button

private struct TabButton: View {
    let title: String
    let isSelected: Bool
    let action: () -> Void

    var body: some View {
        Button(action: action) {
            Text(title)
                .font(.subheadline)
                .fontWeight(isSelected ? .semibold : .regular)
                .foregroundColor(isSelected ? .white : .gray)
                .frame(maxWidth: .infinity)
                .padding(.vertical, 12)
                .background(isSelected ? Color.blue.opacity(0.3) : Color.clear)
                .cornerRadius(12)
        }
    }
}

// MARK: - Transaction Row

private struct TransactionRow: View {
    let transaction: CardTransaction

    var body: some View {
        HStack(spacing: 12) {
            Image(systemName: transaction.category.icon)
                .font(.title3)
                .foregroundColor(statusColor)
                .frame(width: 40, height: 40)
                .background(statusColor.opacity(0.2))
                .cornerRadius(10)

            VStack(alignment: .leading, spacing: 4) {
                Text(transaction.merchantName)
                    .font(.headline)
                    .foregroundColor(.white)

                Text(transaction.formattedDate)
                    .font(.caption)
                    .foregroundColor(.gray)
            }

            Spacer()

            VStack(alignment: .trailing, spacing: 4) {
                Text(transaction.formattedAmount)
                    .font(.headline)
                    .foregroundColor(transaction.status == .completed ? .white : .orange)

                Text(transaction.status.rawValue)
                    .font(.caption2)
                    .foregroundColor(statusColor)
            }
        }
        .padding()
        .background(Color.white.opacity(0.05))
        .cornerRadius(12)
    }

    private var statusColor: Color {
        switch transaction.status {
        case .completed: return .green
        case .pending: return .orange
        case .declined: return .red
        case .refunded: return .blue
        }
    }
}

// MARK: - Collateral Asset Row

private struct CardCollateralAssetRow: View {
    let asset: CardCollateralAsset

    var body: some View {
        HStack {
            VStack(alignment: .leading, spacing: 4) {
                Text(asset.name)
                    .font(.headline)
                    .foregroundColor(.white)

                Text(asset.formattedAmount)
                    .font(.caption)
                    .foregroundColor(.gray)
            }

            Spacer()

            VStack(alignment: .trailing, spacing: 4) {
                Text(asset.formattedValue)
                    .font(.subheadline)
                    .bold()
                    .foregroundColor(.green)

                Text("@ \(String(format: "$%.2f", asset.priceUSD))")
                    .font(.caption)
                    .foregroundColor(.gray)
            }
        }
        .padding()
        .background(Color.white.opacity(0.05))
        .cornerRadius(12)
    }
}

// MARK: - Setting Row

private struct SettingRow: View {
    let icon: String
    let title: String
    let value: String

    var body: some View {
        HStack {
            Image(systemName: icon)
                .foregroundColor(.blue)
                .frame(width: 30)

            Text(title)
                .foregroundColor(.white)

            Spacer()

            Text(value)
                .foregroundColor(.gray)
        }
        .padding()
    }
}

// MARK: - Simulate Transaction View

struct SimulateTransactionView: View {
    let card: CryptoCard
    @ObservedObject var viewModel: CardViewModel
    @Environment(\.dismiss) private var dismiss

    @State private var merchantName = ""
    @State private var amount = ""
    @State private var selectedCategory: CardTransaction.TransactionCategory = .shopping

    var body: some View {
        NavigationView {
            Form {
                Section("Transaction Details") {
                    TextField("Merchant Name", text: $merchantName)

                    HStack {
                        Text("$")
                        TextField("Amount", text: $amount)
                            .keyboardType(.decimalPad)
                    }

                    Picker("Category", selection: $selectedCategory) {
                        ForEach(CardTransaction.TransactionCategory.allCases, id: \.self) { category in
                            Label(category.rawValue, systemImage: category.icon)
                                .tag(category)
                        }
                    }
                }

                Section {
                    let totalSpent = viewModel.getTotalSpent(cardId: card.id)
                    let remaining = card.availableSpendingLimit - totalSpent
                    let transactionAmount = Double(amount) ?? 0

                    HStack {
                        Text("Available")
                        Spacer()
                        Text(String(format: "$%.2f", remaining))
                            .bold()
                    }

                    HStack {
                        Text("After Transaction")
                        Spacer()
                        Text(String(format: "$%.2f", remaining - transactionAmount))
                            .bold()
                            .foregroundColor(transactionAmount > remaining ? .red : .green)
                    }
                } header: {
                    Text("Balance")
                }
            }
            .navigationTitle("Simulate Transaction")
            .navigationBarTitleDisplayMode(.inline)
            .toolbar {
                ToolbarItem(placement: .cancellationAction) {
                    Button("Cancel") {
                        dismiss()
                    }
                }

                ToolbarItem(placement: .confirmationAction) {
                    Button("Charge") {
                        processTransaction()
                    }
                    .disabled(!isValid)
                }
            }
        }
    }

    private var isValid: Bool {
        !merchantName.isEmpty && !amount.isEmpty && (Double(amount) ?? 0) > 0
    }

    private func processTransaction() {
        guard let transactionAmount = Double(amount) else { return }

        let success = viewModel.simulateTransaction(
            cardId: card.id,
            merchant: merchantName,
            amount: transactionAmount,
            category: selectedCategory
        )

        if success {
            dismiss()
        } else {
            // Show error alert
            dismiss()
        }
    }
}

#Preview {
    let card = CryptoCard(
        id: "1",
        cardNumber: "4532123456789012",
        cardholderName: "JOHN DOE",
        expiryDate: Date().addingTimeInterval(365 * 24 * 3600 * 3),
        cvv: "123",
        status: .active,
        createdAt: Date(),
        lastUsedAt: Date(),
        collateralAssets: [],
        spendingLimitRatio: 0.5
    )

    CardDetailView(card: card, viewModel: CardViewModel())
}
