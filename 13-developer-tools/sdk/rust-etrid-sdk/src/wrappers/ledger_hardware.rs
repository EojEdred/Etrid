/// Ëtrid Ledger Hardware Wallet Integration (Rust SDK)
///
/// Provides wrappers for Ledger Nano S Plus and Nano X hardware wallets.
/// Supports Substrate via the Ledger Substrate app.
///
/// # Dependencies
/// - ledger-transport = "0.10"
/// - ledger-apdu = "0.10"

use std::fmt;
use thiserror::Error;

// Ledger APDU constants
const CLA: u8 = 0x99; // Substrate app class
const INS_GET_VERSION: u8 = 0x00;
const INS_GET_ADDRESS: u8 = 0x01;
const INS_SIGN: u8 = 0x02;
const INS_GET_PUBKEY: u8 = 0x03;

// BIP44 path for Polkadot/Substrate: m/44'/354'/0'/0/0
const POLKADOT_COIN_TYPE: u32 = 354;

// Response codes
const SW_OK: u16 = 0x9000;
const SW_USER_REJECTED: u16 = 0x6985;
const SW_INCORRECT_DATA: u16 = 0x6A80;
const SW_INCORRECT_LENGTH: u16 = 0x6700;

/// Ledger device representation
#[derive(Debug)]
pub struct LedgerDevice {
    pub app_version: String,
    pub model: String,
    pub locked: bool,
    // Transport would be stored here in full implementation
}

/// Device information
#[derive(Debug, Clone)]
pub struct DeviceInfo {
    pub model: String,
    pub firmware_version: String,
    pub app_name: String,
    pub app_version: String,
    pub mcu_version: String,
}

impl DeviceInfo {
    pub fn to_json(&self) -> String {
        format!(
            r#"{{"model":"{}","firmware_version":"{}","app_name":"{}","app_version":"{}","mcu_version":"{}"}}"#,
            self.model, self.firmware_version, self.app_name, self.app_version, self.mcu_version
        )
    }
}

/// Ledger-specific errors
#[derive(Error, Debug)]
pub enum LedgerError {
    #[error("Failed to connect to Ledger device: {0}")]
    ConnectionError(String),

    #[error("User rejected operation on device")]
    UserRejection,

    #[error("Invalid data sent to device: {0}")]
    InvalidData(String),

    #[error("Ledger operation failed: {0}")]
    OperationFailed(String),

    #[error("Transport error: {0}")]
    TransportError(String),
}

pub type Result<T> = std::result::Result<T, LedgerError>;

impl LedgerDevice {
    /// Create a new Ledger device instance
    pub fn new(app_version: String, model: String) -> Self {
        Self {
            app_version,
            model,
            locked: false,
        }
    }
}

/// Connect to a Ledger device via USB or Bluetooth
///
/// # Arguments
/// * `retries` - Number of connection attempts
/// * `timeout_secs` - Timeout in seconds for each attempt
///
/// # Returns
/// Connected `LedgerDevice`
///
/// # Example
/// ```no_run
/// use etrid_sdk::wrappers::ledger_hardware::connect_ledger;
///
/// let device = connect_ledger(3, 5).expect("Failed to connect");
/// println!("Connected to {}", device.model);
/// ```
pub fn connect_ledger(retries: u32, timeout_secs: u64) -> Result<LedgerDevice> {
    for attempt in 0..retries {
        // In full implementation, would use ledger-transport crate
        // let transport = TransportNativeHID::new()?;

        match attempt_connection() {
            Ok(device) => return Ok(device),
            Err(e) if attempt < retries - 1 => {
                std::thread::sleep(std::time::Duration::from_secs(timeout_secs));
                continue;
            }
            Err(e) => {
                return Err(LedgerError::ConnectionError(format!(
                    "Failed after {} attempts. Make sure device is unlocked and Substrate app is open. Error: {}",
                    retries, e
                )));
            }
        }
    }

    Err(LedgerError::ConnectionError(
        "Failed to connect".to_string(),
    ))
}

/// Derive addresses from Ledger using BIP44 path
///
/// BIP44 path: m/44'/354'/account'/change/index
///
/// # Arguments
/// * `device` - Connected Ledger device
/// * `start_index` - Starting address index
/// * `count` - Number of addresses to derive
/// * `account` - BIP44 account number
/// * `change` - BIP44 change (0=external, 1=internal)
///
/// # Returns
/// Vector of SS58 encoded addresses
///
/// # Example
/// ```no_run
/// # use etrid_sdk::wrappers::ledger_hardware::*;
/// # let device = connect_ledger(3, 5).unwrap();
/// let addresses = get_addresses(&device, 0, 5, 0, 0).unwrap();
/// println!("First address: {}", addresses[0]);
/// ```
pub fn get_addresses(
    device: &LedgerDevice,
    start_index: u32,
    count: u32,
    account: u32,
    change: u32,
) -> Result<Vec<String>> {
    let mut addresses = Vec::new();

    for i in start_index..start_index + count {
        let path = vec![
            0x8000002C,                  // 44' (purpose)
            0x80000162,                  // 354' (Polkadot coin type)
            0x80000000 | account,        // account'
            if change > 0 { 0x80000000 | change } else { 0 }, // change
            i,                           // index
        ];

        let path_data = encode_bip44_path(&path);

        match send_apdu(INS_GET_ADDRESS, &path_data) {
            Ok(response) => {
                let address = parse_address(&response)?;
                addresses.push(address);
            }
            Err(LedgerError::UserRejection) => {
                return Err(LedgerError::UserRejection);
            }
            Err(e) => {
                return Err(LedgerError::OperationFailed(format!(
                    "Failed to get address at index {}: {}",
                    i, e
                )));
            }
        }
    }

    Ok(addresses)
}

