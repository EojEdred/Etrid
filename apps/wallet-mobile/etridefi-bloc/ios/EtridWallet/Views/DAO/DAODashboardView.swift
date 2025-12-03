//
//  DAODashboardView.swift
//  EtridWallet
//
//  Production DAO Governance Dashboard with full consensus functionality
//

import SwiftUI

struct DAODashboardView: View {
    @StateObject private var viewModel = DAODashboardViewModel()
    @Environment(\.dismiss) private var dismiss
    @State private var showCreateProposal = false
    @State private var selectedDAO: DAO?
    @State private var showError = false
    @State private var errorMessage = ""

    var body: some View {
        NavigationView {
            ZStack {
                ScrollView {
                    VStack(spacing: 20) {
                        // Your DAOs Section
                        yourDAOs

                        // Active Proposals Section
                        activeProposals
                    }
                    .padding()
                }
                .refreshable {
                    await viewModel.refresh()
                }

                // Loading Overlay
                if viewModel.isLoading {
                    DAOLoadingView(message: "Loading DAOs...")
                }
            }
            .navigationTitle("DAO Governance")
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
                        Button(action: {
                            selectedDAO = viewModel.daos.first
                            showCreateProposal = true
                        }) {
                            Label("Create Proposal", systemImage: "doc.badge.plus")
                        }
                        .disabled(viewModel.daos.isEmpty)

                        Button(action: {
                            Task { await viewModel.refresh() }
                        }) {
                            Label("Refresh", systemImage: "arrow.clockwise")
                        }
                    } label: {
                        Image(systemName: "ellipsis.circle")
                            .font(.title2)
                            .foregroundColor(.blue)
                    }
                }
            }
            .sheet(item: $selectedDAO) { dao in
                CreateProposalView(dao: dao, viewModel: viewModel)
            }
            .alert("Error", isPresented: $showError) {
                Button("OK") { }
            } message: {
                Text(errorMessage)
            }
            .task {
                await viewModel.loadData()
            }
        }
    }

    // MARK: - Your DAOs
    private var yourDAOs: some View {
        VStack(alignment: .leading, spacing: 12) {
            HStack {
                Text("Your DAOs")
                    .font(.headline)
                    .fontWeight(.bold)

                Spacer()

                if viewModel.isLoadingDAOs {
                    ProgressView()
                        .scaleEffect(0.8)
                }
            }

            if viewModel.daos.isEmpty && !viewModel.isLoadingDAOs {
                DAOEmptyStateView(
                    icon: "building.columns",
                    title: "No DAOs",
                    message: "Join a DAO to participate in governance"
                )
            } else {
                ForEach(viewModel.daos) { dao in
                    DAOCard(dao: dao, activeProposals: viewModel.activeProposalCount(for: dao.id))
                }
            }
        }
    }

    // MARK: - Active Proposals
    private var activeProposals: some View {
        VStack(alignment: .leading, spacing: 12) {
            HStack {
                Text("Active Proposals")
                    .font(.headline)
                    .fontWeight(.bold)

                Spacer()

                if viewModel.isLoadingProposals {
                    ProgressView()
                        .scaleEffect(0.8)
                }
            }

            if viewModel.proposals.isEmpty && !viewModel.isLoadingProposals {
                DAOEmptyStateView(
                    icon: "doc.text",
                    title: "No Active Proposals",
                    message: "Create a proposal to get started"
                )
            } else {
                ForEach(viewModel.proposals) { proposal in
                    ProposalCard(
                        proposal: proposal,
                        userVote: viewModel.userVote(for: proposal.id),
                        onVote: { voteType in
                            Task {
                                await viewModel.vote(proposalId: proposal.id, voteType: voteType)
                            }
                        },
                        onViewDetails: {
                            // Show detailed proposal view
                        }
                    )
                }
            }
        }
    }
}

// MARK: - DAO Card Component
struct DAOCard: View {
    let dao: DAO
    let activeProposals: Int

