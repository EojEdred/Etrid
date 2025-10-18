//! Async Encrypted Communications (aecomms)
//! 
//! Provides ECIES (Elliptic Curve Integrated Encryption Scheme) for peer-to-peer encrypted messaging
//! over async TCP with session management and handshake protocol.

use std::sync::Arc;
use tokio::sync::RwLock;
use x25519_dalek::{PublicKey, StaticSecret};
use sha2::{Sha256, Digest};
use chacha20poly1305::{
    ChaCha20Poly1305, Nonce, Key, KeyInit,
    aead::{Aead, Payload},
};
// use rand::Rng;

/// Session state for an encrypted peer connection
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SessionState {
    New,
    HandshakeInitiated,
    HandshakeComplete,
    Active,
    Closed,
}

/// ECIES-based session for encrypted communications
pub struct CipherSession {
    session_id: u64,
    state: Arc<RwLock<SessionState>>,
    local_secret: StaticSecret,
    remote_public: Option<PublicKey>,
    shared_secret: Option<[u8; 32]>,
    cipher: Option<ChaCha20Poly1305>,
    nonce_counter: Arc<RwLock<u64>>,
}

impl CipherSession {
    /// Create a new cipher session
    pub fn new(session_id: u64) -> Self {
        let local_secret = StaticSecret::random_from_rng(rand::thread_rng());
        Self {
            session_id,
            state: Arc::new(RwLock::new(SessionState::New)),
            local_secret,
            remote_public: None,
            shared_secret: None,
            cipher: None,
            nonce_counter: Arc::new(RwLock::new(0)),
        }
    }

    /// Get local public key for handshake
    pub fn local_public_key(&self) -> PublicKey {
        PublicKey::from(&self.local_secret)
    }

    /// Initiate handshake - return our public key
    pub async fn initiate_handshake(&mut self) -> Vec<u8> {
        let mut state = self.state.write().await;
        *state = SessionState::HandshakeInitiated;
        self.local_public_key().as_bytes().to_vec()
    }

    /// Complete handshake with remote public key
    pub async fn complete_handshake(&mut self, remote_public_bytes: &[u8]) -> Result<(), String> {
        if remote_public_bytes.len() != 32 {
            return Err("Invalid public key length".to_string());
        }

        let mut remote_array = [0u8; 32];
        remote_array.copy_from_slice(remote_public_bytes);
        let remote_public = PublicKey::from(remote_array);

        self.remote_public = Some(remote_public);

        // Compute shared secret using x25519
        let shared_secret = self.local_secret.diffie_hellman(&remote_public);
        self.shared_secret = Some(*shared_secret.as_bytes());

        // Derive cipher from shared secret
        let mut hasher = Sha256::new();
        hasher.update(shared_secret.as_bytes());
        let derived_key = hasher.finalize();

        let mut key_array = [0u8; 32];
        key_array.copy_from_slice(&derived_key[..32]);
        self.cipher = Some(ChaCha20Poly1305::new(&Key::from(key_array)));

        let mut state = self.state.write().await;
        *state = SessionState::HandshakeComplete;
        Ok(())
    }

    /// Encrypt a message
    pub async fn encrypt(&self, plaintext: &[u8]) -> Result<Vec<u8>, String> {
        let cipher = self.cipher.as_ref().ok_or("Session not ready")?;
        let mut state = self.state.write().await;
        
        if *state != SessionState::HandshakeComplete && *state != SessionState::Active {
            return Err("Invalid session state".to_string());
        }
        *state = SessionState::Active;

        let mut counter = self.nonce_counter.write().await;
        let nonce_bytes = counter.to_le_bytes();
        let mut nonce_array = [0u8; 12];
        nonce_array[..8].copy_from_slice(&nonce_bytes);
        *counter += 1;

        let nonce = Nonce::from(nonce_array);
        let ciphertext = cipher
            .encrypt(&nonce, Payload { msg: plaintext, aad: b"" })
            .map_err(|e| format!("Encryption failed: {}", e))?;

        // Return: nonce (8 bytes) + ciphertext
        let mut result = Vec::with_capacity(8 + ciphertext.len());
        result.extend_from_slice(&nonce_bytes);
        result.extend_from_slice(&ciphertext);
        Ok(result)
    }

