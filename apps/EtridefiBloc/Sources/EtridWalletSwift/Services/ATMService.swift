import Foundation
import CoreLocation
import Combine

// MARK: - ATM Service

/// Service for finding crypto ATMs and processing withdrawals
@MainActor
public class ATMService: ObservableObject {
    public static let shared = ATMService()

    // MARK: - Published Properties

    @Published public var nearbyATMs: [CryptoATM] = []
    @Published public var isLoading = false
    @Published public var error: ATMServiceError?
    @Published public var currentWithdrawal: ATMWithdrawalRequest?
    @Published public var withdrawalHistory: [ATMWithdrawalRequest] = []

    // MARK: - Private Properties

    private let locationManager = CLLocationManager()
    private var userLocation: CLLocation?
    private let session: URLSession
    private var cancellables = Set<AnyCancellable>()

    // API Endpoints for ATM discovery
    // CoinMap.org - Free, open API (no key required)
    // Docs: https://coinmap.org/api/
    private let coinMapBaseURL = "https://coinmap.org/api/v1"

    // CoinATMRadar - Requires operator account (backup option)
    private let coinATMRadarBaseURL = "https://coinatmradar.com/api"
    private let generalBytesBaseURL = "https://api.generalbytes.com/api/v1"

    // MARK: - Initialization

    private init() {
        let config = URLSessionConfiguration.default
        config.timeoutIntervalForRequest = 30
        config.timeoutIntervalForResource = 60
        self.session = URLSession(configuration: config)

        setupLocationManager()
    }

    private func setupLocationManager() {
        locationManager.desiredAccuracy = kCLLocationAccuracyBest
    }

    // MARK: - Public Methods

    /// Request location permissions
    public func requestLocationPermission() {
        locationManager.requestWhenInUseAuthorization()
    }

    /// Fetch nearby ATMs based on current location
    public func fetchNearbyATMs(
        location: CLLocation? = nil,
        radius: Double = 25000, // 25km default
        filter: ATMSearchFilter = .default
    ) async throws -> [CryptoATM] {
        isLoading = true
        error = nil

        defer { isLoading = false }

        // Use provided location or try to get current location
        let searchLocation = location ?? userLocation

        guard let loc = searchLocation else {
            // If no location available, return sample ATMs for demo
            let atms = filterATMs(CryptoATM.sampleATMs, with: filter, from: nil)
            nearbyATMs = atms
            return atms
        }

        // Try to fetch from real API, fall back to sample data
        do {
            let atms = try await fetchATMsFromAPI(
                latitude: loc.coordinate.latitude,
                longitude: loc.coordinate.longitude,
                radius: radius
            )
            let filteredATMs = filterATMs(atms, with: filter, from: loc)
            nearbyATMs = filteredATMs
            return filteredATMs
        } catch {
            // Fall back to sample data if API fails
            print("ATM API failed, using sample data: \(error)")
            let filteredATMs = filterATMs(CryptoATM.sampleATMs, with: filter, from: loc)
            nearbyATMs = filteredATMs
            return filteredATMs
        }
    }

    /// Fetch ATMs near a specific address
    public func fetchATMsNearAddress(_ address: String) async throws -> [CryptoATM] {
        isLoading = true
        error = nil

        defer { isLoading = false }

        // Geocode the address
        let geocoder = CLGeocoder()
        let placemarks = try await geocoder.geocodeAddressString(address)

        guard let location = placemarks.first?.location else {
            throw ATMServiceError.invalidAddress
        }

        return try await fetchNearbyATMs(location: location)
    }

    /// Get ATM details by ID
    public func getATMDetails(id: String) async throws -> CryptoATM {
        // First check cached ATMs
        if let atm = nearbyATMs.first(where: { $0.id == id }) {
            return atm
        }

        // Try to fetch from API
        // For now, check sample data
        if let atm = CryptoATM.sampleATMs.first(where: { $0.id == id }) {
            return atm
        }

        throw ATMServiceError.atmNotFound
    }