    var body: some View {
        VStack(alignment: .leading, spacing: 12) {
            // Header
            HStack {
                VStack(alignment: .leading, spacing: 4) {
                    Text(dao.name)
                        .font(.title3)
                        .fontWeight(.bold)

                    Text(dao.description)
                        .font(.caption)
                        .foregroundColor(.secondary)
                        .lineLimit(1)
                }

                Spacer()

                if activeProposals > 0 {
                    Text("\(activeProposals) active")
                        .font(.caption)
                        .fontWeight(.semibold)
                        .padding(.horizontal, 10)
                        .padding(.vertical, 4)
                        .background(Color.blue.opacity(0.2))
                        .foregroundColor(.blue)
                        .cornerRadius(12)
                }
            }

            // Balance
            Text("$\(Int(dao.treasury / 1000))K")
                .font(.system(size: 28, weight: .bold))
                .foregroundColor(.primary)

            // Info
            HStack(spacing: 20) {
                DAOStatView(label: "Members", value: "\(dao.members)")
                DAOStatView(label: "Token", value: dao.governanceToken)
                DAOStatView(label: "Threshold", value: "\(dao.votingThreshold / 1000)K")
            }
        }
        .padding()
        .background(Color(.systemGray6))
        .cornerRadius(16)
        .overlay(
            RoundedRectangle(cornerRadius: 16)
                .stroke(Color(.systemGray4), lineWidth: 1)
        )
    }
}

// MARK: - DAO Stat View
struct DAOStatView: View {
    let label: String
    let value: String

    var body: some View {
        VStack(alignment: .center, spacing: 4) {
            Text(label)
                .font(.caption)
                .foregroundColor(.secondary)

            Text(value)
                .font(.body)
                .fontWeight(.bold)
        }
        .frame(maxWidth: .infinity)
    }
}

// MARK: - Proposal Card Component
struct ProposalCard: View {
    let proposal: Proposal
    let userVote: VoteType?
    let onVote: (VoteType) -> Void
    let onViewDetails: () -> Void

