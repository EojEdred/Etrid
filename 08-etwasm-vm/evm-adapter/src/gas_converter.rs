use crate::error::{AdapterError, AdapterResult};

/// Default conversion ratio from EVM gas (gwei) to VMw.
pub const DEFAULT_GWEI_TO_VMW_RATIO: u64 = 10_000;

/// Converts EVM gas units into ETWasm VM watts (VMw).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct GasConverter {
    ratio: u64,
}

impl GasConverter {
    /// Create a new converter with the default ratio (1 gwei = 10_000 VMw).
    pub fn new() -> Self {
        Self { ratio: DEFAULT_GWEI_TO_VMW_RATIO }
    }

    /// Create a converter with a custom ratio.
    pub fn with_ratio(ratio: u64) -> Self {
        Self { ratio }
    }

    /// Convert EVM gas units into VMw.
    pub fn gwei_to_vmw(&self, gas: u64) -> AdapterResult<u128> {
        let vmw = (gas as u128)
            .checked_mul(self.ratio as u128)
            .ok_or(AdapterError::GasOverflow)?;
        Ok(vmw)
    }

    /// Convert VMw back to gwei for reporting.
    pub fn vmw_to_gwei(&self, vmw: u128) -> AdapterResult<u64> {
        if self.ratio == 0 {
            return Err(AdapterError::GasOverflow);
        }
        let gas = vmw
            .checked_div(self.ratio as u128)
            .ok_or(AdapterError::GasOverflow)?
            as u64;
        Ok(gas)
    }
}

impl Default for GasConverter {
    fn default() -> Self {
        Self::new()
    }
}
