//
//  TradingView.swift
//  EtridWallet
//
//  Trading screen with spot trading and swaps
//

import SwiftUI

struct TradingView: View {
    @StateObject private var viewModel = TradingViewModel()
    @State private var selectedTab: TradingTab = .spot

    enum TradingTab {
        case spot, swap
    }

    var body: some View {
        NavigationStack {
            ZStack {
                // Gradient background
                LinearGradient(
                    colors: [Color.etridBlue.opacity(0.2), Color.etridPurple.opacity(0.2)],
                    startPoint: .topLeading,
                    endPoint: .bottomTrailing
                )
                .ignoresSafeArea()

                ScrollView {
                    VStack(spacing: 20) {
                        // Trading Balance Card
                        HStack {
                            VStack(alignment: .leading, spacing: 4) {
                                Text("Trading Balance")
                                    .font(.subheadline)
                                    .foregroundColor(.secondary)

                                Text("$5,234.56")
                                    .font(.title)
                                    .fontWeight(.bold)
                            }

                            Spacer()

                            Button(action: {}) {
                                HStack {
                                    Image(systemName: "plus")
                                    Text("Deposit")
                                }
                                .font(.subheadline)
                                .fontWeight(.medium)
                                .foregroundColor(.white)
                                .padding(.horizontal, 16)
                                .padding(.vertical, 10)
                                .background(Color.etridBlue)
                                .cornerRadius(12)
                            }
                        }
                        .padding()
                        .background(Color(.systemBackground))
                        .cornerRadius(16)
                        .shadow(color: Color.black.opacity(0.05), radius: 5)
                        .padding(.horizontal)

                        // Tabs
                        HStack(spacing: 0) {
                            TabButton(title: "Spot Trading", isSelected: selectedTab == .spot) {
                                selectedTab = .spot
                            }
                            TabButton(title: "Quick Swap", isSelected: selectedTab == .swap) {
                                selectedTab = .swap
                            }
                        }
                        .padding(.horizontal)

                        // Markets Section
                        VStack(alignment: .leading, spacing: 12) {
                            Text("Markets")
                                .font(.headline)
                                .padding(.horizontal)

                            ForEach(viewModel.tradingPairs) { pair in
                                MarketCard(pair: pair)
                            }
                        }

                        // Quick Actions
                        VStack(alignment: .leading, spacing: 12) {
                            Text("Quick Actions")
                                .font(.headline)
                                .padding(.horizontal)

                            HStack(spacing: 12) {
                                QuickActionCard(
                                    icon: "arrow.2.squarepath",
                                    title: "Swap",
                                    subtitle: "Exchange tokens",
                                    color: .etridBlue
                                ) {
                                    // Navigate to swap
                                }

                                QuickActionCard(
                                    icon: "chart.line.uptrend.xyaxis",
                                    title: "Lending",
                                    subtitle: "Earn APY",
                                    color: .etridGreen
                                ) {
                                    // Navigate to lending
                                }
                            }
                            .padding(.horizontal)
                        }
                    }
                    .padding(.vertical)
                }
            }
            .navigationTitle("Trade")
            .navigationBarTitleDisplayMode(.inline)
            .toolbar {
                ToolbarItem(placement: .navigationBarTrailing) {
                    Button(action: {}) {
                        Image(systemName: "chart.xyaxis.line")
                    }
                }
            }
        }
        .task {
            await viewModel.loadMarkets()
        }
    }
}

struct MarketCard: View {
    let pair: TradingPair

    var body: some View {
        HStack {
            // Pair Info
            VStack(alignment: .leading, spacing: 4) {
                Text(pair.pair)
                    .font(.headline)

                Text("Vol: \(pair.volume24h)")
                    .font(.caption)
                    .foregroundColor(.secondary)
            }

            Spacer()

            // Price & Change
            VStack(alignment: .trailing, spacing: 4) {
                Text(pair.price)
                    .font(.headline)

                Text(pair.change24h)
                    .font(.caption)
                    .fontWeight(.medium)
                    .foregroundColor(pair.isPositiveChange ? .green : .red)
            }

            // Trade Button
            Button(action: {}) {
                Text("Trade")
                    .font(.caption)
                    .fontWeight(.medium)
                    .foregroundColor(.white)
                    .padding(.horizontal, 12)
                    .padding(.vertical, 6)
                    .background(Color.etridBlue)
                    .cornerRadius(8)
            }
        }
        .padding()
        .background(Color(.systemBackground))
        .cornerRadius(12)
        .padding(.horizontal)
    }
}

struct QuickActionCard: View {
    let icon: String
    let title: String
    let subtitle: String
    let color: Color
    let action: () -> Void

    var body: some View {
        Button(action: action) {
            VStack(spacing: 12) {
                Image(systemName: icon)
                    .font(.title2)
                    .foregroundColor(color)

                VStack(spacing: 4) {
                    Text(title)
                        .font(.subheadline)
                        .fontWeight(.semibold)
                        .foregroundColor(.primary)

                    Text(subtitle)
                        .font(.caption)
                        .foregroundColor(.secondary)
                }
            }
            .frame(maxWidth: .infinity)
            .padding(.vertical, 20)
            .background(Color(.systemBackground))
            .cornerRadius(16)
            .shadow(color: Color.black.opacity(0.05), radius: 3)
        }
    }
}

@MainActor
class TradingViewModel: ObservableObject {
    @Published var tradingPairs: [TradingPair] = []
    @Published var isLoading = false

    func loadMarkets() async {
        isLoading = true
        try? await Task.sleep(nanoseconds: 500_000_000)
        tradingPairs = TradingPair.mockPairs
        isLoading = false
    }
}

// MARK: - Helper Views
private struct TabButton: View {
    let title: String
    let isSelected: Bool
    let action: () -> Void

    var body: some View {
        Button(action: action) {
            Text(title)
                .font(.subheadline)
                .fontWeight(.medium)
                .foregroundColor(isSelected ? .white : .primary)
                .frame(maxWidth: .infinity)
                .padding(.vertical, 12)
                .background(isSelected ? Color.etridBlue : Color.clear)
                .cornerRadius(10)
        }
    }
}

#Preview {
    TradingView()
}
