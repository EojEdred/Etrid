//
//  CardManagementView.swift
//  EtridWallet
//
//  Main card management interface with NFC payment
//

import SwiftUI

@MainActor
class CardManagementViewModel: ObservableObject {
    @Published var cards: [CryptoCard] = []
    @Published var selectedCard: CryptoCard?
    @Published var recentTransactions: [CardTransaction] = []
    @Published var isLoading = false
    @Published var errorMessage: String?
    @Published var showCreateCard = false
    @Published var showNFCPayment = false
    @Published var showCardDetail = false

    private let cardService = CardService.shared
    private let userId = "user_123" // TODO: Get from auth

    func loadData() async {
        isLoading = true
        errorMessage = nil

        do {
            cards = try await cardService.getCards(for: userId)
            if let firstActive = cards.first(where: { $0.isActive }) {
                selectedCard = firstActive
                await loadTransactions()
            }
        } catch {
            errorMessage = error.localizedDescription
        }

        isLoading = false
    }

    func loadTransactions() async {
        guard let cardId = selectedCard?.id else { return }

        do {
            recentTransactions = try await cardService.getTransactions(for: cardId, limit: 10)
        } catch {
            errorMessage = error.localizedDescription
        }
    }

    func createCard(cardType: CardType, collateralAsset: String, collateralAmount: Double) async {
        isLoading = true

        do {
            let newCard = try await cardService.createCard(
                userId: userId,
                cardType: cardType,
                collateralAsset: collateralAsset,
                collateralAmount: collateralAmount
            )
            cards.append(newCard)
            selectedCard = newCard
            showCreateCard = false
        } catch {
            errorMessage = error.localizedDescription
        }

        isLoading = false
    }

    func freezeCard() async {
        guard let cardId = selectedCard?.id else { return }

        do {
            let updatedCard = try await cardService.freezeCard(cardId: cardId)
            updateCard(updatedCard)
        } catch {
            errorMessage = error.localizedDescription
        }
    }

    func unfreezeCard() async {
        guard let cardId = selectedCard?.id else { return }

        do {
            let updatedCard = try await cardService.unfreezeCard(cardId: cardId)
            updateCard(updatedCard)
        } catch {
            errorMessage = error.localizedDescription
        }
    }

    private func updateCard(_ card: CryptoCard) {
        if let index = cards.firstIndex(where: { $0.id == card.id }) {
            cards[index] = card
        }
        if selectedCard?.id == card.id {
            selectedCard = card
        }
    }
}

struct CardManagementView: View {
    @StateObject private var viewModel = CardManagementViewModel()
    @State private var selectedTab = 0

