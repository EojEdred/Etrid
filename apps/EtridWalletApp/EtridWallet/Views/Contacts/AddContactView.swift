//
//  AddContactView.swift
//  EtridWallet
//
//  View for adding/editing contacts
//

import SwiftUI

struct AddContactView: View {
    @Environment(\.dismiss) var dismiss
    @ObservedObject var viewModel: ContactsViewModel

    var existingContact: Contact?

    @State private var name = ""
    @State private var address = ""
    @State private var email = ""
    @State private var phone = ""
    @State private var notes = ""
    @State private var selectedAvatar = ""
    @State private var tags: [String] = []
    @State private var newTag = ""
    @State private var isFavorite = false
    @State private var showingScanner = false
    @State private var showingError = false
    @State private var errorMessage = ""

    private let avatarEmojis = ["👤", "👨", "👩", "👦", "👧", "🧑", "👴", "👵", "🧔", "👨‍💼", "👩‍💼"]

    var isEditing: Bool {
        existingContact != nil
    }

    init(viewModel: ContactsViewModel, contact: Contact? = nil) {
        self.viewModel = viewModel
        self.existingContact = contact

        if let contact = contact {
            _name = State(initialValue: contact.name)
            _address = State(initialValue: contact.address)
            _email = State(initialValue: contact.email ?? "")
            _phone = State(initialValue: contact.phone ?? "")
            _notes = State(initialValue: contact.notes ?? "")
            _selectedAvatar = State(initialValue: contact.avatar ?? "")
            _tags = State(initialValue: contact.tags)
            _isFavorite = State(initialValue: contact.isFavorite)
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

                ScrollView {
                    VStack(spacing: 24) {
                        // Avatar Selection
                        avatarSection

                        // Contact Information
                        VStack(spacing: 16) {
                            FormField(title: "Name", text: $name, placeholder: "Enter name")

                            HStack {
                                FormField(title: "Address", text: $address, placeholder: "0x...")

                                Button {
                                    showingScanner = true
                                } label: {
                                    Image(systemName: "qrcode.viewfinder")
                                        .foregroundColor(.white)
                                        .padding()
                                        .background(Color.blue.opacity(0.3))
                                        .cornerRadius(8)
                                }
                            }

                            FormField(title: "Email", text: $email, placeholder: "email@example.com")
                            FormField(title: "Phone", text: $phone, placeholder: "+1 234 567 8900")

                            // Tags
                            tagsSection

                            // Notes
                            VStack(alignment: .leading, spacing: 8) {
                                Text("Notes")
                                    .font(.subheadline)
                                    .foregroundColor(.gray)

                                TextEditor(text: $notes)
                                    .foregroundColor(.white)
                                    .frame(height: 100)
                                    .padding(8)
                                    .background(Color.white.opacity(0.1))
                                    .cornerRadius(8)
                            }

                            // Favorite Toggle
                            Toggle(isOn: $isFavorite) {
                                HStack {
                                    Image(systemName: "star.fill")
                                        .foregroundColor(.yellow)
                                    Text("Add to Favorites")
                                        .foregroundColor(.white)
                                }
                            }
                            .toggleStyle(SwitchToggleStyle(tint: .yellow))
                        }
                        .padding()
                        .background(Color.white.opacity(0.05))
                        .cornerRadius(16)

                        // Save Button
                        Button {
                            saveContact()
                        } label: {
                            Text(isEditing ? "Update Contact" : "Add Contact")
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
                        .disabled(name.isEmpty || address.isEmpty)
                        .opacity((name.isEmpty || address.isEmpty) ? 0.5 : 1.0)
                    }
                    .padding()
                }
            }
            .navigationTitle(isEditing ? "Edit Contact" : "Add Contact")
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
        }
    }

    private var avatarSection: some View {
        VStack(spacing: 12) {
            Text("Choose Avatar")
                .font(.subheadline)
                .foregroundColor(.gray)

            ScrollView(.horizontal, showsIndicators: false) {
                HStack(spacing: 12) {
                    ForEach(avatarEmojis, id: \.self) { emoji in
                        Button {
                            selectedAvatar = emoji
                        } label: {
                            ZStack {
                                Circle()
                                    .fill(selectedAvatar == emoji ? Color.blue.opacity(0.3) : Color.white.opacity(0.1))
                                    .frame(width: 50, height: 50)

                                Text(emoji)
                                    .font(.title2)
                            }
                        }
                    }
                }
            }
        }
        .padding()
        .background(Color.white.opacity(0.05))
        .cornerRadius(16)
    }

