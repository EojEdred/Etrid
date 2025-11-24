//
//  MyRentalsView.swift
//  EtridWallet
//
//  My Rentals View
//

import SwiftUI

struct MyRentalsView: View {
    @StateObject private var service = GPUMarketplaceService()
    @State private var rentals: [Rental] = []
    @State private var selectedStatus: RentalStatus?
    @State private var selectedRental: Rental?

    var body: some View {
        VStack(spacing: 0) {
            // Status Filter
            ScrollView(.horizontal, showsIndicators: false) {
                HStack(spacing: 12) {
                    StatusFilterButton(title: "All", status: nil, selectedStatus: $selectedStatus, onTap: loadRentals)
                    StatusFilterButton(title: "Active", status: .active, selectedStatus: $selectedStatus, onTap: loadRentals)
                    StatusFilterButton(title: "Pending", status: .pending, selectedStatus: $selectedStatus, onTap: loadRentals)
                    StatusFilterButton(title: "Completed", status: .completed, selectedStatus: $selectedStatus, onTap: loadRentals)
                }
                .padding()
            }

            // Rentals List
            if service.isLoading {
                Spacer()
                ProgressView("Loading rentals...")
                Spacer()
            } else if rentals.isEmpty {
                Spacer()
                VStack(spacing: 16) {
                    Image(systemName: "tray")
                        .font(.system(size: 60))
                        .foregroundColor(.gray)
                    Text("No Rentals Found")
                        .font(.title2)
                        .foregroundColor(.gray)
                    Text("You haven't rented any GPUs yet")
                        .font(.subheadline)
                        .foregroundColor(.gray)
                }
                Spacer()
            } else {
                ScrollView {
                    LazyVStack(spacing: 12) {
                        ForEach(rentals) { rental in
                            RentalCardView(rental: rental)
                                .onTapGesture {
                                    selectedRental = rental
                                }
                        }
                    }
                    .padding()
                }
            }
        }
        .navigationTitle("My Rentals")
        .navigationBarTitleDisplayMode(.inline)
        .sheet(item: $selectedRental) { rental in
            RentalDetailView(rental: rental)
        }
        .onAppear {
            loadRentals()
        }
    }

    private func loadRentals() {
        Task {
            do {
                // Replace with actual wallet address
                let address = "0x1234567890123456789012345678901234567890"
                rentals = try await service.getMyRentals(address: address, status: selectedStatus)
            } catch {
                print("Error loading rentals: \(error)")
            }
        }
    }
}

// MARK: - Status Filter Button
struct StatusFilterButton: View {
    let title: String
    let status: RentalStatus?
    @Binding var selectedStatus: RentalStatus?
    let onTap: () -> Void

    var isSelected: Bool {
        if title == "All" {
            return selectedStatus == nil
        }
        return selectedStatus == status
    }

    var body: some View {
        Button(action: {
            selectedStatus = status
            onTap()
        }) {
            Text(title)
                .font(.subheadline)
                .fontWeight(.medium)
                .foregroundColor(isSelected ? .white : .blue)
                .padding(.horizontal, 16)
                .padding(.vertical, 8)
                .background(isSelected ? Color.blue : Color.blue.opacity(0.1))
                .cornerRadius(20)
        }
    }
}

// MARK: - Rental Card View
struct RentalCardView: View {
    let rental: Rental

    var statusColor: Color {
        Color(hex: rental.status.color) ?? .gray
    }

