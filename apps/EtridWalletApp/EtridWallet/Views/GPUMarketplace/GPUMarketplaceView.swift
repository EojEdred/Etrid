//
//  GPUMarketplaceView.swift
//  EtridWallet
//
//  GPU Marketplace Main View
//

import SwiftUI

struct GPUMarketplaceView: View {
    @StateObject private var service = GPUMarketplaceService()
    @State private var gpus: [GPU] = []
    @State private var filter = GPUFilter()
    @State private var sortBy = "price"
    @State private var sortOrder = "asc"
    @State private var currentPage = 1
    @State private var showFilterSheet = false
    @State private var selectedGPU: GPU?
    @State private var searchText = ""

    var body: some View {
        NavigationView {
            VStack(spacing: 0) {
                // Search bar
                HStack {
                    Image(systemName: "magnifyingglass")
                        .foregroundColor(.gray)
                    TextField("Search GPUs...", text: $searchText)
                        .textFieldStyle(RoundedBorderTextFieldStyle())
                }
                .padding()

                // Filter and Sort bar
                HStack {
                    Button(action: { showFilterSheet = true }) {
                        HStack {
                            Image(systemName: "slider.horizontal.3")
                            Text("Filters")
                        }
                        .padding(.horizontal, 12)
                        .padding(.vertical, 8)
                        .background(Color.blue.opacity(0.1))
                        .cornerRadius(8)
                    }

                    Spacer()

                    Menu {
                        Button("Price: Low to High") { sortBy = "price"; sortOrder = "asc"; loadGPUs() }
                        Button("Price: High to Low") { sortBy = "price"; sortOrder = "desc"; loadGPUs() }
                        Button("VRAM: High to Low") { sortBy = "vram"; sortOrder = "desc"; loadGPUs() }
                        Button("Rating: High to Low") { sortBy = "rating"; sortOrder = "desc"; loadGPUs() }
                        Button("Uptime: High to Low") { sortBy = "uptime"; sortOrder = "desc"; loadGPUs() }
                    } label: {
                        HStack {
                            Image(systemName: "arrow.up.arrow.down")
                            Text("Sort")
                        }
                        .padding(.horizontal, 12)
                        .padding(.vertical, 8)
                        .background(Color.blue.opacity(0.1))
                        .cornerRadius(8)
                    }
                }
                .padding(.horizontal)

                // GPU List
                if service.isLoading {
                    Spacer()
                    ProgressView("Loading GPUs...")
                    Spacer()
                } else if gpus.isEmpty {
                    Spacer()
                    VStack(spacing: 16) {
                        Image(systemName: "cpu")
                            .font(.system(size: 60))
                            .foregroundColor(.gray)
                        Text("No GPUs Found")
                            .font(.title2)
                            .foregroundColor(.gray)
                        Text("Try adjusting your filters")
                            .font(.subheadline)
                            .foregroundColor(.gray)
                    }
                    Spacer()
                } else {
                    ScrollView {
                        LazyVStack(spacing: 12) {
                            ForEach(gpus) { gpu in
                                GPUCardView(gpu: gpu)
                                    .onTapGesture {
                                        selectedGPU = gpu
                                    }
                            }
                        }
                        .padding()
                    }
                }
            }
            .navigationTitle("GPU Marketplace")
            .navigationBarTitleDisplayMode(.large)
            .toolbar {
                ToolbarItem(placement: .navigationBarTrailing) {
                    NavigationLink(destination: MyRentalsView()) {
                        Image(systemName: "list.bullet.rectangle")
                    }
                }
            }
            .sheet(isPresented: $showFilterSheet) {
                GPUFilterSheet(filter: $filter, onApply: loadGPUs)
            }
            .sheet(item: $selectedGPU) { gpu in
                GPUDetailView(gpu: gpu)
            }
            .onAppear {
                loadGPUs()
            }
        }
    }

    private func loadGPUs() {
        Task {
            do {
                let response = try await service.searchGPUs(filter: filter, sortBy: sortBy, sortOrder: sortOrder, page: currentPage)
                gpus = response.gpus
            } catch {
                print("Error loading GPUs: \(error)")
            }
        }
    }
}

// MARK: - GPU Card View
struct GPUCardView: View {
    let gpu: GPU

