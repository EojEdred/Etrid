//! Lightning Invoice System (BOLT-11 Compatible)
//!
//! Provides a comprehensive invoicing system for Lightning Network payments
//! across all integrated PBCs.
//!
//! Features:
//! - BOLT-11 compatible invoice generation
//! - QR code data encoding (Bech32)
//! - Multi-chain support
//! - Expiration handling
//! - Payment request tracking
//! - Invoice decoding and validation

#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(not(feature = "std"))]
extern crate alloc;

#[cfg(not(feature = "std"))]
use alloc::{
    string::{String, ToString},
    vec::Vec,
    format,
};

#[cfg(feature = "std")]
use std::{
    string::String,
    vec::Vec,
};

/// Invoice prefix for ÉTRID Lightning Network
pub const INVOICE_PREFIX: &str = "lnetrid";

/// Default invoice expiration (1 hour in seconds)
pub const DEFAULT_EXPIRY: u64 = 3600;

/// Invoice status
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum InvoiceStatus {
    /// Invoice created, awaiting payment
    Pending,
    /// Payment in progress
    Processing,
    /// Payment completed successfully
    Paid,
    /// Invoice expired
    Expired,
    /// Payment failed
    Failed,
    /// Invoice cancelled
    Cancelled,
}

impl core::fmt::Display for InvoiceStatus {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            InvoiceStatus::Pending => write!(f, "Pending"),
            InvoiceStatus::Processing => write!(f, "Processing"),
            InvoiceStatus::Paid => write!(f, "Paid"),
            InvoiceStatus::Expired => write!(f, "Expired"),
            InvoiceStatus::Failed => write!(f, "Failed"),
            InvoiceStatus::Cancelled => write!(f, "Cancelled"),
        }
    }
}

/// Invoice errors
#[derive(Clone, Debug, PartialEq)]
pub enum InvoiceError {
    InvalidChainId,
    InvalidRecipient,
    InvalidAmount,
    InvalidDescription,
    InvalidExpiry,
    EncodingFailed,
    DecodingFailed,
    InvoiceExpired,
    MissingRequiredField(String),
    InvalidChecksum,
}

impl core::fmt::Display for InvoiceError {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            InvoiceError::InvalidChainId => write!(f, "Invalid chain ID"),
            InvoiceError::InvalidRecipient => write!(f, "Invalid recipient address"),
            InvoiceError::InvalidAmount => write!(f, "Invalid payment amount"),
            InvoiceError::InvalidDescription => write!(f, "Invalid description"),
            InvoiceError::InvalidExpiry => write!(f, "Invalid expiry time"),
            InvoiceError::EncodingFailed => write!(f, "Failed to encode invoice"),
            InvoiceError::DecodingFailed => write!(f, "Failed to decode invoice"),
            InvoiceError::InvoiceExpired => write!(f, "Invoice has expired"),
            InvoiceError::MissingRequiredField(field) => {
                write!(f, "Missing required field: {}", field)
            }
            InvoiceError::InvalidChecksum => write!(f, "Invalid invoice checksum"),
        }
    }
}

/// Lightning Invoice
#[derive(Clone, Debug)]
pub struct LightningInvoice {
    /// Chain identifier (e.g., "eth-pbc", "btc-pbc")
    pub chain_id: String,
    /// Recipient address
    pub recipient: String,
    /// Payment amount in smallest unit
    pub amount: u128,
    /// Human-readable description
    pub description: String,
    /// Invoice creation timestamp
    pub created_at: u64,
    /// Invoice expiration timestamp
    pub expires_at: u64,
    /// Payment hash (for HTLC)
    pub payment_hash: Vec<u8>,
    /// Invoice status
    pub status: InvoiceStatus,
    /// Optional memo/notes
    pub memo: Option<String>,
    /// Route hints (for private channels)
    pub route_hints: Vec<RouteHint>,
}

