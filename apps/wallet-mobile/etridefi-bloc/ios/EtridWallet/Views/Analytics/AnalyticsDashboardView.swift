//
//  AnalyticsDashboardView.swift
//  EtridWallet
//
//  Production Analytics Dashboard with AnalyticsService integration
//

import SwiftUI
import Charts

struct AnalyticsDashboardView: View {
    @StateObject private var viewModel = AnalyticsDashboardViewModel()
    @Environment(\.dismiss) private var dismiss

    var body: some View {
        NavigationView {
            ZStack {
                if viewModel.isLoading {
                    loadingView
                } else if let errorMessage = viewModel.errorMessage {
                    errorView(errorMessage)
                } else {
                    contentView
                }
            }
            .navigationTitle("Analytics")
            .navigationBarTitleDisplayMode(.large)
            .toolbar {
                ToolbarItem(placement: .navigationBarLeading) {
                    Button(action: { dismiss() }) {
                        Image(systemName: "chevron.left")
                            .foregroundColor(.blue)
                    }
                }
                ToolbarItem(placement: .navigationBarTrailing) {
                    Menu {
                        Button("Export Report") {}
                        Button("Settings") {}
                        Button("Refresh") {
                            Task {
                                await viewModel.loadData()
                            }
                        }
                    } label: {
                        Image(systemName: "ellipsis")
                            .foregroundColor(.blue)
                    }
                }
            }
            .task {
                await viewModel.loadData()
            }
        }
    }

    // MARK: - Content View
    private var contentView: some View {
        ScrollView {
            VStack(spacing: 20) {
                // Total Portfolio Card
                totalPortfolioCard

                // Metrics Grid
                metricsGrid

                // Performance Chart
                performanceChart

                // Asset Allocation
                assetAllocation

                // Risk Analysis Tools
                riskAnalysisTools
            }
            .padding()
        }
        .refreshable {
            await viewModel.loadData()
        }
    }

    // MARK: - Loading View
    private var loadingView: some View {
        VStack(spacing: 20) {
            ProgressView()
                .scaleEffect(1.5)
            Text("Loading Analytics...")
                .font(.headline)
                .foregroundColor(.secondary)
        }
    }

    // MARK: - Error View
    private func errorView(_ message: String) -> some View {
        VStack(spacing: 20) {
            Image(systemName: "exclamationmark.triangle.fill")
                .font(.system(size: 60))
                .foregroundColor(.orange)

            Text("Error Loading Data")
                .font(.title2)
                .fontWeight(.bold)

            Text(message)
                .font(.body)
                .foregroundColor(.secondary)
                .multilineTextAlignment(.center)
                .padding(.horizontal)

            Button(action: {
                Task {
                    await viewModel.loadData()
                }
            }) {
                Label("Retry", systemImage: "arrow.clockwise")
                    .font(.headline)
                    .foregroundColor(.white)
                    .padding()
                    .background(Color.blue)
                    .cornerRadius(12)
            }
        }
        .padding()
    }

    // MARK: - Total Portfolio Card
    private var totalPortfolioCard: some View {
        VStack(spacing: 8) {
            Text("Total Portfolio Value")
                .font(.subheadline)
                .foregroundColor(.secondary)

            Text("$\(viewModel.totalValue, specifier: "%.2f")")
                .font(.system(size: 36, weight: .bold))

            HStack(spacing: 4) {
                Image(systemName: viewModel.change24h >= 0 ? "arrow.up" : "arrow.down")
                    .font(.caption)
                Text("\(viewModel.change24h >= 0 ? "+" : "")\(viewModel.change24h, specifier: "%.2f")% (24h)")
                    .font(.body)
                    .fontWeight(.semibold)
            }
            .foregroundColor(viewModel.change24h >= 0 ? .green : .red)

            Text("Updated: \(viewModel.lastUpdated, formatter: dateFormatter)")
                .font(.caption2)
                .foregroundColor(.secondary)
                .padding(.top, 4)
        }
        .frame(maxWidth: .infinity)
        .padding(24)
        .background(
            LinearGradient(
                gradient: Gradient(colors: [Color.blue.opacity(0.1), Color.blue.opacity(0.05)]),
                startPoint: .topLeading,
                endPoint: .bottomTrailing
            )
        )
        .cornerRadius(16)
        .overlay(
            RoundedRectangle(cornerRadius: 16)
                .stroke(Color.blue, lineWidth: 1)
        )
    }

