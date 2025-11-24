//
//  BuyCryptoView.swift
//  EtridWallet
//
//  Fiat on-ramp interface for buying crypto
//

import SwiftUI

struct BuyCryptoView: View {
    @StateObject private var viewModel = BuyCryptoViewModel()
    @Environment(\.dismiss) private var dismiss
    @State private var showProviderSheet = false
    @State private var showPaymentMethodSheet = false

    var body: some View {
        NavigationStack {
            ScrollView {
                VStack(spacing: 20) {
                    // Buy/Sell Toggle
                    Picker("Transaction Type", selection: $viewModel.transactionType) {
                        Text("Buy").tag(FiatTransactionType.buy)
                        Text("Sell").tag(FiatTransactionType.sell)
                    }
                    .pickerStyle(.segmented)
                    .padding(.horizontal)

                    // Fiat Amount Section
                    VStack(alignment: .leading, spacing: 8) {
                        Text("You Pay")
                            .font(.subheadline)
                            .foregroundColor(.secondary)

                        HStack {
                            TextField("0.00", text: $viewModel.fiatAmount)
                                .keyboardType(.decimalPad)
                                .font(.title)
                                .onChange(of: viewModel.fiatAmount) { _, _ in
                                    viewModel.calculateQuote()
                                }

                            Spacer()

                            Text(viewModel.selectedCurrency)
                                .font(.headline)
                                .foregroundColor(.secondary)
                        }
                        .padding()
                        .background(Color(.systemBackground))
                        .cornerRadius(16)
                        .shadow(color: Color.black.opacity(0.05), radius: 5, x: 0, y: 2)
                    }
                    .padding(.horizontal)

                    // Exchange Icon
                    Image(systemName: "arrow.down")
                        .font(.title2)
                        .foregroundColor(.secondary)

                    // Crypto Amount Section
                    VStack(alignment: .leading, spacing: 8) {
                        Text("You Receive (estimated)")
                            .font(.subheadline)
                            .foregroundColor(.secondary)

                        HStack {
                            if viewModel.isCalculating {
                                ProgressView()
                            } else {
                                Text(viewModel.cryptoAmount)
                                    .font(.title)
                                    .fontWeight(.semibold)
                            }

                            Spacer()

                            Button(action: { viewModel.showCryptoSelector = true }) {
                                HStack {
                                    Circle()
                                        .fill(Color.etridBlue.opacity(0.2))
                                        .frame(width: 32, height: 32)
                                        .overlay(
                                            Text(viewModel.selectedToken.symbol.prefix(1))
                                                .font(.caption)
                                                .foregroundColor(.etridBlue)
                                        )

                                    Text(viewModel.selectedToken.symbol)
                                        .font(.headline)

                                    Image(systemName: "chevron.down")
                                        .font(.caption)
                                        .foregroundColor(.secondary)
                                }
                            }
                            .foregroundColor(.primary)
                        }
                        .padding()
                        .background(Color(.systemBackground))
                        .cornerRadius(16)
                        .shadow(color: Color.black.opacity(0.05), radius: 5, x: 0, y: 2)
                    }
                    .padding(.horizontal)

                    // Payment Method
                    VStack(alignment: .leading, spacing: 12) {
                        Text("Payment Method")
                            .font(.headline)

                        Button(action: { showPaymentMethodSheet = true }) {
                            HStack {
                                Image(systemName: viewModel.selectedPaymentMethod?.iconName ?? "creditcard.fill")
                                    .foregroundColor(.etridBlue)

                                if let method = viewModel.selectedPaymentMethod {
                                    VStack(alignment: .leading, spacing: 4) {
                                        Text(method.displayName)
                                            .font(.subheadline)

                                        Text(method.type.processingTime)
                                            .font(.caption)
                                            .foregroundColor(.secondary)
                                    }
                                } else {
                                    Text("Select payment method")
                                        .foregroundColor(.secondary)
                                }

                                Spacer()

                                Image(systemName: "chevron.right")
                                    .foregroundColor(.secondary)
                            }
                            .padding()
                            .background(Color(.systemGray6))
                            .cornerRadius(12)
                        }
                        .foregroundColor(.primary)
                    }
                    .padding(.horizontal)

                    // Provider Selection
                    VStack(alignment: .leading, spacing: 12) {
                        Text("Provider")
                            .font(.headline)

                        if viewModel.isLoadingProviders {
                            ProgressView()
                                .frame(maxWidth: .infinity)
                                .padding()
                        } else if viewModel.availableProviders.isEmpty {
                            Text("No providers available")
                                .foregroundColor(.secondary)
                                .frame(maxWidth: .infinity)
                                .padding()
                        } else {
                            ForEach(viewModel.availableProviders.prefix(3)) { provider in
                                ProviderCard(
                                    provider: provider,
                                    isSelected: viewModel.selectedProvider?.id == provider.id
                                ) {
                                    viewModel.selectProvider(provider)
                                }
                            }

                            if viewModel.availableProviders.count > 3 {
                                Button("View All Providers") {
                                    showProviderSheet = true
                                }
                                .font(.subheadline)
                                .foregroundColor(.etridBlue)
                                .frame(maxWidth: .infinity)
                            }
                        }
                    }
                    .padding(.horizontal)

                    // Quote Details
                    if let quote = viewModel.currentQuote {
                        VStack(spacing: 12) {
                            QuoteDetailRow(title: "Exchange Rate", value: String(format: "1 %@ = $%.2f", viewModel.selectedToken.symbol, quote.exchangeRate))
                            QuoteDetailRow(title: "Fee", value: String(format: "$%.2f", quote.fee))
                            QuoteDetailRow(title: "Total", value: String(format: "$%.2f", quote.total), isBold: true)
                            QuoteDetailRow(title: "Processing Time", value: viewModel.selectedProvider?.processingTime ?? "-")
                        }
                        .padding()
                        .background(Color(.systemGray6))
                        .cornerRadius(12)
                        .padding(.horizontal)
                    }

                    // Continue Button
                    Button(action: {
                        viewModel.proceedToCheckout()
                    }) {
                        HStack {
                            if viewModel.isProcessing {
                                ProgressView()
                                    .progressViewStyle(CircularProgressViewStyle(tint: .white))
                            }

                            Text(viewModel.transactionType == .buy ? "Buy \(viewModel.selectedToken.symbol)" : "Sell \(viewModel.selectedToken.symbol)")
                                .fontWeight(.semibold)
                        }
                        .frame(maxWidth: .infinity)
                        .padding()
                        .background(viewModel.canProceed ? Color.etridBlue : Color.gray)
                        .foregroundColor(.white)
                        .cornerRadius(12)
                    }
                    .disabled(!viewModel.canProceed || viewModel.isProcessing)
                    .padding(.horizontal)

                    // Disclaimer
                    VStack(spacing: 8) {
                        HStack {
                            Image(systemName: "info.circle")
                                .foregroundColor(.secondary)

                            Text("Provided by third-party services")
                                .font(.caption)
                                .foregroundColor(.secondary)
                        }

                        Text("Etrid Wallet does not control these services. Please review terms before proceeding.")
                            .font(.caption)
                            .foregroundColor(.secondary)
                            .multilineTextAlignment(.center)
                    }
                    .padding()
                    .background(Color(.systemGray6).opacity(0.5))
                    .cornerRadius(12)
                    .padding(.horizontal)

                    Spacer()
                }
                .padding(.vertical)
            }
            .navigationTitle(viewModel.transactionType == .buy ? "Buy Crypto" : "Sell Crypto")
            .navigationBarTitleDisplayMode(.inline)
            .toolbar {
                ToolbarItem(placement: .cancellationAction) {
                    Button("Close") {
                        dismiss()
                    }
                }
            }
            .sheet(isPresented: $showProviderSheet) {
                ProviderSelectionView(
                    providers: viewModel.availableProviders,
                    selectedProvider: $viewModel.selectedProvider
                )
            }
            .sheet(isPresented: $showPaymentMethodSheet) {
                PaymentMethodSelectionView(
                    methods: viewModel.paymentMethods,
                    selectedMethod: $viewModel.selectedPaymentMethod
                )
            }
            .sheet(isPresented: $viewModel.showCryptoSelector) {
                TokenSelectorView(selectedToken: $viewModel.selectedToken)
            }
            .alert("Transaction Error", isPresented: $viewModel.showError) {
                Button("OK", role: .cancel) { }
            } message: {
                Text(viewModel.errorMessage)
            }
        }
        .task {
            await viewModel.loadData()
        }
    }
}

