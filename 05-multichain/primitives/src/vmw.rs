//! VM Watts (VMw) - Ëtrid Virtual Machine Computation Units
//!
//! From Ivory Papers:
//! - VMw Available: Total VMw paid by sender for execution
//! - VMw OP_Price: Total price of VMw for execution
//! - VMw Wattage Limit: Max VM watts per block
//! - VMw Watts Used: Total consumed VM watts

use codec::{Decode, Encode};
use scale_info::TypeInfo;
use sp_runtime::RuntimeDebug;

#[cfg(feature = "std")]
use serde::{Deserialize, Serialize};

/// VM Watts - computation unit for ËtwasmVM
#[derive(
    Clone,
    Copy,
    Encode,
    Decode,
    Eq,
    PartialEq,
    Ord,
    PartialOrd,
    RuntimeDebug,
    TypeInfo,
    Default,
)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct VMw(pub u64);

impl VMw {
    /// Create new VMw amount
    pub const fn new(amount: u64) -> Self {
        Self(amount)
    }

    /// Zero VMw
    pub const fn zero() -> Self {
        Self(0)
    }

    /// Maximum VMw
    pub const fn max() -> Self {
        Self(u64::MAX)
    }

    /// Get raw value
    pub const fn get(&self) -> u64 {
        self.0
    }

    /// Add VMw (saturating)
    pub fn saturating_add(self, other: Self) -> Self {
        Self(self.0.saturating_add(other.0))
    }

    /// Subtract VMw (saturating)
    pub fn saturating_sub(self, other: Self) -> Self {
        Self(self.0.saturating_sub(other.0))
    }

    /// Multiply VMw by scalar (saturating)
    pub fn saturating_mul(self, scalar: u64) -> Self {
        Self(self.0.saturating_mul(scalar))
    }

    /// Check if sufficient VMw available
    pub fn is_sufficient(&self, required: Self) -> bool {
        self.0 >= required.0
    }
}

impl core::ops::Add for VMw {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}

impl core::ops::Sub for VMw {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0)
    }
}

impl core::ops::Mul<u64> for VMw {
    type Output = Self;
    fn mul(self, rhs: u64) -> Self::Output {
        Self(self.0 * rhs)
    }
}

impl From<u64> for VMw {
    fn from(value: u64) -> Self {
        Self(value)
    }
}

impl From<VMw> for u64 {
    fn from(value: VMw) -> Self {
        value.0
    }
}

/// VMw pricing configuration
#[derive(Clone, Copy, Encode, Decode, Eq, PartialEq, RuntimeDebug, TypeInfo)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct VMwPrice {
    /// Price per VMw unit
    pub price_per_unit: u128,
    /// Minimum VMw required
    pub minimum_vmw: VMw,
}

impl Default for VMwPrice {
    fn default() -> Self {
        Self {
            price_per_unit: 1_000_000_000_000, // 0.000001 ËTR per VMw
            minimum_vmw: VMw::new(21_000),      // Minimum transaction cost
        }
    }
}

/// VMw metering result
#[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, TypeInfo)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct VMwMetering {
    /// VMw allocated for execution
    pub vmw_available: VMw,
    /// VMw actually consumed
    pub vmw_used: VMw,
    /// VMw refunded (unused)
    pub vmw_refund: VMw,
    /// Total cost in native currency
    pub total_cost: u128,
}

impl VMwMetering {
    /// Create new metering record
    pub fn new(available: VMw, used: VMw, price_per_unit: u128) -> Self {
        let refund = available.saturating_sub(used);
        let total_cost = (used.get() as u128).saturating_mul(price_per_unit);

        Self {
            vmw_available: available,
            vmw_used: used,
            vmw_refund: refund,
            total_cost,
        }
    }

    /// Check if execution succeeded
    pub fn is_success(&self) -> bool {
        self.vmw_used.get() <= self.vmw_available.get()
    }
}

/// VMw limits per block
#[derive(Clone, Copy, Encode, Decode, Eq, PartialEq, RuntimeDebug, TypeInfo)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct BlockVMwLimit {
    /// Maximum VMw per block
    pub max_vmw: VMw,
    /// Target VMw per block (for adjustment)
    pub target_vmw: VMw,
}

impl Default for BlockVMwLimit {
    fn default() -> Self {
        Self {
            max_vmw: VMw::new(30_000_000),    // 30M VMw per block
            target_vmw: VMw::new(15_000_000), // 15M target (50% utilization)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn vmw_operations_work() {
        let vmw1 = VMw::new(100);
        let vmw2 = VMw::new(50);

        assert_eq!(vmw1 + vmw2, VMw::new(150));
        assert_eq!(vmw1 - vmw2, VMw::new(50));
        assert_eq!(vmw1 * 2, VMw::new(200));
    }

    #[test]
    fn vmw_saturating_works() {
        let vmw = VMw::new(u64::MAX);
        assert_eq!(vmw.saturating_add(VMw::new(1)), VMw::max());
    }

    #[test]
    fn vmw_metering_works() {
        let metering = VMwMetering::new(
            VMw::new(100_000),
            VMw::new(75_000),
            1_000_000_000_000,
        );

        assert_eq!(metering.vmw_refund, VMw::new(25_000));
        assert!(metering.is_success());
    }
}
