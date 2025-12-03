//
//  BusinessDashboardView.swift
//  EtridWallet
//
//  Native iOS implementation of Business Dashboard
//

import SwiftUI

struct BusinessDashboardView: View {
    @StateObject private var viewModel = BusinessDashboardViewModel()
    @Environment(\.dismiss) private var dismiss

    var body: some View {
        NavigationView {
            ZStack {
                if viewModel.isLoading && viewModel.metrics == nil {
                    ProgressView("Loading business data...")
                } else {
                    ScrollView {
                        VStack(spacing: 20) {
                            // Error Alert
                            if let error = viewModel.errorMessage {
                                BusinessErrorBanner(message: error, onDismiss: {
                                    viewModel.errorMessage = nil
                                })
                            }

                            // Stats Cards Grid
                            statsGrid

                            // Quick Actions
                            quickActions

                            // Revenue Chart
                            revenueChart

                            // Recent Invoices
                            recentInvoices

                            // Merchant Tools
                            merchantTools
                        }
                        .padding()
                    }
                    .refreshable {
                        await viewModel.loadData()
                    }
                }
            }
            .navigationTitle("Business Dashboard")
            .navigationBarTitleDisplayMode(.large)
            .toolbar {
                ToolbarItem(placement: .navigationBarLeading) {
                    Button(action: { dismiss() }) {
                        Image(systemName: "chevron.left")
                            .foregroundColor(.blue)
                    }
                }

                ToolbarItem(placement: .navigationBarTrailing) {
                    if viewModel.isLoading {
                        ProgressView()
                            .progressViewStyle(CircularProgressViewStyle())
                    }
                }
            }
        }
        .task {
            await viewModel.loadData()
        }
    }

    // MARK: - Stats Grid
    private var statsGrid: some View {
        LazyVGrid(columns: [GridItem(.flexible()), GridItem(.flexible())], spacing: 12) {
            BusinessStatCard(
                label: "Monthly Revenue",
                value: viewModel.metrics != nil ? "$\(formatNumber(viewModel.metrics!.monthlyRevenue))" : "---",
                change: "+12% from last month",
                isPrimary: true
            )

            BusinessStatCard(
                label: "Outstanding Invoices",
                value: viewModel.metrics != nil ? "\(viewModel.metrics!.outstandingInvoices)" : "---",
                change: "$\(formatNumber(viewModel.outstandingAmount)) pending",
                isPrimary: false
            )

            BusinessStatCard(
                label: "Team Members",
                value: viewModel.metrics != nil ? "\(viewModel.metrics!.teamMembers)" : "---",
                change: "Active payroll",
                isPrimary: false
            )

            BusinessStatCard(
                label: "Monthly Expenses",
                value: viewModel.metrics != nil ? "$\(formatNumber(viewModel.metrics!.monthlyExpenses))" : "---",
                change: "-5% from last month",
                isPrimary: false
            )
        }
    }

    // MARK: - Quick Actions
    private var quickActions: some View {
        VStack(alignment: .leading, spacing: 12) {
            Text("Quick Actions")
                .font(.headline)
                .fontWeight(.bold)

            LazyVGrid(columns: [GridItem(.flexible()), GridItem(.flexible())], spacing: 12) {
                ActionButton(
                    icon: "doc.text",
                    title: "Create Invoice",
                    action: { viewModel.showCreateInvoice() }
                )
                ActionButton(
                    icon: "dollarsign.circle",
                    title: "Run Payroll",
                    action: { viewModel.showRunPayroll() }
                )
                ActionButton(
                    icon: "chart.bar",
                    title: "Track Expenses",
                    action: { viewModel.showTrackExpenses() }
                )
                ActionButton(
                    icon: "person.2",
                    title: "Add Team",
                    action: { viewModel.showAddTeam() }
                )
            }
        }
    }

