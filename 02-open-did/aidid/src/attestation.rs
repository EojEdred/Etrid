//! AIDID Attestation
//!
//! Model attestation and verification logic

use codec::{Decode, Encode};
use scale_info::TypeInfo;
use sp_std::prelude::*;
use sp_core::{H256, crypto::AccountId32};

use crate::types::*;

/// Attestation verification result
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum VerificationResult {
    Valid,
    Invalid(Vec<u8>), // Error message
}

/// Model Attestation Verifier
pub struct AttestationVerifier;

impl AttestationVerifier {
    /// Verify model attestation
    pub fn verify(attestation: &ModelAttestation) -> VerificationResult {
        // Verify model hash is not zero
        if attestation.model_hash == [0u8; 32] {
            return VerificationResult::Invalid(b"Model hash is zero".to_vec());
        }

        // Verify training data hash is not empty
        if attestation.training_data_hash.is_empty() {
            return VerificationResult::Invalid(b"Training data hash is empty".to_vec());
        }

        // Verify version is not empty
        if attestation.version.is_empty() {
            return VerificationResult::Invalid(b"Version is empty".to_vec());
        }

        // Verify benchmark scores are valid (0-10000)
        for benchmark in &attestation.benchmarks {
            if benchmark.score > 10000 {
                return VerificationResult::Invalid(b"Invalid benchmark score".to_vec());
            }
        }

        VerificationResult::Valid
    }

    /// Verify benchmark data
    pub fn verify_benchmarks(benchmarks: &[Benchmark]) -> VerificationResult {
        for benchmark in benchmarks {
            // Verify score is in valid range
            if benchmark.score > 10000 {
                return VerificationResult::Invalid(
                    b"Benchmark score out of range (0-10000)".to_vec()
                );
            }

            // Verify benchmark name is not empty
            if benchmark.name.is_empty() {
                return VerificationResult::Invalid(b"Benchmark name is empty".to_vec());
            }
        }

        VerificationResult::Valid
    }

    /// Calculate attestation hash
    pub fn hash_attestation(attestation: &ModelAttestation) -> H256 {
        let encoded = attestation.encode();
        H256::from(sp_io::hashing::blake2_256(&encoded))
    }
}

/// Attestation Builder - Helper for creating attestations
pub struct AttestationBuilder {
    model_hash: Option<[u8; 32]>,
    training_data_hash: Option<Vec<u8>>,
    version: Option<Vec<u8>>,
    training_date: Option<u64>,
    reproducible: bool,
    benchmarks: Vec<Benchmark>,
}

impl AttestationBuilder {
    pub fn new() -> Self {
        Self {
            model_hash: None,
            training_data_hash: None,
            version: None,
            training_date: None,
            reproducible: false,
            benchmarks: Vec::new(),
        }
    }

    pub fn model_hash(mut self, hash: [u8; 32]) -> Self {
        self.model_hash = Some(hash);
        self
    }

    pub fn training_data_hash(mut self, hash: Vec<u8>) -> Self {
        self.training_data_hash = Some(hash);
        self
    }

    pub fn version(mut self, version: Vec<u8>) -> Self {
        self.version = Some(version);
        self
    }

    pub fn training_date(mut self, date: u64) -> Self {
        self.training_date = Some(date);
        self
    }

    pub fn reproducible(mut self, reproducible: bool) -> Self {
        self.reproducible = reproducible;
        self
    }

    pub fn add_benchmark(mut self, name: Vec<u8>, score: u32) -> Self {
        self.benchmarks.push(Benchmark { name, score });
        self
    }

    pub fn build(self) -> Result<ModelAttestation, &'static str> {
        Ok(ModelAttestation {
            model_hash: self.model_hash.ok_or("Model hash is required")?,
            training_data_hash: self.training_data_hash.ok_or("Training data hash is required")?,
            version: self.version.ok_or("Version is required")?,
            training_date: self.training_date.ok_or("Training date is required")?,
            reproducible: self.reproducible,
            benchmarks: self.benchmarks,
        })
    }
}

impl Default for AttestationBuilder {
    fn default() -> Self {
        Self::new()
    }
}

/// Capability Validator
pub struct CapabilityValidator;

impl CapabilityValidator {
    /// Validate capabilities
    pub fn validate(capabilities: &Capabilities) -> VerificationResult {
        // Check tasks are not empty for non-agent AIs
        if capabilities.tasks.is_empty() {
            return VerificationResult::Invalid(b"At least one task must be specified".to_vec());
        }

        // Check modalities
        if capabilities.input_modalities.is_empty() {
            return VerificationResult::Invalid(b"At least one input modality required".to_vec());
        }

        if capabilities.output_modalities.is_empty() {
            return VerificationResult::Invalid(b"At least one output modality required".to_vec());
        }

        // Validate context window
        if let Some(max_context) = capabilities.max_context {
            if max_context == 0 {
                return VerificationResult::Invalid(b"Max context must be > 0".to_vec());
            }
        }

        VerificationResult::Valid
    }

