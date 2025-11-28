//! DID Types
//!
//! Defines all DID (Decentralized Identifier) data structures, schemas, and validation.
//! Implements W3C-compatible DID format with ÉTRID extensions.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// ÉTRID DID identifier (did:etrid:xxx)
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Did {
    pub method: String,     // "etrid"
    pub identifier: String, // unique identifier
}

impl Did {
    pub fn new(identifier: String) -> Self {
        Self {
            method: "etrid".to_string(),
            identifier,
        }
    }

    pub fn from_string(did_str: &str) -> Result<Self, String> {
        let parts: Vec<&str> = did_str.split(':').collect();
        
        if parts.len() != 3 {
            return Err("Invalid DID format".to_string());
        }
        
        if parts[0] != "did" {
            return Err("Must start with 'did'".to_string());
        }
        
        if parts[1] != "etrid" {
            return Err("Only 'etrid' method supported".to_string());
        }
        
        if parts[2].is_empty() || parts[2].len() > 64 {
            return Err("Invalid identifier".to_string());
        }

        Ok(Self {
            method: parts[1].to_string(),
            identifier: parts[2].to_string(),
        })
    }

    pub fn to_string(&self) -> String {
        format!("did:{}:{}", self.method, self.identifier)
    }
}

/// Verification method for DID documents
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VerificationMethod {
    pub id: String,
    pub controller: String, // DID that controls this method
    pub method_type: String, // Ed25519VerificationKey2020, etc
    pub public_key: Vec<u8>,
    pub created_at: u64,
}

impl VerificationMethod {
    pub fn new(id: String, controller: String, method_type: String, public_key: Vec<u8>) -> Self {
        Self {
            id,
            controller,
            method_type,
            public_key,
            created_at: timestamp_secs(),
        }
    }

    pub fn validate(&self) -> Result<(), String> {
        if self.id.is_empty() {
            return Err("ID cannot be empty".to_string());
        }
        if self.public_key.is_empty() {
            return Err("Public key cannot be empty".to_string());
        }
        if !["Ed25519VerificationKey2020", "X25519KeyAgreementKey2020"].contains(&self.method_type.as_str()) {
            return Err("Unsupported verification method type".to_string());
        }
        Ok(())
    }
}

/// Service endpoint
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceEndpoint {
    pub id: String,
    pub service_type: String,
    pub endpoint_url: String,
}

impl ServiceEndpoint {
    pub fn new(id: String, service_type: String, endpoint_url: String) -> Self {
        Self {
            id,
            service_type,
            endpoint_url,
        }
    }

    pub fn validate(&self) -> Result<(), String> {
        if self.id.is_empty() {
            return Err("ID cannot be empty".to_string());
        }
        if self.service_type.is_empty() {
            return Err("Service type cannot be empty".to_string());
        }
        if !self.endpoint_url.starts_with("http://") && !self.endpoint_url.starts_with("https://") {
            return Err("Invalid endpoint URL".to_string());
        }
        Ok(())
    }
}

/// DID Document containing identity information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DidDocument {
    pub id: Did,
    pub context: Vec<String>, // JSON-LD contexts
    pub verification_methods: Vec<VerificationMethod>,
    pub authentication: Vec<String>, // IDs of verification methods for authentication
    pub assertion_method: Vec<String>, // IDs for assertions
    pub key_agreement: Vec<String>, // IDs for key agreement
    pub service_endpoints: Vec<ServiceEndpoint>,
    pub properties: HashMap<String, String>, // Custom properties
    pub created_at: u64,
    pub updated_at: u64,
    pub proof: Option<DocumentProof>,
}

impl DidDocument {
    pub fn new(id: Did) -> Self {
        let now = timestamp_secs();
        Self {
            id,
            context: vec!["https://www.w3.org/ns/did/v1".to_string()],
            verification_methods: vec![],
            authentication: vec![],
            assertion_method: vec![],
            key_agreement: vec![],
            service_endpoints: vec![],
            properties: HashMap::new(),
            created_at: now,
            updated_at: now,
            proof: None,
        }
    }

    pub fn add_verification_method(&mut self, method: VerificationMethod) -> Result<(), String> {
        method.validate()?;
        self.verification_methods.push(method);
        self.updated_at = timestamp_secs();
        Ok(())
    }

    pub fn add_service_endpoint(&mut self, endpoint: ServiceEndpoint) -> Result<(), String> {
        endpoint.validate()?;
        self.service_endpoints.push(endpoint);
        self.updated_at = timestamp_secs();
        Ok(())
    }

