//
//  PositionsView.swift
//  EtridWallet
//
//  View all lending/borrowing positions
//

import SwiftUI

struct PositionsView: View {
    @StateObject private var viewModel = PositionsViewModel()
    @Environment(\.dismiss) private var dismiss

    var body: some View {
        NavigationStack {
            ScrollView {
                VStack(spacing: 20) {
                    // Summary Cards
                    HStack(spacing: 16) {
                        SummaryCard(
                            title: "Total Supplied",
                            value: viewModel.totalSuppliedFormatted,
                            color: .etridGreen
                        )

                        SummaryCard(
                            title: "Total Borrowed",
                            value: viewModel.totalBorrowedFormatted,
                            color: .etridPurple
                        )
                    }
                    .padding(.horizontal)

                    // Net APY
                    VStack(spacing: 8) {
                        Text("Net APY")
                            .font(.subheadline)
                            .foregroundColor(.secondary)

                        Text(viewModel.netAPYFormatted)
                            .font(.title.bold())
                            .foregroundColor(viewModel.netAPY >= 0 ? .etridGreen : .etridRed)
                    }
                    .frame(maxWidth: .infinity)
                    .padding()
                    .background(Color(.systemGray6))
                    .cornerRadius(12)
                    .padding(.horizontal)

                    // Supply Positions
                    if !viewModel.supplyPositions.isEmpty {
                        VStack(alignment: .leading, spacing: 12) {
                            Text("Supply Positions")
                                .font(.headline)
                                .padding(.horizontal)

                            ForEach(viewModel.supplyPositions) { position in
                                SupplyPositionCard(position: position)
                            }
                            .padding(.horizontal)
                        }
                    }

                    // Borrow Positions
                    if !viewModel.borrowPositions.isEmpty {
                        VStack(alignment: .leading, spacing: 12) {
                            Text("Borrow Positions")
                                .font(.headline)
                                .padding(.horizontal)

                            ForEach(viewModel.borrowPositions) { position in
                                BorrowPositionCard(position: position)
                            }
                            .padding(.horizontal)
                        }
                    }

                    // Empty State
                    if viewModel.supplyPositions.isEmpty && viewModel.borrowPositions.isEmpty {
                        VStack(spacing: 16) {
                            Image(systemName: "chart.bar.doc.horizontal")
                                .font(.system(size: 60))
                                .foregroundColor(.secondary)

                            Text("No Active Positions")
                                .font(.headline)

                            Text("Start lending or borrowing to see your positions here")
                                .font(.subheadline)
                                .foregroundColor(.secondary)
                                .multilineTextAlignment(.center)
                        }
                        .frame(maxWidth: .infinity)
                        .padding(.vertical, 60)
                    }
                }
                .padding(.vertical)
            }
            .navigationTitle("My Positions")
            .navigationBarTitleDisplayMode(.inline)
            .toolbar {
                ToolbarItem(placement: .cancellationAction) {
                    Button("Close") {
                        dismiss()
                    }
                }
            }
            .refreshable {
                await viewModel.loadPositions()
            }
        }
        .task {
            await viewModel.loadPositions()
        }
    }
}

struct SummaryCard: View {
    let title: String
    let value: String
    let color: Color

    var body: some View {
        VStack(spacing: 8) {
            Text(title)
                .font(.caption)
                .foregroundColor(.secondary)

            Text(value)
                .font(.title3.bold())
                .foregroundColor(color)
        }
        .frame(maxWidth: .infinity)
        .padding()
        .background(color.opacity(0.1))
        .cornerRadius(12)
    }
}

struct SupplyPositionCard: View {
    let position: LoanPosition

    var body: some View {
        VStack(spacing: 12) {
            HStack {
                Circle()
                    .fill(Color.etridGreen.opacity(0.2))
                    .frame(width: 40, height: 40)
                    .overlay(
                        Text(position.asset.symbol.prefix(1))
                            .font(.headline)
                            .foregroundColor(.etridGreen)
                    )

                VStack(alignment: .leading, spacing: 4) {
                    Text(position.asset.symbol)
                        .font(.headline)

                    Text("Supplied")
                        .font(.caption)
                        .foregroundColor(.secondary)
                }

                Spacer()

                VStack(alignment: .trailing, spacing: 4) {
                    Text(position.formattedSupplied)
                        .font(.subheadline.bold())

                    Text("$\(String(format: "%.2f", position.collateralValue))")
                        .font(.caption)
                        .foregroundColor(.secondary)
                }
            }

            Divider()

            HStack {
                VStack(alignment: .leading, spacing: 4) {
                    Text("Earning")
                        .font(.caption)
                        .foregroundColor(.secondary)

                    Text("3.2% APY")
                        .font(.subheadline)
                        .foregroundColor(.etridGreen)
                }

                Spacer()

                VStack(alignment: .trailing, spacing: 4) {
                    Text("Interest Earned")
                        .font(.caption)
                        .foregroundColor(.secondary)

                    Text(position.interestAccrued)
                        .font(.subheadline)
                        .foregroundColor(.etridGreen)
                }
            }
        }
        .padding()
        .background(Color(.systemBackground))
        .cornerRadius(16)
        .shadow(color: Color.black.opacity(0.05), radius: 5, x: 0, y: 2)
    }
}

