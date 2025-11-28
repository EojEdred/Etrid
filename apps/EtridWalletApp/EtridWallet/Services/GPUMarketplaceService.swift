//
//  GPUMarketplaceService.swift
//  EtridWallet
//
//  GPU Marketplace Service
//

import Foundation

class GPUMarketplaceService: ObservableObject {
    @Published var isLoading = false
    @Published var error: String?

    private let baseURL = "https://api.etrid.org/v1/gpu" // Replace with actual API

    // MARK: - Search GPUs
    func searchGPUs(filter: GPUFilter = GPUFilter(), sortBy: String = "price", sortOrder: String = "asc", page: Int = 1, limit: Int = 20) async throws -> GPUSearchResponse {
        isLoading = true
        defer { isLoading = false }

        let request = GPUSearchRequest(filter: filter, sortBy: sortBy, sortOrder: sortOrder, page: page, limit: limit)

        var urlComponents = URLComponents(string: "\(baseURL)/search")!
        var queryItems: [URLQueryItem] = []

        // Add filter parameters
        if let minVRAM = filter.minVRAM {
            queryItems.append(URLQueryItem(name: "minVRAM", value: "\(minVRAM)"))
        }
        if let maxVRAM = filter.maxVRAM {
            queryItems.append(URLQueryItem(name: "maxVRAM", value: "\(maxVRAM)"))
        }
        if let minPrice = filter.minPrice {
            queryItems.append(URLQueryItem(name: "minPrice", value: "\(minPrice)"))
        }
        if let maxPrice = filter.maxPrice {
            queryItems.append(URLQueryItem(name: "maxPrice", value: "\(maxPrice)"))
        }
        if filter.availableOnly {
            queryItems.append(URLQueryItem(name: "available", value: "true"))
        }
        if let minRating = filter.minRating {
            queryItems.append(URLQueryItem(name: "minRating", value: "\(minRating)"))
        }

        // Add sorting and pagination
        queryItems.append(URLQueryItem(name: "sortBy", value: sortBy))
        queryItems.append(URLQueryItem(name: "sortOrder", value: sortOrder))
        queryItems.append(URLQueryItem(name: "page", value: "\(page)"))
        queryItems.append(URLQueryItem(name: "limit", value: "\(limit)"))

        urlComponents.queryItems = queryItems

        guard let url = urlComponents.url else {
            throw GPUServiceError.invalidURL
        }

        var urlRequest = URLRequest(url: url)
        urlRequest.httpMethod = "GET"
        urlRequest.setValue("application/json", forHTTPHeaderField: "Content-Type")

        let (data, response) = try await URLSession.shared.data(for: urlRequest)

        guard let httpResponse = response as? HTTPURLResponse else {
            throw GPUServiceError.invalidResponse
        }

        guard (200...299).contains(httpResponse.statusCode) else {
            throw GPUServiceError.httpError(statusCode: httpResponse.statusCode)
        }

        let decoder = JSONDecoder()
        decoder.dateDecodingStrategy = .iso8601
        let searchResponse = try decoder.decode(GPUSearchResponse.self, from: data)

        return searchResponse
    }

    // MARK: - Rent GPU
    func rentGPU(gpuId: String, durationHours: Int, paymentMethod: String = "wallet") async throws -> RentGPUResponse {
        isLoading = true
        defer { isLoading = false }

        let request = RentGPURequest(gpuId: gpuId, durationHours: durationHours, paymentMethod: paymentMethod)

        guard let url = URL(string: "\(baseURL)/rent") else {
            throw GPUServiceError.invalidURL
        }

        var urlRequest = URLRequest(url: url)
        urlRequest.httpMethod = "POST"
        urlRequest.setValue("application/json", forHTTPHeaderField: "Content-Type")

        let encoder = JSONEncoder()
        encoder.dateEncodingStrategy = .iso8601
        urlRequest.httpBody = try encoder.encode(request)

        let (data, response) = try await URLSession.shared.data(for: urlRequest)

        guard let httpResponse = response as? HTTPURLResponse else {
            throw GPUServiceError.invalidResponse
        }

        guard (200...299).contains(httpResponse.statusCode) else {
            let errorMessage = String(data: data, encoding: .utf8) ?? "Unknown error"
            throw GPUServiceError.rentalFailed(message: errorMessage)
        }

        let decoder = JSONDecoder()
        decoder.dateDecodingStrategy = .iso8601
        let rentResponse = try decoder.decode(RentGPUResponse.self, from: data)

        return rentResponse
    }

