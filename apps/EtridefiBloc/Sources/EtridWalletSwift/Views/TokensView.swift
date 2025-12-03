import SwiftUI
#if os(iOS)
import UIKit
#elseif os(macOS)
import AppKit
#endif

// MARK: - Tokens View

struct TokensView: View {
    @EnvironmentObject var walletManager: EtridWalletManager
    @State private var searchText = ""
    @State private var selectedCategory: TokenCategory = .all
    @State private var showBuyToken = false
    @State private var selectedTokenToBuy: Token?

    enum TokenCategory: String, CaseIterable {
        case all = "All"
        case native = "Native"
        case wETR = "wETR"
        case bridged = "Bridged"
        case stablecoins = "Stablecoins"
    }

    var body: some View {
        NavigationStack {
            VStack(spacing: 0) {
                // Category Filter
                ScrollView(.horizontal, showsIndicators: false) {
                    HStack(spacing: 12) {
                        ForEach(TokenCategory.allCases, id: \.self) { category in
                            CategoryChip(
                                title: category.rawValue,
                                isSelected: selectedCategory == category,
                                action: { selectedCategory = category }
                            )
                        }
                    }
                    .padding(.horizontal)
                }
                .padding(.vertical, 12)

                // Token List
                List {
                    // My Holdings Section
                    if !myHoldings.isEmpty {
                        Section("My Holdings") {
                            ForEach(myHoldings) { balance in
                                TokenDetailRow(balance: balance)
                            }
                        }
                    }

                    // Available Tokens Section
                    Section("Available Tokens") {
                        ForEach(filteredTokens, id: \.id) { token in
                            AvailableTokenRow(
                                token: token,
                                onBuy: {
                                    selectedTokenToBuy = token
                                    showBuyToken = true
                                }
                            )
                        }
                    }
                }
                #if os(iOS)
                .listStyle(.insetGrouped)
                #else
                .listStyle(.inset)
                #endif
                .searchable(text: $searchText, prompt: "Search tokens")
            }
            .navigationTitle("Tokens")
            .sheet(isPresented: $showBuyToken) {
                if let token = selectedTokenToBuy {
                    BuyTokenView(token: token)
                }
            }
        }
    }

    private var myHoldings: [TokenBalance] {
        let balances = walletManager.selectedAccount?.balances ?? []
        return balances.filter { balance in
            let matchesSearch = searchText.isEmpty ||
                balance.token.name.localizedCaseInsensitiveContains(searchText) ||
                balance.token.symbol.localizedCaseInsensitiveContains(searchText)

            let matchesCategory: Bool
            switch selectedCategory {
            case .all: matchesCategory = true
            case .native: matchesCategory = balance.token == .etrid || balance.token == .edsc
            case .wETR: matchesCategory = Token.wrappedETRTokens.contains(balance.token)
            case .bridged: matchesCategory = balance.token.isBridged
            case .stablecoins: matchesCategory = ["USDT", "USDC", "EDSC"].contains(balance.token.symbol)
            }

            return matchesSearch && matchesCategory && balance.amount > 0
        }
    }

    private var filteredTokens: [Token] {
        Token.allTokens.filter { token in
            let matchesSearch = searchText.isEmpty ||
                token.name.localizedCaseInsensitiveContains(searchText) ||
                token.symbol.localizedCaseInsensitiveContains(searchText)

            let matchesCategory: Bool
            switch selectedCategory {
            case .all: matchesCategory = true
            case .native: matchesCategory = token == .etrid || token == .edsc
            case .wETR: matchesCategory = Token.wrappedETRTokens.contains(token)
            case .bridged: matchesCategory = token.isBridged
            case .stablecoins: matchesCategory = ["USDT", "USDC", "EDSC"].contains(token.symbol)
            }

            return matchesSearch && matchesCategory
        }
    }
}

// MARK: - Category Chip

struct CategoryChip: View {
    let title: String
    let isSelected: Bool
    let action: () -> Void

    var body: some View {
        Button(action: action) {
            Text(title)
                .font(.subheadline)
                .fontWeight(isSelected ? .semibold : .regular)
                .foregroundColor(isSelected ? .white : .primary)
                .padding(.horizontal, 16)
                .padding(.vertical, 8)
                .background(isSelected ? EtridColors.primary : EtridColors.cardBackground)
                .cornerRadius(20)
        }
    }
}

// MARK: - Token Detail Row

