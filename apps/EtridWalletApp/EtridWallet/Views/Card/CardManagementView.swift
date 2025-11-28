//
//  CardManagementView.swift
//  EtridWallet
//
//  Crypto-backed virtual debit card interface
//

import SwiftUI

struct CardManagementView: View {
    @StateObject private var viewModel = CardViewModel()
    @State private var showCreateCard = false
    @State private var selectedCard: CryptoCard?

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
                        if viewModel.cards.isEmpty {
                            emptyState
                        } else {
                            cardsSection
                        }
                    }
                    .padding()
                }
            }
            .navigationTitle("Crypto Card")
            .navigationBarTitleDisplayMode(.large)
            .toolbar {
                ToolbarItem(placement: .navigationBarTrailing) {
                    Button {
                        showCreateCard = true
                    } label: {
                        Image(systemName: "plus.circle.fill")
                            .foregroundColor(.orange)
                    }
                }
            }
            .sheet(isPresented: $showCreateCard) {
                CreateCardView(viewModel: viewModel)
            }
            .sheet(item: $selectedCard) { card in
                CardDetailView(card: card, viewModel: viewModel)
            }
        }
    }

    private var emptyState: some View {
        VStack(spacing: 20) {
            Spacer()

            Image(systemName: "creditcard.fill")
                .font(.system(size: 80))
                .foregroundColor(.orange.opacity(0.5))

            Text("No Cards Yet")
                .font(.title2)
                .bold()
                .foregroundColor(.white)

            Text("Create a crypto-backed card to start spending")
                .font(.body)
                .foregroundColor(.gray)
                .multilineTextAlignment(.center)
                .padding(.horizontal, 40)

            Button {
                showCreateCard = true
            } label: {
                Text("Create Card")
                    .font(.headline)
                    .foregroundColor(.white)
                    .frame(maxWidth: .infinity)
                    .padding()
                    .background(
                        LinearGradient(
                            colors: [Color.orange, Color.red],
                            startPoint: .leading,
                            endPoint: .trailing
                        )
                    )
                    .cornerRadius(12)
            }
            .padding(.horizontal, 40)

            Spacer()
        }
    }

    private var cardsSection: some View {
        VStack(spacing: 20) {
            ForEach(viewModel.cards) { card in
                Button {
                    selectedCard = card
                } label: {
                    VirtualCardView(card: card)
                }
                .buttonStyle(PlainButtonStyle())
            }
        }
    }
}

// MARK: - Virtual Card View

struct VirtualCardView: View {
    let card: CryptoCard

    var body: some View {
        ZStack {
            // Card background with gradient
            RoundedRectangle(cornerRadius: 20)
                .fill(
                    LinearGradient(
                        colors: [
                            statusColor,
                            statusColor.opacity(0.7)
                        ],
                        startPoint: .topLeading,
                        endPoint: .bottomTrailing
                    )
                )
                .shadow(color: statusColor.opacity(0.3), radius: 10, x: 0, y: 5)

            VStack(alignment: .leading, spacing: 16) {
                // Header
                HStack {
                    Image(systemName: "creditcard.fill")
                        .font(.title2)

                    Spacer()

                    StatusBadge(status: card.status)
                }

                Spacer()

                // Card number
                Text(card.maskedCardNumber)
                    .font(.title3)
                    .fontWeight(.medium)
                    .tracking(2)

                // Card details
                HStack {
                    VStack(alignment: .leading, spacing: 4) {
                        Text("CARDHOLDER")
                            .font(.caption2)
                            .opacity(0.7)

                        Text(card.cardholderName)
                            .font(.subheadline)
                            .fontWeight(.medium)
                    }

                    Spacer()

                    VStack(alignment: .trailing, spacing: 4) {
                        Text("EXPIRES")
                            .font(.caption2)
                            .opacity(0.7)

                        Text(card.expiryFormatted)
                            .font(.subheadline)
                            .fontWeight(.medium)
                    }
                }

                // Spending limit
                VStack(alignment: .leading, spacing: 8) {
                    HStack {
                        Text("Available")
                            .font(.caption)
                            .opacity(0.7)

                        Spacer()

                        Text("Collateral: \(String(format: "$%.2f", card.totalCollateralValue))")
                            .font(.caption)
                            .opacity(0.7)
                    }

                    Text(String(format: "$%.2f", card.availableSpendingLimit))
                        .font(.title2)
                        .bold()
                }
            }
            .foregroundColor(.white)
            .padding(24)
        }
        .frame(height: 220)
    }

    private var statusColor: Color {
        switch card.status {
        case .active:
            return Color.blue
        case .frozen:
            return Color.gray
        case .suspended:
            return Color.orange
        case .expired:
            return Color.red
        }
    }
}

// MARK: - Status Badge

private struct StatusBadge: View {
    let status: CryptoCard.CardStatus

    var body: some View {
        Text(status.rawValue)
            .font(.caption2)
            .fontWeight(.semibold)
            .padding(.horizontal, 10)
            .padding(.vertical, 4)
            .background(Color.white.opacity(0.3))
            .cornerRadius(8)
    }
}

// MARK: - Create Card View

struct CreateCardView: View {
    @ObservedObject var viewModel: CardViewModel
    @Environment(\.dismiss) private var dismiss

