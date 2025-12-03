import Foundation
import Combine

// MARK: - Liquidity Pool Service

/// Service for interacting with MasterChef yield farming and liquidity pools
@MainActor
public class LiquidityPoolService: ObservableObject {
    public static let shared = LiquidityPoolService()

    // MARK: - Published Properties

    @Published public var pools: [LiquidityPool] = []
    @Published public var userPositions: [UserPoolPosition] = []
    @Published public var pendingRewards: Decimal = 0
    @Published public var totalValueLocked: Decimal = 0
    @Published public var isLoading = false
    @Published public var error: String?

    // MARK: - Private Properties

    private let session: URLSession
    private var cancellables = Set<AnyCancellable>()

    // RPC endpoints
    private let primearcRPC = "https://rpc.primearc.etrid.org"

    private init() {
        let config = URLSessionConfiguration.default
        config.timeoutIntervalForRequest = 30
        self.session = URLSession(configuration: config)

        // Load initial pool data
        Task {
            await loadPools()
        }
    }

    // MARK: - Public Methods

    /// Load all available liquidity pools
    public func loadPools() async {
        isLoading = true
        error = nil

        defer { isLoading = false }

        do {
            // In production, this queries MasterChef.poolLength() and poolInfo()
            // For now, use mock data representing deployed pools
            pools = LiquidityPool.availablePools

            // Calculate TVL
            totalValueLocked = pools.reduce(0) { $0 + $1.tvl }

        } catch {
            self.error = "Failed to load pools: \(error.localizedDescription)"
        }
    }

    /// Get user's positions across all pools
    public func loadUserPositions(walletAddress: String) async {
        isLoading = true

        defer { isLoading = false }

        do {
            // In production, query userInfo(pid, address) for each pool
            var positions: [UserPoolPosition] = []
            var totalPending: Decimal = 0

            for pool in pools {
                // Query MasterChef.userInfo and pendingReward
                let position = try await fetchUserPosition(
                    poolId: pool.id,
                    walletAddress: walletAddress
                )

                if let position = position, position.stakedAmount > 0 {
                    positions.append(position)
                    totalPending += position.pendingRewards
                }
            }

            userPositions = positions
            pendingRewards = totalPending

        } catch {
            self.error = "Failed to load positions: \(error.localizedDescription)"
        }
    }

    /// Stake LP tokens in a pool
    public func stake(
        poolId: Int,
        amount: Decimal,
        walletAddress: String
    ) async throws -> String {
        guard let pool = pools.first(where: { $0.id == poolId }) else {
            throw PoolError.poolNotFound
        }

        guard amount >= pool.minStake else {
            throw PoolError.amountTooLow(minimum: pool.minStake)
        }

        // Build MasterChef.deposit(pid, amount) transaction
        let txData = buildDepositTransaction(poolId: poolId, amount: amount)

        // In production, sign and submit transaction
        // For now, return mock tx hash
        return "0x" + UUID().uuidString.replacingOccurrences(of: "-", with: "")
    }

    /// Unstake LP tokens from a pool
    public func unstake(
        poolId: Int,
        amount: Decimal,
        walletAddress: String
    ) async throws -> String {
        guard let position = userPositions.first(where: { $0.poolId == poolId }) else {
            throw PoolError.noPosition
        }

        guard amount <= position.stakedAmount else {
            throw PoolError.insufficientStake
        }

        // Build MasterChef.withdraw(pid, amount) transaction
        let txData = buildWithdrawTransaction(poolId: poolId, amount: amount)

        // In production, sign and submit transaction
        return "0x" + UUID().uuidString.replacingOccurrences(of: "-", with: "")
    }

    /// Claim pending rewards from all pools
    public func claimAllRewards(walletAddress: String) async throws -> String {
        guard pendingRewards > 0 else {
            throw PoolError.noRewardsToClaim
        }

        // Claim by calling deposit(pid, 0) for each pool with pending rewards
        var claimedPools: [Int] = []

        for position in userPositions where position.pendingRewards > 0 {
            // In production, batch these transactions
            claimedPools.append(position.poolId)
        }

        // Return mock tx hash
        return "0x" + UUID().uuidString.replacingOccurrences(of: "-", with: "")
    }

    /// Claim rewards from a specific pool
    public func claimRewards(poolId: Int, walletAddress: String) async throws -> String {
        // deposit(pid, 0) claims rewards without adding stake
        let txData = buildDepositTransaction(poolId: poolId, amount: 0)

        return "0x" + UUID().uuidString.replacingOccurrences(of: "-", with: "")
    }

    /// Calculate estimated APY for a pool
    public func calculateAPY(pool: LiquidityPool) -> Double {
        // APY = (rewardPerBlock * blocksPerYear * rewardPrice * allocPoint) / (poolTVL * totalAllocPoint)
        let blocksPerYear: Double = 365 * 24 * 60 * 60 / 6 // ~6 second blocks
        let rewardPerBlock: Double = 1.0 // ETR per block
        let rewardPrice: Double = 0.15 // ETR price in USD

        guard pool.tvl > 0 else { return 0 }

        let annualRewards = rewardPerBlock * blocksPerYear * rewardPrice * pool.allocPoint
        let apy = (annualRewards / Double(truncating: pool.tvl as NSNumber)) * 100

        return min(apy, 1000) // Cap at 1000% APY for display
    }