struct TokenDetailRow: View {
    let balance: TokenBalance

    var body: some View {
        HStack {
            TokenBadge(token: balance.token)

            VStack(alignment: .leading, spacing: 4) {
                Text(balance.token.name)
                    .font(.headline)

                HStack {
                    Text(balance.token.symbol)
                        .font(.caption)
                        .foregroundColor(.secondary)

                    if balance.token.isBridged {
                        Text("Bridged")
                            .font(.caption2)
                            .padding(.horizontal, 6)
                            .padding(.vertical, 2)
                            .background(EtridColors.primary.opacity(0.2))
                            .cornerRadius(4)
                    }
                }
            }

            Spacer()

            VStack(alignment: .trailing, spacing: 4) {
                Text(balance.formattedAmount)
                    .font(.headline)

                Text(balance.formattedUSDValue)
                    .font(.caption)
                    .foregroundColor(.secondary)
            }
        }
        .padding(.vertical, 4)
    }
}

// MARK: - Available Token Row

struct AvailableTokenRow: View {
    let token: Token
    let onBuy: () -> Void

    var body: some View {
        HStack {
            TokenBadge(token: token)

            VStack(alignment: .leading, spacing: 4) {
                Text(token.name)
                    .font(.headline)

                HStack {
                    Text(token.symbol)
                        .font(.caption)
                        .foregroundColor(.secondary)

                    if token.isBridged, let original = token.originalChain {
                        Text("from \(original.rawValue)")
                            .font(.caption2)
                            .foregroundColor(.secondary)
                    }
                }
            }

            Spacer()

            Button(action: onBuy) {
                Text("Buy")
                    .font(.subheadline)
                    .fontWeight(.semibold)
                    .foregroundColor(.white)
                    .padding(.horizontal, 16)
                    .padding(.vertical, 8)
                    .background(EtridColors.primary)
                    .cornerRadius(8)
            }
        }
        .padding(.vertical, 4)
    }
}

// MARK: - Buy Token View

struct BuyTokenView: View {
    let token: Token
    @Environment(\.dismiss) var dismiss
    @EnvironmentObject var walletManager: EtridWalletManager

    @State private var amount = ""
    @State private var paymentMethod: PaymentMethod = .etrid
    @State private var isProcessing = false
    @State private var showSuccess = false
    @State private var errorMessage: String?

    enum PaymentMethod: String, CaseIterable {
        case etrid = "ETR"
        case card = "Card"
        case bank = "Bank Transfer"

        var icon: String {
            switch self {
            case .etrid: return "bitcoinsign.circle.fill"
            case .card: return "creditcard.fill"
            case .bank: return "building.columns.fill"
            }
        }
    }

    var body: some View {
        NavigationStack {
            ScrollView {
                VStack(spacing: 24) {
                    // Token Info
                    VStack(spacing: 12) {
                        TokenBadge(token: token)
                            .scaleEffect(1.5)

                        Text(token.name)
                            .font(.title2)
                            .fontWeight(.bold)

                        Text(token.symbol)
                            .font(.subheadline)
                            .foregroundColor(.secondary)
                    }
                    .padding()

                    // Amount Input
                    VStack(alignment: .leading, spacing: 8) {
                        Text("Amount to Buy")
                            .font(.subheadline)
                            .foregroundColor(.secondary)

                        HStack {
                            TextField("0.00", text: $amount)
                                .font(.system(size: 32, weight: .bold, design: .rounded))
                                #if os(iOS)
                                .keyboardType(.decimalPad)
                                #endif

                            Text(token.symbol)
                                .font(.title2)
                                .foregroundColor(.secondary)
                        }
                        .padding()
                        .background(EtridColors.cardBackground)
                        .cornerRadius(12)
                    }

                    // Payment Method
                    VStack(alignment: .leading, spacing: 8) {
                        Text("Pay With")
                            .font(.subheadline)
                            .foregroundColor(.secondary)

                        ForEach(PaymentMethod.allCases, id: \.self) { method in
                            PaymentMethodRow(
                                method: method,
                                isSelected: paymentMethod == method,
                                onSelect: { paymentMethod = method }
                            )
                        }
                    }

                    // Price Estimate
                    if let amountDecimal = Decimal(string: amount), amountDecimal > 0 {
                        PriceEstimateView(token: token, amount: amountDecimal)
                    }

                    Spacer(minLength: 20)

                    // Buy Button
                    Button(action: processPurchase) {
                        HStack {
                            if isProcessing {
                                ProgressView()
                                    .tint(.white)
                            } else {
                                Image(systemName: "cart.fill")
                                Text("Buy \(token.symbol)")
                            }
                        }
                        .font(.headline)
                        .foregroundColor(.white)
                        .frame(maxWidth: .infinity)
                        .padding()
                        .background(isValid ? EtridColors.primary : Color.gray)
                        .cornerRadius(16)
                    }
                    .disabled(!isValid || isProcessing)

                    if let error = errorMessage {
                        Text(error)
                            .font(.caption)
                            .foregroundColor(.red)
                    }
                }
                .padding()
            }
            .navigationTitle("Buy \(token.symbol)")
            #if os(iOS)
            .navigationBarTitleDisplayMode(.inline)
            #endif
            .toolbar {
                ToolbarItem(placement: .cancellationAction) {
                    Button("Cancel") { dismiss() }
                }
            }
            .alert("Purchase Successful!", isPresented: $showSuccess) {
                Button("Done") { dismiss() }
            } message: {
                Text("You have successfully purchased \(amount) \(token.symbol)")
            }
        }
    }