struct ProviderCard: View {
    let provider: FiatProvider
    let isSelected: Bool
    let action: () -> Void

    var body: some View {
        Button(action: action) {
            HStack {
                VStack(alignment: .leading, spacing: 4) {
                    Text(provider.name)
                        .font(.headline)

                    HStack(spacing: 8) {
                        HStack(spacing: 2) {
                            ForEach(0..<5) { index in
                                Image(systemName: index < Int(provider.rating) ? "star.fill" : "star")
                                    .font(.caption)
                                    .foregroundColor(.etridOrange)
                            }
                        }

                        Text(provider.formattedRating)
                            .font(.caption)
                            .foregroundColor(.secondary)
                    }

                    Text("Fee: \(provider.feeSummary)")
                        .font(.caption)
                        .foregroundColor(.secondary)
                }

                Spacer()

                if isSelected {
                    Image(systemName: "checkmark.circle.fill")
                        .foregroundColor(.etridBlue)
                }
            }
            .padding()
            .background(isSelected ? Color.etridBlue.opacity(0.1) : Color(.systemBackground))
            .cornerRadius(12)
            .overlay(
                RoundedRectangle(cornerRadius: 12)
                    .strokeBorder(isSelected ? Color.etridBlue : Color.clear, lineWidth: 2)
            )
        }
        .foregroundColor(.primary)
    }
}

