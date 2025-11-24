//
//  BorrowView.swift
//  EtridWallet
//
//  Borrowing interface
//

import SwiftUI

struct BorrowView: View {
    @StateObject private var viewModel = BorrowViewModel()
    @Environment(\.dismiss) private var dismiss
    @State private var selectedPool: LendingPool?
    @State private var showBorrowSheet = false

    var body: some View {
        NavigationStack {
            ScrollView {
                VStack(spacing: 20) {
                    // Borrow Limit Card
                    VStack(spacing: 16) {
                        HStack {
                            VStack(alignment: .leading, spacing: 8) {
                                Text("Borrow Limit")
                                    .font(.subheadline)
                                    .foregroundColor(.white.opacity(0.8))

                                Text(viewModel.borrowLimitFormatted)
                                    .font(.title.bold())
                                    .foregroundColor(.white)
                            }

                            Spacer()

                            VStack(alignment: .trailing, spacing: 8) {
                                Text("Used")
                                    .font(.subheadline)
                                    .foregroundColor(.white.opacity(0.8))

                                Text(viewModel.borrowLimitUsedFormatted)
                                    .font(.headline)
                                    .foregroundColor(.white)
                            }
                        }

                        // Progress Bar
                        GeometryReader { geometry in
                            ZStack(alignment: .leading) {
                                Rectangle()
                                    .fill(Color.white.opacity(0.3))
                                    .frame(height: 8)

                                Rectangle()
                                    .fill(Color.white)
                                    .frame(width: geometry.size.width * (viewModel.borrowLimitUsed / 100), height: 8)
                            }
                            .cornerRadius(4)
                        }
                        .frame(height: 8)
                    }
                    .padding()
                    .background(
                        LinearGradient(
                            colors: [Color.etridPurple, Color.etridBlue],
                            startPoint: .topLeading,
                            endPoint: .bottomTrailing
                        )
                    )
                    .cornerRadius(20)
                    .padding(.horizontal)

                    // Collateral Section
                    VStack(alignment: .leading, spacing: 12) {
                        Text("Your Collateral")
                            .font(.headline)
                            .padding(.horizontal)

                        if viewModel.collateralAssets.isEmpty {
                            VStack(spacing: 12) {
                                Image(systemName: "shield.slash")
                                    .font(.largeTitle)
                                    .foregroundColor(.secondary)

                                Text("No collateral supplied")
                                    .font(.subheadline)
                                    .foregroundColor(.secondary)

                                Text("Supply assets to use as collateral")
                                    .font(.caption)
                                    .foregroundColor(.secondary)
                            }
                            .frame(maxWidth: .infinity)
                            .padding(.vertical, 30)
                            .background(Color(.systemGray6))
                            .cornerRadius(12)
                            .padding(.horizontal)
                        } else {
                            ForEach(viewModel.collateralAssets) { asset in
                                CollateralCard(asset: asset)
                            }
                            .padding(.horizontal)
                        }
                    }

                    // Available to Borrow
                    VStack(alignment: .leading, spacing: 12) {
                        Text("Available to Borrow")
                            .font(.headline)
                            .padding(.horizontal)

                        if viewModel.pools.isEmpty {
                            VStack(spacing: 12) {
                                Image(systemName: "tray")
                                    .font(.largeTitle)
                                    .foregroundColor(.secondary)

                                Text("No borrowing pools available")
                                    .font(.subheadline)
                                    .foregroundColor(.secondary)
                            }
                            .frame(maxWidth: .infinity)
                            .padding(.vertical, 40)
                        } else {
                            ForEach(viewModel.pools) { pool in
                                BorrowPoolCard(pool: pool) {
                                    selectedPool = pool
                                    showBorrowSheet = true
                                }
                            }
                            .padding(.horizontal)
                        }
                    }
                }
                .padding(.vertical)
            }
            .navigationTitle("Borrow")
            .navigationBarTitleDisplayMode(.inline)
            .toolbar {
                ToolbarItem(placement: .cancellationAction) {
                    Button("Close") {
                        dismiss()
                    }
                }
            }
            .sheet(isPresented: $showBorrowSheet) {
                if let pool = selectedPool {
                    BorrowSheetView(pool: pool, borrowLimit: viewModel.availableToBorrow)
                }
            }
        }
        .task {
            await viewModel.loadData()
        }
    }
}

