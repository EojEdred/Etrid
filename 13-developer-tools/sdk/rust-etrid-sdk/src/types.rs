//! Common types used throughout the SDK

use serde::{Deserialize, Serialize};

/// Account balance information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Balance {
    /// Free balance
    pub free: u128,
    /// Reserved balance
    pub reserved: u128,
    /// Frozen balance
    pub frozen: u128,
}

/// Block information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Block {
    /// Block number
    pub number: u64,
    /// Block hash
    pub hash: String,
    /// Parent hash
    pub parent_hash: String,
    /// State root
    pub state_root: String,
    /// Extrinsics root
    pub extrinsics_root: String,
}

/// Transaction hash
pub type TxHash = String;

/// Account address (SS58 encoded)
pub type Address = String;

/// Account ID type
pub type AccountId = [u8; 32];

/// Block hash type
pub type Hash = [u8; 32];

// ===== Lightning Bloc Types =====

/// Lightning channel state
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ChannelState {
    /// Channel is pending opening
    Pending,
    /// Channel is open and active
    Open,
    /// Channel is closing
    Closing,
    /// Channel is closed
    Closed,
}

/// Lightning channel information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Channel {
    /// Channel ID
    pub id: String,
    /// Channel state
    pub state: ChannelState,
    /// Local balance
    pub local_balance: u128,
    /// Remote balance
    pub remote_balance: u128,
    /// Channel capacity
    pub capacity: u128,
    /// Remote peer address
    pub peer: Address,
}

/// Lightning payment route
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Route {
    /// Route hops
    pub hops: Vec<RouteHop>,
    /// Total fee
    pub fee: u128,
    /// Total amount
    pub amount: u128,
}

/// Single hop in a payment route
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RouteHop {
    /// Node address
    pub node: Address,
    /// Channel ID
    pub channel_id: String,
    /// Fee for this hop
    pub fee: u128,
}

// ===== Distribution Pay Types =====

/// Distribution reward information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Reward {
    /// Reward amount
    pub amount: u128,
    /// Distribution category
    pub category: RewardCategory,
    /// Timestamp
    pub timestamp: u64,
}

/// Reward category
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum RewardCategory {
    /// Validator rewards
    Validator,
    /// Director rewards
    Director,
    /// Developer rewards
    Developer,
    /// Community rewards
    Community,
}

/// Distribution schedule
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DistributionSchedule {
    /// Total amount to distribute
    pub total_amount: u128,
    /// Distribution period in blocks
    pub period: u32,
    /// Next distribution block
    pub next_block: u64,
}

// ===== ETWASM VM Types =====

/// Contract deployment result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContractDeployment {
    /// Contract address
    pub address: Address,
    /// Code hash
    pub code_hash: Hash,
    /// Gas used
    pub gas_used: u64,
}

/// Contract call result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContractCallResult {
    /// Return data
    pub data: Vec<u8>,
    /// Gas used
    pub gas_used: u64,
    /// Success flag
    pub success: bool,
}

/// Gas estimation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GasEstimate {
    /// Estimated gas
    pub gas: u64,
    /// Storage deposit
    pub storage_deposit: u128,
}

// ===== AI DID Types =====

/// AI profile information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AiProfile {
    /// AI agent ID
    pub id: String,
    /// AI name
    pub name: String,
    /// Model type
    pub model_type: String,
    /// Reputation score
    pub reputation: u32,
    /// Permissions
    pub permissions: Vec<Permission>,
}

/// Permission type
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum Permission {
    /// Read permission
    Read,
    /// Write permission
    Write,
    /// Execute permission
    Execute,
    /// Transfer permission
    Transfer,
}

// ===== Bridge Types =====

/// Supported blockchain chains
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum Chain {
    /// Ethereum
    Ethereum,
    /// Bitcoin
    Bitcoin,
    /// Binance Smart Chain
    BinanceSmartChain,
    /// Polygon
    Polygon,
    /// Solana
    Solana,
    /// Cardano
    Cardano,
}

/// Bridge transfer status
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum TransferStatus {
    /// Transfer initiated
    Initiated,
    /// Transfer pending
    Pending,
    /// Transfer confirmed
    Confirmed,
    /// Transfer completed
    Completed,
    /// Transfer failed
    Failed,
}

/// Bridge transfer information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BridgeTransfer {
    /// Transfer ID
    pub id: String,
    /// Source chain
    pub source_chain: Chain,
    /// Destination chain
    pub destination_chain: Chain,
    /// Amount
    pub amount: u128,
    /// Status
    pub status: TransferStatus,
    /// Fee
    pub fee: u128,
}

// ===== Oracle Types =====

/// Price data from oracle
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PriceData {
    /// Asset symbol
    pub symbol: String,
    /// Price in USD
    pub price: u128,
    /// Timestamp
    pub timestamp: u64,
    /// Decimals
    pub decimals: u8,
}

/// TWAP (Time-Weighted Average Price) data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TwapData {
    /// Asset symbol
    pub symbol: String,
    /// Average price
    pub price: u128,
    /// Time period in seconds
    pub period: u64,
}

// ===== Reserve Vault Types =====

/// Vault information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Vault {
    /// Vault ID
    pub id: String,
    /// Owner address
    pub owner: Address,
    /// Collateral amount
    pub collateral: u128,
    /// Borrowed amount
    pub borrowed: u128,
    /// Health factor (scaled by 10000)
    pub health_factor: u32,
}

/// Collateral type
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum CollateralType {
    /// ETR native token
    Etr,
    /// Bitcoin
    Btc,
    /// Ethereum
    Eth,
    /// Stablecoins
    Stable,
}

// ===== Staking Types =====

/// Validator information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidatorInfo {
    /// Validator address
    pub address: Address,
    /// Total stake
    pub total_stake: u128,
    /// Commission rate (0-100)
    pub commission: u8,
    /// Active status
    pub active: bool,
    /// Number of nominators
    pub nominators: u32,
}

/// Staking rewards estimate
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StakingRewards {
    /// Annual percentage yield
    pub apy: f64,
    /// Estimated daily rewards
    pub daily_rewards: u128,
    /// Estimated monthly rewards
    pub monthly_rewards: u128,
    /// Estimated annual rewards
    pub annual_rewards: u128,
}

/// Network statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkStats {
    /// Total staked amount
    pub total_staked: u128,
    /// Number of validators
    pub validator_count: u32,
    /// Number of nominators
    pub nominator_count: u32,
    /// Average APY
    pub average_apy: f64,
}

// ===== Governance Types =====

/// Proposal status
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ProposalStatus {
    /// Proposal is pending
    Pending,
    /// Proposal is active for voting
    Active,
    /// Proposal passed
    Passed,
    /// Proposal rejected
    Rejected,
    /// Proposal executed
    Executed,
}

/// Governance proposal
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Proposal {
    /// Proposal ID
    pub id: u64,
    /// Proposal title
    pub title: String,
    /// Proposal description
    pub description: String,
    /// Proposer address
    pub proposer: Address,
    /// Status
    pub status: ProposalStatus,
    /// Votes for
    pub votes_for: u128,
    /// Votes against
    pub votes_against: u128,
    /// Voting end block
    pub end_block: u64,
}

/// Vote type
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum VoteType {
    /// Vote for
    Aye,
    /// Vote against
    Nay,
    /// Abstain
    Abstain,
}

/// Proposal outcome estimation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProposalOutcome {
    /// Estimated result
    pub will_pass: bool,
    /// Current support percentage
    pub support_percentage: f64,
    /// Remaining time in blocks
    pub blocks_remaining: u64,
}
