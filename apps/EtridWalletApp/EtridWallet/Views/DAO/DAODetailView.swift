//
//  DAODetailView.swift
//  EtridWallet
//
//  Detailed DAO view with proposals and voting
//

import SwiftUI

struct DAODetailView: View {
    let dao: DAO
    @StateObject private var service = DAOService.shared
    @State private var selectedTab = 0
    @State private var showCreateProposal = false

    var body: some View {
        VStack(spacing: 0) {
            // Header
            daoHeader

            // Tabs
            Picker("View", selection: $selectedTab) {
                Text("Proposals").tag(0)
                Text("Treasury").tag(1)
                Text("Members").tag(2)
            }
            .pickerStyle(.segmented)
            .padding()

            // Content
            TabView(selection: $selectedTab) {
                ProposalsTab(dao: dao).tag(0)
                TreasuryTab(dao: dao).tag(1)
                MembersTab(dao: dao).tag(2)
            }
            .tabViewStyle(.page(indexDisplayMode: .never))
        }
        .navigationTitle(dao.name)
        .navigationBarTitleDisplayMode(.inline)
        .toolbar {
            ToolbarItem(placement: .navigationBarTrailing) {
                if dao.userRole == .owner || dao.userRole == .admin {
                    Button(action: { showCreateProposal = true }) {
                        Image(systemName: "plus")
                    }
                }
            }
        }
        .sheet(isPresented: $showCreateProposal) {
            DAOCreateProposalView(dao: dao)
        }
    }

    private var daoHeader: some View {
        VStack(spacing: 12) {
            // Logo
            Circle()
                .fill(Color.purple.opacity(0.1))
                .frame(width: 60, height: 60)
                .overlay(
                    Text(String(dao.name.prefix(1)))
                        .font(.title)
                        .foregroundColor(.purple)
                )

            // Stats
            HStack(spacing: 24) {
                StatItem(value: "\(dao.memberCount)", label: "Members")
                StatItem(value: "\(dao.activeProposalCount)", label: "Active")
                StatItem(value: "$\(dao.treasuryValue)", label: "Treasury")
            }
        }
        .padding()
        .background(Color(.systemGray6))
    }
}

private struct StatItem: View {
    let value: String
    let label: String

    var body: some View {
        VStack(spacing: 4) {
            Text(value)
                .font(.headline)
            Text(label)
                .font(.caption)
                .foregroundColor(.secondary)
        }
    }
}

private struct ProposalsTab: View {
    let dao: DAO
    @StateObject private var service = DAOService.shared
    @State private var filter: DAOProposal.ProposalStatus? = nil

    var body: some View {
        List {
            if proposals.isEmpty {
                ContentUnavailableView(
                    "No Proposals",
                    systemImage: "doc.text",
                    description: Text("Create a proposal to start voting")
                )
            } else {
                ForEach(proposals) { proposal in
                    NavigationLink {
                        ProposalDetailView(proposal: proposal, dao: dao)
                    } label: {
                        ProposalRowView(proposal: proposal)
                    }
                }
            }
        }
    }

    private var proposals: [DAOProposal] {
        service.getProposals(daoId: dao.id, filter: filter)
    }
}

private struct ProposalRowView: View {
    let proposal: DAOProposal

    var body: some View {
        VStack(alignment: .leading, spacing: 8) {
            // Title and Status
            HStack {
                Text(proposal.title)
                    .font(.headline)
                    .lineLimit(1)

                Spacer()

                StatusBadge(status: proposal.status)
            }

            // Description
            Text(proposal.description)
                .font(.caption)
                .foregroundColor(.secondary)
                .lineLimit(2)

            // Voting Progress
            if proposal.status == .active {
                VStack(alignment: .leading, spacing: 4) {
                    HStack {
                        Text("For: \(Int(proposal.forPercentage))%")
                            .font(.caption2)
                        Spacer()
                        Text("Against: \(Int(proposal.againstPercentage))%")
                            .font(.caption2)
                    }
                    .foregroundColor(.secondary)

                    GeometryReader { geometry in
                        ZStack(alignment: .leading) {
                            Rectangle()
                                .fill(Color.red.opacity(0.3))
                                .frame(width: geometry.size.width)

                            Rectangle()
                                .fill(Color.green)
                                .frame(width: geometry.size.width * proposal.forPercentage / 100)
                        }
                    }
                    .frame(height: 4)
                    .cornerRadius(2)
                }
            }

            // Time Remaining
            HStack {
                Image(systemName: "clock")
                    .font(.caption2)
                Text(proposal.timeRemaining)
                    .font(.caption2)

                Spacer()

                if let vote = proposal.userVote {
                    HStack(spacing: 4) {
                        Image(systemName: "checkmark.circle.fill")
                            .foregroundColor(.green)
                        Text("Voted \(vote.rawValue)")
                    }
                    .font(.caption2)
                    .foregroundColor(.secondary)
                }
            }
        }
        .padding(.vertical, 4)
    }
}