struct BorrowPoolCard: View {
    let pool: LendingPool
    let action: () -> Void

    var body: some View {
        Button(action: action) {
            VStack(spacing: 12) {
                HStack {
                    Circle()
                        .fill(Color.etridPurple.opacity(0.2))
                        .frame(width: 40, height: 40)
                        .overlay(
                            Text(pool.asset.symbol.prefix(1))
                                .font(.headline)
                                .foregroundColor(.etridPurple)
                        )

                    VStack(alignment: .leading, spacing: 4) {
                        Text(pool.asset.symbol)
                            .font(.headline)

                        Text(pool.asset.name)
                            .font(.caption)
                            .foregroundColor(.secondary)
                    }

                    Spacer()

                    VStack(alignment: .trailing, spacing: 4) {
                        Text(pool.formattedBorrowAPY)
                            .font(.title3.bold())
                            .foregroundColor(.etridPurple)

                        Text("Borrow APY")
                            .font(.caption)
                            .foregroundColor(.secondary)
                    }
                }

                Divider()

                HStack {
                    VStack(alignment: .leading, spacing: 4) {
                        Text("Available")
                            .font(.caption)
                            .foregroundColor(.secondary)

                        Text(pool.availableLiquidity)
                            .font(.subheadline)
                            .fontWeight(.medium)
                    }

                    Spacer()

                    VStack(alignment: .trailing, spacing: 4) {
                        Text("Collateral Factor")
                            .font(.caption)
                            .foregroundColor(.secondary)

                        Text(String(format: "%.0f%%", pool.collateralFactor * 100))
                            .font(.subheadline)
                            .fontWeight(.medium)
                    }
                }
            }
            .padding()
            .background(Color(.systemBackground))
            .cornerRadius(16)
            .shadow(color: Color.black.opacity(0.05), radius: 5, x: 0, y: 2)
        }
        .foregroundColor(.primary)
    }
}

struct CollateralCard: View {
    let asset: CollateralAsset

    var body: some View {
        HStack {
            Circle()
                .fill(Color.etridBlue.opacity(0.2))
                .frame(width: 40, height: 40)
                .overlay(
                    Text(asset.token.symbol.prefix(1))
                        .font(.headline)
                        .foregroundColor(.etridBlue)
                )

            VStack(alignment: .leading, spacing: 4) {
                Text(asset.token.symbol)
                    .font(.headline)

                Text(asset.formattedAmount)
                    .font(.caption)
                    .foregroundColor(.secondary)
            }

            Spacer()

            VStack(alignment: .trailing, spacing: 4) {
                Text(asset.formattedValue)
                    .font(.subheadline.bold())

                Text("Max Borrow: \(asset.formattedMaxBorrow)")
                    .font(.caption)
                    .foregroundColor(.secondary)
            }
        }
        .padding()
        .background(Color(.systemBackground))
        .cornerRadius(12)
        .shadow(color: Color.black.opacity(0.05), radius: 5, x: 0, y: 2)
    }
}

struct BorrowSheetView: View {
    let pool: LendingPool
    let borrowLimit: Double
    @Environment(\.dismiss) private var dismiss
    @State private var amount: String = ""
    @State private var isBorrowing = false

    var borrowAmount: Double {
        Double(amount) ?? 0
    }

    var newHealthFactor: Double {
        // Mock calculation
        return 1.85
    }

    var canBorrow: Bool {
        borrowAmount > 0 && borrowAmount <= borrowLimit
    }

