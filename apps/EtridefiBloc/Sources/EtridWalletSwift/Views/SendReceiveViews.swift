import SwiftUI
#if os(iOS)
import UIKit
#elseif os(macOS)
import AppKit
#endif

// MARK: - Send View

struct SendView: View {
    @EnvironmentObject var walletManager: EtridWalletManager
    @Environment(\.dismiss) var dismiss

    @State private var recipient = ""
    @State private var amount = ""
    @State private var selectedToken: Token = .etrid
    @State private var isScanning = false
    @State private var isSending = false
    @State private var showConfirmation = false
    @State private var transactionHash: String?
    @State private var errorMessage: String?

    var body: some View {
        NavigationStack {
            ScrollView {
                VStack(spacing: 24) {
                    // Token Selector
                    TokenSelectorView(selectedToken: $selectedToken)

                    // Amount Input
                    AmountInputView(
                        amount: $amount,
                        token: selectedToken,
                        balance: currentBalance
                    )

                    // Recipient Input
                    RecipientInputView(
                        recipient: $recipient,
                        onScan: { isScanning = true }
                    )

                    // Fee Estimate
                    if !amount.isEmpty && !recipient.isEmpty {
                        FeeEstimateView(amount: Decimal(string: amount) ?? 0, token: selectedToken)
                    }

                    Spacer(minLength: 20)

                    // Send Button
                    Button(action: { showConfirmation = true }) {
                        HStack {
                            if isSending {
                                ProgressView()
                                    .tint(.white)
                            } else {
                                Image(systemName: "arrow.up.circle.fill")
                                Text("Send \(selectedToken.symbol)")
                            }
                        }
                        .font(.headline)
                        .foregroundColor(.white)
                        .frame(maxWidth: .infinity)
                        .padding()
                        .background(isValid ? EtridColors.primary : Color.gray)
                        .cornerRadius(16)
                    }
                    .disabled(!isValid || isSending)

                    if let error = errorMessage {
                        Text(error)
                            .font(.caption)
                            .foregroundColor(.red)
                            .multilineTextAlignment(.center)
                    }
                }
                .padding()
            }
            .navigationTitle("Send")
            #if os(iOS)
            .navigationBarTitleDisplayMode(.inline)
            #endif
            .toolbar {
                ToolbarItem(placement: .cancellationAction) {
                    Button("Cancel") { dismiss() }
                }
            }
            .sheet(isPresented: $isScanning) {
                QRScannerView { code in
                    recipient = code
                    isScanning = false
                }
            }
            .alert("Confirm Transaction", isPresented: $showConfirmation) {
                Button("Cancel", role: .cancel) { }
                Button("Send") { Task { await sendTransaction() } }
            } message: {
                Text("Send \(amount) \(selectedToken.symbol) to \(truncatedRecipient)?")
            }
            .alert("Transaction Sent", isPresented: .constant(transactionHash != nil)) {
                Button("Done") {
                    transactionHash = nil
                    dismiss()
                }
            } message: {
                if let hash = transactionHash {
                    Text("Transaction hash:\n\(hash)")
                }
            }
        }
    }

    private var isValid: Bool {
        guard let amountDecimal = Decimal(string: amount),
              amountDecimal > 0,
              amountDecimal <= currentBalance,
              isValidAddress(recipient) else {
            return false
        }
        return true
    }

    private var currentBalance: Decimal {
        walletManager.selectedAccount?.balances.first(where: { $0.token == selectedToken })?.amount ?? 0
    }

    private var truncatedRecipient: String {
        guard recipient.count > 12 else { return recipient }
        return "\(recipient.prefix(6))...\(recipient.suffix(4))"
    }

    private func isValidAddress(_ address: String) -> Bool {
        return address.hasPrefix("0x") && address.count == 42
    }