    private var tagsSection: some View {
        VStack(alignment: .leading, spacing: 8) {
            Text("Tags")
                .font(.subheadline)
                .foregroundColor(.gray)

            FlowLayout(spacing: 8) {
                ForEach(tags, id: \.self) { tag in
                    TagChip(text: tag, onDelete: {
                        tags.removeAll { $0 == tag }
                    })
                }
            }

            HStack {
                TextField("Add tag", text: $newTag)
                    .foregroundColor(.white)
                    .padding(8)
                    .background(Color.white.opacity(0.1))
                    .cornerRadius(8)

                Button {
                    if !newTag.isEmpty && !tags.contains(newTag) {
                        tags.append(newTag)
                        newTag = ""
                    }
                } label: {
                    Image(systemName: "plus.circle.fill")
                        .foregroundColor(.blue)
                        .font(.title2)
                }
                .disabled(newTag.isEmpty)
            }
        }
    }

    private func saveContact() {
        // Validate address
        guard address.hasPrefix("0x") && address.count == 42 else {
            errorMessage = "Invalid Ethereum address"
            showingError = true
            return
        }

        let contact = Contact(
            id: existingContact?.id ?? UUID(),
            name: name,
            address: address,
            email: email.isEmpty ? nil : email,
            phone: phone.isEmpty ? nil : phone,
            avatar: selectedAvatar.isEmpty ? nil : selectedAvatar,
            isFavorite: isFavorite,
            tags: tags,
            createdAt: existingContact?.createdAt ?? Date(),
            lastInteraction: existingContact?.lastInteraction,
            notes: notes.isEmpty ? nil : notes
        )

        if isEditing {
            viewModel.updateContact(contact)
        } else {
            viewModel.addContact(contact)
        }

        dismiss()
    }
}

// MARK: - Form Field Component
struct FormField: View {
    let title: String
    @Binding var text: String
    let placeholder: String

    var body: some View {
        VStack(alignment: .leading, spacing: 8) {
            Text(title)
                .font(.subheadline)
                .foregroundColor(.gray)

            TextField(placeholder, text: $text)
                .foregroundColor(.white)
                .padding()
                .background(Color.white.opacity(0.1))
                .cornerRadius(8)
        }
    }
}

// MARK: - Tag Chip Component
struct TagChip: View {
    let text: String
    let onDelete: () -> Void

    var body: some View {
        HStack(spacing: 4) {
            Text(text)
                .font(.caption)
                .foregroundColor(.white)

            Button(action: onDelete) {
                Image(systemName: "xmark.circle.fill")
                    .font(.caption)
                    .foregroundColor(.white.opacity(0.7))
            }
        }
        .padding(.horizontal, 10)
        .padding(.vertical, 6)
        .background(Color.blue.opacity(0.4))
        .cornerRadius(12)
    }
}

// MARK: - Flow Layout for Tags
struct FlowLayout: Layout {
    var spacing: CGFloat = 8

    func sizeThatFits(proposal: ProposedViewSize, subviews: Subviews, cache: inout ()) -> CGSize {
        let result = FlowResult(
            in: proposal.replacingUnspecifiedDimensions().width,
            subviews: subviews,
            spacing: spacing
        )
        return result.size
    }

    func placeSubviews(in bounds: CGRect, proposal: ProposedViewSize, subviews: Subviews, cache: inout ()) {
        let result = FlowResult(
            in: bounds.width,
            subviews: subviews,
            spacing: spacing
        )
        for (index, subview) in subviews.enumerated() {
            subview.place(at: CGPoint(x: bounds.minX + result.positions[index].x,
                                     y: bounds.minY + result.positions[index].y),
                         proposal: .unspecified)
        }
    }

    struct FlowResult {
        var size: CGSize = .zero
        var positions: [CGPoint] = []

        init(in maxWidth: CGFloat, subviews: Subviews, spacing: CGFloat) {
            var x: CGFloat = 0
            var y: CGFloat = 0
            var lineHeight: CGFloat = 0

            for subview in subviews {
                let size = subview.sizeThatFits(.unspecified)

                if x + size.width > maxWidth && x > 0 {
                    x = 0
                    y += lineHeight + spacing
                    lineHeight = 0
                }

                positions.append(CGPoint(x: x, y: y))
                lineHeight = max(lineHeight, size.height)
                x += size.width + spacing
            }

            self.size = CGSize(width: maxWidth, height: y + lineHeight)
        }
    }
}

struct AddContactView_Previews: PreviewProvider {
    static var previews: some View {
        AddContactView(viewModel: ContactsViewModel())
    }
}
