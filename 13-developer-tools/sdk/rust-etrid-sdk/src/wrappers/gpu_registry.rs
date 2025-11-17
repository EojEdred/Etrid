//! # GPU Registry Wrapper
//!
//! Rust wrapper for pallet-gpu-registry - GPU provider registration and management.
//!
//! ## Features
//! - GPU node registration with hardware attestation
//! - Staking mechanism (providers stake ËDSC)
//! - Reputation tracking (uptime, job success, ratings)
//! - Hardware verification
//! - Availability scheduling
//!
//! ## Example
//! ```no_run
//! use etrid_sdk::Client;
//! use etrid_sdk::wrappers::gpu_registry::{GpuRegistryWrapper, GpuSpecs, HardwareAttestation};
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let client = Client::new("wss://ai-compute-pbc.etrid.io").await?;
//!     let registry = GpuRegistryWrapper::new(client);
//!
//!     // Register GPU
//!     let specs = GpuSpecs {
//!         model: "RTX 4090".to_string(),
//!         vram_gb: 24,
//!         compute_units: 16384,
//!         clock_speed_mhz: 2520,
//!         tdp_watts: 450,
//!     };
//!
//!     let attestation = HardwareAttestation {
//!         tpm_quote: vec![0u8; 256],
//!         benchmark_score: 98500,
//!         timestamp: 1700000000,
//!     };
//!
//!     let gpu_id = registry.register_gpu(
//!         &keypair,
//!         specs,
//!         attestation,
//!         100_000_000_000_000_000_000, // 100 ËDSC
//!         AvailabilitySchedule::AlwaysOn
//!     ).await?;
//!
//!     println!("GPU registered with ID: {}", gpu_id);
//!     Ok(())
//! }
//! ```

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// GPU ID type
pub type GpuId = u64;

/// Balance type (u128 in ËDSC smallest units)
pub type Balance = u128;

/// Timestamp type (Unix timestamp)
pub type Timestamp = u64;

/// Basis points (0-10000 representing 0%-100%)
pub type BasisPoints = u16;

/// GPU hardware specifications
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GpuSpecs {
    /// GPU model name (e.g., "RTX 4090", "A100")
    pub model: String,
    /// VRAM in gigabytes
    pub vram_gb: u16,
    /// Compute units (CUDA cores, stream processors, etc.)
    pub compute_units: u32,
    /// Clock speed in MHz
    pub clock_speed_mhz: u16,
    /// Power consumption in watts
    pub tdp_watts: u16,
}

/// Hardware attestation proof
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HardwareAttestation {
    /// TPM quote proving hardware authenticity (max 256 bytes)
    pub tpm_quote: Vec<u8>,
    /// Benchmark score proving performance
    pub benchmark_score: u32,
    /// Attestation timestamp
    pub timestamp: u64,
}

/// GPU provider reputation metrics
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Reputation {
    /// Total jobs completed successfully
    pub jobs_completed: u64,
    /// Total jobs failed
    pub jobs_failed: u64,
    /// Uptime percentage in basis points (0-10000 = 0.00%-100.00%)
    pub uptime_bps: u16,
    /// Average user rating (0-50000 = 0.0-5.0 stars, scaled by 10000)
    pub rating: u32,
    /// Total ratings received
    pub rating_count: u32,
}

impl Reputation {
    /// Get uptime as percentage (0.0-100.0)
    pub fn uptime_percent(&self) -> f64 {
        self.uptime_bps as f64 / 100.0
    }

    /// Get rating as stars (0.0-5.0)
    pub fn rating_stars(&self) -> f64 {
        self.rating as f64 / 10000.0
    }

    /// Get job success rate as percentage
    pub fn success_rate(&self) -> f64 {
        let total = self.jobs_completed + self.jobs_failed;
        if total == 0 {
            0.0
        } else {
            (self.jobs_completed as f64 / total as f64) * 100.0
        }
    }
}

/// GPU node status
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum GpuStatus {
    /// GPU is online and accepting jobs
    Active,
    /// GPU is temporarily paused (manual)
    Paused,
    /// GPU is offline (detected by off-chain worker)
    Offline,
    /// GPU is slashed for misbehavior
    Slashed,
}