    var body: some View {
        NavigationStack {
            VStack(spacing: 24) {
                // Pool Info
                VStack(spacing: 12) {
                    Circle()
                        .fill(Color.etridPurple.opacity(0.2))
                        .frame(width: 60, height: 60)
                        .overlay(
                            Text(pool.asset.symbol.prefix(1))
                                .font(.title)
                                .foregroundColor(.etridPurple)
                        )

                    Text("Borrow \(pool.asset.symbol)")
                        .font(.title2.bold())

                    Text("\(pool.formattedBorrowAPY) APY")
                        .font(.subheadline)
                        .foregroundColor(.etridPurple)
                }
                .padding(.top, 20)

                // Amount Input
                VStack(alignment: .leading, spacing: 8) {
                    Text("Amount")
                        .font(.subheadline)
                        .foregroundColor(.secondary)

                    HStack {
                        TextField("0.0", text: $amount)
                            .keyboardType(.decimalPad)
                            .font(.title)

                        Text(pool.asset.symbol)
                            .font(.headline)
                            .foregroundColor(.secondary)

                        Button("Max") {
                            amount = String(borrowLimit)
                        }
                        .font(.caption.bold())
                        .padding(.horizontal, 12)
                        .padding(.vertical, 6)
                        .background(Color.etridPurple)
                        .foregroundColor(.white)
                        .cornerRadius(8)
                    }
                    .padding()
                    .background(Color(.systemGray6))
                    .cornerRadius(12)

                    Text("Available: \(String(format: "%.2f", borrowLimit)) \(pool.asset.symbol)")
                        .font(.caption)
                        .foregroundColor(.secondary)
                }

                // Borrow Details
                VStack(spacing: 12) {
                    DetailRow(title: "Borrow APY", value: pool.formattedBorrowAPY, valueColor: .etridPurple)
                    DetailRow(title: "Current Health Factor", value: "2.50")
                    DetailRow(title: "New Health Factor", value: String(format: "%.2f", newHealthFactor),
                             valueColor: newHealthFactor >= 1.5 ? .etridGreen : .etridOrange)
                }
                .padding()
                .background(Color(.systemGray6))
                .cornerRadius(12)

                // Warning
                if newHealthFactor < 1.5 {
                    HStack(spacing: 12) {
                        Image(systemName: "exclamationmark.triangle.fill")
                            .foregroundColor(.etridOrange)

                        Text("Borrowing this amount will reduce your health factor significantly")
                            .font(.caption)
                            .foregroundColor(.secondary)
                    }
                    .padding()
                    .background(Color.etridOrange.opacity(0.1))
                    .cornerRadius(8)
                }

                Spacer()

                // Borrow Button
                Button(action: {
                    isBorrowing = true
                    // Execute borrow
                }) {
                    HStack {
                        if isBorrowing {
                            ProgressView()
                                .progressViewStyle(CircularProgressViewStyle(tint: .white))
                        }

                        Text("Borrow")
                            .fontWeight(.semibold)
                    }
                    .frame(maxWidth: .infinity)
                    .padding()
                    .background(canBorrow ? Color.etridPurple : Color.gray)
                    .foregroundColor(.white)
                    .cornerRadius(12)
                }
                .disabled(!canBorrow || isBorrowing)
            }
            .padding()
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
class BorrowViewModel: ObservableObject {
    @Published var pools: [LendingPool] = []
    @Published var collateralAssets: [CollateralAsset] = []
    @Published var borrowLimit: Double = 0
    @Published var borrowLimitUsed: Double = 0
    @Published var totalBorrowed: Double = 0

    var borrowLimitFormatted: String {
        String(format: "$%.2f", borrowLimit)
    }

    var borrowLimitUsedFormatted: String {
        String(format: "%.1f%%", borrowLimitUsed)
    }

    var availableToBorrow: Double {
        borrowLimit * (1 - borrowLimitUsed / 100)
    }

    func loadData() async {
        // Load pools
        pools = LendingPool.mockPools

        // Mock collateral
        collateralAssets = [
            CollateralAsset(
                token: Token(symbol: "ETH", name: "Ethereum", decimals: 18, isNative: true),
                amount: "5.0",
                value: 12500,
                collateralFactor: 0.75,
                liquidationThreshold: 0.80
            )
        ]

        borrowLimit = 9375 // $12,500 * 0.75
        borrowLimitUsed = 32.0 // 32% of limit used
        totalBorrowed = 3000
    }
}

// MARK: - Helper Views
private struct DetailRow: View {
    let title: String
    let value: String
    var valueColor: Color = .primary

    var body: some View {
        HStack {
            Text(title)
                .font(.subheadline)
                .foregroundColor(.secondary)

            Spacer()

            Text(value)
                .font(.subheadline)
                .fontWeight(.medium)
                .foregroundColor(valueColor)
        }
    }
}

#Preview {
    BorrowView()
}
