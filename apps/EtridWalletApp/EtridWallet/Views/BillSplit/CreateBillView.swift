//
//  CreateBillView.swift
//  EtridWallet
//
//  View for creating a new bill split
//

import SwiftUI

struct CreateBillView: View {
    @Environment(\.dismiss) var dismiss
    @ObservedObject var viewModel: BillSplitViewModel
    @StateObject private var contactsViewModel = ContactsViewModel()

    @State private var title = ""
    @State private var description = ""
    @State private var totalAmount = ""
    @State private var selectedToken = "ETR"
    @State private var splitType: SplitType = .even
    @State private var selectedContacts: [Contact] = []
    @State private var participants: [Participant] = []
    @State private var selectedCategory: BillCategory?
    @State private var showingContactPicker = false
    @State private var showingError = false
    @State private var errorMessage = ""
    @State private var expiryDays = 7

    private let tokens = ["ETR", "USDT", "USDC", "DAI"]

    var body: some View {
        NavigationView {
            ZStack {
                LinearGradient(
                    colors: [Color(red: 0.1, green: 0.05, blue: 0.2), Color(red: 0.2, green: 0.1, blue: 0.3)],
                    startPoint: .topLeading,
                    endPoint: .bottomTrailing
                )
                .ignoresSafeArea()

                ScrollView {
                    VStack(spacing: 24) {
                        // Basic Information
                        basicInfoSection

                        // Amount Section
                        amountSection

                        // Split Type
                        splitTypeSection

                        // Participants
                        participantsSection

                        // Category
                        categorySection

                        // Create Button
                        Button {
                            createBill()
                        } label: {
                            Text("Create Bill Split")
                                .fontWeight(.semibold)
                                .foregroundColor(.white)
                                .frame(maxWidth: .infinity)
                                .padding()
                                .background(
                                    LinearGradient(
                                        colors: [Color.blue, Color.purple],
                                        startPoint: .leading,
                                        endPoint: .trailing
                                    )
                                )
                                .cornerRadius(12)
                        }
                        .disabled(!isValidForm)
                        .opacity(isValidForm ? 1.0 : 0.5)
                    }
                    .padding()
                }
            }
            .navigationTitle("Create Bill Split")
            .navigationBarTitleDisplayMode(.inline)
            .toolbar {
                ToolbarItem(placement: .navigationBarLeading) {
                    Button("Cancel") {
                        dismiss()
                    }
                    .foregroundColor(.white)
                }
            }
            .alert("Error", isPresented: $showingError) {
                Button("OK", role: .cancel) { }
            } message: {
                Text(errorMessage)
            }
            .sheet(isPresented: $showingContactPicker) {
                ContactPickerView(
                    contacts: contactsViewModel.contacts,
                    selectedContacts: $selectedContacts
                ) {
                    updateParticipants()
                }
            }
        }
    }

    private var basicInfoSection: some View {
        VStack(spacing: 16) {
            FormField(title: "Title", text: $title, placeholder: "e.g., Dinner at Joe's")
            FormField(title: "Description (Optional)", text: $description, placeholder: "Add details...")
        }
        .padding()
        .background(Color.white.opacity(0.05))
        .cornerRadius(16)
    }

    private var amountSection: some View {
        VStack(spacing: 16) {
            HStack {
                VStack(alignment: .leading, spacing: 8) {
                    Text("Total Amount")
                        .font(.subheadline)
                        .foregroundColor(.gray)

                    TextField("0.00", text: $totalAmount)
                        .keyboardType(.decimalPad)
                        .font(.title2)
                        .foregroundColor(.white)
                        .padding()
                        .background(Color.white.opacity(0.1))
                        .cornerRadius(8)
                }

                VStack(alignment: .leading, spacing: 8) {
                    Text("Token")
                        .font(.subheadline)
                        .foregroundColor(.gray)

                    Menu {
                        ForEach(tokens, id: \.self) { token in
                            Button(token) {
                                selectedToken = token
                            }
                        }
                    } label: {
                        HStack {
                            Text(selectedToken)
                                .foregroundColor(.white)
                            Image(systemName: "chevron.down")
                                .foregroundColor(.gray)
                        }
                        .padding()
                        .frame(width: 100)
                        .background(Color.white.opacity(0.1))
                        .cornerRadius(8)
                    }
                }
            }

            HStack {
                Text("Expires in")
                    .font(.subheadline)
                    .foregroundColor(.gray)

                Spacer()

                Stepper("\(expiryDays) days", value: $expiryDays, in: 1...90)
                    .foregroundColor(.white)
            }
        }
        .padding()
        .background(Color.white.opacity(0.05))
        .cornerRadius(16)
    }