    var body: some View {
        VStack(alignment: .leading, spacing: 12) {
            // Header
            HStack {
                if let gpu = rental.gpu {
                    Text(gpu.specs.displayName)
                        .font(.headline)
                } else {
                    Text("GPU Rental")
                        .font(.headline)
                }

                Spacer()

                Text(rental.status.rawValue.capitalized)
                    .font(.caption)
                    .fontWeight(.medium)
                    .foregroundColor(.white)
                    .padding(.horizontal, 10)
                    .padding(.vertical, 4)
                    .background(statusColor)
                    .cornerRadius(12)
            }

            // Duration and Time Remaining
            if rental.status == .active {
                VStack(spacing: 8) {
                    HStack {
                        Text("Time Remaining")
                            .font(.caption)
                            .foregroundColor(.gray)
                        Spacer()
                        Text(rental.timeRemaining)
                            .font(.subheadline)
                            .fontWeight(.medium)
                    }

                    ProgressView(value: rental.progressPercentage, total: 100)
                        .progressViewStyle(LinearProgressViewStyle(tint: .blue))
                }
            }

            // Rental Info
            HStack(spacing: 16) {
                VStack(alignment: .leading, spacing: 4) {
                    Text("Duration")
                        .font(.caption)
                        .foregroundColor(.gray)
                    Text("\(rental.durationHours)h")
                        .font(.subheadline)
                        .fontWeight(.medium)
                }

                Divider()
                    .frame(height: 30)

                VStack(alignment: .leading, spacing: 4) {
                    Text("Total Cost")
                        .font(.caption)
                        .foregroundColor(.gray)
                    Text(String(format: "%.4f ETR", rental.totalCost))
                        .font(.subheadline)
                        .fontWeight(.medium)
                }

                Divider()
                    .frame(height: 30)

                VStack(alignment: .leading, spacing: 4) {
                    Text("Start Time")
                        .font(.caption)
                        .foregroundColor(.gray)
                    Text(rental.startTime, style: .relative)
                        .font(.subheadline)
                        .fontWeight(.medium)
                }
            }

            // SSH Credentials Button
            if rental.status == .active, let credentials = rental.sshCredentials {
                NavigationLink(destination: SSHCredentialsView(credentials: credentials)) {
                    HStack {
                        Image(systemName: "terminal.fill")
                        Text("View SSH Credentials")
                    }
                    .font(.subheadline)
                    .fontWeight(.medium)
                    .foregroundColor(.white)
                    .frame(maxWidth: .infinity)
                    .padding(.vertical, 10)
                    .background(Color.blue)
                    .cornerRadius(8)
                }
            }
        }
        .padding()
        .background(Color(.systemBackground))
        .cornerRadius(12)
        .shadow(color: Color.black.opacity(0.1), radius: 5, x: 0, y: 2)
    }
}

// MARK: - Rental Detail View
struct RentalDetailView: View {
    let rental: Rental
    @StateObject private var service = GPUMarketplaceService()
    @State private var metrics: [UsageMetrics] = []
    @Environment(\.presentationMode) var presentationMode

