//
//  CreateProposalView.swift
//  EtridWallet
//
//  Create new governance proposals
//

import SwiftUI

struct CreateProposalView: View {
    @ObservedObject var viewModel: ConsensusDayViewModel
    @Environment(\.dismiss) private var dismiss

    @State private var title = ""
    @State private var description = ""
    @State private var selectedCategory: ProposalCategory = .parameterChange
    @State private var budgetAmount = ""
    @State private var showBudgetField = false
    @State private var showValidationError = false
    @State private var validationMessage = ""

    var body: some View {
        NavigationView {
            ScrollView {
                VStack(alignment: .leading, spacing: 24) {
                    // Title
                    VStack(alignment: .leading, spacing: 8) {
                        Text("Title")
                            .font(.headline)
                            .foregroundColor(.white)

                        TextField("Enter proposal title", text: $title)
                            .textFieldStyle(CustomTextFieldStyle())
                    }

                    // Category
                    VStack(alignment: .leading, spacing: 8) {
                        Text("Category")
                            .font(.headline)
                            .foregroundColor(.white)

                        Menu {
                            ForEach(ProposalCategory.allCases) { category in
                                Button {
                                    selectedCategory = category
                                    showBudgetField = category == .treasuryAllocation
                                } label: {
                                    HStack {
                                        Image(systemName: category.icon)
                                        Text(category.rawValue)
                                        if category.requiresSupermajority {
                                            Text("(66% required)")
                                                .font(.caption)
                                        }
                                    }
                                }
                            }
                        } label: {
                            HStack {
                                Image(systemName: selectedCategory.icon)
                                    .foregroundColor(Color(selectedCategory.color))

                                Text(selectedCategory.rawValue)
                                    .foregroundColor(.white)

                                Spacer()

                                Image(systemName: "chevron.down")
                                    .foregroundColor(.gray)
                            }
                            .padding()
                            .background(Color.white.opacity(0.1))
                            .cornerRadius(12)
                        }

                        if selectedCategory.requiresSupermajority {
                            Text("⚠️ This category requires 66% approval")
                                .font(.caption)
                                .foregroundColor(.orange)
                        }
                    }

                    // Budget (if treasury allocation)
                    if showBudgetField {
                        VStack(alignment: .leading, spacing: 8) {
                            Text("Budget Request")
                                .font(.headline)
                                .foregroundColor(.white)

                            HStack {
                                TextField("Amount", text: $budgetAmount)
                                    .keyboardType(.decimalPad)
                                    .textFieldStyle(CustomTextFieldStyle())

                                Text("ËTR")
                                    .foregroundColor(.gray)
                                    .padding(.trailing, 12)
                            }
                        }
                    }

                    // Description
                    VStack(alignment: .leading, spacing: 8) {
                        Text("Description")
                            .font(.headline)
                            .foregroundColor(.white)

                        TextEditor(text: $description)
                            .frame(height: 150)
                            .padding(8)
                            .background(Color.white.opacity(0.1))
                            .cornerRadius(12)
                            .foregroundColor(.white)
                    }

                    // Info Box
                    VStack(alignment: .leading, spacing: 12) {
                        HStack(spacing: 8) {
                            Image(systemName: "info.circle.fill")
                                .foregroundColor(.blue)

                            Text("Proposal Guidelines")
                                .font(.headline)
                                .foregroundColor(.white)
                        }

                        VStack(alignment: .leading, spacing: 8) {
                            InfoRow(icon: "checkmark.circle", text: "Be clear and specific")
                            InfoRow(icon: "checkmark.circle", text: "Include implementation details")
                            InfoRow(icon: "checkmark.circle", text: "Consider community impact")
                            InfoRow(icon: "checkmark.circle", text: "Voting period: 12 hours")
                        }
                    }
                    .padding()
                    .background(Color.blue.opacity(0.1))
                    .cornerRadius(12)

                    // Submit Button
                    Button(action: submitProposal) {
                        Text("Submit Proposal")
                            .font(.headline)
                            .foregroundColor(.white)
                            .frame(maxWidth: .infinity)
                            .padding()
                            .background(
                                LinearGradient(
                                    colors: [Color.orange, Color.red],
                                    startPoint: .leading,
                                    endPoint: .trailing
                                )
                            )
                            .cornerRadius(12)
                    }
                    .disabled(!isValidProposal)
                    .opacity(isValidProposal ? 1.0 : 0.5)
                }
                .padding()
            }
            .background(
                LinearGradient(
                    colors: [Color(red: 0.1, green: 0.05, blue: 0.2), Color(red: 0.2, green: 0.1, blue: 0.3)],
                    startPoint: .topLeading,
                    endPoint: .bottomTrailing
                )
                .ignoresSafeArea()
            )
            .navigationTitle("New Proposal")
            .navigationBarTitleDisplayMode(.inline)
            .toolbar {
                ToolbarItem(placement: .navigationBarLeading) {
                    Button("Cancel") {
                        dismiss()
                    }
                }
            }
            .alert("Validation Error", isPresented: $showValidationError) {
                Button("OK", role: .cancel) {}
            } message: {
                Text(validationMessage)
            }
        }
    }

    private var isValidProposal: Bool {
        !title.isEmpty &&
        !description.isEmpty &&
        description.count >= 50 &&
        (!showBudgetField || !budgetAmount.isEmpty)
    }

    private func submitProposal() {
        guard isValidProposal else {
            validationMessage = "Please fill all required fields. Description must be at least 50 characters."
            showValidationError = true
            return
        }

        let budgetRequest: String? = showBudgetField ? convertToWei(budgetAmount) : nil

        viewModel.createProposal(
            title: title,
            description: description,
            category: selectedCategory,
            budgetRequest: budgetRequest
        )

        dismiss()
    }

    private func convertToWei(_ amount: String) -> String {
        guard let value = Double(amount) else { return "0" }
        let weiValue = value * 1e18
        return String(format: "%.0f", weiValue)
    }
}

// MARK: - Custom Text Field Style

struct CustomTextFieldStyle: TextFieldStyle {
    func _body(configuration: TextField<Self._Label>) -> some View {
        configuration
            .padding()
            .background(Color.white.opacity(0.1))
            .cornerRadius(12)
            .foregroundColor(.white)
    }
}

// MARK: - Info Row

private struct InfoRow: View {
    let icon: String
    let text: String

    var body: some View {
        HStack(spacing: 8) {
            Image(systemName: icon)
                .font(.caption)
                .foregroundColor(.blue)
                .frame(width: 20)

            Text(text)
                .font(.caption)
                .foregroundColor(.gray)
        }
    }
}

#Preview {
    CreateProposalView(viewModel: ConsensusDayViewModel())
}