    // MARK: - Revenue Chart
    private var revenueChart: some View {
        VStack(alignment: .leading, spacing: 12) {
            Text("Revenue Trend")
                .font(.headline)
                .fontWeight(.bold)

            ZStack {
                RoundedRectangle(cornerRadius: 16)
                    .fill(Color(.systemGray6))
                    .frame(height: 200)

                HStack(alignment: .bottom, spacing: 20) {
                    ForEach(viewModel.revenueData) { data in
                        VStack {
                            RoundedRectangle(cornerRadius: 4)
                                .fill(Color.blue)
                                .frame(width: 30, height: CGFloat(data.revenue) / 60)

                            Text(data.month)
                                .font(.caption2)
                                .foregroundColor(.secondary)
                        }
                    }
                }
                .padding()
            }
        }
    }

    // MARK: - Recent Invoices
    private var recentInvoices: some View {
        VStack(alignment: .leading, spacing: 12) {
            HStack {
                Text("Recent Invoices")
                    .font(.headline)
                    .fontWeight(.bold)

                Spacer()

                Button("See All") {
                    viewModel.showAllInvoices()
                }
                .font(.subheadline)
                .foregroundColor(.blue)
            }

            if viewModel.invoices.isEmpty {
                Text("No invoices yet")
                    .font(.subheadline)
                    .foregroundColor(.secondary)
                    .frame(maxWidth: .infinity, alignment: .center)
                    .padding()
            } else {
                ForEach(viewModel.invoices.prefix(3)) { invoice in
                    InvoiceRow(invoice: invoice)
                }
            }
        }
    }

    // MARK: - Merchant Tools
    private var merchantTools: some View {
        VStack(alignment: .leading, spacing: 12) {
            Text("Merchant Tools")
                .font(.headline)
                .fontWeight(.bold)

            ToolRow(
                icon: "🏪",
                title: "Point of Sale",
                description: "Accept crypto payments in-person",
                action: { viewModel.showPOS() }
            )
            ToolRow(
                icon: "🔗",
                title: "Payment Links",
                description: "Generate shareable payment links",
                action: { viewModel.showPaymentLinks() }
            )
            ToolRow(
                icon: "📦",
                title: "Product Catalog",
                description: "Manage your inventory",
                action: { viewModel.showProductCatalog() }
            )
        }
    }

    // MARK: - Helper Functions
    private func formatNumber(_ number: Double) -> String {
        let formatter = NumberFormatter()
        formatter.numberStyle = .decimal
        formatter.maximumFractionDigits = 0
        return formatter.string(from: NSNumber(value: number)) ?? "\(Int(number))"
    }
}

// MARK: - Error Banner Component
struct BusinessErrorBanner: View {
    let message: String
    let onDismiss: () -> Void

    var body: some View {
        HStack {
            Image(systemName: "exclamationmark.triangle.fill")
                .foregroundColor(.orange)

            Text(message)
                .font(.subheadline)
                .foregroundColor(.primary)

            Spacer()

            Button(action: onDismiss) {
                Image(systemName: "xmark.circle.fill")
                    .foregroundColor(.secondary)
            }
        }
        .padding()
        .background(Color.orange.opacity(0.1))
        .cornerRadius(12)
    }
}

// MARK: - Stat Card Component
struct BusinessStatCard: View {
    let label: String
    let value: String
    let change: String
    let isPrimary: Bool

    var body: some View {
        VStack(alignment: .leading, spacing: 8) {
            Text(label)
                .font(.caption)
                .foregroundColor(.secondary)

            Text(value)
                .font(.title2)
                .fontWeight(.bold)

            Text(change)
                .font(.caption)
                .foregroundColor(.green)
        }
        .frame(maxWidth: .infinity, alignment: .leading)
        .padding()
        .background(isPrimary ? Color.blue.opacity(0.1) : Color(.systemGray6))
        .cornerRadius(16)
        .overlay(
            RoundedRectangle(cornerRadius: 16)
                .stroke(isPrimary ? Color.blue : Color.clear, lineWidth: 1)
        )
    }
}

