#![cfg_attr(not(feature = "std"), no_std)]

use codec::{Decode, Encode};
use scale_info::TypeInfo;
use sp_std::prelude::*;

/// Balance type - used for all currency amounts
pub type Balance = u128;

/// ============================================================================
/// CURRENCY DENOMINATION SYSTEM
/// ============================================================================
/// 
/// Étrid uses a hierarchical denomination system with 9 levels:
/// - Bite (smallest, atomic unit)
/// - Tribite, Quadrite, Octobite, Sextobite (intermediate)
/// - Étrid (base unit, 1 ÉTR = 100,000 Bite)
/// - KiloÉtrid, MegaÉtrid, GigaÉtrid (large units)
///
/// This provides precision from nano-scale up to giga-scale in a single system

/// Bite - smallest atomic unit (1 ÉTR = 100,000 Bite)
pub const ONE_BITE: Balance = 1;

/// Tribite = 10 Bite
pub const ONE_TRIBITE: Balance = 10;

/// Quadrite = 100 Bite
pub const ONE_QUADRITE: Balance = 100;

/// Octobite = 1,000 Bite
pub const ONE_OCTOBITE: Balance = 1_000;

/// Sextobite = 10,000 Bite
pub const ONE_SEXTOBITE: Balance = 10_000;

/// Étrid (ÉTR) - base unit = 100,000 Bite
pub const ONE_ETRID: Balance = 100_000;

/// KiloÉtrid = 1,000 Étrid = 100,000,000 Bite
pub const ONE_KILO_ETRID: Balance = 100_000_000;

/// MegaÉtrid = 1,000,000 Étrid = 100,000,000,000 Bite
pub const ONE_MEGA_ETRID: Balance = 100_000_000_000;

/// GigaÉtrid = 1,000,000,000 Étrid = 100,000,000,000,000 Bite
pub const ONE_GIGA_ETRID: Balance = 100_000_000_000_000;

/// ============================================================================
/// TOTAL SUPPLY LIMITS
/// ============================================================================

/// Total ÉTR supply - 1 billion ÉTR (in atomic units)
pub const TOTAL_ETRID_SUPPLY: Balance = 1_000_000_000 * ONE_ETRID;

/// Total ETD (Étrid Dollar Stablecoin) supply - 2.5 billion ETD
pub const TOTAL_ETD_SUPPLY: Balance = 2_500_000_000 * ONE_ETRID;

/// ============================================================================
/// CURRENCY TYPE ENUMERATION
/// ============================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq, Encode, Decode, TypeInfo)]
pub enum CurrencyType {
    /// Étrid - primary token for payments, staking, governance
    Etrid,
    /// Étrid Dollar - stablecoin pegged 1:1 to USD
    EtridDollar,
    /// VMw - non-tradable gas units for computation
    VMw,
}

impl CurrencyType {
    /// Get human-readable name
    pub fn name(&self) -> &'static str {
        match self {
            CurrencyType::Etrid => "Étrid",
            CurrencyType::EtridDollar => "Étrid Dollar",
            CurrencyType::VMw => "Virtual Machine Watts",
        }
    }

    /// Get symbol
    pub fn symbol(&self) -> &'static str {
        match self {
            CurrencyType::Etrid => "ÉTR",
            CurrencyType::EtridDollar => "ETD",
            CurrencyType::VMw => "VMw",
        }
    }

    /// Check if currency is tradable on exchanges
    pub fn is_tradable(&self) -> bool {
        matches!(self, CurrencyType::Etrid | CurrencyType::EtridDollar)
    }
}

/// ============================================================================
/// DENOMINATION UNIT TYPE
/// ============================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq, Encode, Decode, TypeInfo)]
pub enum CurrencyUnit {
    /// 1 Bite (atomic unit)
    Bite,
    /// 10 Bite
    Tribite,
    /// 100 Bite
    Quadrite,
    /// 1,000 Bite
    Octobite,
    /// 10,000 Bite
    Sextobite,
    /// 100,000 Bite (base unit)
    Etrid,
    /// 100,000,000 Bite
    KiloEtrid,
    /// 100,000,000,000 Bite
    MegaEtrid,
    /// 100,000,000,000,000 Bite
    GigaEtrid,
}

