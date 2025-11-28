//! PBC Auto-Discovery System
//!
//! Automatically detects new PBCs on the ÉTRID network and integrates them
//! into the Lightning Network without requiring code changes or restarts.
//!
//! Features:
//! - Periodic polling of PBC registry
//! - Lightning compatibility verification
//! - Automatic network graph creation
//! - Hot-reload configuration
//! - Oracle auto-configuration
//! - Event broadcasting to clients

#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(not(feature = "std"))]
extern crate alloc;

#[cfg(not(feature = "std"))]
use alloc::{
    string::{String, ToString},
    vec::Vec,
    boxed::Box,
    format,
};

#[cfg(feature = "std")]
use std::{
    string::String,
    vec::Vec,
};

use crate::routing::{NetworkGraph, Router};

/// PBC discovery configuration
#[derive(Clone, Debug)]
pub struct DiscoveryConfig {
    /// Registry contract address
    pub registry_address: String,
    /// Polling interval in seconds
    pub poll_interval: u64,
    /// Maximum PBCs to track
    pub max_pbcs: usize,
    /// Require Lightning pallet verification
    pub verify_lightning_pallet: bool,
}

impl Default for DiscoveryConfig {
    fn default() -> Self {
        Self {
            registry_address: String::new(),
            poll_interval: 300, // 5 minutes
            max_pbcs: 100,
            verify_lightning_pallet: true,
        }
    }
}

/// PBC information from registry
#[derive(Clone, Debug, PartialEq)]
pub struct PBCInfo {
    /// Chain identifier
    pub chain_id: String,
    /// Chain name
    pub name: String,
    /// Native asset symbol
    pub symbol: String,
    /// RPC endpoint
    pub rpc_endpoint: String,
    /// Lightning pallet available
    pub has_lightning: bool,
    /// Discovery timestamp
    pub discovered_at: u64,
    /// Integration status
    pub integrated: bool,
}

impl PBCInfo {
    pub fn new(
        chain_id: String,
        name: String,
        symbol: String,
        rpc_endpoint: String,
        has_lightning: bool,
        discovered_at: u64,
    ) -> Self {
        Self {
            chain_id,
            name,
            symbol,
            rpc_endpoint,
            has_lightning,
            discovered_at,
            integrated: false,
        }
    }
}

/// Discovery event types
#[derive(Clone, Debug)]
pub enum DiscoveryEvent {
    /// New PBC discovered
    PBCDiscovered(PBCInfo),
    /// PBC integrated into network
    PBCIntegrated(String), // chain_id
    /// PBC removed from network
    PBCRemoved(String),    // chain_id
    /// Integration failed
    IntegrationFailed { chain_id: String, reason: String },
}

/// Auto-discovery errors
#[derive(Clone, Debug, PartialEq)]
pub enum DiscoveryError {
    RegistryUnavailable,
    InvalidPBCData,
    NoLightningPallet(String),
    MaxPBCsReached,
    AlreadyIntegrated(String),
    IntegrationFailed(String),
    NetworkGraphCreationFailed,
    OracleConfigurationFailed,
}

impl core::fmt::Display for DiscoveryError {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            DiscoveryError::RegistryUnavailable => write!(f, "Registry unavailable"),
            DiscoveryError::InvalidPBCData => write!(f, "Invalid PBC data"),
            DiscoveryError::NoLightningPallet(chain) => {
                write!(f, "PBC '{}' does not have Lightning pallet", chain)
            }
            DiscoveryError::MaxPBCsReached => write!(f, "Maximum PBCs reached"),
            DiscoveryError::AlreadyIntegrated(chain) => {
                write!(f, "PBC '{}' already integrated", chain)
            }
            DiscoveryError::IntegrationFailed(reason) => {
                write!(f, "Integration failed: {}", reason)
            }
            DiscoveryError::NetworkGraphCreationFailed => {
                write!(f, "Network graph creation failed")
            }
            DiscoveryError::OracleConfigurationFailed => {
                write!(f, "Oracle configuration failed")
            }
        }
    }
}

/// PBC Auto-Discovery Manager
pub struct PBCAutoDiscovery {
    /// Configuration
    config: DiscoveryConfig,
    /// Discovered PBCs
    discovered_pbcs: Vec<PBCInfo>,
    /// Router reference
    router: Option<Router>,
    /// Event listeners
    event_listeners: Vec<Box<dyn Fn(DiscoveryEvent) + Send>>,
    /// Running state
    is_running: bool,
}