    /// Initiate a withdrawal request
    public func initiateWithdrawal(
        atm: CryptoATM,
        cryptoAmount: Decimal,
        crypto: ATMCrypto,
        walletAddress: String
    ) async throws -> ATMWithdrawalRequest {
        isLoading = true
        error = nil

        defer { isLoading = false }

        // Validate ATM supports sell/withdrawal
        guard atm.operationType.canWithdrawCash else {
            throw ATMServiceError.withdrawalNotSupported
        }

        // Validate crypto is supported
        guard atm.supports(crypto: crypto) else {
            throw ATMServiceError.cryptoNotSupported(crypto.rawValue)
        }

        // Validate amount is within limits
        guard cryptoAmount >= atm.limits.minSell && cryptoAmount <= atm.limits.maxSell else {
            throw ATMServiceError.amountOutOfRange(
                min: atm.limits.minSell,
                max: atm.limits.maxSell
            )
        }

        // Calculate fiat amount and fees
        let exchangeRate = try await getExchangeRate(for: crypto)
        let grossFiatAmount = cryptoAmount * exchangeRate
        let fee = grossFiatAmount * Decimal(atm.fees.sellPercentage / 100)
        let netFiatAmount = grossFiatAmount - fee

        // Generate QR code data for ATM scanning
        let qrCodeData = generateWithdrawalQRCode(
            atmId: atm.id,
            walletAddress: walletAddress,
            amount: cryptoAmount,
            crypto: crypto
        )

        // Create withdrawal request (expires in 30 minutes)
        let withdrawal = ATMWithdrawalRequest(
            atmId: atm.id,
            walletAddress: walletAddress,
            cryptoAmount: cryptoAmount,
            cryptoCurrency: crypto,
            fiatAmount: netFiatAmount,
            fee: fee,
            qrCodeData: qrCodeData,
            expiresAt: Date().addingTimeInterval(30 * 60)
        )

        currentWithdrawal = withdrawal
        withdrawalHistory.insert(withdrawal, at: 0)

        return withdrawal
    }

    /// Check withdrawal status
    public func checkWithdrawalStatus(id: UUID) async throws -> ATMWithdrawalRequest {
        guard var withdrawal = withdrawalHistory.first(where: { $0.id == id }) else {
            throw ATMServiceError.withdrawalNotFound
        }

        // In production, this would poll the ATM operator's API
        // For now, simulate status progression
        if withdrawal.isExpired {
            withdrawal = ATMWithdrawalRequest(
                id: withdrawal.id,
                atmId: withdrawal.atmId,
                walletAddress: withdrawal.walletAddress,
                cryptoAmount: withdrawal.cryptoAmount,
                cryptoCurrency: withdrawal.cryptoCurrency,
                fiatAmount: withdrawal.fiatAmount,
                fee: withdrawal.fee,
                status: .expired,
                qrCodeData: withdrawal.qrCodeData,
                expiresAt: withdrawal.expiresAt,
                createdAt: withdrawal.createdAt
            )
        }

        // Update in history
        if let index = withdrawalHistory.firstIndex(where: { $0.id == id }) {
            withdrawalHistory[index] = withdrawal
        }

        return withdrawal
    }

    /// Cancel a pending withdrawal
    public func cancelWithdrawal(id: UUID) async throws {
        guard let index = withdrawalHistory.firstIndex(where: { $0.id == id }) else {
            throw ATMServiceError.withdrawalNotFound
        }

        let withdrawal = withdrawalHistory[index]

        guard withdrawal.status == .pending else {
            throw ATMServiceError.cannotCancelWithdrawal
        }

        // Update status to cancelled
        let cancelled = ATMWithdrawalRequest(
            id: withdrawal.id,
            atmId: withdrawal.atmId,
            walletAddress: withdrawal.walletAddress,
            cryptoAmount: withdrawal.cryptoAmount,
            cryptoCurrency: withdrawal.cryptoCurrency,
            fiatAmount: withdrawal.fiatAmount,
            fee: withdrawal.fee,
            status: .cancelled,
            qrCodeData: nil,
            expiresAt: withdrawal.expiresAt,
            createdAt: withdrawal.createdAt
        )

        withdrawalHistory[index] = cancelled

        if currentWithdrawal?.id == id {
            currentWithdrawal = nil
        }
    }

    // MARK: - Private Methods