    @State private var cardholderName = ""
    @State private var selectedAssets: [CardCollateralAsset] = []
    @State private var spendingLimitRatio = 0.5
    @State private var showAssetPicker = false

    var body: some View {
        NavigationView {
            Form {
                Section("Card Details") {
                    TextField("Cardholder Name", text: $cardholderName)
                        .textInputAutocapitalization(.characters)
                }

                Section("Collateral Assets") {
                    if selectedAssets.isEmpty {
                        Button("Add Collateral") {
                            showAssetPicker = true
                        }
                    } else {
                        ForEach(selectedAssets) { asset in
                            HStack {
                                VStack(alignment: .leading) {
                                    Text(asset.name)
                                        .font(.headline)
                                    Text(asset.formattedAmount)
                                        .font(.caption)
                                        .foregroundColor(.secondary)
                                }

                                Spacer()

                                Text(asset.formattedValue)
                                    .font(.subheadline)
                                    .bold()
                            }
                        }

                        Button("Add More") {
                            showAssetPicker = true
                        }
                    }

                    if !selectedAssets.isEmpty {
                        HStack {
                            Text("Total Collateral")
                                .bold()
                            Spacer()
                            Text(String(format: "$%.2f", totalCollateralValue))
                                .bold()
                                .foregroundColor(.green)
                        }
                    }
                }

                Section {
                    VStack(alignment: .leading, spacing: 12) {
                        HStack {
                            Text("Spending Limit")
                                .font(.headline)

                            Spacer()

                            Text("\(Int(spendingLimitRatio * 100))% of collateral")
                                .font(.subheadline)
                                .foregroundColor(.secondary)
                        }

                        Slider(value: $spendingLimitRatio, in: 0.1...0.9, step: 0.05)

                        HStack {
                            Text("Available to spend:")
                                .font(.caption)
                                .foregroundColor(.secondary)

                            Spacer()

                            Text(String(format: "$%.2f", totalCollateralValue * spendingLimitRatio))
                                .font(.headline)
                                .foregroundColor(.green)
                        }
                    }
                } header: {
                    Text("Spending Limit")
                } footer: {
                    Text("Choose how much of your collateral value you can spend. Lower limits reduce risk.")
                }
            }
            .navigationTitle("Create Card")
            .navigationBarTitleDisplayMode(.inline)
            .toolbar {
                ToolbarItem(placement: .cancellationAction) {
                    Button("Cancel") {
                        dismiss()
                    }
                }

                ToolbarItem(placement: .confirmationAction) {
                    Button("Create") {
                        createCard()
                    }
                    .disabled(!isValid)
                }
            }
            .sheet(isPresented: $showAssetPicker) {
                AssetPickerView(selectedAssets: $selectedAssets)
            }
        }
    }

    private var isValid: Bool {
        !cardholderName.isEmpty && !selectedAssets.isEmpty
    }

    private var totalCollateralValue: Double {
        selectedAssets.reduce(0) { $0 + $1.currentValue }
    }

    private func createCard() {
        let application = CardApplication(
            selectedAssets: selectedAssets,
            spendingLimitRatio: spendingLimitRatio,
            cardholderName: cardholderName
        )

        viewModel.createCard(application: application)
        dismiss()
    }
}

// MARK: - Asset Picker View

struct AssetPickerView: View {
    @Binding var selectedAssets: [CardCollateralAsset]
    @Environment(\.dismiss) private var dismiss

    // Mock available assets
    private let availableAssets = [
        CardCollateralAsset(
            id: "1",
            symbol: "ËTR",
            name: "Ëtrid",
            amount: "5000000000000000000000",
            lockedAt: Date(),
            priceUSD: 2.50
        ),
        CardCollateralAsset(
            id: "2",
            symbol: "ETH",
            name: "Ethereum",
            amount: "2000000000000000000",
            lockedAt: Date(),
            priceUSD: 3200.00
        ),
        CardCollateralAsset(
            id: "3",
            symbol: "BTC",
            name: "Bitcoin",
            amount: "50000000000000000",
            lockedAt: Date(),
            priceUSD: 65000.00
        )
    ]

    var body: some View {
        NavigationView {
            List {
                ForEach(availableAssets) { asset in
                    Button {
                        if !selectedAssets.contains(where: { $0.id == asset.id }) {
                            selectedAssets.append(asset)
                        }
                    } label: {
                        HStack {
                            VStack(alignment: .leading) {
                                Text(asset.name)
                                    .font(.headline)

                                Text(asset.formattedAmount)
                                    .font(.caption)
                                    .foregroundColor(.secondary)
                            }

                            Spacer()

                            VStack(alignment: .trailing) {
                                Text(asset.formattedValue)
                                    .font(.subheadline)
                                    .bold()

                                if selectedAssets.contains(where: { $0.id == asset.id }) {
                                    Image(systemName: "checkmark.circle.fill")
                                        .foregroundColor(.green)
                                }
                            }
                        }
                    }
                }
            }
            .navigationTitle("Select Assets")
            .navigationBarTitleDisplayMode(.inline)
            .toolbar {
                ToolbarItem(placement: .confirmationAction) {
                    Button("Done") {
                        dismiss()
                    }
                }
            }
        }
    }
}

#Preview {
    CardManagementView()
}
