//
//  CreateGoalView.swift
//  EtridWallet
//
//  Create new savings goal wizard
//

import SwiftUI

struct CreateGoalView: View {
    @Environment(\.dismiss) private var dismiss
    @StateObject private var viewModel = CreateGoalViewModel()

    var body: some View {
        NavigationStack {
            Form {
                // Basic Info Section
                Section {
                    TextField("Goal Name", text: $viewModel.name)

                    Picker("Category", selection: $viewModel.category) {
                        ForEach(GoalCategory.allCases, id: \.self) { category in
                            HStack {
                                Image(systemName: category.iconName)
                                Text(category.rawValue)
                            }
                            .tag(category)
                        }
                    }
                } header: {
                    Text("Goal Details")
                }

                // Amount Section
                Section {
                    HStack {
                        Text("Target Amount")
                        Spacer()
                        TextField("0.0", text: $viewModel.targetAmount)
                            .keyboardType(.decimalPad)
                            .multilineTextAlignment(.trailing)
                        Text(viewModel.selectedToken.symbol)
                            .foregroundColor(.secondary)
                    }

                    Picker("Token", selection: $viewModel.selectedToken) {
                        ForEach(viewModel.availableTokens) { token in
                            Text(token.symbol).tag(token)
                        }
                    }
                } header: {
                    Text("Target")
                }

                // Deadline Section
                Section {
                    Toggle("Set Deadline", isOn: $viewModel.hasDeadline)

                    if viewModel.hasDeadline {
                        DatePicker("Deadline", selection: $viewModel.deadline, in: Date()..., displayedComponents: .date)
                    }
                } header: {
                    Text("Timeline")
                }

                // Auto-save Rules
                Section {
                    ForEach(viewModel.autoSaveRules) { rule in
                        AutoSaveRuleRow(rule: rule) {
                            viewModel.removeRule(rule)
                        }
                    }

                    Button(action: { viewModel.showAddRule = true }) {
                        HStack {
                            Image(systemName: "plus.circle.fill")
                                .foregroundColor(.etridBlue)
                            Text("Add Auto-save Rule")
                        }
                    }
                } header: {
                    Text("Auto-save Rules (Optional)")
                } footer: {
                    Text("Auto-save rules help you reach your goal automatically")
                }

                // Summary
                if viewModel.canCreate {
                    Section {
                        VStack(alignment: .leading, spacing: 12) {
                            HStack {
                                Text("Goal Summary")
                                    .font(.headline)
                                Spacer()
                            }

                            SummaryRow(title: "Target", value: viewModel.formattedTarget)

                            if let deadline = viewModel.goalDeadline {
                                SummaryRow(title: "Deadline", value: deadline.shortDateTime)
                            }

                            if !viewModel.autoSaveRules.isEmpty {
                                SummaryRow(
                                    title: "Est. Monthly Savings",
                                    value: String(format: "%.2f %@", viewModel.estimatedMonthlySavings, viewModel.selectedToken.symbol)
                                )

                                if let completion = viewModel.estimatedCompletion {
                                    SummaryRow(title: "Est. Completion", value: completion.shortDateTime)
                                }
                            }
                        }
                    }
                }
            }
            .navigationTitle("Create Goal")
            .navigationBarTitleDisplayMode(.inline)
            .toolbar {
                ToolbarItem(placement: .cancellationAction) {
                    Button("Cancel") {
                        dismiss()
                    }
                }

                ToolbarItem(placement: .confirmationAction) {
                    Button("Create") {
                        viewModel.createGoal()
                        dismiss()
                    }
                    .disabled(!viewModel.canCreate)
                }
            }
            .sheet(isPresented: $viewModel.showAddRule) {
                AddAutoSaveRuleView { rule in
                    viewModel.addRule(rule)
                }
            }
        }
    }
}

struct AutoSaveRuleRow: View {
    let rule: AutoSaveRule
    let onDelete: () -> Void

    var body: some View {
        HStack {
            VStack(alignment: .leading, spacing: 4) {
                Text(rule.type.rawValue)
                    .font(.subheadline.bold())

                Text(rule.description)
                    .font(.caption)
                    .foregroundColor(.secondary)
            }

            Spacer()

            Button(action: onDelete) {
                Image(systemName: "trash")
                    .foregroundColor(.etridRed)
            }
        }
    }
}

struct SummaryRow: View {
    let title: String
    let value: String

    var body: some View {
        HStack {
            Text(title)
                .foregroundColor(.secondary)
            Spacer()
            Text(value)
                .fontWeight(.medium)
        }
        .font(.subheadline)
    }
}

