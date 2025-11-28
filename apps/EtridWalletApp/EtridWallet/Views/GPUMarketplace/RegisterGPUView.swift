//
//  RegisterGPUView.swift
//  EtridWallet
//
//  Register GPU Provider View
//

import SwiftUI

struct RegisterGPUView: View {
    @StateObject private var service = GPUMarketplaceService()
    @Environment(\.presentationMode) var presentationMode

    // GPU Specs
    @State private var gpuModel = ""
    @State private var manufacturer = ""
    @State private var vram = 8.0
    @State private var cudaCores = ""
    @State private var tensorCores = ""
    @State private var architecture = ""

    // Rental Info
    @State private var pricePerHour = ""
    @State private var location = ""
    @State private var minRentalHours = 1.0
    @State private var maxRentalHours = 168.0

    @State private var isRegistering = false
    @State private var registrationSuccess = false
    @State private var errorMessage: String?

    var body: some View {
        NavigationView {
            Form {
                Section(header: Text("GPU Specifications")) {
                    TextField("Manufacturer (e.g., NVIDIA)", text: $manufacturer)
                    TextField("Model (e.g., RTX 4090)", text: $gpuModel)

                    VStack(alignment: .leading, spacing: 8) {
                        Text("VRAM: \(Int(vram)) GB")
                            .font(.subheadline)
                        Slider(value: $vram, in: 4...80, step: 4)
                    }

                    TextField("CUDA Cores (Optional)", text: $cudaCores)
                        .keyboardType(.numberPad)
                    TextField("Tensor Cores (Optional)", text: $tensorCores)
                        .keyboardType(.numberPad)
                    TextField("Architecture (e.g., Ada Lovelace)", text: $architecture)
                }

                Section(header: Text("Rental Information")) {
                    HStack {
                        TextField("Price per Hour", text: $pricePerHour)
                            .keyboardType(.decimalPad)
                        Text("ETR")
                            .foregroundColor(.gray)
                    }

                    TextField("Location (e.g., US-East)", text: $location)

                    VStack(alignment: .leading, spacing: 8) {
                        Text("Min Rental: \(Int(minRentalHours)) hours")
                            .font(.subheadline)
                        Slider(value: $minRentalHours, in: 1...24, step: 1)
                    }

                    VStack(alignment: .leading, spacing: 8) {
                        Text("Max Rental: \(Int(maxRentalHours)) hours")
                            .font(.subheadline)
                        Slider(value: $maxRentalHours, in: 24...720, step: 24)
                    }
                }

                Section(header: Text("Preview")) {
                    VStack(alignment: .leading, spacing: 12) {
                        Text("\(manufacturer) \(gpuModel)")
                            .font(.headline)

                        HStack {
                            VStack(alignment: .leading) {
                                Text("VRAM")
                                    .font(.caption)
                                    .foregroundColor(.gray)
                                Text("\(Int(vram)) GB")
                                    .font(.subheadline)
                                    .fontWeight(.medium)
                            }

                            Spacer()

                            VStack(alignment: .leading) {
                                Text("Price")
                                    .font(.caption)
                                    .foregroundColor(.gray)
                                if let price = Double(pricePerHour) {
                                    Text(String(format: "%.4f ETR/hr", price))
                                        .font(.subheadline)
                                        .fontWeight(.medium)
                                        .foregroundColor(.blue)
                                } else {
                                    Text("0.0000 ETR/hr")
                                        .font(.subheadline)
                                        .foregroundColor(.gray)
                                }
                            }
                        }

                        HStack {
                            VStack(alignment: .leading) {
                                Text("Location")
                                    .font(.caption)
                                    .foregroundColor(.gray)
                                Text(location.isEmpty ? "Not set" : location)
                                    .font(.subheadline)
                            }

                            Spacer()

                            VStack(alignment: .trailing) {
                                Text("Rental Range")
                                    .font(.caption)
                                    .foregroundColor(.gray)
                                Text("\(Int(minRentalHours))-\(Int(maxRentalHours))h")
                                    .font(.subheadline)
                            }
                        }
                    }
                    .padding(.vertical, 8)
                }

                if let error = errorMessage {
                    Section {
                        Text(error)
                            .font(.subheadline)
                            .foregroundColor(.red)
                    }
                }

                Section {
                    Button(action: registerGPU) {
                        if isRegistering {
                            HStack {
                                Spacer()
                                ProgressView()
                                Spacer()
                            }
                        } else {
                            Text("Register GPU")
                                .font(.headline)
                                .foregroundColor(.white)
                                .frame(maxWidth: .infinity)
                                .padding()
                                .background(canRegister ? Color.blue : Color.gray)
                                .cornerRadius(10)
                        }
                    }
                    .disabled(!canRegister || isRegistering)
                    .listRowBackground(Color.clear)
                }
            }
            .navigationTitle("Register GPU")
            .navigationBarTitleDisplayMode(.inline)
            .toolbar {
                ToolbarItem(placement: .navigationBarLeading) {
                    Button("Cancel") {
                        presentationMode.wrappedValue.dismiss()
                    }
                }
            }
            .alert(isPresented: $registrationSuccess) {
                Alert(
                    title: Text("Registration Successful!"),
                    message: Text("Your GPU has been registered and is now available for rent."),
                    dismissButton: .default(Text("OK")) {
                        presentationMode.wrappedValue.dismiss()
                    }
                )
            }
        }
    }

    private var canRegister: Bool {
        !manufacturer.isEmpty &&
        !gpuModel.isEmpty &&
        !location.isEmpty &&
        !pricePerHour.isEmpty &&
        Double(pricePerHour) != nil
    }

    private func registerGPU() {
        guard canRegister else { return }

        isRegistering = true
        errorMessage = nil

        Task {
            do {
                let specs = GPUSpecs(
                    id: UUID().uuidString,
                    model: gpuModel,
                    manufacturer: manufacturer,
                    vram: Int(vram),
                    cudaCores: Int(cudaCores),
                    tensorCores: Int(tensorCores),
                    computeCapability: nil,
                    memory_bandwidth: nil,
                    tdp: nil,
                    architecture: architecture.isEmpty ? nil : architecture
                )

                guard let price = Double(pricePerHour) else {
                    errorMessage = "Invalid price"
                    isRegistering = false
                    return
                }

                _ = try await service.registerGPU(
                    specs: specs,
                    pricePerHour: price,
                    location: location,
                    minRentalHours: Int(minRentalHours),
                    maxRentalHours: Int(maxRentalHours)
                )

                registrationSuccess = true
            } catch {
                errorMessage = error.localizedDescription
            }
            isRegistering = false
        }
    }
}

struct RegisterGPUView_Previews: PreviewProvider {
    static var previews: some View {
        RegisterGPUView()
    }
}
