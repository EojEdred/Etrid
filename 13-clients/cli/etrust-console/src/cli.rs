// ═══════════════════════════════════════════════════════════════════════════════
// CLI Command Definitions
// Inspired by Reth/Lighthouse CLI patterns
// ═══════════════════════════════════════════════════════════════════════════════

use clap::{Parser, Subcommand};

/// ËTRUST - ËTRID Rust CLI
///
/// Professional command-line interface for interacting with ËTRID Protocol.
/// Supports account management, staking, queries, transactions, and consensus operations.
#[derive(Parser, Debug)]
#[command(name = "etrust")]
#[command(author = "ËTRID Foundation")]
#[command(version = "0.1.0")]
#[command(about = "ËTRID Rust CLI - Professional interface for ËTRID Protocol", long_about = None)]
#[command(propagate_version = true)]
pub struct Cli {
    /// RPC endpoint URL
    #[arg(
        short = 'e',
        long = "endpoint",
        global = true,
        default_value = "ws://localhost:9944",
        env = "ETRID_RPC_ENDPOINT"
    )]
    pub endpoint: String,

    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Account management operations
    #[command(name = "account")]
    Account {
        #[command(subcommand)]
        subcommand: AccountCommands,
    },

    /// Staking operations for validators and nominators
    #[command(name = "stake")]
    Stake {
        #[command(subcommand)]
        subcommand: StakeCommands,
    },

    /// Query blockchain state and data
    #[command(name = "query")]
    Query {
        #[command(subcommand)]
        subcommand: QueryCommands,
    },

    /// Send transactions to the network
    #[command(name = "send")]
    Send {
        #[command(subcommand)]
        subcommand: SendCommands,
    },

    /// Consensus day operations (proposals, voting, distribution)
    #[command(name = "consensus")]
    Consensus {
        #[command(subcommand)]
        subcommand: ConsensusCommands,
    },

    /// Key management operations
    #[command(name = "keys")]
    Keys {
        #[command(subcommand)]
        subcommand: KeysCommands,
    },
}

// ═══════════════════════════════════════════════════════════════════════════════
// Account Subcommands
// ═══════════════════════════════════════════════════════════════════════════════

#[derive(Subcommand, Debug)]
pub enum AccountCommands {
    /// Create a new account (generates keypair)
    Create {
        /// Account name (optional)
        #[arg(short, long)]
        name: Option<String>,

        /// Password for keystore encryption
        #[arg(short, long)]
        password: Option<String>,
    },

    /// List all local accounts
    List,

    /// Import an account from seed phrase or private key
    Import {
        /// Seed phrase or private key (hex)
        #[arg(short, long)]
        secret: String,

        /// Account name
        #[arg(short, long)]
        name: Option<String>,

        /// Password for keystore encryption
        #[arg(short, long)]
        password: Option<String>,
    },

    /// Export account details
    Export {
        /// Account address or name
        account: String,

        /// Export format: json, seed, hex
        #[arg(short, long, default_value = "json")]
        format: String,
    },

    /// Show account balance
    Balance {
        /// Account address
        account: String,
    },
}

// ═══════════════════════════════════════════════════════════════════════════════
// Stake Subcommands
// ═══════════════════════════════════════════════════════════════════════════════

#[derive(Subcommand, Debug)]
pub enum StakeCommands {
    /// Stake tokens to become a validator or nominator
    Stake {
        /// Amount to stake (in ETR)
        amount: String,

        /// Validator address (for nominating)
        #[arg(short, long)]
        validator: Option<String>,

        /// Account to stake from
        #[arg(short, long)]
        from: String,
    },

    /// Unstake tokens
    Unstake {
        /// Amount to unstake (in ETR)
        amount: String,

        /// Account to unstake from
        #[arg(short, long)]
        from: String,
    },

    /// Query staking information
    Info {
        /// Account address
        account: String,
    },

    /// List all validators
    Validators,

    /// Nominate a validator
    Nominate {
        /// Validator address to nominate
        validator: String,

        /// Amount to nominate (in ETR)
        amount: String,

        /// Account to nominate from
        #[arg(short, long)]
        from: String,
    },
}

// ═══════════════════════════════════════════════════════════════════════════════
// Query Subcommands
// ═══════════════════════════════════════════════════════════════════════════════

#[derive(Subcommand, Debug)]
pub enum QueryCommands {
    /// Get block information by number or hash
    Block {
        /// Block number or hash
        block_id: String,
    },

    /// Get transaction by hash
    Transaction {
        /// Transaction hash
        tx_hash: String,
    },

    /// Get account balance
    Balance {
        /// Account address
        account: String,
    },

    /// Get chain metadata
    ChainInfo,

    /// Get network peers
    Peers,

    /// Get current block number
    BlockNumber,
}

// ═══════════════════════════════════════════════════════════════════════════════
// Send Subcommands
// ═══════════════════════════════════════════════════════════════════════════════

#[derive(Subcommand, Debug)]
pub enum SendCommands {
    /// Send native tokens (ETR, ETD, VMW)
    Transfer {
        /// Recipient address
        to: String,

        /// Amount to send
        amount: String,

        /// Sender account
        #[arg(short, long)]
        from: String,

        /// Token type (ETR, ETD, VMW)
        #[arg(short = 't', long, default_value = "ETR")]
        token: String,
    },

    /// Deploy a smart contract
    Deploy {
        /// WASM contract file path
        contract: String,

        /// Deployer account
        #[arg(short, long)]
        from: String,

        /// Constructor arguments (JSON)
        #[arg(short, long)]
        args: Option<String>,
    },

    /// Call a smart contract
    Call {
        /// Contract address
        contract: String,

        /// Method name
        method: String,

        /// Method arguments (JSON)
        #[arg(short, long)]
        args: Option<String>,

        /// Caller account
        #[arg(short, long)]
        from: String,
    },
}

// ═══════════════════════════════════════════════════════════════════════════════
// Consensus Subcommands
// ═══════════════════════════════════════════════════════════════════════════════

#[derive(Subcommand, Debug)]
pub enum ConsensusCommands {
    /// Submit a governance proposal
    ProposeSubmit {
        /// Proposal title
        title: String,

        /// Proposal description
        description: String,

        /// Proposer account
        #[arg(short, long)]
        from: String,
    },

    /// List active proposals
    ProposeList,

    /// Vote on a proposal
    Vote {
        /// Proposal ID
        proposal_id: u32,

        /// Vote: yes, no, abstain
        vote: String,

        /// Voter account
        #[arg(short, long)]
        from: String,
    },

    /// Query proposal details
    ProposalInfo {
        /// Proposal ID
        proposal_id: u32,
    },

    /// Query consensus day status
    Status,

    /// Query fiscal distribution schedule
    Distribution,
}

// ═══════════════════════════════════════════════════════════════════════════════
// Keys Subcommands
// ═══════════════════════════════════════════════════════════════════════════════

#[derive(Subcommand, Debug)]
pub enum KeysCommands {
    /// Generate a new keypair
    Generate {
        /// Key type: sr25519, ed25519, ecdsa
        #[arg(short, long, default_value = "sr25519")]
        key_type: String,
    },

    /// Derive a child key from a parent key
    Derive {
        /// Parent seed or mnemonic
        parent: String,

        /// Derivation path (e.g., //Alice//stash)
        path: String,
    },

    /// Inspect a key (show public key, address)
    Inspect {
        /// Seed, mnemonic, or secret key
        key: String,
    },

    /// Generate a mnemonic phrase
    Mnemonic,
}