impl CurrencyUnit {
    /// Get the conversion factor to atomic units (Bite)
    pub fn to_bite(&self) -> Balance {
        match self {
            CurrencyUnit::Bite => ONE_BITE,
            CurrencyUnit::Tribite => ONE_TRIBITE,
            CurrencyUnit::Quadrite => ONE_QUADRITE,
            CurrencyUnit::Octobite => ONE_OCTOBITE,
            CurrencyUnit::Sextobite => ONE_SEXTOBITE,
            CurrencyUnit::Etrid => ONE_ETRID,
            CurrencyUnit::KiloEtrid => ONE_KILO_ETRID,
            CurrencyUnit::MegaEtrid => ONE_MEGA_ETRID,
            CurrencyUnit::GigaEtrid => ONE_GIGA_ETRID,
        }
    }

    /// Get human-readable name
    pub fn name(&self) -> &'static str {
        match self {
            CurrencyUnit::Bite => "Bite",
            CurrencyUnit::Tribite => "Tribite",
            CurrencyUnit::Quadrite => "Quadrite",
            CurrencyUnit::Octobite => "Octobite",
            CurrencyUnit::Sextobite => "Sextobite",
            CurrencyUnit::Etrid => "Étrid",
            CurrencyUnit::KiloEtrid => "KiloÉtrid",
            CurrencyUnit::MegaEtrid => "MegaÉtrid",
            CurrencyUnit::GigaEtrid => "GigaÉtrid",
        }
    }

    /// Convert amount from this unit to another unit
    pub fn convert_to(&self, amount: Balance, target: CurrencyUnit) -> Balance {
        let in_bite = amount.saturating_mul(self.to_bite());
        in_bite.saturating_div(target.to_bite().max(1))
    }
}

/// ============================================================================
/// ECONOMICS CALCULATOR
/// ============================================================================

#[derive(Debug, Clone, Encode, Decode, TypeInfo)]
pub struct EconomicsCalculator {
    /// Total ÉTR minted so far
    pub etrid_minted: Balance,
    /// Total ETD minted so far
    pub etd_minted: Balance,
}

impl EconomicsCalculator {
    /// Create new economics calculator
    pub fn new() -> Self {
        Self {
            etrid_minted: 0,
            etd_minted: 0,
        }
    }

    /// Check if ÉTR mint would exceed cap
    pub fn can_mint_etrid(&self, amount: Balance) -> bool {
        self.etrid_minted.saturating_add(amount) <= TOTAL_ETRID_SUPPLY
    }

    /// Check if ETD mint would exceed cap
    pub fn can_mint_etd(&self, amount: Balance) -> bool {
        self.etd_minted.saturating_add(amount) <= TOTAL_ETD_SUPPLY
    }