    // MARK: - Metrics Grid
    private var metricsGrid: some View {
        LazyVGrid(columns: [GridItem(.flexible()), GridItem(.flexible())], spacing: 12) {
            MetricCard(
                label: "Diversification",
                value: "\(viewModel.diversification)/100",
                icon: "chart.pie.fill"
            )

            MetricCard(
                label: "Risk Level",
                value: viewModel.riskLevel,
                icon: "exclamationmark.triangle.fill",
                valueColor: riskLevelColor(viewModel.riskLevel)
            )

            MetricCard(
                label: "Top Asset",
                value: viewModel.topAsset,
                icon: "star.fill"
            )

            MetricCard(
                label: "ROI (\(viewModel.selectedPeriod.rawValue))",
                value: viewModel.roi >= 0 ? String(format: "+%.2f%%", viewModel.roi) : String(format: "%.2f%%", viewModel.roi),
                icon: "chart.line.uptrend.xyaxis",
                valueColor: viewModel.roi >= 0 ? .green : .red
            )
        }
    }

    // MARK: - Performance Chart
    private var performanceChart: some View {
        VStack(alignment: .leading, spacing: 12) {
            HStack {
                Text("Performance Over Time")
                    .font(.headline)
                    .fontWeight(.bold)

                Spacer()

                if viewModel.isLoadingPerformance {
                    ProgressView()
                        .scaleEffect(0.8)
                }
            }

            // Period Selector
            ScrollView(.horizontal, showsIndicators: false) {
                HStack(spacing: 8) {
                    ForEach(PerformanceData.TimePeriod.allCases, id: \.self) { period in
                        PeriodButton(
                            period: period,
                            isSelected: viewModel.selectedPeriod == period,
                            action: {
                                Task {
                                    await viewModel.selectPeriod(period)
                                }
                            }
                        )
                    }
                }
            }

            if viewModel.performanceData.isEmpty {
                Text("No performance data available")
                    .font(.body)
                    .foregroundColor(.secondary)
                    .frame(height: 200)
                    .frame(maxWidth: .infinity)
                    .background(Color(.systemGray6))
                    .cornerRadius(16)
            } else {
                Chart(viewModel.performanceData) { dataPoint in
                    LineMark(
                        x: .value("Date", dataPoint.date),
                        y: .value("Value", dataPoint.value)
                    )
                    .foregroundStyle(Color.blue.gradient)
                    .interpolationMethod(.catmullRom)

                    AreaMark(
                        x: .value("Date", dataPoint.date),
                        y: .value("Value", dataPoint.value)
                    )
                    .foregroundStyle(
                        LinearGradient(
                            gradient: Gradient(colors: [Color.blue.opacity(0.3), Color.blue.opacity(0.05)]),
                            startPoint: .top,
                            endPoint: .bottom
                        )
                    )
                    .interpolationMethod(.catmullRom)
                }
                .frame(height: 200)
                .chartYAxis {
                    AxisMarks(position: .leading) { value in
                        AxisGridLine()
                        AxisValueLabel {
                            if let doubleValue = value.as(Double.self) {
                                Text("$\(doubleValue / 1000, specifier: "%.0f")k")
                            }
                        }
                    }
                }
                .chartXAxis {
                    AxisMarks { _ in
                        AxisGridLine()
                        AxisValueLabel(format: .dateTime.month().day())
                    }
                }
                .padding()
                .background(Color(.systemGray6))
                .cornerRadius(16)

                // Performance Summary
                HStack {
                    VStack(alignment: .leading, spacing: 4) {
                        Text("Total Gain")
                            .font(.caption)
                            .foregroundColor(.secondary)
                        Text("$\(viewModel.totalGain, specifier: "%.2f")")
                            .font(.body)
                            .fontWeight(.bold)
                            .foregroundColor(viewModel.totalGain >= 0 ? .green : .red)
                    }

                    Spacer()

                    VStack(alignment: .trailing, spacing: 4) {
                        Text("Total Gain %")
                            .font(.caption)
                            .foregroundColor(.secondary)
                        Text("\(viewModel.totalGainPercent >= 0 ? "+" : "")\(viewModel.totalGainPercent, specifier: "%.2f")%")
                            .font(.body)
                            .fontWeight(.bold)
                            .foregroundColor(viewModel.totalGainPercent >= 0 ? .green : .red)
                    }
                }
                .padding()
                .background(Color(.systemGray6))
                .cornerRadius(16)
            }
        }
    }