    var body: some View {
        NavigationView {
            ScrollView {
                VStack(alignment: .leading, spacing: 20) {
                    // GPU Info
                    if let gpu = rental.gpu {
                        VStack(alignment: .leading, spacing: 12) {
                            Text(gpu.specs.displayName)
                                .font(.title)
                                .fontWeight(.bold)

                            Text(gpu.location)
                                .font(.subheadline)
                                .foregroundColor(.gray)
                        }
                        .frame(maxWidth: .infinity, alignment: .leading)
                        .padding()
                        .background(Color(.systemGray6))
                        .cornerRadius(12)
                    }

                    // Rental Status
                    HStack {
                        Text("Status")
                            .font(.headline)
                        Spacer()
                        Text(rental.status.rawValue.capitalized)
                            .font(.subheadline)
                            .fontWeight(.medium)
                            .foregroundColor(.white)
                            .padding(.horizontal, 12)
                            .padding(.vertical, 6)
                            .background(Color(hex: rental.status.color) ?? .gray)
                            .cornerRadius(12)
                    }

                    // Time Info
                    VStack(spacing: 12) {
                        HStack {
                            Text("Start Time")
                                .font(.subheadline)
                                .foregroundColor(.gray)
                            Spacer()
                            Text(rental.startTime, style: .date)
                                .font(.subheadline)
                                .fontWeight(.medium)
                        }

                        HStack {
                            Text("End Time")
                                .font(.subheadline)
                                .foregroundColor(.gray)
                            Spacer()
                            Text(rental.endTime, style: .date)
                                .font(.subheadline)
                                .fontWeight(.medium)
                        }

                        if rental.status == .active {
                            Divider()
                            HStack {
                                Text("Time Remaining")
                                    .font(.subheadline)
                                    .foregroundColor(.gray)
                                Spacer()
                                Text(rental.timeRemaining)
                                    .font(.subheadline)
                                    .fontWeight(.medium)
                                    .foregroundColor(.blue)
                            }
                        }
                    }
                    .padding()
                    .background(Color(.systemGray6))
                    .cornerRadius(12)

                    // SSH Credentials
                    if let credentials = rental.sshCredentials {
                        NavigationLink(destination: SSHCredentialsView(credentials: credentials)) {
                            HStack {
                                Image(systemName: "terminal.fill")
                                    .font(.title3)
                                VStack(alignment: .leading, spacing: 4) {
                                    Text("SSH Credentials")
                                        .font(.headline)
                                    Text("Tap to view connection details")
                                        .font(.caption)
                                        .foregroundColor(.gray)
                                }
                                Spacer()
                                Image(systemName: "chevron.right")
                                    .foregroundColor(.gray)
                            }
                            .padding()
                            .background(Color(.systemGray6))
                            .cornerRadius(12)
                        }
                    }

                    // Usage Metrics
                    if rental.status == .active {
                        VStack(alignment: .leading, spacing: 12) {
                            Text("Usage Metrics")
                                .font(.headline)

                            if let latestMetric = metrics.first {
                                VStack(spacing: 12) {
                                    MetricRow(icon: "cpu", label: "CPU Usage", value: latestMetric.cpuDisplay)
                                    MetricRow(icon: "memorychip", label: "GPU Usage", value: latestMetric.gpuDisplay)
                                    MetricRow(icon: "chart.bar.fill", label: "Memory Usage", value: latestMetric.memoryDisplay)
                                    MetricRow(icon: "arrow.down.circle", label: "Network Rx", value: latestMetric.networkRxDisplay)
                                    MetricRow(icon: "arrow.up.circle", label: "Network Tx", value: latestMetric.networkTxDisplay)
                                }
                                .padding()
                                .background(Color(.systemGray6))
                                .cornerRadius(12)
                            } else {
                                Text("No metrics available")
                                    .font(.subheadline)
                                    .foregroundColor(.gray)
                                    .frame(maxWidth: .infinity)
                                    .padding()
                                    .background(Color(.systemGray6))
                                    .cornerRadius(12)
                            }
                        }
                    }
                }
                .padding()
            }
            .navigationTitle("Rental Details")
            .navigationBarTitleDisplayMode(.inline)
            .toolbar {
                ToolbarItem(placement: .navigationBarTrailing) {
                    Button("Done") {
                        presentationMode.wrappedValue.dismiss()
                    }
                }
            }
            .onAppear(perform: loadMetrics)
        }
    }

    private func loadMetrics() {
        if rental.status == .active {
            Task {
                do {
                    metrics = try await service.getUsageMetrics(rentalId: rental.id)
                } catch {
                    print("Error loading metrics: \(error)")
                }
            }
        }
    }
}

// MARK: - Metric Row
struct MetricRow: View {
    let icon: String
    let label: String
    let value: String

    var body: some View {
        HStack {
            Image(systemName: icon)
                .foregroundColor(.blue)
                .frame(width: 24)
            Text(label)
                .font(.subheadline)
            Spacer()
            Text(value)
                .font(.subheadline)
                .fontWeight(.medium)
        }
    }
}

// MARK: - SSH Credentials View
struct SSHCredentialsView: View {
    let credentials: SSHCredentials
    @State private var showCopiedAlert = false