    pub fn set_authentication(&mut self, method_ids: Vec<String>) {
        self.authentication = method_ids;
        self.updated_at = timestamp_secs();
    }

    pub fn set_assertion_method(&mut self, method_ids: Vec<String>) {
        self.assertion_method = method_ids;
        self.updated_at = timestamp_secs();
    }

    pub fn set_key_agreement(&mut self, method_ids: Vec<String>) {
        self.key_agreement = method_ids;
        self.updated_at = timestamp_secs();
    }

    pub fn add_property(&mut self, key: String, value: String) {
        self.properties.insert(key, value);
        self.updated_at = timestamp_secs();
    }

    pub fn get_verification_method(&self, id: &str) -> Option<&VerificationMethod> {
        self.verification_methods.iter().find(|m| m.id == id)
    }

    pub fn get_service_endpoint(&self, id: &str) -> Option<&ServiceEndpoint> {
        self.service_endpoints.iter().find(|s| s.id == id)
    }

    pub fn validate(&self) -> Result<(), String> {
        if self.id.identifier.is_empty() {
            return Err("DID identifier cannot be empty".to_string());
        }
        if self.verification_methods.is_empty() {
            return Err("At least one verification method required".to_string());
        }
        for method in &self.verification_methods {
            method.validate()?;
        }
        for endpoint in &self.service_endpoints {
            endpoint.validate()?;
        }
        Ok(())
    }

    pub fn is_valid_authentication_method(&self, method_id: &str) -> bool {
        self.authentication.contains(&method_id.to_string())
            && self.get_verification_method(method_id).is_some()
    }
}

/// Proof of DID document
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocumentProof {
    pub proof_type: String,
    pub created: u64,
    pub verification_method: String,
    pub signature: Vec<u8>,
}

impl DocumentProof {
    pub fn new(proof_type: String, verification_method: String, signature: Vec<u8>) -> Self {
        Self {
            proof_type,
            created: timestamp_secs(),
            verification_method,
            signature,
        }
    }

    pub fn validate(&self) -> Result<(), String> {
        if self.proof_type.is_empty() {
            return Err("Proof type cannot be empty".to_string());
        }
        if self.signature.is_empty() {
            return Err("Signature cannot be empty".to_string());
        }
        Ok(())
    }
}

/// DID Metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DidMetadata {
    pub did: Did,
    pub created: u64,
    pub updated: u64,
    pub deactivated: bool,
    pub version_id: String,
}

impl DidMetadata {
    pub fn new(did: Did) -> Self {
        let now = timestamp_secs();
        Self {
            did,
            created: now,
            updated: now,
            deactivated: false,
            version_id: "1".to_string(),
        }
    }
}

