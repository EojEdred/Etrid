//
//  WrapUnwrapView.swift
//  EtridWallet
//
//  ETH Wrap/Unwrap View
//

import SwiftUI

struct WrapUnwrapView: View {
    @StateObject private var service = ETHPBCService()
    @State private var isWrap = true
    @State private var amount = ""
    @State private var wrappedETHInfo: WrappedETH?
    @State private var isProcessing = false
    @State private var showSuccess = false
    @State private var errorMessage: String?

    var body: some View {
        ScrollView {
            VStack(spacing: 24) {
                // Balances Display
                if let info = wrappedETHInfo {
                    VStack(spacing: 16) {
                        BalanceCard(
                            title: "ETH Balance",
                            amount: info.ethDisplay,
                            icon: "e.circle.fill",
                            color: .blue
                        )

                        BalanceCard(
                            title: "wETH Balance",
                            amount: info.wethDisplay,
                            icon: "w.circle.fill",
                            color: .purple
                        )
                    }
                } else {
                    ProgressView("Loading balances...")
                        .padding()
                }

                // Direction Toggle
                VStack(alignment: .leading, spacing: 12) {
                    Text("Operation")
                        .font(.headline)

                    HStack(spacing: 12) {
                        OperationButton(
                            title: "Wrap",
                            subtitle: "ETH → wETH",
                            isSelected: isWrap,
                            action: { isWrap = true }
                        )

                        OperationButton(
                            title: "Unwrap",
                            subtitle: "wETH → ETH",
                            isSelected: !isWrap,
                            action: { isWrap = false }
                        )
                    }
                }

                // Amount Input
                VStack(alignment: .leading, spacing: 12) {
                    HStack {
                        Text("Amount")
                            .font(.headline)
                        Spacer()
                        if let info = wrappedETHInfo {
                            Button("Max") {
                                if isWrap {
                                    amount = info.ethBalance
                                } else {
                                    amount = info.wethBalance
                                }
                            }
                            .font(.subheadline)
                            .foregroundColor(.blue)
                        }
                    }

                    HStack {
                        TextField("0.0", text: $amount)
                            .keyboardType(.decimalPad)
                            .font(.title2)

                        Text(isWrap ? "ETH" : "wETH")
                            .font(.headline)
                            .foregroundColor(.gray)
                    }
                    .padding()
                    .background(Color(.systemGray6))
                    .cornerRadius(12)

                    // Quick Amount Buttons
                    HStack(spacing: 12) {
                        QuickAmountButton(title: "0.1", amount: "0.1", selectedAmount: $amount)
                        QuickAmountButton(title: "0.5", amount: "0.5", selectedAmount: $amount)
                        QuickAmountButton(title: "1.0", amount: "1.0", selectedAmount: $amount)
                        QuickAmountButton(title: "5.0", amount: "5.0", selectedAmount: $amount)
                    }
                }

                // Conversion Preview
                if !amount.isEmpty, let value = Double(amount) {
                    VStack(spacing: 12) {
                        HStack {
                            Text("You will receive")
                                .font(.subheadline)
                                .foregroundColor(.gray)
                            Spacer()
                            Text(String(format: "%.6f %@", value, isWrap ? "wETH" : "ETH"))
                                .font(.headline)
                                .foregroundColor(.blue)
                        }

                        HStack {
                            Text("Exchange Rate")
                                .font(.subheadline)
                                .foregroundColor(.gray)
                            Spacer()
                            Text("1:1")
                                .font(.subheadline)
                                .fontWeight(.medium)
                        }

                        HStack {
                            Text("Gas Fee")
                                .font(.subheadline)
                                .foregroundColor(.gray)
                            Spacer()
                            HStack(spacing: 4) {
                                Text("FREE")
                                    .font(.subheadline)
                                    .fontWeight(.bold)
                                    .foregroundColor(.green)
                                Image(systemName: "bolt.fill")
                                    .foregroundColor(.green)
                                    .font(.caption)
                            }
                        }
                    }
                    .padding()
                    .background(Color(.systemGray6))
                    .cornerRadius(12)
                }

                // Info Box
                VStack(alignment: .leading, spacing: 8) {
                    HStack {
                        Image(systemName: "info.circle.fill")
                            .foregroundColor(.blue)
                        Text("How it works")
                            .font(.subheadline)
                            .fontWeight(.medium)
                    }

                    Text(isWrap ?
                        "Wrapping ETH converts your native ETH into wETH (Wrapped ETH), an ERC-20 token with the same value. This allows you to use ETH in DeFi protocols." :
                        "Unwrapping wETH converts your wrapped tokens back into native ETH. The exchange rate is always 1:1."
                    )
                    .font(.caption)
                    .foregroundColor(.gray)
                }
                .padding()
                .background(Color.blue.opacity(0.1))
                .cornerRadius(12)

                // Error Message
                if let error = errorMessage {
                    Text(error)
                        .font(.subheadline)
                        .foregroundColor(.red)
                        .padding()
                        .background(Color.red.opacity(0.1))
                        .cornerRadius(8)
                }

                // Execute Button
                Button(action: executeWrapUnwrap) {
                    if isProcessing {
                        ProgressView()
                            .progressViewStyle(CircularProgressViewStyle(tint: .white))
                    } else {
                        HStack {
                            Image(systemName: "arrow.triangle.2.circlepath")
                            Text(isWrap ? "Wrap ETH" : "Unwrap wETH")
                        }
                        .font(.headline)
                    }
                }
                .foregroundColor(.white)
                .frame(maxWidth: .infinity)
                .padding()
                .background(canExecute ? Color.blue : Color.gray)
                .cornerRadius(12)
                .disabled(!canExecute || isProcessing)
            }
            .padding()
        }
        .navigationTitle("Wrap/Unwrap")
        .navigationBarTitleDisplayMode(.inline)
        .onAppear(perform: loadBalances)
        .alert(isPresented: $showSuccess) {
            Alert(
                title: Text("Success!"),
                message: Text("\(isWrap ? "Wrapped" : "Unwrapped") \(amount) successfully"),
                dismissButton: .default(Text("OK")) {
                    amount = ""
                    loadBalances()
                }
            )
        }
    }