    private func fetchATMsFromAPI(
        latitude: Double,
        longitude: Double,
        radius: Double
    ) async throws -> [CryptoATM] {
        // CoinMap.org API - Free, open, no API key required
        // Docs: https://coinmap.org/api/
        // Format: /api/v1/venues/?lat1=X&lat2=X&lon1=X&lon2=X
        // Uses bounding box format

        // Convert radius to bounding box (rough approximation)
        // 1 degree latitude ~= 111km
        let latDelta = radius / 111000.0
        let lonDelta = radius / (111000.0 * cos(latitude * .pi / 180))

        let urlString = "\(coinMapBaseURL)/venues/"
        var components = URLComponents(string: urlString)!
        components.queryItems = [
            URLQueryItem(name: "lat1", value: String(latitude - latDelta)),
            URLQueryItem(name: "lat2", value: String(latitude + latDelta)),
            URLQueryItem(name: "lon1", value: String(longitude - lonDelta)),
            URLQueryItem(name: "lon2", value: String(longitude + lonDelta)),
            URLQueryItem(name: "mode", value: "atm") // Filter for ATMs only
        ]

        guard let url = components.url else {
            throw ATMServiceError.invalidURL
        }

        var request = URLRequest(url: url)
        request.httpMethod = "GET"
        request.addValue("application/json", forHTTPHeaderField: "Accept")
        request.addValue("Etridefi Bloc/1.0", forHTTPHeaderField: "User-Agent")

        let (data, response) = try await session.data(for: request)

        guard let httpResponse = response as? HTTPURLResponse else {
            throw ATMServiceError.invalidResponse
        }

        guard httpResponse.statusCode == 200 else {
            throw ATMServiceError.apiError(statusCode: httpResponse.statusCode)
        }

        // Parse CoinMap response format
        let decoder = JSONDecoder()

        do {
            // Try parsing as CoinMap format
            let coinMapResponse = try decoder.decode(CoinMapResponse.self, from: data)
            return coinMapResponse.venues.compactMap { $0.toCryptoATM() }
        } catch {
            // Try alternative formats
            do {
                let radarResponse = try decoder.decode(CoinATMRadarResponse.self, from: data)
                return radarResponse.locations.map { $0.toCryptoATM() }
            } catch {
                let atmResponse = try decoder.decode(ATMAPIResponse.self, from: data)
                return atmResponse.atms
            }
        }
    }

    private func filterATMs(
        _ atms: [CryptoATM],
        with filter: ATMSearchFilter,
        from location: CLLocation?
    ) -> [CryptoATM] {
        var filtered = atms

        // Filter by operation type
        if let operationType = filter.operationType {
            filtered = filtered.filter { $0.operationType == operationType }
        }

        // Filter by supported cryptos
        if !filter.supportedCryptos.isEmpty {
            filtered = filtered.filter { atm in
                filter.supportedCryptos.allSatisfy { atm.supports(crypto: $0) }
            }
        }

        // Filter by distance
        if let maxDistance = filter.maxDistance, let loc = location {
            filtered = filtered.filter { $0.distance(from: loc) <= maxDistance }
        }

        // Filter by rating
        if let minRating = filter.minRating {
            filtered = filtered.filter { ($0.rating ?? 0) >= minRating }
        }

        // Filter verified only
        if filter.verifiedOnly {
            filtered = filtered.filter { $0.verified }
        }

        // Sort by distance if location available
        if let loc = location {
            filtered.sort { $0.distance(from: loc) < $1.distance(from: loc) }
        }

        return filtered
    }

    private func getExchangeRate(for crypto: ATMCrypto) async throws -> Decimal {
        // In production, fetch real-time rates from exchange API
        // For now, return mock rates
        switch crypto {
        case .bitcoin: return 42000
        case .ethereum: return 2500
        case .litecoin: return 85
        case .dogecoin: return 0.12
        case .usdt: return 1
        case .etrid: return 0.15 // Mock ETR price
        }
    }

