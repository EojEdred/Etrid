import SwiftUI

// MARK: - Swap View

/// DEX swap interface for trading tokens
public struct SwapView: View {
    @StateObject private var dexService = DEXService.shared
    @StateObject private var balanceService = BalanceService.shared
    @StateObject private var priceService = PriceService.shared

    @State private var fromToken: SwapToken = .eth
    @State private var toToken: SwapToken = .wETR
    @State private var fromAmount: String = ""
    @State private var selectedChain: ContractAddresses.Network = .ethereum
    @State private var slippage: Double = 0.5
    @State private var showingSettings = false
    @State private var showingTokenPicker = false
    @State private var isSelectingFromToken = true
    @State private var showingConfirmation = false
    @State private var transactionHash: String?
    @State private var errorMessage: String?

    public init() {}

    public var body: some View {
        NavigationStack {
            ScrollView {
                VStack(spacing: 20) {
                    // Chain Selector
                    chainSelector

                    // Swap Card
                    swapCard

                    // Quote Details
                    if let quote = dexService.currentQuote {
                        quoteDetails(quote: quote)
                    }

                    // Swap Button
                    swapButton

                    // Transaction Result
                    if let hash = transactionHash {
                        transactionResult(hash: hash)
                    }

                    // Error Message
                    if let error = errorMessage {
                        errorBanner(message: error)
                    }
                }
                .padding()
            }
            .background(Color(.systemGroupedBackground))
            .navigationTitle("Swap")
            .toolbar {
                ToolbarItem(placement: .navigationBarTrailing) {
                    Button(action: { showingSettings = true }) {
                        Image(systemName: "gearshape")
                    }
                }
            }
            .sheet(isPresented: $showingSettings) {
                slippageSettings
            }
            .sheet(isPresented: $showingTokenPicker) {
                tokenPicker
            }
            .sheet(isPresented: $showingConfirmation) {
                confirmationSheet
            }
        }
    }

    // MARK: - Chain Selector

    private var chainSelector: some View {
        ScrollView(.horizontal, showsIndicators: false) {
            HStack(spacing: 12) {
                ForEach(supportedChains, id: \.self) { chain in
                    SwapChainButton(
                        chain: chain,
                        isSelected: selectedChain == chain
                    ) {
                        selectedChain = chain
                        updateTokensForChain()
                    }
                }
            }
            .padding(.horizontal, 4)
        }
    }

    private var supportedChains: [ContractAddresses.Network] {
        [.ethereum, .bnbChain, .polygon, .arbitrum, .solana]
    }

    // MARK: - Swap Card

    private var swapCard: some View {
        VStack(spacing: 0) {
            // From Token
            tokenInputRow(
                label: "From",
                token: fromToken,
                amount: $fromAmount,
                isEditable: true,
                onTokenTap: {
                    isSelectingFromToken = true
                    showingTokenPicker = true
                }
            )

            // Swap Arrow
            swapArrowButton

            // To Token
            tokenInputRow(
                label: "To",
                token: toToken,
                amount: .constant(estimatedOutput),
                isEditable: false,
                onTokenTap: {
                    isSelectingFromToken = false
                    showingTokenPicker = true
                }
            )
        }
        .background(Color(.systemBackground))
        .cornerRadius(16)
        .shadow(color: .black.opacity(0.05), radius: 8, x: 0, y: 2)
    }

