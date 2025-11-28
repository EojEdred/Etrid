//
//  RentGPUView.swift
//  EtridWallet
//
//  Rent GPU View
//

import SwiftUI

struct RentGPUView: View {
    let gpu: GPU
    @StateObject private var service = GPUMarketplaceService()
    @Environment(\.presentationMode) var presentationMode

    @State private var durationHours: Double = 1
    @State private var paymentMethod = "wallet"
    @State private var isRenting = false
    @State private var rentalSuccess = false
    @State private var rental: Rental?
    @State private var errorMessage: String?

    var totalCost: Double {
        gpu.pricePerHour * durationHours
    }

    var body: some View {
        NavigationView {
            ScrollView {
                VStack(spacing: 24) {
                    // GPU Info
                    VStack(alignment: .leading, spacing: 12) {
                        Text("Renting")
                            .font(.headline)
                            .foregroundColor(.gray)

                        HStack {
                            Image(systemName: "cpu")
                                .font(.system(size: 40))
                                .foregroundColor(.blue)

                            VStack(alignment: .leading, spacing: 4) {
                                Text(gpu.specs.displayName)
                                    .font(.headline)
                                Text(gpu.location)
                                    .font(.subheadline)
                                    .foregroundColor(.gray)
                            }

                            Spacer()
                        }
                        .padding()
                        .background(Color(.systemGray6))
                        .cornerRadius(12)
                    }

                    // Duration Selector
                    VStack(alignment: .leading, spacing: 12) {
                        Text("Rental Duration")
                            .font(.headline)

                        VStack(spacing: 8) {
                            HStack {
                                Text("\(Int(durationHours)) hours")
                                    .font(.title2)
                                    .fontWeight(.bold)
                                    .foregroundColor(.blue)
                                Spacer()
                            }

                            Slider(value: $durationHours, in: Double(gpu.minRentalHours)...Double(gpu.maxRentalHours), step: 1)

                            HStack {
                                Text("Min: \(gpu.minRentalHours)h")
                                    .font(.caption)
                                    .foregroundColor(.gray)
                                Spacer()
                                Text("Max: \(gpu.maxRentalHours)h")
                                    .font(.caption)
                                    .foregroundColor(.gray)
                            }
                        }
                        .padding()
                        .background(Color(.systemGray6))
                        .cornerRadius(12)
                    }

                    // Quick Duration Buttons
                    VStack(alignment: .leading, spacing: 12) {
                        Text("Quick Select")
                            .font(.headline)

                        HStack(spacing: 12) {
                            QuickDurationButton(title: "1h", hours: 1, selectedHours: $durationHours)
                            QuickDurationButton(title: "6h", hours: 6, selectedHours: $durationHours)
                            QuickDurationButton(title: "24h", hours: 24, selectedHours: $durationHours)
                            QuickDurationButton(title: "7d", hours: 168, selectedHours: $durationHours)
                        }
                    }

                    // Payment Method
                    VStack(alignment: .leading, spacing: 12) {
                        Text("Payment Method")
                            .font(.headline)

                        VStack(spacing: 0) {
                            PaymentMethodRow(
                                icon: "wallet.pass.fill",
                                title: "Wallet Balance",
                                subtitle: "Pay from your ETR balance",
                                method: "wallet",
                                selectedMethod: $paymentMethod
                            )

                            Divider()

                            PaymentMethodRow(
                                icon: "lock.fill",
                                title: "Staked Tokens",
                                subtitle: "Use your staked ETR",
                                method: "staked",
                                selectedMethod: $paymentMethod
                            )
                        }
                        .background(Color(.systemGray6))
                        .cornerRadius(12)
                    }

                    // Cost Summary
                    VStack(spacing: 12) {
                        HStack {
                            Text("Price per Hour")
                                .font(.subheadline)
                            Spacer()
                            Text(gpu.priceDisplay)
                                .font(.subheadline)
                                .fontWeight(.medium)
                        }

                        HStack {
                            Text("Duration")
                                .font(.subheadline)
                            Spacer()
                            Text("\(Int(durationHours)) hours")
                                .font(.subheadline)
                                .fontWeight(.medium)
                        }

                        Divider()

                        HStack {
                            Text("Total Cost")
                                .font(.headline)
                            Spacer()
                            Text(String(format: "%.4f ETR", totalCost))
                                .font(.title3)
                                .fontWeight(.bold)
                                .foregroundColor(.blue)
                        }
                    }
                    .padding()
                    .background(Color(.systemGray6))
                    .cornerRadius(12)

                    // Error Message
                    if let error = errorMessage {
                        Text(error)
                            .font(.subheadline)
                            .foregroundColor(.red)
                            .padding()
                            .background(Color.red.opacity(0.1))
                            .cornerRadius(8)
                    }

                    // Rent Button
                    Button(action: rentGPU) {
                        if isRenting {
                            ProgressView()
                                .progressViewStyle(CircularProgressViewStyle(tint: .white))
                        } else {
                            HStack {
                                Image(systemName: "checkmark.circle.fill")
                                Text("Confirm Rental")
                            }
                            .font(.headline)
                        }
                    }
                    .foregroundColor(.white)
                    .frame(maxWidth: .infinity)
                    .padding()
                    .background(isRenting ? Color.gray : Color.blue)
                    .cornerRadius(12)
                    .disabled(isRenting)
                }
                .padding()
            }
            .navigationTitle("Rent GPU")
            .navigationBarTitleDisplayMode(.inline)
            .toolbar {
                ToolbarItem(placement: .navigationBarLeading) {
                    Button("Cancel") {
                        presentationMode.wrappedValue.dismiss()
                    }
                }
            }
            .alert(isPresented: $rentalSuccess) {
                Alert(
                    title: Text("Rental Successful!"),
                    message: Text("Your GPU has been rented successfully. You can view the SSH credentials in My Rentals."),
                    dismissButton: .default(Text("OK")) {
                        presentationMode.wrappedValue.dismiss()
                    }
                )
            }
        }
    }