    private func sendTransaction() async {
        isSending = true
        errorMessage = nil

        do {
            guard let amountDecimal = Decimal(string: amount) else {
                throw WalletError.encodingError
            }

            let hash: String
            if selectedToken == .etrid {
                hash = try await walletManager.sendETR(to: recipient, amount: amountDecimal)
            } else {
                hash = try await walletManager.sendBridgedToken(token: selectedToken, to: recipient, amount: amountDecimal)
            }

            transactionHash = hash
        } catch {
            errorMessage = error.localizedDescription
        }

        isSending = false
    }
}

// MARK: - Receive View

struct ReceiveView: View {
    @EnvironmentObject var walletManager: EtridWalletManager
    @Environment(\.dismiss) var dismiss

    @State private var selectedToken: Token = .etrid
    @State private var copied = false

    var body: some View {
        NavigationStack {
            VStack(spacing: 24) {
                // Token Selector
                TokenSelectorView(selectedToken: $selectedToken)

                // QR Code
                QRCodeView(content: receiveAddress)
                    .frame(width: 200, height: 200)
                    .padding()
                    .background(Color.white)
                    .cornerRadius(16)

                // Address Display
                VStack(spacing: 8) {
                    Text("Your \(selectedToken.symbol) Address")
                        .font(.subheadline)
                        .foregroundColor(.secondary)

                    Text(receiveAddress)
                        .font(.system(.caption, design: .monospaced))
                        .multilineTextAlignment(.center)
                        .padding()
                        .background(EtridColors.cardBackground)
                        .cornerRadius(12)
                }

                // Copy Button
                Button(action: copyAddress) {
                    HStack {
                        Image(systemName: copied ? "checkmark" : "doc.on.doc")
                        Text(copied ? "Copied!" : "Copy Address")
                    }
                    .font(.headline)
                    .foregroundColor(.white)
                    .frame(maxWidth: .infinity)
                    .padding()
                    .background(EtridColors.primary)
                    .cornerRadius(16)
                }

                // Share Button
                ShareLink(item: receiveAddress) {
                    HStack {
                        Image(systemName: "square.and.arrow.up")
                        Text("Share Address")
                    }
                    .font(.headline)
                    .foregroundColor(EtridColors.primary)
                    .frame(maxWidth: .infinity)
                    .padding()
                    .background(EtridColors.cardBackground)
                    .cornerRadius(16)
                }

                Spacer()

                // Warning
                HStack {
                    Image(systemName: "exclamationmark.triangle.fill")
                        .foregroundColor(.orange)
                    Text("Only send \(selectedToken.symbol) to this address")
                        .font(.caption)
                        .foregroundColor(.secondary)
                }
            }
            .padding()
            .navigationTitle("Receive")
            #if os(iOS)
            .navigationBarTitleDisplayMode(.inline)
            #endif
            .toolbar {
                ToolbarItem(placement: .confirmationAction) {
                    Button("Done") { dismiss() }
                }
            }
        }
    }

    private var receiveAddress: String {
        walletManager.selectedAccount?.address ?? ""
    }

    private func copyAddress() {
        #if os(iOS)
        UIPasteboard.general.string = receiveAddress
        #elseif os(macOS)
        NSPasteboard.general.clearContents()
        NSPasteboard.general.setString(receiveAddress, forType: .string)
        #endif
        copied = true
        DispatchQueue.main.asyncAfter(deadline: .now() + 2) {
            copied = false
        }
    }
}

// MARK: - Token Selector

struct TokenSelectorView: View {
    @Binding var selectedToken: Token

    var body: some View {
        VStack(alignment: .leading, spacing: 8) {
            Text("Token")
                .font(.subheadline)
                .foregroundColor(.secondary)

            Menu {
                ForEach(Token.allTokens, id: \.id) { token in
                    Button(action: { selectedToken = token }) {
                        HStack {
                            Text(token.symbol)
                            Text(token.name)
                                .foregroundColor(.secondary)
                            if token == selectedToken {
                                Image(systemName: "checkmark")
                            }
                        }
                    }
                }
            } label: {
                HStack {
                    TokenBadge(token: selectedToken)

                    VStack(alignment: .leading) {
                        Text(selectedToken.symbol)
                            .font(.headline)
                        Text(selectedToken.name)
                            .font(.caption)
                            .foregroundColor(.secondary)
                    }

                    Spacer()

                    Image(systemName: "chevron.down")
                        .foregroundColor(.secondary)
                }
                .padding()
                .background(EtridColors.cardBackground)
                .cornerRadius(12)
            }
        }
    }
}