    // MARK: - Asset Allocation
    private var assetAllocation: some View {
        VStack(alignment: .leading, spacing: 12) {
            Text("Asset Allocation")
                .font(.headline)
                .fontWeight(.bold)

            if viewModel.allocationData.isEmpty {
                Text("No allocation data available")
                    .font(.body)
                    .foregroundColor(.secondary)
                    .frame(height: 200)
                    .frame(maxWidth: .infinity)
                    .background(Color(.systemGray6))
                    .cornerRadius(16)
            } else {
                // Pie Chart
                Chart(viewModel.allocationData) { allocation in
                    SectorMark(
                        angle: .value("Percentage", allocation.percentage),
                        innerRadius: .ratio(0.5),
                        angularInset: 2
                    )
                    .foregroundStyle(by: .value("Asset", allocation.asset))
                    .cornerRadius(8)
                }
                .frame(height: 200)
                .padding()
                .background(Color(.systemGray6))
                .cornerRadius(16)

                // Allocation Details
                VStack(spacing: 8) {
                    ForEach(viewModel.allocationData) { allocation in
                        AllocationRow(allocation: allocation)
                    }
                }
                .padding()
                .background(Color(.systemGray6))
                .cornerRadius(16)
            }
        }
    }

    // MARK: - Risk Analysis Tools
    private var riskAnalysisTools: some View {
        VStack(alignment: .leading, spacing: 12) {
            Text("Risk Metrics")
                .font(.headline)
                .fontWeight(.bold)

            if let metrics = viewModel.riskMetrics {
                VStack(spacing: 12) {
                    RiskMetricRow(
                        label: "Volatility",
                        value: String(format: "%.2f%%", metrics.volatility * 100),
                        icon: "waveform.path.ecg"
                    )

                    RiskMetricRow(
                        label: "Sharpe Ratio",
                        value: String(format: "%.2f", metrics.sharpeRatio),
                        icon: "chart.bar.fill"
                    )

                    RiskMetricRow(
                        label: "Max Drawdown",
                        value: String(format: "%.2f%%", metrics.maxDrawdown * 100),
                        icon: "arrow.down.circle.fill"
                    )

                    RiskMetricRow(
                        label: "Beta",
                        value: String(format: "%.2f", metrics.beta),
                        icon: "gauge"
                    )
                }
                .padding()
                .background(Color(.systemGray6))
                .cornerRadius(16)
            }
        }
    }

    // MARK: - Helper Functions
    private func riskLevelColor(_ level: String) -> Color {
        switch level.lowercased() {
        case "low":
            return .green
        case "medium":
            return .orange
        case "high", "very high":
            return .red
        default:
            return .primary
        }
    }

    private var dateFormatter: DateFormatter {
        let formatter = DateFormatter()
        formatter.dateStyle = .short
        formatter.timeStyle = .short
        return formatter
    }
}

// MARK: - Period Button
struct PeriodButton: View {
    let period: PerformanceData.TimePeriod
    let isSelected: Bool
    let action: () -> Void

    var body: some View {
        Button(action: action) {
            Text(period.rawValue)
                .font(.caption)
                .fontWeight(isSelected ? .bold : .regular)
                .foregroundColor(isSelected ? .white : .blue)
                .padding(.horizontal, 12)
                .padding(.vertical, 6)
                .background(isSelected ? Color.blue : Color.blue.opacity(0.1))
                .cornerRadius(8)
        }
    }
}

// MARK: - Metric Card Component
struct MetricCard: View {
    let label: String
    let value: String
    let icon: String
    var valueColor: Color = .primary

    var body: some View {
        VStack(alignment: .leading, spacing: 8) {
            HStack {
                Image(systemName: icon)
                    .foregroundColor(.blue)
                    .font(.caption)
                Spacer()
            }

            Text(label)
                .font(.caption)
                .foregroundColor(.secondary)

            Text(value)
                .font(.title3)
                .fontWeight(.bold)
                .foregroundColor(valueColor)
        }
        .frame(maxWidth: .infinity, alignment: .leading)
        .padding()
        .background(Color(.systemGray6))
        .cornerRadius(12)
        .overlay(
            RoundedRectangle(cornerRadius: 12)
                .stroke(Color(.systemGray4), lineWidth: 1)
        )
    }
}

// MARK: - Allocation Row
struct AllocationRow: View {
    let allocation: AllocationData

    var body: some View {
        HStack {
            Circle()
                .fill(allocation.color)
                .frame(width: 12, height: 12)

            VStack(alignment: .leading, spacing: 2) {
                Text(allocation.asset)
                    .font(.body)
                    .fontWeight(.semibold)

                Text("$\(allocation.valueUSD, specifier: "%.2f")")
                    .font(.caption)
                    .foregroundColor(.secondary)
            }

            Spacer()

            VStack(alignment: .trailing, spacing: 2) {
                Text("\(allocation.percentage, specifier: "%.1f")%")
                    .font(.body)
                    .fontWeight(.bold)

                HStack(spacing: 2) {
                    Image(systemName: allocation.change24h >= 0 ? "arrow.up" : "arrow.down")
                        .font(.caption2)
                    Text("\(allocation.change24h >= 0 ? "+" : "")\(allocation.change24h, specifier: "%.2f")%")
                        .font(.caption)
                }
                .foregroundColor(allocation.change24h >= 0 ? .green : .red)
            }
        }
    }
}

