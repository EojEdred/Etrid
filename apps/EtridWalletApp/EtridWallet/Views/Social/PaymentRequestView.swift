//
//  PaymentRequestView.swift
//  EtridWallet
//
//  View for creating payment requests
//

import SwiftUI

struct PaymentRequestView: View {
    @Environment(\.dismiss) var dismiss
    @StateObject private var contactsViewModel = ContactsViewModel()

    @State private var selectedContact: Contact?
    @State private var amount = ""
    @State private var selectedToken = "ETR"
    @State private var message = ""
    @State private var expiryDays = 7
    @State private var showingContactPicker = false
    @State private var showingSuccess = false
    @State private var showingError = false
    @State private var errorMessage = ""

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
                        // Header
                        headerSection

                        // Contact Selection
                        contactSection

                        // Amount
                        amountSection

                        // Message
                        messageSection

                        // Expiry
                        expirySection

                        // Create Button
                        Button {
                            createRequest()
                        } label: {
                            HStack {
                                Image(systemName: "paperplane.fill")
                                Text("Send Request")
                                    .fontWeight(.semibold)
                            }
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
            .navigationTitle("Payment Request")
            .navigationBarTitleDisplayMode(.inline)
            .toolbar {
                ToolbarItem(placement: .navigationBarLeading) {
                    Button("Cancel") {
                        dismiss()
                    }
                    .foregroundColor(.white)
                }
            }
            .alert("Success", isPresented: $showingSuccess) {
                Button("OK") {
                    dismiss()
                }
            } message: {
                Text("Payment request sent successfully")
            }
            .alert("Error", isPresented: $showingError) {
                Button("OK", role: .cancel) { }
            } message: {
                Text(errorMessage)
            }
            .sheet(isPresented: $showingContactPicker) {
                SingleContactPickerView(
                    contacts: contactsViewModel.contacts,
                    selectedContact: $selectedContact
                )
            }
        }
    }

    private var headerSection: some View {
        VStack(spacing: 12) {
            ZStack {
                Circle()
                    .fill(
                        LinearGradient(
                            colors: [Color.blue.opacity(0.6), Color.purple.opacity(0.6)],
                            startPoint: .topLeading,
                            endPoint: .bottomTrailing
                        )
                    )
                    .frame(width: 80, height: 80)

                Image(systemName: "arrow.down.circle.fill")
                    .font(.system(size: 40))
                    .foregroundColor(.white)
            }

            Text("Request Payment")
                .font(.title2)
                .fontWeight(.bold)
                .foregroundColor(.white)

            Text("Ask someone to send you crypto")
                .font(.subheadline)
                .foregroundColor(.gray)
        }
        .padding()
        .background(Color.white.opacity(0.05))
        .cornerRadius(16)
    }

    private var contactSection: some View {
        VStack(alignment: .leading, spacing: 12) {
            Text("From")
                .font(.headline)
                .foregroundColor(.white)

            if let contact = selectedContact {
                HStack {
                    ZStack {
                        Circle()
                            .fill(
                                LinearGradient(
                                    colors: [Color.blue.opacity(0.6), Color.purple.opacity(0.6)],
                                    startPoint: .topLeading,
                                    endPoint: .bottomTrailing
                                )
                            )
                            .frame(width: 40, height: 40)

                        if let avatar = contact.avatar, avatar.count == 1 {
                            Text(avatar)
                                .font(.title3)
                        } else {
                            Text(contact.initials)
                                .font(.subheadline)
                                .foregroundColor(.white)
                        }
                    }

                    VStack(alignment: .leading, spacing: 4) {
                        Text(contact.name)
                            .font(.body)
                            .fontWeight(.semibold)
                            .foregroundColor(.white)

                        Text(contact.shortAddress)
                            .font(.caption)
                            .foregroundColor(.gray)
                    }

                    Spacer()

                    Button {
                        showingContactPicker = true
                    } label: {
                        Text("Change")
                            .font(.subheadline)
                            .foregroundColor(.blue)
                    }
                }
                .padding()
                .background(Color.white.opacity(0.05))
                .cornerRadius(12)
            } else {
                Button {
                    showingContactPicker = true
                } label: {
                    HStack {
                        Image(systemName: "person.crop.circle.badge.plus")
                            .foregroundColor(.blue)

                        Text("Select Contact")
                            .foregroundColor(.white)

                        Spacer()

                        Image(systemName: "chevron.right")
                            .foregroundColor(.gray)
                    }
                    .padding()
                    .background(Color.white.opacity(0.05))
                    .cornerRadius(12)
                }
            }
        }
        .padding()
        .background(Color.white.opacity(0.05))
        .cornerRadius(16)
    }

    private var amountSection: some View {
        VStack(spacing: 16) {
            HStack {
                VStack(alignment: .leading, spacing: 8) {
                    Text("Amount")
                        .font(.subheadline)
                        .foregroundColor(.gray)

                    TextField("0.00", text: $amount)
                        .keyboardType(.decimalPad)
                        .font(.title)
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
        }
        .padding()
        .background(Color.white.opacity(0.05))
        .cornerRadius(16)
    }

    private var messageSection: some View {
        VStack(alignment: .leading, spacing: 12) {
            Text("Message (Optional)")
                .font(.headline)
                .foregroundColor(.white)

            TextEditor(text: $message)
                .foregroundColor(.white)
                .frame(height: 100)
                .padding(8)
                .background(Color.white.opacity(0.1))
                .cornerRadius(8)
                .overlay(
                    Group {
                        if message.isEmpty {
                            Text("Add a message to your request...")
                                .foregroundColor(.gray)
                                .padding(.horizontal, 12)
                                .padding(.vertical, 16)
                                .frame(maxWidth: .infinity, maxHeight: .infinity, alignment: .topLeading)
                                .allowsHitTesting(false)
                        }
                    }
                )
        }
        .padding()
        .background(Color.white.opacity(0.05))
        .cornerRadius(16)
    }

    private var expirySection: some View {
        VStack(spacing: 12) {
            HStack {
                VStack(alignment: .leading, spacing: 4) {
                    Text("Request Expiry")
                        .font(.headline)
                        .foregroundColor(.white)

                    Text("Request will expire after \(expiryDays) days")
                        .font(.caption)
                        .foregroundColor(.gray)
                }

                Spacer()

                Stepper("\(expiryDays)d", value: $expiryDays, in: 1...90)
                    .foregroundColor(.white)
            }
        }
        .padding()
        .background(Color.white.opacity(0.05))
        .cornerRadius(16)
    }

    private var isValidForm: Bool {
        selectedContact != nil &&
        !amount.isEmpty &&
        Double(amount) ?? 0 > 0
    }

    private func createRequest() {
        guard let contact = selectedContact else {
            errorMessage = "Please select a contact"
            showingError = true
            return
        }

        guard let amountValue = Double(amount), amountValue > 0 else {
            errorMessage = "Please enter a valid amount"
            showingError = true
            return
        }

        // Create payment request
        let request = PaymentRequest(
            fromAddress: "0x742d35Cc6634C0532925a3b844Bc9e7595f0bEb", // Current user
            toAddress: contact.address,
            fromName: "Me",
            toName: contact.name,
            amount: String(UInt64(amountValue * 1e18)),
            tokenSymbol: selectedToken,
            message: message.isEmpty ? nil : message,
            expiresAt: Date().addingTimeInterval(TimeInterval(expiryDays * 86400))
        )

        // Save and send request
        // TODO: Integrate with storage and notification system

        showingSuccess = true
    }
}

// MARK: - Single Contact Picker
struct SingleContactPickerView: View {
    @Environment(\.dismiss) var dismiss
    let contacts: [Contact]
    @Binding var selectedContact: Contact?

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
                                selectedContact = contact
                                dismiss()
                            } label: {
                                HStack {
                                    ZStack {
                                        Circle()
                                            .fill(
                                                LinearGradient(
                                                    colors: [Color.blue.opacity(0.6), Color.purple.opacity(0.6)],
                                                    startPoint: .topLeading,
                                                    endPoint: .bottomTrailing
                                                )
                                            )
                                            .frame(width: 40, height: 40)

                                        if let avatar = contact.avatar, avatar.count == 1 {
                                            Text(avatar)
                                        } else {
                                            Text(contact.initials)
                                                .font(.subheadline)
                                                .foregroundColor(.white)
                                        }
                                    }

                                    VStack(alignment: .leading, spacing: 4) {
                                        Text(contact.name)
                                            .foregroundColor(.white)

                                        Text(contact.shortAddress)
                                            .font(.caption)
                                            .foregroundColor(.gray)
                                    }

                                    Spacer()
                                }
                            }
                            .listRowBackground(Color.white.opacity(0.05))
                        }
                    }
                    .listStyle(.plain)
                    .scrollContentBackground(.hidden)
                }
            }
            .navigationTitle("Select Contact")
            .navigationBarTitleDisplayMode(.inline)
            .toolbar {
                ToolbarItem(placement: .navigationBarLeading) {
                    Button("Cancel") {
                        dismiss()
                    }
                    .foregroundColor(.white)
                }
            }
        }
    }
}

struct PaymentRequestView_Previews: PreviewProvider {
    static var previews: some View {
        PaymentRequestView()
    }
}
