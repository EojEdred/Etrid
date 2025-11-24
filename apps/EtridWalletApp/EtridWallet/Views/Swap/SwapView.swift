//
//  SwapView.swift
//  EtridWallet
//
//  Token swap interface
//

import SwiftUI

struct SwapView: View {
    @StateObject private var viewModel = SwapViewModel()
    @Environment(\.dismiss) private var dismiss
    @State private var showTokenSelector = false
    @State private var selectingTokenType: TokenSelectionType = .tokenIn
    @State private var showSlippageSettings = false
    @State private var showConfirmation = false

    var body: some View {
        NavigationStack {
            ScrollView {
                VStack(spacing: 20) {
                    // From Token Section
                    VStack(alignment: .leading, spacing: 8) {
                        Text("From")
                            .font(.subheadline)
                            .foregroundColor(.secondary)

                        HStack {
                            Button(action: {
                                selectingTokenType = .tokenIn
                                showTokenSelector = true
                            }) {
                                HStack {
                                    Circle()
                                        .fill(Color.etridBlue.opacity(0.2))
                                        .frame(width: 32, height: 32)
                                        .overlay(
                                            Text(viewModel.tokenIn.symbol.prefix(1))
                                                .font(.caption)
                                                .foregroundColor(.etridBlue)
                                        )

                                    Text(viewModel.tokenIn.symbol)
                                        .font(.headline)

                                    Image(systemName: "chevron.down")
                                        .font(.caption)
                                        .foregroundColor(.secondary)
                                }
                                .padding(.horizontal, 12)
                                .padding(.vertical, 8)
                                .background(Color(.systemGray6))
                                .cornerRadius(20)
                            }
                            .foregroundColor(.primary)

                            Spacer()

                            VStack(alignment: .trailing, spacing: 4) {
                                TextField("0.0", text: $viewModel.amountIn)
                                    .keyboardType(.decimalPad)
                                    .multilineTextAlignment(.trailing)
                                    .font(.title2)
                                    .onChange(of: viewModel.amountIn) { _, newValue in
                                        viewModel.calculateSwap()
                                    }

                                if let balance = viewModel.tokenInBalance {
                                    HStack(spacing: 4) {
                                        Text("Balance: \(balance)")
                                            .font(.caption)
                                            .foregroundColor(.secondary)

                                        Button("Max") {
                                            viewModel.setMaxAmount()
                                        }
                                        .font(.caption.bold())
                                        .foregroundColor(.etridBlue)
                                    }
                                }
                            }
                        }
                        .padding()
                        .background(Color(.systemBackground))
                        .cornerRadius(16)
                        .shadow(color: Color.black.opacity(0.05), radius: 5, x: 0, y: 2)
                    }

                    // Swap Direction Button
                    Button(action: viewModel.swapTokens) {
                        Circle()
                            .fill(Color.etridBlue)
                            .frame(width: 40, height: 40)
                            .overlay(
                                Image(systemName: "arrow.up.arrow.down")
                                    .foregroundColor(.white)
                            )
                    }

                    // To Token Section
                    VStack(alignment: .leading, spacing: 8) {
                        Text("To (estimated)")
                            .font(.subheadline)
                            .foregroundColor(.secondary)

                        HStack {
                            Button(action: {
                                selectingTokenType = .tokenOut
                                showTokenSelector = true
                            }) {
                                HStack {
                                    Circle()
                                        .fill(Color.etridGreen.opacity(0.2))
                                        .frame(width: 32, height: 32)
                                        .overlay(
                                            Text(viewModel.tokenOut.symbol.prefix(1))
                                                .font(.caption)
                                                .foregroundColor(.etridGreen)
                                        )

                                    Text(viewModel.tokenOut.symbol)
                                        .font(.headline)

                                    Image(systemName: "chevron.down")
                                        .font(.caption)
                                        .foregroundColor(.secondary)
                                }
                                .padding(.horizontal, 12)
                                .padding(.vertical, 8)
                                .background(Color(.systemGray6))
                                .cornerRadius(20)
                            }
                            .foregroundColor(.primary)

                            Spacer()

                            VStack(alignment: .trailing, spacing: 4) {
                                if viewModel.isCalculating {
                                    ProgressView()
                                } else {
                                    Text(viewModel.amountOut)
                                        .font(.title2)
                                        .fontWeight(.semibold)
                                }

                                if let balance = viewModel.tokenOutBalance {
                                    Text("Balance: \(balance)")
                                        .font(.caption)
                                        .foregroundColor(.secondary)
                                }
                            }
                        }
                        .padding()
                        .background(Color(.systemBackground))
                        .cornerRadius(16)
                        .shadow(color: Color.black.opacity(0.05), radius: 5, x: 0, y: 2)
                    }

                    // Swap Details
                    if viewModel.quote != nil {
                        VStack(spacing: 12) {
                            SwapDetailRow(title: "Exchange Rate", value: viewModel.exchangeRate)
                            SwapDetailRow(title: "Price Impact", value: viewModel.priceImpactText, valueColor: viewModel.isPriceImpactHigh ? .etridRed : .primary)
                            SwapDetailRow(title: "Minimum Received", value: viewModel.minimumReceived)
                            SwapDetailRow(title: "Slippage Tolerance", value: viewModel.slippage.formattedTolerance, isButton: true) {
                                showSlippageSettings = true
                            }
                            SwapDetailRow(title: "Est. Gas Fee", value: viewModel.estimatedGas)
                        }
                        .padding()
                        .background(Color(.systemGray6))
                        .cornerRadius(12)
                    }

                    // Route (if available)
                    if !viewModel.route.isEmpty {
                        VStack(alignment: .leading, spacing: 8) {
                            Text("Route")
                                .font(.subheadline)
                                .foregroundColor(.secondary)

                            HStack(spacing: 4) {
                                ForEach(viewModel.route, id: \.self) { token in
                                    Text(token)
                                        .font(.caption)
                                        .padding(.horizontal, 8)
                                        .padding(.vertical, 4)
                                        .background(Color.etridBlue.opacity(0.1))
                                        .cornerRadius(6)

                                    if token != viewModel.route.last {
                                        Image(systemName: "arrow.right")
                                            .font(.caption)
                                            .foregroundColor(.secondary)
                                    }
                                }
                            }
                        }
                        .padding()
                        .background(Color(.systemGray6))
                        .cornerRadius(12)
                    }

                    // Warning for high price impact
                    if viewModel.isPriceImpactHigh {
                        HStack(spacing: 12) {
                            Image(systemName: "exclamationmark.triangle.fill")
                                .foregroundColor(.etridOrange)

                            VStack(alignment: .leading, spacing: 4) {
                                Text("High Price Impact")
                                    .font(.subheadline.bold())
                                Text("This trade will significantly affect the market price")
                                    .font(.caption)
                                    .foregroundColor(.secondary)
                            }

                            Spacer()
                        }
                        .padding()
                        .background(Color.etridOrange.opacity(0.1))
                        .cornerRadius(12)
                    }

                    // Swap Button
                    Button(action: {
                        showConfirmation = true
                    }) {
                        HStack {
                            if viewModel.isSwapping {
                                ProgressView()
                                    .progressViewStyle(CircularProgressViewStyle(tint: .white))
                            }

                            Text(viewModel.swapButtonText)
                                .fontWeight(.semibold)
                        }
                        .frame(maxWidth: .infinity)
                        .padding()
                        .background(viewModel.canSwap ? Color.etridBlue : Color.gray)
                        .foregroundColor(.white)
                        .cornerRadius(12)
                    }
                    .disabled(!viewModel.canSwap || viewModel.isSwapping)

                    Spacer()
                }
                .padding()
            }
            .navigationTitle("Swap")
            .navigationBarTitleDisplayMode(.inline)
            .toolbar {
                ToolbarItem(placement: .cancellationAction) {
                    Button("Close") {
                        dismiss()
                    }
                }

                ToolbarItem(placement: .navigationBarTrailing) {
                    Button(action: { showSlippageSettings = true }) {
                        Image(systemName: "gearshape.fill")
                    }
                }
            }
            .sheet(isPresented: $showTokenSelector) {
                TokenSelectorView(selectedToken: selectingTokenType == .tokenIn ? $viewModel.tokenIn : $viewModel.tokenOut)
            }
            .sheet(isPresented: $showSlippageSettings) {
                SlippageSettingsView(settings: $viewModel.slippage)
            }
            .sheet(isPresented: $showConfirmation) {
                SwapConfirmationView(viewModel: viewModel)
            }
            .alert("Swap Error", isPresented: $viewModel.showError) {
                Button("OK", role: .cancel) { }
            } message: {
                Text(viewModel.errorMessage)
            }
        }
        .task {
            await viewModel.loadBalances()
        }
    }
}