    var body: some View {
        VStack(alignment: .leading, spacing: 12) {
            // Header
            HStack(alignment: .top) {
                VStack(alignment: .leading, spacing: 4) {
                    Text(proposal.title)
                        .font(.body)
                        .fontWeight(.semibold)
                        .lineLimit(2)

                    Text(proposal.description)
                        .font(.caption)
                        .foregroundColor(.secondary)
                        .lineLimit(2)
                }

                Spacer()

                ProposalStatusBadge(status: proposal.status)
            }

            // Vote Progress Bar
            VoteProgressView(
                votesFor: proposal.votesForPercentage,
                votesAgainst: proposal.votesAgainstPercentage
            )

            // Vote Stats
            HStack(spacing: 12) {
                VStack(alignment: .leading, spacing: 2) {
                    HStack(spacing: 4) {
                        Circle()
                            .fill(Color.green)
                            .frame(width: 8, height: 8)
                        Text("For: \(proposal.votesFor)")
                            .font(.caption)
                            .fontWeight(.semibold)
                    }

                    HStack(spacing: 4) {
                        Circle()
                            .fill(Color.red)
                            .frame(width: 8, height: 8)
                        Text("Against: \(proposal.votesAgainst)")
                            .font(.caption)
                            .fontWeight(.semibold)
                    }
                }

                Spacer()

                VStack(alignment: .trailing, spacing: 2) {
                    if proposal.hasReachedQuorum {
                        HStack(spacing: 4) {
                            Image(systemName: "checkmark.circle.fill")
                                .foregroundColor(.green)
                                .font(.caption)
                            Text("Quorum reached")
                                .font(.caption)
                                .fontWeight(.semibold)
                                .foregroundColor(.green)
                        }
                    } else {
                        Text("Quorum: \(proposal.totalVotes)/\(proposal.threshold)")
                            .font(.caption)
                            .foregroundColor(.orange)
                    }

                    Text("Ends in \(proposal.timeRemaining)")
                        .font(.caption)
                        .foregroundColor(.secondary)
                }
            }

            // Voting Buttons or Your Vote Status
            if proposal.status == .active {
                if let userVote = userVote {
                    HStack(spacing: 8) {
                        Image(systemName: "checkmark.circle.fill")
                            .foregroundColor(.green)
                        Text("You voted \(userVote.rawValue)")
                            .font(.subheadline)
                            .fontWeight(.semibold)
                            .foregroundColor(.green)
                    }
                    .frame(maxWidth: .infinity)
                    .padding(.vertical, 8)
                } else {
                    HStack(spacing: 12) {
                        Button(action: { onVote(.for) }) {
                            HStack(spacing: 4) {
                                Image(systemName: "hand.thumbsup.fill")
                                Text("For")
                            }
                            .font(.subheadline)
                            .fontWeight(.semibold)
                            .foregroundColor(.white)
                            .frame(maxWidth: .infinity)
                            .padding(.vertical, 12)
                            .background(Color.green)
                            .cornerRadius(8)
                        }

                        Button(action: { onVote(.against) }) {
                            HStack(spacing: 4) {
                                Image(systemName: "hand.thumbsdown.fill")
                                Text("Against")
                            }
                            .font(.subheadline)
                            .fontWeight(.semibold)
                            .foregroundColor(.white)
                            .frame(maxWidth: .infinity)
                            .padding(.vertical, 12)
                            .background(Color.red)
                            .cornerRadius(8)
                        }

                        Button(action: { onVote(.abstain) }) {
                            Text("Abstain")
                                .font(.subheadline)
                                .fontWeight(.semibold)
                                .foregroundColor(.white)
                                .frame(width: 80)
                                .padding(.vertical, 12)
                                .background(Color.gray)
                                .cornerRadius(8)
                        }
                    }
                }
            }
        }
        .padding()
        .background(Color(.systemGray6))
        .cornerRadius(16)
        .overlay(
            RoundedRectangle(cornerRadius: 16)
                .stroke(proposal.status == .active ? Color.blue : Color(.systemGray4), lineWidth: 1)
        )
        .onTapGesture {
            onViewDetails()
        }
    }
}

// MARK: - Supporting Views
struct ProposalStatusBadge: View {
    let status: ProposalStatus

    var body: some View {
        Text(status.rawValue.uppercased())
            .font(.caption2)
            .fontWeight(.bold)
            .padding(.horizontal, 8)
            .padding(.vertical, 4)
            .background(statusColor.opacity(0.2))
            .foregroundColor(statusColor)
            .cornerRadius(6)
    }

    private var statusColor: Color {
        switch status {
        case .active: return .blue
        case .passed: return .green
        case .rejected: return .red
        case .executed: return .purple
        case .pending: return .orange
        case .cancelled: return .gray
        }
    }
}

struct VoteProgressView: View {
    let votesFor: Double
    let votesAgainst: Double

    var body: some View {
        GeometryReader { geometry in
            ZStack(alignment: .leading) {
                // Background
                Rectangle()
                    .fill(Color(.systemGray4))
                    .frame(height: 8)
                    .cornerRadius(4)

                // For votes (green)
                Rectangle()
                    .fill(Color.green)
                    .frame(width: geometry.size.width * CGFloat(votesFor / 100), height: 8)
                    .cornerRadius(4)

                // Against votes (red) - positioned after the green
                Rectangle()
                    .fill(Color.red)
                    .frame(width: geometry.size.width * CGFloat(votesAgainst / 100), height: 8)
                    .cornerRadius(4)
                    .offset(x: geometry.size.width * CGFloat(votesFor / 100))
            }
        }
        .frame(height: 8)
    }
}

