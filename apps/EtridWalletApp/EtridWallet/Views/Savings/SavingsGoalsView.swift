//
//  SavingsGoalsView.swift
//  EtridWallet
//
//  Savings goals management interface
//

import SwiftUI

struct SavingsGoalsView: View {
    @StateObject private var viewModel = SavingsGoalsViewModel()
    @Environment(\.dismiss) private var dismiss
    @State private var showCreateGoal = false
    @State private var selectedGoal: SavingsGoal?

    var body: some View {
        NavigationStack {
            ScrollView {
                VStack(spacing: 20) {
                    // Total Savings Card
                    VStack(spacing: 16) {
                        Text("Total Saved")
                            .font(.subheadline)
                            .foregroundColor(.white.opacity(0.8))

                        if viewModel.isLoading {
                            ProgressView()
                                .progressViewStyle(CircularProgressViewStyle(tint: .white))
                        } else {
                            Text(viewModel.totalSavedFormatted)
                                .font(.system(size: 36, weight: .bold))
                                .foregroundColor(.white)

                            Text("\(viewModel.activeGoalsCount) Active Goals")
                                .font(.subheadline)
                                .foregroundColor(.white.opacity(0.8))
                        }
                    }
                    .frame(maxWidth: .infinity)
                    .padding(.vertical, 30)
                    .background(
                        LinearGradient(
                            colors: [Color.etridOrange, Color.etridPurple],
                            startPoint: .topLeading,
                            endPoint: .bottomTrailing
                        )
                    )
                    .cornerRadius(20)
                    .padding(.horizontal)

                    // Goals List
                    VStack(alignment: .leading, spacing: 12) {
                        HStack {
                            Text("Your Goals")
                                .font(.headline)

                            Spacer()

                            Button(action: { showCreateGoal = true }) {
                                HStack(spacing: 4) {
                                    Image(systemName: "plus.circle.fill")
                                    Text("New Goal")
                                }
                                .font(.subheadline.bold())
                                .foregroundColor(.etridBlue)
                            }
                        }
                        .padding(.horizontal)

                        if viewModel.goals.isEmpty && !viewModel.isLoading {
                            VStack(spacing: 16) {
                                Image(systemName: "target")
                                    .font(.system(size: 60))
                                    .foregroundColor(.secondary)

                                Text("No Savings Goals Yet")
                                    .font(.headline)

                                Text("Create your first goal and start saving")
                                    .font(.subheadline)
                                    .foregroundColor(.secondary)

                                Button(action: { showCreateGoal = true }) {
                                    Text("Create Goal")
                                        .fontWeight(.semibold)
                                        .padding(.horizontal, 24)
                                        .padding(.vertical, 12)
                                        .background(Color.etridBlue)
                                        .foregroundColor(.white)
                                        .cornerRadius(20)
                                }
                            }
                            .frame(maxWidth: .infinity)
                            .padding(.vertical, 60)
                        } else {
                            ForEach(viewModel.goals) { goal in
                                GoalCard(goal: goal) {
                                    selectedGoal = goal
                                }
                            }
                            .padding(.horizontal)
                        }
                    }

                    // Achievements (if any)
                    if !viewModel.completedGoals.isEmpty {
                        VStack(alignment: .leading, spacing: 12) {
                            Text("Completed Goals")
                                .font(.headline)
                                .padding(.horizontal)

                            ForEach(viewModel.completedGoals) { goal in
                                CompletedGoalCard(goal: goal)
                            }
                            .padding(.horizontal)
                        }
                    }
                }
                .padding(.vertical)
            }
            .navigationTitle("Savings Goals")
            .navigationBarTitleDisplayMode(.inline)
            .toolbar {
                ToolbarItem(placement: .cancellationAction) {
                    Button("Close") {
                        dismiss()
                    }
                }
            }
            .sheet(isPresented: $showCreateGoal) {
                CreateGoalView()
            }
            .sheet(item: $selectedGoal) { goal in
                GoalDetailView(goal: goal)
            }
            .refreshable {
                await viewModel.loadGoals()
            }
        }
        .task {
            await viewModel.loadGoals()
        }
    }
}

struct GoalCard: View {
    let goal: SavingsGoal
    let action: () -> Void

