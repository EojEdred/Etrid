//
//  NetworkService.swift
//  EtridWallet
//
//  Core networking service for API communication
//

import Foundation
import Combine

enum NetworkError: Error {
    case invalidURL
    case noData
    case decodingError
    case serverError(String)
    case unauthorized
    case networkFailure(Error)

    var localizedDescription: String {
        switch self {
        case .invalidURL:
            return "Invalid URL"
        case .noData:
            return "No data received from server"
        case .decodingError:
            return "Failed to decode response"
        case .serverError(let message):
            return "Server error: \(message)"
        case .unauthorized:
            return "Unauthorized access"
        case .networkFailure(let error):
            return "Network error: \(error.localizedDescription)"
        }
    }
}

class NetworkService {
    static let shared = NetworkService()

    private let baseURL: String
    private let session: URLSession

    private init() {
        // For Simulator: use 127.0.0.1 (not localhost), for device: use production server
        #if targetEnvironment(simulator)
        self.baseURL = "http://127.0.0.1:3001/api/v1"
        #else
        self.baseURL = "http://100.70.242.106/api/v1"
        #endif

        let configuration = URLSessionConfiguration.default
        configuration.timeoutIntervalForRequest = 30
        configuration.timeoutIntervalForResource = 300
        self.session = URLSession(configuration: configuration)
    }

    // MARK: - Generic Request
    func request<T: Decodable>(
        endpoint: String,
        method: HTTPMethod = .get,
        body: Encodable? = nil,
        headers: [String: String]? = nil
    ) async throws -> T {
        guard let url = URL(string: "\(baseURL)\(endpoint)") else {
            throw NetworkError.invalidURL
        }

        var request = URLRequest(url: url)
        request.httpMethod = method.rawValue

        // Add headers
        request.setValue("application/json", forHTTPHeaderField: "Content-Type")
        headers?.forEach { request.setValue($1, forHTTPHeaderField: $0) }

        // Add authentication token if available
        if let token = KeychainService.shared.getAuthToken() {
            request.setValue("Bearer \(token)", forHTTPHeaderField: "Authorization")
        }

        // Add body
        if let body = body {
            request.httpBody = try JSONEncoder().encode(body)
        }

        do {
            let (data, response) = try await session.data(for: request)

            guard let httpResponse = response as? HTTPURLResponse else {
                throw NetworkError.serverError("Invalid response")
            }

            switch httpResponse.statusCode {
            case 200...299:
                do {
                    let decoded = try JSONDecoder().decode(T.self, from: data)
                    return decoded
                } catch {
                    print("Decoding error: \(error)")
                    throw NetworkError.decodingError
                }
            case 401:
                throw NetworkError.unauthorized
            default:
                let errorMessage = String(data: data, encoding: .utf8) ?? "Unknown error"
                throw NetworkError.serverError(errorMessage)
            }
        } catch let error as NetworkError {
            throw error
        } catch {
            throw NetworkError.networkFailure(error)
        }
    }

    // MARK: - Upload
    func upload<T: Decodable>(
        endpoint: String,
        data: Data,
        fileName: String,
        mimeType: String
    ) async throws -> T {
        guard let url = URL(string: "\(baseURL)\(endpoint)") else {
            throw NetworkError.invalidURL
        }

        var request = URLRequest(url: url)
        request.httpMethod = "POST"

        let boundary = UUID().uuidString
        request.setValue("multipart/form-data; boundary=\(boundary)", forHTTPHeaderField: "Content-Type")

        if let token = KeychainService.shared.getAuthToken() {
            request.setValue("Bearer \(token)", forHTTPHeaderField: "Authorization")
        }

        var body = Data()
        body.append("--\(boundary)\r\n".data(using: .utf8)!)
        body.append("Content-Disposition: form-data; name=\"file\"; filename=\"\(fileName)\"\r\n".data(using: .utf8)!)
        body.append("Content-Type: \(mimeType)\r\n\r\n".data(using: .utf8)!)
        body.append(data)
        body.append("\r\n--\(boundary)--\r\n".data(using: .utf8)!)

        request.httpBody = body

        let (responseData, response) = try await session.data(for: request)

        guard let httpResponse = response as? HTTPURLResponse,
              httpResponse.statusCode == 200 else {
            throw NetworkError.serverError("Upload failed")
        }

        return try JSONDecoder().decode(T.self, from: responseData)
    }
}

enum HTTPMethod: String {
    case get = "GET"
    case post = "POST"
    case put = "PUT"
    case delete = "DELETE"
    case patch = "PATCH"
}
