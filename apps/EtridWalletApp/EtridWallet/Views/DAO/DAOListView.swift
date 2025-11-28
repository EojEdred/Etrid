//
//  DAOListView.swift
//  EtridWallet
//
//  DAO list and management
//

import SwiftUI

struct DAOListView: View {
    @StateObject private var service = DAOService.shared
    @State private var selectedFilter: FilterOption = .all
    @State private var showCreateDAO = false

    enum FilterOption: String, CaseIterable {
        case all = "All"
        case owner = "Owner"
        case member = "Member"
    }

    var body: some View {
        NavigationView {
            VStack(spacing: 0) {
                // Filter Picker
                Picker("Filter", selection: $selectedFilter) {
                    ForEach(FilterOption.allCases, id: \.self) { option in
                        Text(option.rawValue).tag(option)
                    }
                }
                .pickerStyle(.segmented)
                .padding()

                // DAO List
                List {
                    if filteredDAOs.isEmpty {
                        EmptyDAOView {
                            showCreateDAO = true
                        }
                    } else {
                        ForEach(filteredDAOs) { dao in
                            NavigationLink {
                                DAODetailView(dao: dao)
                            } label: {
                                DAORow(dao: dao)
                            }
                        }
                    }
                }
            }
            .navigationTitle("DAOs")
            .toolbar {
                ToolbarItem(placement: .navigationBarTrailing) {
                    Button(action: { showCreateDAO = true }) {
                        Image(systemName: "plus")
                    }
                }
            }
            .sheet(isPresented: $showCreateDAO) {
                CreateDAOView()
            }
        }
    }

    private var filteredDAOs: [DAO] {
        switch selectedFilter {
        case .all:
            return service.daos
        case .owner:
            return service.getDAOs(filter: .owner)
        case .member:
            return service.daos.filter { $0.userRole == .member || $0.userRole == .voter }
        }
    }
}

struct DAORow: View {
    let dao: DAO

    var body: some View {
        HStack(spacing: 12) {
            // Logo
            Circle()
                .fill(Color.purple.opacity(0.1))
                .frame(width: 50, height: 50)
                .overlay(
                    Text(String(dao.name.prefix(1)))
                        .font(.title3)
                        .foregroundColor(.purple)
                )

            // Info
            VStack(alignment: .leading, spacing: 4) {
                HStack {
                    Text(dao.name)
                        .font(.headline)

                    if dao.userRole == .owner {
                        Text("OWNER")
                            .font(.caption2)
                            .foregroundColor(.white)
                            .padding(.horizontal, 6)
                            .padding(.vertical, 2)
                            .background(Color.purple)
                            .cornerRadius(4)
                    }
                }

                Text(dao.description)
                    .font(.caption)
                    .foregroundColor(.secondary)
                    .lineLimit(1)

                HStack(spacing: 12) {
                    Label("\(dao.memberCount)", systemImage: "person.2")
                    Label("\(dao.activeProposalCount)", systemImage: "doc.text")
                    Label(formatTreasuryValue(dao.treasuryValue), systemImage: "banknote")
                }
                .font(.caption2)
                .foregroundColor(.secondary)
            }

            Spacer()
        }
        .padding(.vertical, 4)
    }

    private func formatTreasuryValue(_ value: String) -> String {
        if let doubleValue = Double(value), doubleValue >= 1000 {
            return String(format: "%.1fK", doubleValue / 1000)
        }
        return value
    }
}

struct EmptyDAOView: View {
    let onCreate: () -> Void

    var body: some View {
        VStack(spacing: 16) {
            Image(systemName: "building.columns")
                .font(.system(size: 48))
                .foregroundColor(.secondary)

            Text("No DAOs Yet")
                .font(.headline)

            Text("Create or join a DAO to get started with decentralized governance")
                .font(.caption)
                .foregroundColor(.secondary)
                .multilineTextAlignment(.center)
                .padding(.horizontal)

            Button("Create DAO") {
                onCreate()
            }
            .buttonStyle(.borderedProminent)
        }
        .padding()
        .frame(maxWidth: .infinity)
    }
}

#Preview {
    DAOListView()
}