    private var canExecute: Bool {
        !amount.isEmpty && Double(amount) != nil && Double(amount)! > 0
    }

    private func loadBalances() {
        Task {
            do {
                // Replace with actual wallet address
                let address = "0x1234567890123456789012345678901234567890"
                wrappedETHInfo = try await service.getWrappedETHInfo(address: address)
            } catch {
                errorMessage = error.localizedDescription
            }
        }
    }

    private func executeWrapUnwrap() {
        guard canExecute else { return }

        isProcessing = true
        errorMessage = nil

        Task {
            do {
                if isWrap {
                    _ = try await service.wrapETH(amount: amount)
                } else {
                    _ = try await service.unwrapETH(amount: amount)
                }
                showSuccess = true
            } catch {
                errorMessage = error.localizedDescription
            }
            isProcessing = false
        }
    }
}

// MARK: - Balance Card
struct BalanceCard: View {
    let title: String
    let amount: String
    let icon: String
    let color: Color

    var body: some View {
        HStack {
            Image(systemName: icon)
                .font(.title)
                .foregroundColor(color)

            VStack(alignment: .leading, spacing: 4) {
                Text(title)
                    .font(.subheadline)
                    .foregroundColor(.gray)
                Text(amount)
                    .font(.title3)
                    .fontWeight(.bold)
            }

            Spacer()
        }
        .padding()
        .background(Color(.systemGray6))
        .cornerRadius(12)
    }
}

// MARK: - Operation Button
struct OperationButton: View {
    let title: String
    let subtitle: String
    let isSelected: Bool
    let action: () -> Void

    var body: some View {
        Button(action: action) {
            VStack(spacing: 8) {
                Text(title)
                    .font(.headline)
                    .foregroundColor(isSelected ? .white : .primary)

                Text(subtitle)
                    .font(.caption)
                    .foregroundColor(isSelected ? .white.opacity(0.8) : .gray)
            }
            .frame(maxWidth: .infinity)
            .padding()
            .background(isSelected ? Color.blue : Color(.systemGray6))
            .cornerRadius(12)
        }
    }
}

// MARK: - Quick Amount Button
struct QuickAmountButton: View {
    let title: String
    let amount: String
    @Binding var selectedAmount: String

    var body: some View {
        Button(action: {
            selectedAmount = amount
        }) {
            Text(title)
                .font(.subheadline)
                .fontWeight(.medium)
                .foregroundColor(.blue)
                .frame(maxWidth: .infinity)
                .padding(.vertical, 10)
                .background(Color.blue.opacity(0.1))
                .cornerRadius(8)
        }
    }
}

struct WrapUnwrapView_Previews: PreviewProvider {
    static var previews: some View {
        NavigationView {
            WrapUnwrapView()
        }
    }
}
