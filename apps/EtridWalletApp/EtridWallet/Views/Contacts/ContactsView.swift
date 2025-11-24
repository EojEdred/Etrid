//
//  ContactsView.swift
//  EtridWallet
//
//  Main contacts management view
//

import SwiftUI

struct ContactsView: View {
    @StateObject private var viewModel = ContactsViewModel()
    @State private var searchText = ""
    @State private var showingAddContact = false
    @State private var selectedContact: Contact?
    @State private var showingContactDetail = false
    @State private var filterOption: ContactFilter = .all

    enum ContactFilter: String, CaseIterable {
        case all = "All"
        case favorites = "Favorites"
        case recent = "Recent"
    }

    var filteredContacts: [Contact] {
        var contacts = viewModel.contacts

        // Apply filter
        switch filterOption {
        case .all:
            break
        case .favorites:
            contacts = contacts.filter { $0.isFavorite }
        case .recent:
            contacts = contacts.sorted { ($0.lastInteraction ?? .distantPast) > ($1.lastInteraction ?? .distantPast) }
        }

        // Apply search
        if !searchText.isEmpty {
            contacts = contacts.filter {
                $0.name.localizedCaseInsensitiveContains(searchText) ||
                $0.address.localizedCaseInsensitiveContains(searchText) ||
                $0.tags.contains { $0.localizedCaseInsensitiveContains(searchText) }
            }
        }

        return contacts
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
                    // Search Bar
                    HStack {
                        Image(systemName: "magnifyingglass")
                            .foregroundColor(.gray)
                        TextField("Search contacts...", text: $searchText)
                            .foregroundColor(.white)
                    }
                    .padding()
                    .background(Color.white.opacity(0.1))
                    .cornerRadius(12)
                    .padding()

                    // Filter Tabs
                    Picker("Filter", selection: $filterOption) {
                        ForEach(ContactFilter.allCases, id: \.self) { option in
                            Text(option.rawValue).tag(option)
                        }
                    }
                    .pickerStyle(.segmented)
                    .padding(.horizontal)

                    // Recent Contacts (if showing all)
                    if filterOption == .all && searchText.isEmpty {
                        recentContactsSection
                    }

                    // Contacts List
                    if filteredContacts.isEmpty {
                        emptyStateView
                    } else {
                        contactsList
                    }
                }
            }
            .navigationTitle("Contacts")
            .navigationBarTitleDisplayMode(.large)
            .toolbar {
                ToolbarItem(placement: .navigationBarTrailing) {
                    Button {
                        showingAddContact = true
                    } label: {
                        Image(systemName: "plus")
                            .foregroundColor(.white)
                    }
                }
            }
            .sheet(isPresented: $showingAddContact) {
                AddContactView(viewModel: viewModel)
            }
            .sheet(isPresented: $showingContactDetail) {
                if let contact = selectedContact {
                    ContactDetailView(contact: contact, viewModel: viewModel)
                }
            }
        }
    }

    private var recentContactsSection: some View {
        VStack(alignment: .leading, spacing: 12) {
            Text("Recent")
                .font(.headline)
                .foregroundColor(.white)
                .padding(.horizontal)

            ScrollView(.horizontal, showsIndicators: false) {
                HStack(spacing: 12) {
                    ForEach(viewModel.recentContacts.prefix(5)) { recent in
                        RecentContactCard(recent: recent) {
                            if let contact = viewModel.contacts.first(where: { $0.id == recent.contactId }) {
                                selectedContact = contact
                                showingContactDetail = true
                            }
                        }
                    }
                }
                .padding(.horizontal)
            }
        }
        .padding(.vertical)
    }

    private var contactsList: some View {
        List {
            ForEach(filteredContacts) { contact in
                ContactRowView(contact: contact)
                    .listRowBackground(Color.clear)
                    .listRowInsets(EdgeInsets(top: 4, leading: 16, bottom: 4, trailing: 16))
                    .onTapGesture {
                        selectedContact = contact
                        showingContactDetail = true
                    }
                    .swipeActions(edge: .trailing, allowsFullSwipe: false) {
                        Button(role: .destructive) {
                            viewModel.deleteContact(contact)
                        } label: {
                            Label("Delete", systemImage: "trash")
                        }

                        Button {
                            viewModel.toggleFavorite(contact)
                        } label: {
                            Label(contact.isFavorite ? "Unfavorite" : "Favorite",
                                  systemImage: contact.isFavorite ? "star.slash" : "star")
                        }
                        .tint(.yellow)
                    }
            }
        }
        .listStyle(.plain)
        .scrollContentBackground(.hidden)
    }

    private var emptyStateView: some View {
        VStack(spacing: 20) {
            Image(systemName: "person.2.slash")
                .font(.system(size: 60))
                .foregroundColor(.gray)

            Text("No contacts found")
                .font(.title2)
                .foregroundColor(.white)

            Text(searchText.isEmpty ? "Add your first contact to get started" : "Try a different search")
                .font(.subheadline)
                .foregroundColor(.gray)

            if searchText.isEmpty {
                Button {
                    showingAddContact = true
                } label: {
                    Text("Add Contact")
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
        }
        .frame(maxWidth: .infinity, maxHeight: .infinity)
        .padding()
    }
}

// MARK: - Contact Row View
struct ContactRowView: View {
    let contact: Contact

    var body: some View {
        HStack(spacing: 12) {
            // Avatar
            ZStack {
                Circle()
                    .fill(
                        LinearGradient(
                            colors: [Color.blue.opacity(0.6), Color.purple.opacity(0.6)],
                            startPoint: .topLeading,
                            endPoint: .bottomTrailing
                        )
                    )
                    .frame(width: 50, height: 50)

                if let avatar = contact.avatar, avatar.count == 1 {
                    Text(avatar)
                        .font(.title2)
                } else {
                    Text(contact.initials)
                        .font(.headline)
                        .foregroundColor(.white)
                }
            }

            VStack(alignment: .leading, spacing: 4) {
                HStack {
                    Text(contact.name)
                        .font(.headline)
                        .foregroundColor(.white)

                    if contact.isFavorite {
                        Image(systemName: "star.fill")
                            .font(.caption)
                            .foregroundColor(.yellow)
                    }
                }

                Text(contact.shortAddress)
                    .font(.caption)
                    .foregroundColor(.gray)

                if !contact.tags.isEmpty {
                    HStack(spacing: 4) {
                        ForEach(contact.tags.prefix(2), id: \.self) { tag in
                            Text(tag)
                                .font(.caption2)
                                .foregroundColor(.blue)
                                .padding(.horizontal, 6)
                                .padding(.vertical, 2)
                                .background(Color.blue.opacity(0.2))
                                .cornerRadius(4)
                        }
                    }
                }
            }

            Spacer()

            Image(systemName: "chevron.right")
                .font(.caption)
                .foregroundColor(.gray)
        }
        .padding()
        .background(Color.white.opacity(0.05))
        .cornerRadius(12)
    }
}

// MARK: - Recent Contact Card
struct RecentContactCard: View {
    let recent: RecentContact
    let action: () -> Void

    var body: some View {
        Button(action: action) {
            VStack(spacing: 8) {
                ZStack {
                    Circle()
                        .fill(
                            LinearGradient(
                                colors: [Color.green.opacity(0.6), Color.blue.opacity(0.6)],
                                startPoint: .topLeading,
                                endPoint: .bottomTrailing
                            )
                        )
                        .frame(width: 50, height: 50)

                    Text(recent.name?.prefix(1).uppercased() ?? "?")
                        .font(.title3)
                        .foregroundColor(.white)
                }

                Text(recent.displayName)
                    .font(.caption)
                    .foregroundColor(.white)
                    .lineLimit(1)

                Text("\(recent.transactionCount) tx")
                    .font(.caption2)
                    .foregroundColor(.gray)
            }
            .frame(width: 80)
            .padding(.vertical, 8)
            .background(Color.white.opacity(0.05))
            .cornerRadius(12)
        }
    }
}

// MARK: - View Model
class ContactsViewModel: ObservableObject {
    @Published var contacts: [Contact] = []
    @Published var recentContacts: [RecentContact] = []

    init() {
        loadContacts()
        loadRecentContacts()
    }

    private func loadContacts() {
        // Load from storage - placeholder for now
        contacts = [
            Contact(
                name: "Alice Johnson",
                address: "0x742d35Cc6634C0532925a3b844Bc9e7595f0bEb",
                avatar: "👩",
                isFavorite: true,
                tags: ["Friend", "ETR"],
                lastInteraction: Date().addingTimeInterval(-86400)
            ),
            Contact(
                name: "Bob Smith",
                address: "0x1234567890123456789012345678901234567890",
                avatar: "👨",
                tags: ["Work"],
                lastInteraction: Date().addingTimeInterval(-172800)
            )
        ]
    }

    private func loadRecentContacts() {
        recentContacts = [
            RecentContact(
                contactId: contacts.first?.id,
                address: "0x742d35Cc6634C0532925a3b844Bc9e7595f0bEb",
                name: "Alice Johnson",
                transactionCount: 5
            )
        ]
    }

    func addContact(_ contact: Contact) {
        contacts.append(contact)
        // Save to storage
    }

    func deleteContact(_ contact: Contact) {
        contacts.removeAll { $0.id == contact.id }
        // Update storage
    }

    func toggleFavorite(_ contact: Contact) {
        if let index = contacts.firstIndex(where: { $0.id == contact.id }) {
            contacts[index].isFavorite.toggle()
            // Update storage
        }
    }

    func updateContact(_ contact: Contact) {
        if let index = contacts.firstIndex(where: { $0.id == contact.id }) {
            contacts[index] = contact
            // Update storage
        }
    }
}

struct ContactsView_Previews: PreviewProvider {
    static var previews: some View {
        ContactsView()
    }
}