private struct StatusBadge: View {
    let status: DAOProposal.ProposalStatus

    var body: some View {
        Text(status.rawValue)
            .font(.caption2)
            .padding(.horizontal, 8)
            .padding(.vertical, 4)
            .background(statusColor.opacity(0.2))
            .foregroundColor(statusColor)
            .cornerRadius(8)
    }

    private var statusColor: Color {
        switch status {
        case .active: return .blue
        case .succeeded, .executed: return .green
        case .defeated, .cancelled: return .red
        case .pending: return .orange
        case .queued: return .purple
        case .expired: return .gray
        }
    }
}

private struct TreasuryTab: View {
    let dao: DAO

    var body: some View {
        List {
            Section("Overview") {
                HStack {
                    Text("Total Value")
                    Spacer()
                    Text("$\(dao.treasuryValue)")
                        .bold()
                }
            }

            Section("Assets") {
                Text("No assets yet")
                    .foregroundColor(.secondary)
                    .font(.caption)
            }

            Section("Recent Transactions") {
                Text("No transactions yet")
                    .foregroundColor(.secondary)
                    .font(.caption)
            }
        }
    }
}

private struct MembersTab: View {
    let dao: DAO
    @StateObject private var service = DAOService.shared
    @State private var members: [DAOMember] = []

    var body: some View {
        List {
            if members.isEmpty {
                ContentUnavailableView(
                    "No Members",
                    systemImage: "person.2",
                    description: Text("Loading members...")
                )
            } else {
                ForEach(members) { member in
                    MemberRow(member: member)
                }
            }
        }
        .task {
            do {
                members = try await service.getMembers(daoId: dao.id)
            } catch {
                print("Error loading members: \(error)")
            }
        }
    }
}

private struct MemberRow: View {
    let member: DAOMember

    var body: some View {
        HStack {
            VStack(alignment: .leading, spacing: 4) {
                Text(member.username ?? member.address)
                    .font(.headline)

                Text(member.role.rawValue)
                    .font(.caption)
                    .foregroundColor(.secondary)

                HStack(spacing: 12) {
                    Label("\(member.proposalsCreated)", systemImage: "doc.text")
                    Label("\(member.votesCast)", systemImage: "hand.raised")
                }
                .font(.caption2)
                .foregroundColor(.secondary)
            }

            Spacer()

            Text(member.votingPowerFormatted)
                .font(.caption)
                .foregroundColor(.secondary)
        }
    }
}

private struct ProposalDetailView: View {
    let proposal: DAOProposal
    let dao: DAO
    @StateObject private var service = DAOService.shared
    @State private var selectedVote: DAOVoteType = .for
    @State private var showVoteSheet = false
    @State private var isVoting = false

    var body: some View {
        ScrollView {
            VStack(alignment: .leading, spacing: 20) {
                // Status
                StatusBadge(status: proposal.status)

                // Title
                Text(proposal.title)
                    .font(.title2)
                    .bold()

                // Proposer
                HStack {
                    Image(systemName: "person.circle")
                    Text("Proposed by \(proposal.proposerName ?? "Unknown")")
                        .font(.caption)
                        .foregroundColor(.secondary)
                }

                // Description
                Text(proposal.description)
                    .font(.body)

                // Voting Stats
                VStack(alignment: .leading, spacing: 12) {
                    Text("Voting Results")
                        .font(.headline)

                    VoteBar(label: "For", percentage: proposal.forPercentage, color: .green)
                    VoteBar(label: "Against", percentage: proposal.againstPercentage, color: .red)
                    VoteBar(label: "Abstain", percentage: proposal.abstainPercentage, color: .gray)
                }
                .padding()
                .background(Color(.systemGray6))
                .cornerRadius(12)

                // Actions
                if proposal.status == .active && proposal.userVote == nil {
                    Button("Cast Your Vote") {
                        showVoteSheet = true
                    }
                    .buttonStyle(.borderedProminent)
                    .frame(maxWidth: .infinity)
                } else if let vote = proposal.userVote {
                    HStack {
                        Image(systemName: "checkmark.circle.fill")
                            .foregroundColor(.green)
                        Text("You voted \(vote.rawValue)")
                    }
                    .frame(maxWidth: .infinity)
                    .padding()
                    .background(Color.green.opacity(0.1))
                    .cornerRadius(8)
                }
            }
            .padding()
        }
        .navigationTitle("Proposal")
        .navigationBarTitleDisplayMode(.inline)
        .sheet(isPresented: $showVoteSheet) {
            VoteSheet(
                proposal: proposal,
                selectedVote: $selectedVote,
                onVote: castVote
            )
        }
    }

