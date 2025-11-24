//
//  BillSplitView.swift
//  EtridWallet
//
//  Main bill splitting view
//

import SwiftUI

struct BillSplitView: View {
    @StateObject private var viewModel = BillSplitViewModel()
    @State private var showingCreateBill = false
    @State private var selectedBill: SplitBill?
    @State private var showingBillDetail = false
    @State private var filterOption: BillFilter = .active

    enum BillFilter: String, CaseIterable {
        case all = "All"
        case active = "Active"
        case completed = "Completed"
        case created = "Created by Me"
    }

    var filteredBills: [SplitBill] {
        switch filterOption {
        case .all:
            return viewModel.bills
        case .active:
            return viewModel.bills.filter { $0.isActive && !$0.isCompleted }
        case .completed:
            return viewModel.bills.filter { $0.isCompleted }
        case .created:
            return viewModel.bills.filter { $0.creatorAddress == viewModel.userAddress }
        }
    }

    var body: some View {
        NavigationView {
            ZStack {
                LinearGradient(
                    colors: [Color(red: 0.1, green: 0.05, blue: 0.2), Color(red: 0.2, green: 0.1, blue: 0.3)],
                    startPoint: .topLeading,
                    endPoint: .bottomTrailing
                )
                .ignoresSafeArea()

                VStack(spacing: 0) {
                    // Stats Card
                    statsCard

                    // Filter Tabs
                    Picker("Filter", selection: $filterOption) {
                        ForEach(BillFilter.allCases, id: \.self) { option in
                            Text(option.rawValue).tag(option)
                        }
                    }
                    .pickerStyle(.segmented)
                    .padding()

                    // Bills List
                    if filteredBills.isEmpty {
                        emptyStateView
                    } else {
                        billsList
                    }
                }
            }
            .navigationTitle("Bill Split")
            .navigationBarTitleDisplayMode(.large)
            .toolbar {
                ToolbarItem(placement: .navigationBarTrailing) {
                    Button {
                        showingCreateBill = true
                    } label: {
                        Image(systemName: "plus")
                            .foregroundColor(.white)
                    }
                }
            }
            .sheet(isPresented: $showingCreateBill) {
                CreateBillView(viewModel: viewModel)
            }
            .sheet(isPresented: $showingBillDetail) {
                if let bill = selectedBill {
                    BillDetailView(bill: bill, viewModel: viewModel)
                }
            }
        }
    }

    private var statsCard: some View {
        HStack(spacing: 20) {
            StatItem(
                title: "Active Bills",
                value: "\(viewModel.activeBillsCount)",
                icon: "doc.text",
                color: .blue
            )

            Divider()
                .background(Color.white.opacity(0.3))
                .frame(height: 40)

            StatItem(
                title: "Total Owed",
                value: viewModel.totalOwedFormatted,
                icon: "arrow.up.circle",
                color: .orange
            )

            Divider()
                .background(Color.white.opacity(0.3))
                .frame(height: 40)

            StatItem(
                title: "Total Due",
                value: viewModel.totalDueFormatted,
                icon: "arrow.down.circle",
                color: .green
            )
        }
        .padding()
        .background(Color.white.opacity(0.05))
        .cornerRadius(16)
        .padding()
    }

    private var billsList: some View {
        ScrollView {
            LazyVStack(spacing: 12) {
                ForEach(filteredBills) { bill in
                    BillCard(bill: bill)
                        .onTapGesture {
                            selectedBill = bill
                            showingBillDetail = true
                        }
                }
            }
            .padding()
        }
    }

    private var emptyStateView: some View {
        VStack(spacing: 20) {
            Image(systemName: "doc.text.magnifyingglass")
                .font(.system(size: 60))
                .foregroundColor(.gray)

            Text("No bills found")
                .font(.title2)
                .foregroundColor(.white)

            Text("Create a bill to split expenses with friends")
                .font(.subheadline)
                .foregroundColor(.gray)
                .multilineTextAlignment(.center)

            Button {
                showingCreateBill = true
            } label: {
                Text("Create Bill")
                    .fontWeight(.semibold)
                    .foregroundColor(.white)
                    .padding(.horizontal, 24)
                    .padding(.vertical, 12)
                    .background(
                        LinearGradient(
                            colors: [Color.blue, Color.purple],
                            startPoint: .leading,
                            endPoint: .trailing
                        )
                    )
                    .cornerRadius(12)
            }
        }
        .frame(maxWidth: .infinity, maxHeight: .infinity)
        .padding()
    }
}

// MARK: - Stat Item
private struct StatItem: View {
    let title: String
    let value: String
    let icon: String
    let color: Color

    var body: some View {
        VStack(spacing: 8) {
            Image(systemName: icon)
                .font(.title3)
                .foregroundColor(color)

            Text(value)
                .font(.headline)
                .foregroundColor(.white)

            Text(title)
                .font(.caption2)
                .foregroundColor(.gray)
        }
        .frame(maxWidth: .infinity)
    }
}

// MARK: - Bill Card
private struct BillCard: View {
    let bill: SplitBill