    private var splitTypeSection: some View {
        VStack(alignment: .leading, spacing: 12) {
            Text("Split Type")
                .font(.headline)
                .foregroundColor(.white)

            ForEach(SplitType.allCases, id: \.self) { type in
                Button {
                    splitType = type
                    updateParticipants()
                } label: {
                    HStack {
                        Image(systemName: type.icon)
                            .foregroundColor(splitType == type ? .blue : .gray)

                        VStack(alignment: .leading, spacing: 4) {
                            Text(type.rawValue)
                                .foregroundColor(.white)
                                .font(.body)

                            Text(splitTypeDescription(type))
                                .foregroundColor(.gray)
                                .font(.caption)
                        }

                        Spacer()

                        if splitType == type {
                            Image(systemName: "checkmark.circle.fill")
                                .foregroundColor(.blue)
                        }
                    }
                    .padding()
                    .background(splitType == type ? Color.blue.opacity(0.2) : Color.white.opacity(0.05))
                    .cornerRadius(12)
                }
            }
        }
        .padding()
        .background(Color.white.opacity(0.05))
        .cornerRadius(16)
    }

    private var participantsSection: some View {
        VStack(alignment: .leading, spacing: 12) {
            HStack {
                Text("Participants (\(selectedContacts.count))")
                    .font(.headline)
                    .foregroundColor(.white)

                Spacer()

                Button {
                    showingContactPicker = true
                } label: {
                    HStack {
                        Image(systemName: "plus.circle.fill")
                        Text("Add")
                    }
                    .foregroundColor(.blue)
                }
            }

            if selectedContacts.isEmpty {
                HStack {
                    Image(systemName: "person.2")
                        .foregroundColor(.gray)
                    Text("No participants added")
                        .foregroundColor(.gray)
                }
                .frame(maxWidth: .infinity)
                .padding()
                .background(Color.white.opacity(0.05))
                .cornerRadius(12)
            } else {
                ForEach(Array(participants.enumerated()), id: \.element.id) { index, participant in
                    ParticipantRow(
                        participant: participant,
                        splitType: splitType,
                        totalAmount: Double(totalAmount) ?? 0
                    ) { updatedParticipant in
                        participants[index] = updatedParticipant
                    } onRemove: {
                        removeParticipant(participant)
                    }
                }
            }
        }
        .padding()
        .background(Color.white.opacity(0.05))
        .cornerRadius(16)
    }

    private var categorySection: some View {
        VStack(alignment: .leading, spacing: 12) {
            Text("Category (Optional)")
                .font(.headline)
                .foregroundColor(.white)

            ScrollView(.horizontal, showsIndicators: false) {
                HStack(spacing: 12) {
                    ForEach(BillCategory.allCases, id: \.self) { category in
                        CategoryChip(
                            category: category,
                            isSelected: selectedCategory == category
                        ) {
                            selectedCategory = category
                        }
                    }
                }
            }
        }
        .padding()
        .background(Color.white.opacity(0.05))
        .cornerRadius(16)
    }

    private var isValidForm: Bool {
        !title.isEmpty &&
        !totalAmount.isEmpty &&
        Double(totalAmount) ?? 0 > 0 &&
        !participants.isEmpty
    }

    private func splitTypeDescription(_ type: SplitType) -> String {
        switch type {
        case .even:
            return "Split amount equally among all participants"
        case .custom:
            return "Assign custom amounts to each participant"
        case .percentage:
            return "Split by percentage of total amount"
        case .shares:
            return "Split by shares (e.g., 1x, 2x, 3x)"
        }
    }

    private func updateParticipants() {
        let total = Double(totalAmount) ?? 0
        let count = max(selectedContacts.count, 1)

        participants = selectedContacts.map { contact in
            let amount: String
            switch splitType {
            case .even:
                amount = String(UInt64(total * 1e18 / Double(count)))
            case .custom, .percentage, .shares:
                amount = "0"
            }

            return Participant(
                contactId: contact.id,
                name: contact.name,
                address: contact.address,
                amount: amount
            )
        }
    }

    private func removeParticipant(_ participant: Participant) {
        selectedContacts.removeAll { $0.id == participant.contactId }
        participants.removeAll { $0.id == participant.id }
    }

    private func createBill() {
        guard isValidForm else { return }

        // Validate total amounts match
        let participantTotal = participants.compactMap { Double($0.amount) }.reduce(0, +)
        let billTotal = (Double(totalAmount) ?? 0) * 1e18

        if abs(participantTotal - billTotal) > 1 {
            errorMessage = "Participant amounts must equal total amount"
            showingError = true
            return
        }

        let bill = SplitBill(
            title: title,
            description: description.isEmpty ? nil : description,
            totalAmount: String(UInt64(billTotal)),
            tokenSymbol: selectedToken,
            splitType: splitType,
            participants: participants,
            creatorAddress: viewModel.userAddress,
            expiresAt: Date().addingTimeInterval(TimeInterval(expiryDays * 86400)),
            category: selectedCategory
        )

        viewModel.addBill(bill)
        dismiss()
    }
}

// MARK: - Participant Row
struct ParticipantRow: View {
    let participant: Participant
    let splitType: SplitType
    let totalAmount: Double
    let onUpdate: (Participant) -> Void
    let onRemove: () -> Void

    @State private var customAmount = ""
    @State private var percentage = ""
    @State private var shares = 1