    var body: some View {
        NavigationView {
            ZStack {
                // Background gradient
                LinearGradient(
                    gradient: Gradient(colors: [Color(hex: "0F172A"), Color(hex: "1E293B")]),
                    startPoint: .top,
                    endPoint: .bottom
                )
                .ignoresSafeArea()

                ScrollView {
                    VStack(spacing: 24) {
                        if viewModel.isLoading && viewModel.cards.isEmpty {
                            CardLoadingView()
                                .padding(.top, 100)
                        } else if viewModel.cards.isEmpty {
                            EmptyCardState {
                                viewModel.showCreateCard = true
                            }
                            .padding(.top, 100)
                        } else {
                            // Card carousel
                            CardCarouselView(
                                cards: viewModel.cards,
                                selectedCard: $viewModel.selectedCard
                            )
                            .padding(.top, 20)

                            // Quick actions
                            if let card = viewModel.selectedCard {
                                QuickActionsView(
                                    card: card,
                                    onNFCPayment: {
                                        viewModel.showNFCPayment = true
                                    },
                                    onFreeze: {
                                        Task {
                                            if card.isFrozen {
                                                await viewModel.unfreezeCard()
                                            } else {
                                                await viewModel.freezeCard()
                                            }
                                        }
                                    },
                                    onDetails: {
                                        viewModel.showCardDetail = true
                                    }
                                )
                                .padding(.horizontal)

                                // Card stats
                                CardStatsView(card: card)
                                    .padding(.horizontal)

                                // Recent transactions
                                if !viewModel.recentTransactions.isEmpty {
                                    RecentTransactionsSection(
                                        transactions: viewModel.recentTransactions
                                    )
                                    .padding(.horizontal)
                                }
                            }
                        }
                    }
                    .padding(.bottom, 100)
                }
                .refreshable {
                    await viewModel.loadData()
                }
            }
            .navigationTitle("Cards")
            .navigationBarTitleDisplayMode(.large)
            .toolbar {
                ToolbarItem(placement: .navigationBarTrailing) {
                    Button(action: { viewModel.showCreateCard = true }) {
                        Image(systemName: "plus.circle.fill")
                            .font(.title2)
                            .foregroundColor(.white)
                    }
                }
            }
            .sheet(isPresented: $viewModel.showCreateCard) {
                CreateCardView { cardType, asset, amount in
                    Task {
                        await viewModel.createCard(
                            cardType: cardType,
                            collateralAsset: asset,
                            collateralAmount: amount
                        )
                    }
                }
            }
            .sheet(isPresented: $viewModel.showNFCPayment) {
                if let card = viewModel.selectedCard {
                    NFCPaymentView(card: card) {
                        Task {
                            await viewModel.loadTransactions()
                            await viewModel.loadData()
                        }
                    }
                }
            }
            .sheet(isPresented: $viewModel.showCardDetail) {
                if let card = viewModel.selectedCard {
                    CardDetailView(card: card, onUpdate: {
                        Task {
                            await viewModel.loadData()
                        }
                    })
                }
            }
        }
        .task {
            await viewModel.loadData()
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

// MARK: - Card Carousel

struct CardCarouselView: View {
    let cards: [CryptoCard]
    @Binding var selectedCard: CryptoCard?

    var body: some View {
        ScrollView(.horizontal, showsIndicators: false) {
            HStack(spacing: 20) {
                ForEach(cards) { card in
                    VirtualCardView(card: card)
                        .scaleEffect(selectedCard?.id == card.id ? 1.0 : 0.9)
                        .opacity(selectedCard?.id == card.id ? 1.0 : 0.6)
                        .onTapGesture {
                            withAnimation(.spring()) {
                                selectedCard = card
                            }
                        }
                }
            }
            .padding(.horizontal, 30)
        }
        .frame(height: 240)
    }
}

// MARK: - Quick Actions

struct QuickActionsView: View {
    let card: CryptoCard
    let onNFCPayment: () -> Void
    let onFreeze: () -> Void
    let onDetails: () -> Void

    var body: some View {
        HStack(spacing: 16) {
            // NFC Payment
            QuickActionButton(
                icon: "wave.3.right.circle.fill",
                title: "Pay",
                subtitle: "NFC",
                color: .blue,
                action: onNFCPayment
            )
            .disabled(!card.isActive)

            // Freeze/Unfreeze
            QuickActionButton(
                icon: card.isFrozen ? "flame.fill" : "snowflake",
                title: card.isFrozen ? "Unfreeze" : "Freeze",
                subtitle: "Security",
                color: card.isFrozen ? .orange : .cyan,
                action: onFreeze
            )

            // Card Details
            QuickActionButton(
                icon: "creditcard.circle.fill",
                title: "Details",
                subtitle: "Manage",
                color: .purple,
                action: onDetails
            )
        }
    }
}

struct QuickActionButton: View {
    let icon: String
    let title: String
    let subtitle: String
    let color: Color
    let action: () -> Void

    var body: some View {
        Button(action: action) {
            VStack(spacing: 8) {
                Image(systemName: icon)
                    .font(.system(size: 28))
                    .foregroundColor(color)

                VStack(spacing: 2) {
                    Text(title)
                        .font(.system(size: 14, weight: .semibold))
                        .foregroundColor(.white)

                    Text(subtitle)
                        .font(.system(size: 10))
                        .foregroundColor(.white.opacity(0.6))
                }
            }
            .frame(maxWidth: .infinity)
            .padding(.vertical, 16)
            .background(Color.white.opacity(0.1))
            .cornerRadius(12)
        }
    }
}

// MARK: - Card Stats

struct CardStatsView: View {
    let card: CryptoCard

    var body: some View {
        VStack(spacing: 16) {
            // Available balance
            VStack(spacing: 8) {
                Text("Available Balance")
                    .font(.system(size: 14))
                    .foregroundColor(.white.opacity(0.6))

                Text("$\(card.collateral.availableAmount, specifier: "%.2f")")
                    .font(.system(size: 32, weight: .bold))
                    .foregroundColor(.white)
            }
            .padding()
            .frame(maxWidth: .infinity)
            .background(Color.white.opacity(0.05))
            .cornerRadius(16)

            // Spending stats
            HStack(spacing: 12) {
                CardStatCard(
                    title: "Daily",
                    value: String(format: "$%.0f", card.limits.dailySpent),
                    subtitle: String(format: "of $%.0f", card.limits.dailyLimit),
                    progress: card.limits.dailySpent / card.limits.dailyLimit
                )

                CardStatCard(
                    title: "Monthly",
                    value: String(format: "$%.0f", card.limits.monthlySpent),
                    subtitle: String(format: "of $%.0f", card.limits.monthlyLimit),
                    progress: card.limits.monthlySpent / card.limits.monthlyLimit
                )
            }

            // Collateral info
            HStack {
                VStack(alignment: .leading, spacing: 4) {
                    Text("Collateral")
                        .font(.system(size: 12))
                        .foregroundColor(.white.opacity(0.6))

                    HStack(spacing: 4) {
                        Text("\(card.collateral.collateralAmount, specifier: "%.4f")")
                            .font(.system(size: 16, weight: .semibold))
                        Text(card.collateral.cryptoAsset)
                            .font(.system(size: 14))
                    }
                    .foregroundColor(.white)
                }

                Spacer()

                VStack(alignment: .trailing, spacing: 4) {
                    Text("LTV Ratio")
                        .font(.system(size: 12))
                        .foregroundColor(.white.opacity(0.6))

                    Text("\(card.collateral.ltv * 100, specifier: "%.0f")%")
                        .font(.system(size: 16, weight: .semibold))
                        .foregroundColor(ltvColor)
                }
            }
            .padding()
            .background(Color.white.opacity(0.05))
            .cornerRadius(12)
        }
    }

    private var ltvColor: Color {
        if card.collateral.ltv < 0.5 {
            return .green
        } else if card.collateral.ltv < 0.7 {
            return .yellow
        } else {
            return .orange
        }
    }
}

struct CardStatCard: View {
    let title: String
    let value: String
    let subtitle: String
    let progress: Double

    var body: some View {
        VStack(alignment: .leading, spacing: 8) {
            Text(title)
                .font(.system(size: 12))
                .foregroundColor(.white.opacity(0.6))

            Text(value)
                .font(.system(size: 18, weight: .bold))
                .foregroundColor(.white)

            Text(subtitle)
                .font(.system(size: 10))
                .foregroundColor(.white.opacity(0.5))

            // Progress bar
            GeometryReader { geometry in
                ZStack(alignment: .leading) {
                    Rectangle()
                        .fill(Color.white.opacity(0.1))
                        .frame(height: 4)
                        .cornerRadius(2)

                    Rectangle()
                        .fill(progressColor)
                        .frame(width: geometry.size.width * min(progress, 1.0), height: 4)
                        .cornerRadius(2)
                }
            }
            .frame(height: 4)
        }
        .padding()
        .frame(maxWidth: .infinity, alignment: .leading)
        .background(Color.white.opacity(0.05))
        .cornerRadius(12)
    }

    private var progressColor: Color {
        if progress < 0.5 {
            return .green
        } else if progress < 0.8 {
            return .yellow
        } else {
            return .red
        }
    }
}

// MARK: - Recent Transactions

struct RecentTransactionsSection: View {
    let transactions: [CardTransaction]

    var body: some View {
        VStack(alignment: .leading, spacing: 12) {
            Text("Recent Transactions")
                .font(.system(size: 18, weight: .bold))
                .foregroundColor(.white)
                .padding(.horizontal, 4)

            VStack(spacing: 8) {
                ForEach(transactions.prefix(5)) { transaction in
                    CardTransactionRow(transaction: transaction)
                }
            }
            .padding()
            .background(Color.white.opacity(0.05))
            .cornerRadius(16)
        }
    }
}

struct CardTransactionRow: View {
    let transaction: CardTransaction

    var body: some View {
        HStack(spacing: 12) {
            // Icon
            ZStack {
                Circle()
                    .fill(iconBackgroundColor)
                    .frame(width: 40, height: 40)

                Image(systemName: transaction.icon)
                    .font(.system(size: 16))
                    .foregroundColor(.white)
            }

            // Details
            VStack(alignment: .leading, spacing: 2) {
                Text(transaction.merchantName)
                    .font(.system(size: 14, weight: .semibold))
                    .foregroundColor(.white)

                HStack(spacing: 4) {
                    if transaction.nfcTransaction {
                        Image(systemName: "wave.3.right")
                            .font(.system(size: 10))
                    }
                    Text(transaction.timestamp, style: .relative)
                        .font(.system(size: 12))
                }
                .foregroundColor(.white.opacity(0.6))
            }

            Spacer()

            // Amount
            VStack(alignment: .trailing, spacing: 2) {
                Text(transaction.amountString)
                    .font(.system(size: 14, weight: .bold))
                    .foregroundColor(amountColor)

                StatusBadge(status: transaction.status)
            }
        }
        .padding(.vertical, 4)
    }

    private var iconBackgroundColor: Color {
        switch transaction.status {
        case .completed: return .blue.opacity(0.3)
        case .pending: return .yellow.opacity(0.3)
        case .declined: return .red.opacity(0.3)
        case .refunded: return .green.opacity(0.3)
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

struct StatusBadge: View {
    let status: CardTransactionStatus

    var body: some View {
        Text(status.rawValue.capitalized)
            .font(.system(size: 9, weight: .semibold))
            .foregroundColor(statusColor)
            .padding(.horizontal, 6)
            .padding(.vertical, 2)
            .background(statusColor.opacity(0.2))
            .cornerRadius(4)
    }

    private var statusColor: Color {
        switch status {
        case .completed: return .green
        case .pending: return .yellow
        case .declined: return .red
        case .refunded: return .blue
        }
    }
}

// MARK: - Empty State

struct EmptyCardState: View {
    let onCreate: () -> Void

    var body: some View {
        VStack(spacing: 24) {
            Image(systemName: "creditcard.circle.fill")
                .font(.system(size: 80))
                .foregroundColor(.white.opacity(0.3))

            VStack(spacing: 8) {
                Text("No Cards Yet")
                    .font(.system(size: 24, weight: .bold))
                    .foregroundColor(.white)

                Text("Create your first crypto-backed card to start spending")
                    .font(.system(size: 14))
                    .foregroundColor(.white.opacity(0.6))
                    .multilineTextAlignment(.center)
                    .padding(.horizontal, 40)
            }

            Button(action: onCreate) {
                HStack {
                    Image(systemName: "plus.circle.fill")
                    Text("Create Card")
                        .fontWeight(.semibold)
                }
                .foregroundColor(.white)
                .padding(.horizontal, 32)
                .padding(.vertical, 16)
                .background(Color.blue)
                .cornerRadius(12)
            }
        }
    }
}

// MARK: - Loading View

struct CardLoadingView: View {
    var body: some View {
        VStack(spacing: 16) {
            ProgressView()
                .scaleEffect(1.5)
                .tint(.white)

            Text("Loading cards...")
                .font(.system(size: 14))
                .foregroundColor(.white.opacity(0.6))
        }
    }
}

// MARK: - Create Card View

struct CreateCardView: View {
    @Environment(\.dismiss) var dismiss
    @State private var selectedAsset = "ETH"
    @State private var collateralAmount = ""
    @State private var cardType: CardType = .virtual

    let onCreate: (CardType, String, Double) -> Void

    private let assets = ["ETH", "BTC", "USDC", "DAI", "USDT"]

    var body: some View {
        NavigationView {
            Form {
                Section("Card Type") {
                    Picker("Type", selection: $cardType) {
                        Text("Virtual Card").tag(CardType.virtual)
                        Text("Physical Card").tag(CardType.physical)
                    }
                    .pickerStyle(.segmented)
                }

                Section("Collateral") {
                    Picker("Asset", selection: $selectedAsset) {
                        ForEach(assets, id: \.self) { asset in
                            Text(asset).tag(asset)
                        }
                    }

                    TextField("Amount", text: $collateralAmount)
                        .keyboardType(.decimalPad)
                }

                Section {
                    Button("Create Card") {
                        if let amount = Double(collateralAmount), amount > 0 {
                            onCreate(cardType, selectedAsset, amount)
                            dismiss()
                        }
                    }
                    .frame(maxWidth: .infinity)
                    .disabled(collateralAmount.isEmpty)
                }
            }
            .navigationTitle("Create Card")
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
