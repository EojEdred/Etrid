//
//  LendingView.swift
//  EtridWallet
//
//  Lending pools and supply interface
//

import SwiftUI

struct LendingView: View {
    @StateObject private var viewModel = LendingViewModel()
    @Environment(\.dismiss) private var dismiss
    @State private var selectedPool: LendingPool?
    @State private var showSupplySheet = false
    @State private var showWithdrawSheet = false

    var body: some View {
        NavigationStack {
            ScrollView {
                VStack(spacing: 20) {
                    // Total Supply Card
                    VStack(spacing: 16) {
                        Text("Total Supplied")
                            .font(.subheadline)
                            .foregroundColor(.white.opacity(0.8))

                        if viewModel.isLoading {
                            ProgressView()
                                .progressViewStyle(CircularProgressViewStyle(tint: .white))
                        } else {
                            Text(viewModel.totalSuppliedFormatted)
                                .font(.system(size: 36, weight: .bold))
                                .foregroundColor(.white)

                            Text("Earning \(viewModel.avgAPYFormatted) APY")
                                .font(.subheadline)
                                .foregroundColor(.white.opacity(0.8))
                        }
                    }
                    .frame(maxWidth: .infinity)
                    .padding(.vertical, 30)
                    .background(
                        LinearGradient(
                            colors: [Color.etridGreen, Color.etridBlue],
                            startPoint: .topLeading,
                            endPoint: .bottomTrailing
                        )
                    )
                    .cornerRadius(20)
                    .padding(.horizontal)

                    // My Positions
                    if !viewModel.positions.isEmpty {
                        VStack(alignment: .leading, spacing: 12) {
                            Text("My Positions")
                                .font(.headline)
                                .padding(.horizontal)

                            ForEach(viewModel.positions) { position in
                                PositionCard(position: position) {
                                    // Handle position tap
                                }
                            }
                            .padding(.horizontal)
                        }
                    }

                    // Available Pools
                    VStack(alignment: .leading, spacing: 12) {
                        HStack {
                            Text("Available Pools")
                                .font(.headline)

                            Spacer()

                            Button(action: { viewModel.refreshPools() }) {
                                Image(systemName: "arrow.clockwise")
                                    .font(.subheadline)
                            }
                        }
                        .padding(.horizontal)

                        if viewModel.pools.isEmpty && !viewModel.isLoading {
                            VStack(spacing: 12) {
                                Image(systemName: "tray")
                                    .font(.largeTitle)
                                    .foregroundColor(.secondary)

                                Text("No lending pools available")
                                    .font(.subheadline)
                                    .foregroundColor(.secondary)
                            }
                            .frame(maxWidth: .infinity)
                            .padding(.vertical, 40)
                        } else {
                            ForEach(viewModel.pools) { pool in
                                LendingPoolCard(pool: pool) {
                                    selectedPool = pool
                                    showSupplySheet = true
                                }
                            }
                            .padding(.horizontal)
                        }
                    }
                }
                .padding(.vertical)
            }
            .navigationTitle("Lending")
            .navigationBarTitleDisplayMode(.inline)
            .toolbar {
                ToolbarItem(placement: .cancellationAction) {
                    Button("Close") {
                        dismiss()
                    }
                }
            }
            .sheet(isPresented: $showSupplySheet) {
                if let pool = selectedPool {
                    SupplyView(pool: pool)
                }
            }
            .refreshable {
                await viewModel.loadData()
            }
        }
        .task {
            await viewModel.loadData()
        }
    }
}

private struct LendingPoolCard: View {
    let pool: LendingPool
    let action: () -> Void

