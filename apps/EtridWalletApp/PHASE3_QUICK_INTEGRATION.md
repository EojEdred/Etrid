# Phase 3 - Quick Integration Guide

## Step 1: Update MainTabView (Required)

Edit: `/Users/macbook/Desktop/etrid/apps/EtridWalletApp/EtridWallet/Views/MainTabView.swift`

Replace lines 36-81 (the SocialView placeholder) with:

```swift
// Social Tab
SocialTabView()
    .tabItem {
        Label("Social", systemImage: "person.2")
    }
    .tag(3)
```

## Step 2: Add to Xcode Project

Add all new files to your Xcode project:

### Models
- Models/Contact.swift
- Models/BillSplit.swift
- Models/Social.swift

### Views/Contacts
- Views/Contacts/ContactsView.swift
- Views/Contacts/AddContactView.swift
- Views/Contacts/ContactDetailView.swift

### Views/BillSplit
- Views/BillSplit/BillSplitView.swift
- Views/BillSplit/CreateBillView.swift
- Views/BillSplit/BillDetailView.swift

### Views/Social
- Views/Social/SocialFeedView.swift
- Views/Social/PaymentRequestView.swift
- Views/Social/SocialTabView.swift

## Step 3: Storage Integration (Optional but Recommended)

Add these methods to `StorageManager.swift`:

```swift
// Contacts
func saveContacts(_ contacts: [Contact]) {
    if let data = try? JSONEncoder().encode(contacts) {
        UserDefaults.standard.set(data, forKey: "etrid.contacts")
    }
}

func loadContacts() -> [Contact] {
    guard let data = UserDefaults.standard.data(forKey: "etrid.contacts"),
          let contacts = try? JSONDecoder().decode([Contact].self, from: data) else {
        return []
    }
    return contacts
}

// Bills
func saveBills(_ bills: [SplitBill]) {
    if let data = try? JSONEncoder().encode(bills) {
        UserDefaults.standard.set(data, forKey: "etrid.bills")
    }
}

func loadBills() -> [SplitBill] {
    guard let data = UserDefaults.standard.data(forKey: "etrid.bills"),
          let bills = try? JSONDecoder().decode([SplitBill].self, from: data) else {
        return []
    }
    return bills
}

// Payment Requests
func savePaymentRequests(_ requests: [PaymentRequest]) {
    if let data = try? JSONEncoder().encode(requests) {
        UserDefaults.standard.set(data, forKey: "etrid.paymentRequests")
    }
}

func loadPaymentRequests() -> [PaymentRequest] {
    guard let data = UserDefaults.standard.data(forKey: "etrid.paymentRequests"),
          let requests = try? JSONDecoder().decode([PaymentRequest].self, from: data) else {
        return []
    }
    return requests
}

// Activities
func saveActivities(_ activities: [ActivityItem]) {
    if let data = try? JSONEncoder().encode(activities) {
        UserDefaults.standard.set(data, forKey: "etrid.activities")
    }
}

func loadActivities() -> [ActivityItem] {
    guard let data = UserDefaults.standard.data(forKey: "etrid.activities"),
          let activities = try? JSONDecoder().decode([ActivityItem].self, from: data) else {
        return []
    }
    return activities
}
```

## Step 4: Update ViewModels to Use Storage

### ContactsViewModel
```swift
private func loadContacts() {
    // Replace placeholder with:
    contacts = StorageManager.shared.loadContacts()
}

func addContact(_ contact: Contact) {
    contacts.append(contact)
    StorageManager.shared.saveContacts(contacts)
}

func deleteContact(_ contact: Contact) {
    contacts.removeAll { $0.id == contact.id }
    StorageManager.shared.saveContacts(contacts)
}

func updateContact(_ contact: Contact) {
    if let index = contacts.firstIndex(where: { $0.id == contact.id }) {
        contacts[index] = contact
        StorageManager.shared.saveContacts(contacts)
    }
}
```

### BillSplitViewModel
```swift
private func loadBills() {
    bills = StorageManager.shared.loadBills()
}

func addBill(_ bill: SplitBill) {
    bills.insert(bill, at: 0)
    StorageManager.shared.saveBills(bills)
}

func updateBill(_ bill: SplitBill) {
    if let index = bills.firstIndex(where: { $0.id == bill.id }) {
        bills[index] = bill
        StorageManager.shared.saveBills(bills)
    }
}
```

### SocialFeedViewModel
```swift
private func loadData() {
    activities = StorageManager.shared.loadActivities()
    paymentRequests = StorageManager.shared.loadPaymentRequests()
    // Add similar for other data
}
```

## Step 5: Test the Integration

1. Build and run the app
2. Tap the "Social" tab
3. Test each feature:
   - Create a contact
   - Create a bill split
   - View social feed

## Optional Enhancements

### QR Scanner Integration
In `AddContactView.swift`, connect the QR scanner:

```swift
.sheet(isPresented: $showingScanner) {
    QRScannerView { scannedAddress in
        address = scannedAddress
        showingScanner = false
    }
}
```

### Send Integration
Update `SendView.swift` to include contact picker:

```swift
// Add contact selection button
Button("Select from Contacts") {
    showingContactPicker = true
}

// Add sheet
.sheet(isPresented: $showingContactPicker) {
    SingleContactPickerView(
        contacts: contactsViewModel.contacts,
        selectedContact: $selectedContact
    )
}

// Update address when contact selected
.onChange(of: selectedContact) { contact in
    if let contact = contact {
        recipientAddress = contact.address
    }
}
```

## Troubleshooting

### Build Errors
- Ensure all files are added to the Xcode target
- Clean build folder (Cmd+Shift+K)
- Rebuild project

### Missing Color Extensions
If `.etridBlue` or `.etridPurple` are not defined, add to `Extensions.swift`:

```swift
extension Color {
    static let etridBlue = Color(red: 0.2, green: 0.5, blue: 1.0)
    static let etridPurple = Color(red: 0.6, green: 0.3, blue: 0.9)
}
```

### Navigation Issues
Make sure `SocialTabView` is wrapped in `NavigationView` (already done in the implementation).

## File Locations

All files are in: `/Users/macbook/Desktop/etrid/apps/EtridWalletApp/EtridWallet/`

```
Models/
├── Contact.swift (156 lines)
├── BillSplit.swift (295 lines)
└── Social.swift (330 lines)

Views/Contacts/
├── ContactsView.swift
├── AddContactView.swift
└── ContactDetailView.swift

Views/BillSplit/
├── BillSplitView.swift
├── CreateBillView.swift
└── BillDetailView.swift

Views/Social/
├── SocialFeedView.swift
├── PaymentRequestView.swift
└── SocialTabView.swift
```

## Code Statistics

- Total Files: 12 Swift files
- Total Lines: ~4,600 lines
- Models: 781 lines
- Views: 3,816 lines
- ViewModels: Integrated in views

## Next Phase

After Phase 3 integration, you can proceed to:
- Phase 4: Advanced DeFi features
- Phase 5: Web3 integrations
- Phase 6: Enterprise features

For detailed documentation, see: `PHASE3_IMPLEMENTATION_SUMMARY.md`
