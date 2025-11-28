//
//  GPU.swift
//  EtridWallet
//
//  GPU Marketplace Models
//

import Foundation

// MARK: - Reputation Tier
enum ReputationTier: String, Codable, CaseIterable {
    case bronze = "Bronze"
    case silver = "Silver"
    case gold = "Gold"
    case platinum = "Platinum"

    var color: String {
        switch self {
        case .bronze: return "#CD7F32"
        case .silver: return "#C0C0C0"
        case .gold: return "#FFD700"
        case .platinum: return "#E5E4E2"
        }
    }

    var minRating: Double {
        switch self {
        case .bronze: return 0.0
        case .silver: return 3.5
        case .gold: return 4.0
        case .platinum: return 4.5
        }
    }
}

// MARK: - GPU Specs
struct GPUSpecs: Codable, Identifiable {
    let id: String
    let model: String
    let manufacturer: String
    let vram: Int // in GB
    let cudaCores: Int?
    let tensorCores: Int?
    let computeCapability: String?
    let memory_bandwidth: Int? // GB/s
    let tdp: Int? // Watts
    let architecture: String?

    var displayName: String {
        "\(manufacturer) \(model)"
    }

    var vramDisplay: String {
        "\(vram)GB VRAM"
    }
}

// MARK: - Provider
struct Provider: Codable, Identifiable {
    let id: String
    let name: String
    let address: String // Wallet address
    let reputationTier: ReputationTier
    let rating: Double
    let totalRentals: Int
    let joinedDate: Date
    let location: String?
    let verified: Bool

    var reputationBadge: String {
        reputationTier.rawValue
    }

    var ratingDisplay: String {
        String(format: "%.1f/5.0", rating)
    }
}

// MARK: - GPU Listing
struct GPU: Codable, Identifiable {
    let id: String
    let providerId: String
    let specs: GPUSpecs
    let pricePerHour: Double // in ETR
    let available: Bool
    let location: String
    let minRentalHours: Int
    let maxRentalHours: Int
    let uptime: Double // percentage
    let lastOnline: Date
    let provider: Provider?

    var priceDisplay: String {
        String(format: "%.4f ETR/hr", pricePerHour)
    }

    var uptimeDisplay: String {
        String(format: "%.1f%%", uptime)
    }

    var statusColor: String {
        available ? "#00FF00" : "#FF0000"
    }

    var statusText: String {
        available ? "Available" : "Rented"
    }
}

// MARK: - Usage Metrics
struct UsageMetrics: Codable {
    let timestamp: Date
    let cpuUsage: Double // percentage
    let gpuUsage: Double // percentage
    let memoryUsage: Double // percentage
    let networkRx: Int64 // bytes
    let networkTx: Int64 // bytes
    let gpuTemp: Int? // Celsius
    let powerUsage: Int? // Watts

    var cpuDisplay: String {
        String(format: "%.1f%%", cpuUsage)
    }

    var gpuDisplay: String {
        String(format: "%.1f%%", gpuUsage)
    }

    var memoryDisplay: String {
        String(format: "%.1f%%", memoryUsage)
    }

    var networkRxDisplay: String {
        formatBytes(networkRx)
    }

    var networkTxDisplay: String {
        formatBytes(networkTx)
    }

    private func formatBytes(_ bytes: Int64) -> String {
        let kb = Double(bytes) / 1024
        let mb = kb / 1024
        let gb = mb / 1024

        if gb >= 1 {
            return String(format: "%.2f GB", gb)
        } else if mb >= 1 {
            return String(format: "%.2f MB", mb)
        } else {
            return String(format: "%.2f KB", kb)
        }
    }
}

// MARK: - SSH Credentials
struct SSHCredentials: Codable {
    let host: String
    let port: Int
    let username: String
    let password: String?
    let privateKey: String?
    let expiresAt: Date

    var connectionString: String {
        "ssh \(username)@\(host) -p \(port)"
    }

    var isExpired: Bool {
        Date() > expiresAt
    }
}

// MARK: - Rental Status
enum RentalStatus: String, Codable {
    case pending = "pending"
    case active = "active"
    case completed = "completed"
    case cancelled = "cancelled"
    case expired = "expired"

    var color: String {
        switch self {
        case .pending: return "#FFA500"
        case .active: return "#00FF00"
        case .completed: return "#808080"
        case .cancelled: return "#FF0000"
        case .expired: return "#8B0000"
        }
    }
}

// MARK: - Rental
struct Rental: Codable, Identifiable {
    let id: String
    let gpuId: String
    let renterId: String
    let providerId: String
    let startTime: Date
    let endTime: Date
    let pricePerHour: Double
    let totalCost: Double
    let status: RentalStatus
    let sshCredentials: SSHCredentials?
    let gpu: GPU?
    let usageMetrics: [UsageMetrics]?

    var durationHours: Int {
        let interval = endTime.timeIntervalSince(startTime)
        return Int(interval / 3600)
    }

    var timeRemaining: String {
        let remaining = endTime.timeIntervalSince(Date())
        if remaining <= 0 {
            return "Expired"
        }

        let hours = Int(remaining / 3600)
        let minutes = Int((remaining.truncatingRemainder(dividingBy: 3600)) / 60)

        if hours > 0 {
            return "\(hours)h \(minutes)m"
        } else {
            return "\(minutes)m"
        }
    }

    var progressPercentage: Double {
        let total = endTime.timeIntervalSince(startTime)
        let elapsed = Date().timeIntervalSince(startTime)
        return min(max(elapsed / total, 0), 1) * 100
    }
}

// MARK: - GPU Filter
struct GPUFilter: Codable {
    var minVRAM: Int?
    var maxVRAM: Int?
    var minPrice: Double?
    var maxPrice: Double?
    var models: [String]?
    var manufacturers: [String]?
    var locations: [String]?
    var availableOnly: Bool
    var minRating: Double?
    var reputationTiers: [ReputationTier]?

    init() {
        self.availableOnly = true
    }
}

// MARK: - GPU Search Request
struct GPUSearchRequest: Codable {
    let filter: GPUFilter
    let sortBy: String? // "price", "vram", "rating", "uptime"
    let sortOrder: String? // "asc", "desc"
    let page: Int
    let limit: Int

    init(filter: GPUFilter = GPUFilter(), sortBy: String = "price", sortOrder: String = "asc", page: Int = 1, limit: Int = 20) {
        self.filter = filter
        self.sortBy = sortBy
        self.sortOrder = sortOrder
        self.page = page
        self.limit = limit
    }
}

// MARK: - GPU Search Response
struct GPUSearchResponse: Codable {
    let gpus: [GPU]
    let total: Int
    let page: Int
    let totalPages: Int
}

// MARK: - Rent GPU Request
struct RentGPURequest: Codable {
    let gpuId: String
    let durationHours: Int
    let paymentMethod: String // "wallet", "staked"
}

// MARK: - Rent GPU Response
struct RentGPUResponse: Codable {
    let rental: Rental
    let transactionHash: String
    let sshCredentials: SSHCredentials
}

// MARK: - Register GPU Request
struct RegisterGPURequest: Codable {
    let specs: GPUSpecs
    let pricePerHour: Double
    let location: String
    let minRentalHours: Int
    let maxRentalHours: Int
}

// MARK: - Register GPU Response
struct RegisterGPUResponse: Codable {
    let gpuId: String
    let transactionHash: String
    let message: String
}