struct BorrowPositionCard: View {
    let position: LoanPosition

    var body: some View {
        VStack(spacing: 12) {
            HStack {
                Circle()
                    .fill(Color.etridPurple.opacity(0.2))
                    .frame(width: 40, height: 40)
                    .overlay(
                        Text(position.asset.symbol.prefix(1))
                            .font(.headline)
                            .foregroundColor(.etridPurple)
                    )

                VStack(alignment: .leading, spacing: 4) {
                    Text(position.asset.symbol)
                        .font(.headline)

                    Text("Borrowed")
                        .font(.caption)
                        .foregroundColor(.secondary)
                }

                Spacer()

                VStack(alignment: .trailing, spacing: 4) {
                    Text(position.formattedBorrowed)
                        .font(.subheadline.bold())

                    HStack(spacing: 4) {
                        Circle()
                            .fill(healthColor(position.healthStatus))
                            .frame(width: 8, height: 8)

                        Text("Health: \(position.formattedHealthFactor)")
                            .font(.caption)
                            .foregroundColor(healthColor(position.healthStatus))
                    }
                }
            }

            Divider()

            HStack {
                VStack(alignment: .leading, spacing: 4) {
                    Text("Borrow APY")
                        .font(.caption)
                        .foregroundColor(.secondary)

                    Text("5.8%")
                        .font(.subheadline)
                        .foregroundColor(.etridPurple)
                }

                Spacer()

                VStack(alignment: .trailing, spacing: 4) {
                    Text("Interest Owed")
                        .font(.caption)
                        .foregroundColor(.secondary)

                    Text(position.interestAccrued)
                        .font(.subheadline)
                        .foregroundColor(.etridRed)
                }
            }

            // Health Factor Bar
            VStack(alignment: .leading, spacing: 4) {
                Text("Health Factor")
                    .font(.caption)
                    .foregroundColor(.secondary)

                GeometryReader { geometry in
                    ZStack(alignment: .leading) {
                        Rectangle()
                            .fill(Color(.systemGray5))
                            .frame(height: 6)

                        Rectangle()
                            .fill(healthColor(position.healthStatus))
                            .frame(width: geometry.size.width * min(position.healthFactor / 3.0, 1.0), height: 6)
                    }
                    .cornerRadius(3)
                }
                .frame(height: 6)
            }
        }
        .padding()
        .background(Color(.systemBackground))
        .cornerRadius(16)
        .shadow(color: Color.black.opacity(0.05), radius: 5, x: 0, y: 2)
    }

    func healthColor(_ status: HealthStatus) -> Color {
        switch status {
        case .safe: return .etridGreen
        case .moderate: return .yellow
        case .risky: return .etridOrange
        case .critical: return .etridRed
        }
    }
}

@MainActor
class PositionsViewModel: ObservableObject {
    @Published var supplyPositions: [LoanPosition] = []
    @Published var borrowPositions: [LoanPosition] = []
    @Published var totalSupplied: Double = 0
    @Published var totalBorrowed: Double = 0
    @Published var netAPY: Double = 0

    var totalSuppliedFormatted: String {
        String(format: "$%.2f", totalSupplied)
    }

    var totalBorrowedFormatted: String {
        String(format: "$%.2f", totalBorrowed)
    }

    var netAPYFormatted: String {
        let sign = netAPY >= 0 ? "+" : ""
        return String(format: "%@%.2f%%", sign, netAPY)
    }

    func loadPositions() async {
        // Mock data - replace with actual data fetching
        supplyPositions = [LoanPosition.mock]
        borrowPositions = []

        totalSupplied = 12500.0
        totalBorrowed = 3000.0
        netAPY = 1.5 // Net APY after borrowing costs
    }
}

#Preview {
    PositionsView()
}