struct TokenBadge: View {
    let token: Token

    var body: some View {
        Circle()
            .fill(tokenColor)
            .frame(width: 40, height: 40)
            .overlay(
                Text(String(token.symbol.prefix(1)))
                    .font(.headline)
                    .fontWeight(.bold)
                    .foregroundColor(.white)
            )
    }

    private var tokenColor: Color {
        switch token.symbol {
        case "ETR": return EtridColors.primary
        case "bSOL": return .purple
        case "bBNB": return .yellow
        case "bXRP": return .blue
        case "bTRX": return .red
        case "bADA": return .blue
        case "bXLM": return .black
        case "bDOGE": return .orange
        case "USDT": return .green
        case "USDC": return .blue
        default: return .gray
        }
    }
}

// MARK: - Amount Input

struct AmountInputView: View {
    @Binding var amount: String
    let token: Token
    let balance: Decimal

    var body: some View {
        VStack(alignment: .leading, spacing: 8) {
            HStack {
                Text("Amount")
                    .font(.subheadline)
                    .foregroundColor(.secondary)

                Spacer()

                Button(action: setMax) {
                    Text("Max: \((balance as NSDecimalNumber).doubleValue, specifier: "%.4f") \(token.symbol)")
                        .font(.caption)
                        .foregroundColor(EtridColors.primary)
                }
            }

            HStack {
                TextField("0.00", text: $amount)
                    .font(.system(size: 32, weight: .bold, design: .rounded))
                    #if os(iOS)
                    .keyboardType(.decimalPad)
                    #endif
                    .multilineTextAlignment(.leading)

                Text(token.symbol)
                    .font(.title2)
                    .foregroundColor(.secondary)
            }
            .padding()
            .background(EtridColors.cardBackground)
            .cornerRadius(12)

            // Quick amount buttons
            HStack(spacing: 8) {
                ForEach(["25%", "50%", "75%", "Max"], id: \.self) { percentage in
                    Button(action: { setPercentage(percentage) }) {
                        Text(percentage)
                            .font(.caption)
                            .padding(.horizontal, 12)
                            .padding(.vertical, 6)
                            .background(EtridColors.cardBackground)
                            .cornerRadius(8)
                    }
                }
            }
        }
    }

    private func setMax() {
        amount = "\((balance as NSDecimalNumber).doubleValue)"
    }

    private func setPercentage(_ percentage: String) {
        let multiplier: Double
        switch percentage {
        case "25%": multiplier = 0.25
        case "50%": multiplier = 0.5
        case "75%": multiplier = 0.75
        default: multiplier = 1.0
        }
        let value = (balance as NSDecimalNumber).doubleValue * multiplier
        amount = String(format: "%.6f", value)
    }
}

// MARK: - Recipient Input

struct RecipientInputView: View {
    @Binding var recipient: String
    let onScan: () -> Void

    var body: some View {
        VStack(alignment: .leading, spacing: 8) {
            Text("Recipient")
                .font(.subheadline)
                .foregroundColor(.secondary)

            HStack {
                TextField("0x...", text: $recipient)
                    .font(.system(.body, design: .monospaced))
                    #if os(iOS)
                    .textInputAutocapitalization(.never)
                    #endif
                    .autocorrectionDisabled(true)

                Button(action: onScan) {
                    Image(systemName: "qrcode.viewfinder")
                        .font(.title2)
                        .foregroundColor(EtridColors.primary)
                }

                if !recipient.isEmpty {
                    Button(action: { recipient = "" }) {
                        Image(systemName: "xmark.circle.fill")
                            .foregroundColor(.secondary)
                    }
                }
            }
            .padding()
            .background(EtridColors.cardBackground)
            .cornerRadius(12)
        }
    }
}