    private func tokenInputRow(
        label: String,
        token: SwapToken,
        amount: Binding<String>,
        isEditable: Bool,
        onTokenTap: @escaping () -> Void
    ) -> some View {
        VStack(alignment: .leading, spacing: 8) {
            Text(label)
                .font(.caption)
                .foregroundColor(.secondary)

            HStack {
                if isEditable {
                    TextField("0.0", text: amount)
                        .font(.system(size: 28, weight: .semibold))
                        .keyboardType(.decimalPad)
                        .onChange(of: fromAmount) { _, _ in
                            fetchQuote()
                        }
                } else {
                    Text(amount.wrappedValue.isEmpty ? "0.0" : amount.wrappedValue)
                        .font(.system(size: 28, weight: .semibold))
                        .foregroundColor(.primary)
                }

                Spacer()

                Button(action: onTokenTap) {
                    HStack(spacing: 6) {
                        Circle()
                            .fill(token.color)
                            .frame(width: 24, height: 24)
                            .overlay(
                                Text(token.symbol.prefix(1))
                                    .font(.caption2.bold())
                                    .foregroundColor(.white)
                            )

                        Text(token.symbol)
                            .font(.headline)

                        Image(systemName: "chevron.down")
                            .font(.caption)
                    }
                    .padding(.horizontal, 12)
                    .padding(.vertical, 8)
                    .background(Color(.secondarySystemBackground))
                    .cornerRadius(20)
                }
                .buttonStyle(.plain)
            }

            HStack {
                if let balance = getBalance(for: token) {
                    Text("Balance: \(balance)")
                        .font(.caption)
                        .foregroundColor(.secondary)

                    if isEditable {
                        Button("MAX") {
                            fromAmount = balance
                            fetchQuote()
                        }
                        .font(.caption.bold())
                        .foregroundColor(.blue)
                    }
                }

                Spacer()

                if let price = priceService.getPrice(for: token.symbol) {
                    let value = (Decimal(string: amount.wrappedValue) ?? 0) * price
                    Text("~$\(formatDecimal(value))")
                        .font(.caption)
                        .foregroundColor(.secondary)
                }
            }
        }
        .padding()
    }

    private var swapArrowButton: some View {
        Button(action: swapTokens) {
            ZStack {
                Circle()
                    .fill(Color(.systemBackground))
                    .frame(width: 40, height: 40)
                    .shadow(color: .black.opacity(0.1), radius: 4, x: 0, y: 2)

                Image(systemName: "arrow.up.arrow.down")
                    .font(.system(size: 16, weight: .semibold))
                    .foregroundColor(.blue)
            }
        }
        .offset(y: -4)
        .zIndex(1)
    }

    // MARK: - Quote Details

    private func quoteDetails(quote: SwapQuote) -> some View {
        VStack(spacing: 12) {
            HStack {
                Text("Rate")
                    .foregroundColor(.secondary)
                Spacer()
                Text("1 \(fromToken.symbol) = \(quote.formattedRate) \(toToken.symbol)")
                    .fontWeight(.medium)
            }

            HStack {
                Text("Price Impact")
                    .foregroundColor(.secondary)
                Spacer()
                Text(quote.formattedPriceImpact)
                    .foregroundColor(quote.isHighPriceImpact ? .red : .green)
                    .fontWeight(.medium)
            }

            HStack {
                Text("Route")
                    .foregroundColor(.secondary)
                Spacer()
                Text(quote.route)
                    .fontWeight(.medium)
            }

            HStack {
                Text("Est. Gas")
                    .foregroundColor(.secondary)
                Spacer()
                Text("~\(quote.estimatedGas) units")
                    .fontWeight(.medium)
            }

            HStack {
                Text("Min. Received")
                    .foregroundColor(.secondary)
                Spacer()
                let minReceived = dexService.calculateMinimumReceived(quote: quote, slippage: slippage)
                Text("\(formatDecimal(minReceived)) \(toToken.symbol)")
                    .fontWeight(.medium)
            }
        }
        .font(.subheadline)
        .padding()
        .background(Color(.secondarySystemBackground))
        .cornerRadius(12)
    }

    // MARK: - Swap Button

    private var swapButton: some View {
        Button(action: {
            if dexService.currentQuote != nil {
                showingConfirmation = true
            } else {
                fetchQuote()
            }
        }) {
            HStack {
                if dexService.isLoadingQuote {
                    ProgressView()
                        .tint(.white)
                } else {
                    Text(dexService.currentQuote != nil ? "Swap" : "Get Quote")
                        .fontWeight(.semibold)
                }
            }
            .frame(maxWidth: .infinity)
            .padding()
            .background(isSwapEnabled ? Color.blue : Color.gray)
            .foregroundColor(.white)
            .cornerRadius(12)
        }
        .disabled(!isSwapEnabled || dexService.isLoadingQuote)
    }

    // MARK: - Transaction Result