    var body: some View {
        Button(action: action) {
            VStack(spacing: 12) {
                HStack {
                    // Token Icon
                    Circle()
                        .fill(Color.etridGreen.opacity(0.2))
                        .frame(width: 40, height: 40)
                        .overlay(
                            Text(pool.asset.symbol.prefix(1))
                                .font(.headline)
                                .foregroundColor(.etridGreen)
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
                        Text(pool.formattedSupplyAPY)
                            .font(.title3.bold())
                            .foregroundColor(.etridGreen)

                        Text("Supply APY")
                            .font(.caption)
                            .foregroundColor(.secondary)
                    }
                }

                Divider()

                HStack {
                    VStack(alignment: .leading, spacing: 4) {
                        Text("Total Supply")
                            .font(.caption)
                            .foregroundColor(.secondary)

                        Text(pool.totalSupply)
                            .font(.subheadline)
                            .fontWeight(.medium)
                    }

                    Spacer()

                    VStack(alignment: .trailing, spacing: 4) {
                        Text("Utilization")
                            .font(.caption)
                            .foregroundColor(.secondary)

                        HStack(spacing: 4) {
                            Text(pool.formattedUtilization)
                                .font(.subheadline)
                                .fontWeight(.medium)

                            if pool.isHighUtilization {
                                Image(systemName: "exclamationmark.triangle.fill")
                                    .font(.caption)
                                    .foregroundColor(.etridOrange)
                            }
                        }
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

private struct PositionCard: View {
    let position: LoanPosition
    let action: () -> Void

    var body: some View {
        Button(action: action) {
            VStack(spacing: 12) {
                HStack {
                    Circle()
                        .fill(healthStatusColor(position.healthStatus).opacity(0.2))
                        .frame(width: 40, height: 40)
                        .overlay(
                            Text(position.asset.symbol.prefix(1))
                                .font(.headline)
                                .foregroundColor(healthStatusColor(position.healthStatus))
                        )

                    VStack(alignment: .leading, spacing: 4) {
                        Text(position.asset.symbol)
                            .font(.headline)

                        Text("Health: \(position.formattedHealthFactor)")
                            .font(.caption)
                            .foregroundColor(healthStatusColor(position.healthStatus))
                    }

                    Spacer()

                    VStack(alignment: .trailing, spacing: 4) {
                        Text(position.formattedSupplied)
                            .font(.subheadline.bold())

                        Text("Supplied")
                            .font(.caption)
                            .foregroundColor(.secondary)
                    }
                }

                // Health Factor Bar
                GeometryReader { geometry in
                    ZStack(alignment: .leading) {
                        Rectangle()
                            .fill(Color(.systemGray5))
                            .frame(height: 4)

                        Rectangle()
                            .fill(healthStatusColor(position.healthStatus))
                            .frame(width: geometry.size.width * min(position.healthFactor / 3.0, 1.0), height: 4)
                    }
                    .cornerRadius(2)
                }
                .frame(height: 4)

                HStack {
                    Text(position.healthStatus.description)
                        .font(.caption)
                        .foregroundColor(.secondary)

                    Spacer()

                    if position.isLiquidatable {
                        Text("At Risk!")
                            .font(.caption.bold())
                            .foregroundColor(.etridRed)
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

    func healthStatusColor(_ status: HealthStatus) -> Color {
        switch status {
        case .safe:
            return .etridGreen
        case .moderate:
            return .yellow
        case .risky:
            return .etridOrange
        case .critical:
            return .etridRed
        }
    }
}

private struct SupplyView: View {
    let pool: LendingPool
    @Environment(\.dismiss) private var dismiss
    @State private var amount: String = ""
    @State private var isSupplying = false

    var body: some View {
        NavigationStack {
            VStack(spacing: 24) {
                // Pool Info
                VStack(spacing: 12) {
                    Circle()
                        .fill(Color.etridGreen.opacity(0.2))
                        .frame(width: 60, height: 60)
                        .overlay(
                            Text(pool.asset.symbol.prefix(1))
                                .font(.title)
                                .foregroundColor(.etridGreen)
                        )

                    Text("Supply \(pool.asset.symbol)")
                        .font(.title2.bold())

                    Text("Earn \(pool.formattedSupplyAPY) APY")
                        .font(.subheadline)
                        .foregroundColor(.etridGreen)
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
                            amount = "100.0" // Mock max
                        }
                        .font(.caption.bold())
                        .padding(.horizontal, 12)
                        .padding(.vertical, 6)
                        .background(Color.etridGreen)
                        .foregroundColor(.white)
                        .cornerRadius(8)
                    }
                    .padding()
                    .background(Color(.systemGray6))
                    .cornerRadius(12)

                    Text("Balance: 100.0 \(pool.asset.symbol)")
                        .font(.caption)
                        .foregroundColor(.secondary)
                }

                // Details
                VStack(spacing: 12) {
                    DetailRow(title: "Supply APY", value: pool.formattedSupplyAPY, valueColor: .etridGreen)
                    DetailRow(title: "Collateral Factor", value: String(format: "%.0f%%", pool.collateralFactor * 100))
                    DetailRow(title: "Available Liquidity", value: pool.availableLiquidity)
                }
                .padding()
                .background(Color(.systemGray6))
                .cornerRadius(12)

                Spacer()

                // Supply Button
                Button(action: {
                    isSupplying = true
                    // Execute supply
                }) {
                    HStack {
                        if isSupplying {
                            ProgressView()
                                .progressViewStyle(CircularProgressViewStyle(tint: .white))
                        }

                        Text("Supply")
                            .fontWeight(.semibold)
                    }
                    .frame(maxWidth: .infinity)
                    .padding()
                    .background(amount.isEmpty || Double(amount) == nil ? Color.gray : Color.etridGreen)
                    .foregroundColor(.white)
                    .cornerRadius(12)
                }
                .disabled(amount.isEmpty || Double(amount) == nil || isSupplying)
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

@MainActor
class LendingViewModel: ObservableObject {
    @Published var pools: [LendingPool] = []
    @Published var positions: [LoanPosition] = []
    @Published var isLoading = false
    @Published var totalSupplied: Double = 0
    @Published var avgAPY: Double = 0

    var totalSuppliedFormatted: String {
        String(format: "$%.2f", totalSupplied)
    }

    var avgAPYFormatted: String {
        String(format: "%.2f%%", avgAPY)
    }

    func loadData() async {
        isLoading = true

        // Load pools
        pools = LendingPool.mockPools

        // Load positions (mock data)
        positions = [LoanPosition.mock]

        // Calculate totals
        totalSupplied = 12500.0
        avgAPY = 4.2

        isLoading = false
    }

    func refreshPools() {
        Task {
            await loadData()
        }
    }
}

#Preview {
    LendingView()
}