    private var isValid: Bool {
        guard let amountDecimal = Decimal(string: amount),
              amountDecimal > 0 else {
            return false
        }
        return true
    }

    private func processPurchase() {
        isProcessing = true
        errorMessage = nil

        // Simulate purchase
        DispatchQueue.main.asyncAfter(deadline: .now() + 2) {
            isProcessing = false
            showSuccess = true
        }
    }
}

// MARK: - Payment Method Row

struct PaymentMethodRow: View {
    let method: BuyTokenView.PaymentMethod
    let isSelected: Bool
    let onSelect: () -> Void

    var body: some View {
        Button(action: onSelect) {
            HStack {
                Image(systemName: method.icon)
                    .font(.title2)
                    .foregroundColor(isSelected ? EtridColors.primary : .secondary)
                    .frame(width: 40)

                Text(method.rawValue)
                    .foregroundColor(.primary)

                Spacer()

                if isSelected {
                    Image(systemName: "checkmark.circle.fill")
                        .foregroundColor(EtridColors.primary)
                }
            }
            .padding()
            .background(EtridColors.cardBackground)
            .cornerRadius(12)
            .overlay(
                RoundedRectangle(cornerRadius: 12)
                    .stroke(isSelected ? EtridColors.primary : Color.clear, lineWidth: 2)
            )
        }
    }
}

// MARK: - Price Estimate View

struct PriceEstimateView: View {
    let token: Token
    let amount: Decimal

    var body: some View {
        VStack(spacing: 12) {
            HStack {
                Text("Price per \(token.symbol)")
                    .foregroundColor(.secondary)
                Spacer()
                Text("$\(estimatedPrice, specifier: "%.2f")")
            }

            Divider()

            HStack {
                Text("Total Cost")
                    .fontWeight(.semibold)
                Spacer()
                Text("$\(totalCost, specifier: "%.2f")")
                    .fontWeight(.bold)
                    .foregroundColor(EtridColors.primary)
            }
        }
        .padding()
        .background(EtridColors.cardBackground)
        .cornerRadius(12)
    }

    private var estimatedPrice: Double {
        // Placeholder prices - In production, use PriceService
        switch token.symbol {
        case "ETR": return 0.15          // Native ËTRID
        case "wETR": return 0.15         // Wrapped ËTRID (same price across all chains)
        case "EDSC": return 1.0          // ËTRID stablecoin
        case "bSOL": return 150.0
        case "bBNB": return 300.0
        case "bXRP": return 0.50
        case "bTRX": return 0.10
        case "bADA": return 0.35
        case "bXLM": return 0.12
        case "bDOGE": return 0.08
        case "USDT", "USDC": return 1.0
        default: return 0.15  // Default to ETR price
        }
    }

    private var totalCost: Double {
        return (amount as NSDecimalNumber).doubleValue * estimatedPrice
    }
}

// MARK: - Bridge View

struct BridgeView: View {
    @EnvironmentObject var walletManager: EtridWalletManager
    @Environment(\.dismiss) var dismiss

    @State private var selectedChain: SupportedChain = .solana
    @State private var bridgeDirection: BridgeDirection = .deposit
    @State private var amount = ""
    @State private var isLoading = false
    @State private var depositInfo: BridgeDepositInfo?
    @State private var errorMessage: String?

