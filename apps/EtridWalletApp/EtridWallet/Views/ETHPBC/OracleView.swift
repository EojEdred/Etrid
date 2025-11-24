//
//  OracleView.swift
//  EtridWallet
//
//  Oracle Price Feeds View
//

import SwiftUI

struct OracleView: View {
    @StateObject private var service = ETHPBCService()
    @State private var prices: [OraclePrice] = []
    @State private var selectedToken = "ETR"
    @State private var currentPrice: OraclePrice?
    @State private var searchText = ""

    let availableTokens = ["ETR", "BTC", "ETH", "USDT", "USDC", "BNB", "ADA", "SOL", "DOT", "MATIC"]

    var filteredTokens: [String] {
        if searchText.isEmpty {
            return availableTokens
        }
        return availableTokens.filter { $0.lowercased().contains(searchText.lowercased()) }
    }

    var body: some View {
        VStack(spacing: 0) {
            // Current Price Display
            if let price = currentPrice {
                VStack(spacing: 16) {
                    HStack {
                        Text(price.token)
                            .font(.title)
                            .fontWeight(.bold)
                        Spacer()
                        HStack(spacing: 4) {
                            Circle()
                                .fill(Color(hex: price.confidenceColor) ?? .gray)
                                .frame(width: 8, height: 8)
                            Text("Confidence: \(price.confidenceDisplay)")
                                .font(.caption)
                                .foregroundColor(.gray)
                        }
                    }

                    Text(price.priceDisplay)
                        .font(.system(size: 48, weight: .bold))
                        .foregroundColor(.blue)

                    HStack {
                        VStack(alignment: .leading, spacing: 4) {
                            Text("Source")
                                .font(.caption)
                                .foregroundColor(.gray)
                            Text(price.source)
                                .font(.subheadline)
                                .fontWeight(.medium)
                        }

                        Spacer()

                        VStack(alignment: .trailing, spacing: 4) {
                            Text("Updated")
                                .font(.caption)
                                .foregroundColor(.gray)
                            Text(price.timestamp, style: .relative)
                                .font(.subheadline)
                                .fontWeight(.medium)
                        }
                    }
                }
                .padding()
                .background(Color(.systemGray6))
                .cornerRadius(12)
                .padding()
            } else {
                VStack(spacing: 12) {
                    ProgressView()
                    Text("Loading price...")
                        .font(.subheadline)
                        .foregroundColor(.gray)
                }
                .frame(height: 200)
            }

            // Search Bar
            HStack {
                Image(systemName: "magnifyingglass")
                    .foregroundColor(.gray)
                TextField("Search tokens...", text: $searchText)
            }
            .padding()
            .background(Color(.systemGray6))
            .cornerRadius(12)
            .padding(.horizontal)

            // Token List
            ScrollView {
                LazyVStack(spacing: 12) {
                    ForEach(filteredTokens, id: \.self) { token in
                        TokenPriceRow(token: token, isSelected: selectedToken == token) {
                            selectedToken = token
                            loadPrice(for: token)
                        }
                    }
                }
                .padding()
            }
        }
        .navigationTitle("Oracle Price Feeds")
        .navigationBarTitleDisplayMode(.inline)
        .toolbar {
            ToolbarItem(placement: .navigationBarTrailing) {
                Button(action: { loadPrice(for: selectedToken) }) {
                    Image(systemName: "arrow.clockwise")
                }
            }
        }
        .onAppear {
            loadPrice(for: selectedToken)
        }
    }

    private func loadPrice(for token: String) {
        Task {
            do {
                currentPrice = try await service.getOraclePrice(token: token)
            } catch {
                print("Error loading price: \(error)")
            }
        }
    }
}

// MARK: - Token Price Row
struct TokenPriceRow: View {
    let token: String
    let isSelected: Bool
    let action: () -> Void

    var body: some View {
        Button(action: action) {
            HStack {
                ZStack {
                    Circle()
                        .fill(Color.blue.opacity(0.2))
                        .frame(width: 40, height: 40)

                    Text(token.prefix(1))
                        .font(.headline)
                        .foregroundColor(.blue)
                }

                VStack(alignment: .leading, spacing: 4) {
                    Text(token)
                        .font(.headline)
                        .foregroundColor(.primary)
                    Text("Cryptocurrency")
                        .font(.caption)
                        .foregroundColor(.gray)
                }

                Spacer()

                if isSelected {
                    Image(systemName: "checkmark.circle.fill")
                        .foregroundColor(.blue)
                }

                Image(systemName: "chevron.right")
                    .foregroundColor(.gray)
                    .font(.caption)
            }
            .padding()
            .background(Color(.systemBackground))
            .cornerRadius(12)
            .overlay(
                RoundedRectangle(cornerRadius: 12)
                    .stroke(isSelected ? Color.blue : Color.gray.opacity(0.2), lineWidth: isSelected ? 2 : 1)
            )
        }
    }
}