    private func rentGPU() {
        isRenting = true
        errorMessage = nil

        Task {
            do {
                let response = try await service.rentGPU(gpuId: gpu.id, durationHours: Int(durationHours), paymentMethod: paymentMethod)
                rental = response.rental
                rentalSuccess = true
            } catch {
                errorMessage = error.localizedDescription
            }
            isRenting = false
        }
    }
}

// MARK: - Quick Duration Button
struct QuickDurationButton: View {
    let title: String
    let hours: Double
    @Binding var selectedHours: Double

    var body: some View {
        Button(action: {
            selectedHours = hours
        }) {
            Text(title)
                .font(.subheadline)
                .fontWeight(.medium)
                .foregroundColor(selectedHours == hours ? .white : .blue)
                .frame(maxWidth: .infinity)
                .padding(.vertical, 10)
                .background(selectedHours == hours ? Color.blue : Color.blue.opacity(0.1))
                .cornerRadius(8)
        }
    }
}

// MARK: - Payment Method Row
struct PaymentMethodRow: View {
    let icon: String
    let title: String
    let subtitle: String
    let method: String
    @Binding var selectedMethod: String

    var body: some View {
        Button(action: {
            selectedMethod = method
        }) {
            HStack(spacing: 12) {
                Image(systemName: icon)
                    .font(.title2)
                    .foregroundColor(.blue)
                    .frame(width: 40)

                VStack(alignment: .leading, spacing: 4) {
                    Text(title)
                        .font(.subheadline)
                        .fontWeight(.medium)
                        .foregroundColor(.primary)
                    Text(subtitle)
                        .font(.caption)
                        .foregroundColor(.gray)
                }

                Spacer()

                if selectedMethod == method {
                    Image(systemName: "checkmark.circle.fill")
                        .foregroundColor(.blue)
                } else {
                    Image(systemName: "circle")
                        .foregroundColor(.gray)
                }
            }
            .padding()
        }
    }
}
