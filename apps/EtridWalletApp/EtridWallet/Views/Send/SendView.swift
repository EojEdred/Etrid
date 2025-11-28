//
//  SendView.swift
//  EtridWallet
//
//  Send transaction screen
//

import SwiftUI

struct SendView: View {
    @Environment(\.dismiss) private var dismiss
    @StateObject private var viewModel = SendViewModel()

    var body: some View {
        NavigationStack {
            ScrollView {
                VStack(spacing: 24) {
                    // Recipient
                    VStack(alignment: .leading, spacing: 8) {
                        Text("To")
                            .font(.headline)

                        HStack {
                            TextField("0x... or ENS name", text: $viewModel.recipient)
                                .textFieldStyle(.roundedBorder)
                                .autocorrectionDisabled()
                                .textInputAutocapitalization(.never)

                            Button(action: { viewModel.showQRScanner = true }) {
                                Image(systemName: "qrcode.viewfinder")
                                    .font(.title3)
                            }
                        }

                        if !viewModel.recipientError.isEmpty {
                            Text(viewModel.recipientError)
                                .font(.caption)
                                .foregroundColor(.red)
                        }
                    }

                    // Amount
                    VStack(alignment: .leading, spacing: 8) {
                        Text("Amount")
                            .font(.headline)

                        HStack {
                            TextField("0.0", text: $viewModel.amount)
                                .keyboardType(.decimalPad)
                                .textFieldStyle(.roundedBorder)

                            Text(viewModel.selectedToken.symbol)
                                .font(.headline)
                                .padding(.horizontal)
                        }

                        Button(action: { viewModel.useMaxAmount() }) {
                            Text("Max")
                                .font(.caption)
                                .foregroundColor(.etridBlue)
                        }

                        if let usdValue = viewModel.amountUSD {
                            Text("≈ \(usdValue)")
                                .font(.caption)
                                .foregroundColor(.secondary)
                        }
                    }

                    // Token Selector
                    VStack(alignment: .leading, spacing: 8) {
                        Text("Token")
                            .font(.headline)

                        Button(action: { viewModel.showTokenPicker = true }) {
                            HStack {
                                Circle()
                                    .fill(Color.etridBlue.opacity(0.2))
                                    .frame(width: 30, height: 30)
                                    .overlay(
                                        Text(viewModel.selectedToken.symbol.prefix(1))
                                            .font(.caption)
                                            .foregroundColor(.etridBlue)
                                    )

                                Text(viewModel.selectedToken.symbol)
                                    .font(.subheadline)

                                Spacer()

                                Text("Balance: \(viewModel.selectedToken.displayBalance)")
                                    .font(.caption)
                                    .foregroundColor(.secondary)

                                Image(systemName: "chevron.down")
                                    .font(.caption)
                            }
                            .padding()
                            .background(Color(.systemGray6))
                            .cornerRadius(12)
                        }
                        .foregroundColor(.primary)
                    }

                    // Gas Fee
                    if viewModel.isEstimatingGas {
                        HStack {
                            Text("Estimating gas...")
                                .font(.subheadline)
                                .foregroundColor(.secondary)

                            Spacer()

                            ProgressView()
                        }
                        .padding()
                        .background(Color(.systemGray6))
                        .cornerRadius(12)
                    } else if let gasFee = viewModel.estimatedGasFee {
                        VStack(alignment: .leading, spacing: 8) {
                            Text("Network Fee")
                                .font(.headline)

                            HStack {
                                Text("Gas Fee")
                                    .font(.subheadline)

                                Spacer()

                                VStack(alignment: .trailing, spacing: 4) {
                                    Text(gasFee)
                                        .font(.subheadline)

                                    if let usd = viewModel.gasFeeUSD {
                                        Text("≈ \(usd)")
                                            .font(.caption)
                                            .foregroundColor(.secondary)
                                    }
                                }
                            }
                            .padding()
                            .background(Color(.systemGray6))
                            .cornerRadius(12)
                        }
                    }

                    // Total
                    if viewModel.canEstimate {
                        Divider()

                        HStack {
                            Text("Total")
                                .font(.headline)

                            Spacer()

                            VStack(alignment: .trailing, spacing: 4) {
                                Text(viewModel.totalAmount)
                                    .font(.headline)

                                if let usd = viewModel.totalUSD {
                                    Text("≈ \(usd)")
                                        .font(.caption)
                                        .foregroundColor(.secondary)
                                }
                            }
                        }
                    }

                    // Send Button
                    Button(action: {
                        Task {
                            await viewModel.sendTransaction()
                        }
                    }) {
                        if viewModel.isSending {
                            ProgressView()
                                .tint(.white)
                        } else {
                            Text("Send")
                                .font(.headline)
                        }
                    }
                    .foregroundColor(.white)
                    .frame(maxWidth: .infinity)
                    .padding()
                    .background(viewModel.canSend ? Color.etridBlue : Color.gray)
                    .cornerRadius(16)
                    .disabled(!viewModel.canSend || viewModel.isSending)

                    if let error = viewModel.error {
                        Text(error)
                            .font(.caption)
                            .foregroundColor(.red)
                            .padding()
                            .background(Color.red.opacity(0.1))
                            .cornerRadius(8)
                    }
                }
                .padding()
            }
            .navigationTitle("Send")
            .navigationBarTitleDisplayMode(.inline)
            .toolbar {
                ToolbarItem(placement: .cancellationAction) {
                    Button("Cancel") {
                        dismiss()
                    }
                }
            }
            .sheet(isPresented: $viewModel.showQRScanner) {
                QRScannerView(onCodeScanned: { scannedAddress in
                    viewModel.recipient = scannedAddress
                    viewModel.showQRScanner = false
                })
            }
            .sheet(isPresented: $viewModel.showTokenPicker) {
                TokenPickerView(selectedToken: $viewModel.selectedToken)
            }
            .alert("Transaction Sent!", isPresented: $viewModel.showSuccess) {
                Button("View") {
                    // Open explorer
                }
                Button("Done") {
                    dismiss()
                }
            } message: {
                if let hash = viewModel.transactionHash {
                    Text("Transaction hash:\n\(hash.formatAddress())")
                }
            }
        }
        .task {
            await viewModel.loadData()
        }
    }
}