    var body: some View {
        ScrollView {
            VStack(alignment: .leading, spacing: 20) {
                // Connection String
                VStack(alignment: .leading, spacing: 8) {
                    Text("Connection String")
                        .font(.headline)

                    HStack {
                        Text(credentials.connectionString)
                            .font(.system(.subheadline, design: .monospaced))
                            .padding()
                            .frame(maxWidth: .infinity, alignment: .leading)
                            .background(Color(.systemGray6))
                            .cornerRadius(8)

                        Button(action: {
                            UIPasteboard.general.string = credentials.connectionString
                            showCopiedAlert = true
                        }) {
                            Image(systemName: "doc.on.doc.fill")
                                .foregroundColor(.blue)
                        }
                        .padding(.trailing)
                    }
                }

                // Credentials Details
                VStack(spacing: 12) {
                    CredentialRow(label: "Host", value: credentials.host)
                    CredentialRow(label: "Port", value: "\(credentials.port)")
                    CredentialRow(label: "Username", value: credentials.username)

                    if let password = credentials.password {
                        CredentialRow(label: "Password", value: password, sensitive: true)
                    }
                }

                // Expiration
                VStack(alignment: .leading, spacing: 8) {
                    Text("Expires")
                        .font(.headline)

                    HStack {
                        Image(systemName: credentials.isExpired ? "exclamationmark.triangle.fill" : "clock.fill")
                            .foregroundColor(credentials.isExpired ? .red : .blue)
                        Text(credentials.expiresAt, style: .relative)
                            .font(.subheadline)
                        Spacer()
                    }
                    .padding()
                    .background(Color(.systemGray6))
                    .cornerRadius(8)
                }

                // Instructions
                VStack(alignment: .leading, spacing: 8) {
                    Text("How to Connect")
                        .font(.headline)

                    Text("1. Copy the connection string above\n2. Open your terminal application\n3. Paste and execute the command\n4. Enter the password when prompted")
                        .font(.subheadline)
                        .foregroundColor(.gray)
                        .padding()
                        .background(Color(.systemGray6))
                        .cornerRadius(8)
                }
            }
            .padding()
        }
        .navigationTitle("SSH Credentials")
        .navigationBarTitleDisplayMode(.inline)
        .alert(isPresented: $showCopiedAlert) {
            Alert(title: Text("Copied!"), message: Text("Connection string copied to clipboard"), dismissButton: .default(Text("OK")))
        }
    }
}

// MARK: - Credential Row
struct CredentialRow: View {
    let label: String
    let value: String
    var sensitive: Bool = false

    @State private var isVisible = false
    @State private var showCopiedAlert = false

    var body: some View {
        VStack(alignment: .leading, spacing: 4) {
            Text(label)
                .font(.caption)
                .foregroundColor(.gray)

            HStack {
                if sensitive && !isVisible {
                    Text(String(repeating: "•", count: 12))
                        .font(.system(.subheadline, design: .monospaced))
                } else {
                    Text(value)
                        .font(.system(.subheadline, design: .monospaced))
                }

                Spacer()

                HStack(spacing: 12) {
                    if sensitive {
                        Button(action: { isVisible.toggle() }) {
                            Image(systemName: isVisible ? "eye.slash.fill" : "eye.fill")
                                .foregroundColor(.blue)
                        }
                    }

                    Button(action: {
                        UIPasteboard.general.string = value
                        showCopiedAlert = true
                    }) {
                        Image(systemName: "doc.on.doc.fill")
                            .foregroundColor(.blue)
                    }
                }
            }
        }
        .padding()
        .background(Color(.systemGray6))
        .cornerRadius(8)
        .alert(isPresented: $showCopiedAlert) {
            Alert(title: Text("Copied!"), message: Text("\(label) copied to clipboard"), dismissButton: .default(Text("OK")))
        }
    }
}