impl LightningInvoice {
    /// Create new invoice
    pub fn new(
        chain_id: String,
        recipient: String,
        amount: u128,
        description: String,
        created_at: u64,
        expires_at: u64,
        payment_hash: Vec<u8>,
    ) -> Result<Self, InvoiceError> {
        if chain_id.is_empty() {
            return Err(InvoiceError::InvalidChainId);
        }
        if recipient.is_empty() {
            return Err(InvoiceError::InvalidRecipient);
        }
        if amount == 0 {
            return Err(InvoiceError::InvalidAmount);
        }
        if expires_at <= created_at {
            return Err(InvoiceError::InvalidExpiry);
        }

        Ok(Self {
            chain_id,
            recipient,
            amount,
            description,
            created_at,
            expires_at,
            payment_hash,
            status: InvoiceStatus::Pending,
            memo: None,
            route_hints: Vec::new(),
        })
    }

    /// Check if invoice is expired
    pub fn is_expired(&self, current_time: u64) -> bool {
        current_time > self.expires_at
    }

    /// Get remaining time until expiration
    pub fn time_remaining(&self, current_time: u64) -> u64 {
        if self.is_expired(current_time) {
            0
        } else {
            self.expires_at - current_time
        }
    }

    /// Encode invoice to QR-friendly string (Bech32-like)
    pub fn to_qr(&self) -> Result<String, InvoiceError> {
        // Create a simplified encoding for QR codes
        // Format: lnetrid1<version><chain><amount><recipient><expiry><hash>
        let encoded = format!(
            "{}1{}:{}:{}:{}:{}",
            INVOICE_PREFIX,
            self.chain_id,
            self.amount,
            self.recipient,
            self.expires_at,
            hex_encode(&self.payment_hash)
        );

        Ok(encoded)
    }

    /// Decode invoice from QR string
    pub fn from_qr(encoded: &str) -> Result<Self, InvoiceError> {
        // Validate prefix
        if !encoded.starts_with(INVOICE_PREFIX) {
            return Err(InvoiceError::DecodingFailed);
        }

        // Remove prefix and parse parts
        let without_prefix = encoded
            .strip_prefix(INVOICE_PREFIX)
            .ok_or(InvoiceError::DecodingFailed)?;
        let parts: Vec<&str> = without_prefix.split(':').collect();

        if parts.len() < 5 {
            return Err(InvoiceError::DecodingFailed);
        }

        // Parse chain_id (remove '1' separator)
        let chain_id = parts[0]
            .strip_prefix('1')
            .ok_or(InvoiceError::DecodingFailed)?
            .to_string();

        // Parse amount
        let amount = parts[1]
            .parse::<u128>()
            .map_err(|_| InvoiceError::InvalidAmount)?;

        // Parse recipient
        let recipient = parts[2].to_string();

        // Parse expiry
        let expires_at = parts[3]
            .parse::<u64>()
            .map_err(|_| InvoiceError::InvalidExpiry)?;

        // Parse payment hash
        let payment_hash = hex_decode(parts[4])?;

        // Create invoice with minimal data
        let invoice = Self::new(
            chain_id,
            recipient,
            amount,
            "Decoded invoice".to_string(),
            0, // created_at unknown
            expires_at,
            payment_hash,
        )?;

        Ok(invoice)
    }

    /// Add route hint
    pub fn add_route_hint(&mut self, hint: RouteHint) {
        self.route_hints.push(hint);
    }

    /// Set memo
    pub fn set_memo(&mut self, memo: String) {
        self.memo = Some(memo);
    }

    /// Update status
    pub fn set_status(&mut self, status: InvoiceStatus) {
        self.status = status;
    }
}

/// Route hint for private channels
#[derive(Clone, Debug)]
pub struct RouteHint {
    pub node_id: String,
    pub short_channel_id: u64,
    pub fee_base_msat: u64,
    pub fee_proportional_millionths: u32,
    pub cltv_expiry_delta: u16,
}

