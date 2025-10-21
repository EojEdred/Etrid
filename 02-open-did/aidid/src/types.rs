//! AIDID Types
//!
//! Core types for AI Decentralized Identity

use codec::{Decode, Encode};
use scale_info::TypeInfo;
use sp_std::prelude::*;

#[cfg(feature = "std")]
use serde::{Deserialize, Serialize};

/// AI DID identifier
#[derive(Debug, Clone, PartialEq, Eq, Hash, Encode, Decode, TypeInfo)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct AIDid {
    pub ai_type: AIType,
    pub identifier: Vec<u8>,
}

impl AIDid {
    pub fn new(ai_type: AIType, identifier: String) -> Self {
        Self {
            ai_type,
            identifier: identifier.into_bytes(),
        }
    }

    #[cfg(feature = "std")]
    pub fn to_string(&self) -> String {
        format!(
            "did:etrid:ai:{}:{}",
            self.ai_type.to_str(),
            String::from_utf8_lossy(&self.identifier)
        )
    }

    #[cfg(feature = "std")]
    pub fn from_string(did_str: &str) -> Result<Self, &'static str> {
        let parts: Vec<&str> = did_str.split(':').collect();

        if parts.len() != 5 {
            return Err("Invalid AIDID format");
        }

        if parts[0] != "did" || parts[1] != "etrid" || parts[2] != "ai" {
            return Err("Invalid AIDID prefix");
        }

        let ai_type = AIType::from_str(parts[3])?;
        let identifier = parts[4].to_string();

        Ok(Self::new(ai_type, identifier))
    }
}

/// AI Type Classification
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Encode, Decode, TypeInfo)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub enum AIType {
    /// Large Language Model
    LLM,
    /// Computer Vision Model
    Vision,
    /// Audio Processing Model
    Audio,
    /// Multi-modal AI System
    Multimodal,
    /// Autonomous AI Agent
    Agent,
    /// Ensemble of Multiple Models
    Ensemble,
}

impl AIType {
    pub fn to_str(&self) -> &'static str {
        match self {
            AIType::LLM => "llm",
            AIType::Vision => "vision",
            AIType::Audio => "audio",
            AIType::Multimodal => "multimodal",
            AIType::Agent => "agent",
            AIType::Ensemble => "ensemble",
        }
    }

    pub fn from_str(s: &str) -> Result<Self, &'static str> {
        match s {
            "llm" => Ok(AIType::LLM),
            "vision" => Ok(AIType::Vision),
            "audio" => Ok(AIType::Audio),
            "multimodal" => Ok(AIType::Multimodal),
            "agent" => Ok(AIType::Agent),
            "ensemble" => Ok(AIType::Ensemble),
            _ => Err("Unknown AI type"),
        }
    }
}

/// AI Task Categories
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Encode, Decode, TypeInfo)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub enum Task {
    TextGeneration,
    TextClassification,
    QuestionAnswering,
    CodeGeneration,
    Translation,
    Summarization,
    ImageGeneration,
    ImageClassification,
    ObjectDetection,
    AudioTranscription,
    AudioGeneration,
    VideoGeneration,
    VideoAnalysis,
    Reasoning,
    PlanningExecution,
    DataAnalysis,
}

/// AI Capabilities
#[derive(Debug, Clone, Default, PartialEq, Eq, Encode, Decode, TypeInfo)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct Capabilities {
    /// Tasks this AI can perform
    pub tasks: Vec<Task>,
    /// Supported input modalities
    pub input_modalities: Vec<Modality>,
    /// Supported output modalities
    pub output_modalities: Vec<Modality>,
    /// Supported languages (ISO 639-1 codes)
    pub languages: Vec<Vec<u8>>,
    /// Maximum context window (tokens)
    pub max_context: Option<u64>,
    /// Maximum output tokens
    pub max_output: Option<u64>,
}

impl Capabilities {
    pub fn can_perform(&self, task: &Task) -> bool {
        self.tasks.contains(task)
    }