    private func transactionResult(hash: String) -> some View {
        VStack(spacing: 12) {
            Image(systemName: "checkmark.circle.fill")
                .font(.system(size: 48))
                .foregroundColor(.green)

            Text("Transaction Submitted")
                .font(.headline)

            Text(hash)
                .font(.caption)
                .foregroundColor(.secondary)
                .lineLimit(1)
                .truncationMode(.middle)

            Button("View on Explorer") {
                openExplorer(hash: hash)
            }
            .font(.subheadline.bold())
        }
        .padding()
        .frame(maxWidth: .infinity)
        .background(Color(.secondarySystemBackground))
        .cornerRadius(12)
    }

    private func errorBanner(message: String) -> some View {
        HStack {
            Image(systemName: "exclamationmark.triangle.fill")
                .foregroundColor(.orange)

            Text(message)
                .font(.subheadline)

            Spacer()

            Button(action: { errorMessage = nil }) {
                Image(systemName: "xmark")
            }
        }
        .padding()
        .background(Color.orange.opacity(0.1))
        .cornerRadius(8)
    }

    // MARK: - Slippage Settings

    private var slippageSettings: some View {
        NavigationStack {
            Form {
                Section("Slippage Tolerance") {
                    HStack {
                        ForEach([0.1, 0.5, 1.0], id: \.self) { value in
                            Button(action: { slippage = value }) {
                                Text("\(String(format: "%.1f", value))%")
                                    .padding(.horizontal, 16)
                                    .padding(.vertical, 8)
                                    .background(slippage == value ? Color.blue : Color(.secondarySystemBackground))
                                    .foregroundColor(slippage == value ? .white : .primary)
                                    .cornerRadius(8)
                            }
                            .buttonStyle(.plain)
                        }

                        Spacer()

                        TextField("Custom", value: $slippage, format: .number)
                            .textFieldStyle(.roundedBorder)
                            .frame(width: 80)
                            .keyboardType(.decimalPad)
                    }

                    Text("Higher slippage tolerance increases the chance of a successful swap but may result in a worse rate.")
                        .font(.caption)
                        .foregroundColor(.secondary)
                }

                Section("Transaction Deadline") {
                    Text("20 minutes")
                        .foregroundColor(.secondary)
                }
            }
            .navigationTitle("Swap Settings")
            .navigationBarTitleDisplayMode(.inline)
            .toolbar {
                ToolbarItem(placement: .navigationBarTrailing) {
                    Button("Done") { showingSettings = false }
                }
            }
        }
    }

    // MARK: - Token Picker

    private var tokenPicker: some View {
        NavigationStack {
            List {
                ForEach(availableTokens) { token in
                    Button(action: {
                        if isSelectingFromToken {
                            fromToken = token
                        } else {
                            toToken = token
                        }
                        showingTokenPicker = false
                        fetchQuote()
                    }) {
                        HStack {
                            Circle()
                                .fill(token.color)
                                .frame(width: 40, height: 40)
                                .overlay(
                                    Text(token.symbol.prefix(2))
                                        .font(.caption.bold())
                                        .foregroundColor(.white)
                                )

                            VStack(alignment: .leading) {
                                Text(token.symbol)
                                    .font(.headline)
                                Text(token.name)
                                    .font(.caption)
                                    .foregroundColor(.secondary)
                            }

                            Spacer()

                            if let balance = getBalance(for: token) {
                                VStack(alignment: .trailing) {
                                    Text(balance)
                                        .font(.subheadline)
                                    if let price = priceService.getPrice(for: token.symbol) {
                                        let value = (Decimal(string: balance) ?? 0) * price
                                        Text("$\(formatDecimal(value))")
                                            .font(.caption)
                                            .foregroundColor(.secondary)
                                    }
                                }
                            }
                        }
                    }
                    .buttonStyle(.plain)
                }
            }
            .navigationTitle(isSelectingFromToken ? "Swap From" : "Swap To")
            .navigationBarTitleDisplayMode(.inline)
            .toolbar {
                ToolbarItem(placement: .navigationBarLeading) {
                    Button("Cancel") { showingTokenPicker = false }
                }
            }
        }
    }

    // MARK: - Confirmation Sheet

