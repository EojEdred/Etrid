//
//  GoalDetailView.swift
//  EtridWallet
//
//  Detailed view of a savings goal with deposit/withdraw functionality
//

import SwiftUI
import Charts

struct GoalDetailView: View {
    let goal: SavingsGoal
    @Environment(\.dismiss) private var dismiss
    @StateObject private var viewModel: GoalDetailViewModel
    @State private var showDeposit = false
    @State private var showWithdraw = false
    @State private var showEditGoal = false

    init(goal: SavingsGoal) {
        self.goal = goal
        _viewModel = StateObject(wrappedValue: GoalDetailViewModel(goal: goal))
    }

    var body: some View {
        NavigationStack {
            ScrollView {
                VStack(spacing: 24) {
                    // Goal Header
                    VStack(spacing: 16) {
                        Circle()
                            .fill(categoryColor.opacity(0.2))
                            .frame(width: 80, height: 80)
                            .overlay(
                                Image(systemName: goal.category.iconName)
                                    .font(.system(size: 40))
                                    .foregroundColor(categoryColor)
                            )

                        Text(goal.name)
                            .font(.title.bold())

                        Text(goal.category.rawValue)
                            .font(.subheadline)
                            .foregroundColor(.secondary)
                    }
                    .padding(.top)

                    // Progress Circle
                    VStack(spacing: 12) {
                        ZStack {
                            Circle()
                                .stroke(Color(.systemGray5), lineWidth: 12)
                                .frame(width: 200, height: 200)

                            Circle()
                                .trim(from: 0, to: goal.progressPercentage / 100)
                                .stroke(
                                    AngularGradient(
                                        colors: [categoryColor, categoryColor.opacity(0.5)],
                                        center: .center,
                                        startAngle: .degrees(0),
                                        endAngle: .degrees(360)
                                    ),
                                    style: StrokeStyle(lineWidth: 12, lineCap: .round)
                                )
                                .frame(width: 200, height: 200)
                                .rotationEffect(.degrees(-90))

                            VStack(spacing: 8) {
                                Text(String(format: "%.1f%%", goal.progressPercentage))
                                    .font(.system(size: 36, weight: .bold))

                                Text("Complete")
                                    .font(.caption)
                                    .foregroundColor(.secondary)
                            }
                        }

                        HStack(spacing: 40) {
                            VStack(spacing: 4) {
                                Text("Current")
                                    .font(.caption)
                                    .foregroundColor(.secondary)
                                Text(goal.formattedCurrent)
                                    .font(.headline)
                            }

                            VStack(spacing: 4) {
                                Text("Target")
                                    .font(.caption)
                                    .foregroundColor(.secondary)
                                Text(goal.formattedTarget)
                                    .font(.headline)
                            }
                        }
                    }
                    .padding()

                    // Action Buttons
                    HStack(spacing: 16) {
                        Button(action: { showDeposit = true }) {
                            HStack {
                                Image(systemName: "plus.circle.fill")
                                Text("Deposit")
                            }
                            .font(.headline)
                            .frame(maxWidth: .infinity)
                            .padding()
                            .background(categoryColor)
                            .foregroundColor(.white)
                            .cornerRadius(12)
                        }

                        Button(action: { showWithdraw = true }) {
                            HStack {
                                Image(systemName: "minus.circle.fill")
                                Text("Withdraw")
                            }
                            .font(.headline)
                            .frame(maxWidth: .infinity)
                            .padding()
                            .background(Color(.systemGray5))
                            .foregroundColor(.primary)
                            .cornerRadius(12)
                        }
                    }
                    .padding(.horizontal)

                    // Details Section
                    VStack(spacing: 16) {
                        if let days = goal.daysRemaining {
                            DetailCard(
                                icon: "calendar",
                                title: "Time Remaining",
                                value: "\(days) days",
                                color: days < 30 ? .etridOrange : categoryColor
                            )
                        }

                        DetailCard(
                            icon: "chart.line.uptrend.xyaxis",
                            title: "Remaining Amount",
                            value: goal.formattedRemaining,
                            color: categoryColor
                        )

                        if let completion = goal.estimatedCompletionDate {
                            DetailCard(
                                icon: "flag.checkered",
                                title: "Estimated Completion",
                                value: completion.shortDateTime,
                                color: categoryColor
                            )
                        }
                    }
                    .padding(.horizontal)

                    // Auto-save Rules
                    if !goal.autoSaveRules.isEmpty {
                        VStack(alignment: .leading, spacing: 12) {
                            Text("Auto-save Rules")
                                .font(.headline)
                                .padding(.horizontal)

                            VStack(spacing: 12) {
                                ForEach(goal.autoSaveRules) { rule in
                                    AutoSaveRuleCard(rule: rule)
                                }
                            }
                            .padding(.horizontal)
                        }
                    }

                    // Recent Activity
                    VStack(alignment: .leading, spacing: 12) {
                        HStack {
                            Text("Recent Activity")
                                .font(.headline)

                            Spacer()

                            Text("View All")
                                .font(.caption)
                                .foregroundColor(.etridBlue)
                        }
                        .padding(.horizontal)

                        if viewModel.recentDeposits.isEmpty {
                            Text("No deposits yet")
                                .font(.subheadline)
                                .foregroundColor(.secondary)
                                .frame(maxWidth: .infinity)
                                .padding()
                        } else {
                            VStack(spacing: 8) {
                                ForEach(viewModel.recentDeposits) { deposit in
                                    DepositRow(deposit: deposit, token: goal.token)
                                }
                            }
                            .padding(.horizontal)
                        }
                    }
                }
                .padding(.vertical)
            }
            .navigationBarTitleDisplayMode(.inline)
            .toolbar {
                ToolbarItem(placement: .cancellationAction) {
                    Button("Close") {
                        dismiss()
                    }
                }

                ToolbarItem(placement: .navigationBarTrailing) {
                    Menu {
                        Button(action: { showEditGoal = true }) {
                            Label("Edit Goal", systemImage: "pencil")
                        }

                        Button(role: .destructive, action: { viewModel.deleteGoal() }) {
                            Label("Delete Goal", systemImage: "trash")
                        }
                    } label: {
                        Image(systemName: "ellipsis.circle")
                    }
                }
            }
            .sheet(isPresented: $showDeposit) {
                DepositSheetView(goal: goal)
            }
            .sheet(isPresented: $showWithdraw) {
                WithdrawSheetView(goal: goal)
            }
        }
        .task {
            await viewModel.loadActivity()
        }
    }