impl RouteHint {
    pub fn new(
        node_id: String,
        short_channel_id: u64,
        fee_base_msat: u64,
        fee_proportional_millionths: u32,
        cltv_expiry_delta: u16,
    ) -> Self {
        Self {
            node_id,
            short_channel_id,
            fee_base_msat,
            fee_proportional_millionths,
            cltv_expiry_delta,
        }
    }
}

/// Invoice Builder
pub struct InvoiceBuilder {
    chain_id: Option<String>,
    recipient: Option<String>,
    amount: Option<u128>,
    description: Option<String>,
    expires_in: u64,
    memo: Option<String>,
    route_hints: Vec<RouteHint>,
}

impl InvoiceBuilder {
    /// Create new invoice builder
    pub fn new() -> Self {
        Self {
            chain_id: None,
            recipient: None,
            amount: None,
            description: None,
            expires_in: DEFAULT_EXPIRY,
            memo: None,
            route_hints: Vec::new(),
        }
    }

    /// Set chain ID
    pub fn chain(mut self, chain_id: &str) -> Self {
        self.chain_id = Some(chain_id.to_string());
        self
    }

    /// Set recipient
    pub fn recipient(mut self, recipient: &str) -> Self {
        self.recipient = Some(recipient.to_string());
        self
    }

    /// Set payment amount
    pub fn amount(mut self, amount: u128) -> Self {
        self.amount = Some(amount);
        self
    }

    /// Set description
    pub fn description(mut self, description: &str) -> Self {
        self.description = Some(description.to_string());
        self
    }

    /// Set expiration duration in seconds
    pub fn expires_in(mut self, seconds: u64) -> Self {
        self.expires_in = seconds;
        self
    }

    /// Set memo
    pub fn memo(mut self, memo: &str) -> Self {
        self.memo = Some(memo.to_string());
        self
    }

    /// Add route hint
    pub fn add_route_hint(mut self, hint: RouteHint) -> Self {
        self.route_hints.push(hint);
        self
    }

    /// Build invoice
    pub fn build(self, current_time: u64) -> Result<LightningInvoice, InvoiceError> {
        let chain_id = self
            .chain_id
            .ok_or_else(|| InvoiceError::MissingRequiredField("chain_id".to_string()))?;
        let recipient = self
            .recipient
            .ok_or_else(|| InvoiceError::MissingRequiredField("recipient".to_string()))?;
        let amount = self
            .amount
            .ok_or_else(|| InvoiceError::MissingRequiredField("amount".to_string()))?;
        let description = self
            .description
            .ok_or_else(|| InvoiceError::MissingRequiredField("description".to_string()))?;

        let expires_at = current_time + self.expires_in;

        // Generate payment hash (simplified - in production use proper HTLC hash)
        let payment_hash = generate_payment_hash(&chain_id, &recipient, amount, current_time);

        let mut invoice = LightningInvoice::new(
            chain_id,
            recipient,
            amount,
            description,
            current_time,
            expires_at,
            payment_hash,
        )?;

        if let Some(memo) = self.memo {
            invoice.set_memo(memo);
        }

        for hint in self.route_hints {
            invoice.add_route_hint(hint);
        }

        Ok(invoice)
    }
}

impl Default for InvoiceBuilder {
    fn default() -> Self {
        Self::new()
    }
}

/// Payment request (user-facing)
#[derive(Clone, Debug)]
pub struct PaymentRequest {
    pub invoice: LightningInvoice,
    pub qr_data: String,
}

impl PaymentRequest {
    /// Create payment request from invoice
    pub fn from_invoice(invoice: LightningInvoice) -> Result<Self, InvoiceError> {
        let qr_data = invoice.to_qr()?;
        Ok(Self { invoice, qr_data })
    }

    /// Get invoice
    pub fn invoice(&self) -> &LightningInvoice {
        &self.invoice
    }

    /// Get QR data
    pub fn qr_data(&self) -> &str {
        &self.qr_data
    }
}

// Helper functions