/// GPU availability schedule
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AvailabilitySchedule {
    /// Available 24/7
    AlwaysOn,
    /// Business hours (9am-5pm UTC)
    BusinessHours,
    /// Custom hours (168 bits for each hour of the week)
    Custom(Vec<u8>), // 21 bytes = 168 bits
}

/// Complete GPU node information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GpuNode {
    /// Provider account address
    pub provider: String,
    /// GPU specifications
    pub specs: GpuSpecs,
    /// Hardware attestation
    pub attestation: HardwareAttestation,
    /// Staked amount in ËDSC
    pub stake: Balance,
    /// Current status
    pub status: GpuStatus,
    /// Reputation metrics
    pub reputation: Reputation,
    /// Availability schedule
    pub schedule: AvailabilitySchedule,
    /// Registration timestamp
    pub registered_at: Timestamp,
    /// Last heartbeat timestamp
    pub last_heartbeat: Timestamp,
}

/// Provider earnings information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProviderEarnings {
    /// Total earned in ËDSC
    pub total_earned: Balance,
    /// Pending payout amount
    pub pending_payout: Balance,
    /// Last payout timestamp
    pub last_payout: Timestamp,
    /// Earnings history
    pub earnings_history: Vec<EarningsRecord>,
}

/// Individual earnings record
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EarningsRecord {
    /// Amount earned
    pub amount: Balance,
    /// Job ID or rental ID
    pub source_id: u64,
    /// Timestamp
    pub timestamp: Timestamp,
}

/// GPU search filters
#[derive(Debug, Clone, Default)]
pub struct GpuSearchFilters {
    /// Minimum VRAM in GB
    pub min_vram_gb: Option<u16>,
    /// Minimum compute units
    pub min_compute_units: Option<u32>,
    /// GPU status filter
    pub status: Option<GpuStatus>,
    /// Minimum rating (0.0-5.0)
    pub min_rating: Option<f64>,
    /// Minimum uptime percentage (0.0-100.0)
    pub min_uptime: Option<f64>,
    /// Maximum results
    pub limit: usize,
}

impl Default for AvailabilitySchedule {
    fn default() -> Self {
        AvailabilitySchedule::AlwaysOn
    }
}

/// GPU Registry Wrapper
///
/// Provides methods for interacting with the GPU registry pallet.
pub struct GpuRegistryWrapper {
    pallet_name: String,
}

impl GpuRegistryWrapper {
    /// Create new GPU registry wrapper
    pub fn new() -> Self {
        Self {
            pallet_name: "gpuRegistry".to_string(),
        }
    }

    /// Register a new GPU node
    ///
    /// # Arguments
    /// * `specs` - GPU hardware specifications
    /// * `attestation` - Hardware attestation proof
    /// * `stake` - Amount to stake (must be >= MinimumStake)
    /// * `schedule` - Availability schedule
    ///
    /// # Returns
    /// GPU ID assigned to the registered GPU
    ///
    /// # Errors
    /// Returns error if stake is insufficient or registration fails
    pub async fn register_gpu(
        &self,
        specs: GpuSpecs,
        attestation: HardwareAttestation,
        stake: Balance,
        schedule: AvailabilitySchedule,
    ) -> Result<GpuId, Box<dyn std::error::Error>> {
        // Implementation would use substrate-subxt or similar
        // This is a placeholder showing the API structure
        todo!("Implement with substrate-subxt")
    }

    /// Unregister GPU and withdraw stake
    ///
    /// # Arguments
    /// * `gpu_id` - GPU ID to unregister
    ///
    /// # Returns
    /// True if successful
    pub async fn unregister_gpu(&self, gpu_id: GpuId) -> Result<bool, Box<dyn std::error::Error>> {
        todo!("Implement with substrate-subxt")
    }

    /// Update GPU availability schedule
    ///
    /// # Arguments
    /// * `gpu_id` - GPU ID
    /// * `schedule` - New availability schedule
    pub async fn update_availability(
        &self,
        gpu_id: GpuId,
        schedule: AvailabilitySchedule,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        todo!("Implement with substrate-subxt")
    }

    /// Get GPU specifications and details
    ///
    /// # Arguments
    /// * `gpu_id` - GPU ID to query
    ///
    /// # Returns
    /// Complete GPU node information
    pub async fn get_gpu_specs(&self, gpu_id: GpuId) -> Result<GpuNode, Box<dyn std::error::Error>> {
        todo!("Implement with substrate-subxt")
    }

