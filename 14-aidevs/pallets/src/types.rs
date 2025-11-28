//! AIDID Types
//!
//! Core types for AI Decentralized Identity

use codec::{Decode, Encode, MaxEncodedLen, DecodeWithMemTracking};
use frame_support::{BoundedVec, traits::ConstU32};
use scale_info::TypeInfo;
use sp_runtime::RuntimeDebug;
use sp_std::prelude::*;

use super::*;

/// AI Type Classification
#[derive(Clone, Copy, Encode, Decode, PartialEq, Eq, TypeInfo, RuntimeDebug, MaxEncodedLen)]
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

impl DecodeWithMemTracking for AIType {}

impl AIType {
    pub fn to_u8(&self) -> u8 {
        match self {
            AIType::LLM => 0,
            AIType::Vision => 1,
            AIType::Audio => 2,
            AIType::Multimodal => 3,
            AIType::Agent => 4,
            AIType::Ensemble => 5,
        }
    }
}

/// AI Task Categories
#[derive(Clone, Copy, Encode, Decode, PartialEq, Eq, TypeInfo, RuntimeDebug, MaxEncodedLen)]
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

impl DecodeWithMemTracking for Task {}

/// Input/Output Modality
#[derive(Clone, Copy, Encode, Decode, PartialEq, Eq, TypeInfo, RuntimeDebug, MaxEncodedLen)]
pub enum Modality {
    Text,
    Image,
    Audio,
    Video,
    StructuredData,
    Code,
}

impl DecodeWithMemTracking for Modality {}