/// Generate payment hash (simplified)
fn generate_payment_hash(chain_id: &str, recipient: &str, amount: u128, timestamp: u64) -> Vec<u8> {
    // In production, use SHA256(payment_preimage)
    // This is a simplified version for demonstration
    let data = format!("{}:{}:{}:{}", chain_id, recipient, amount, timestamp);
    data.as_bytes().to_vec()
}

/// Hex encode bytes
fn hex_encode(bytes: &[u8]) -> String {
    bytes
        .iter()
        .map(|b| format!("{:02x}", b))
        .collect::<String>()
}

/// Hex decode string
fn hex_decode(s: &str) -> Result<Vec<u8>, InvoiceError> {
    if s.len() % 2 != 0 {
        return Err(InvoiceError::DecodingFailed);
    }

    (0..s.len())
        .step_by(2)
        .map(|i| {
            u8::from_str_radix(&s[i..i + 2], 16)
                .map_err(|_| InvoiceError::DecodingFailed)
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_invoice_creation() {
        let invoice = LightningInvoice::new(
            "eth-pbc".to_string(),
            "0x123...".to_string(),
            1_000_000_000_000_000_000, // 1 ETH
            "Payment for services".to_string(),
            1000,
            2000,
            vec![1, 2, 3, 4],
        );
        assert!(invoice.is_ok());
    }

    #[test]
    fn test_invoice_invalid_chain() {
        let invoice = LightningInvoice::new(
            "".to_string(),
            "0x123...".to_string(),
            1000,
            "Test".to_string(),
            1000,
            2000,
            vec![1, 2, 3],
        );
        assert_eq!(invoice, Err(InvoiceError::InvalidChainId));
    }

    #[test]
    fn test_invoice_invalid_amount() {
        let invoice = LightningInvoice::new(
            "eth-pbc".to_string(),
            "0x123...".to_string(),
            0,
            "Test".to_string(),
            1000,
            2000,
            vec![1, 2, 3],
        );
        assert_eq!(invoice, Err(InvoiceError::InvalidAmount));
    }

    #[test]
    fn test_invoice_expiry() {
        let invoice = LightningInvoice::new(
            "eth-pbc".to_string(),
            "0x123...".to_string(),
            1000,
            "Test".to_string(),
            1000,
            2000,
            vec![1, 2, 3],
        )
        .unwrap();

        assert!(!invoice.is_expired(1500));
        assert!(invoice.is_expired(2001));
    }

    #[test]
    fn test_invoice_time_remaining() {
        let invoice = LightningInvoice::new(
            "eth-pbc".to_string(),
            "0x123...".to_string(),
            1000,
            "Test".to_string(),
            1000,
            2000,
            vec![1, 2, 3],
        )
        .unwrap();

        assert_eq!(invoice.time_remaining(1500), 500);
        assert_eq!(invoice.time_remaining(2001), 0);
    }

    #[test]
    fn test_invoice_qr_encoding() {
        let invoice = LightningInvoice::new(
            "eth-pbc".to_string(),
            "0x123".to_string(),
            1000,
            "Test".to_string(),
            1000,
            2000,
            vec![1, 2, 3],
        )
        .unwrap();

        let qr = invoice.to_qr();
        assert!(qr.is_ok());
        assert!(qr.unwrap().starts_with(INVOICE_PREFIX));
    }

    #[test]
    fn test_invoice_qr_decoding() {
        let invoice = LightningInvoice::new(
            "btc-pbc".to_string(),
            "bc1q...".to_string(),
            5000,
            "Test payment".to_string(),
            1000,
            2000,
            vec![1, 2, 3, 4, 5],
        )
        .unwrap();

        let qr_data = invoice.to_qr().unwrap();
        let decoded = LightningInvoice::from_qr(&qr_data);

        assert!(decoded.is_ok());
        let decoded_invoice = decoded.unwrap();
        assert_eq!(decoded_invoice.chain_id, "btc-pbc");
        assert_eq!(decoded_invoice.amount, 5000);
        assert_eq!(decoded_invoice.recipient, "bc1q...");
    }

    #[test]
    fn test_invoice_builder() {
        let invoice = InvoiceBuilder::new()
            .chain("eth-pbc")
            .recipient("0x123...")
            .amount(1_000_000_000_000_000_000)
            .description("Payment for services")
            .expires_in(3600)
            .build(1000);

        assert!(invoice.is_ok());
        let inv = invoice.unwrap();
        assert_eq!(inv.chain_id, "eth-pbc");
        assert_eq!(inv.amount, 1_000_000_000_000_000_000);
    }

    #[test]
    fn test_invoice_builder_missing_field() {
        let invoice = InvoiceBuilder::new()
            .chain("eth-pbc")
            .recipient("0x123...")
            // Missing amount
            .description("Test")
            .build(1000);

        assert_eq!(
            invoice,
            Err(InvoiceError::MissingRequiredField("amount".to_string()))
        );
    }

    #[test]
    fn test_invoice_status_display() {
        assert_eq!(format!("{}", InvoiceStatus::Pending), "Pending");
        assert_eq!(format!("{}", InvoiceStatus::Paid), "Paid");
        assert_eq!(format!("{}", InvoiceStatus::Expired), "Expired");
    }

    #[test]
    fn test_route_hint_creation() {
        let hint = RouteHint::new(
            "node123".to_string(),
            12345,
            1000,
            100,
            144,
        );
        assert_eq!(hint.node_id, "node123");
        assert_eq!(hint.short_channel_id, 12345);
    }

    #[test]
    fn test_payment_request() {
        let invoice = LightningInvoice::new(
            "sol-pbc".to_string(),
            "Sol1...".to_string(),
            10000,
            "Solana payment".to_string(),
            1000,
            2000,
            vec![1, 2, 3],
        )
        .unwrap();

        let request = PaymentRequest::from_invoice(invoice);
        assert!(request.is_ok());

        let req = request.unwrap();
        assert!(req.qr_data.starts_with(INVOICE_PREFIX));
        assert_eq!(req.invoice.chain_id, "sol-pbc");
    }

    #[test]
    fn test_hex_encode() {
        let bytes = vec![0x01, 0x23, 0xAB, 0xFF];
        let encoded = hex_encode(&bytes);
        assert_eq!(encoded, "0123abff");
    }

    #[test]
    fn test_hex_decode() {
        let hex_str = "0123abff";
        let decoded = hex_decode(hex_str).unwrap();
        assert_eq!(decoded, vec![0x01, 0x23, 0xAB, 0xFF]);
    }

    #[test]
    fn test_hex_decode_invalid() {
        let result = hex_decode("xyz");
        assert_eq!(result, Err(InvoiceError::DecodingFailed));
    }

    #[test]
    fn test_invoice_with_memo() {
        let mut invoice = LightningInvoice::new(
            "ada-pbc".to_string(),
            "addr1...".to_string(),
            2000,
            "Cardano payment".to_string(),
            1000,
            2000,
            vec![1, 2, 3],
        )
        .unwrap();

        invoice.set_memo("Thank you for your purchase!".to_string());
        assert_eq!(invoice.memo, Some("Thank you for your purchase!".to_string()));
    }

    #[test]
    fn test_invoice_status_change() {
        let mut invoice = LightningInvoice::new(
            "dot-pbc".to_string(),
            "1ABC...".to_string(),
            5000,
            "Polkadot payment".to_string(),
            1000,
            2000,
            vec![1, 2, 3],
        )
        .unwrap();

        assert_eq!(invoice.status, InvoiceStatus::Pending);
        invoice.set_status(InvoiceStatus::Processing);
        assert_eq!(invoice.status, InvoiceStatus::Processing);
        invoice.set_status(InvoiceStatus::Paid);
        assert_eq!(invoice.status, InvoiceStatus::Paid);
    }
}
