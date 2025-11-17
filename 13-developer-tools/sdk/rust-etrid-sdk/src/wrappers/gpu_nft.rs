//! # GPU NFT Wrapper
//!
//! Rust wrapper for pallet-gpu-nft - Tradeable GPU certificates.
//!
//! ## Features
//! - Mint GPUs as tradeable NFTs
//! - Transfer GPU ownership
//! - Marketplace trading (list/buy)
//! - Ownership history tracking
//! - Rental configuration and management
//!
//! ## Example
//! ```no_run
//! use etrid_sdk::Client;
//! use etrid_sdk::wrappers::gpu_nft::{GpuNFTWrapper, RentalTerms};
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let client = Client::new("wss://ai-compute-pbc.etrid.io").await?;
//!     let nft_wrapper = GpuNFTWrapper::new(client);
//!
//!     // Mint GPU as NFT
//!     let nft_id = nft_wrapper.mint_gpu_nft(&keypair, gpu_id).await?;
//!     println!("Minted NFT #{}", nft_id);
//!
//!     // List for sale
//!     nft_wrapper.list_for_sale(&keypair, nft_id, 1000 * 10u128.pow(18)).await?;
//!
//!     // Configure rental
//!     let terms = RentalTerms {
//!         hourly_rate: 10 * 10u128.pow(18),
//!         minimum_hours: 1,
//!         maximum_hours: 720,
//!         deposit_required: 100 * 10u128.pow(18),
//!         auto_renew: false,
//!     };
//!     nft_wrapper.set_rental_terms(&keypair, nft_id, terms).await?;
//!
//!     Ok(())
//! }
//! ```

use serde::{Deserialize, Serialize};

/// NFT ID type
pub type NftId = u64;

/// GPU ID type
pub type GpuId = u64;

/// Balance type (u128 in ËDSC smallest units)
pub type Balance = u128;

/// Timestamp type (Unix timestamp)
pub type Timestamp = u64;

/// GPU NFT metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GpuNFT {
    /// NFT ID
    pub nft_id: NftId,
    /// Current owner address
    pub owner: String,
    /// Associated GPU ID
    pub gpu_id: GpuId,
    /// Reputation score at mint time (0-10000)
    pub reputation_snapshot: u32,
    /// Total earnings accumulated
    pub total_earnings: Balance,
    /// Whether listed for sale
    pub is_listed: bool,
    /// Sale price if listed
    pub list_price: Balance,
    /// Mint timestamp
    pub minted_at: Timestamp,
}

impl GpuNFT {
    /// Get reputation as score out of 100
    pub fn reputation_score(&self) -> f64 {
        self.reputation_snapshot as f64 / 100.0
    }
}

/// NFT ownership history record
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OwnershipRecord {
    /// Previous owner address
    pub previous_owner: String,
    /// New owner address
    pub new_owner: String,
    /// Sale price
    pub price: Balance,
    /// Transfer timestamp
    pub timestamp: Timestamp,
}

/// GPU rental terms configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RentalTerms {
    /// Price per hour in ËDSC smallest units
    pub hourly_rate: Balance,
    /// Minimum rental duration in hours
    pub minimum_hours: u32,
    /// Maximum rental duration in hours
    pub maximum_hours: u32,
    /// Security deposit required
    pub deposit_required: Balance,
    /// Auto-renew after expiration
    pub auto_renew: bool,
}

impl RentalTerms {
    /// Calculate total cost for given duration
    pub fn calculate_cost(&self, hours: u32) -> Balance {
        let compute_cost = self.hourly_rate.saturating_mul(hours as u128);
        compute_cost.saturating_add(self.deposit_required)
    }
}

/// Active rental information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RentalInfo {
    /// Rental ID
    pub rental_id: u64,
    /// Renter address
    pub renter: String,
    /// NFT ID being rented
    pub nft_id: NftId,
    /// Rental start time
    pub start_time: Timestamp,
    /// Rental end time
    pub end_time: Timestamp,
    /// Hourly rate paid
    pub hourly_rate: Balance,
    /// Deposit held
    pub deposit: Balance,
    /// Whether rental is active
    pub is_active: bool,
}