@MainActor
class SendViewModel: ObservableObject {
    @Published var recipient = "" {
        didSet {
            validateRecipient()
            if canEstimate {
                Task {
                    await estimateGas()
                }
            }
        }
    }
    @Published var amount = "" {
        didSet {
            if canEstimate {
                Task {
                    await estimateGas()
                }
            }
        }
    }
    @Published var selectedToken: Token = Token(symbol: "ETH", name: "Ethereum", isNative: true) {
        didSet {
            if canEstimate {
                Task {
                    await estimateGas()
                }
            }
        }
    }
    @Published var estimatedGasFee: String?
    @Published var gasFeeUSD: String?
    @Published var isEstimatingGas = false
    @Published var isSending = false
    @Published var error: String?
    @Published var recipientError = ""
    @Published var showQRScanner = false
    @Published var showTokenPicker = false
    @Published var showSuccess = false
    @Published var transactionHash: String?

    private var currentAccount: WalletAccount?
    private var network: Network = .ethereum

    var amountUSD: String? {
        guard let value = Double(amount), value > 0 else { return nil }
        // TODO: Fetch real price asynchronously
        return "$0.00"
    }

    var totalAmount: String {
        guard let value = Double(amount), value > 0 else { return "0 \(selectedToken.symbol)" }
        return String(format: "%.6f %@", value, selectedToken.symbol)
    }

    var totalUSD: String? {
        amountUSD
    }