struct DAOEmptyStateView: View {
    let icon: String
    let title: String
    let message: String

    var body: some View {
        VStack(spacing: 12) {
            Image(systemName: icon)
                .font(.system(size: 48))
                .foregroundColor(.secondary)

            Text(title)
                .font(.headline)
                .foregroundColor(.primary)

            Text(message)
                .font(.subheadline)
                .foregroundColor(.secondary)
                .multilineTextAlignment(.center)
        }
        .frame(maxWidth: .infinity)
        .padding(.vertical, 40)
    }
}

struct DAOLoadingView: View {
    let message: String

    var body: some View {
        VStack(spacing: 16) {
            ProgressView()
                .scaleEffect(1.5)

            Text(message)
                .font(.subheadline)
                .foregroundColor(.secondary)
        }
        .padding(32)
        .background(Color(.systemBackground).opacity(0.9))
        .cornerRadius(16)
        .shadow(radius: 20)
    }
}

// MARK: - Create Proposal View
struct CreateProposalView: View {
    let dao: DAO
    @ObservedObject var viewModel: DAODashboardViewModel
    @Environment(\.dismiss) private var dismiss

    @State private var title = ""
    @State private var description = ""
    @State private var category = "General"
    @State private var budget: String = ""
    @State private var recipient = ""
    @State private var isSubmitting = false
    @State private var showError = false
    @State private var errorMessage = ""

    let categories = ["General", "Treasury", "Development", "Network", "Marketing"]

    var body: some View {
        NavigationView {
            Form {
                Section(header: Text("Proposal Details")) {
                    TextField("Title", text: $title)

                    TextEditor(text: $description)
                        .frame(height: 150)
                        .overlay(
                            RoundedRectangle(cornerRadius: 8)
                                .stroke(Color(.systemGray4), lineWidth: 1)
                        )

                    Picker("Category", selection: $category) {
                        ForEach(categories, id: \.self) { category in
                            Text(category).tag(category)
                        }
                    }
                }

                Section(header: Text("Funding (Optional)")) {
                    TextField("Budget Amount", text: $budget)
                        .keyboardType(.decimalPad)

                    if !budget.isEmpty {
                        TextField("Recipient Address", text: $recipient)
                            .autocapitalization(.none)
                            .disableAutocorrection(true)
                    }
                }

                Section(header: Text("Requirements")) {
                    VStack(alignment: .leading, spacing: 8) {
                        RequirementRow(
                            label: "Proposal Deposit",
                            value: "\(dao.proposalDeposit) \(dao.governanceToken)"
                        )
                        RequirementRow(
                            label: "Voting Period",
                            value: "\(dao.votingPeriod / 600) hours"
                        )
                        RequirementRow(
                            label: "Min. Votes Required",
                            value: "\(dao.votingThreshold)"
                        )
                    }
                }

                Section {
                    Text("Creating a proposal requires a deposit of \(dao.proposalDeposit) \(dao.governanceToken). This deposit will be returned if the proposal passes.")
                        .font(.caption)
                        .foregroundColor(.secondary)
                }
            }
            .navigationTitle("Create Proposal")
            .navigationBarTitleDisplayMode(.inline)
            .toolbar {
                ToolbarItem(placement: .navigationBarLeading) {
                    Button("Cancel") {
                        dismiss()
                    }
                    .disabled(isSubmitting)
                }

                ToolbarItem(placement: .navigationBarTrailing) {
                    if isSubmitting {
                        ProgressView()
                    } else {
                        Button("Submit") {
                            Task {
                                await submitProposal()
                            }
                        }
                        .disabled(!isValidProposal)
                    }
                }
            }
            .alert("Error", isPresented: $showError) {
                Button("OK") { }
            } message: {
                Text(errorMessage)
            }
        }
    }

    private var isValidProposal: Bool {
        !title.isEmpty && description.count >= 50
    }