    // MARK: - Register GPU
    func registerGPU(specs: GPUSpecs, pricePerHour: Double, location: String, minRentalHours: Int = 1, maxRentalHours: Int = 720) async throws -> RegisterGPUResponse {
        isLoading = true
        defer { isLoading = false }

        let request = RegisterGPURequest(specs: specs, pricePerHour: pricePerHour, location: location, minRentalHours: minRentalHours, maxRentalHours: maxRentalHours)

        guard let url = URL(string: "\(baseURL)/register") else {
            throw GPUServiceError.invalidURL
        }

        var urlRequest = URLRequest(url: url)
        urlRequest.httpMethod = "POST"
        urlRequest.setValue("application/json", forHTTPHeaderField: "Content-Type")

        let encoder = JSONEncoder()
        urlRequest.httpBody = try encoder.encode(request)

        let (data, response) = try await URLSession.shared.data(for: urlRequest)

        guard let httpResponse = response as? HTTPURLResponse else {
            throw GPUServiceError.invalidResponse
        }

        guard (200...299).contains(httpResponse.statusCode) else {
            let errorMessage = String(data: data, encoding: .utf8) ?? "Unknown error"
            throw GPUServiceError.registrationFailed(message: errorMessage)
        }

        let decoder = JSONDecoder()
        let registerResponse = try decoder.decode(RegisterGPUResponse.self, from: data)

        return registerResponse
    }

    // MARK: - Get My Rentals
    func getMyRentals(address: String, status: RentalStatus? = nil, page: Int = 1, limit: Int = 20) async throws -> [Rental] {
        isLoading = true
        defer { isLoading = false }

        var urlComponents = URLComponents(string: "\(baseURL)/rentals")!
        var queryItems: [URLQueryItem] = [
            URLQueryItem(name: "address", value: address),
            URLQueryItem(name: "page", value: "\(page)"),
            URLQueryItem(name: "limit", value: "\(limit)")
        ]

        if let status = status {
            queryItems.append(URLQueryItem(name: "status", value: status.rawValue))
        }

        urlComponents.queryItems = queryItems

        guard let url = urlComponents.url else {
            throw GPUServiceError.invalidURL
        }

        var urlRequest = URLRequest(url: url)
        urlRequest.httpMethod = "GET"
        urlRequest.setValue("application/json", forHTTPHeaderField: "Content-Type")

        let (data, response) = try await URLSession.shared.data(for: urlRequest)

        guard let httpResponse = response as? HTTPURLResponse else {
            throw GPUServiceError.invalidResponse
        }

        guard (200...299).contains(httpResponse.statusCode) else {
            throw GPUServiceError.httpError(statusCode: httpResponse.statusCode)
        }

        let decoder = JSONDecoder()
        decoder.dateDecodingStrategy = .iso8601
        let rentals = try decoder.decode([Rental].self, from: data)

        return rentals
    }