struct SwapDetailRow: View {
    let title: String
    let value: String
    var valueColor: Color = .primary
    var isButton: Bool = false
    var action: (() -> Void)?

    var body: some View {
        HStack {
            Text(title)
                .font(.subheadline)
                .foregroundColor(.secondary)

            Spacer()

            if isButton {
                Button(action: { action?() }) {
                    HStack(spacing: 4) {
                        Text(value)
                            .font(.subheadline)
                            .foregroundColor(.etridBlue)
                        Image(systemName: "chevron.right")
                            .font(.caption)
                            .foregroundColor(.etridBlue)
                    }
                }
            } else {
                Text(value)
                    .font(.subheadline)
                    .fontWeight(.medium)
                    .foregroundColor(valueColor)
            }
        }
    }
}

struct SwapConfirmationView: View {
    @ObservedObject var viewModel: SwapViewModel
    @Environment(\.dismiss) private var dismiss

    var body: some View {
        NavigationStack {
            VStack(spacing: 24) {
                // Header
                VStack(spacing: 12) {
                    Image(systemName: "arrow.left.arrow.right.circle.fill")
                        .font(.system(size: 60))
                        .foregroundColor(.etridBlue)

                    Text("Confirm Swap")
                        .font(.title2.bold())
                }
                .padding(.top, 30)

                // Swap Summary
                VStack(spacing: 16) {
                    HStack {
                        VStack(alignment: .leading) {
                            Text("You Pay")
                                .font(.caption)
                                .foregroundColor(.secondary)
                            Text("\(viewModel.amountIn) \(viewModel.tokenIn.symbol)")
                                .font(.title3.bold())
                        }
                        Spacer()
                        Image(systemName: "arrow.right")
                            .foregroundColor(.secondary)
                        Spacer()
                        VStack(alignment: .trailing) {
                            Text("You Receive")
                                .font(.caption)
                                .foregroundColor(.secondary)
                            Text("\(viewModel.amountOut) \(viewModel.tokenOut.symbol)")
                                .font(.title3.bold())
                        }
                    }
                    .padding()
                    .background(Color(.systemGray6))
                    .cornerRadius(12)

                    // Details
                    VStack(spacing: 12) {
                        SwapDetailRow(title: "Exchange Rate", value: viewModel.exchangeRate)
                        SwapDetailRow(title: "Price Impact", value: viewModel.priceImpactText)
                        SwapDetailRow(title: "Minimum Received", value: viewModel.minimumReceived)
                        SwapDetailRow(title: "Gas Fee", value: viewModel.estimatedGas)
                    }
                    .padding()
                    .background(Color(.systemBackground))
                    .cornerRadius(12)
                }
                .padding(.horizontal)

                Spacer()

                // Confirm Button
                VStack(spacing: 12) {
                    Button(action: {
                        Task {
                            await viewModel.executeSwap()
                            if !viewModel.showError {
                                dismiss()
                            }
                        }
                    }) {
                        Text("Confirm Swap")
                            .fontWeight(.semibold)
                            .frame(maxWidth: .infinity)
                            .padding()
                            .background(Color.etridBlue)
                            .foregroundColor(.white)
                            .cornerRadius(12)
                    }
                    .disabled(viewModel.isSwapping)

                    Button("Cancel") {
                        dismiss()
                    }
                    .foregroundColor(.secondary)
                }
                .padding()
            }
            .navigationBarTitleDisplayMode(.inline)
        }
    }
}