/// GPU NFT Wrapper
///
/// Provides methods for interacting with the GPU NFT pallet.
pub struct GpuNFTWrapper {
    pallet_name: String,
}

impl GpuNFTWrapper {
    /// Create new GPU NFT wrapper
    pub fn new() -> Self {
        Self {
            pallet_name: "gpuNft".to_string(),
        }
    }

    /// Mint GPU as NFT
    ///
    /// Creates an NFT representing ownership of a GPU.
    /// The minter must be the GPU owner/provider.
    ///
    /// # Arguments
    /// * `gpu_id` - GPU ID to mint as NFT
    ///
    /// # Returns
    /// NFT ID of newly minted certificate
    ///
    /// # Errors
    /// Returns error if not authorized or minting fails
    pub async fn mint_gpu_nft(&self, gpu_id: GpuId) -> Result<NftId, Box<dyn std::error::Error>> {
        todo!("Implement with substrate-subxt")
    }

    /// Transfer GPU NFT to another account
    ///
    /// # Arguments
    /// * `nft_id` - NFT ID to transfer
    /// * `to` - Recipient address
    ///
    /// # Returns
    /// True if successful
    pub async fn transfer_gpu_nft(
        &self,
        nft_id: NftId,
        to: &str,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        todo!("Implement with substrate-subxt")
    }

    /// List GPU NFT for sale on marketplace
    ///
    /// # Arguments
    /// * `nft_id` - NFT ID to list
    /// * `price` - Sale price in ËDSC
    ///
    /// # Returns
    /// True if successful
    pub async fn list_for_sale(
        &self,
        nft_id: NftId,
        price: Balance,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        todo!("Implement with substrate-subxt")
    }

    /// Purchase GPU NFT from marketplace
    ///
    /// Buys a listed NFT, transferring ownership and paying the list price.
    ///
    /// # Arguments
    /// * `nft_id` - NFT ID to purchase
    ///
    /// # Returns
    /// True if successful
    ///
    /// # Errors
    /// Returns error if not listed, insufficient funds, or purchase fails
    pub async fn buy_gpu_nft(&self, nft_id: NftId) -> Result<bool, Box<dyn std::error::Error>> {
        todo!("Implement with substrate-subxt")
    }

    /// Query NFT metadata and details
    ///
    /// # Arguments
    /// * `nft_id` - NFT ID
    ///
    /// # Returns
    /// Complete NFT information
    pub async fn get_nft_metadata(&self, nft_id: NftId) -> Result<GpuNFT, Box<dyn std::error::Error>> {
        todo!("Implement with substrate-subxt")
    }

    /// Track NFT ownership provenance
    ///
    /// Returns complete ownership history showing all transfers.
    ///
    /// # Arguments
    /// * `nft_id` - NFT ID
    ///
    /// # Returns
    /// List of ownership records (chronological)
    pub async fn get_ownership_history(
        &self,
        nft_id: NftId,
    ) -> Result<Vec<OwnershipRecord>, Box<dyn std::error::Error>> {
        todo!("Implement with substrate-subxt")
    }

    /// Configure GPU rental pricing and terms
    ///
    /// Sets hourly rate, duration limits, deposit requirements.
    ///
    /// # Arguments
    /// * `nft_id` - NFT ID
    /// * `terms` - Rental terms configuration
    ///
    /// # Returns
    /// True if successful
    ///
    /// # Example
    /// ```no_run
    /// let terms = RentalTerms {
    ///     hourly_rate: 10 * 10u128.pow(18),  // 10 ËDSC/hour
    ///     minimum_hours: 1,
    ///     maximum_hours: 720,  // 30 days max
    ///     deposit_required: 100 * 10u128.pow(18),  // 100 ËDSC
    ///     auto_renew: false,
    /// };
    ///
    /// nft_wrapper.set_rental_terms(nft_id, terms).await?;
    /// ```
    pub async fn set_rental_terms(
        &self,
        nft_id: NftId,
        terms: RentalTerms,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        todo!("Implement with substrate-subxt")
    }