    private func castVote() {
        isVoting = true
        Task {
            do {
                try await service.vote(
                    proposalId: proposal.id,
                    daoId: dao.id,
                    voteType: selectedVote,
                    votingPower: "1000"
                )
                await MainActor.run {
                    showVoteSheet = false
                    isVoting = false
                }
            } catch {
                print("Vote error: \(error)")
                isVoting = false
            }
        }
    }
}

private struct VoteBar: View {
    let label: String
    let percentage: Double
    let color: Color

    var body: some View {
        VStack(alignment: .leading, spacing: 4) {
            HStack {
                Text(label)
                    .font(.subheadline)
                Spacer()
                Text("\(Int(percentage))%")
                    .font(.subheadline)
                    .bold()
            }

            GeometryReader { geometry in
                ZStack(alignment: .leading) {
                    Rectangle()
                        .fill(Color(.systemGray5))
                        .frame(width: geometry.size.width)

                    Rectangle()
                        .fill(color)
                        .frame(width: geometry.size.width * percentage / 100)
                }
            }
            .frame(height: 8)
            .cornerRadius(4)
        }
    }
}

private struct VoteSheet: View {
    let proposal: DAOProposal
    @Binding var selectedVote: DAOVoteType
    let onVote: () -> Void
    @Environment(\.dismiss) private var dismiss

    var body: some View {
        NavigationView {
            VStack(spacing: 20) {
                Text("Cast Your Vote")
                    .font(.title2)
                    .bold()

                Picker("Vote", selection: $selectedVote) {
                    ForEach(DAOVoteType.allCases, id: \.self) { type in
                        Text(type.rawValue).tag(type)
                    }
                }
                .pickerStyle(.segmented)

                Text(proposal.title)
                    .font(.headline)
                    .multilineTextAlignment(.center)

                Spacer()

                Button("Submit Vote") {
                    onVote()
                }
                .buttonStyle(.borderedProminent)
                .frame(maxWidth: .infinity)
            }
            .padding()
            .navigationBarTitleDisplayMode(.inline)
            .toolbar {
                ToolbarItem(placement: .cancellationAction) {
                    Button("Cancel") {
                        dismiss()
                    }
                }
            }
        }
    }
}

private struct DAOCreateProposalView: View {
    let dao: DAO
    @StateObject private var service = DAOService.shared
    @Environment(\.dismiss) private var dismiss

    @State private var title = ""
    @State private var description = ""
    @State private var votingPeriodDays = 3

    var body: some View {
        NavigationView {
            Form {
                Section("Proposal Details") {
                    TextField("Title", text: $title)
                    TextField("Description", text: $description, axis: .vertical)
                        .lineLimit(5...10)
                }

                Section("Voting Period") {
                    Picker("Duration", selection: $votingPeriodDays) {
                        Text("1 Day").tag(1)
                        Text("3 Days").tag(3)
                        Text("7 Days").tag(7)
                    }
                }
            }
            .navigationTitle("Create Proposal")
            .navigationBarTitleDisplayMode(.inline)
            .toolbar {
                ToolbarItem(placement: .cancellationAction) {
                    Button("Cancel") {
                        dismiss()
                    }
                }
                ToolbarItem(placement: .confirmationAction) {
                    Button("Create") {
                        createProposal()
                    }
                    .disabled(title.isEmpty || description.isEmpty)
                }
            }
        }
    }

    private func createProposal() {
        let params = CreateProposalParams(
            daoId: dao.id,
            title: title,
            description: description,
            actions: [],
            votingPeriodDays: votingPeriodDays
        )

        Task {
            do {
                _ = try await service.createProposal(params)
                await MainActor.run {
                    dismiss()
                }
            } catch {
                print("Create proposal error: \(error)")
            }
        }
    }
}
