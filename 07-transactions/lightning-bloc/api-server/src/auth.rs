use axum::{
    extract::{Request, State},
    http::StatusCode,
    middleware::Next,
    response::{IntoResponse, Response},
};
use sp_core::{crypto::AccountId32, sr25519, Pair};
use std::str::FromStr;

/// Authentication middleware
pub async fn auth_middleware(
    request: Request,
    next: Next,
) -> Result<Response, Response> {
    // Get Authorization header
    let auth_header = request
        .headers()
        .get("Authorization")
        .and_then(|h| h.to_str().ok());

    // For now, skip auth in development
    // In production, verify wallet signature
    if let Some(_token) = auth_header {
        // TODO: Verify wallet signature
        // 1. Extract signature from header
        // 2. Verify signature against message
        // 3. Extract account ID from signature
        // 4. Attach account to request
    }

    Ok(next.run(request).await)
}

/// Verify Substrate signature
pub fn verify_signature(
    address: &str,
    message: &[u8],
    signature: &str,
) -> Result<bool, String> {
    // Parse address
    let account = AccountId32::from_str(address)
        .map_err(|_| "Invalid address".to_string())?;

    // Parse signature
    let sig_bytes = hex::decode(signature.trim_start_matches("0x"))
        .map_err(|_| "Invalid signature format".to_string())?;

    if sig_bytes.len() != 64 {
        return Err("Invalid signature length".to_string());
    }

    let mut sig_array = [0u8; 64];
    sig_array.copy_from_slice(&sig_bytes);
    let signature = sr25519::Signature::from_raw(sig_array);

    // Verify
    let public = sr25519::Public::from(account);
    Ok(sr25519::Pair::verify(&signature, message, &public))
}

/// Extract bearer token from Authorization header
pub fn extract_bearer_token(auth_header: &str) -> Option<String> {
    if auth_header.starts_with("Bearer ") {
        Some(auth_header[7..].to_string())
    } else {
        None
    }
}

/// Generate auth message for signing
pub fn generate_auth_message(address: &str, timestamp: i64) -> String {
    format!(
        "Sign this message to authenticate with Ëtrid Lightning Network.\n\nAddress: {}\nTimestamp: {}",
        address, timestamp
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_bearer_token() {
        let header = "Bearer abc123";
        assert_eq!(extract_bearer_token(header), Some("abc123".to_string()));
    }

    #[test]
    fn test_generate_auth_message() {
        let msg = generate_auth_message("5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY", 1234567890);
        assert!(msg.contains("Sign this message"));
        assert!(msg.contains("5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY"));
    }
}