    var categoryColor: Color {
        switch goal.category {
        case .emergency: return .etridRed
        case .vacation: return .etridBlue
        case .investment: return .etridGreen
        case .purchase: return .etridPurple
        case .education: return .etridOrange
        case .retirement: return .indigo
        case .custom: return .gray
        }
    }
}

struct DetailCard: View {
    let icon: String
    let title: String
    let value: String
    let color: Color

    var body: some View {
        HStack {
            Image(systemName: icon)
                .font(.title2)
                .foregroundColor(color)
                .frame(width: 40)

            VStack(alignment: .leading, spacing: 4) {
                Text(title)
                    .font(.caption)
                    .foregroundColor(.secondary)

                Text(value)
                    .font(.headline)
            }

            Spacer()
        }
        .padding()
        .background(Color(.systemGray6))
        .cornerRadius(12)
    }
}

struct AutoSaveRuleCard: View {
    let rule: AutoSaveRule

    var body: some View {
        HStack {
            Image(systemName: rule.isActive ? "checkmark.circle.fill" : "pause.circle.fill")
                .foregroundColor(rule.isActive ? .etridGreen : .secondary)

            VStack(alignment: .leading, spacing: 4) {
                Text(rule.type.rawValue)
                    .font(.subheadline.bold())

                Text(rule.description)
                    .font(.caption)
                    .foregroundColor(.secondary)
            }

            Spacer()
        }
        .padding()
        .background(Color(.systemGray6))
        .cornerRadius(12)
    }
}

struct DepositRow: View {
    let deposit: SavingsDeposit
    let token: Token

    var body: some View {
        HStack {
            Image(systemName: sourceIcon)
                .foregroundColor(sourceColor)
                .frame(width: 30)

            VStack(alignment: .leading, spacing: 4) {
                Text(deposit.source.rawValue)
                    .font(.subheadline)

                Text(deposit.timestamp.relativeTime)
                    .font(.caption)
                    .foregroundColor(.secondary)
            }

            Spacer()

            Text("+\(deposit.formattedAmount) \(token.symbol)")
                .font(.subheadline.bold())
                .foregroundColor(.etridGreen)
        }
        .padding()
        .background(Color(.systemGray6))
        .cornerRadius(12)
    }

    var sourceIcon: String {
        switch deposit.source {
        case .manual: return "hand.tap.fill"
        case .autoSave: return "arrow.triangle.2.circlepath"
        case .roundUp: return "arrow.up.circle.fill"
        case .milestone: return "flag.fill"
        }
    }