    // MARK: - Generate SSH Credentials
    func generateSSHCredentials(rentalId: String) async throws -> SSHCredentials {
        isLoading = true
        defer { isLoading = false }

        guard let url = URL(string: "\(baseURL)/rentals/\(rentalId)/ssh") else {
            throw GPUServiceError.invalidURL
        }

        var urlRequest = URLRequest(url: url)
        urlRequest.httpMethod = "POST"
        urlRequest.setValue("application/json", forHTTPHeaderField: "Content-Type")

        let (data, response) = try await URLSession.shared.data(for: urlRequest)

        guard let httpResponse = response as? HTTPURLResponse else {
            throw GPUServiceError.invalidResponse
        }

        guard (200...299).contains(httpResponse.statusCode) else {
            throw GPUServiceError.httpError(statusCode: httpResponse.statusCode)
        }

        let decoder = JSONDecoder()
        decoder.dateDecodingStrategy = .iso8601
        let credentials = try decoder.decode(SSHCredentials.self, from: data)

        return credentials
    }

    // MARK: - Get Usage Metrics
    func getUsageMetrics(rentalId: String, from: Date? = nil, to: Date? = nil) async throws -> [UsageMetrics] {
        isLoading = true
        defer { isLoading = false }

        var urlComponents = URLComponents(string: "\(baseURL)/rentals/\(rentalId)/metrics")!
        var queryItems: [URLQueryItem] = []

        if let fromDate = from {
            let timestamp = Int(fromDate.timeIntervalSince1970)
            queryItems.append(URLQueryItem(name: "from", value: "\(timestamp)"))
        }

        if let toDate = to {
            let timestamp = Int(toDate.timeIntervalSince1970)
            queryItems.append(URLQueryItem(name: "to", value: "\(timestamp)"))
        }

        urlComponents.queryItems = queryItems

        guard let url = urlComponents.url else {
            throw GPUServiceError.invalidURL
        }

        var urlRequest = URLRequest(url: url)
        urlRequest.httpMethod = "GET"
        urlRequest.setValue("application/json", forHTTPHeaderField: "Content-Type")

        let (data, response) = try await URLSession.shared.data(for: urlRequest)

        guard let httpResponse = response as? HTTPURLResponse else {
            throw GPUServiceError.invalidResponse
        }

        guard (200...299).contains(httpResponse.statusCode) else {
            throw GPUServiceError.httpError(statusCode: httpResponse.statusCode)
        }

        let decoder = JSONDecoder()
        decoder.dateDecodingStrategy = .iso8601
        let metrics = try decoder.decode([UsageMetrics].self, from: data)

        return metrics
    }

    // MARK: - Get GPU Details
    func getGPUDetails(gpuId: String) async throws -> GPU {
        isLoading = true
        defer { isLoading = false }

        guard let url = URL(string: "\(baseURL)/gpus/\(gpuId)") else {
            throw GPUServiceError.invalidURL
        }

        var urlRequest = URLRequest(url: url)
        urlRequest.httpMethod = "GET"
        urlRequest.setValue("application/json", forHTTPHeaderField: "Content-Type")

        let (data, response) = try await URLSession.shared.data(for: urlRequest)

        guard let httpResponse = response as? HTTPURLResponse else {
            throw GPUServiceError.invalidResponse
        }

        guard (200...299).contains(httpResponse.statusCode) else {
            throw GPUServiceError.httpError(statusCode: httpResponse.statusCode)
        }

        let decoder = JSONDecoder()
        decoder.dateDecodingStrategy = .iso8601
        let gpu = try decoder.decode(GPU.self, from: data)

        return gpu
    }
}

// MARK: - GPU Service Errors
enum GPUServiceError: LocalizedError {
    case invalidURL
    case invalidResponse
    case httpError(statusCode: Int)
    case rentalFailed(message: String)
    case registrationFailed(message: String)
    case networkError(Error)

    var errorDescription: String? {
        switch self {
        case .invalidURL:
            return "Invalid URL"
        case .invalidResponse:
            return "Invalid response from server"
        case .httpError(let statusCode):
            return "HTTP error: \(statusCode)"
        case .rentalFailed(let message):
            return "Rental failed: \(message)"
        case .registrationFailed(let message):
            return "Registration failed: \(message)"
        case .networkError(let error):
            return "Network error: \(error.localizedDescription)"
        }
    }
}