    /// Rent GPU for compute jobs
    ///
    /// Rents GPU for specified duration, paying hourly rate + deposit.
    ///
    /// # Arguments
    /// * `nft_id` - NFT ID to rent
    /// * `duration_hours` - Rental duration in hours
    ///
    /// # Returns
    /// Rental ID
    ///
    /// # Errors
    /// Returns error if terms not set, duration invalid, or insufficient funds
    pub async fn rent_gpu(
        &self,
        nft_id: NftId,
        duration_hours: u32,
    ) -> Result<u64, Box<dyn std::error::Error>> {
        todo!("Implement with substrate-subxt")
    }

    /// Get all NFTs currently listed for sale
    ///
    /// # Arguments
    /// * `limit` - Maximum results to return
    ///
    /// # Returns
    /// List of listed NFTs
    pub async fn get_listed_nfts(&self, limit: usize) -> Result<Vec<GpuNFT>, Box<dyn std::error::Error>> {
        todo!("Implement with substrate-subxt")
    }

    /// Get active rental information
    ///
    /// # Arguments
    /// * `rental_id` - Rental ID
    ///
    /// # Returns
    /// Rental information
    pub async fn get_rental_info(&self, rental_id: u64) -> Result<RentalInfo, Box<dyn std::error::Error>> {
        todo!("Implement with substrate-subxt")
    }

    /// Cancel active rental (owner only)
    ///
    /// # Arguments
    /// * `rental_id` - Rental ID to cancel
    ///
    /// # Returns
    /// True if successful
    pub async fn cancel_rental(&self, rental_id: u64) -> Result<bool, Box<dyn std::error::Error>> {
        todo!("Implement with substrate-subxt")
    }

    /// Delist NFT from marketplace
    ///
    /// # Arguments
    /// * `nft_id` - NFT ID to delist
    ///
    /// # Returns
    /// True if successful
    pub async fn delist_nft(&self, nft_id: NftId) -> Result<bool, Box<dyn std::error::Error>> {
        todo!("Implement with substrate-subxt")
    }

    /// Get NFTs owned by account
    ///
    /// # Arguments
    /// * `owner` - Owner address
    ///
    /// # Returns
    /// List of NFT IDs owned by the account
    pub async fn get_owned_nfts(&self, owner: &str) -> Result<Vec<NftId>, Box<dyn std::error::Error>> {
        todo!("Implement with substrate-subxt")
    }
}

impl Default for GpuNFTWrapper {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rental_cost_calculation() {
        let terms = RentalTerms {
            hourly_rate: 10_000_000_000_000_000_000, // 10 ËDSC
            minimum_hours: 1,
            maximum_hours: 720,
            deposit_required: 100_000_000_000_000_000_000, // 100 ËDSC
            auto_renew: false,
        };

        // 24 hours = 240 ËDSC compute + 100 ËDSC deposit = 340 ËDSC
        let cost_24h = terms.calculate_cost(24);
        assert_eq!(cost_24h, 340_000_000_000_000_000_000);

        // 1 hour = 10 ËDSC + 100 ËDSC = 110 ËDSC
        let cost_1h = terms.calculate_cost(1);
        assert_eq!(cost_1h, 110_000_000_000_000_000_000);
    }

    #[test]
    fn test_gpu_nft_reputation_score() {
        let nft = GpuNFT {
            nft_id: 1,
            owner: "test_owner".to_string(),
            gpu_id: 1,
            reputation_snapshot: 8500,
            total_earnings: 1000_000_000_000_000_000_000,
            is_listed: false,
            list_price: 0,
            minted_at: 1700000000,
        };

        assert_eq!(nft.reputation_score(), 85.0);
    }

    #[test]
    fn test_rental_terms_validation() {
        let terms = RentalTerms {
            hourly_rate: 10_000_000_000_000_000_000,
            minimum_hours: 1,
            maximum_hours: 720,
            deposit_required: 100_000_000_000_000_000_000,
            auto_renew: false,
        };

        assert!(terms.minimum_hours <= terms.maximum_hours);
        assert!(terms.hourly_rate > 0);
    }
}