    var sourceColor: Color {
        switch deposit.source {
        case .manual: return .etridBlue
        case .autoSave: return .etridGreen
        case .roundUp: return .etridOrange
        case .milestone: return .etridPurple
        }
    }
}

struct DepositSheetView: View {
    let goal: SavingsGoal
    @Environment(\.dismiss) private var dismiss
    @State private var amount: String = ""

    var body: some View {
        NavigationStack {
            VStack(spacing: 24) {
                Text("Deposit to \(goal.name)")
                    .font(.title2.bold())
                    .padding(.top)

                VStack(alignment: .leading, spacing: 8) {
                    Text("Amount")
                        .font(.subheadline)
                        .foregroundColor(.secondary)

                    HStack {
                        TextField("0.0", text: $amount)
                            .keyboardType(.decimalPad)
                            .font(.title)

                        Text(goal.token.symbol)
                            .font(.headline)
                            .foregroundColor(.secondary)
                    }
                    .padding()
                    .background(Color(.systemGray6))
                    .cornerRadius(12)
                }

                Spacer()

                Button("Deposit") {
                    // Execute deposit
                    dismiss()
                }
                .font(.headline)
                .frame(maxWidth: .infinity)
                .padding()
                .background(amount.isEmpty ? Color.gray : Color.etridGreen)
                .foregroundColor(.white)
                .cornerRadius(12)
                .disabled(amount.isEmpty)
            }
            .padding()
            .navigationBarTitleDisplayMode(.inline)
            .toolbar {
                ToolbarItem(placement: .cancellationAction) {
                    Button("Cancel") { dismiss() }
                }
            }
        }
    }
}

struct WithdrawSheetView: View {
    let goal: SavingsGoal
    @Environment(\.dismiss) private var dismiss
    @State private var amount: String = ""

    var body: some View {
        NavigationStack {
            VStack(spacing: 24) {
                Text("Withdraw from \(goal.name)")
                    .font(.title2.bold())
                    .padding(.top)

                VStack(alignment: .leading, spacing: 8) {
                    Text("Amount")
                        .font(.subheadline)
                        .foregroundColor(.secondary)

                    HStack {
                        TextField("0.0", text: $amount)
                            .keyboardType(.decimalPad)
                            .font(.title)

                        Text(goal.token.symbol)
                            .font(.headline)
                            .foregroundColor(.secondary)

                        Button("Max") {
                            amount = String(goal.currentAmount)
                        }
                        .font(.caption.bold())
                        .padding(.horizontal, 12)
                        .padding(.vertical, 6)
                        .background(Color.etridBlue)
                        .foregroundColor(.white)
                        .cornerRadius(8)
                    }
                    .padding()
                    .background(Color(.systemGray6))
                    .cornerRadius(12)

                    Text("Available: \(goal.formattedCurrent)")
                        .font(.caption)
                        .foregroundColor(.secondary)
                }

                Spacer()

                Button("Withdraw") {
                    // Execute withdrawal
                    dismiss()
                }
                .font(.headline)
                .frame(maxWidth: .infinity)
                .padding()
                .background(amount.isEmpty ? Color.gray : Color.etridRed)
                .foregroundColor(.white)
                .cornerRadius(12)
                .disabled(amount.isEmpty)
            }
            .padding()
            .navigationBarTitleDisplayMode(.inline)
            .toolbar {
                ToolbarItem(placement: .cancellationAction) {
                    Button("Cancel") { dismiss() }
                }
            }
        }
    }
}

@MainActor
class GoalDetailViewModel: ObservableObject {
    let goal: SavingsGoal
    @Published var recentDeposits: [SavingsDeposit] = []

    init(goal: SavingsGoal) {
        self.goal = goal
    }

    func loadActivity() async {
        // Mock recent deposits
        recentDeposits = [
            SavingsDeposit(goalId: goal.id, amount: 0.5, source: .manual, timestamp: Date().addingTimeInterval(-86400)),
            SavingsDeposit(goalId: goal.id, amount: 0.1, source: .autoSave, timestamp: Date().addingTimeInterval(-172800)),
            SavingsDeposit(goalId: goal.id, amount: 0.25, source: .roundUp, timestamp: Date().addingTimeInterval(-259200))
        ]
    }

    func deleteGoal() {
        // Implement delete logic
        print("Deleting goal: \(goal.name)")
    }
}

#Preview {
    GoalDetailView(goal: SavingsGoal.mockGoals[0])
}