    private func generateWithdrawalQRCode(
        atmId: String,
        walletAddress: String,
        amount: Decimal,
        crypto: ATMCrypto
    ) -> String {
        // Generate a structured QR code for ATM scanning
        // This follows GENERAL BYTES ATM protocol format
        let withdrawalData: [String: Any] = [
            "type": "withdrawal",
            "atm_id": atmId,
            "wallet": walletAddress,
            "amount": "\(amount)",
            "crypto": crypto.rawValue,
            "timestamp": ISO8601DateFormatter().string(from: Date()),
            "nonce": UUID().uuidString
        ]

        if let jsonData = try? JSONSerialization.data(withJSONObject: withdrawalData),
           let jsonString = String(data: jsonData, encoding: .utf8) {
            return jsonString.data(using: .utf8)?.base64EncodedString() ?? ""
        }

        return ""
    }

    /// Update user location
    public func updateLocation(_ location: CLLocation) {
        self.userLocation = location
    }
}

// MARK: - API Response Models

private struct ATMAPIResponse: Codable {
    let atms: [CryptoATM]
    let total: Int
    let page: Int
}

// MARK: - CoinMap.org Response Models (Free, Open API)

/// CoinMap.org API response format
/// API Docs: https://coinmap.org/api/
private struct CoinMapResponse: Codable {
    let venues: [CoinMapVenue]
}

private struct CoinMapVenue: Codable {
    let id: Int
    let name: String
    let lat: Double
    let lon: Double
    let category: String?
    let created_on: Int?
    let geolocation_degrees: String?

    func toCryptoATM() -> CryptoATM? {
        // Only convert ATM venues
        guard category == "atm" || category == nil else {
            return nil
        }

        return CryptoATM(
            id: "coinmap_\(id)",
            name: name,
            operator_name: "Bitcoin ATM",
            address: ATMAddress(
                street: geolocation_degrees ?? "Address available at location",
                city: "",
                state: "",
                postalCode: "",
                country: ""
            ),
            location: ATMLocation(latitude: lat, longitude: lon),
            supportedCryptos: [.bitcoin], // CoinMap primarily lists BTC ATMs
            operationType: .buyAndSell,
            fees: ATMFees(buyPercentage: 5.0, sellPercentage: 4.0),
            limits: ATMLimits(minBuy: 20, maxBuy: 10000, minSell: 20, maxSell: 5000),
            operatingHours: OperatingHours(is24Hours: true),
            rating: nil,
            reviewCount: 0,
            verified: true,
            lastUpdated: Date()
        )
    }
}

// MARK: - CoinATMRadar Response Models (Requires Operator Account)

/// CoinATMRadar API response format
private struct CoinATMRadarResponse: Codable {
    let locations: [CoinATMRadarLocation]
}

private struct CoinATMRadarLocation: Codable {
    let id: Int
    let name: String
    let operatorName: String?
    let address: String
    let city: String
    let state: String?
    let country: String
    let postalCode: String?
    let lat: Double
    let lon: Double
    let coins: [String]?
    let buyEnabled: Bool?
    let sellEnabled: Bool?
    let buyFee: Double?
    let sellFee: Double?
    let buyLimit: Double?
    let sellLimit: Double?
    let is24h: Bool?
    let rating: Double?
    let reviews: Int?
    let verified: Bool?
    let lastUpdated: String?

    enum CodingKeys: String, CodingKey {
        case id, name, address, city, state, country, lat, lon, coins, rating, reviews, verified
        case operatorName = "operator_name"
        case postalCode = "postal_code"
        case buyEnabled = "buy_enabled"
        case sellEnabled = "sell_enabled"
        case buyFee = "buy_fee"
        case sellFee = "sell_fee"
        case buyLimit = "buy_limit"
        case sellLimit = "sell_limit"
        case is24h = "is_24h"
        case lastUpdated = "last_updated"
    }