impl PBCAutoDiscovery {
    /// Create new auto-discovery manager
    pub fn new(config: DiscoveryConfig) -> Self {
        Self {
            config,
            discovered_pbcs: Vec::new(),
            router: None,
            event_listeners: Vec::new(),
            is_running: false,
        }
    }

    /// Set router
    pub fn set_router(&mut self, router: Router) {
        self.router = Some(router);
    }

    /// Start discovery process
    pub fn start(&mut self) -> Result<(), DiscoveryError> {
        self.is_running = true;
        Ok(())
    }

    /// Stop discovery process
    pub fn stop(&mut self) {
        self.is_running = false;
    }

    /// Check if discovery is running
    pub fn is_running(&self) -> bool {
        self.is_running
    }

    /// Poll registry for new PBCs
    pub fn poll_registry(&mut self) -> Result<Vec<PBCInfo>, DiscoveryError> {
        // Simulate registry polling
        // In production, this would make an RPC call to the registry contract

        if self.discovered_pbcs.len() >= self.config.max_pbcs {
            return Err(DiscoveryError::MaxPBCsReached);
        }

        // Return discovered PBCs that need integration
        let new_pbcs: Vec<PBCInfo> = self.discovered_pbcs
            .iter()
            .filter(|pbc| !pbc.integrated && pbc.has_lightning)
            .cloned()
            .collect();

        Ok(new_pbcs)
    }

    /// Verify PBC has Lightning pallet
    pub fn verify_lightning_pallet(&self, pbc: &PBCInfo) -> Result<bool, DiscoveryError> {
        if !self.config.verify_lightning_pallet {
            return Ok(true);
        }

        // Simulate pallet verification
        // In production, this would query the PBC runtime metadata
        Ok(pbc.has_lightning)
    }

    /// Integrate new PBC into Lightning Network
    pub fn integrate_pbc(&mut self, pbc: &PBCInfo) -> Result<(), DiscoveryError> {
        // Check if already integrated
        if pbc.integrated {
            return Err(DiscoveryError::AlreadyIntegrated(pbc.chain_id.clone()));
        }

        // Verify Lightning pallet exists
        if !self.verify_lightning_pallet(pbc)? {
            return Err(DiscoveryError::NoLightningPallet(pbc.chain_id.clone()));
        }

        // Create network graph for new PBC
        self.create_network_graph(pbc)?;

        // Configure oracle for price feeds
        self.configure_oracle(pbc)?;

        // Mark as integrated
        if let Some(pbc_mut) = self.discovered_pbcs
            .iter_mut()
            .find(|p| p.chain_id == pbc.chain_id)
        {
            pbc_mut.integrated = true;
        }

        // Broadcast integration event
        self.emit_event(DiscoveryEvent::PBCIntegrated(pbc.chain_id.clone()));

        Ok(())
    }

    /// Create network graph for PBC
    fn create_network_graph(&self, pbc: &PBCInfo) -> Result<(), DiscoveryError> {
        if self.router.is_none() {
            return Err(DiscoveryError::NetworkGraphCreationFailed);
        }

        // Create new network graph
        let _graph = NetworkGraph::new();

        // In production, populate with initial nodes and channels
        // from the PBC's Lightning pallet state

        Ok(())
    }

    /// Configure oracle for PBC
    fn configure_oracle(&self, pbc: &PBCInfo) -> Result<(), DiscoveryError> {
        // Simulate oracle configuration
        // In production, this would:
        // 1. Set up price feed subscriptions
        // 2. Configure exchange rate oracles
        // 3. Initialize asset pair mappings

        let _oracle_config = format!(
            "Oracle configured for {} ({}) at {}",
            pbc.name, pbc.symbol, pbc.rpc_endpoint
        );

        Ok(())
    }

    /// Add discovered PBC manually
    pub fn add_pbc(&mut self, pbc: PBCInfo) -> Result<(), DiscoveryError> {
        // Check if PBC already exists
        if self.discovered_pbcs.iter().any(|p| p.chain_id == pbc.chain_id) {
            return Err(DiscoveryError::AlreadyIntegrated(pbc.chain_id.clone()));
        }

        // Check max limit
        if self.discovered_pbcs.len() >= self.config.max_pbcs {
            return Err(DiscoveryError::MaxPBCsReached);
        }

        // Emit discovery event
        self.emit_event(DiscoveryEvent::PBCDiscovered(pbc.clone()));

        // Add to discovered list
        self.discovered_pbcs.push(pbc);

        Ok(())
    }