    /// Record ÉTR mint
    pub fn mint_etrid(&mut self, amount: Balance) -> Result<(), &'static str> {
        if self.can_mint_etrid(amount) {
            self.etrid_minted = self.etrid_minted.saturating_add(amount);
            Ok(())
        } else {
            Err("ÉTR supply cap exceeded")
        }
    }

    /// Record ETD mint
    pub fn mint_etd(&mut self, amount: Balance) -> Result<(), &'static str> {
        if self.can_mint_etd(amount) {
            self.etd_minted = self.etd_minted.saturating_add(amount);
            Ok(())
        } else {
            Err("ETD supply cap exceeded")
        }
    }

    /// Record ÉTR burn
    pub fn burn_etrid(&mut self, amount: Balance) -> Result<(), &'static str> {
        if self.etrid_minted >= amount {
            self.etrid_minted = self.etrid_minted.saturating_sub(amount);
            Ok(())
        } else {
            Err("Cannot burn more than minted")
        }
    }

    /// Record ETD burn
    pub fn burn_etd(&mut self, amount: Balance) -> Result<(), &'static str> {
        if self.etd_minted >= amount {
            self.etd_minted = self.etd_minted.saturating_sub(amount);
            Ok(())
        } else {
            Err("Cannot burn more than minted")
        }
    }

    /// Get remaining ÉTR mint capacity
    pub fn etrid_remaining(&self) -> Balance {
        TOTAL_ETRID_SUPPLY.saturating_sub(self.etrid_minted)
    }

    /// Get remaining ETD mint capacity
    pub fn etd_remaining(&self) -> Balance {
        TOTAL_ETD_SUPPLY.saturating_sub(self.etd_minted)
    }

    /// Get percentage of ÉTR supply minted
    pub fn etrid_minted_percentage(&self) -> u32 {
        if TOTAL_ETRID_SUPPLY == 0 {
            return 0;
        }
        ((self.etrid_minted as u64 * 100) / (TOTAL_ETRID_SUPPLY as u64)) as u32
    }

    /// Get percentage of ETD supply minted
    pub fn etd_minted_percentage(&self) -> u32 {
        if TOTAL_ETD_SUPPLY == 0 {
            return 0;
        }
        ((self.etd_minted as u64 * 100) / (TOTAL_ETD_SUPPLY as u64)) as u32
    }
}

impl Default for EconomicsCalculator {
    fn default() -> Self {
        Self::new()
    }
}

/// ============================================================================
/// GENESIS DISTRIBUTION
/// ============================================================================

#[derive(Debug, Clone, Encode, Decode, TypeInfo)]
pub struct GenesisAllocation {
    /// Account identifier (Vec<u8> for flexibility)
    pub account: Vec<u8>,
    /// Amount of ÉTR to allocate
    pub etrid_amount: Balance,
    /// Amount of ETD to allocate (if any)
    pub etd_amount: Balance,
}

/// Default genesis configuration
/// Total ÉTR: 1B (1,000,000,000)
/// - Alice: 10M (1%)
/// - Bob: 10M (1%)
/// - Charlie: 10M (1%)
/// - Treasury: 970M (97%)
pub fn get_default_genesis_config() -> Vec<GenesisAllocation> {
    vec![
        GenesisAllocation {
            account: b"alice".to_vec(),
            etrid_amount: 10_000_000 * ONE_ETRID,
            etd_amount: 0,
        },
        GenesisAllocation {
            account: b"bob".to_vec(),
            etrid_amount: 10_000_000 * ONE_ETRID,
            etd_amount: 0,
        },
        GenesisAllocation {
            account: b"charlie".to_vec(),
            etrid_amount: 10_000_000 * ONE_ETRID,
            etd_amount: 0,
        },
        GenesisAllocation {
            account: b"treasury".to_vec(),
            etrid_amount: 970_000_000 * ONE_ETRID,
            etd_amount: 0,
        },
    ]
}