// MARK: - Action Button Component
struct ActionButton: View {
    let icon: String
    let title: String
    let action: () -> Void

    var body: some View {
        Button(action: action) {
            VStack(spacing: 8) {
                Image(systemName: icon)
                    .font(.title)
                    .foregroundColor(.blue)

                Text(title)
                    .font(.caption)
                    .fontWeight(.semibold)
                    .foregroundColor(.primary)
                    .multilineTextAlignment(.center)
            }
            .frame(maxWidth: .infinity)
            .padding()
            .background(Color(.systemGray6))
            .cornerRadius(12)
        }
    }
}

// MARK: - Invoice Row Component
struct InvoiceRow: View {
    let invoice: Invoice

    var body: some View {
        HStack {
            VStack(alignment: .leading, spacing: 4) {
                Text(invoice.client)
                    .font(.body)
                    .fontWeight(.semibold)

                Text(formatDate(invoice.createdAt))
                    .font(.caption)
                    .foregroundColor(.secondary)
            }

            Spacer()

            VStack(alignment: .trailing, spacing: 4) {
                Text("$\(formatAmount(invoice.amount))")
                    .font(.body)
                    .fontWeight(.bold)

                Text(invoice.status.rawValue.uppercased())
                    .font(.caption2)
                    .fontWeight(.semibold)
                    .padding(.horizontal, 8)
                    .padding(.vertical, 4)
                    .background(statusColor(for: invoice.status).opacity(0.2))
                    .foregroundColor(statusColor(for: invoice.status))
                    .cornerRadius(6)
            }
        }
        .padding()
        .background(Color(.systemGray6))
        .cornerRadius(12)
    }

    private func statusColor(for status: Invoice.InvoiceStatus) -> Color {
        switch status {
        case .paid: return .green
        case .sent, .draft: return .orange
        case .overdue: return .red
        case .cancelled: return .gray
        }
    }

    private func formatDate(_ date: Date) -> String {
        let formatter = DateFormatter()
        formatter.dateStyle = .medium
        formatter.timeStyle = .none
        return formatter.string(from: date)
    }

    private func formatAmount(_ amount: Double) -> String {
        let formatter = NumberFormatter()
        formatter.numberStyle = .decimal
        formatter.maximumFractionDigits = 2
        return formatter.string(from: NSNumber(value: amount)) ?? "\(amount)"
    }
}

// MARK: - Tool Row Component
struct ToolRow: View {
    let icon: String
    let title: String
    let description: String
    let action: () -> Void

    var body: some View {
        Button(action: action) {
            HStack(spacing: 16) {
                Text(icon)
                    .font(.largeTitle)

                VStack(alignment: .leading, spacing: 4) {
                    Text(title)
                        .font(.body)
                        .fontWeight(.semibold)
                        .foregroundColor(.primary)

                    Text(description)
                        .font(.caption)
                        .foregroundColor(.secondary)
                }

                Spacer()

                Image(systemName: "chevron.right")
                    .foregroundColor(.secondary)
            }
            .padding()
            .background(Color(.systemGray6))
            .cornerRadius(12)
        }
    }
}

// MARK: - View Model
@MainActor
class BusinessDashboardViewModel: ObservableObject {
    @Published var metrics: BusinessMetrics?
    @Published var invoices: [Invoice] = []
    @Published var isLoading = false
    @Published var errorMessage: String?

    // Revenue chart data (mock for now - could be fetched from backend in future)
    let revenueData: [RevenueData] = [
        RevenueData(month: "Jan", revenue: 4000),
        RevenueData(month: "Feb", revenue: 3000),
        RevenueData(month: "Mar", revenue: 5000),
        RevenueData(month: "Apr", revenue: 4500),
        RevenueData(month: "May", revenue: 6000),
        RevenueData(month: "Jun", revenue: 5500)
    ]