    var body: some View {
        VStack(alignment: .leading, spacing: 12) {
            // Header
            HStack {
                if let category = bill.category {
                    Image(systemName: category.icon)
                        .foregroundColor(Color(category.color))
                }

                Text(bill.title)
                    .font(.headline)
                    .foregroundColor(.white)

                Spacer()

                if bill.isCompleted {
                    Image(systemName: "checkmark.circle.fill")
                        .foregroundColor(.green)
                }
            }

            // Amount
            HStack {
                VStack(alignment: .leading, spacing: 4) {
                    Text("Total Amount")
                        .font(.caption)
                        .foregroundColor(.gray)

                    Text("\(bill.formattedTotalAmount) \(bill.tokenSymbol)")
                        .font(.title3)
                        .fontWeight(.semibold)
                        .foregroundColor(.white)
                }

                Spacer()

                VStack(alignment: .trailing, spacing: 4) {
                    Text("\(bill.paidCount)/\(bill.participants.count) paid")
                        .font(.caption)
                        .foregroundColor(.gray)

                    Text("\(Int(bill.completionPercentage))%")
                        .font(.headline)
                        .foregroundColor(.white)
                }
            }

            // Progress Bar
            GeometryReader { geometry in
                ZStack(alignment: .leading) {
                    RoundedRectangle(cornerRadius: 4)
                        .fill(Color.white.opacity(0.1))
                        .frame(height: 8)

                    RoundedRectangle(cornerRadius: 4)
                        .fill(
                            LinearGradient(
                                colors: [Color.green, Color.blue],
                                startPoint: .leading,
                                endPoint: .trailing
                            )
                        )
                        .frame(width: geometry.size.width * CGFloat(bill.completionPercentage / 100), height: 8)
                }
            }
            .frame(height: 8)

            // Participants
            HStack {
                Image(systemName: "person.2")
                    .font(.caption)
                    .foregroundColor(.gray)

                Text("\(bill.participants.count) participants")
                    .font(.caption)
                    .foregroundColor(.gray)

                Spacer()

                Text(bill.createdAt.formatted(date: .abbreviated, time: .omitted))
                    .font(.caption)
                    .foregroundColor(.gray)
            }

            // Status Badge
            if bill.isExpired {
                HStack {
                    Image(systemName: "exclamationmark.triangle.fill")
                        .font(.caption)
                    Text("Expired")
                        .font(.caption)
                }
                .foregroundColor(.red)
                .padding(.horizontal, 8)
                .padding(.vertical, 4)
                .background(Color.red.opacity(0.2))
                .cornerRadius(8)
            }
        }
        .padding()
        .background(Color.white.opacity(0.05))
        .cornerRadius(16)
    }
}

// MARK: - View Model
class BillSplitViewModel: ObservableObject {
    @Published var bills: [SplitBill] = []
    @Published var paymentRequests: [PaymentRequest] = []

    let userAddress = "0x742d35Cc6634C0532925a3b844Bc9e7595f0bEb" // Current user's address

    init() {
        loadBills()
    }

    private func loadBills() {
        // Load from storage - placeholder data
        bills = [
            SplitBill(
                title: "Dinner at Restaurant",
                description: "Team dinner celebration",
                totalAmount: String(UInt64(10 * 1e18)),
                tokenSymbol: "ETR",
                splitType: .even,
                participants: [
                    Participant(name: "Alice", address: "0x1234...5678", amount: String(UInt64(5 * 1e18)), status: .paid),
                    Participant(name: "Bob", address: "0x5678...9012", amount: String(UInt64(5 * 1e18)), status: .pending)
                ],
                creatorAddress: userAddress,
                category: .food
            ),
            SplitBill(
                title: "Vacation House",
                description: "Weekend getaway rental",
                totalAmount: String(UInt64(5 * 1e18)),
                tokenSymbol: "ETR",
                splitType: .custom,
                participants: [
                    Participant(name: "Alice", address: "0x1234...5678", amount: String(UInt64(2 * 1e18)), status: .paid),
                    Participant(name: "Bob", address: "0x5678...9012", amount: String(UInt64(2 * 1e18)), status: .paid),
                    Participant(name: "Carol", address: "0x9012...3456", amount: String(UInt64(1 * 1e18)), status: .pending)
                ],
                creatorAddress: "0x1234...5678",
                category: .travel
            )
        ]
    }

    var activeBillsCount: Int {
        bills.filter { $0.isActive && !$0.isCompleted }.count
    }

    var totalOwedFormatted: String {
        let total = bills
            .filter { $0.creatorAddress == userAddress }
            .compactMap { Double($0.totalPending) }
            .reduce(0, +)
        return String(format: "%.2f", total / 1e18)
    }

    var totalDueFormatted: String {
        // Break down complex chain to avoid type-checking timeout
        let notCreatedByUser = bills.filter { $0.creatorAddress != userAddress }
        let allParticipants = notCreatedByUser.flatMap { $0.participants }
        let myPendingParticipations = allParticipants.filter { participant in
            participant.address.lowercased() == userAddress.lowercased() && participant.status == .pending
        }
        let amounts = myPendingParticipations.compactMap { Double($0.amount) }
        let total = amounts.reduce(0, +)
        return String(format: "%.2f", total / 1e18)
    }

    func addBill(_ bill: SplitBill) {
        bills.insert(bill, at: 0)
        // Save to storage
    }

    func updateBill(_ bill: SplitBill) {
        if let index = bills.firstIndex(where: { $0.id == bill.id }) {
            bills[index] = bill
            // Update storage
        }
    }

    func deleteBill(_ bill: SplitBill) {
        bills.removeAll { $0.id == bill.id }
        // Update storage
    }
}

struct BillSplitView_Previews: PreviewProvider {
    static var previews: some View {
        BillSplitView()
    }
}