    var body: some View {
        Button(action: action) {
            VStack(spacing: 16) {
                HStack {
                    // Category Icon
                    Circle()
                        .fill(categoryColor.opacity(0.2))
                        .frame(width: 50, height: 50)
                        .overlay(
                            Image(systemName: goal.category.iconName)
                                .font(.title3)
                                .foregroundColor(categoryColor)
                        )

                    VStack(alignment: .leading, spacing: 4) {
                        Text(goal.name)
                            .font(.headline)

                        HStack(spacing: 4) {
                            Text(goal.category.rawValue)
                                .font(.caption)
                                .foregroundColor(.secondary)

                            if let days = goal.daysRemaining {
                                Text("•")
                                    .foregroundColor(.secondary)
                                Text("\(days) days left")
                                    .font(.caption)
                                    .foregroundColor(days < 30 ? .etridOrange : .secondary)
                            }
                        }
                    }

                    Spacer()

                    // Status Badge
                    if goal.isCompleted {
                        Image(systemName: "checkmark.circle.fill")
                            .font(.title2)
                            .foregroundColor(.etridGreen)
                    } else if goal.isOverdue {
                        Image(systemName: "exclamationmark.triangle.fill")
                            .font(.title3)
                            .foregroundColor(.etridOrange)
                    }
                }

                // Progress Section
                VStack(spacing: 8) {
                    HStack {
                        VStack(alignment: .leading, spacing: 4) {
                            Text("Current")
                                .font(.caption)
                                .foregroundColor(.secondary)
                            Text(goal.formattedCurrent)
                                .font(.subheadline.bold())
                        }

                        Spacer()

                        VStack(alignment: .trailing, spacing: 4) {
                            Text("Target")
                                .font(.caption)
                                .foregroundColor(.secondary)
                            Text(goal.formattedTarget)
                                .font(.subheadline.bold())
                        }
                    }

                    // Progress Bar
                    GeometryReader { geometry in
                        ZStack(alignment: .leading) {
                            Rectangle()
                                .fill(Color(.systemGray5))
                                .frame(height: 8)

                            Rectangle()
                                .fill(
                                    LinearGradient(
                                        colors: [categoryColor, categoryColor.opacity(0.7)],
                                        startPoint: .leading,
                                        endPoint: .trailing
                                    )
                                )
                                .frame(width: geometry.size.width * (goal.progressPercentage / 100), height: 8)
                        }
                        .cornerRadius(4)
                    }
                    .frame(height: 8)

                    HStack {
                        Text(goal.progress.progressText)
                            .font(.caption.bold())
                            .foregroundColor(categoryColor)

                        Spacer()

                        Text("Remaining: \(goal.formattedRemaining)")
                            .font(.caption)
                            .foregroundColor(.secondary)
                    }
                }

                // Auto-save Rules
                if !goal.autoSaveRules.isEmpty {
                    VStack(alignment: .leading, spacing: 4) {
                        HStack {
                            Image(systemName: "arrow.triangle.2.circlepath")
                                .font(.caption)
                                .foregroundColor(.etridBlue)

                            Text("\(goal.autoSaveRules.count) Auto-save rules active")
                                .font(.caption)
                                .foregroundColor(.secondary)
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

    var categoryColor: Color {
        switch goal.category {
        case .emergency:
            return .etridRed
        case .vacation:
            return .etridBlue
        case .investment:
            return .etridGreen
        case .purchase:
            return .etridPurple
        case .education:
            return .etridOrange
        case .retirement:
            return .indigo
        case .custom:
            return .gray
        }
    }
}

struct CompletedGoalCard: View {
    let goal: SavingsGoal

    var body: some View {
        HStack(spacing: 12) {
            Image(systemName: "checkmark.seal.fill")
                .font(.title)
                .foregroundColor(.etridGreen)

            VStack(alignment: .leading, spacing: 4) {
                Text(goal.name)
                    .font(.headline)
                    .foregroundColor(.primary)

                Text("Completed \(goal.lastUpdated.relativeTime)")
                    .font(.caption)
                    .foregroundColor(.secondary)
            }

            Spacer()

            Text(goal.formattedTarget)
                .font(.subheadline.bold())
                .foregroundColor(.etridGreen)
        }
        .padding()
        .background(Color.etridGreen.opacity(0.05))
        .cornerRadius(12)
    }
}

@MainActor
class SavingsGoalsViewModel: ObservableObject {
    @Published var goals: [SavingsGoal] = []
    @Published var completedGoals: [SavingsGoal] = []
    @Published var isLoading = false

    var totalSaved: Double {
        goals.reduce(0) { $0 + $1.currentAmount }
    }

    var totalSavedFormatted: String {
        String(format: "$%.2f", totalSaved)
    }

    var activeGoalsCount: Int {
        goals.filter { !$0.isCompleted }.count
    }

    func loadGoals() async {
        isLoading = true

        // Load mock goals
        let mockGoals = SavingsGoal.mockGoals
        goals = mockGoals.filter { !$0.isCompleted }
        completedGoals = mockGoals.filter { $0.isCompleted }

        isLoading = false
    }
}

#Preview {
    SavingsGoalsView()
}
