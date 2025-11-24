//
//  GPUDetailView.swift
//  EtridWallet
//
//  GPU Detail View
//

import SwiftUI

struct GPUDetailView: View {
    let gpu: GPU
    @State private var showRentSheet = false
    @Environment(\.presentationMode) var presentationMode

    var body: some View {
        ScrollView {
            VStack(alignment: .leading, spacing: 20) {
                // Header
                VStack(alignment: .leading, spacing: 12) {
                    HStack {
                        Text(gpu.specs.displayName)
                            .font(.title)
                            .fontWeight(.bold)
                        Spacer()
                        HStack(spacing: 6) {
                            Circle()
                                .fill(gpu.available ? Color.green : Color.red)
                                .frame(width: 10, height: 10)
                            Text(gpu.statusText)
                                .font(.subheadline)
                                .foregroundColor(gpu.available ? .green : .red)
                        }
                    }

                    Text(gpu.location)
                        .font(.subheadline)
                        .foregroundColor(.gray)
                }
                .padding()
                .background(Color(.systemGray6))
                .cornerRadius(12)

                // Specifications
                VStack(alignment: .leading, spacing: 12) {
                    Text("Specifications")
                        .font(.title2)
                        .fontWeight(.semibold)

                    VStack(spacing: 12) {
                        SpecRow(label: "Model", value: gpu.specs.model)
                        SpecRow(label: "Manufacturer", value: gpu.specs.manufacturer)
                        SpecRow(label: "VRAM", value: "\(gpu.specs.vram) GB")

                        if let cudaCores = gpu.specs.cudaCores {
                            SpecRow(label: "CUDA Cores", value: "\(cudaCores)")
                        }

                        if let tensorCores = gpu.specs.tensorCores {
                            SpecRow(label: "Tensor Cores", value: "\(tensorCores)")
                        }

                        if let architecture = gpu.specs.architecture {
                            SpecRow(label: "Architecture", value: architecture)
                        }

                        if let bandwidth = gpu.specs.memory_bandwidth {
                            SpecRow(label: "Memory Bandwidth", value: "\(bandwidth) GB/s")
                        }

                        if let tdp = gpu.specs.tdp {
                            SpecRow(label: "TDP", value: "\(tdp)W")
                        }
                    }
                    .padding()
                    .background(Color(.systemGray6))
                    .cornerRadius(12)
                }

                // Provider Information
                if let provider = gpu.provider {
                    VStack(alignment: .leading, spacing: 12) {
                        Text("Provider")
                            .font(.title2)
                            .fontWeight(.semibold)

                        VStack(spacing: 12) {
                            HStack {
                                Image(systemName: "person.circle.fill")
                                    .font(.system(size: 40))
                                    .foregroundColor(.blue)

                                VStack(alignment: .leading, spacing: 4) {
                                    Text(provider.name)
                                        .font(.headline)

                                    HStack {
                                        Image(systemName: "star.fill")
                                            .foregroundColor(.yellow)
                                            .font(.caption)
                                        Text(provider.ratingDisplay)
                                            .font(.subheadline)

                                        Text("•")
                                            .foregroundColor(.gray)

                                        Text(provider.reputationBadge)
                                            .font(.caption)
                                            .padding(.horizontal, 8)
                                            .padding(.vertical, 4)
                                            .background(Color.orange.opacity(0.2))
                                            .cornerRadius(4)
                                    }
                                }

                                Spacer()
                            }

                            Divider()

                            HStack {
                                VStack(alignment: .leading) {
                                    Text("Total Rentals")
                                        .font(.caption)
                                        .foregroundColor(.gray)
                                    Text("\(provider.totalRentals)")
                                        .font(.headline)
                                }
                                Spacer()
                                VStack(alignment: .leading) {
                                    Text("Verified")
                                        .font(.caption)
                                        .foregroundColor(.gray)
                                    HStack {
                                        Image(systemName: provider.verified ? "checkmark.seal.fill" : "xmark.seal")
                                            .foregroundColor(provider.verified ? .green : .gray)
                                        Text(provider.verified ? "Yes" : "No")
                                            .font(.headline)
                                    }
                                }
                            }
                        }
                        .padding()
                        .background(Color(.systemGray6))
                        .cornerRadius(12)
                    }
                }

                // Rental Information
                VStack(alignment: .leading, spacing: 12) {
                    Text("Rental Information")
                        .font(.title2)
                        .fontWeight(.semibold)

                    VStack(spacing: 12) {
                        HStack {
                            VStack(alignment: .leading, spacing: 4) {
                                Text("Price per Hour")
                                    .font(.caption)
                                    .foregroundColor(.gray)
                                Text(gpu.priceDisplay)
                                    .font(.title3)
                                    .fontWeight(.bold)
                                    .foregroundColor(.blue)
                            }
                            Spacer()
                        }

                        Divider()

                        HStack {
                            VStack(alignment: .leading) {
                                Text("Min Rental")
                                    .font(.caption)
                                    .foregroundColor(.gray)
                                Text("\(gpu.minRentalHours)h")
                                    .font(.subheadline)
                                    .fontWeight(.medium)
                            }
                            Spacer()
                            VStack(alignment: .trailing) {
                                Text("Max Rental")
                                    .font(.caption)
                                    .foregroundColor(.gray)
                                Text("\(gpu.maxRentalHours)h")
                                    .font(.subheadline)
                                    .fontWeight(.medium)
                            }
                        }

                        Divider()

                        HStack {
                            VStack(alignment: .leading) {
                                Text("Uptime")
                                    .font(.caption)
                                    .foregroundColor(.gray)
                                Text(gpu.uptimeDisplay)
                                    .font(.subheadline)
                                    .fontWeight(.medium)
                            }
                            Spacer()
                            VStack(alignment: .trailing) {
                                Text("Last Online")
                                    .font(.caption)
                                    .foregroundColor(.gray)
                                Text(gpu.lastOnline, style: .relative)
                                    .font(.subheadline)
                                    .fontWeight(.medium)
                            }
                        }
                    }
                    .padding()
                    .background(Color(.systemGray6))
                    .cornerRadius(12)
                }

                // Rent Button
                if gpu.available {
                    Button(action: {
                        showRentSheet = true
                    }) {
                        HStack {
                            Image(systemName: "cpu")
                            Text("Rent This GPU")
                        }
                        .font(.headline)
                        .foregroundColor(.white)
                        .frame(maxWidth: .infinity)
                        .padding()
                        .background(Color.blue)
                        .cornerRadius(12)
                    }
                } else {
                    HStack {
                        Image(systemName: "lock.fill")
                        Text("Currently Unavailable")
                    }
                    .font(.headline)
                    .foregroundColor(.white)
                    .frame(maxWidth: .infinity)
                    .padding()
                    .background(Color.gray)
                    .cornerRadius(12)
                }
            }
            .padding()
        }
        .navigationTitle("GPU Details")
        .navigationBarTitleDisplayMode(.inline)
        .sheet(isPresented: $showRentSheet) {
            RentGPUView(gpu: gpu)
        }
    }
}

// MARK: - Spec Row
struct SpecRow: View {
    let label: String
    let value: String

    var body: some View {
        HStack {
            Text(label)
                .font(.subheadline)
                .foregroundColor(.gray)
            Spacer()
            Text(value)
                .font(.subheadline)
                .fontWeight(.medium)
        }
    }
}