    var body: some View {
        HStack {
            VStack(alignment: .leading, spacing: 4) {
                Text(participant.name)
                    .font(.body)
                    .foregroundColor(.white)

                Text(participant.shortAddress)
                    .font(.caption)
                    .foregroundColor(.gray)
            }

            Spacer()

            if splitType == .even {
                Text(participant.formattedAmount)
                    .font(.body)
                    .foregroundColor(.white)
            } else if splitType == .custom {
                TextField("0.00", text: $customAmount)
                    .keyboardType(.decimalPad)
                    .foregroundColor(.white)
                    .multilineTextAlignment(.trailing)
                    .frame(width: 100)
                    .padding(8)
                    .background(Color.white.opacity(0.1))
                    .cornerRadius(8)
                    .onChange(of: customAmount) { newValue in
                        updateAmount(newValue)
                    }
            } else if splitType == .percentage {
                HStack {
                    TextField("0", text: $percentage)
                        .keyboardType(.numberPad)
                        .foregroundColor(.white)
                        .multilineTextAlignment(.trailing)
                        .frame(width: 60)
                        .padding(8)
                        .background(Color.white.opacity(0.1))
                        .cornerRadius(8)
                        .onChange(of: percentage) { newValue in
                            updatePercentage(newValue)
                        }
                    Text("%")
                        .foregroundColor(.gray)
                }
            } else if splitType == .shares {
                Stepper("\(shares)x", value: $shares, in: 1...10)
                    .foregroundColor(.white)
                    .onChange(of: shares) { newValue in
                        updateShares(newValue)
                    }
            }

            Button {
                onRemove()
            } label: {
                Image(systemName: "minus.circle.fill")
                    .foregroundColor(.red)
            }
        }
        .padding()
        .background(Color.white.opacity(0.05))
        .cornerRadius(12)
    }

    private func updateAmount(_ value: String) {
        let amount = (Double(value) ?? 0) * 1e18
        var updated = participant
        updated.amount = String(UInt64(amount))
        onUpdate(updated)
    }

    private func updatePercentage(_ value: String) {
        let pct = Double(value) ?? 0
        let amount = (totalAmount * pct / 100) * 1e18
        var updated = participant
        updated.percentage = pct
        updated.amount = String(UInt64(amount))
        onUpdate(updated)
    }

    private func updateShares(_ value: Int) {
        var updated = participant
        updated.shares = value
        onUpdate(updated)
    }
}

// MARK: - Category Chip
struct CategoryChip: View {
    let category: BillCategory
    let isSelected: Bool
    let action: () -> Void

    var body: some View {
        Button(action: action) {
            VStack(spacing: 8) {
                Image(systemName: category.icon)
                    .font(.title3)
                    .foregroundColor(Color(category.color))

                Text(category.rawValue)
                    .font(.caption)
                    .foregroundColor(.white)
            }
            .frame(width: 80)
            .padding()
            .background(isSelected ? Color(category.color).opacity(0.3) : Color.white.opacity(0.05))
            .cornerRadius(12)
            .overlay(
                RoundedRectangle(cornerRadius: 12)
                    .stroke(isSelected ? Color(category.color) : Color.clear, lineWidth: 2)
            )
        }
    }
}

// MARK: - Contact Picker View
struct ContactPickerView: View {
    @Environment(\.dismiss) var dismiss
    let contacts: [Contact]
    @Binding var selectedContacts: [Contact]
    let onComplete: () -> Void

    @State private var searchText = ""

    var filteredContacts: [Contact] {
        if searchText.isEmpty {
            return contacts
        }
        return contacts.filter {
            $0.name.localizedCaseInsensitiveContains(searchText) ||
            $0.address.localizedCaseInsensitiveContains(searchText)
        }
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

                VStack {
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

                    // Contacts List
                    List {
                        ForEach(filteredContacts) { contact in
                            Button {
                                toggleSelection(contact)
                            } label: {
                                HStack {
                                    Text(contact.name)
                                        .foregroundColor(.white)

                                    Spacer()

                                    if selectedContacts.contains(where: { $0.id == contact.id }) {
                                        Image(systemName: "checkmark.circle.fill")
                                            .foregroundColor(.blue)
                                    }
                                }
                            }
                            .listRowBackground(Color.white.opacity(0.05))
                        }
                    }
                    .listStyle(.plain)
                    .scrollContentBackground(.hidden)
                }
            }
            .navigationTitle("Select Participants")
            .navigationBarTitleDisplayMode(.inline)
            .toolbar {
                ToolbarItem(placement: .navigationBarLeading) {
                    Button("Cancel") {
                        dismiss()
                    }
                    .foregroundColor(.white)
                }

                ToolbarItem(placement: .navigationBarTrailing) {
                    Button("Done") {
                        onComplete()
                        dismiss()
                    }
                    .foregroundColor(.white)
                    .disabled(selectedContacts.isEmpty)
                }
            }
        }
    }

    private func toggleSelection(_ contact: Contact) {
        if let index = selectedContacts.firstIndex(where: { $0.id == contact.id }) {
            selectedContacts.remove(at: index)
        } else {
            selectedContacts.append(contact)
        }
    }
}

struct CreateBillView_Previews: PreviewProvider {
    static var previews: some View {
        CreateBillView(viewModel: BillSplitViewModel())
    }
}