    private func submitProposal() async {
        isSubmitting = true

        let metadata = ProposalMetadata(
            category: category,
            tags: nil,
            budget: Double(budget),
            recipient: recipient.isEmpty ? nil : recipient,
            contractCall: nil
        )

        do {
            try await viewModel.createProposal(
                daoId: dao.id,
                title: title,
                description: description,
                metadata: metadata
            )
            dismiss()
        } catch {
            errorMessage = error.localizedDescription
            showError = true
        }

        isSubmitting = false
    }
}

struct RequirementRow: View {
    let label: String
    let value: String

    var body: some View {
        HStack {
            Text(label)
                .font(.subheadline)
                .foregroundColor(.secondary)

            Spacer()

            Text(value)
                .font(.subheadline)
                .fontWeight(.semibold)
        }
    }
}

// MARK: - View Model
@MainActor
class DAODashboardViewModel: ObservableObject {
    @Published var daos: [DAO] = []
    @Published var proposals: [Proposal] = []
    @Published var userVotes: [String: VoteType] = [:]
    @Published var isLoading = false
    @Published var isLoadingDAOs = false
    @Published var isLoadingProposals = false

    private let governanceService = GovernanceService.shared
    private let userAddress = "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY" // TODO: Get from wallet

    func loadData() async {
        isLoading = true
        await loadDAOs()
        await loadProposals()
        isLoading = false
    }

    func loadDAOs() async {
        isLoadingDAOs = true
        do {
            daos = try await governanceService.getDAOs(for: userAddress)
        } catch {
            print("Error loading DAOs: \(error)")
        }
        isLoadingDAOs = false
    }

    func loadProposals() async {
        isLoadingProposals = true
        var allProposals: [Proposal] = []

        for dao in daos {
            do {
                let daoProposals = try await governanceService.getProposals(for: dao.id, status: .active)
                allProposals.append(contentsOf: daoProposals)
            } catch {
                print("Error loading proposals for DAO \(dao.id): \(error)")
            }
        }

        proposals = allProposals.sorted { $0.blocksRemaining < $1.blocksRemaining }

        // Load user votes for each proposal
        await loadUserVotes()

        isLoadingProposals = false
    }

    private func loadUserVotes() async {
        for proposal in proposals {
            do {
                let vote = try await governanceService.getVote(proposalId: proposal.id, voter: userAddress)
                userVotes[proposal.id] = vote.voteType
            } catch {
                // User hasn't voted on this proposal
                userVotes[proposal.id] = nil
            }
        }
    }

    func vote(proposalId: String, voteType: VoteType) async {
        do {
            guard let proposal = proposals.first(where: { $0.id == proposalId }) else {
                throw GovernanceError.proposalNotFound
            }

            // Get user's voting power for this DAO
            let member = try await governanceService.getMember(daoId: proposal.daoId, address: userAddress)

            let vote = try await governanceService.vote(
                proposalId: proposalId,
                voter: userAddress,
                voteType: voteType,
                votingPower: member.votingPower
            )

            // Update local state
            userVotes[proposalId] = voteType

            // Refresh proposals to get updated vote counts
            await loadProposals()
        } catch {
            print("Error voting: \(error)")
        }
    }

    func createProposal(daoId: String, title: String, description: String, metadata: ProposalMetadata?) async throws {
        let proposal = try await governanceService.createProposal(
            daoId: daoId,
            title: title,
            description: description,
            proposer: userAddress,
            metadata: metadata
        )

        // Reload proposals
        await loadProposals()
    }

    func refresh() async {
        await loadData()
    }

    func activeProposalCount(for daoId: String) -> Int {
        proposals.filter { $0.daoId == daoId && $0.status == .active }.count
    }

    func userVote(for proposalId: String) -> VoteType? {
        userVotes[proposalId]
    }
}

// MARK: - Preview
struct DAODashboardView_Previews: PreviewProvider {
    static var previews: some View {
        DAODashboardView()
    }
}