struct QuoteDetailRow: View {
    let title: String
    let value: String
    var isBold: Bool = false

    var body: some View {
        HStack {
            Text(title)
                .font(.subheadline)
                .foregroundColor(.secondary)

            Spacer()

            Text(value)
                .font(isBold ? .subheadline.bold() : .subheadline)
                .foregroundColor(.primary)
        }
    }
}

struct ProviderSelectionView: View {
    @Environment(\.dismiss) private var dismiss
    let providers: [FiatProvider]
    @Binding var selectedProvider: FiatProvider?

    var body: some View {
        NavigationStack {
            List(providers) { provider in
                Button(action: {
                    selectedProvider = provider
                    dismiss()
                }) {
                    ProviderCard(provider: provider, isSelected: selectedProvider?.id == provider.id) {
                        selectedProvider = provider
                        dismiss()
                    }
                }
                .listRowInsets(EdgeInsets(top: 8, leading: 16, bottom: 8, trailing: 16))
            }
            .navigationTitle("Select Provider")
            .navigationBarTitleDisplayMode(.inline)
            .toolbar {
                ToolbarItem(placement: .cancellationAction) {
                    Button("Cancel") {
                        dismiss()
                    }
                }
            }
        }
    }
}

struct PaymentMethodSelectionView: View {
    @Environment(\.dismiss) private var dismiss
    let methods: [PaymentMethod]
    @Binding var selectedMethod: PaymentMethod?