/// AI Capabilities
#[derive(Clone, Encode, Decode, PartialEq, Eq, TypeInfo, RuntimeDebug, MaxEncodedLen)]
pub struct Capabilities {
    /// Tasks this AI can perform
    pub tasks: BoundedVec<Task, ConstU32<MAX_TASKS>>,
    /// Supported input modalities
    pub input_modalities: BoundedVec<Modality, ConstU32<MAX_MODALITIES>>,
    /// Supported output modalities
    pub output_modalities: BoundedVec<Modality, ConstU32<MAX_MODALITIES>>,
    /// Supported languages (ISO 639-1 codes)
    pub languages: BoundedVec<BoundedVec<u8, ConstU32<MAX_LANGUAGE_LENGTH>>, ConstU32<MAX_LANGUAGES>>,
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

impl Default for Capabilities {
    fn default() -> Self {
        Self {
            tasks: BoundedVec::default(),
            input_modalities: BoundedVec::default(),
            output_modalities: BoundedVec::default(),
            languages: BoundedVec::default(),
            max_context: None,
            max_output: None,
        }
    }
}

impl DecodeWithMemTracking for Capabilities {}

/// AI Restrictions
#[derive(Clone, Encode, Decode, PartialEq, Eq, TypeInfo, RuntimeDebug, MaxEncodedLen)]
pub struct Restrictions {
    /// Tasks this AI cannot perform
    pub prohibited_tasks: BoundedVec<Task, ConstU32<MAX_PROHIBITED_TASKS>>,
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

impl Default for Restrictions {
    fn default() -> Self {
        Self {
            prohibited_tasks: BoundedVec::default(),
            content_filtering: false,
            requires_supervision: false,
            no_realtime_data: false,
            knowledge_cutoff: None,
            rate_limit: None,
        }
    }
}

impl DecodeWithMemTracking for Restrictions {}

/// Safety Profile
#[derive(Clone, Encode, Decode, PartialEq, Eq, TypeInfo, RuntimeDebug, MaxEncodedLen)]
pub struct SafetyProfile {
    /// Alignment method (e.g., "RLHF", "Constitutional AI")
    pub alignment_method: BoundedVec<u8, ConstU32<MAX_ALIGNMENT_METHOD_LENGTH>>,
    /// Content filtering enabled
    pub content_filtering: bool,
    /// Bias evaluation performed
    pub bias_evaluated: bool,
    /// Toxicity score (0-10000 representing 0.00% - 100.00%)
    pub toxicity_score: u32,
}

impl Default for SafetyProfile {
    fn default() -> Self {
        Self {
            alignment_method: BoundedVec::default(),
            content_filtering: false,
            bias_evaluated: false,
            toxicity_score: 0,
        }
    }
}

impl DecodeWithMemTracking for SafetyProfile {}

/// Benchmark Result
#[derive(Clone, Encode, Decode, PartialEq, Eq, TypeInfo, RuntimeDebug, MaxEncodedLen)]
pub struct Benchmark {
    /// Benchmark name (e.g., "MMLU", "HumanEval")
    pub name: BoundedVec<u8, ConstU32<MAX_BENCHMARK_NAME_LENGTH>>,
    /// Score (0-10000 representing 0.00% - 100.00%)
    pub score: u32,
}

impl DecodeWithMemTracking for Benchmark {}

/// Model Attestation
#[derive(Clone, Encode, Decode, PartialEq, Eq, TypeInfo, RuntimeDebug, MaxEncodedLen)]
pub struct ModelAttestation {
    /// Model hash (SHA-256 of weights)
    pub model_hash: [u8; 32],
    /// Training data hash (IPFS CID or similar)
    pub training_data_hash: BoundedVec<u8, ConstU32<MAX_TRAINING_DATA_HASH_LENGTH>>,
    /// Model version
    pub version: BoundedVec<u8, ConstU32<MAX_VERSION_LENGTH>>,
    /// Training timestamp (UNIX)
    pub training_date: u64,
    /// Reproducible build
    pub reproducible: bool,
    /// Benchmark scores
    pub benchmarks: BoundedVec<Benchmark, ConstU32<MAX_BENCHMARKS>>,
}

impl DecodeWithMemTracking for ModelAttestation {}

/// AI Profile - Complete metadata for an AI entity
#[derive(Clone, Encode, Decode, PartialEq, Eq, TypeInfo, RuntimeDebug, MaxEncodedLen)]
pub struct AIProfile {
    /// AI type
    pub ai_type: AIType,
    /// Model version
    pub version: BoundedVec<u8, ConstU32<MAX_VERSION_LENGTH>>,
    /// Model architecture description
    pub architecture: BoundedVec<u8, ConstU32<MAX_ARCHITECTURE_LENGTH>>,
    /// Number of parameters (as string for very large numbers)
    pub parameters: BoundedVec<u8, ConstU32<MAX_PARAMETERS_LENGTH>>,
    /// Capabilities
    pub capabilities: Capabilities,
    /// Restrictions
    pub restrictions: Restrictions,
    /// Safety features
    pub safety: SafetyProfile,
}

impl DecodeWithMemTracking for AIProfile {}

/// Authorization Permission
#[derive(Clone, Encode, Decode, PartialEq, Eq, TypeInfo, RuntimeDebug, MaxEncodedLen)]
pub struct Permission {
    /// Action this AI is allowed to perform
    pub action: BoundedVec<u8, ConstU32<MAX_ACTION_LENGTH>>,
    /// Resource this applies to
    pub resource: BoundedVec<u8, ConstU32<MAX_RESOURCE_LENGTH>>,
    /// Conditions that must be met
    pub conditions: BoundedVec<BoundedVec<u8, ConstU32<MAX_CONDITION_LENGTH>>, ConstU32<MAX_CONDITIONS>>,
}

impl DecodeWithMemTracking for Permission {}

/// Reputation Score
#[derive(Clone, Encode, Decode, PartialEq, Eq, TypeInfo, RuntimeDebug, MaxEncodedLen, Default)]
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
        self.total_inferences = self.total_inferences.saturating_add(1);
        if success {
            self.successful_inferences = self.successful_inferences.saturating_add(1);
        } else {
            self.failed_inferences = self.failed_inferences.saturating_add(1);
        }
        self.score = self.calculate_score();
    }

    pub fn add_rating(&mut self, rating: u32) {
        let total = (self.user_rating as u64)
            .saturating_mul(self.rating_count as u64)
            .saturating_add(rating as u64);
        self.rating_count = self.rating_count.saturating_add(1);
        self.user_rating = (total / self.rating_count as u64) as u32;
        self.score = self.calculate_score();
    }

    fn calculate_score(&self) -> u32 {
        // Weighted average of success rate, user rating, and uptime
        let success = self.success_rate();
        let rating = self.user_rating;
        let uptime = self.uptime;

        // 40% success rate, 40% user rating, 20% uptime
        success.saturating_mul(4)
            .saturating_add(rating.saturating_mul(4))
            .saturating_add(uptime.saturating_mul(2))
            / 10
    }
}

impl DecodeWithMemTracking for Reputation {}

/// Pricing Model
#[derive(Clone, Encode, Decode, PartialEq, Eq, TypeInfo, RuntimeDebug, MaxEncodedLen)]
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

impl DecodeWithMemTracking for PricingModel {}

#[derive(Clone, Copy, Encode, Decode, PartialEq, Eq, TypeInfo, RuntimeDebug, MaxEncodedLen)]
pub enum BillingMethod {
    PerToken,
    PerRequest,
    Subscription,
    PayAsYouGo,
}

impl DecodeWithMemTracking for BillingMethod {}