    // MARK: - Private Methods

    private func fetchUserPosition(
        poolId: Int,
        walletAddress: String
    ) async throws -> UserPoolPosition? {
        // In production, call MasterChef.userInfo(poolId, address) and pendingReward(poolId, address)
        // For now, return nil (no positions)
        return nil
    }

    private func buildDepositTransaction(poolId: Int, amount: Decimal) -> Data {
        // Build deposit(uint256 _pid, uint256 _amount) calldata
        // Function selector: 0xe2bbb158
        return Data()
    }

    private func buildWithdrawTransaction(poolId: Int, amount: Decimal) -> Data {
        // Build withdraw(uint256 _pid, uint256 _amount) calldata
        // Function selector: 0x441a3e70
        return Data()
    }
}

// MARK: - Pool Models

public struct LiquidityPool: Identifiable, Equatable {
    public let id: Int
    public let name: String
    public let lpTokenAddress: String
    public let token0Symbol: String
    public let token1Symbol: String
    public let allocPoint: Double
    public let tvl: Decimal
    public let apy: Double
    public let rewardToken: String
    public let minStake: Decimal
    public let network: ContractAddresses.Network

    public var displayName: String {
        "\(token0Symbol)-\(token1Symbol)"
    }

    public var formattedAPY: String {
        String(format: "%.2f%%", apy)
    }

    public var formattedTVL: String {
        let formatter = NumberFormatter()
        formatter.numberStyle = .currency
        formatter.currencyCode = "USD"
        formatter.maximumFractionDigits = 0
        return formatter.string(from: tvl as NSNumber) ?? "$0"
    }

    // Available pools on ËTRID ecosystem
    public static let availablePools: [LiquidityPool] = [
        LiquidityPool(
            id: 0,
            name: "ETR-USDT",
            lpTokenAddress: "0x0000000000000000000000000000000000000001",
            token0Symbol: "ETR",
            token1Symbol: "USDT",
            allocPoint: 40,
            tvl: 2_500_000,
            apy: 85.5,
            rewardToken: "ETR",
            minStake: 0.01,
            network: .primearc
        ),
        LiquidityPool(
            id: 1,
            name: "ETR-EDSC",
            lpTokenAddress: "0x0000000000000000000000000000000000000002",
            token0Symbol: "ETR",
            token1Symbol: "EDSC",
            allocPoint: 30,
            tvl: 1_800_000,
            apy: 72.3,
            rewardToken: "ETR",
            minStake: 0.01,
            network: .primearc
        ),
        LiquidityPool(
            id: 2,
            name: "wETR-ETH",
            lpTokenAddress: "0x0000000000000000000000000000000000000003",
            token0Symbol: "wETR",
            token1Symbol: "ETH",
            allocPoint: 20,
            tvl: 5_200_000,
            apy: 45.8,
            rewardToken: "ETR",
            minStake: 0.001,
            network: .ethereum
        ),
        LiquidityPool(
            id: 3,
            name: "wETR-BNB",
            lpTokenAddress: "0x0000000000000000000000000000000000000004",
            token0Symbol: "wETR",
            token1Symbol: "BNB",
            allocPoint: 10,
            tvl: 980_000,
            apy: 62.1,
            rewardToken: "ETR",
            minStake: 0.01,
            network: .bnbChain
        ),
        LiquidityPool(
            id: 4,
            name: "EDSC-USDT",
            lpTokenAddress: "0x0000000000000000000000000000000000000005",
            token0Symbol: "EDSC",
            token1Symbol: "USDT",
            allocPoint: 5,
            tvl: 3_100_000,
            apy: 12.5,
            rewardToken: "ETR",
            minStake: 1,
            network: .primearc
        ),
    ]
}

public struct UserPoolPosition: Identifiable {
    public var id: Int { poolId }
    public let poolId: Int
    public let poolName: String
    public let stakedAmount: Decimal
    public let stakedValueUSD: Decimal
    public let pendingRewards: Decimal
    public let pendingRewardsUSD: Decimal
    public let shareOfPool: Double // Percentage
    public let entryDate: Date

    public var formattedStaked: String {
        let formatter = NumberFormatter()
        formatter.minimumFractionDigits = 4
        formatter.maximumFractionDigits = 4
        return (formatter.string(from: stakedAmount as NSNumber) ?? "0") + " LP"
    }

    public var formattedPending: String {
        let formatter = NumberFormatter()
        formatter.minimumFractionDigits = 4
        formatter.maximumFractionDigits = 4
        return (formatter.string(from: pendingRewards as NSNumber) ?? "0") + " ETR"
    }
}

// MARK: - Pool Errors

public enum PoolError: LocalizedError {
    case poolNotFound
    case amountTooLow(minimum: Decimal)
    case noPosition
    case insufficientStake
    case noRewardsToClaim
    case transactionFailed(String)

    public var errorDescription: String? {
        switch self {
        case .poolNotFound:
            return "Pool not found"
        case .amountTooLow(let minimum):
            return "Minimum stake is \(minimum) LP tokens"
        case .noPosition:
            return "No staked position in this pool"
        case .insufficientStake:
            return "Insufficient staked amount"
        case .noRewardsToClaim:
            return "No rewards available to claim"
        case .transactionFailed(let message):
            return "Transaction failed: \(message)"
        }
    }
}