/// ============================================================================
/// TESTS
/// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_denomination_conversion() {
        // 1 ÉTR = 100,000 Bite
        assert_eq!(ONE_ETRID, 100_000);

        // 1 KiloÉtrid = 1,000 Étrid
        assert_eq!(ONE_KILO_ETRID, 100_000_000);

        // 1 MegaÉtrid = 1,000,000 Étrid
        assert_eq!(ONE_MEGA_ETRID, 100_000_000_000);

        // 1 GigaÉtrid = 1,000,000,000 Étrid
        assert_eq!(ONE_GIGA_ETRID, 100_000_000_000_000);
    }

    #[test]
    fn test_currency_unit_conversion() {
        // Convert 1 ÉTR to Bite
        let bite = CurrencyUnit::Etrid.convert_to(1, CurrencyUnit::Bite);
        assert_eq!(bite, 100_000);

        // Convert 100,000 Bite to ÉTR
        let etrid = CurrencyUnit::Bite.convert_to(100_000, CurrencyUnit::Etrid);
        assert_eq!(etrid, 1);

        // Convert 1 KiloÉtrid to ÉTR
        let etrid = CurrencyUnit::KiloEtrid.convert_to(1, CurrencyUnit::Etrid);
        assert_eq!(etrid, 1_000);
    }

    #[test]
    fn test_economics_calculator_mint() {
        let mut calc = EconomicsCalculator::new();

        // Can mint within cap
        assert!(calc.can_mint_etrid(100 * ONE_ETRID));
        assert!(calc.mint_etrid(100 * ONE_ETRID).is_ok());
        assert_eq!(calc.etrid_minted, 100 * ONE_ETRID);

        // Cannot mint beyond cap
        assert!(!calc.can_mint_etrid(TOTAL_ETRID_SUPPLY));
        assert!(calc.mint_etrid(TOTAL_ETRID_SUPPLY).is_err());
    }

    #[test]
    fn test_economics_calculator_burn() {
        let mut calc = EconomicsCalculator::new();

        calc.mint_etrid(100 * ONE_ETRID).unwrap();
        assert_eq!(calc.etrid_minted, 100 * ONE_ETRID);

        calc.burn_etrid(50 * ONE_ETRID).unwrap();
        assert_eq!(calc.etrid_minted, 50 * ONE_ETRID);

        // Cannot burn more than minted
        assert!(calc.burn_etrid(100 * ONE_ETRID).is_err());
    }

    #[test]
    fn test_economics_calculator_percentage() {
        let mut calc = EconomicsCalculator::new();

        // 0% initially
        assert_eq!(calc.etrid_minted_percentage(), 0);

        // Mint 10%
        calc.mint_etrid(100_000_000 * ONE_ETRID).unwrap();
        assert_eq!(calc.etrid_minted_percentage(), 10);

        // Mint to 50%
        calc.mint_etrid(400_000_000 * ONE_ETRID).unwrap();
        assert_eq!(calc.etrid_minted_percentage(), 50);
    }

    #[test]
    fn test_currency_type_properties() {
        assert_eq!(CurrencyType::Etrid.name(), "Étrid");
        assert_eq!(CurrencyType::Etrid.symbol(), "ÉTR");
        assert!(CurrencyType::Etrid.is_tradable());

        assert_eq!(CurrencyType::VMw.name(), "Virtual Machine Watts");
        assert_eq!(CurrencyType::VMw.symbol(), "VMw");
        assert!(!CurrencyType::VMw.is_tradable());
    }

    #[test]
    fn test_genesis_config() {
        let genesis = get_default_genesis_config();
        
        // Should have 4 accounts
        assert_eq!(genesis.len(), 4);

        // Total should be 1B ÉTR
        let total: Balance = genesis.iter().map(|a| a.etrid_amount).sum();
        assert_eq!(total, 1_000_000_000 * ONE_ETRID);

        // Alice should have 10M
        assert_eq!(genesis[0].etrid_amount, 10_000_000 * ONE_ETRID);
    }

    #[test]
    fn test_total_supply_constants() {
        // Verify total supply is exactly 1B ÉTR
        assert_eq!(TOTAL_ETRID_SUPPLY, 1_000_000_000 * ONE_ETRID);

        // Verify ETD supply is exactly 2.5B ETD
        assert_eq!(TOTAL_ETD_SUPPLY, 2_500_000_000 * ONE_ETRID);
    }

    #[test]
    fn test_remaining_capacity() {
        let mut calc = EconomicsCalculator::new();

        // Initially all supply remaining
        assert_eq!(calc.etrid_remaining(), TOTAL_ETRID_SUPPLY);

        // Mint 100M
        calc.mint_etrid(100_000_000 * ONE_ETRID).unwrap();

        // Remaining should be 900M
        assert_eq!(calc.etrid_remaining(), 900_000_000 * ONE_ETRID);
    }
}
