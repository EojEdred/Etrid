//! AIDID - AI Decentralized Identity
//!
//! Provides decentralized identities for Artificial Intelligence agents,
//! models, and systems on the Ëtrid blockchain.
//!
//! # Overview
//!
//! AIDID extends the W3C DID specification to provide identities specifically
//! designed for AI systems. This includes:
//!
//! - **Identity**: Unique DIDs for AI agents (did:etrid:ai:{type}:{id})
//! - **Capabilities**: Declare what an AI can and cannot do
//! - **Attestation**: Cryptographic proof of model provenance and training
//! - **Reputation**: Track performance and build trust scores
//! - **Authorization**: Permission system for AI actions
//! - **Safety**: Alignment methods, content filtering, bias evaluation
//!
//! # Examples
//!
//! ## Register an AI
//!
//! ```ignore
//! use aidid::types::*;
//! use aidid::registry::*;
//!
//! // Create AI profile
//! let mut capabilities = Capabilities::default();
//! capabilities.tasks.push(Task::TextGeneration);
//! capabilities.input_modalities.push(Modality::Text);
//! capabilities.output_modalities.push(Modality::Text);
//!
//! let profile = AIProfile {
//!     ai_type: AIType::LLM,
//!     version: b"v1.0.0".to_vec(),
//!     architecture: b"transformer".to_vec(),
//!     parameters: b"175B".to_vec(),
//!     capabilities,
//!     restrictions: Restrictions::default(),
//!     safety: SafetyProfile::default(),
//! };
//!
//! // Register AI
//! // register_ai(origin, b"gpt-4".to_vec(), AIType::LLM, profile)?;
//! ```
//!
//! ## Attest a Model
//!
//! ```ignore
//! use aidid::attestation::*;
//!
//! let attestation = AttestationBuilder::new()
//!     .model_hash([1u8; 32])
//!     .training_data_hash(b"QmXyz...".to_vec())
//!     .version(b"v1.0.0".to_vec())
//!     .training_date(1634567890)
//!     .reproducible(false)
//!     .add_benchmark(b"MMLU".to_vec(), 8670)
//!     .build()?;
//!
//! // attest_model(origin, did_hash, attestation)?;
//! ```

#![cfg_attr(not(feature = "std"), no_std)]

pub mod types;
pub mod registry;
pub mod attestation;

// Re-export commonly used types
pub use types::{
    AIDid, AIType, Task, Modality,
    Capabilities, Restrictions,
    AIProfile, SafetyProfile,
    ModelAttestation, Benchmark,
    Permission, Reputation,
    PricingModel, BillingMethod,
};

pub use attestation::{
    AttestationBuilder, AttestationVerifier,
    CapabilityValidator, SafetyValidator,
    VerificationResult,
};

// Re-export pallet
pub use registry::pallet;

/// AIDID Library Version
pub const VERSION: &str = "1.0.0";

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_aidid_format() {
        let did = AIDid::new(AIType::LLM, "gpt4-turbo".to_string());
        #[cfg(feature = "std")]
        assert_eq!(did.to_string(), "did:etrid:ai:llm:gpt4-turbo");
    }

    #[test]
    fn test_capability_matching() {
        let mut caps = Capabilities::default();
        caps.tasks.push(Task::TextGeneration);
        caps.tasks.push(Task::CodeGeneration);

        assert!(caps.can_perform(&Task::TextGeneration));
        assert!(caps.can_perform(&Task::CodeGeneration));
        assert!(!caps.can_perform(&Task::ImageGeneration));
    }

    #[test]
    fn test_ai_type_conversion() {
        assert_eq!(AIType::LLM.to_str(), "llm");
        assert_eq!(AIType::Vision.to_str(), "vision");
    }

    #[test]
    fn test_reputation_calculation() {
        let mut rep = Reputation::default();

        // Perfect score initially
        assert_eq!(rep.success_rate(), 10000);

        // Record some successful inferences
        for _ in 0..90 {
            rep.record_inference(true);
        }
        for _ in 0..10 {
            rep.record_inference(false);
        }

        // Should be 90%
        assert!(rep.success_rate() >= 9000 && rep.success_rate() <= 9100);
    }
}