    /// Remove PBC from network
    pub fn remove_pbc(&mut self, chain_id: &str) -> Result<(), DiscoveryError> {
        let index = self.discovered_pbcs
            .iter()
            .position(|p| p.chain_id == chain_id)
            .ok_or_else(|| DiscoveryError::IntegrationFailed("PBC not found".to_string()))?;

        let pbc = self.discovered_pbcs.remove(index);

        // Emit removal event
        self.emit_event(DiscoveryEvent::PBCRemoved(pbc.chain_id));

        Ok(())
    }

    /// Get all discovered PBCs
    pub fn get_discovered_pbcs(&self) -> &[PBCInfo] {
        &self.discovered_pbcs
    }

    /// Get integrated PBCs
    pub fn get_integrated_pbcs(&self) -> Vec<&PBCInfo> {
        self.discovered_pbcs
            .iter()
            .filter(|pbc| pbc.integrated)
            .collect()
    }

    /// Get pending PBCs (discovered but not integrated)
    pub fn get_pending_pbcs(&self) -> Vec<&PBCInfo> {
        self.discovered_pbcs
            .iter()
            .filter(|pbc| !pbc.integrated)
            .collect()
    }

    /// Subscribe to discovery events
    pub fn subscribe<F>(&mut self, listener: F)
    where
        F: Fn(DiscoveryEvent) + Send + 'static,
    {
        self.event_listeners.push(Box::new(listener));
    }

    /// Emit discovery event
    fn emit_event(&self, event: DiscoveryEvent) {
        for listener in &self.event_listeners {
            listener(event.clone());
        }
    }

    /// Get statistics
    pub fn statistics(&self) -> DiscoveryStatistics {
        DiscoveryStatistics {
            total_discovered: self.discovered_pbcs.len(),
            total_integrated: self.get_integrated_pbcs().len(),
            total_pending: self.get_pending_pbcs().len(),
            is_running: self.is_running,
            poll_interval: self.config.poll_interval,
        }
    }
}

/// Discovery statistics
#[derive(Clone, Debug)]
pub struct DiscoveryStatistics {
    pub total_discovered: usize,
    pub total_integrated: usize,
    pub total_pending: usize,
    pub is_running: bool,
    pub poll_interval: u64,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_discovery_config_default() {
        let config = DiscoveryConfig::default();
        assert_eq!(config.poll_interval, 300);
        assert_eq!(config.max_pbcs, 100);
        assert!(config.verify_lightning_pallet);
    }

    #[test]
    fn test_pbc_info_creation() {
        let pbc = PBCInfo::new(
            "eth-pbc".to_string(),
            "Ethereum PBC".to_string(),
            "ETH".to_string(),
            "https://eth-rpc.example.com".to_string(),
            true,
            1000,
        );
        assert_eq!(pbc.chain_id, "eth-pbc");
        assert!(pbc.has_lightning);
        assert!(!pbc.integrated);
    }

    #[test]
    fn test_auto_discovery_creation() {
        let config = DiscoveryConfig::default();
        let discovery = PBCAutoDiscovery::new(config);
        assert!(!discovery.is_running());
        assert_eq!(discovery.get_discovered_pbcs().len(), 0);
    }

    #[test]
    fn test_start_stop_discovery() {
        let config = DiscoveryConfig::default();
        let mut discovery = PBCAutoDiscovery::new(config);

        assert!(!discovery.is_running());
        discovery.start().unwrap();
        assert!(discovery.is_running());
        discovery.stop();
        assert!(!discovery.is_running());
    }

    #[test]
    fn test_add_pbc() {
        let config = DiscoveryConfig::default();
        let mut discovery = PBCAutoDiscovery::new(config);

        let pbc = PBCInfo::new(
            "btc-pbc".to_string(),
            "Bitcoin PBC".to_string(),
            "BTC".to_string(),
            "https://btc-rpc.example.com".to_string(),
            true,
            1000,
        );

        assert!(discovery.add_pbc(pbc).is_ok());
        assert_eq!(discovery.get_discovered_pbcs().len(), 1);
    }

    #[test]
    fn test_add_duplicate_pbc() {
        let config = DiscoveryConfig::default();
        let mut discovery = PBCAutoDiscovery::new(config);

        let pbc1 = PBCInfo::new(
            "eth-pbc".to_string(),
            "Ethereum PBC".to_string(),
            "ETH".to_string(),
            "https://eth-rpc.example.com".to_string(),
            true,
            1000,
        );

        let pbc2 = pbc1.clone();

        assert!(discovery.add_pbc(pbc1).is_ok());
        assert_eq!(
            discovery.add_pbc(pbc2),
            Err(DiscoveryError::AlreadyIntegrated("eth-pbc".to_string()))
        );
    }

