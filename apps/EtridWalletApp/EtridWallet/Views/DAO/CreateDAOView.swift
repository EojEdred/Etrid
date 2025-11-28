//
//  CreateDAOView.swift
//  EtridWallet
//
//  DAO creation wizard
//

import SwiftUI

struct CreateDAOView: View {
    @StateObject private var service = DAOService.shared
    @Environment(\.dismiss) private var dismiss
    @State private var currentStep = 0

    // Basic Info
    @State private var name = ""
    @State private var description = ""
    @State private var logoURL = ""

    // Governance
    @State private var votingPeriodDays = 3
    @State private var quorumPercentage = 50.0
    @State private var proposalThreshold = "100"
    @State private var membershipType = DAOGovernance.MembershipType.open
    @State private var tokenAddress = ""
    @State private var nftAddress = ""

    // Members
    @State private var initialMembers: [String] = []
    @State private var newMemberAddress = ""

    @State private var isCreating = false
    @State private var errorMessage: String?

    var body: some View {
        NavigationView {
            VStack {
                // Progress
                ProgressView(value: Double(currentStep), total: 2)
                    .padding()

                TabView(selection: $currentStep) {
                    basicInfoStep.tag(0)
                    governanceStep.tag(1)
                    membersStep.tag(2)
                }
                .tabViewStyle(.page(indexDisplayMode: .never))

                // Navigation
                HStack {
                    if currentStep > 0 {
                        Button("Back") {
                            withAnimation {
                                currentStep -= 1
                            }
                        }
                        .buttonStyle(.bordered)
                    }

                    Spacer()

                    if currentStep < 2 {
                        Button("Next") {
                            withAnimation {
                                currentStep += 1
                            }
                        }
                        .buttonStyle(.borderedProminent)
                        .disabled(!canProceed)
                    } else {
                        Button("Create DAO") {
                            createDAO()
                        }
                        .buttonStyle(.borderedProminent)
                        .disabled(!isValid || isCreating)
                    }
                }
                .padding()
            }
            .navigationTitle("Create DAO")
            .navigationBarTitleDisplayMode(.inline)
            .toolbar {
                ToolbarItem(placement: .cancellationAction) {
                    Button("Cancel") {
                        dismiss()
                    }
                }
            }
            .overlay {
                if isCreating {
                    ProgressView("Creating DAO...")
                        .padding()
                        .background(Color(.systemBackground))
                        .cornerRadius(12)
                        .shadow(radius: 10)
                }
            }
        }
    }

    private var basicInfoStep: some View {
        Form {
            Section("Basic Information") {
                TextField("DAO Name", text: $name)
                TextField("Description", text: $description, axis: .vertical)
                    .lineLimit(3...6)
                TextField("Logo URL (optional)", text: $logoURL)
                    .autocapitalization(.none)
                    .keyboardType(.URL)
            }

            Section {
                Text("Choose a memorable name and description for your DAO")
                    .font(.caption)
                    .foregroundColor(.secondary)
            }
        }
    }

    private var governanceStep: some View {
        Form {
            Section("Voting Settings") {
                Picker("Voting Period", selection: $votingPeriodDays) {
                    Text("1 Day").tag(1)
                    Text("3 Days").tag(3)
                    Text("7 Days").tag(7)
                    Text("14 Days").tag(14)
                }

                VStack(alignment: .leading) {
                    Text("Quorum: \(Int(quorumPercentage))%")
                        .font(.subheadline)
                    Slider(value: $quorumPercentage, in: 1...100, step: 1)
                    Text("Minimum percentage of votes required for a proposal to pass")
                        .font(.caption)
                        .foregroundColor(.secondary)
                }

                TextField("Proposal Threshold", text: $proposalThreshold)
                    .keyboardType(.numberPad)
            }

            Section("Membership") {
                Picker("Membership Type", selection: $membershipType) {
                    ForEach([DAOGovernance.MembershipType.open, .tokenGated, .nftGated, .inviteOnly], id: \.self) { type in
                        Text(type.rawValue).tag(type)
                    }
                }

                if membershipType == .tokenGated {
                    TextField("Token Address", text: $tokenAddress)
                        .autocapitalization(.none)
                }

                if membershipType == .nftGated {
                    TextField("NFT Address", text: $nftAddress)
                        .autocapitalization(.none)
                }
            }
        }
    }

    private var membersStep: some View {
        Form {
            Section("Initial Members (Optional)") {
                ForEach(Array(initialMembers.enumerated()), id: \.offset) { index, member in
                    HStack {
                        Text(member)
                            .font(.caption)
                            .lineLimit(1)
                        Spacer()
                        Button(action: { initialMembers.remove(at: index) }) {
                            Image(systemName: "minus.circle.fill")
                                .foregroundColor(.red)
                        }
                    }
                }

                HStack {
                    TextField("Member Address", text: $newMemberAddress)
                        .autocapitalization(.none)
                    Button(action: addMember) {
                        Image(systemName: "plus.circle.fill")
                            .foregroundColor(.green)
                    }
                    .disabled(newMemberAddress.isEmpty)
                }
            }

            if let error = errorMessage {
                Section {
                    Text(error)
                        .foregroundColor(.red)
                        .font(.caption)
                }
            }
        }
    }

    private var canProceed: Bool {
        switch currentStep {
        case 0:
            return !name.isEmpty && !description.isEmpty
        case 1:
            if membershipType == .tokenGated {
                return !tokenAddress.isEmpty
            } else if membershipType == .nftGated {
                return !nftAddress.isEmpty
            }
            return true
        default:
            return true
        }
    }

    private var isValid: Bool {
        !name.isEmpty && !description.isEmpty
    }

    private func addMember() {
        guard !newMemberAddress.isEmpty else { return }
        initialMembers.append(newMemberAddress)
        newMemberAddress = ""
    }

    private func createDAO() {
        isCreating = true
        errorMessage = nil

        let params = CreateDAOParams(
            name: name,
            description: description,
            logoUrl: logoURL.isEmpty ? nil : logoURL,
            votingPeriod: votingPeriodDays * 86400,
            quorumPercentage: quorumPercentage,
            proposalThreshold: proposalThreshold,
            membershipType: membershipType,
            tokenAddress: tokenAddress.isEmpty ? nil : tokenAddress,
            nftAddress: nftAddress.isEmpty ? nil : nftAddress,
            initialMembers: initialMembers.isEmpty ? nil : initialMembers,
            initialTreasuryAmount: nil
        )

        Task {
            do {
                _ = try await service.createDAO(params)
                await MainActor.run {
                    dismiss()
                }
            } catch {
                await MainActor.run {
                    errorMessage = error.localizedDescription
                    isCreating = false
                }
            }
        }
    }
}

#Preview {
    CreateDAOView()
}