enum TokenSelectionType {
    case tokenIn
    case tokenOut
}

@MainActor
class SwapViewModel: ObservableObject {
    @Published var tokenIn: Token = Token(symbol: "ETH", name: "Ethereum", decimals: 18, isNative: true)
    @Published var tokenOut: Token = Token(symbol: "USDC", name: "USD Coin", contractAddress: "0xA0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48", decimals: 6)
    @Published var amountIn: String = ""
    @Published var amountOut: String = "0.0"
    @Published var tokenInBalance: String?
    @Published var tokenOutBalance: String?
    @Published var slippage: SlippageSettings = .default
    @Published var quote: SwapQuote?
    @Published var isCalculating = false
    @Published var isSwapping = false
    @Published var showError = false
    @Published var errorMessage = ""

    var exchangeRate: String {
        guard let quote = quote else { return "-" }
        return "1 \(tokenIn.symbol) = \(quote.amountOut) \(tokenOut.symbol)"
    }

    var priceImpactText: String {
        guard let quote = quote else { return "-" }
        return String(format: "%.2f%%", quote.priceImpact)
    }

    var isPriceImpactHigh: Bool {
        guard let quote = quote else { return false }
        return quote.priceImpact > 5.0
    }

    var minimumReceived: String {
        guard let quote = quote, let outAmount = Double(quote.amountOut) else { return "-" }
        let minimum = outAmount * (1 - slippage.tolerance / 100)
        return String(format: "%.6f %@", minimum, tokenOut.symbol)
    }