struct AddAutoSaveRuleView: View {
    @Environment(\.dismiss) private var dismiss
    let onAdd: (AutoSaveRule) -> Void

    @State private var selectedType: AutoSaveType = .fixed
    @State private var amount: String = ""
    @State private var frequency: SaveFrequency = .weekly

    var canAdd: Bool {
        !amount.isEmpty && Double(amount) != nil && Double(amount)! > 0
    }

    var body: some View {
        NavigationStack {
            Form {
                Section {
                    Picker("Rule Type", selection: $selectedType) {
                        ForEach([AutoSaveType.fixed, .percentage, .roundUp], id: \.self) { type in
                            Text(type.rawValue).tag(type)
                        }
                    }
                    .pickerStyle(.segmented)

                    Text(selectedType.description)
                        .font(.caption)
                        .foregroundColor(.secondary)
                } header: {
                    Text("Type")
                }

                Section {
                    HStack {
                        TextField(selectedType == .percentage ? "Percentage" : "Amount", text: $amount)
                            .keyboardType(.decimalPad)

                        Text(selectedType == .percentage ? "%" : "")
                            .foregroundColor(.secondary)
                    }
                } header: {
                    Text("Amount")
                }

                Section {
                    Picker("Frequency", selection: $frequency) {
                        ForEach(SaveFrequency.allCases, id: \.self) { freq in
                            Text(freq.rawValue).tag(freq)
                        }
                    }
                } header: {
                    Text("Frequency")
                }
            }
            .navigationTitle("Add Rule")
            .navigationBarTitleDisplayMode(.inline)
            .toolbar {
                ToolbarItem(placement: .cancellationAction) {
                    Button("Cancel") {
                        dismiss()
                    }
                }

                ToolbarItem(placement: .confirmationAction) {
                    Button("Add") {
                        if let amountValue = Double(amount) {
                            let rule = AutoSaveRule(
                                type: selectedType,
                                amount: amountValue,
                                frequency: frequency
                            )
                            onAdd(rule)
                            dismiss()
                        }
                    }
                    .disabled(!canAdd)
                }
            }
        }
    }
}

@MainActor
class CreateGoalViewModel: ObservableObject {
    @Published var name: String = ""
    @Published var category: GoalCategory = .emergency
    @Published var targetAmount: String = ""
    @Published var selectedToken: Token = Token(symbol: "ETH", name: "Ethereum", decimals: 18, isNative: true)
    @Published var hasDeadline: Bool = false
    @Published var deadline: Date = Calendar.current.date(byAdding: .month, value: 6, to: Date()) ?? Date()
    @Published var autoSaveRules: [AutoSaveRule] = []
    @Published var showAddRule: Bool = false

    var availableTokens: [Token] {
        [
            Token(symbol: "ETH", name: "Ethereum", decimals: 18, isNative: true),
            Token(symbol: "USDC", name: "USD Coin", contractAddress: "0xA0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48", decimals: 6),
            Token(symbol: "DAI", name: "Dai Stablecoin", contractAddress: "0x6B175474E89094C44Da98b954EedeAC495271d0F", decimals: 18)
        ]
    }

    var canCreate: Bool {
        !name.isEmpty && !targetAmount.isEmpty && Double(targetAmount) != nil && Double(targetAmount)! > 0
    }

    var formattedTarget: String {
        guard let amount = Double(targetAmount) else { return "" }
        return String(format: "%.4f %@", amount, selectedToken.symbol)
    }

    var goalDeadline: Date? {
        hasDeadline ? deadline : nil
    }

    var estimatedMonthlySavings: Double {
        autoSaveRules.reduce(0) { $0 + $1.estimatedMonthlyAmount }
    }

    var estimatedCompletion: Date? {
        guard let target = Double(targetAmount), estimatedMonthlySavings > 0 else { return nil }
        let months = Int(ceil(target / estimatedMonthlySavings))
        return Calendar.current.date(byAdding: .month, value: months, to: Date())
    }

    func addRule(_ rule: AutoSaveRule) {
        autoSaveRules.append(rule)
    }

    func removeRule(_ rule: AutoSaveRule) {
        autoSaveRules.removeAll { $0.id == rule.id }
    }

    func createGoal() {
        guard let targetValue = Double(targetAmount) else { return }

        let goal = SavingsGoal(
            name: name,
            targetAmount: targetValue,
            token: selectedToken,
            deadline: goalDeadline,
            category: category,
            imageIcon: category.iconName,
            autoSaveRules: autoSaveRules
        )

        // Save goal (implement actual saving logic)
        print("Creating goal: \(goal)")
    }
}

#Preview {
    CreateGoalView()
}