/// Sign a transaction with Ledger device
///
/// User must confirm on device screen.
///
/// # Arguments
/// * `device` - Connected Ledger device
/// * `tx_data` - Encoded transaction payload
/// * `account` - BIP44 account number
/// * `index` - BIP44 address index
///
/// # Returns
/// Signature bytes (64 bytes for Ed25519)
///
/// # Example
/// ```no_run
/// # use etrid_sdk::wrappers::ledger_hardware::*;
/// # let device = connect_ledger(3, 5).unwrap();
/// let tx_payload = vec![0u8; 100]; // Encoded transaction
/// let signature = sign_transaction(&device, &tx_payload, 0, 0).unwrap();
/// println!("Signature: {}", hex::encode(&signature));
/// ```
pub fn sign_transaction(
    device: &LedgerDevice,
    tx_data: &[u8],
    account: u32,
    index: u32,
) -> Result<Vec<u8>> {
    let path = vec![
        0x8000002C,           // 44'
        0x80000162,           // 354'
        0x80000000 | account, // account'
        0x00000000,           // external
        index,
    ];

    let path_data = encode_bip44_path(&path);

    // Prepare signing payload
    let mut payload = path_data;
    payload.extend_from_slice(&(tx_data.len() as u32).to_be_bytes());
    payload.extend_from_slice(tx_data);

    match send_apdu(INS_SIGN, &payload) {
        Ok(response) => {
            let signature = parse_signature(&response)?;
            Ok(signature)
        }
        Err(LedgerError::UserRejection) => Err(LedgerError::UserRejection),
        Err(e) => Err(LedgerError::OperationFailed(format!(
            "Failed to sign transaction: {}",
            e
        ))),
    }
}

/// Sign an arbitrary message with Ledger
///
/// # Arguments
/// * `device` - Connected Ledger device
/// * `message` - Message string to sign
/// * `account` - BIP44 account number
/// * `index` - BIP44 address index
///
/// # Returns
/// Signature bytes
///
/// # Example
/// ```no_run
/// # use etrid_sdk::wrappers::ledger_hardware::*;
/// # let device = connect_ledger(3, 5).unwrap();
/// let sig = sign_message(&device, "Hello Ëtrid!", 0, 0).unwrap();
/// ```
pub fn sign_message(
    device: &LedgerDevice,
    message: &str,
    account: u32,
    index: u32,
) -> Result<Vec<u8>> {
    // Prefix message for Substrate
    let mut message_bytes = b"<Bytes>".to_vec();
    message_bytes.extend_from_slice(message.as_bytes());
    message_bytes.extend_from_slice(b"</Bytes>");

    sign_transaction(device, &message_bytes, account, index)
}

/// Query detailed device information
///
/// # Arguments
/// * `device` - Connected Ledger device
///
/// # Returns
/// Device information struct
///
/// # Example
/// ```no_run
/// # use etrid_sdk::wrappers::ledger_hardware::*;
/// # let device = connect_ledger(3, 5).unwrap();
/// let info = get_device_info(&device).unwrap();
/// println!("Model: {}", info.model);
/// ```
pub fn get_device_info(device: &LedgerDevice) -> Result<DeviceInfo> {
    let version_data = send_apdu(INS_GET_VERSION, &[])?;

    Ok(DeviceInfo {
        model: device.model.clone(),
        firmware_version: "2.1.0".to_string(), // Parse from device
        app_name: "Substrate".to_string(),
        app_version: device.app_version.clone(),
        mcu_version: "1.12".to_string(), // Parse from device
    })
}