    /// Check if capabilities match profile
    pub fn matches_profile(capabilities: &Capabilities, ai_type: &AIType) -> bool {
        match ai_type {
            AIType::LLM => {
                // LLMs should support text input/output
                capabilities.supports_input(&Modality::Text) &&
                capabilities.supports_output(&Modality::Text)
            }
            AIType::Vision => {
                // Vision models should support image input
                capabilities.supports_input(&Modality::Image)
            }
            AIType::Audio => {
                // Audio models should support audio input
                capabilities.supports_input(&Modality::Audio)
            }
            AIType::Multimodal => {
                // Multimodal should support multiple modalities
                capabilities.input_modalities.len() > 1 ||
                capabilities.output_modalities.len() > 1
            }
            AIType::Agent | AIType::Ensemble => {
                // Agents and ensembles are flexible
                true
            }
        }
    }
}

/// Safety Profile Validator
pub struct SafetyValidator;

impl SafetyValidator {
    /// Validate safety profile
    pub fn validate(safety: &SafetyProfile) -> VerificationResult {
        // Validate toxicity score
        if safety.toxicity_score > 10000 {
            return VerificationResult::Invalid(b"Toxicity score out of range".to_vec());
        }

        // Alignment method should not be empty
        if safety.alignment_method.is_empty() {
            return VerificationResult::Invalid(b"Alignment method required".to_vec());
        }

        VerificationResult::Valid
    }

    /// Check if AI meets safety requirements
    pub fn meets_requirements(safety: &SafetyProfile, min_safety_score: u32) -> bool {
        // Calculate safety score (inverse of toxicity)
        let safety_score = 10000 - safety.toxicity_score;

        safety_score >= min_safety_score &&
        safety.content_filtering &&
        safety.bias_evaluated
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_attestation_builder() {
        let attestation = AttestationBuilder::new()
            .model_hash([1u8; 32])
            .training_data_hash(b"QmXyz...".to_vec())
            .version(b"v1.0.0".to_vec())
            .training_date(1634567890)
            .reproducible(true)
            .add_benchmark(b"MMLU".to_vec(), 8670)
            .add_benchmark(b"HumanEval".to_vec(), 7300)
            .build()
            .unwrap();

        assert_eq!(attestation.model_hash, [1u8; 32]);
        assert_eq!(attestation.benchmarks.len(), 2);
        assert_eq!(attestation.reproducible, true);
    }

    #[test]
    fn test_attestation_verification() {
        let attestation = ModelAttestation {
            model_hash: [1u8; 32],
            training_data_hash: b"QmXyz...".to_vec(),
            version: b"v1.0.0".to_vec(),
            training_date: 1634567890,
            reproducible: true,
            benchmarks: vec![
                Benchmark {
                    name: b"MMLU".to_vec(),
                    score: 8670,
                }
            ],
        };

        assert_eq!(AttestationVerifier::verify(&attestation), VerificationResult::Valid);
    }

    #[test]
    fn test_invalid_attestation() {
        let attestation = ModelAttestation {
            model_hash: [0u8; 32], // Invalid: all zeros
            training_data_hash: vec![],
            version: vec![],
            training_date: 0,
            reproducible: false,
            benchmarks: vec![],
        };

        match AttestationVerifier::verify(&attestation) {
            VerificationResult::Invalid(_) => (),
            VerificationResult::Valid => panic!("Should be invalid"),
        }
    }

    #[test]
    fn test_capability_validation() {
        let mut caps = Capabilities::default();
        caps.tasks.push(Task::TextGeneration);
        caps.input_modalities.push(Modality::Text);
        caps.output_modalities.push(Modality::Text);

        assert_eq!(CapabilityValidator::validate(&caps), VerificationResult::Valid);
        assert!(CapabilityValidator::matches_profile(&caps, &AIType::LLM));
    }

    #[test]
    fn test_safety_validation() {
        let safety = SafetyProfile {
            alignment_method: b"RLHF".to_vec(),
            content_filtering: true,
            bias_evaluated: true,
            toxicity_score: 200, // 2%
        };

        assert_eq!(SafetyValidator::validate(&safety), VerificationResult::Valid);
        assert!(SafetyValidator::meets_requirements(&safety, 9000)); // Requires 90%+ safety
    }
}