    var body: some View {
        NavigationStack {
            List {
                ForEach(methods) { method in
                    Button(action: {
                        selectedMethod = method
                        dismiss()
                    }) {
                        HStack {
                            Image(systemName: method.iconName)
                                .foregroundColor(.etridBlue)

                            VStack(alignment: .leading, spacing: 4) {
                                Text(method.displayName)
                                    .font(.subheadline)

                                Text(method.type.processingTime)
                                    .font(.caption)
                                    .foregroundColor(.secondary)
                            }

                            Spacer()

                            if selectedMethod?.id == method.id {
                                Image(systemName: "checkmark.circle.fill")
                                    .foregroundColor(.etridBlue)
                            }
                        }
                    }
                }

                Button(action: {
                    // Add new payment method
                }) {
                    HStack {
                        Image(systemName: "plus.circle.fill")
                            .foregroundColor(.etridBlue)

                        Text("Add Payment Method")
                            .foregroundColor(.etridBlue)
                    }
                }
            }
            .navigationTitle("Payment Method")
            .navigationBarTitleDisplayMode(.inline)
            .toolbar {
                ToolbarItem(placement: .cancellationAction) {
                    Button("Cancel") {
                        dismiss()
                    }
                }
            }
        }
    }
}

@MainActor
class BuyCryptoViewModel: ObservableObject {
    @Published var transactionType: FiatTransactionType = .buy
    @Published var fiatAmount: String = ""
    @Published var cryptoAmount: String = "0.0"
    @Published var selectedCurrency: String = "USD"
    @Published var selectedToken: Token = Token(symbol: "ETH", name: "Ethereum", decimals: 18, isNative: true)
    @Published var selectedProvider: FiatProvider?
    @Published var selectedPaymentMethod: PaymentMethod?
    @Published var availableProviders: [FiatProvider] = []
    @Published var paymentMethods: [PaymentMethod] = []
    @Published var currentQuote: FiatQuote?
    @Published var isCalculating = false
    @Published var isLoadingProviders = false
    @Published var isProcessing = false
    @Published var showError = false
    @Published var errorMessage = ""
    @Published var showCryptoSelector = false

    var canProceed: Bool {
        guard let amount = Double(fiatAmount), amount > 0 else { return false }
        return selectedProvider != nil && selectedPaymentMethod != nil && currentQuote != nil
    }

    func loadData() async {
        isLoadingProviders = true

        // Load providers
        availableProviders = FiatProvider.mockProviders

        // Auto-select first provider
        if selectedProvider == nil {
            selectedProvider = availableProviders.first
        }

        // Load payment methods
        paymentMethods = PaymentMethod.mockMethods

        // Auto-select default payment method
        if selectedPaymentMethod == nil {
            selectedPaymentMethod = paymentMethods.first(where: { $0.isDefault }) ?? paymentMethods.first
        }

        isLoadingProviders = false
    }

    func calculateQuote() {
        guard let amount = Double(fiatAmount), amount > 0,
              let provider = selectedProvider else {
            cryptoAmount = "0.0"
            currentQuote = nil
            return
        }

        isCalculating = true

        Task {
            try? await Task.sleep(nanoseconds: 500_000_000)

            // Mock exchange rate
            let rate = transactionType == .buy ? 2500.0 : 2450.0 // Buy/sell spread
            let crypto = amount / rate
            let fee = provider.fees.calculateFee(for: amount)
            let total = transactionType == .buy ? amount + fee : amount - fee

            currentQuote = FiatQuote(
                provider: provider.name,
                fiatAmount: amount,
                cryptoAmount: String(format: "%.6f", crypto),
                exchangeRate: rate,
                fee: fee,
                total: total,
                expiresAt: Date().addingTimeInterval(60)
            )

            cryptoAmount = String(format: "%.6f", crypto)
            isCalculating = false
        }
    }

    func selectProvider(_ provider: FiatProvider) {
        selectedProvider = provider
        calculateQuote()
    }

    func proceedToCheckout() {
        isProcessing = true

        Task {
            try? await Task.sleep(nanoseconds: 2_000_000_000)

            // In a real app, this would open the provider's checkout flow
            // For now, just show success
            isProcessing = false
        }
    }
}

#Preview {
    BuyCryptoView()
}