    private var confirmationSheet: some View {
        NavigationStack {
            VStack(spacing: 24) {
                // Swap Preview
                VStack(spacing: 16) {
                    HStack {
                        VStack {
                            Text(fromAmount)
                                .font(.title.bold())
                            Text(fromToken.symbol)
                                .foregroundColor(.secondary)
                        }

                        Image(systemName: "arrow.right")
                            .font(.title2)
                            .foregroundColor(.secondary)

                        VStack {
                            Text(estimatedOutput)
                                .font(.title.bold())
                            Text(toToken.symbol)
                                .foregroundColor(.secondary)
                        }
                    }

                    if let quote = dexService.currentQuote {
                        Divider()

                        VStack(spacing: 8) {
                            HStack {
                                Text("Rate")
                                Spacer()
                                Text("1 \(fromToken.symbol) = \(quote.formattedRate) \(toToken.symbol)")
                            }

                            HStack {
                                Text("Price Impact")
                                Spacer()
                                Text(quote.formattedPriceImpact)
                                    .foregroundColor(quote.isHighPriceImpact ? .red : .green)
                            }

                            HStack {
                                Text("Slippage")
                                Spacer()
                                Text("\(String(format: "%.1f", slippage))%")
                            }
                        }
                        .font(.subheadline)
                    }
                }
                .padding()
                .background(Color(.secondarySystemBackground))
                .cornerRadius(12)

                // Warning if high price impact
                if let quote = dexService.currentQuote, quote.isHighPriceImpact {
                    HStack {
                        Image(systemName: "exclamationmark.triangle.fill")
                            .foregroundColor(.orange)
                        Text("High price impact! Consider splitting into smaller trades.")
                            .font(.subheadline)
                    }
                    .padding()
                    .background(Color.orange.opacity(0.1))
                    .cornerRadius(8)
                }

                Spacer()

                // Confirm Button
                Button(action: executeSwap) {
                    Text("Confirm Swap")
                        .fontWeight(.semibold)
                        .frame(maxWidth: .infinity)
                        .padding()
                        .background(Color.blue)
                        .foregroundColor(.white)
                        .cornerRadius(12)
                }
            }
            .padding()
            .navigationTitle("Confirm Swap")
            .navigationBarTitleDisplayMode(.inline)
            .toolbar {
                ToolbarItem(placement: .navigationBarLeading) {
                    Button("Cancel") { showingConfirmation = false }
                }
            }
        }
    }

    // MARK: - Helpers

    private var estimatedOutput: String {
        guard let quote = dexService.currentQuote else { return "" }
        return formatDecimal(quote.toAmount)
    }

    private var isSwapEnabled: Bool {
        !fromAmount.isEmpty && (Decimal(string: fromAmount) ?? 0) > 0
    }

    private var availableTokens: [SwapToken] {
        SwapToken.tokensForChain(selectedChain)
    }

    private func swapTokens() {
        let temp = fromToken
        fromToken = toToken
        toToken = temp
        fetchQuote()
    }

    private func updateTokensForChain() {
        let tokens = SwapToken.tokensForChain(selectedChain)
        if !tokens.contains(where: { $0.id == fromToken.id }) {
            fromToken = tokens.first ?? .eth
        }
        if !tokens.contains(where: { $0.id == toToken.id }) {
            toToken = tokens.first(where: { $0.id != fromToken.id }) ?? .wETR
        }
        dexService.currentQuote = nil
    }

    private func fetchQuote() {
        guard let amount = Decimal(string: fromAmount), amount > 0 else { return }

        Task {
            do {
                _ = try await dexService.getQuote(
                    fromToken: fromToken.contractAddress ?? fromToken.symbol,
                    toToken: toToken.contractAddress ?? toToken.symbol,
                    amount: amount,
                    chain: selectedChain,
                    slippage: slippage
                )
            } catch {
                errorMessage = error.localizedDescription
            }
        }
    }

    private func executeSwap() {
        showingConfirmation = false

        // In production, this would:
        // 1. Build the swap transaction
        // 2. Request password/biometric
        // 3. Sign and broadcast

        Task {
            // Simulated success
            try? await Task.sleep(nanoseconds: 1_500_000_000)
            transactionHash = "0x" + (0..<64).map { _ in "0123456789abcdef".randomElement()! }.map(String.init).joined()
        }
    }

    private func getBalance(for token: SwapToken) -> String? {
        // Get balance from balance service
        if token.symbol == "wETR" {
            let balance = balanceService.getWETRBalance(for: selectedChain)
            return formatDecimal(balance)
        }
        return nil
    }

