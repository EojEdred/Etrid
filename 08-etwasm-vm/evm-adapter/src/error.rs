use core::fmt;

/// Common result type for the adapter components.
pub type AdapterResult<T> = Result<T, AdapterError>;

/// Errors surfaced by the EVM compatibility adapter.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum AdapterError {
    /// Execution consumed all allotted gas.
    OutOfGas,
    /// Encountered an opcode we do not support yet.
    InvalidOpcode(u8),
    /// Stack did not have enough values for the requested operation.
    StackUnderflow,
    /// Provided calldata is malformed or shorter than expected.
    MalformedCalldata,
    /// Gas conversion failed because of an overflow.
    GasOverflow,
    /// Mapping errors for storage/account conversions.
    StorageMappingFailed,
    /// Selector not recognized for translation.
    UnknownSelector,
    /// Parameter type is unsupported for translation.
    UnsupportedType,
    /// Numeric value exceeds supported range.
    ValueOverflow,
}

impl fmt::Display for AdapterError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AdapterError::OutOfGas => write!(f, "execution exhausted available gas"),
            AdapterError::InvalidOpcode(op) => write!(f, "invalid opcode 0x{op:02x}"),
            AdapterError::StackUnderflow => write!(f, "stack underflow"),
            AdapterError::MalformedCalldata => write!(f, "malformed calldata"),
            AdapterError::GasOverflow => write!(f, "gas conversion overflow"),
            AdapterError::StorageMappingFailed => write!(f, "storage mapping failed"),
            AdapterError::UnknownSelector => write!(f, "unknown selector"),
            AdapterError::UnsupportedType => write!(f, "unsupported ABI type"),
            AdapterError::ValueOverflow => write!(f, "value exceeds supported range"),
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for AdapterError {}