    enum BridgeDirection: String, CaseIterable {
        case deposit = "Deposit"
        case withdraw = "Withdraw"
    }

    var body: some View {
        NavigationStack {
            ScrollView {
                VStack(spacing: 24) {
                    // Direction Selector
                    Picker("Direction", selection: $bridgeDirection) {
                        ForEach(BridgeDirection.allCases, id: \.self) { direction in
                            Text(direction.rawValue).tag(direction)
                        }
                    }
                    .pickerStyle(.segmented)

                    // Chain Selector
                    ChainSelectorView(selectedChain: $selectedChain)

                    // Amount Input
                    AmountInputView(
                        amount: $amount,
                        token: bridgeToken,
                        balance: bridgeDirection == .withdraw ? currentBalance : 0
                    )

                    // Fee Info
                    if let config = bridgeConfig {
                        BridgeFeeInfoView(config: config, amount: Decimal(string: amount) ?? 0)
                    }

                    // Deposit Address (for deposits)
                    if bridgeDirection == .deposit, let info = depositInfo {
                        BridgeDepositInfoView(info: info)
                    }

                    Spacer(minLength: 20)

                    // Action Button
                    Button(action: executeBridge) {
                        HStack {
                            if isLoading {
                                ProgressView()
                                    .tint(.white)
                            } else {
                                Image(systemName: bridgeDirection == .deposit ? "arrow.right.circle.fill" : "arrow.left.circle.fill")
                                Text(bridgeDirection == .deposit ? "Get Deposit Address" : "Withdraw")
                            }
                        }
                        .font(.headline)
                        .foregroundColor(.white)
                        .frame(maxWidth: .infinity)
                        .padding()
                        .background(isValid ? EtridColors.primary : Color.gray)
                        .cornerRadius(16)
                    }
                    .disabled(!isValid || isLoading)

                    if let error = errorMessage {
                        Text(error)
                            .font(.caption)
                            .foregroundColor(.red)
                    }
                }
                .padding()
            }
            .navigationTitle("Bridge")
            #if os(iOS)
            .navigationBarTitleDisplayMode(.inline)
            #endif
            .toolbar {
                ToolbarItem(placement: .cancellationAction) {
                    Button("Cancel") { dismiss() }
                }
            }
        }
    }

    private var bridgeToken: Token {
        switch selectedChain {
        case .solana: return .bridgedSOL
        case .bnbChain: return .bridgedBNB
        case .xrp: return .bridgedXRP
        case .tron: return .bridgedTRX
        case .cardano: return .bridgedADA
        case .stellar: return .bridgedXLM
        case .dogecoin: return .bridgedDOGE
        default: return .etrid
        }
    }

    private var currentBalance: Decimal {
        walletManager.selectedAccount?.balances.first(where: { $0.token == bridgeToken })?.amount ?? 0
    }

    private var bridgeConfig: BridgeConfig? {
        // Would come from bridge service
        nil
    }

    private var isValid: Bool {
        if bridgeDirection == .deposit {
            return true
        } else {
            guard let amountDecimal = Decimal(string: amount),
                  amountDecimal > 0,
                  amountDecimal <= currentBalance else {
                return false
            }
            return true
        }
    }

    private func executeBridge() {
        isLoading = true
        errorMessage = nil

        Task {
            do {
                guard let bridge = walletManager.getBridgeService(),
                      let address = walletManager.selectedAccount?.address else {
                    throw BridgeError.serviceNotAvailable
                }

                if bridgeDirection == .deposit {
                    depositInfo = try await bridge.getDepositAddress(for: selectedChain, etridAddress: address)
                } else {
                    // Execute withdrawal
                    guard let amountDecimal = Decimal(string: amount) else {
                        throw BridgeError.invalidAmount("Invalid amount")
                    }
                    _ = try await bridge.requestWithdrawal(
                        targetChain: selectedChain,
                        targetAddress: "", // Would need external address input
                        amount: amountDecimal,
                        fromAddress: address
                    )
                }
            } catch {
                errorMessage = error.localizedDescription
            }

            isLoading = false
        }
    }
}

// MARK: - Chain Selector

struct ChainSelectorView: View {
    @Binding var selectedChain: SupportedChain

    private let bridgeableChains: [SupportedChain] = [
        .solana, .bnbChain, .xrp, .tron, .cardano, .stellar, .dogecoin
    ]