    var estimatedGas: String {
        guard let quote = quote else { return "-" }
        let gasEth = (Double(quote.gasEstimate) ?? 0) / 1e18
        return String(format: "%.6f ETH", gasEth)
    }

    var route: [String] {
        quote?.route ?? []
    }

    var canSwap: Bool {
        guard let amount = Double(amountIn), amount > 0 else { return false }
        return quote != nil && !isCalculating
    }

    var swapButtonText: String {
        if amountIn.isEmpty || Double(amountIn) == 0 {
            return "Enter Amount"
        } else if isCalculating {
            return "Calculating..."
        } else if quote == nil {
            return "Get Quote"
        } else {
            return "Swap"
        }
    }

    func loadBalances() async {
        // Mock balances - replace with actual balance fetching
        tokenInBalance = "5.2850"
        tokenOutBalance = "1250.00"
    }

    func calculateSwap() {
        guard let amount = Double(amountIn), amount > 0 else {
            amountOut = "0.0"
            quote = nil
            return
        }

        isCalculating = true

        // Simulate API call with delay
        Task {
            try? await Task.sleep(nanoseconds: 500_000_000) // 0.5 seconds

            // Mock quote calculation
            let mockRate = 2500.0 // 1 ETH = 2500 USDC
            let output = amount * mockRate
            let priceImpact = amount > 1.0 ? (amount - 1.0) * 0.5 : 0.15

            quote = SwapQuote(
                amountIn: amountIn,
                amountOut: String(format: "%.2f", output),
                priceImpact: priceImpact,
                fee: "0.003",
                gasEstimate: "21000000000000000", // 0.021 ETH
                route: [tokenIn.symbol, tokenOut.symbol]
            )

            amountOut = String(format: "%.2f", output)
            isCalculating = false
        }
    }

    func setMaxAmount() {
        guard let balance = tokenInBalance else { return }
        amountIn = balance
        calculateSwap()
    }

    func swapTokens() {
        let temp = tokenIn
        tokenIn = tokenOut
        tokenOut = temp

        let tempBalance = tokenInBalance
        tokenInBalance = tokenOutBalance
        tokenOutBalance = tempBalance

        amountIn = ""
        amountOut = "0.0"
        quote = nil
    }

    func executeSwap() async {
        isSwapping = true

        // Simulate swap execution
        do {
            try await Task.sleep(nanoseconds: 2_000_000_000) // 2 seconds

            // Mock success
            await loadBalances()
            amountIn = ""
            amountOut = "0.0"
            quote = nil
        } catch {
            errorMessage = "Failed to execute swap: \(error.localizedDescription)"
            showError = true
        }

        isSwapping = false
    }
}

#Preview {
    SwapView()
}