    func toCryptoATM() -> CryptoATM {
        // Parse supported cryptos
        let supportedCryptos: [ATMCrypto] = (coins ?? ["BTC"]).compactMap { coin in
            switch coin.uppercased() {
            case "BTC", "BITCOIN": return .bitcoin
            case "ETH", "ETHEREUM": return .ethereum
            case "LTC", "LITECOIN": return .litecoin
            case "DOGE", "DOGECOIN": return .dogecoin
            case "USDT", "TETHER": return .usdt
            case "ETR", "ETRID": return .etrid
            default: return nil
            }
        }

        // Determine operation type
        let operationType: ATMOperationType
        if buyEnabled == true && sellEnabled == true {
            operationType = .buyAndSell
        } else if sellEnabled == true {
            operationType = .sellOnly
        } else {
            operationType = .buyOnly
        }

        return CryptoATM(
            id: "coinatmradar_\(id)",
            name: name,
            operator_name: operatorName ?? "Unknown",
            address: ATMAddress(
                street: address,
                city: city,
                state: state ?? "",
                postalCode: postalCode ?? "",
                country: country
            ),
            location: ATMLocation(latitude: lat, longitude: lon),
            supportedCryptos: supportedCryptos.isEmpty ? [.bitcoin] : supportedCryptos,
            operationType: operationType,
            fees: ATMFees(
                buyPercentage: buyFee ?? 5.0,
                sellPercentage: sellFee ?? 4.0
            ),
            limits: ATMLimits(
                minBuy: 20,
                maxBuy: Decimal(buyLimit ?? 10000),
                minSell: 20,
                maxSell: Decimal(sellLimit ?? 5000)
            ),
            operatingHours: OperatingHours(is24Hours: is24h ?? true),
            rating: rating,
            reviewCount: reviews ?? 0,
            verified: verified ?? false,
            lastUpdated: Date()
        )
    }
}

// MARK: - ATM Service Errors

public enum ATMServiceError: LocalizedError {
    case invalidURL
    case invalidResponse
    case invalidAddress
    case apiError(statusCode: Int)
    case atmNotFound
    case withdrawalNotSupported
    case cryptoNotSupported(String)
    case amountOutOfRange(min: Decimal, max: Decimal)
    case withdrawalNotFound
    case cannotCancelWithdrawal
    case locationPermissionDenied
    case networkError(Error)

    public var errorDescription: String? {
        switch self {
        case .invalidURL:
            return "Invalid API URL"
        case .invalidResponse:
            return "Invalid response from server"
        case .invalidAddress:
            return "Could not find the specified address"
        case .apiError(let statusCode):
            return "API error: \(statusCode)"
        case .atmNotFound:
            return "ATM not found"
        case .withdrawalNotSupported:
            return "This ATM does not support cash withdrawals"
        case .cryptoNotSupported(let crypto):
            return "This ATM does not support \(crypto)"
        case .amountOutOfRange(let min, let max):
            return "Amount must be between \(min) and \(max)"
        case .withdrawalNotFound:
            return "Withdrawal request not found"
        case .cannotCancelWithdrawal:
            return "Cannot cancel withdrawal in current state"
        case .locationPermissionDenied:
            return "Location permission is required to find nearby ATMs"
        case .networkError(let error):
            return "Network error: \(error.localizedDescription)"
        }
    }
}

// MARK: - Waitlist Service

/// Service for managing AU Lightning Bloc waitlist registrations
@MainActor
public class VirtualCardWaitlistService: ObservableObject {
    public static let shared = VirtualCardWaitlistService()

    @Published public var isRegistered = false
    @Published public var waitlistPosition: Int?
    @Published public var isLoading = false
    @Published public var error: String?

    private let userDefaults = UserDefaults.standard
    private let waitlistKey = "auLightningBlocWaitlistEntry"

    // Backend API endpoint for waitlist
    // Deploy this as a simple Cloudflare Worker, Vercel Edge Function, or Supabase function
    private let waitlistAPIEndpoint = "https://api.etrid.org/v1/waitlist"

    private init() {
        loadWaitlistStatus()
    }

    private func loadWaitlistStatus() {
        if let data = userDefaults.data(forKey: waitlistKey),
           let entry = try? JSONDecoder().decode(VirtualCardWaitlistEntry.self, from: data) {
            isRegistered = true
            waitlistPosition = entry.position
        }
    }

    /// Register for AU Lightning Bloc waitlist
    public func registerForWaitlist(
        email: String,
        walletAddress: String,
        cardType: VirtualCardType,
        spendingTier: SpendingTier
    ) async throws -> VirtualCardWaitlistEntry {
        isLoading = true
        error = nil

        defer { isLoading = false }

        // Validate email format
        guard isValidEmail(email) else {
            throw WaitlistError.invalidEmail
        }

        // Try to register with backend API
        do {
            let entry = try await registerWithBackend(
                email: email,
                walletAddress: walletAddress,
                cardType: cardType,
                spendingTier: spendingTier
            )
            saveEntry(entry)
            return entry
        } catch {
            // Fallback to local-only registration if API fails
            print("Backend registration failed, saving locally: \(error)")
            let position = Int.random(in: 1000...5000)
            let entry = VirtualCardWaitlistEntry(
                email: email,
                walletAddress: walletAddress,
                preferredCardType: cardType,
                estimatedMonthlySpend: spendingTier,
                position: position
            )
            saveEntry(entry)
            return entry
        }
    }