    var canEstimate: Bool {
        !recipient.isEmpty && recipient.isValidEthereumAddress &&
        !amount.isEmpty && Double(amount) ?? 0 > 0
    }

    var canSend: Bool {
        canEstimate && estimatedGasFee != nil && recipientError.isEmpty
    }

    func loadData() async {
        do {
            currentAccount = try await WalletStorage.shared.getPrimaryAccount()
        } catch {
            self.error = error.localizedDescription
        }
    }

    func validateRecipient() {
        if recipient.isEmpty {
            recipientError = ""
        } else if !recipient.isValidEthereumAddress {
            recipientError = "Invalid address"
        } else {
            recipientError = ""
        }
    }

    func estimateGas() async {
        guard canEstimate, let from = currentAccount?.address else { return }

        isEstimatingGas = true
        defer { isEstimatingGas = false }

        do {
            let web3 = Web3Service(network: network)
            let amountWei = WeiConverter.etherToWei(Double(amount) ?? 0)

            let gasLimit = try await web3.estimateGas(from: from, to: recipient, value: amountWei)
            let gasPrice = try await web3.getGasPrice()

            estimatedGasFee = GasFormatter.formatGasFee(gasLimit: gasLimit, gasPrice: gasPrice)

            // Get real USD value for gas
            let ethForGas = WeiConverter.weiToEther(String(BigInt.fromHex(gasLimit)! * BigInt.fromHex(gasPrice)!))
            let ethPrice = try await PriceService.shared.getPrice(for: "ETH")
            gasFeeUSD = String(format: "$%.2f", ethForGas * ethPrice.price)
        } catch {
            self.error = error.localizedDescription
            estimatedGasFee = nil
            gasFeeUSD = nil
        }
    }

    func useMaxAmount() {
        // Set to max balance minus gas
        amount = selectedToken.displayBalance
    }

    func sendTransaction() async {
        guard let from = currentAccount?.address else { return }

        isSending = true
        error = nil

        do {
            let web3 = Web3Service(network: network)
            let amountWei = WeiConverter.etherToWei(Double(amount) ?? 0)

            let txRequest = try await web3.buildTransaction(
                from: from,
                to: recipient,
                value: amountWei
            )

            // Sign transaction
            guard let privateKey = try await WalletStorage.shared.loadPrivateKey(for: from) else {
                throw WalletError.invalidPrivateKey
            }

            // Mock signing - in production use proper RLP encoding
            let signedTx = "0x" + privateKey.hexString

            // Send
            let hash = try await web3.sendRawTransaction(signedTx)
            transactionHash = hash

            // Save transaction
            let transaction = Transaction(
                hash: hash,
                from: from,
                to: recipient,
                value: amountWei,
                gasLimit: txRequest.gasLimit ?? "21000",
                gasPrice: txRequest.gasPrice ?? "0",
                nonce: txRequest.nonce ?? 0,
                network: network.name
            )

            try await WalletStorage.shared.addTransaction(transaction)

            showSuccess = true
        } catch {
            self.error = error.localizedDescription
        }

        isSending = false
    }
}

struct TokenPickerView: View {
    @Environment(\.dismiss) private var dismiss
    @Binding var selectedToken: Token

    let tokens: [Token] = [
        Token(symbol: "ETH", name: "Ethereum", balance: "1000000000000000000", isNative: true),
        Token(symbol: "USDT", name: "Tether", contractAddress: "0x...", balance: "1000000"),
        Token(symbol: "USDC", name: "USD Coin", contractAddress: "0x...", balance: "500000")
    ]

    var body: some View {
        NavigationStack {
            List(tokens) { token in
                Button(action: {
                    selectedToken = token
                    dismiss()
                }) {
                    TokenRow(token: token)
                }
            }
            .navigationTitle("Select Token")
            .navigationBarTitleDisplayMode(.inline)
        }
    }
}

#Preview {
    SendView()
}