    var body: some View {
        VStack(alignment: .leading, spacing: 8) {
            Text("Select Chain")
                .font(.subheadline)
                .foregroundColor(.secondary)

            ScrollView(.horizontal, showsIndicators: false) {
                HStack(spacing: 12) {
                    ForEach(bridgeableChains, id: \.self) { chain in
                        ChainButton(
                            chain: chain,
                            isSelected: selectedChain == chain,
                            onSelect: { selectedChain = chain }
                        )
                    }
                }
            }
        }
    }
}

struct ChainButton: View {
    let chain: SupportedChain
    let isSelected: Bool
    let onSelect: () -> Void

    var body: some View {
        Button(action: onSelect) {
            VStack(spacing: 8) {
                Circle()
                    .fill(chainColor)
                    .frame(width: 48, height: 48)
                    .overlay(
                        Text(chain.symbol.prefix(1))
                            .font(.headline)
                            .fontWeight(.bold)
                            .foregroundColor(.white)
                    )
                    .overlay(
                        Circle()
                            .stroke(isSelected ? EtridColors.primary : Color.clear, lineWidth: 3)
                    )

                Text(chain.symbol)
                    .font(.caption)
                    .fontWeight(isSelected ? .semibold : .regular)
            }
        }
        .buttonStyle(.plain)
    }

    private var chainColor: Color {
        switch chain {
        case .solana: return .purple
        case .bnbChain: return .yellow
        case .xrp: return .blue
        case .tron: return .red
        case .cardano: return Color(red: 0.1, green: 0.3, blue: 0.6)
        case .stellar: return .black
        case .dogecoin: return .orange
        default: return .gray
        }
    }
}

// MARK: - Bridge Fee Info View

struct BridgeFeeInfoView: View {
    let config: BridgeConfig
    let amount: Decimal

    var body: some View {
        VStack(spacing: 12) {
            InfoRow(label: "Fee", value: "\(config.feePercent)%")
            InfoRow(label: "Min Amount", value: "\(config.minAmount) \(config.chain.symbol)")
            InfoRow(label: "Max Amount", value: "\(config.maxAmount) \(config.chain.symbol)")
            InfoRow(label: "Confirmations", value: "\(config.requiredConfirmations)")
            InfoRow(label: "Est. Time", value: "\(config.estimatedTime / 60) min")
        }
        .padding()
        .background(EtridColors.cardBackground)
        .cornerRadius(12)
    }
}

struct InfoRow: View {
    let label: String
    let value: String

    var body: some View {
        HStack {
            Text(label)
                .foregroundColor(.secondary)
            Spacer()
            Text(value)
                .fontWeight(.medium)
        }
    }
}

// MARK: - Bridge Deposit Info View

struct BridgeDepositInfoView: View {
    let info: BridgeDepositInfo

    var body: some View {
        VStack(spacing: 16) {
            Text("Deposit Address")
                .font(.headline)

            QRCodeView(content: info.depositAddress)
                .frame(width: 150, height: 150)
                .padding()
                .background(Color.white)
                .cornerRadius(12)

            Text(info.depositAddress)
                .font(.system(.caption, design: .monospaced))
                .multilineTextAlignment(.center)
                .padding()
                .background(EtridColors.cardBackground)
                .cornerRadius(8)

            if let memo = info.memo {
                VStack(spacing: 4) {
                    Text("Memo/Tag (Required)")
                        .font(.caption)
                        .foregroundColor(.orange)
                    Text(memo)
                        .font(.system(.body, design: .monospaced))
                        .fontWeight(.semibold)
                }
            }

            Button(action: copyAddress) {
                HStack {
                    Image(systemName: "doc.on.doc")
                    Text("Copy Address")
                }
                .font(.subheadline)
                .foregroundColor(EtridColors.primary)
            }
        }
        .padding()
        .background(EtridColors.cardBackground.opacity(0.5))
        .cornerRadius(16)
    }

    private func copyAddress() {
        #if os(iOS)
        UIPasteboard.general.string = info.depositAddress
        #elseif os(macOS)
        NSPasteboard.general.clearContents()
        NSPasteboard.general.setString(info.depositAddress, forType: .string)
        #endif
    }
}

// MARK: - Bridge Error Extension

extension BridgeError {
    static var serviceNotAvailable: BridgeError {
        .transactionFailed("Bridge service not available")
    }
}
