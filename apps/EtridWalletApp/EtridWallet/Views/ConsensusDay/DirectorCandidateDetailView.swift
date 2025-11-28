//
//  DirectorCandidateDetailView.swift
//  EtridWallet
//
//  Detailed view of a director candidate with voting interface
//

import SwiftUI

struct DirectorCandidateDetailView: View {
    let candidate: DirectorCandidate
    @ObservedObject var viewModel: ConsensusDayViewModel
    @Environment(\.dismiss) private var dismiss
    @State private var showVoteConfirmation = false

    var body: some View {
        ScrollView {
            VStack(alignment: .leading, spacing: 24) {
                // Candidate Header
                VStack(spacing: 16) {
                    Circle()
                        .fill(
                            LinearGradient(
                                colors: [Color.indigo, Color.purple],
                                startPoint: .topLeading,
                                endPoint: .bottomTrailing
                            )
                        )
                        .frame(width: 100, height: 100)
                        .overlay(
                            Text(String(candidate.displayName.prefix(1)))
                                .font(.system(size: 48, weight: .bold))
                                .foregroundColor(.white)
                        )

                    VStack(spacing: 8) {
                        Text(candidate.displayName)
                            .font(.title)
                            .fontWeight(.bold)
                            .foregroundColor(.white)

                        if candidate.isIncumbent {
                            HStack(spacing: 6) {
                                Image(systemName: "checkmark.seal.fill")
                                    .foregroundColor(.blue)
                                Text("Incumbent Director")
                                    .font(.subheadline)
                                    .foregroundColor(.blue)
                            }
                            .padding(.horizontal, 12)
                            .padding(.vertical, 6)
                            .background(Color.blue.opacity(0.2))
                            .cornerRadius(8)
                        }

                        Text(candidate.address.formatAddress())
                            .font(.caption)
                            .foregroundColor(.gray)
                    }
                }
                .frame(maxWidth: .infinity)
                .padding()
                .background(Color.white.opacity(0.05))
                .cornerRadius(16)

                // Vote Count
                VStack(alignment: .leading, spacing: 12) {
                    Text("Votes Received")
                        .font(.headline)
                        .foregroundColor(.white)

                    HStack {
                        VStack(alignment: .leading, spacing: 4) {
                            Text(candidate.formattedVotes)
                                .font(.system(size: 32, weight: .bold))
                                .foregroundColor(.indigo)

                            Text("Total voting power")
                                .font(.caption)
                                .foregroundColor(.gray)
                        }

                        Spacer()

                        VStack(alignment: .trailing, spacing: 4) {
                            Image(systemName: "person.fill.checkmark")
                                .font(.title)
                                .foregroundColor(.indigo)
                        }
                    }
                    .padding()
                    .background(Color.indigo.opacity(0.1))
                    .cornerRadius(12)
                }

                // Biography
                if let bio = candidate.bio {
                    VStack(alignment: .leading, spacing: 12) {
                        Text("Biography")
                            .font(.headline)
                            .foregroundColor(.white)

                        Text(bio)
                            .font(.body)
                            .foregroundColor(.gray)
                            .lineSpacing(4)
                    }
                    .padding()
                    .background(Color.white.opacity(0.05))
                    .cornerRadius(12)
                }

                // Nomination Date
                VStack(alignment: .leading, spacing: 8) {
                    Text("Nomination Details")
                        .font(.headline)
                        .foregroundColor(.white)

                    HStack {
                        Image(systemName: "calendar")
                            .foregroundColor(.blue)

                        Text("Nominated on")
                            .font(.subheadline)
                            .foregroundColor(.gray)

                        Spacer()

                        Text(candidate.nominatedAt.formatted(date: .long, time: .omitted))
                            .font(.subheadline)
                            .foregroundColor(.white)
                    }
                }
                .padding()
                .background(Color.white.opacity(0.05))
                .cornerRadius(12)

                // Info Box
                VStack(alignment: .leading, spacing: 12) {
                    HStack(spacing: 8) {
                        Image(systemName: "info.circle.fill")
                            .foregroundColor(.blue)

                        Text("About Director Elections")
                            .font(.subheadline)
                            .fontWeight(.semibold)
                            .foregroundColor(.white)
                    }

                    Text("You can vote for up to 7 directors. Directors serve on the Etrid Foundation Board and help guide the protocol's development and governance.")
                        .font(.caption)
                        .foregroundColor(.gray)
                        .lineSpacing(2)
                }
                .padding()
                .background(Color.blue.opacity(0.1))
                .cornerRadius(12)

                // Vote Button
                if viewModel.consensusState.currentPhase == .voting {
                    Button {
                        showVoteConfirmation = true
                    } label: {
                        HStack {
                            Image(systemName: "hand.thumbsup.fill")
                                .font(.title3)

                            Text("Vote for This Candidate")
                                .font(.headline)
                                .fontWeight(.semibold)
                        }
                        .foregroundColor(.white)
                        .frame(maxWidth: .infinity)
                        .padding()
                        .background(
                            LinearGradient(
                                colors: [Color.indigo, Color.purple],
                                startPoint: .leading,
                                endPoint: .trailing
                            )
                        )
                        .cornerRadius(12)
                    }
                    .padding(.top, 8)
                }
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
        .navigationTitle("Candidate")
        .navigationBarTitleDisplayMode(.inline)
        .alert("Confirm Vote", isPresented: $showVoteConfirmation) {
            Button("Cancel", role: .cancel) { }
            Button("Confirm") {
                viewModel.voteForDirector(candidate: candidate)
                dismiss()
            }
        } message: {
            Text("Cast vote for \(candidate.displayName) with \(viewModel.votingPower.formattedPower)?")
        }
    }
}

#Preview {
    NavigationView {
        DirectorCandidateDetailView(
            candidate: DirectorCandidate.mockCandidates[0],
            viewModel: ConsensusDayViewModel()
        )
    }
}