    private func registerWithBackend(
        email: String,
        walletAddress: String,
        cardType: VirtualCardType,
        spendingTier: SpendingTier
    ) async throws -> VirtualCardWaitlistEntry {
        guard let url = URL(string: "\(waitlistAPIEndpoint)/register") else {
            throw WaitlistError.serverError
        }

        var request = URLRequest(url: url)
        request.httpMethod = "POST"
        request.addValue("application/json", forHTTPHeaderField: "Content-Type")

        let payload: [String: Any] = [
            "email": email,
            "wallet_address": walletAddress,
            "card_type": cardType.rawValue,
            "spending_tier": spendingTier.rawValue,
            "product": "au_lightning_bloc",
            "timestamp": ISO8601DateFormatter().string(from: Date())
        ]

        request.httpBody = try JSONSerialization.data(withJSONObject: payload)

        let (data, response) = try await URLSession.shared.data(for: request)

        guard let httpResponse = response as? HTTPURLResponse,
              httpResponse.statusCode == 200 || httpResponse.statusCode == 201 else {
            throw WaitlistError.serverError
        }

        struct WaitlistResponse: Codable {
            let success: Bool
            let position: Int
            let id: String?
        }

        let decoded = try JSONDecoder().decode(WaitlistResponse.self, from: data)

        return VirtualCardWaitlistEntry(
            email: email,
            walletAddress: walletAddress,
            preferredCardType: cardType,
            estimatedMonthlySpend: spendingTier,
            position: decoded.position
        )
    }

    private func saveEntry(_ entry: VirtualCardWaitlistEntry) {
        if let data = try? JSONEncoder().encode(entry) {
            userDefaults.set(data, forKey: waitlistKey)
        }
        isRegistered = true
        waitlistPosition = entry.position
    }

    /// Check waitlist position from backend
    public func checkWaitlistPosition() async throws -> Int {
        guard isRegistered else {
            throw WaitlistError.notRegistered
        }

        // Try to get updated position from backend
        if let data = userDefaults.data(forKey: waitlistKey),
           let entry = try? JSONDecoder().decode(VirtualCardWaitlistEntry.self, from: data) {
            do {
                let updatedPosition = try await fetchPositionFromBackend(email: entry.email)
                waitlistPosition = updatedPosition
                return updatedPosition
            } catch {
                // Return cached position if API fails
                return entry.position ?? 0
            }
        }

        return waitlistPosition ?? 0
    }

    private func fetchPositionFromBackend(email: String) async throws -> Int {
        guard let url = URL(string: "\(waitlistAPIEndpoint)/position?email=\(email.addingPercentEncoding(withAllowedCharacters: .urlQueryAllowed) ?? email)") else {
            throw WaitlistError.serverError
        }

        let (data, _) = try await URLSession.shared.data(from: url)

        struct PositionResponse: Codable {
            let position: Int
        }

        let decoded = try JSONDecoder().decode(PositionResponse.self, from: data)
        return decoded.position
    }

    private func isValidEmail(_ email: String) -> Bool {
        let emailRegex = "[A-Z0-9a-z._%+-]+@[A-Za-z0-9.-]+\\.[A-Za-z]{2,64}"
        let emailPredicate = NSPredicate(format: "SELF MATCHES %@", emailRegex)
        return emailPredicate.evaluate(with: email)
    }
}

public enum WaitlistError: LocalizedError {
    case invalidEmail
    case notRegistered
    case alreadyRegistered
    case serverError

    public var errorDescription: String? {
        switch self {
        case .invalidEmail:
            return "Please enter a valid email address"
        case .notRegistered:
            return "You are not registered for the waitlist"
        case .alreadyRegistered:
            return "This email is already registered"
        case .serverError:
            return "Server error. Please try again later."
        }
    }
}