    /// Decrypt a message
    pub async fn decrypt(&self, ciphertext_with_nonce: &[u8]) -> Result<Vec<u8>, String> {
        let cipher = self.cipher.as_ref().ok_or("Session not ready")?;
        
        if ciphertext_with_nonce.len() < 8 {
            return Err("Ciphertext too short".to_string());
        }

        let nonce_bytes = &ciphertext_with_nonce[..8];
        let ciphertext = &ciphertext_with_nonce[8..];

        let mut nonce_array = [0u8; 12];
        nonce_array[..8].copy_from_slice(nonce_bytes);
        let nonce = Nonce::from(nonce_array);

        cipher
            .decrypt(&nonce, Payload { msg: ciphertext, aad: b"" })
            .map_err(|e| format!("Decryption failed: {}", e))
    }

    /// Get current session state
    pub async fn get_state(&self) -> SessionState {
        *self.state.read().await
    }

    /// Close the session
    pub async fn close(&mut self) {
        let mut state = self.state.write().await;
        *state = SessionState::Closed;
    }
}

/// Message wrapper with framing
#[derive(Debug, Clone)]
pub struct EncryptedMessage {
    pub session_id: u64,
    pub sequence: u64,
    pub payload: Vec<u8>,
}

impl EncryptedMessage {
    /// Serialize to bytes with framing (4-byte length prefix)
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut buf = Vec::new();
        buf.extend_from_slice(&self.session_id.to_le_bytes());
        buf.extend_from_slice(&self.sequence.to_le_bytes());
        buf.extend_from_slice(&(self.payload.len() as u32).to_le_bytes());
        buf.extend_from_slice(&self.payload);
        buf
    }

    /// Deserialize from bytes
    pub fn from_bytes(buf: &[u8]) -> Result<Self, String> {
        if buf.len() < 20 {
            return Err("Buffer too short".to_string());
        }

        let mut session_id_bytes = [0u8; 8];
        session_id_bytes.copy_from_slice(&buf[0..8]);
        let session_id = u64::from_le_bytes(session_id_bytes);

        let mut seq_bytes = [0u8; 8];
        seq_bytes.copy_from_slice(&buf[8..16]);
        let sequence = u64::from_le_bytes(seq_bytes);

        let mut len_bytes = [0u8; 4];
        len_bytes.copy_from_slice(&buf[16..20]);
        let payload_len = u32::from_le_bytes(len_bytes) as usize;

        if buf.len() != 20 + payload_len {
            return Err("Invalid payload length".to_string());
        }

        let payload = buf[20..].to_vec();
        Ok(EncryptedMessage {
            session_id,
            sequence,
            payload,
        })
    }
}

/// Cipher trait for pluggable encryption schemes
pub trait Cipher: Send + Sync {
    fn encrypt(&self, plaintext: &[u8]) -> Result<Vec<u8>, String>;
    fn decrypt(&self, ciphertext: &[u8]) -> Result<Vec<u8>, String>;
}

impl Cipher for CipherSession {
    fn encrypt(&self, plaintext: &[u8]) -> Result<Vec<u8>, String> {
        futures::executor::block_on(self.encrypt(plaintext))
    }