    pub fn supports_input(&self, modality: &Modality) -> bool {
        self.input_modalities.contains(modality)
    }

    pub fn supports_output(&self, modality: &Modality) -> bool {
        self.output_modalities.contains(modality)
    }
}

/// Input/Output Modality
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Encode, Decode, TypeInfo)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub enum Modality {
    Text,
    Image,
    Audio,
    Video,
    StructuredData,
    Code,
}

/// AI Restrictions
#[derive(Debug, Clone, Default, PartialEq, Eq, Encode, Decode, TypeInfo)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct Restrictions {
    /// Tasks this AI cannot perform
    pub prohibited_tasks: Vec<Task>,
    /// Content filtering enabled
    pub content_filtering: bool,
    /// Requires human oversight
    pub requires_supervision: bool,
    /// Cannot access real-time data
    pub no_realtime_data: bool,
    /// Knowledge cutoff date (UNIX timestamp)
    pub knowledge_cutoff: Option<u64>,
    /// Maximum rate limit (requests per second)
    pub rate_limit: Option<u32>,
}

/// Model Attestation
#[derive(Debug, Clone, PartialEq, Eq, Encode, Decode, TypeInfo)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct ModelAttestation {
    /// Model hash (SHA-256 of weights)
    pub model_hash: [u8; 32],
    /// Training data hash (IPFS CID)
    pub training_data_hash: Vec<u8>,
    /// Model version
    pub version: Vec<u8>,
    /// Training timestamp (UNIX)
    pub training_date: u64,
    /// Reproducible build
    pub reproducible: bool,
    /// Benchmark scores
    pub benchmarks: Vec<Benchmark>,
}

/// Benchmark Result
#[derive(Debug, Clone, PartialEq, Eq, Encode, Decode, TypeInfo)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct Benchmark {
    /// Benchmark name (e.g., "MMLU", "HumanEval")
    pub name: Vec<u8>,
    /// Score (0-10000 representing 0.00% - 100.00%)
    pub score: u32,
}

/// AI Profile - Complete metadata for an AI entity
#[derive(Debug, Clone, PartialEq, Eq, Encode, Decode, TypeInfo)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct AIProfile {
    /// AI type
    pub ai_type: AIType,
    /// Model version
    pub version: Vec<u8>,
    /// Model architecture description
    pub architecture: Vec<u8>,
    /// Number of parameters (as string for very large numbers)
    pub parameters: Vec<u8>,
    /// Capabilities
    pub capabilities: Capabilities,
    /// Restrictions
    pub restrictions: Restrictions,
    /// Safety features
    pub safety: SafetyProfile,
}

/// Safety Profile
#[derive(Debug, Clone, Default, PartialEq, Eq, Encode, Decode, TypeInfo)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct SafetyProfile {
    /// Alignment method (e.g., "RLHF", "Constitutional AI")
    pub alignment_method: Vec<u8>,
    /// Content filtering enabled
    pub content_filtering: bool,
    /// Bias evaluation performed
    pub bias_evaluated: bool,
    /// Toxicity score (0-10000 representing 0.00% - 100.00%)
    pub toxicity_score: u32,
}

/// Authorization Permission
#[derive(Debug, Clone, PartialEq, Eq, Encode, Decode, TypeInfo)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct Permission {
    /// Action this AI is allowed to perform
    pub action: Vec<u8>,
    /// Resource this applies to
    pub resource: Vec<u8>,
    /// Conditions that must be met
    pub conditions: Vec<Vec<u8>>,
}

/// Reputation Score
#[derive(Debug, Clone, Default, PartialEq, Eq, Encode, Decode, TypeInfo)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct Reputation {
    /// Overall score (0-10000 representing 0.00 - 100.00)
    pub score: u32,
    /// Total number of inferences performed
    pub total_inferences: u64,
    /// Successful inferences
    pub successful_inferences: u64,
    /// Failed inferences
    pub failed_inferences: u64,
    /// User ratings (0-10000)
    pub user_rating: u32,
    /// Number of user ratings
    pub rating_count: u32,
    /// Uptime percentage (0-10000)
    pub uptime: u32,
    /// Number of incidents reported
    pub incidents: u32,
}