    var body: some View {
        VStack(alignment: .leading, spacing: 12) {
            // Header with availability
            HStack {
                Text(gpu.specs.displayName)
                    .font(.headline)
                Spacer()
                HStack(spacing: 4) {
                    Circle()
                        .fill(gpu.available ? Color.green : Color.red)
                        .frame(width: 8, height: 8)
                    Text(gpu.statusText)
                        .font(.caption)
                        .foregroundColor(gpu.available ? .green : .red)
                }
            }

            // Specs
            HStack(spacing: 16) {
                VStack(alignment: .leading, spacing: 4) {
                    Text("VRAM")
                        .font(.caption)
                        .foregroundColor(.gray)
                    Text(gpu.specs.vramDisplay)
                        .font(.subheadline)
                        .fontWeight(.medium)
                }

                Divider()
                    .frame(height: 30)

                VStack(alignment: .leading, spacing: 4) {
                    Text("Location")
                        .font(.caption)
                        .foregroundColor(.gray)
                    Text(gpu.location)
                        .font(.subheadline)
                        .fontWeight(.medium)
                }

                Divider()
                    .frame(height: 30)

                VStack(alignment: .leading, spacing: 4) {
                    Text("Uptime")
                        .font(.caption)
                        .foregroundColor(.gray)
                    Text(gpu.uptimeDisplay)
                        .font(.subheadline)
                        .fontWeight(.medium)
                }
            }

            // Provider info
            if let provider = gpu.provider {
                HStack {
                    Image(systemName: "person.circle.fill")
                        .foregroundColor(.blue)
                    Text(provider.name)
                        .font(.subheadline)

                    Text(provider.reputationBadge)
                        .font(.caption)
                        .padding(.horizontal, 8)
                        .padding(.vertical, 4)
                        .background(Color.orange.opacity(0.2))
                        .cornerRadius(4)

                    Spacer()

                    Image(systemName: "star.fill")
                        .foregroundColor(.yellow)
                        .font(.caption)
                    Text(provider.ratingDisplay)
                        .font(.subheadline)
                }
            }

            Divider()

            // Price
            HStack {
                Text(gpu.priceDisplay)
                    .font(.title3)
                    .fontWeight(.bold)
                    .foregroundColor(.blue)
                Spacer()
                NavigationLink(destination: GPUDetailView(gpu: gpu)) {
                    Text("View Details")
                        .font(.subheadline)
                        .fontWeight(.medium)
                        .foregroundColor(.white)
                        .padding(.horizontal, 16)
                        .padding(.vertical, 8)
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

// MARK: - Filter Sheet
struct GPUFilterSheet: View {
    @Binding var filter: GPUFilter
    var onApply: () -> Void
    @Environment(\.presentationMode) var presentationMode

    @State private var minVRAM: Double = 0
    @State private var maxVRAM: Double = 80
    @State private var minPrice: Double = 0
    @State private var maxPrice: Double = 10
    @State private var availableOnly: Bool = true

    var body: some View {
        NavigationView {
            Form {
                Section(header: Text("VRAM (GB)")) {
                    HStack {
                        Text("\(Int(minVRAM)) GB")
                        Slider(value: $minVRAM, in: 0...80, step: 4)
                        Text("\(Int(maxVRAM)) GB")
                    }
                    Slider(value: $maxVRAM, in: 0...80, step: 4)
                }

                Section(header: Text("Price (ETR/hr)")) {
                    HStack {
                        Text(String(format: "%.2f", minPrice))
                        Slider(value: $minPrice, in: 0...10, step: 0.1)
                        Text(String(format: "%.2f", maxPrice))
                    }
                    Slider(value: $maxPrice, in: 0...10, step: 0.1)
                }

                Section(header: Text("Availability")) {
                    Toggle("Available Only", isOn: $availableOnly)
                }

                Section {
                    Button(action: {
                        filter.minVRAM = Int(minVRAM)
                        filter.maxVRAM = Int(maxVRAM)
                        filter.minPrice = minPrice
                        filter.maxPrice = maxPrice
                        filter.availableOnly = availableOnly
                        onApply()
                        presentationMode.wrappedValue.dismiss()
                    }) {
                        Text("Apply Filters")
                            .frame(maxWidth: .infinity)
                            .foregroundColor(.white)
                            .padding()
                            .background(Color.blue)
                            .cornerRadius(10)
                    }
                    .listRowBackground(Color.clear)

                    Button(action: {
                        minVRAM = 0
                        maxVRAM = 80
                        minPrice = 0
                        maxPrice = 10
                        availableOnly = true
                        filter = GPUFilter()
                    }) {
                        Text("Reset Filters")
                            .frame(maxWidth: .infinity)
                            .foregroundColor(.red)
                    }
                    .listRowBackground(Color.clear)
                }
            }
            .navigationTitle("Filter GPUs")
            .navigationBarTitleDisplayMode(.inline)
            .toolbar {
                ToolbarItem(placement: .navigationBarTrailing) {
                    Button("Close") {
                        presentationMode.wrappedValue.dismiss()
                    }
                }
            }
        }
    }
}

struct GPUMarketplaceView_Previews: PreviewProvider {
    static var previews: some View {
        GPUMarketplaceView()
    }
}