    private let businessService = BusinessService.shared

    // TODO: Replace with actual user ID from authentication service
    private let userId = "user_123"

    var outstandingAmount: Double {
        invoices
            .filter { $0.status != .paid && $0.status != .cancelled }
            .reduce(0) { $0 + $1.amount }
    }

    // MARK: - Data Loading
    func loadData() async {
        isLoading = true
        errorMessage = nil

        await loadMetrics()
        await loadInvoices()

        isLoading = false
    }

    private func loadMetrics() async {
        do {
            metrics = try await businessService.getMetrics(for: userId)
        } catch {
            errorMessage = "Failed to load metrics: \(error.localizedDescription)"
            print("Error loading metrics: \(error)")
        }
    }

    private func loadInvoices() async {
        do {
            invoices = try await businessService.getInvoices(for: userId)
        } catch {
            errorMessage = "Failed to load invoices: \(error.localizedDescription)"
            print("Error loading invoices: \(error)")
        }
    }

    // MARK: - Actions
    func showCreateInvoice() {
        Task {
            do {
                // Example: Create a new invoice
                let dueDate = Calendar.current.date(byAdding: .day, value: 30, to: Date()) ?? Date()

                let invoice = try await businessService.createInvoice(
                    userId: userId,
                    client: "New Client",
                    amount: 5000.0,
                    dueDate: dueDate,
                    items: []
                )

                // Reload invoices after creation
                await loadInvoices()

                print("Invoice created: \(invoice.invoiceNumber)")
            } catch {
                errorMessage = "Failed to create invoice: \(error.localizedDescription)"
                print("Error creating invoice: \(error)")
            }
        }
    }

    func showRunPayroll() {
        Task {
            do {
                // Example: Run payroll for all team members
                // In a real app, you'd select specific members or use all
                try await businessService.runPayroll(userId: userId, members: [])

                // Show success message
                print("Payroll run successfully")
            } catch {
                errorMessage = "Failed to run payroll: \(error.localizedDescription)"
                print("Error running payroll: \(error)")
            }
        }
    }

    func showTrackExpenses() {
        Task {
            do {
                // Example: Track a new expense
                let expense = try await businessService.trackExpense(
                    userId: userId,
                    category: "Office Supplies",
                    description: "New expense",
                    amount: 150.0,
                    merchant: "Office Depot"
                )

                // Reload metrics after tracking expense
                await loadMetrics()

                print("Expense tracked: \(expense.id)")
            } catch {
                errorMessage = "Failed to track expense: \(error.localizedDescription)"
                print("Error tracking expense: \(error)")
            }
        }
    }

    func showAddTeam() {
        // Navigate to add team member view
        print("Show add team member view")
    }

    func showAllInvoices() {
        // Navigate to all invoices view
        print("Show all invoices view")
    }

    func showPOS() {
        // Navigate to Point of Sale view
        print("Show Point of Sale view")
    }

    func showPaymentLinks() {
        Task {
            do {
                // Example: Create a payment link
                let paymentLink = try await businessService.createPaymentLink(
                    userId: userId,
                    title: "Product Payment",
                    amount: 100.0,
                    currency: "USD"
                )

                print("Payment link created: \(paymentLink.url)")
            } catch {
                errorMessage = "Failed to create payment link: \(error.localizedDescription)"
                print("Error creating payment link: \(error)")
            }
        }
    }

    func showProductCatalog() {
        // Navigate to product catalog view
        print("Show product catalog view")
    }
}

// MARK: - Models
struct RevenueData: Identifiable {
    let id = UUID()
    let month: String
    let revenue: Double
}

// MARK: - Helper Types
struct EmptyResponse: Decodable {}

// MARK: - Preview
struct BusinessDashboardView_Previews: PreviewProvider {
    static var previews: some View {
        BusinessDashboardView()
    }
}