// MARK: - Fee Estimate

struct FeeEstimateView: View {
    let amount: Decimal
    let token: Token

    var body: some View {
        VStack(spacing: 12) {
            HStack {
                Text("Network Fee")
                    .foregroundColor(.secondary)
                Spacer()
                Text("~0.001 ETR")
                    .fontWeight(.medium)
            }

            HStack {
                Text("Total")
                    .foregroundColor(.secondary)
                Spacer()
                VStack(alignment: .trailing) {
                    Text("\((amount as NSDecimalNumber).doubleValue, specifier: "%.6f") \(token.symbol)")
                        .fontWeight(.semibold)
                    Text("+ 0.001 ETR fee")
                        .font(.caption)
                        .foregroundColor(.secondary)
                }
            }
        }
        .padding()
        .background(EtridColors.cardBackground)
        .cornerRadius(12)
    }
}

// MARK: - QR Code View

struct QRCodeView: View {
    let content: String

    var body: some View {
        #if os(iOS)
        if let image = generateQRCode(from: content) {
            Image(uiImage: image)
                .interpolation(.none)
                .resizable()
                .scaledToFit()
        } else {
            placeholderQR
        }
        #else
        if let image = generateQRCode(from: content) {
            Image(nsImage: image)
                .interpolation(.none)
                .resizable()
                .scaledToFit()
        } else {
            placeholderQR
        }
        #endif
    }

    private var placeholderQR: some View {
        Image(systemName: "qrcode")
            .font(.system(size: 100))
            .foregroundColor(.gray)
    }

    #if os(iOS)
    private func generateQRCode(from string: String) -> UIImage? {
        let data = string.data(using: .ascii)

        if let filter = CIFilter(name: "CIQRCodeGenerator") {
            filter.setValue(data, forKey: "inputMessage")
            filter.setValue("H", forKey: "inputCorrectionLevel")

            if let output = filter.outputImage {
                let transform = CGAffineTransform(scaleX: 10, y: 10)
                let scaledOutput = output.transformed(by: transform)

                let context = CIContext()
                if let cgImage = context.createCGImage(scaledOutput, from: scaledOutput.extent) {
                    return UIImage(cgImage: cgImage)
                }
            }
        }

        return nil
    }
    #else
    private func generateQRCode(from string: String) -> NSImage? {
        let data = string.data(using: .ascii)

        if let filter = CIFilter(name: "CIQRCodeGenerator") {
            filter.setValue(data, forKey: "inputMessage")
            filter.setValue("H", forKey: "inputCorrectionLevel")

            if let output = filter.outputImage {
                let transform = CGAffineTransform(scaleX: 10, y: 10)
                let scaledOutput = output.transformed(by: transform)

                let rep = NSCIImageRep(ciImage: scaledOutput)
                let nsImage = NSImage(size: rep.size)
                nsImage.addRepresentation(rep)
                return nsImage
            }
        }

        return nil
    }
    #endif
}

// MARK: - QR Scanner Wrapper

struct QRScannerView: View {
    let onCodeScanned: (String) -> Void
    @Environment(\.dismiss) var dismiss

    var body: some View {
        NavigationStack {
            ZStack {
                // Camera preview would go here
                Color.black

                VStack {
                    Spacer()

                    RoundedRectangle(cornerRadius: 12)
                        .stroke(EtridColors.primary, lineWidth: 3)
                        .frame(width: 250, height: 250)

                    Spacer()

                    Text("Point your camera at a QR code")
                        .foregroundColor(.white)
                        .padding()
                }
            }
            .navigationTitle("Scan QR Code")
            #if os(iOS)
            .navigationBarTitleDisplayMode(.inline)
            #endif
            .toolbar {
                ToolbarItem(placement: .cancellationAction) {
                    Button("Cancel") { dismiss() }
                        .foregroundColor(.white)
                }
            }
        }
    }
}