// MARK: - Risk Metric Row
struct RiskMetricRow: View {
    let label: String
    let value: String
    let icon: String

    var body: some View {
        HStack {
            Image(systemName: icon)
                .foregroundColor(.blue)
                .frame(width: 24)

            Text(label)
                .font(.body)

            Spacer()

            Text(value)
                .font(.body)
                .fontWeight(.semibold)
                .foregroundColor(.secondary)
        }
    }
}

// MARK: - View Model
@MainActor
class AnalyticsDashboardViewModel: ObservableObject {
    // MARK: - Published Properties
    @Published var isLoading = false
    @Published var isLoadingPerformance = false
    @Published var errorMessage: String?

    // Portfolio Data
    @Published var totalValue: Double = 0
    @Published var change24h: Double = 0
    @Published var lastUpdated = Date()

    // Risk Metrics
    @Published var diversification: Int = 0
    @Published var riskLevel: String = "Medium"
    @Published var riskMetrics: RiskMetrics?

    // Performance Data
    @Published var selectedPeriod: PerformanceData.TimePeriod = .month
    @Published var performanceData: [PerformanceDataPoint] = []
    @Published var topAsset: String = "-"
    @Published var roi: Double = 0
    @Published var totalGain: Double = 0
    @Published var totalGainPercent: Double = 0

    // Allocation Data
    @Published var allocationData: [AllocationData] = []

    // MARK: - Private Properties
    private let analyticsService = AnalyticsService.shared
    private let userId = "user_123" // TODO: Get from AuthService

    // MARK: - Data Loading
    func loadData() async {
        isLoading = true
        errorMessage = nil

        do {
            // Load portfolio snapshot
            let portfolio = try await analyticsService.getCurrentPortfolio(for: userId)
            totalValue = portfolio.totalValue
            change24h = portfolio.change24h
            lastUpdated = portfolio.timestamp

            // Load allocation data
            let assets = try await analyticsService.getAssetAllocation(for: userId)
            updateAllocationData(from: assets)

            // Determine top asset
            if let top = assets.max(by: { $0.valueUSD < $1.valueUSD }) {
                topAsset = top.asset
            }

            // Load risk metrics
            let risk = try await analyticsService.getRiskMetrics(for: userId)
            riskMetrics = risk
            diversification = risk.diversificationScore
            riskLevel = risk.riskLevel.rawValue.capitalized

            // Load performance data
            await loadPerformanceData(for: selectedPeriod)

            isLoading = false
        } catch {
            isLoading = false
            errorMessage = error.localizedDescription
        }
    }

    func selectPeriod(_ period: PerformanceData.TimePeriod) async {
        guard period != selectedPeriod else { return }
        selectedPeriod = period
        await loadPerformanceData(for: period)
    }

    private func loadPerformanceData(for period: PerformanceData.TimePeriod) async {
        isLoadingPerformance = true

        do {
            let performance = try await analyticsService.getPerformance(for: userId, period: period)

            // Update performance data
            performanceData = performance.dataPoints.map { dataPoint in
                PerformanceDataPoint(
                    date: dataPoint.timestamp,
                    value: dataPoint.value
                )
            }

            roi = performance.roi
            totalGain = performance.totalGain
            totalGainPercent = performance.totalGainPercent

            isLoadingPerformance = false
        } catch {
            isLoadingPerformance = false
            // Keep existing data on error, just show error in console
            print("Error loading performance data: \(error.localizedDescription)")
        }
    }

    private func updateAllocationData(from assets: [AssetHolding]) {
        let colors: [Color] = [.blue, .green, .purple, .orange, .pink, .yellow, .red, .cyan]

        allocationData = assets.enumerated().map { index, asset in
            AllocationData(
                asset: asset.asset,
                percentage: asset.allocation,
                valueUSD: asset.valueUSD,
                change24h: asset.change24h,
                color: colors[index % colors.count]
            )
        }
    }
}

// MARK: - Models
struct PerformanceDataPoint: Identifiable {
    let id = UUID()
    let date: Date
    let value: Double
}

struct AllocationData: Identifiable {
    let id = UUID()
    let asset: String
    let percentage: Double
    let valueUSD: Double
    let change24h: Double
    let color: Color
}

// MARK: - Extensions
extension PerformanceData.TimePeriod: CaseIterable {
    public static var allCases: [PerformanceData.TimePeriod] {
        [.day, .week, .month, .threeMonths, .year, .all]
    }
}

// MARK: - Preview
struct AnalyticsDashboardView_Previews: PreviewProvider {
    static var previews: some View {
        AnalyticsDashboardView()
    }
}