    private func openExplorer(hash: String) {
        let baseURL = selectedChain.explorerURL
        let url = URL(string: "\(baseURL)/tx/\(hash)")!
        // In production, open URL
    }

    private func formatDecimal(_ value: Decimal) -> String {
        let formatter = NumberFormatter()
        formatter.numberStyle = .decimal
        formatter.maximumFractionDigits = 6
        formatter.minimumFractionDigits = 2
        return formatter.string(from: value as NSDecimalNumber) ?? "0.00"
    }
}

// MARK: - Swap Chain Button

struct SwapChainButton: View {
    let chain: ContractAddresses.Network
    let isSelected: Bool
    let action: () -> Void

    var body: some View {
        Button(action: action) {
            VStack(spacing: 4) {
                Image(systemName: chainIcon)
                    .font(.title2)
                Text(chainName)
                    .font(.caption2)
            }
            .frame(width: 60, height: 60)
            .background(isSelected ? Color.blue.opacity(0.1) : Color(.secondarySystemBackground))
            .foregroundColor(isSelected ? .blue : .primary)
            .cornerRadius(12)
            .overlay(
                RoundedRectangle(cornerRadius: 12)
                    .stroke(isSelected ? Color.blue : Color.clear, lineWidth: 2)
            )
        }
        .buttonStyle(.plain)
    }

    private var chainIcon: String {
        switch chain {
        case .ethereum: return "e.circle.fill"
        case .bnbChain: return "b.circle.fill"
        case .polygon: return "p.circle.fill"
        case .arbitrum: return "a.circle.fill"
        case .solana: return "s.circle.fill"
        default: return "circle.fill"
        }
    }

    private var chainName: String {
        switch chain {
        case .ethereum: return "ETH"
        case .bnbChain: return "BSC"
        case .polygon: return "Polygon"
        case .arbitrum: return "Arb"
        case .solana: return "SOL"
        default: return chain.rawValue
        }
    }
}

// MARK: - Swap Token Model

struct SwapToken: Identifiable, Equatable {
    let id: String
    let symbol: String
    let name: String
    let contractAddress: String?
    let decimals: Int
    let color: Color

    static let eth = SwapToken(id: "eth", symbol: "ETH", name: "Ethereum", contractAddress: nil, decimals: 18, color: .blue)
    static let wETR = SwapToken(id: "wetr", symbol: "wETR", name: "Wrapped ETRID", contractAddress: nil, decimals: 18, color: .purple)
    static let bnb = SwapToken(id: "bnb", symbol: "BNB", name: "BNB", contractAddress: nil, decimals: 18, color: .yellow)
    static let matic = SwapToken(id: "matic", symbol: "MATIC", name: "Polygon", contractAddress: nil, decimals: 18, color: .purple)
    static let sol = SwapToken(id: "sol", symbol: "SOL", name: "Solana", contractAddress: nil, decimals: 9, color: .green)
    static let usdt = SwapToken(id: "usdt", symbol: "USDT", name: "Tether USD", contractAddress: nil, decimals: 6, color: .green)
    static let usdc = SwapToken(id: "usdc", symbol: "USDC", name: "USD Coin", contractAddress: nil, decimals: 6, color: .blue)

    static func tokensForChain(_ chain: ContractAddresses.Network) -> [SwapToken] {
        let wETRAddress = ContractAddresses.wrappedETR[chain]

        let wETRToken = SwapToken(
            id: "wetr-\(chain.rawValue)",
            symbol: "wETR",
            name: "Wrapped ETRID",
            contractAddress: wETRAddress,
            decimals: chain == .solana ? 9 : 18,
            color: .purple
        )

        switch chain {
        case .ethereum:
            return [.eth, wETRToken, .usdt, .usdc]
        case .bnbChain:
            return [.bnb, wETRToken, .usdt, .usdc]
        case .polygon:
            return [.matic, wETRToken, .usdt, .usdc]
        case .arbitrum:
            return [.eth, wETRToken, .usdt, .usdc]
        case .solana:
            return [.sol, wETRToken, .usdc]
        default:
            return [wETRToken]
        }
    }
}

#Preview {
    SwapView()
}