    /// Get provider reputation metrics
    ///
    /// # Arguments
    /// * `gpu_id` - GPU ID
    ///
    /// # Returns
    /// Reputation metrics
    pub async fn get_reputation(&self, gpu_id: GpuId) -> Result<Reputation, Box<dyn std::error::Error>> {
        let gpu = self.get_gpu_specs(gpu_id).await?;
        Ok(gpu.reputation)
    }

    /// Search for GPUs matching criteria
    ///
    /// # Arguments
    /// * `filters` - Search filters
    ///
    /// # Returns
    /// List of matching GPU nodes
    ///
    /// # Example
    /// ```no_run
    /// let filters = GpuSearchFilters {
    ///     min_vram_gb: Some(24),
    ///     min_compute_units: Some(15000),
    ///     status: Some(GpuStatus::Active),
    ///     min_rating: Some(4.0),
    ///     min_uptime: Some(95.0),
    ///     limit: 10,
    /// };
    ///
    /// let results = registry.search_gpus(filters).await?;
    /// ```
    pub async fn search_gpus(
        &self,
        filters: GpuSearchFilters,
    ) -> Result<Vec<GpuNode>, Box<dyn std::error::Error>> {
        todo!("Implement with substrate-subxt")
    }

    /// Report GPU online status (heartbeat)
    ///
    /// Provider should call periodically to prove GPU is online.
    ///
    /// # Arguments
    /// * `gpu_id` - GPU ID
    pub async fn report_uptime(&self, gpu_id: GpuId) -> Result<bool, Box<dyn std::error::Error>> {
        todo!("Implement with substrate-subxt")
    }

    /// Query provider earnings history
    ///
    /// # Arguments
    /// * `provider` - Provider account address
    ///
    /// # Returns
    /// Earnings data
    pub async fn get_provider_earnings(
        &self,
        provider: &str,
    ) -> Result<ProviderEarnings, Box<dyn std::error::Error>> {
        todo!("Implement with substrate-subxt")
    }

    /// Penalize provider for poor performance (validator/governance only)
    ///
    /// Slashes percentage of provider's stake.
    ///
    /// # Arguments
    /// * `gpu_id` - GPU ID to slash
    ///
    /// # Returns
    /// Amount slashed
    pub async fn slash_provider(&self, gpu_id: GpuId) -> Result<Balance, Box<dyn std::error::Error>> {
        todo!("Implement with substrate-subxt")
    }

    /// Update GPU status (active/paused)
    ///
    /// # Arguments
    /// * `gpu_id` - GPU ID
    /// * `status` - New status
    pub async fn update_status(
        &self,
        gpu_id: GpuId,
        status: GpuStatus,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        todo!("Implement with substrate-subxt")
    }
}

impl Default for GpuRegistryWrapper {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reputation_calculations() {
        let rep = Reputation {
            jobs_completed: 95,
            jobs_failed: 5,
            uptime_bps: 9850, // 98.50%
            rating: 45000,    // 4.5 stars
            rating_count: 20,
        };

        assert_eq!(rep.uptime_percent(), 98.5);
        assert_eq!(rep.rating_stars(), 4.5);
        assert_eq!(rep.success_rate(), 95.0);
    }

    #[test]
    fn test_gpu_specs_creation() {
        let specs = GpuSpecs {
            model: "RTX 4090".to_string(),
            vram_gb: 24,
            compute_units: 16384,
            clock_speed_mhz: 2520,
            tdp_watts: 450,
        };

        assert_eq!(specs.model, "RTX 4090");
        assert_eq!(specs.vram_gb, 24);
    }

    #[test]
    fn test_availability_schedule() {
        let always_on = AvailabilitySchedule::AlwaysOn;
        let business_hours = AvailabilitySchedule::BusinessHours;
        let custom = AvailabilitySchedule::Custom(vec![0u8; 21]);

        assert!(matches!(always_on, AvailabilitySchedule::AlwaysOn));
        assert!(matches!(business_hours, AvailabilitySchedule::BusinessHours));
        assert!(matches!(custom, AvailabilitySchedule::Custom(_)));
    }
}