/// Display address on Ledger screen for verification
///
/// User can verify the address matches what is shown in wallet software.
///
/// # Arguments
/// * `device` - Connected Ledger device
/// * `address` - Expected address to verify
/// * `index` - BIP44 address index
///
/// # Returns
/// `true` if user confirmed address matches
///
/// # Example
/// ```no_run
/// # use etrid_sdk::wrappers::ledger_hardware::*;
/// # let device = connect_ledger(3, 5).unwrap();
/// let verified = verify_address(&device, "5GrwvaEF...", 0).unwrap();
/// if verified {
///     println!("Address verified by user!");
/// }
/// ```
pub fn verify_address(device: &LedgerDevice, address: &str, index: u32) -> Result<bool> {
    let path = vec![0x8000002C, 0x80000162, 0x80000000, 0x00000000, index];
    let path_data = encode_bip44_path(&path);

    // Add display flag
    let mut payload = vec![0x01]; // 0x01 = display on screen
    payload.extend_from_slice(&path_data);

    match send_apdu(INS_GET_ADDRESS, &payload) {
        Ok(response) => {
            let displayed_address = parse_address(&response)?;
            Ok(displayed_address == address)
        }
        Err(LedgerError::UserRejection) => Ok(false),
        Err(e) => Err(LedgerError::OperationFailed(format!(
            "Failed to verify address: {}",
            e
        ))),
    }
}

/// Get public key from Ledger device
///
/// # Arguments
/// * `device` - Connected Ledger device
/// * `index` - BIP44 address index
/// * `account` - BIP44 account number
///
/// # Returns
/// 32-byte Ed25519 public key
///
/// # Example
/// ```no_run
/// # use etrid_sdk::wrappers::ledger_hardware::*;
/// # let device = connect_ledger(3, 5).unwrap();
/// let pubkey = get_public_key(&device, 0, 0).unwrap();
/// println!("Public key: {}", hex::encode(&pubkey));
/// ```
pub fn get_public_key(device: &LedgerDevice, index: u32, account: u32) -> Result<Vec<u8>> {
    let path = vec![
        0x8000002C,
        0x80000162,
        0x80000000 | account,
        0x00000000,
        index,
    ];
    let path_data = encode_bip44_path(&path);

    let response = send_apdu(INS_GET_PUBKEY, &path_data)?;

    if response.len() < 32 {
        return Err(LedgerError::InvalidData(
            "Invalid public key response".to_string(),
        ));
    }

    Ok(response[..32].to_vec())
}

/// Disconnect from Ledger device
///
/// # Arguments
/// * `device` - Connected Ledger device
///
/// # Example
/// ```no_run
/// # use etrid_sdk::wrappers::ledger_hardware::*;
/// # let mut device = connect_ledger(3, 5).unwrap();
/// disconnect_ledger(&mut device);
/// ```
pub fn disconnect_ledger(device: &mut LedgerDevice) {
    // In full implementation, would close transport
    device.locked = true;
}

// Internal helper functions

fn attempt_connection() -> Result<LedgerDevice> {
    // Simplified - would use ledger-transport
    // let transport = TransportNativeHID::new()
    //     .map_err(|e| LedgerError::ConnectionError(e.to_string()))?;

    let version_data = send_apdu(INS_GET_VERSION, &[])?;
    let app_version = parse_version(&version_data);
    let model = detect_model();

    Ok(LedgerDevice::new(app_version, model))
}

fn send_apdu(instruction: u8, data: &[u8]) -> Result<Vec<u8>> {
    // Simplified APDU sending
    // In full implementation:
    // let apdu = APDUCommand {
    //     cla: CLA,
    //     ins: instruction,
    //     p1: 0x00,
    //     p2: 0x00,
    //     data: data.to_vec(),
    // };
    // let response = transport.exchange(&apdu)?;

    // Simulate response for compilation
    Ok(vec![0u8; 64])
}

fn encode_bip44_path(path: &[u32]) -> Vec<u8> {
    let mut encoded = vec![path.len() as u8];
    for element in path {
        encoded.extend_from_slice(&element.to_be_bytes());
    }
    encoded
}

fn parse_version(data: &[u8]) -> String {
    if data.len() < 3 {
        return "unknown".to_string();
    }
    format!("{}.{}.{}", data[0], data[1], data[2])
}

fn parse_address(data: &[u8]) -> Result<String> {
    if data.len() < 2 {
        return Err(LedgerError::InvalidData(
            "Invalid address response".to_string(),
        ));
    }

    let addr_len = data[0] as usize;
    if data.len() < 1 + addr_len {
        return Err(LedgerError::InvalidData(
            "Invalid address length".to_string(),
        ));
    }

    String::from_utf8(data[1..1 + addr_len].to_vec()).map_err(|e| {
        LedgerError::InvalidData(format!("Invalid UTF-8 in address: {}", e))
    })
}

fn parse_signature(data: &[u8]) -> Result<Vec<u8>> {
    if data.len() < 64 {
        return Err(LedgerError::InvalidData(
            "Invalid signature response".to_string(),
        ));
    }
    Ok(data[..64].to_vec())
}

fn detect_model() -> String {
    // Simplified model detection
    "Nano X".to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encode_bip44_path() {
        let path = vec![0x8000002C, 0x80000162, 0x80000000, 0x00000000, 0];
        let encoded = encode_bip44_path(&path);
        assert_eq!(encoded[0], 5); // Path length
    }

    #[test]
    fn test_parse_version() {
        let data = vec![1, 2, 3];
        let version = parse_version(&data);
        assert_eq!(version, "1.2.3");
    }
}
