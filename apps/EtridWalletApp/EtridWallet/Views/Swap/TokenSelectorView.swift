//
//  TokenSelectorView.swift
//  EtridWallet
//
//  Token selection interface for swaps
//

import SwiftUI

struct TokenSelectorView: View {
    @Environment(\.dismiss) private var dismiss
    @Binding var selectedToken: Token
    @State private var searchText = ""
    @State private var tokens: [Token] = []

    var filteredTokens: [Token] {
        if searchText.isEmpty {
            return tokens
        } else {
            return tokens.filter {
                $0.symbol.localizedCaseInsensitiveContains(searchText) ||
                $0.name.localizedCaseInsensitiveContains(searchText)
            }
        }
    }

    var body: some View {
        NavigationStack {
            VStack(spacing: 0) {
                // Search Bar
                HStack {
                    Image(systemName: "magnifyingglass")
                        .foregroundColor(.secondary)

                    TextField("Search tokens", text: $searchText)
                        .textFieldStyle(.plain)

                    if !searchText.isEmpty {
                        Button(action: { searchText = "" }) {
                            Image(systemName: "xmark.circle.fill")
                                .foregroundColor(.secondary)
                        }
                    }
                }
                .padding()
                .background(Color(.systemGray6))
                .cornerRadius(12)
                .padding()

                // Token List
                if filteredTokens.isEmpty {
                    VStack(spacing: 16) {
                        Image(systemName: "magnifyingglass")
                            .font(.system(size: 48))
                            .foregroundColor(.secondary)

                        Text("No tokens found")
                            .font(.headline)
                            .foregroundColor(.secondary)
                    }
                    .frame(maxWidth: .infinity, maxHeight: .infinity)
                } else {
                    ScrollView {
                        LazyVStack(spacing: 0) {
                            ForEach(filteredTokens) { token in
                                TokenSelectorRow(token: token, isSelected: token.id == selectedToken.id) {
                                    selectedToken = token
                                    dismiss()
                                }

                                if token.id != filteredTokens.last?.id {
                                    Divider()
                                        .padding(.leading, 72)
                                }
                            }
                        }
                    }
                }
            }
            .navigationTitle("Select Token")
            .navigationBarTitleDisplayMode(.inline)
            .toolbar {
                ToolbarItem(placement: .cancellationAction) {
                    Button("Cancel") {
                        dismiss()
                    }
                }
            }
            .task {
                loadTokens()
            }
        }
    }

    func loadTokens() {
        // Mock token list - replace with actual token fetching
        tokens = [
            Token(symbol: "ETH", name: "Ethereum", decimals: 18, isNative: true),
            Token(symbol: "USDC", name: "USD Coin", contractAddress: "0xA0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48", balance: "0", decimals: 6),
            Token(symbol: "DAI", name: "Dai Stablecoin", contractAddress: "0x6B175474E89094C44Da98b954EedeAC495271d0F", balance: "0", decimals: 18),
            Token(symbol: "USDT", name: "Tether USD", contractAddress: "0xdAC17F958D2ee523a2206206994597C13D831ec7", balance: "0", decimals: 6),
            Token(symbol: "WBTC", name: "Wrapped Bitcoin", contractAddress: "0x2260FAC5E5542a773Aa44fBCfeDf7C193bc2C599", balance: "0", decimals: 8),
            Token(symbol: "UNI", name: "Uniswap", contractAddress: "0x1f9840a85d5aF5bf1D1762F925BDADdC4201F984", balance: "0", decimals: 18),
            Token(symbol: "LINK", name: "Chainlink", contractAddress: "0x514910771AF9Ca656af840dff83E8264EcF986CA", balance: "0", decimals: 18),
            Token(symbol: "AAVE", name: "Aave", contractAddress: "0x7Fc66500c84A76Ad7e9c93437bFc5Ac33E2DDaE9", balance: "0", decimals: 18)
        ]
    }
}

struct TokenSelectorRow: View {
    let token: Token
    let isSelected: Bool
    let action: () -> Void

    var body: some View {
        Button(action: action) {
            HStack(spacing: 16) {
                // Token Icon
                Circle()
                    .fill(Color.etridBlue.opacity(0.2))
                    .frame(width: 40, height: 40)
                    .overlay(
                        Text(token.symbol.prefix(1))
                            .font(.headline)
                            .foregroundColor(.etridBlue)
                    )

                // Token Info
                VStack(alignment: .leading, spacing: 4) {
                    Text(token.symbol)
                        .font(.headline)
                        .foregroundColor(.primary)

                    Text(token.name)
                        .font(.caption)
                        .foregroundColor(.secondary)
                }

                Spacer()

                // Balance
                if !token.balance.isEmpty && token.balance != "0" {
                    Text(token.displayBalance)
                        .font(.subheadline)
                        .foregroundColor(.secondary)
                }

                // Selected indicator
                if isSelected {
                    Image(systemName: "checkmark.circle.fill")
                        .foregroundColor(.etridBlue)
                }
            }
            .padding()
            .background(isSelected ? Color.etridBlue.opacity(0.05) : Color.clear)
        }
    }
}

#Preview {
    TokenSelectorView(selectedToken: .constant(Token(symbol: "ETH", name: "Ethereum", decimals: 18, isNative: true)))
}