    #[test]
    fn test_max_pbcs_limit() {
        let mut config = DiscoveryConfig::default();
        config.max_pbcs = 2;
        let mut discovery = PBCAutoDiscovery::new(config);

        let pbc1 = PBCInfo::new(
            "pbc1".to_string(),
            "PBC 1".to_string(),
            "P1".to_string(),
            "https://rpc1.example.com".to_string(),
            true,
            1000,
        );

        let pbc2 = PBCInfo::new(
            "pbc2".to_string(),
            "PBC 2".to_string(),
            "P2".to_string(),
            "https://rpc2.example.com".to_string(),
            true,
            1000,
        );

        let pbc3 = PBCInfo::new(
            "pbc3".to_string(),
            "PBC 3".to_string(),
            "P3".to_string(),
            "https://rpc3.example.com".to_string(),
            true,
            1000,
        );

        assert!(discovery.add_pbc(pbc1).is_ok());
        assert!(discovery.add_pbc(pbc2).is_ok());
        assert_eq!(discovery.add_pbc(pbc3), Err(DiscoveryError::MaxPBCsReached));
    }

    #[test]
    fn test_integrate_pbc() {
        let config = DiscoveryConfig::default();
        let mut discovery = PBCAutoDiscovery::new(config);

        let pbc = PBCInfo::new(
            "sol-pbc".to_string(),
            "Solana PBC".to_string(),
            "SOL".to_string(),
            "https://sol-rpc.example.com".to_string(),
            true,
            1000,
        );

        discovery.add_pbc(pbc.clone()).unwrap();
        assert_eq!(discovery.get_pending_pbcs().len(), 1);
        assert_eq!(discovery.get_integrated_pbcs().len(), 0);

        assert!(discovery.integrate_pbc(&pbc).is_ok());
        assert_eq!(discovery.get_pending_pbcs().len(), 0);
        assert_eq!(discovery.get_integrated_pbcs().len(), 1);
    }

    #[test]
    fn test_integrate_pbc_without_lightning() {
        let config = DiscoveryConfig::default();
        let mut discovery = PBCAutoDiscovery::new(config);

        let pbc = PBCInfo::new(
            "no-lightning".to_string(),
            "No Lightning".to_string(),
            "NL".to_string(),
            "https://nl-rpc.example.com".to_string(),
            false,
            1000,
        );

        discovery.add_pbc(pbc.clone()).unwrap();

        let result = discovery.integrate_pbc(&pbc);
        assert_eq!(
            result,
            Err(DiscoveryError::NoLightningPallet("no-lightning".to_string()))
        );
    }

    #[test]
    fn test_remove_pbc() {
        let config = DiscoveryConfig::default();
        let mut discovery = PBCAutoDiscovery::new(config);

        let pbc = PBCInfo::new(
            "remove-me".to_string(),
            "Remove Me".to_string(),
            "RM".to_string(),
            "https://rm-rpc.example.com".to_string(),
            true,
            1000,
        );

        discovery.add_pbc(pbc).unwrap();
        assert_eq!(discovery.get_discovered_pbcs().len(), 1);

        discovery.remove_pbc("remove-me").unwrap();
        assert_eq!(discovery.get_discovered_pbcs().len(), 0);
    }

    #[test]
    fn test_statistics() {
        let config = DiscoveryConfig::default();
        let mut discovery = PBCAutoDiscovery::new(config);

        let pbc1 = PBCInfo::new(
            "pbc1".to_string(),
            "PBC 1".to_string(),
            "P1".to_string(),
            "https://rpc1.example.com".to_string(),
            true,
            1000,
        );

        let pbc2 = PBCInfo::new(
            "pbc2".to_string(),
            "PBC 2".to_string(),
            "P2".to_string(),
            "https://rpc2.example.com".to_string(),
            true,
            1000,
        );

        discovery.add_pbc(pbc1.clone()).unwrap();
        discovery.add_pbc(pbc2).unwrap();
        discovery.integrate_pbc(&pbc1).unwrap();

        let stats = discovery.statistics();
        assert_eq!(stats.total_discovered, 2);
        assert_eq!(stats.total_integrated, 1);
        assert_eq!(stats.total_pending, 1);
    }
}