impl Reputation {
    pub fn success_rate(&self) -> u32 {
        if self.total_inferences == 0 {
            return 10000; // 100% for new AIs
        }
        ((self.successful_inferences * 10000) / self.total_inferences) as u32
    }

    pub fn record_inference(&mut self, success: bool) {
        self.total_inferences += 1;
        if success {
            self.successful_inferences += 1;
        } else {
            self.failed_inferences += 1;
        }
        self.score = self.calculate_score();
    }

    pub fn add_rating(&mut self, rating: u32) {
        let total = (self.user_rating as u64 * self.rating_count as u64) + rating as u64;
        self.rating_count += 1;
        self.user_rating = (total / self.rating_count as u64) as u32;
        self.score = self.calculate_score();
    }

    fn calculate_score(&self) -> u32 {
        // Weighted average of success rate, user rating, and uptime
        let success = self.success_rate();
        let rating = self.user_rating;
        let uptime = self.uptime;

        // 40% success rate, 40% user rating, 20% uptime
        (success * 4 + rating * 4 + uptime * 2) / 10
    }
}

/// Pricing Model
#[derive(Debug, Clone, PartialEq, Eq, Encode, Decode, TypeInfo)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct PricingModel {
    /// Cost per input token (in smallest unit of EDSC)
    pub input_token_price: u128,
    /// Cost per output token (in smallest unit of EDSC)
    pub output_token_price: u128,
    /// Fixed cost per request (optional)
    pub request_price: Option<u128>,
    /// Billing method
    pub billing_method: BillingMethod,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Encode, Decode, TypeInfo)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub enum BillingMethod {
    PerToken,
    PerRequest,
    Subscription,
    PayAsYouGo,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_aidid_creation() {
        let did = AIDid::new(AIType::LLM, "gpt4-turbo".to_string());
        assert_eq!(did.ai_type, AIType::LLM);
        assert_eq!(did.to_string(), "did:etrid:ai:llm:gpt4-turbo");
    }

    #[test]
    fn test_aidid_parsing() {
        let did_str = "did:etrid:ai:vision:yolo-v8";
        let did = AIDid::from_string(did_str).unwrap();
        assert_eq!(did.ai_type, AIType::Vision);
        assert_eq!(String::from_utf8(did.identifier).unwrap(), "yolo-v8");
    }

    #[test]
    fn test_capabilities() {
        let mut caps = Capabilities::default();
        caps.tasks.push(Task::TextGeneration);
        caps.tasks.push(Task::CodeGeneration);
        caps.input_modalities.push(Modality::Text);
        caps.output_modalities.push(Modality::Text);

        assert!(caps.can_perform(&Task::TextGeneration));
        assert!(caps.can_perform(&Task::CodeGeneration));
        assert!(!caps.can_perform(&Task::ImageGeneration));
        assert!(caps.supports_input(&Modality::Text));
        assert!(!caps.supports_input(&Modality::Image));
    }

    #[test]
    fn test_reputation() {
        let mut rep = Reputation::default();

        // Record 100 successful inferences
        for _ in 0..100 {
            rep.record_inference(true);
        }
        assert_eq!(rep.success_rate(), 10000); // 100%

        // Record 10 failures
        for _ in 0..10 {
            rep.record_inference(false);
        }
        assert_eq!(rep.total_inferences, 110);
        assert_eq!(rep.successful_inferences, 100);
        // Success rate should be ~90.9%
        assert!(rep.success_rate() > 9000 && rep.success_rate() < 9100);
    }

    #[test]
    fn test_ai_type_conversion() {
        assert_eq!(AIType::LLM.to_str(), "llm");
        assert_eq!(AIType::from_str("llm").unwrap(), AIType::LLM);
        assert_eq!(AIType::Vision.to_str(), "vision");
        assert_eq!(AIType::from_str("vision").unwrap(), AIType::Vision);
    }
}