    fn decrypt(&self, ciphertext: &[u8]) -> Result<Vec<u8>, String> {
        futures::executor::block_on(self.decrypt(ciphertext))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_session_creation() {
        let session = CipherSession::new(1);
        assert_eq!(session.get_state().await, SessionState::New);
    }

    #[tokio::test]
    async fn test_handshake_flow() {
        let mut session1 = CipherSession::new(1);
        let mut session2 = CipherSession::new(2);

        let pub1 = session1.initiate_handshake().await;
        let pub2 = session2.initiate_handshake().await;

        assert_eq!(session1.get_state().await, SessionState::HandshakeInitiated);
        assert_eq!(session2.get_state().await, SessionState::HandshakeInitiated);

        session1.complete_handshake(&pub2).await.unwrap();
        session2.complete_handshake(&pub1).await.unwrap();

        assert_eq!(session1.get_state().await, SessionState::HandshakeComplete);
        assert_eq!(session2.get_state().await, SessionState::HandshakeComplete);
    }

    #[tokio::test]
    async fn test_encrypt_decrypt() {
        let mut session1 = CipherSession::new(1);
        let mut session2 = CipherSession::new(2);

        let pub1 = session1.initiate_handshake().await;
        let pub2 = session2.initiate_handshake().await;

        session1.complete_handshake(&pub2).await.unwrap();
        session2.complete_handshake(&pub1).await.unwrap();

        let plaintext = b"Hello, ETRID!";
        let encrypted = session1.encrypt(plaintext).await.unwrap();
        let decrypted = session2.decrypt(&encrypted).await.unwrap();

        assert_eq!(plaintext, &decrypted[..]);
    }

    #[tokio::test]
    async fn test_message_serialization() {
        let msg = EncryptedMessage {
            session_id: 42,
            sequence: 100,
            payload: vec![1, 2, 3, 4, 5],
        };

        let bytes = msg.to_bytes();
        let restored = EncryptedMessage::from_bytes(&bytes).unwrap();

        assert_eq!(restored.session_id, 42);
        assert_eq!(restored.sequence, 100);
        assert_eq!(restored.payload, vec![1, 2, 3, 4, 5]);
    }

    #[tokio::test]
    async fn test_invalid_public_key() {
        let mut session = CipherSession::new(1);
        let result = session.complete_handshake(&[0u8; 31]).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_multiple_encryptions() {
        let mut session1 = CipherSession::new(1);
        let mut session2 = CipherSession::new(2);

        let pub1 = session1.initiate_handshake().await;
        let pub2 = session2.initiate_handshake().await;

        session1.complete_handshake(&pub2).await.unwrap();
        session2.complete_handshake(&pub1).await.unwrap();

        for i in 0..10 {
            let msg = format!("Message {}", i);
            let encrypted = session1.encrypt(msg.as_bytes()).await.unwrap();
            let decrypted = session2.decrypt(&encrypted).await.unwrap();
            assert_eq!(msg.as_bytes(), &decrypted[..]);
        }
    }

    #[tokio::test]
    async fn test_session_close() {
        let mut session = CipherSession::new(1);
        session.initiate_handshake().await;
        assert_ne!(session.get_state().await, SessionState::Closed);
        
        session.close().await;
        assert_eq!(session.get_state().await, SessionState::Closed);
    }

    #[test]
    fn test_message_framing_edge_case() {
        let msg = EncryptedMessage {
            session_id: 0,
            sequence: 0,
            payload: vec![],
        };
        let bytes = msg.to_bytes();
        let restored = EncryptedMessage::from_bytes(&bytes).unwrap();
        assert_eq!(restored.payload.len(), 0);
    }

    #[test]
    fn test_invalid_serialization() {
        let result = EncryptedMessage::from_bytes(&[0u8; 5]);
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_nonce_incrementing() {
        let mut session1 = CipherSession::new(1);
        let mut session2 = CipherSession::new(2);

        let pub1 = session1.initiate_handshake().await;
        let pub2 = session2.initiate_handshake().await;

        session1.complete_handshake(&pub2).await.unwrap();
        session2.complete_handshake(&pub1).await.unwrap();

        // Encrypt same message twice - should produce different ciphertexts
        let plaintext = b"Same message";
        let enc1 = session1.encrypt(plaintext).await.unwrap();
        let enc2 = session1.encrypt(plaintext).await.unwrap();

        assert_ne!(enc1, enc2); // Different due to incrementing nonce
    }

    #[tokio::test]
    async fn test_reject_decrypt_before_handshake() {
        let session = CipherSession::new(1);
        let result = session.decrypt(&vec![0u8; 20]).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_reject_encrypt_before_handshake() {
        let session = CipherSession::new(1);
        let result = session.encrypt(b"test").await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_bidirectional_communication() {
        let mut alice = CipherSession::new(1);
        let mut bob = CipherSession::new(2);

        let alice_pub = alice.initiate_handshake().await;
        let bob_pub = bob.initiate_handshake().await;

        alice.complete_handshake(&bob_pub).await.unwrap();
        bob.complete_handshake(&alice_pub).await.unwrap();

        // Alice -> Bob
        let msg_a = b"Hi Bob";
        let enc_a = alice.encrypt(msg_a).await.unwrap();
        let dec_a = bob.decrypt(&enc_a).await.unwrap();
        assert_eq!(msg_a, &dec_a[..]);

        // Bob -> Alice
        let msg_b = b"Hi Alice";
        let enc_b = bob.encrypt(msg_b).await.unwrap();
        let dec_b = alice.decrypt(&enc_b).await.unwrap();
        assert_eq!(msg_b, &dec_b[..]);
    }
}