fn timestamp_secs() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_did_creation() {
        let did = Did::new("user123".to_string());
        assert_eq!(did.method, "etrid");
        assert_eq!(did.identifier, "user123");
    }

    #[test]
    fn test_did_to_string() {
        let did = Did::new("user123".to_string());
        assert_eq!(did.to_string(), "did:etrid:user123");
    }

    #[test]
    fn test_did_from_string() {
        let did_str = "did:etrid:user123";
        let did = Did::from_string(did_str).unwrap();
        assert_eq!(did.identifier, "user123");
    }

    #[test]
    fn test_did_from_string_invalid_format() {
        assert!(Did::from_string("invalid").is_err());
        assert!(Did::from_string("did:other:id").is_err());
    }

    #[test]
    fn test_verification_method_creation() {
        let method = VerificationMethod::new(
            "key1".to_string(),
            "did:etrid:user1".to_string(),
            "Ed25519VerificationKey2020".to_string(),
            vec![1, 2, 3],
        );
        assert_eq!(method.id, "key1");
    }

    #[test]
    fn test_verification_method_validation() {
        let method = VerificationMethod::new(
            "key1".to_string(),
            "did:etrid:user1".to_string(),
            "Ed25519VerificationKey2020".to_string(),
            vec![1, 2, 3],
        );
        assert!(method.validate().is_ok());
    }

    #[test]
    fn test_verification_method_invalid_type() {
        let mut method = VerificationMethod::new(
            "key1".to_string(),
            "did:etrid:user1".to_string(),
            "InvalidType".to_string(),
            vec![1, 2, 3],
        );
        assert!(method.validate().is_err());
    }

    #[test]
    fn test_service_endpoint_creation() {
        let endpoint = ServiceEndpoint::new(
            "endpoint1".to_string(),
            "VerifiableCredentialService".to_string(),
            "https://example.com/vc".to_string(),
        );
        assert!(endpoint.validate().is_ok());
    }

    #[test]
    fn test_service_endpoint_invalid_url() {
        let endpoint = ServiceEndpoint::new(
            "endpoint1".to_string(),
            "VerifiableCredentialService".to_string(),
            "ftp://example.com".to_string(),
        );
        assert!(endpoint.validate().is_err());
    }

    #[test]
    fn test_did_document_creation() {
        let did = Did::new("user123".to_string());
        let doc = DidDocument::new(did);
        assert_eq!(doc.context.len(), 1);
    }

    #[test]
    fn test_did_document_add_verification_method() {
        let did = Did::new("user123".to_string());
        let mut doc = DidDocument::new(did);
        let method = VerificationMethod::new(
            "key1".to_string(),
            "did:etrid:user123".to_string(),
            "Ed25519VerificationKey2020".to_string(),
            vec![1, 2, 3],
        );
        assert!(doc.add_verification_method(method).is_ok());
        assert_eq!(doc.verification_methods.len(), 1);
    }

    #[test]
    fn test_did_document_add_service() {
        let did = Did::new("user123".to_string());
        let mut doc = DidDocument::new(did);
        let endpoint = ServiceEndpoint::new(
            "ep1".to_string(),
            "Service".to_string(),
            "https://example.com".to_string(),
        );
        assert!(doc.add_service_endpoint(endpoint).is_ok());
        assert_eq!(doc.service_endpoints.len(), 1);
    }

    #[test]
    fn test_did_document_set_authentication() {
        let did = Did::new("user123".to_string());
        let mut doc = DidDocument::new(did);
        doc.set_authentication(vec!["key1".to_string()]);
        assert_eq!(doc.authentication.len(), 1);
    }

    #[test]
    fn test_did_document_get_verification_method() {
        let did = Did::new("user123".to_string());
        let mut doc = DidDocument::new(did);
        let method = VerificationMethod::new(
            "key1".to_string(),
            "did:etrid:user123".to_string(),
            "Ed25519VerificationKey2020".to_string(),
            vec![1, 2, 3],
        );
        doc.add_verification_method(method).unwrap();
        
        let retrieved = doc.get_verification_method("key1");
        assert!(retrieved.is_some());
    }

    #[test]
    fn test_did_document_validation() {
        let did = Did::new("user123".to_string());
        let mut doc = DidDocument::new(did);
        
        // Without verification methods, should fail
        assert!(doc.validate().is_err());
        
        // Add verification method
        let method = VerificationMethod::new(
            "key1".to_string(),
            "did:etrid:user123".to_string(),
            "Ed25519VerificationKey2020".to_string(),
            vec![1, 2, 3],
        );
        doc.add_verification_method(method).unwrap();
        
        assert!(doc.validate().is_ok());
    }

    #[test]
    fn test_document_proof_creation() {
        let proof = DocumentProof::new(
            "Ed25519Signature2020".to_string(),
            "key1".to_string(),
            vec![1, 2, 3],
        );
        assert!(proof.validate().is_ok());
    }

    #[test]
    fn test_did_metadata_creation() {
        let did = Did::new("user123".to_string());
        let metadata = DidMetadata::new(did);
        assert!(!metadata.deactivated);
    }

    #[test]
    fn test_did_document_properties() {
        let did = Did::new("user123".to_string());
        let mut doc = DidDocument::new(did);
        doc.add_property("name".to_string(), "Alice".to_string());
        
        let value = doc.properties.get("name");
        assert_eq!(value, Some(&"Alice".to_string()));
    }

    #[test]
    fn test_did_is_valid_authentication_method() {
        let did = Did::new("user123".to_string());
        let mut doc = DidDocument::new(did);
        
        let method = VerificationMethod::new(
            "key1".to_string(),
            "did:etrid:user123".to_string(),
            "Ed25519VerificationKey2020".to_string(),
            vec![1, 2, 3],
        );
        doc.add_verification_method(method).unwrap();
        doc.set_authentication(vec!["key1".to_string()]);
        
        assert!(doc.is_valid_authentication_method("key1"));
        assert!(!doc.is_valid_authentication_method("key2"));
    }
}
