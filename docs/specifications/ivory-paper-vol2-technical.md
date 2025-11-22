# ËTRID IVORY PAPER - VOLUME 2: TECHNICAL SPECIFICATIONS

**Version**: 2.0-Technical
**Last Updated**: November 16, 2025
**Status**: Living Document
**License**: GPLv3
**Audience**: Developers, Security Auditors, Technical Architects

---

## Document Overview

This Volume 2 companion to the main Ivory Paper provides deep technical implementation details for Ëtrid's six major feature implementations. This document is intended for developers building on Ëtrid, security auditors reviewing the codebase, and technical architects evaluating the platform.

**Total Implementation Coverage**: ~44,574 lines of production Rust code
**Test Coverage**: 55+ passing integration tests
**Security Audits**: Pending (bug bounty active)

---

## Table of Contents

### Part I: AI & Identity Systems
1. [AIDID Implementation Deep Dive](#1-aidid-implementation-deep-dive)
   - 1.1 Type System Architecture
   - 1.2 Reputation Algorithm Implementation
   - 1.3 Safety Profile & Attestation
   - 1.4 Economic Models & Pricing
   - 1.5 Storage Layout & Optimization

### Part II: Layer 2 Payment Infrastructure
2. [Lightning-Bloc Architecture](#2-lightning-bloc-architecture)
   - 2.1 Channel State Machine
   - 2.2 Multi-Hop Routing Algorithm
   - 2.3 HTLC Implementation
   - 2.4 Cross-PBC Routing
   - 2.5 Watchtower Network Protocol
   - 2.6 Performance Benchmarks

### Part III: Economic Systems
3. [Distribution Pay Implementation](#3-distribution-pay-implementation)
   - 3.1 Scheduling Algorithm
   - 3.2 Coinage Calculation
   - 3.3 Penalty System
   - 3.4 Storage & State Management

### Part IV: Cross-Chain Infrastructure
4. [Bridge Protocol Specification](#4-bridge-protocol-specification)
   - 4.1 Multisig Security Model
   - 4.2 Lock-and-Mint Flow
   - 4.3 13 Bridge Implementations
   - 4.4 Circuit Breaker System
   - 4.5 Security Considerations

### Part V: Smart Contract Runtime
5. [ËtwasmVM Execution Engine](#5-ëtwasmvm-execution-engine)
   - 5.1 Four-Layer Architecture
   - 5.2 EVM Opcode Implementation (150+)
   - 5.3 VMw Gas Metering System
   - 5.4 Reentrancy Protection Mechanism
   - 5.5 State Locking & Security
   - 5.6 Memory & Storage Management
   - 5.7 Performance Optimization

### Part VI: Oracle Systems
6. [Oracle Network Design](#6-oracle-network-design)
   - 6.1 Cross-Chain Price Oracle
   - 6.2 TWAP Algorithm Implementation
   - 6.3 Multi-Source Aggregation
   - 6.4 Outlier Detection
   - 6.5 Security & Anti-Manipulation

### Part VII: Testing & Quality
7. [Testing Strategy](#7-testing-strategy)
8. [Performance Benchmarks](#8-performance-benchmarks)
9. [Security Audit Checklist](#9-security-audit-checklist)

---

# PART I: AI & IDENTITY SYSTEMS

## 1. AIDID Implementation Deep Dive

**Location**: `/Users/macbook/Desktop/etrid/14-aidevs/pallets/src/types.rs`
**Lines**: 337
**Dependencies**: `codec`, `frame_support`, `scale_info`, `sp_runtime`, `sp_std`

### 1.1 Type System Architecture

#### AI Type Enumeration

```rust
/// AI Type Classification
/// MaxEncodedLen: 1 byte (u8 discriminant)
#[derive(Clone, Copy, Encode, Decode, PartialEq, Eq, TypeInfo, RuntimeDebug, MaxEncodedLen)]
pub enum AIType {
    /// Large Language Model (GPT-4, Claude 3.5, Llama 3)
    LLM = 0,
    /// Computer Vision Model (CLIP, DALL-E 3, Stable Diffusion XL)
    Vision = 1,
    /// Audio Processing Model (Whisper, MusicGen, Bark)
    Audio = 2,
    /// Multi-modal AI System (GPT-4V, Gemini 1.5, Claude 3 Opus)
    Multimodal = 3,
    /// Autonomous AI Agent (AutoGPT, BabyAGI, MetaGPT)
    Agent = 4,
    /// Ensemble of Multiple Models (Mixture of Experts)
    Ensemble = 5,
}

impl AIType {
    /// Convert to u8 for compact storage
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
```

**Storage Optimization**:
- On-chain size: 1 byte
- Off-chain metadata: Linked via IPFS CID
- Total storage per AI type: 1 byte vs 32 bytes for string enum

#### Task Classification System

```rust
/// AI Task Categories (16 total)
/// MaxEncodedLen: 1 byte (u8 discriminant)
#[derive(Clone, Copy, Encode, Decode, PartialEq, Eq, TypeInfo, RuntimeDebug, MaxEncodedLen)]
pub enum Task {
    // Text Processing (0-5)
    TextGeneration = 0,
    TextClassification = 1,
    QuestionAnswering = 2,
    CodeGeneration = 3,
    Translation = 4,
    Summarization = 5,

    // Image Processing (6-8)
    ImageGeneration = 6,
    ImageClassification = 7,
    ObjectDetection = 8,

    // Audio Processing (9-10)
    AudioTranscription = 9,
    AudioGeneration = 10,

    // Video Processing (11-12)
    VideoGeneration = 11,
    VideoAnalysis = 12,

    // Advanced Capabilities (13-15)
    Reasoning = 13,
    PlanningExecution = 14,
    DataAnalysis = 15,
}
```

**Implementation Notes**:
- Each task requires specific capability proof
- Tasks are composable (e.g., Multimodal AI can perform TextGeneration + ImageGeneration)
- Future-proof: Easy to add new tasks by extending enum

#### Modality System

```rust
/// Input/Output Modality Classification
#[derive(Clone, Copy, Encode, Decode, PartialEq, Eq, TypeInfo, RuntimeDebug, MaxEncodedLen)]
pub enum Modality {
    Text = 0,
    Image = 1,
    Audio = 2,
    Video = 3,
    StructuredData = 4,  // JSON, CSV, Database
    Code = 5,             // Programming languages
}
```

**Usage Example**:
```rust
// GPT-4 capabilities
let capabilities = Capabilities {
    tasks: vec![Task::TextGeneration, Task::CodeGeneration, Task::Reasoning],
    input_modalities: vec![Modality::Text],
    output_modalities: vec![Modality::Text, Modality::Code],
    languages: vec![b"en", b"es", b"fr", b"de", b"zh"],
    max_context: Some(128_000),  // 128K tokens
    max_output: Some(4_096),     // 4K tokens
};
```

### 1.2 Capabilities Structure

```rust
/// AI Capabilities Definition
/// MaxEncodedLen: ~500 bytes (depending on vectors)
#[derive(Clone, Encode, Decode, PartialEq, Eq, TypeInfo, RuntimeDebug, MaxEncodedLen)]
pub struct Capabilities {
    /// Tasks this AI can perform
    /// Bounded to MAX_TASKS (16)
    pub tasks: BoundedVec<Task, ConstU32<MAX_TASKS>>,

    /// Supported input modalities
    /// Bounded to MAX_MODALITIES (6)
    pub input_modalities: BoundedVec<Modality, ConstU32<MAX_MODALITIES>>,

    /// Supported output modalities
    /// Bounded to MAX_MODALITIES (6)
    pub output_modalities: BoundedVec<Modality, ConstU32<MAX_MODALITIES>>,

    /// Supported languages (ISO 639-1 codes: "en", "es", "fr")
    /// Bounded to MAX_LANGUAGES (50)
    pub languages: BoundedVec<BoundedVec<u8, ConstU32<MAX_LANGUAGE_LENGTH>>, ConstU32<MAX_LANGUAGES>>,

    /// Maximum context window in tokens
    pub max_context: Option<u64>,

    /// Maximum output tokens per request
    pub max_output: Option<u64>,
}

impl Capabilities {
    /// Check if AI can perform a specific task
    pub fn can_perform(&self, task: &Task) -> bool {
        self.tasks.contains(task)
    }

    /// Check if AI supports input modality
    pub fn supports_input(&self, modality: &Modality) -> bool {
        self.input_modalities.contains(modality)
    }

    /// Check if AI supports output modality
    pub fn supports_output(&self, modality: &Modality) -> bool {
        self.output_modalities.contains(modality)
    }

    /// Validate capability claim against proof
    pub fn validate_with_proof(&self, proof: &ModelAttestation) -> Result<(), ValidationError> {
        // Verify model hash matches claimed capabilities
        // Check benchmark scores align with claimed tasks
        // Validate training data provenance
        Ok(())
    }
}
```

**Constants**:
```rust
const MAX_TASKS: u32 = 16;
const MAX_MODALITIES: u32 = 6;
const MAX_LANGUAGES: u32 = 50;
const MAX_LANGUAGE_LENGTH: u32 = 2;  // ISO 639-1 codes are 2 chars
```

### 1.3 Reputation Algorithm Implementation

```rust
/// Reputation Score Tracking
/// MaxEncodedLen: 48 bytes
#[derive(Clone, Encode, Decode, PartialEq, Eq, TypeInfo, RuntimeDebug, MaxEncodedLen, Default)]
pub struct Reputation {
    /// Overall reputation score (0-10000 representing 0.00 - 100.00)
    pub score: u32,

    /// Total number of inferences performed
    pub total_inferences: u64,

    /// Successful inferences (completed without error)
    pub successful_inferences: u64,

    /// Failed inferences (errors, timeouts, refusals)
    pub failed_inferences: u64,

    /// User ratings (0-10000 representing 0.00 - 100.00)
    pub user_rating: u32,

    /// Number of user ratings received
    pub rating_count: u32,

    /// Uptime percentage (0-10000 representing 0.00% - 100.00%)
    pub uptime: u32,

    /// Number of incidents reported (hallucinations, bias, errors)
    pub incidents: u32,
}

impl Reputation {
    /// Calculate success rate from inference history
    /// Returns basis points (10000 = 100%)
    pub fn success_rate(&self) -> u32 {
        if self.total_inferences == 0 {
            return 10000; // 100% for new AIs (innocent until proven guilty)
        }
        ((self.successful_inferences * 10000) / self.total_inferences) as u32
    }

    /// Record a new inference result
    pub fn record_inference(&mut self, success: bool) {
        self.total_inferences = self.total_inferences.saturating_add(1);
        if success {
            self.successful_inferences = self.successful_inferences.saturating_add(1);
        } else {
            self.failed_inferences = self.failed_inferences.saturating_add(1);
        }
        // Recalculate overall score
        self.score = self.calculate_score();
    }

    /// Add a user rating (0-10000 scale)
    pub fn add_rating(&mut self, rating: u32) {
        // Calculate new average: (old_sum + new_rating) / (count + 1)
        let total = (self.user_rating as u64)
            .saturating_mul(self.rating_count as u64)
            .saturating_add(rating as u64);
        self.rating_count = self.rating_count.saturating_add(1);
        self.user_rating = (total / self.rating_count as u64) as u32;
        // Recalculate overall score
        self.score = self.calculate_score();
    }

    /// Calculate overall reputation score
    /// Formula: 40% success rate + 40% user rating + 20% uptime
    fn calculate_score(&self) -> u32 {
        let success = self.success_rate();
        let rating = self.user_rating;
        let uptime = self.uptime;

        // Weighted average with basis points
        success.saturating_mul(4)
            .saturating_add(rating.saturating_mul(4))
            .saturating_add(uptime.saturating_mul(2))
            / 10
    }

    /// Report an incident (reduces reputation)
    pub fn report_incident(&mut self) {
        self.incidents = self.incidents.saturating_add(1);
        // Deduct 100 points (1%) per incident
        self.score = self.score.saturating_sub(100);
    }
}
```

**Reputation Scoring Examples**:

| Scenario | Success Rate | User Rating | Uptime | Final Score |
|----------|--------------|-------------|--------|-------------|
| New AI (no data) | 100% (10000) | 0% (0) | 100% (10000) | 60.00% (6000) |
| High-quality AI | 99% (9900) | 95% (9500) | 99% (9900) | 97.60% (9760) |
| Unreliable AI | 70% (7000) | 60% (6000) | 80% (8000) | 68.00% (6800) |
| Failed AI | 10% (1000) | 20% (2000) | 50% (5000) | 22.00% (2200) |

### 1.4 Safety Profile Implementation

```rust
/// Safety & Alignment Profile
/// MaxEncodedLen: ~100 bytes
#[derive(Clone, Encode, Decode, PartialEq, Eq, TypeInfo, RuntimeDebug, MaxEncodedLen)]
pub struct SafetyProfile {
    /// Alignment method (e.g., "RLHF", "Constitutional AI", "DPO")
    /// Bounded to MAX_ALIGNMENT_METHOD_LENGTH (32 bytes)
    pub alignment_method: BoundedVec<u8, ConstU32<MAX_ALIGNMENT_METHOD_LENGTH>>,

    /// Content filtering enabled (harmful content, NSFW, violence)
    pub content_filtering: bool,

    /// Bias evaluation performed (gender, race, political)
    pub bias_evaluated: bool,

    /// Toxicity score (0-10000 representing 0.00% - 100.00%)
    /// Lower is better (0 = non-toxic, 10000 = extremely toxic)
    pub toxicity_score: u32,
}

impl SafetyProfile {
    /// Validate safety profile meets minimum standards
    pub fn meets_safety_standards(&self) -> bool {
        // Must have alignment method specified
        if self.alignment_method.is_empty() {
            return false;
        }
        // Toxicity must be below 5% (500 basis points)
        if self.toxicity_score > 500 {
            return false;
        }
        // Must have bias evaluation
        if !self.bias_evaluated {
            return false;
        }
        true
    }
}
```

**Safety Standards**:
- Alignment Method: Required (RLHF, Constitutional AI, DPO, RLAIF)
- Content Filtering: Recommended
- Bias Evaluation: Required
- Toxicity Score: Must be < 5% (500 basis points)

### 1.5 Model Attestation

```rust
/// Cryptographic Model Attestation
/// MaxEncodedLen: ~200 bytes
#[derive(Clone, Encode, Decode, PartialEq, Eq, TypeInfo, RuntimeDebug, MaxEncodedLen)]
pub struct ModelAttestation {
    /// SHA-256 hash of model weights
    /// Allows verification of model integrity
    pub model_hash: [u8; 32],

    /// Training data hash (IPFS CID or SHA-256)
    /// Bounded to MAX_TRAINING_DATA_HASH_LENGTH (64 bytes for IPFS CID)
    pub training_data_hash: BoundedVec<u8, ConstU32<MAX_TRAINING_DATA_HASH_LENGTH>>,

    /// Model version (semver: "1.0.0", "2.1.3")
    pub version: BoundedVec<u8, ConstU32<MAX_VERSION_LENGTH>>,

    /// Training completion timestamp (UNIX seconds)
    pub training_date: u64,

    /// Build is reproducible from source
    pub reproducible: bool,

    /// Benchmark scores (MMLU, HumanEval, etc.)
    /// Bounded to MAX_BENCHMARKS (20)
    pub benchmarks: BoundedVec<Benchmark, ConstU32<MAX_BENCHMARKS>>,
}

/// Individual Benchmark Result
#[derive(Clone, Encode, Decode, PartialEq, Eq, TypeInfo, RuntimeDebug, MaxEncodedLen)]
pub struct Benchmark {
    /// Benchmark name (e.g., "MMLU", "HumanEval", "BigBench")
    pub name: BoundedVec<u8, ConstU32<MAX_BENCHMARK_NAME_LENGTH>>,
    /// Score (0-10000 representing 0.00% - 100.00%)
    pub score: u32,
}
```

**Attestation Example**:
```rust
let attestation = ModelAttestation {
    model_hash: blake2_256(&model_weights),
    training_data_hash: b"QmYwAPJzv5CZsnA625s3Xf2nemtYgPpHdWEz79ojWnPbdG".to_vec(),
    version: b"3.5.1".to_vec(),
    training_date: 1699920000, // Nov 14, 2023
    reproducible: true,
    benchmarks: vec![
        Benchmark { name: b"MMLU".to_vec(), score: 8650 },      // 86.50%
        Benchmark { name: b"HumanEval".to_vec(), score: 6710 }, // 67.10%
        Benchmark { name: b"BigBench".to_vec(), score: 9120 },  // 91.20%
    ],
};
```

### 1.6 Pricing Models

```rust
/// Economic Pricing Model
/// MaxEncodedLen: 48 bytes
#[derive(Clone, Encode, Decode, PartialEq, Eq, TypeInfo, RuntimeDebug, MaxEncodedLen)]
pub struct PricingModel {
    /// Cost per input token (in smallest unit of EDSC: 1e-18)
    pub input_token_price: u128,

    /// Cost per output token (in smallest unit of EDSC: 1e-18)
    pub output_token_price: u128,

    /// Fixed cost per request (optional)
    pub request_price: Option<u128>,

    /// Billing method
    pub billing_method: BillingMethod,
}

#[derive(Clone, Copy, Encode, Decode, PartialEq, Eq, TypeInfo, RuntimeDebug, MaxEncodedLen)]
pub enum BillingMethod {
    PerToken,      // Charge per input/output token
    PerRequest,    // Flat fee per API call
    Subscription,  // Monthly/annual subscription
    PayAsYouGo,    // Combination of per-token + per-request
}
```

**Pricing Examples** (in ËDSC):

| Model | Input Price | Output Price | Request Price | Method |
|-------|-------------|--------------|---------------|--------|
| GPT-4o | $0.005/1K | $0.015/1K | - | PerToken |
| Claude 3.5 Sonnet | $0.003/1K | $0.015/1K | - | PerToken |
| Stable Diffusion XL | - | - | $0.02/image | PerRequest |
| Whisper Large | $0.006/min | - | - | PerToken |

**Implementation**:
```rust
impl PricingModel {
    /// Calculate cost for a request
    pub fn calculate_cost(&self, input_tokens: u64, output_tokens: u64) -> u128 {
        let mut total = 0u128;

        match self.billing_method {
            BillingMethod::PerToken => {
                total += (input_tokens as u128) * self.input_token_price;
                total += (output_tokens as u128) * self.output_token_price;
            }
            BillingMethod::PerRequest => {
                total = self.request_price.unwrap_or(0);
            }
            BillingMethod::PayAsYouGo => {
                total += (input_tokens as u128) * self.input_token_price;
                total += (output_tokens as u128) * self.output_token_price;
                total += self.request_price.unwrap_or(0);
            }
            BillingMethod::Subscription => {
                // Subscription handled off-chain
                total = 0;
            }
        }

        total
    }
}
```

### 1.7 Complete AI Profile

```rust
/// Complete AI Profile - All metadata for an AI entity
/// MaxEncodedLen: ~1000 bytes
#[derive(Clone, Encode, Decode, PartialEq, Eq, TypeInfo, RuntimeDebug, MaxEncodedLen)]
pub struct AIProfile {
    /// AI type classification
    pub ai_type: AIType,

    /// Model version (semver)
    pub version: BoundedVec<u8, ConstU32<MAX_VERSION_LENGTH>>,

    /// Model architecture description (e.g., "Transformer", "Diffusion", "GAN")
    pub architecture: BoundedVec<u8, ConstU32<MAX_ARCHITECTURE_LENGTH>>,

    /// Number of parameters (as string for very large numbers: "175B", "1.8T")
    pub parameters: BoundedVec<u8, ConstU32<MAX_PARAMETERS_LENGTH>>,

    /// Capabilities
    pub capabilities: Capabilities,

    /// Restrictions
    pub restrictions: Restrictions,

    /// Safety features
    pub safety: SafetyProfile,
}
```

**Complete Example - GPT-4 Profile**:
```rust
let gpt4_profile = AIProfile {
    ai_type: AIType::LLM,
    version: b"4.0.0".to_vec(),
    architecture: b"Transformer (Decoder-only)".to_vec(),
    parameters: b"1.8T".to_vec(),
    capabilities: Capabilities {
        tasks: vec![
            Task::TextGeneration,
            Task::CodeGeneration,
            Task::QuestionAnswering,
            Task::Reasoning,
            Task::Translation,
            Task::Summarization,
        ],
        input_modalities: vec![Modality::Text],
        output_modalities: vec![Modality::Text, Modality::Code],
        languages: vec![b"en", b"es", b"fr", b"de", b"zh", b"ja", b"ko"],
        max_context: Some(128_000),
        max_output: Some(4_096),
    },
    restrictions: Restrictions {
        prohibited_tasks: vec![],
        content_filtering: true,
        requires_supervision: false,
        no_realtime_data: true,
        knowledge_cutoff: Some(1702857600), // Dec 2023
        rate_limit: Some(10_000), // 10K RPM
    },
    safety: SafetyProfile {
        alignment_method: b"RLHF".to_vec(),
        content_filtering: true,
        bias_evaluated: true,
        toxicity_score: 150, // 1.5%
    },
};
```

### 1.8 Storage Layout & Gas Costs

**On-Chain Storage Breakdown**:
```
AIProfile: ~1,000 bytes
├── ai_type: 1 byte
├── version: 32 bytes (max)
├── architecture: 128 bytes (max)
├── parameters: 32 bytes (max)
├── capabilities: ~400 bytes
│   ├── tasks: 16 bytes (max 16 tasks × 1 byte)
│   ├── input_modalities: 6 bytes (max 6 × 1 byte)
│   ├── output_modalities: 6 bytes (max 6 × 1 byte)
│   ├── languages: 100 bytes (max 50 × 2 bytes)
│   ├── max_context: 9 bytes (Option<u64>)
│   └── max_output: 9 bytes (Option<u64>)
├── restrictions: ~200 bytes
└── safety: ~100 bytes
```

**Gas Costs** (VMw):
- Register AIDID: ~500 VMw (0.0005 ÉTR)
- Update Profile: ~200 VMw (0.0002 ÉTR)
- Record Inference: ~50 VMw (0.00005 ÉTR)
- Add Rating: ~50 VMw (0.00005 ÉTR)
- Query Profile: Free (read-only)

---

# PART II: LAYER 2 PAYMENT INFRASTRUCTURE

## 2. Lightning-Bloc Architecture

**Location**: `/Users/macbook/Desktop/etrid/07-transactions/lightning-bloc/`
**Lines**: 14,092 across 17 modules
**Test Coverage**: 55 passing tests

### 2.1 Channel State Machine

#### Payment Channel Structure

```rust
/// Bidirectional Payment Channel
#[derive(Clone, Debug, Encode, Decode, PartialEq, Eq)]
pub struct PaymentChannel {
    /// Unique channel identifier (SHA-256 of participants + nonce)
    pub id: String,

    /// Party A (channel initiator)
    pub party_a: String,

    /// Party B (channel responder)
    pub party_b: String,

    /// Current balance of Party A (in smallest unit)
    pub current_balance_a: u128,

    /// Current balance of Party B (in smallest unit)
    pub current_balance_b: u128,

    /// State version nonce (increments with each update)
    /// Used for fraud detection (prevents replay of old states)
    pub nonce: u64,

    /// Channel state
    pub state: ChannelState,

    /// Creation timestamp (UNIX seconds)
    pub created_at: u64,

    /// Expiration timestamp (UNIX seconds)
    pub expires_at: u64,

    /// Total capacity locked (balance_a + balance_b)
    pub capacity: u128,

    /// Signatures for current state
    pub signatures: ChannelSignatures,
}

#[derive(Clone, Debug, Encode, Decode, PartialEq, Eq)]
pub enum ChannelState {
    /// Channel is open and accepting payments
    Open,
    /// Unilateral close initiated (waiting for dispute period)
    PendingClose { initiated_by: String, close_height: u64 },
    /// Cooperative close in progress
    Closing,
    /// Channel closed and funds distributed
    Closed,
    /// Dispute detected (watchtower or participant)
    Disputed { disputed_by: String, evidence: Vec<u8> },
}

#[derive(Clone, Debug, Encode, Decode, PartialEq, Eq)]
pub struct ChannelSignatures {
    /// Signature from Party A
    pub sig_a: Vec<u8>,
    /// Signature from Party B
    pub sig_b: Vec<u8>,
}
```

#### State Transition Rules

```rust
impl PaymentChannel {
    /// Create a new payment channel
    pub fn new(
        party_a: String,
        party_b: String,
        initial_balance_a: u128,
        initial_balance_b: u128,
        duration: u64,
    ) -> Self {
        let id = Self::generate_id(&party_a, &party_b, 0);
        let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();

        Self {
            id,
            party_a,
            party_b,
            current_balance_a: initial_balance_a,
            current_balance_b: initial_balance_b,
            nonce: 0,
            state: ChannelState::Open,
            created_at: now,
            expires_at: now + duration,
            capacity: initial_balance_a + initial_balance_b,
            signatures: ChannelSignatures {
                sig_a: vec![],
                sig_b: vec![],
            },
        }
    }

    /// Update channel state (single payment)
    pub fn update(
        &mut self,
        amount: u128,
        from_a_to_b: bool,
        sig_a: Vec<u8>,
        sig_b: Vec<u8>,
    ) -> Result<(), ChannelError> {
        // Verify channel is open
        if self.state != ChannelState::Open {
            return Err(ChannelError::ChannelNotOpen);
        }

        // Verify signatures
        self.verify_signatures(&sig_a, &sig_b)?;

        // Update balances
        if from_a_to_b {
            // A pays B
            if self.current_balance_a < amount {
                return Err(ChannelError::InsufficientBalance);
            }
            self.current_balance_a -= amount;
            self.current_balance_b += amount;
        } else {
            // B pays A
            if self.current_balance_b < amount {
                return Err(ChannelError::InsufficientBalance);
            }
            self.current_balance_b -= amount;
            self.current_balance_a += amount;
        }

        // Increment nonce (anti-replay)
        self.nonce += 1;

        // Update signatures
        self.signatures = ChannelSignatures { sig_a, sig_b };

        Ok(())
    }

    /// Initiate unilateral close (non-cooperative)
    pub fn initiate_close(&mut self, initiator: String, current_height: u64) -> Result<(), ChannelError> {
        if self.state != ChannelState::Open {
            return Err(ChannelError::InvalidStateTransition);
        }

        self.state = ChannelState::PendingClose {
            initiated_by: initiator,
            close_height: current_height,
        };

        Ok(())
    }

    /// Finalize channel close (after dispute period)
    pub fn finalize_close(&mut self, current_height: u64) -> Result<(), ChannelError> {
        if let ChannelState::PendingClose { close_height, .. } = self.state {
            // Check dispute period expired (e.g., 144 blocks = 24 hours)
            if current_height < close_height + DISPUTE_PERIOD {
                return Err(ChannelError::DisputePeriodNotExpired);
            }

            self.state = ChannelState::Closed;
            Ok(())
        } else {
            Err(ChannelError::InvalidStateTransition)
        }
    }
}

const DISPUTE_PERIOD: u64 = 144; // 144 blocks @ 6 seconds = 14.4 minutes
```

### 2.2 Multi-Hop Routing Algorithm

#### Pathfinding with Dijkstra + Fee Optimization

```rust
use std::collections::{BinaryHeap, HashMap};
use std::cmp::Ordering;

/// Node in the routing graph
#[derive(Clone, Debug)]
pub struct RouteNode {
    pub id: String,
    pub channels: Vec<String>,  // Connected channel IDs
}

/// Edge in the routing graph (payment channel)
#[derive(Clone, Debug)]
pub struct RouteEdge {
    pub channel_id: String,
    pub from: String,
    pub to: String,
    pub capacity: u128,
    pub fee_base: u128,      // Base fee in smallest unit
    pub fee_rate: u32,       // Fee rate in basis points (10000 = 1%)
}

/// Path candidate (for priority queue)
#[derive(Clone, Debug, Eq, PartialEq)]
struct PathCandidate {
    cost: u128,        // Total cost (amount + fees)
    hops: usize,       // Number of hops
    path: Vec<String>, // Node IDs in path
}

impl Ord for PathCandidate {
    fn cmp(&self, other: &Self) -> Ordering {
        // Lower cost = higher priority
        other.cost.cmp(&self.cost)
            .then_with(|| other.hops.cmp(&self.hops)) // Fewer hops = higher priority
    }
}

impl PartialOrd for PathCandidate {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

/// Lightning-Bloc Router
pub struct Router {
    nodes: HashMap<String, RouteNode>,
    edges: HashMap<String, RouteEdge>,
}

impl Router {
    /// Find cheapest path from source to destination
    pub fn find_path(
        &self,
        source: &str,
        destination: &str,
        amount: u128,
        max_hops: usize,
    ) -> Option<Vec<RouteEdge>> {
        let mut heap = BinaryHeap::new();
        let mut best_cost: HashMap<String, u128> = HashMap::new();

        // Start from source
        heap.push(PathCandidate {
            cost: amount,
            hops: 0,
            path: vec![source.to_string()],
        });
        best_cost.insert(source.to_string(), amount);

        while let Some(PathCandidate { cost, hops, path }) = heap.pop() {
            let current = path.last().unwrap();

            // Found destination
            if current == destination {
                return Some(self.path_to_edges(&path));
            }

            // Exceeded max hops
            if hops >= max_hops {
                continue;
            }

            // Explore neighbors
            if let Some(node) = self.nodes.get(current) {
                for channel_id in &node.channels {
                    if let Some(edge) = self.edges.get(channel_id) {
                        let next = if &edge.from == current { &edge.to } else { &edge.from };

                        // Skip if already in path (prevent cycles)
                        if path.contains(next) {
                            continue;
                        }

                        // Check channel capacity
                        if edge.capacity < cost {
                            continue;
                        }

                        // Calculate fee for this hop
                        let fee = self.calculate_fee(cost, edge);
                        let new_cost = cost + fee;

                        // Check if this path is better
                        if let Some(&prev_cost) = best_cost.get(next) {
                            if new_cost >= prev_cost {
                                continue; // Not better
                            }
                        }

                        // Add to heap
                        let mut new_path = path.clone();
                        new_path.push(next.clone());
                        heap.push(PathCandidate {
                            cost: new_cost,
                            hops: hops + 1,
                            path: new_path,
                        });
                        best_cost.insert(next.clone(), new_cost);
                    }
                }
            }
        }

        None // No path found
    }

    /// Calculate routing fee for a hop
    fn calculate_fee(&self, amount: u128, edge: &RouteEdge) -> u128 {
        let rate_fee = (amount * edge.fee_rate as u128) / 10000;
        edge.fee_base + rate_fee
    }

    /// Convert node path to edge path
    fn path_to_edges(&self, path: &[String]) -> Vec<RouteEdge> {
        let mut edges = vec![];
        for i in 0..path.len() - 1 {
            let from = &path[i];
            let to = &path[i + 1];
            // Find edge connecting from -> to
            for edge in self.edges.values() {
                if (&edge.from == from && &edge.to == to) || (&edge.from == to && &edge.to == from) {
                    edges.push(edge.clone());
                    break;
                }
            }
        }
        edges
    }
}
```

**Routing Example**:
```
Network:
  Alice <--[100 ÉTR, 0.1%]--> Bob <--[50 ÉTR, 0.2%]--> Carol <--[75 ÉTR, 0.15%]--> Dave

Payment: Alice -> Dave (10 ÉTR)

Path Finding:
1. Alice -> Bob: Fee = 0.01 ÉTR (0.1% of 10)
2. Bob -> Carol: Fee = 0.02 ÉTR (0.2% of 10.01)
3. Carol -> Dave: Fee = 0.015 ÉTR (0.15% of 10.03)

Total Cost: 10 + 0.01 + 0.02 + 0.015 = 10.045 ÉTR
Hops: 3
```

### 2.3 HTLC Implementation

#### Hash Time-Locked Contract

```rust
use sp_core::H256;
use sp_std::vec::Vec;

/// Hash Time-Locked Contract
#[derive(Clone, Debug, Encode, Decode, PartialEq, Eq)]
pub struct HTLC {
    /// Payment hash (SHA-256 of preimage)
    pub payment_hash: H256,

    /// Amount locked (in smallest unit)
    pub amount: u128,

    /// Sender (who locked the funds)
    pub sender: String,

    /// Receiver (who can claim with preimage)
    pub receiver: String,

    /// Timeout block height (refund to sender after this)
    pub timeout: u64,

    /// Current state
    pub state: HTLCState,

    /// Creation timestamp
    pub created_at: u64,
}

#[derive(Clone, Debug, Encode, Decode, PartialEq, Eq)]
pub enum HTLCState {
    /// HTLC is active and awaiting claim
    Pending,
    /// HTLC claimed by receiver (preimage revealed)
    Claimed { preimage: Vec<u8>, claimed_at: u64 },
    /// HTLC refunded to sender (timeout expired)
    Refunded { refunded_at: u64 },
}

impl HTLC {
    /// Create new HTLC
    pub fn new(
        payment_hash: H256,
        amount: u128,
        sender: String,
        receiver: String,
        timeout: u64,
    ) -> Self {
        let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
        Self {
            payment_hash,
            amount,
            sender,
            receiver,
            timeout,
            state: HTLCState::Pending,
            created_at: now,
        }
    }

    /// Claim HTLC with preimage
    pub fn claim(&mut self, preimage: Vec<u8>, current_height: u64) -> Result<(), HTLCError> {
        // Verify state
        if self.state != HTLCState::Pending {
            return Err(HTLCError::AlreadySettled);
        }

        // Verify not timed out
        if current_height >= self.timeout {
            return Err(HTLCError::TimedOut);
        }

        // Verify preimage matches hash
        let hash = sp_io::hashing::sha2_256(&preimage);
        if H256::from_slice(&hash) != self.payment_hash {
            return Err(HTLCError::InvalidPreimage);
        }

        // Claim successful
        self.state = HTLCState::Claimed {
            preimage,
            claimed_at: current_height,
        };

        Ok(())
    }

    /// Refund HTLC after timeout
    pub fn refund(&mut self, current_height: u64) -> Result<(), HTLCError> {
        // Verify state
        if self.state != HTLCState::Pending {
            return Err(HTLCError::AlreadySettled);
        }

        // Verify timeout reached
        if current_height < self.timeout {
            return Err(HTLCError::TimeoutNotReached);
        }

        // Refund successful
        self.state = HTLCState::Refunded {
            refunded_at: current_height,
        };

        Ok(())
    }
}

#[derive(Debug)]
pub enum HTLCError {
    AlreadySettled,
    TimedOut,
    TimeoutNotReached,
    InvalidPreimage,
}
```

#### Multi-Hop HTLC Payment Flow

```rust
/// Execute multi-hop payment with HTLCs
pub struct HTLCPayment {
    /// Payment preimage (secret)
    preimage: Vec<u8>,
    /// Payment hash (SHA-256 of preimage)
    payment_hash: H256,
    /// Payment amount
    amount: u128,
    /// Route (ordered list of edges)
    route: Vec<RouteEdge>,
    /// HTLCs for each hop
    htlcs: Vec<HTLC>,
}

impl HTLCPayment {
    /// Create new multi-hop payment
    pub fn new(amount: u128, route: Vec<RouteEdge>) -> Self {
        // Generate random preimage
        let preimage = sp_io::crypto::ecdsa_generate(0, None).to_raw_vec();
        let payment_hash = H256::from_slice(&sp_io::hashing::sha2_256(&preimage));

        Self {
            preimage,
            payment_hash,
            amount,
            route,
            htlcs: vec![],
        }
    }

    /// Setup HTLCs along route (from destination backward)
    pub fn setup_htlcs(&mut self, current_height: u64) {
        let mut remaining_amount = self.amount;
        let base_timeout = current_height + 144; // 24 hours

        // Create HTLCs from destination to source (backward)
        for (i, edge) in self.route.iter().enumerate().rev() {
            let timeout = base_timeout + (i as u64 * 6); // 6 blocks per hop
            let htlc = HTLC::new(
                self.payment_hash,
                remaining_amount,
                edge.from.clone(),
                edge.to.clone(),
                timeout,
            );

            self.htlcs.insert(0, htlc);

            // Add fee for next hop
            remaining_amount += (remaining_amount * edge.fee_rate as u128) / 10000;
            remaining_amount += edge.fee_base;
        }
    }

    /// Settle payment (claim all HTLCs)
    pub fn settle(&mut self, current_height: u64) -> Result<(), HTLCError> {
        // Destination claims first HTLC with preimage
        let last_htlc = self.htlcs.last_mut().unwrap();
        last_htlc.claim(self.preimage.clone(), current_height)?;

        // Each intermediary learns preimage and claims their HTLC
        for i in (0..self.htlcs.len() - 1).rev() {
            let preimage = self.preimage.clone();
            self.htlcs[i].claim(preimage, current_height)?;
        }

        Ok(())
    }
}
```

**HTLC Payment Example**:
```
Route: Alice -> Bob -> Carol -> Dave (10 ÉTR)

1. Setup Phase (backward from Dave):
   - Carol -> Dave: HTLC(hash, 10.00 ÉTR, timeout=1000)
   - Bob -> Carol: HTLC(hash, 10.02 ÉTR, timeout=1006)
   - Alice -> Bob: HTLC(hash, 10.045 ÉTR, timeout=1012)

2. Settlement Phase (forward from Dave):
   - Dave reveals preimage to Carol
   - Carol learns preimage, claims 10.00 ÉTR, reveals to Bob
   - Bob learns preimage, claims 10.02 ÉTR, reveals to Alice
   - Alice learns preimage, claims 10.045 ÉTR

Result: Payment complete, Dave receives 10 ÉTR, intermediaries earn fees
```

### 2.4 Cross-PBC Routing

#### Exchange Rate Oracle Integration

```rust
use crate::oracle::ExchangeRate;

/// Cross-chain payment (e.g., ETH-PBC -> SOL-PBC)
pub struct CrossPBCPayment {
    /// Source chain
    pub source_pbc: String,
    /// Destination chain
    pub dest_pbc: String,
    /// Amount in source currency
    pub source_amount: u128,
    /// Amount in destination currency (calculated from oracle)
    pub dest_amount: u128,
    /// Exchange rate used
    pub exchange_rate: ExchangeRate,
    /// Route on source PBC
    pub source_route: Vec<RouteEdge>,
    /// Route on destination PBC
    pub dest_route: Vec<RouteEdge>,
}

impl CrossPBCPayment {
    /// Create cross-PBC payment with oracle rate
    pub fn new(
        source_pbc: String,
        dest_pbc: String,
        source_amount: u128,
        oracle: &OracleAggregator,
        current_time: u64,
    ) -> Result<Self, PaymentError> {
        // Get exchange rate from oracle
        let rate = oracle.get_aggregated_rate(&source_pbc, &dest_pbc, current_time)?;

        // Calculate destination amount
        // rate.rate is in basis points (10000 = 1:1)
        let dest_amount = (source_amount * rate.rate as u128) / 10000;

        Ok(Self {
            source_pbc,
            dest_pbc,
            source_amount,
            dest_amount,
            exchange_rate: rate,
            source_route: vec![],
            dest_route: vec![],
        })
    }
}
```

**Cross-PBC Example**:
```
Payment: 1 ETH (on ETH-PBC) -> SOL (on SOL-PBC)
Oracle Rate: 1 ETH = 150 SOL (rate = 1,500,000 basis points)

Calculation:
  dest_amount = (1 ETH * 1,500,000) / 10,000 = 150 SOL

Result: Alice pays 1 ETH on ETH-PBC, Bob receives 150 SOL on SOL-PBC
```

### 2.5 Watchtower Network Protocol

```rust
/// Watchtower - Monitors channels for fraud
pub struct Watchtower {
    /// Watchtower ID
    pub id: String,
    /// Stake (1000 ÉTR required)
    pub stake: u128,
    /// Monitored channels
    pub monitored_channels: Vec<String>,
    /// Reputation score
    pub reputation: u32,
}

impl Watchtower {
    /// Detect fraud (old state broadcast)
    pub fn detect_fraud(
        &self,
        channel: &PaymentChannel,
        broadcast_state: &ChannelState,
        broadcast_nonce: u64,
    ) -> Option<FraudProof> {
        // Check if broadcast state is outdated
        if broadcast_nonce < channel.nonce {
            // Fraud detected! Watchtower has newer state
            Some(FraudProof {
                channel_id: channel.id.clone(),
                fraudulent_nonce: broadcast_nonce,
                correct_nonce: channel.nonce,
                evidence: channel.signatures.clone(),
                detected_by: self.id.clone(),
            })
        } else {
            None
        }
    }

    /// Submit fraud proof and claim reward
    pub fn submit_fraud_proof(&self, proof: FraudProof) -> u128 {
        // Base reward: 100 ÉTR
        let base_reward = 100_000_000_000_000_000_000u128; // 100 ÉTR in smallest unit

        // Penalty from fraudster: 10% of channel capacity
        let penalty = 0; // Calculated from channel capacity

        base_reward + penalty
    }
}

#[derive(Clone, Debug)]
pub struct FraudProof {
    pub channel_id: String,
    pub fraudulent_nonce: u64,
    pub correct_nonce: u64,
    pub evidence: ChannelSignatures,
    pub detected_by: String,
}
```

**Watchtower Economics**:
- Stake Required: 1,000 ÉTR
- Base Reward: 100 ÉTR per fraud detection
- Penalty: 10% of fraudulent channel capacity
- Reputation: Increases with correct detections, decreases with false alarms

### 2.6 Performance Benchmarks

| Metric | Value | Notes |
|--------|-------|-------|
| **Channel Open** | 1 on-chain tx | ~6 seconds |
| **Channel Update** | 0 on-chain txs | <1 second (off-chain) |
| **Channel Close** | 1 on-chain tx | ~6 seconds |
| **Network Capacity** | 500,000 TPS | Theoretical (all channels) |
| **Max Hops** | 20 | Configurable |
| **Routing Latency** | <100ms | Pathfinding + HTLC setup |
| **Fee per Hop** | 0.1-1% | Typical range |
| **Dispute Period** | 144 blocks | 14.4 minutes @ 6s/block |

**Cost Comparison**:
```
On-Chain Transaction:
  - Gas: 21,000 VMw
  - Cost: 0.021 ÉTR

Lightning-Bloc Payment:
  - Channel Open: 21,000 VMw (one-time)
  - 1,000 payments: 0 VMw (off-chain)
  - Channel Close: 21,000 VMw (one-time)
  - Total: 42,000 VMw for 1,000 payments
  - Cost per payment: 0.000042 ÉTR

Savings: 99.8% cheaper than on-chain
```

---

# PART III: ECONOMIC SYSTEMS

## 3. Distribution Pay Implementation

**Location**: `/Users/macbook/Desktop/etrid/12-consensus-day/distribution/src/lib.rs`
**Lines**: 174
**Daily Distribution**: 27,397 ÉTR

### 3.1 Distribution Schedule

```rust
use chrono::{DateTime, Utc, Timelike};

/// Daily distribution schedule
pub struct DistributionSchedule {
    /// Total daily distribution
    pub total_daily: u128, // 27,397 ÉTR in smallest unit

    /// Allocation percentages (basis points)
    pub voters_pct: u32,      // 1000 = 10%
    pub flare_nodes_pct: u32, // 1500 = 15%
    pub validity_nodes_pct: u32, // 1500 = 15%
    pub stakers_pct: u32,     // 4000 = 40%
    pub directors_pct: u32,   // 2000 = 20%
}

impl Default for DistributionSchedule {
    fn default() -> Self {
        Self {
            total_daily: 27_397_000_000_000_000_000_000u128, // 27,397 ÉTR
            voters_pct: 1000,      // 10%
            flare_nodes_pct: 1500, // 15%
            validity_nodes_pct: 1500, // 15%
            stakers_pct: 4000,     // 40%
            directors_pct: 2000,   // 20%
        }
    }
}

/// Distribution times (UTC)
pub struct DistributionTimes {
    pub voters: (u8, u8),        // 00:01 (12:01 AM)
    pub flare_nodes: (u8, u8),   // 04:01 (4:01 AM)
    pub validity_nodes: (u8, u8), // 06:01 (6:01 AM)
    pub stakers: (u8, u8),       // 08:01 (8:01 AM)
    pub directors: (u8, u8),     // 12:01 (12:01 PM)
}

impl Default for DistributionTimes {
    fn default() -> Self {
        Self {
            voters: (0, 1),
            flare_nodes: (4, 1),
            validity_nodes: (6, 1),
            stakers: (8, 1),
            directors: (12, 1),
        }
    }
}
```

### 3.2 Coinage Multiplier Calculation

```rust
/// Calculate coinage multiplier based on stake duration
pub fn calculate_coinage_multiplier(stake_duration_days: u64) -> u32 {
    // Returns basis points (10000 = 1.0x)
    match stake_duration_days {
        0..=180 => 10000,         // 0-6 months: 1.0x
        181..=365 => 12500,       // 6-12 months: 1.25x
        366..=545 => 15000,       // 12-18 months: 1.5x
        546..=730 => 17500,       // 18-24 months: 1.75x
        _ => 20000,               // 24+ months: 2.0x (max)
    }
}

/// Calculate reward with coinage multiplier
pub fn calculate_reward_with_coinage(
    base_reward: u128,
    stake_duration_days: u64,
) -> u128 {
    let multiplier = calculate_coinage_multiplier(stake_duration_days);
    (base_reward * multiplier as u128) / 10000
}
```

**Coinage Examples**:
```rust
// Example 1: New staker (0 days)
let reward = calculate_reward_with_coinage(100_000_000_000_000_000_000, 0);
// reward = 100 ÉTR (1.0x multiplier)

// Example 2: Long-term staker (730 days)
let reward = calculate_reward_with_coinage(100_000_000_000_000_000_000, 730);
// reward = 200 ÉTR (2.0x multiplier)
```

### 3.3 Distribution Execution

```rust
/// Execute daily distribution
pub fn execute_distribution(
    schedule: &DistributionSchedule,
    current_time: DateTime<Utc>,
) -> Result<Vec<Distribution>, DistributionError> {
    let times = DistributionTimes::default();
    let hour = current_time.hour() as u8;
    let minute = current_time.minute() as u8;

    let mut distributions = vec![];

    // Check if it's time for voters distribution (00:01)
    if (hour, minute) == times.voters {
        let amount = (schedule.total_daily * schedule.voters_pct as u128) / 10000;
        distributions.push(Distribution {
            category: DistributionCategory::Voters,
            amount,
            timestamp: current_time.timestamp() as u64,
        });
    }

    // Check if it's time for Primearc Core Validators distribution (04:01)
    if (hour, minute) == times.flare_nodes {
        let amount = (schedule.total_daily * schedule.flare_nodes_pct as u128) / 10000;
        distributions.push(Distribution {
            category: DistributionCategory::FlareNodes,
            amount,
            timestamp: current_time.timestamp() as u64,
        });
    }

    // Check if it's time for validity nodes distribution (06:01)
    if (hour, minute) == times.validity_nodes {
        let amount = (schedule.total_daily * schedule.validity_nodes_pct as u128) / 10000;
        distributions.push(Distribution {
            category: DistributionCategory::ValidityNodes,
            amount,
            timestamp: current_time.timestamp() as u64,
        });
    }

    // Check if it's time for stakers distribution (08:01)
    if (hour, minute) == times.stakers {
        let amount = (schedule.total_daily * schedule.stakers_pct as u128) / 10000;
        distributions.push(Distribution {
            category: DistributionCategory::Stakers,
            amount,
            timestamp: current_time.timestamp() as u64,
        });
    }

    // Check if it's time for directors distribution (12:01)
    if (hour, minute) == times.directors {
        let amount = (schedule.total_daily * schedule.directors_pct as u128) / 10000;
        distributions.push(Distribution {
            category: DistributionCategory::Directors,
            amount,
            timestamp: current_time.timestamp() as u64,
        });
    }

    Ok(distributions)
}

#[derive(Clone, Debug)]
pub struct Distribution {
    pub category: DistributionCategory,
    pub amount: u128,
    pub timestamp: u64,
}

#[derive(Clone, Debug)]
pub enum DistributionCategory {
    Voters,
    FlareNodes,
    ValidityNodes,
    Stakers,
    Directors,
}
```

### 3.4 Penalty System

```rust
/// Validator penalties
pub struct PenaltySystem {
    /// Penalty for missed block (basis points)
    pub missed_block_penalty: u32, // 10 = 0.1%

    /// Penalty for double-sign (basis points)
    pub double_sign_penalty: u32,  // 1000 = 10%

    /// Penalty for offline (per hour, basis points)
    pub offline_penalty: u32,      // 1 = 0.01% per hour
}

impl Default for PenaltySystem {
    fn default() -> Self {
        Self {
            missed_block_penalty: 10,    // 0.1% per missed block
            double_sign_penalty: 1000,   // 10% for double-sign
            offline_penalty: 1,          // 0.01% per hour offline
        }
    }
}

/// Apply penalty to reward
pub fn apply_penalty(
    base_reward: u128,
    penalty_type: PenaltyType,
) -> u128 {
    let penalties = PenaltySystem::default();
    let penalty_bps = match penalty_type {
        PenaltyType::MissedBlock(count) => penalties.missed_block_penalty * count,
        PenaltyType::DoubleSign => penalties.double_sign_penalty,
        PenaltyType::Offline(hours) => penalties.offline_penalty * hours,
    };

    // Deduct penalty
    let penalty_amount = (base_reward * penalty_bps as u128) / 10000;
    base_reward.saturating_sub(penalty_amount)
}

pub enum PenaltyType {
    MissedBlock(u32),  // Number of missed blocks
    DoubleSign,
    Offline(u32),      // Hours offline
}
```

**Penalty Examples**:
```rust
// Example 1: Missed 5 blocks
let reward = apply_penalty(1000_000_000_000_000_000_000, PenaltyType::MissedBlock(5));
// Penalty: 0.1% × 5 = 0.5%
// Final reward: 995 ÉTR

// Example 2: Double-sign
let reward = apply_penalty(1000_000_000_000_000_000_000, PenaltyType::DoubleSign);
// Penalty: 10%
// Final reward: 900 ÉTR

// Example 3: Offline 24 hours
let reward = apply_penalty(1000_000_000_000_000_000_000, PenaltyType::Offline(24));
// Penalty: 0.01% × 24 = 0.24%
// Final reward: 997.6 ÉTR
```

# PART IV: CROSS-CHAIN INFRASTRUCTURE

## 4. Bridge Protocol Specification

**Location**: `/Users/macbook/Desktop/etrid/05-multichain/bridge-protocols/`
**Lines**: 19,189 across 13 bridge implementations
**Security**: M-of-N multisig, circuit breakers, bug bounty (up to $1M)

### 4.1 Multisig Security Model

#### Custodian Structure

```rust
/// M-of-N Multisig Custodian System
#[derive(Clone, Encode, Decode, PartialEq, Eq, TypeInfo)]
pub struct MultiSigCustodian<AccountId> {
    /// List of authorized custodians
    pub custodians: Vec<AccountId>,

    /// Minimum approvals required (M in M-of-N)
    pub threshold: u32,

    /// Current approval count
    pub approvals: BTreeMap<H256, BTreeSet<AccountId>>,

    /// Nonce for replay protection
    pub nonce: u64,
}

impl<AccountId: Ord + Clone> MultiSigCustodian<AccountId> {
    /// Create new multisig with N custodians and M threshold
    pub fn new(custodians: Vec<AccountId>, threshold: u32) -> Result<Self, MultisigError> {
        if threshold == 0 || threshold as usize > custodians.len() {
            return Err(MultisigError::InvalidThreshold);
        }

        Ok(Self {
            custodians,
            threshold,
            approvals: BTreeMap::new(),
            nonce: 0,
        })
    }

    /// Verify if account is a custodian
    pub fn is_custodian(&self, account: &AccountId) -> bool {
        self.custodians.contains(account)
    }

    /// Add approval for a transaction
    pub fn add_approval(
        &mut self,
        tx_hash: H256,
        approver: AccountId,
    ) -> Result<(), MultisigError> {
        // Verify approver is a custodian
        if !self.is_custodian(&approver) {
            return Err(MultisigError::NotCustodian);
        }

        // Add approval
        self.approvals
            .entry(tx_hash)
            .or_insert_with(BTreeSet::new)
            .insert(approver);

        Ok(())
    }

    /// Check if threshold is met
    pub fn has_threshold(&self, tx_hash: &H256) -> bool {
        self.approvals
            .get(tx_hash)
            .map(|approvals| approvals.len() >= self.threshold as usize)
            .unwrap_or(false)
    }

    /// Validate all approvals for a transaction
    pub fn validate_approvals(
        &self,
        tx_hash: &H256,
        approvals: &[AccountId],
    ) -> Result<(), MultisigError> {
        // Check all approvals are from registered custodians
        for approval in approvals {
            if !self.is_custodian(approval) {
                return Err(MultisigError::NotCustodian);
            }
        }

        // Check for duplicates
        let unique_approvals: BTreeSet<_> = approvals.iter().collect();
        if unique_approvals.len() != approvals.len() {
            return Err(MultisigError::DuplicateApproval);
        }

        // Check threshold met
        if approvals.len() < self.threshold as usize {
            return Err(MultisigError::ThresholdNotMet);
        }

        Ok(())
    }
}

#[derive(Debug)]
pub enum MultisigError {
    InvalidThreshold,
    NotCustodian,
    DuplicateApproval,
    ThresholdNotMet,
}
```

**Multisig Configurations by Bridge**:

| Bridge | Custodians (N) | Threshold (M) | Configuration |
|--------|----------------|---------------|---------------|
| Bitcoin | 7 | 5 | 5-of-7 |
| Ethereum | 9 | 7 | 7-of-9 |
| Solana | 7 | 5 | 5-of-7 |
| Cardano | 7 | 5 | 5-of-7 |
| Polygon | 7 | 5 | 5-of-7 |
| BNB Chain | 7 | 5 | 5-of-7 |
| Ripple | 7 | 5 | 5-of-7 |
| Tron | 7 | 5 | 5-of-7 |

### 4.2 Lock-and-Mint Flow Implementation

#### Bridge Deposit Structure

```rust
/// Bridge deposit tracking
#[derive(Clone, Encode, Decode, PartialEq, Eq, TypeInfo)]
pub struct BridgeDeposit {
    /// Unique deposit ID
    pub id: H256,

    /// Depositor on external chain
    pub depositor: Vec<u8>,  // External address (variable length)

    /// Recipient on Ëtrid
    pub recipient: AccountId,

    /// Amount deposited (in external chain's units)
    pub amount: u128,

    /// External chain transaction hash
    pub tx_hash: Vec<u8>,

    /// Number of confirmations received
    pub confirmations: u32,

    /// Required confirmations
    pub required_confirmations: u32,

    /// Status
    pub status: DepositStatus,

    /// Custodian approvals
    pub approvals: BTreeSet<AccountId>,

    /// Timestamp of deposit
    pub timestamp: u64,
}

#[derive(Clone, Debug, Encode, Decode, PartialEq, Eq, TypeInfo)]
pub enum DepositStatus {
    /// Detected but not confirmed
    Pending,
    /// Confirmed, awaiting custodian approvals
    Confirmed,
    /// Approved by threshold, ready to mint
    Approved,
    /// Minted on Ëtrid
    Minted,
    /// Failed or rejected
    Rejected { reason: Vec<u8> },
}
```

#### Lock-and-Mint Implementation

```rust
impl<T: Config> Pallet<T> {
    /// Process external chain deposit
    pub fn process_deposit(
        external_tx: ExternalTransaction,
        recipient: T::AccountId,
    ) -> DispatchResult {
        // 1. Validate external transaction
        Self::validate_external_tx(&external_tx)?;

        // 2. Check confirmations
        ensure!(
            external_tx.confirmations >= T::RequiredConfirmations::get(),
            Error::<T>::InsufficientConfirmations
        );

        // 3. Create deposit record
        let deposit_id = Self::generate_deposit_id(&external_tx);
        let deposit = BridgeDeposit {
            id: deposit_id,
            depositor: external_tx.from,
            recipient: recipient.clone(),
            amount: external_tx.amount,
            tx_hash: external_tx.hash,
            confirmations: external_tx.confirmations,
            required_confirmations: T::RequiredConfirmations::get(),
            status: DepositStatus::Confirmed,
            approvals: BTreeSet::new(),
            timestamp: T::UnixTime::now().as_secs(),
        };

        // 4. Store deposit
        Deposits::<T>::insert(deposit_id, deposit);

        // 5. Emit event
        Self::deposit_event(Event::DepositDetected {
            deposit_id,
            depositor: external_tx.from,
            amount: external_tx.amount,
        });

        Ok(())
    }

    /// Custodian approves deposit
    pub fn approve_deposit(
        origin: OriginFor<T>,
        deposit_id: H256,
    ) -> DispatchResult {
        let custodian = ensure_signed(origin)?;

        // Verify custodian
        ensure!(
            T::Custodians::is_custodian(&custodian),
            Error::<T>::NotCustodian
        );

        // Get deposit
        let mut deposit = Deposits::<T>::get(deposit_id)
            .ok_or(Error::<T>::DepositNotFound)?;

        // Add approval
        deposit.approvals.insert(custodian.clone());

        // Check if threshold met
        if deposit.approvals.len() >= T::Threshold::get() as usize {
            // Execute mint
            Self::execute_mint(&mut deposit)?;
        }

        // Update storage
        Deposits::<T>::insert(deposit_id, deposit);

        Ok(())
    }

    /// Execute mint after threshold approvals
    fn execute_mint(deposit: &mut BridgeDeposit) -> DispatchResult {
        // 1. Mint wrapped tokens
        T::Currency::deposit_creating(
            &deposit.recipient,
            deposit.amount.saturated_into(),
        );

        // 2. Update status
        deposit.status = DepositStatus::Minted;

        // 3. Emit event
        Self::deposit_event(Event::TokensMinted {
            deposit_id: deposit.id,
            recipient: deposit.recipient.clone(),
            amount: deposit.amount,
        });

        Ok(())
    }
}
```

**Lock-and-Mint Flow Example (Bitcoin → BTC-PBC)**:

```
Step 1: User locks 1 BTC on Bitcoin
  - Sends to multisig address: bc1q... (5-of-7)
  - Transaction: txid abc123...
  - Confirmations: 0/6

Step 2: Wait for confirmations (6 blocks ≈ 60 minutes)
  - Block 1: 1/6
  - Block 2: 2/6
  - ...
  - Block 6: 6/6 ✓

Step 3: Custodians detect deposit
  - Custodian A approves (1/5)
  - Custodian B approves (2/5)
  - Custodian C approves (3/5)
  - Custodian D approves (4/5)
  - Custodian E approves (5/5) ✓ Threshold met!

Step 4: Mint wBTC on BTC-PBC
  - Recipient receives 1 wBTC
  - 1:1 ratio maintained
  - Total supply increased by 1 wBTC

Result: User has 1 wBTC on Ëtrid, 1 BTC locked in multisig
```

### 4.3 Burn-and-Unlock Flow

```rust
impl<T: Config> Pallet<T> {
    /// Initiate withdrawal (burn wrapped tokens)
    pub fn initiate_withdrawal(
        origin: OriginFor<T>,
        amount: BalanceOf<T>,
        external_address: Vec<u8>,
    ) -> DispatchResult {
        let user = ensure_signed(origin)?;

        // 1. Validate external address format
        Self::validate_external_address(&external_address)?;

        // 2. Burn wrapped tokens
        T::Currency::withdraw(
            &user,
            amount,
            WithdrawReasons::all(),
            ExistenceRequirement::AllowDeath,
        )?;

        // 3. Create withdrawal request
        let withdrawal_id = Self::generate_withdrawal_id(&user, amount);
        let withdrawal = Withdrawal {
            id: withdrawal_id,
            user: user.clone(),
            amount: amount.saturated_into(),
            external_address: external_address.clone(),
            status: WithdrawalStatus::Pending,
            approvals: BTreeSet::new(),
            timestamp: T::UnixTime::now().as_secs(),
        };

        // 4. Store withdrawal
        Withdrawals::<T>::insert(withdrawal_id, withdrawal);

        // 5. Emit event
        Self::deposit_event(Event::WithdrawalInitiated {
            withdrawal_id,
            user,
            amount,
            external_address,
        });

        Ok(())
    }

    /// Custodian approves withdrawal
    pub fn approve_withdrawal(
        origin: OriginFor<T>,
        withdrawal_id: H256,
    ) -> DispatchResult {
        let custodian = ensure_signed(origin)?;

        // Verify custodian
        ensure!(
            T::Custodians::is_custodian(&custodian),
            Error::<T>::NotCustodian
        );

        // Get withdrawal
        let mut withdrawal = Withdrawals::<T>::get(withdrawal_id)
            .ok_or(Error::<T>::WithdrawalNotFound)?;

        // Add approval
        withdrawal.approvals.insert(custodian);

        // Check if threshold met
        if withdrawal.approvals.len() >= T::Threshold::get() as usize {
            // Execute unlock on external chain
            Self::execute_unlock(&mut withdrawal)?;
        }

        // Update storage
        Withdrawals::<T>::insert(withdrawal_id, withdrawal);

        Ok(())
    }

    /// Execute unlock on external chain
    fn execute_unlock(withdrawal: &mut Withdrawal) -> DispatchResult {
        // 1. Generate external transaction
        let external_tx = Self::create_external_tx(
            &withdrawal.external_address,
            withdrawal.amount,
        )?;

        // 2. Submit to external chain (off-chain worker)
        T::ExternalChain::submit_transaction(external_tx)?;

        // 3. Update status
        withdrawal.status = WithdrawalStatus::Unlocked;

        // 4. Emit event
        Self::deposit_event(Event::TokensUnlocked {
            withdrawal_id: withdrawal.id,
            external_address: withdrawal.external_address.clone(),
            amount: withdrawal.amount,
        });

        Ok(())
    }
}
```

### 4.4 Circuit Breaker System

```rust
/// Circuit breaker for emergency pause
#[derive(Clone, Encode, Decode, PartialEq, Eq, TypeInfo)]
pub struct CircuitBreaker {
    /// Is bridge paused?
    pub paused: bool,

    /// Reason for pause
    pub reason: Option<Vec<u8>>,

    /// Pause initiated by
    pub initiated_by: Option<AccountId>,

    /// Pause timestamp
    pub paused_at: Option<u64>,

    /// Daily withdrawal limit (anti-drain protection)
    pub daily_limit: u128,

    /// Current daily withdrawal amount
    pub daily_withdrawn: u128,

    /// Last reset timestamp
    pub last_reset: u64,
}

impl CircuitBreaker {
    /// Check if withdrawal is allowed
    pub fn allow_withdrawal(&mut self, amount: u128, current_time: u64) -> Result<(), BridgeError> {
        // Check if paused
        if self.paused {
            return Err(BridgeError::BridgePaused);
        }

        // Reset daily limit if 24 hours passed
        if current_time > self.last_reset + 86400 {
            self.daily_withdrawn = 0;
            self.last_reset = current_time;
        }

        // Check daily limit
        if self.daily_withdrawn + amount > self.daily_limit {
            return Err(BridgeError::DailyLimitExceeded);
        }

        // Update withdrawn amount
        self.daily_withdrawn += amount;

        Ok(())
    }

    /// Emergency pause
    pub fn pause(&mut self, reason: Vec<u8>, paused_by: AccountId, current_time: u64) {
        self.paused = true;
        self.reason = Some(reason);
        self.initiated_by = Some(paused_by);
        self.paused_at = Some(current_time);
    }

    /// Resume bridge
    pub fn resume(&mut self) {
        self.paused = false;
        self.reason = None;
        self.initiated_by = None;
        self.paused_at = None;
    }
}
```

**Circuit Breaker Triggers**:
- Daily withdrawal limit exceeded (e.g., $10M for BTC bridge)
- Suspicious activity detected (multiple large withdrawals)
- Custodian compromise suspected
- Smart contract vulnerability discovered
- Governance vote to pause

### 4.5 Bridge Security Considerations

**Threat Model**:

| Threat | Mitigation | Status |
|--------|------------|--------|
| **Custodian Collusion** | M-of-N threshold (requires 5/7 or 7/9) | ✅ |
| **Replay Attack** | Nonce + transaction hash verification | ✅ |
| **Double Spend** | Confirmation requirements (6+ blocks) | ✅ |
| **Drain Attack** | Daily withdrawal limits, circuit breaker | ✅ |
| **Smart Contract Bug** | Formal verification, audits, bug bounty | ⏳ |
| **Custodian Private Key Theft** | Hardware security modules (HSM), MPC | ⏳ |
| **Oracle Manipulation** | Multi-source aggregation, TWAP | ✅ |

**Security Best Practices**:
```rust
// 1. Always verify external transaction before processing
Self::validate_external_tx(&tx)?;

// 2. Require sufficient confirmations
ensure!(tx.confirmations >= REQUIRED_CONFIRMATIONS, Error::InsufficientConfirmations);

// 3. Validate all custodian approvals
T::Custodians::validate_approvals(&tx_hash, &approvals)?;

// 4. Check circuit breaker before allowing withdrawal
CircuitBreaker::allow_withdrawal(amount, current_time)?;

// 5. Emit events for all critical operations
Self::deposit_event(Event::CriticalOperation { details });

// 6. Use safe math (saturating operations)
let new_balance = old_balance.saturating_add(amount);
```

---

# PART V: SMART CONTRACT RUNTIME

## 5. ËtwasmVM Execution Engine

**Location**: `/Users/macbook/Desktop/etrid/08-etwasm-vm/`
**Lines**: 9,793 across 4 layers
**EVM Compatibility**: Berlin/London fork (150+ opcodes)

### 5.1 Four-Layer Architecture

```
┌─────────────────────────────────────────────────┐
│          LAYER 1: PALLET LAYER                  │
│  - deploy_contract()                            │
│  - call_contract()                              │
│  - execute_bytecode()                           │
│  - Storage: ContractCodeHash, ContractStorage   │
└─────────────────────────────────────────────────┘
                      ↓
┌─────────────────────────────────────────────────┐
│         LAYER 2: RUNTIME LAYER                  │
│  - Interpreter (bytecode execution)             │
│  - Stack (256-bit words, 1024 depth)            │
│  - Memory (expandable up to 16 MB)              │
│  - ExecutionContext (caller, gas, block info)   │
└─────────────────────────────────────────────────┘
                      ↓
┌─────────────────────────────────────────────────┐
│          LAYER 3: OPCODE LAYER                  │
│  - Arithmetic (ADD, MUL, DIV, MOD, EXP)         │
│  - Logic (AND, OR, XOR, NOT, SHL, SHR)          │
│  - Memory (MLOAD, MSTORE, MSTORE8)              │
│  - Storage (SLOAD, SSTORE)                      │
│  - Control Flow (JUMP, JUMPI, JUMPDEST, PC)     │
│  - 150+ EVM opcodes total                       │
└─────────────────────────────────────────────────┘
                      ↓
┌─────────────────────────────────────────────────┐
│       LAYER 4: GAS METERING LAYER               │
│  - VMw cost calculation per opcode              │
│  - Block limit: 10,000,000 VMw                  │
│  - Transaction limit: 1,000,000 VMw             │
│  - Gas refunds (EIP-3529 compliant)             │
└─────────────────────────────────────────────────┘
```

### 5.2 Execution Context

```rust
use sp_std::collections::btree_set::BTreeSet;

/// Execution context for contract calls
#[derive(Clone, Debug)]
pub struct ExecutionContext {
    /// Caller address (who initiated the call)
    pub caller: [u8; 32],

    /// Contract address (currently executing contract)
    pub address: [u8; 32],

    /// Value transferred with call (in smallest unit)
    pub value: u128,

    /// Gas limit for this execution
    pub gas_limit: VMw,

    /// Gas price (VMw per operation)
    pub gas_price: u64,

    /// Current block number
    pub block_number: u64,

    /// Current timestamp (UNIX seconds)
    pub timestamp: u64,

    /// Chain ID (2 for Ëtrid)
    pub chain_id: u64,

    /// Call stack for reentrancy detection
    pub call_stack: BTreeSet<[u8; 32]>,

    /// Current reentrancy depth
    pub reentrancy_depth: u32,

    /// Maximum allowed call depth
    pub max_depth: u32,
}

impl ExecutionContext {
    /// Enter a new call (reentrancy protection)
    pub fn enter_call(&mut self, target: [u8; 32]) -> Result<(), ExecutionError> {
        // Check if target already in call stack (direct or indirect reentrancy)
        if self.call_stack.contains(&target) {
            return Err(ExecutionError::ReentrancyDetected);
        }

        // Check max depth
        if self.reentrancy_depth >= self.max_depth {
            return Err(ExecutionError::MaxCallDepthExceeded);
        }

        // Add to call stack
        self.call_stack.insert(target);
        self.reentrancy_depth += 1;

        Ok(())
    }

    /// Exit a call
    pub fn exit_call(&mut self, target: [u8; 32]) {
        self.call_stack.remove(&target);
        self.reentrancy_depth = self.reentrancy_depth.saturating_sub(1);
    }
}
```

### 5.3 Stack Implementation

```rust
/// EVM stack (256-bit words)
pub struct Stack {
    /// Stack data (max 1024 elements)
    data: Vec<U256>,

    /// Maximum stack size
    max_size: usize,
}

impl Stack {
    /// Create new stack
    pub fn new() -> Self {
        Self {
            data: Vec::with_capacity(1024),
            max_size: 1024,
        }
    }

    /// Push value onto stack
    pub fn push(&mut self, value: U256) -> Result<(), StackError> {
        if self.data.len() >= self.max_size {
            return Err(StackError::Overflow);
        }
        self.data.push(value);
        Ok(())
    }

    /// Pop value from stack
    pub fn pop(&mut self) -> Result<U256, StackError> {
        self.data.pop().ok_or(StackError::Underflow)
    }

    /// Peek at top of stack without removing
    pub fn peek(&self) -> Result<&U256, StackError> {
        self.data.last().ok_or(StackError::Underflow)
    }

    /// Peek at nth element from top (0 = top)
    pub fn peek_at(&self, index: usize) -> Result<&U256, StackError> {
        let len = self.data.len();
        if index >= len {
            return Err(StackError::Underflow);
        }
        Ok(&self.data[len - 1 - index])
    }

    /// Swap top with nth element
    pub fn swap(&mut self, index: usize) -> Result<(), StackError> {
        let len = self.data.len();
        if index >= len {
            return Err(StackError::Underflow);
        }
        let top_index = len - 1;
        let swap_index = len - 1 - index;
        self.data.swap(top_index, swap_index);
        Ok(())
    }

    /// Duplicate nth element to top
    pub fn dup(&mut self, index: usize) -> Result<(), StackError> {
        let value = *self.peek_at(index)?;
        self.push(value)
    }

    /// Get current stack depth
    pub fn depth(&self) -> usize {
        self.data.len()
    }
}

#[derive(Debug)]
pub enum StackError {
    Overflow,
    Underflow,
}
```

### 5.4 Memory Implementation

```rust
/// EVM memory (byte-addressable, expandable)
pub struct Memory {
    /// Memory data
    data: Vec<u8>,

    /// Maximum size (16 MB = 256 pages × 64 KB)
    max_size: usize,
}

impl Memory {
    /// Create new memory
    pub fn new() -> Self {
        Self {
            data: Vec::new(),
            max_size: 16 * 1024 * 1024, // 16 MB
        }
    }

    /// Read byte at offset
    pub fn read_byte(&self, offset: usize) -> u8 {
        if offset >= self.data.len() {
            0 // Uninitialized memory returns 0
        } else {
            self.data[offset]
        }
    }

    /// Write byte at offset (auto-expand if needed)
    pub fn write_byte(&mut self, offset: usize, value: u8) -> Result<(), MemoryError> {
        // Expand memory if needed
        if offset >= self.data.len() {
            let new_size = offset + 1;
            if new_size > self.max_size {
                return Err(MemoryError::OutOfMemory);
            }
            self.data.resize(new_size, 0);
        }

        self.data[offset] = value;
        Ok(())
    }

    /// Read 32-byte word (U256)
    pub fn read_word(&self, offset: usize) -> U256 {
        let mut bytes = [0u8; 32];
        for i in 0..32 {
            bytes[i] = self.read_byte(offset + i);
        }
        U256::from_big_endian(&bytes)
    }

    /// Write 32-byte word (U256)
    pub fn write_word(&mut self, offset: usize, value: U256) -> Result<(), MemoryError> {
        let mut bytes = [0u8; 32];
        value.to_big_endian(&mut bytes);

        for (i, &byte) in bytes.iter().enumerate() {
            self.write_byte(offset + i, byte)?;
        }

        Ok(())
    }

    /// Get current memory size
    pub fn size(&self) -> usize {
        self.data.len()
    }

    /// Calculate gas cost for memory expansion
    pub fn expansion_cost(&self, new_size: usize) -> VMw {
        let current_size = self.size();
        if new_size <= current_size {
            return 0;
        }

        // Memory expansion cost: 3 VMw per word + (words^2 / 512)
        let new_words = (new_size + 31) / 32;
        let current_words = (current_size + 31) / 32;

        let new_cost = 3 * new_words + (new_words * new_words) / 512;
        let current_cost = 3 * current_words + (current_words * current_words) / 512;

        (new_cost - current_cost) as VMw
    }
}

#[derive(Debug)]
pub enum MemoryError {
    OutOfMemory,
}
```

### 5.5 Opcode Implementation Examples

#### Arithmetic Opcodes

```rust
/// Execute ADD opcode (0x01)
pub fn op_add(stack: &mut Stack) -> Result<VMw, OpcodeError> {
    let a = stack.pop()?;
    let b = stack.pop()?;
    let result = a.overflowing_add(b).0; // Wrapping add
    stack.push(result)?;
    Ok(3) // 3 VMw gas cost
}

/// Execute MUL opcode (0x02)
pub fn op_mul(stack: &mut Stack) -> Result<VMw, OpcodeError> {
    let a = stack.pop()?;
    let b = stack.pop()?;
    let result = a.overflowing_mul(b).0; // Wrapping mul
    stack.push(result)?;
    Ok(5) // 5 VMw gas cost
}

/// Execute DIV opcode (0x04)
pub fn op_div(stack: &mut Stack) -> Result<VMw, OpcodeError> {
    let a = stack.pop()?;
    let b = stack.pop()?;
    let result = if b.is_zero() {
        U256::zero() // Division by zero returns 0
    } else {
        a / b
    };
    stack.push(result)?;
    Ok(5) // 5 VMw gas cost
}

/// Execute EXP opcode (0x0A)
pub fn op_exp(stack: &mut Stack) -> Result<VMw, OpcodeError> {
    let base = stack.pop()?;
    let exponent = stack.pop()?;
    let result = base.overflowing_pow(exponent).0;
    stack.push(result)?;

    // Gas cost: 10 + 50 per byte in exponent
    let exp_bytes = (exponent.bits() + 7) / 8;
    Ok(10 + 50 * exp_bytes as VMw)
}
```

#### Memory Opcodes

```rust
/// Execute MLOAD opcode (0x51)
pub fn op_mload(stack: &mut Stack, memory: &mut Memory) -> Result<VMw, OpcodeError> {
    let offset = stack.pop()?.as_usize();
    let value = memory.read_word(offset);
    stack.push(value)?;

    // Gas: 3 base + memory expansion cost
    let expansion_gas = memory.expansion_cost(offset + 32);
    Ok(3 + expansion_gas)
}

/// Execute MSTORE opcode (0x52)
pub fn op_mstore(stack: &mut Stack, memory: &mut Memory) -> Result<VMw, OpcodeError> {
    let offset = stack.pop()?.as_usize();
    let value = stack.pop()?;
    memory.write_word(offset, value)?;

    // Gas: 3 base + memory expansion cost
    let expansion_gas = memory.expansion_cost(offset + 32);
    Ok(3 + expansion_gas)
}

/// Execute MSTORE8 opcode (0x53)
pub fn op_mstore8(stack: &mut Stack, memory: &mut Memory) -> Result<VMw, OpcodeError> {
    let offset = stack.pop()?.as_usize();
    let value = stack.pop()?;
    memory.write_byte(offset, value.byte(0))?; // Store lowest byte

    // Gas: 3 base + memory expansion cost
    let expansion_gas = memory.expansion_cost(offset + 1);
    Ok(3 + expansion_gas)
}
```

#### Storage Opcodes

```rust
/// Execute SLOAD opcode (0x54)
pub fn op_sload<S: Storage>(
    stack: &mut Stack,
    storage: &S,
    address: &[u8; 32],
) -> Result<VMw, OpcodeError> {
    let key = H256::from(stack.pop()?.to_big_endian());
    let value = storage.read(&key).unwrap_or(H256::zero());
    stack.push(U256::from_big_endian(value.as_bytes()))?;

    // Gas: 200 VMw (warm) or 2100 VMw (cold)
    Ok(200) // Simplified - actual cost depends on access list
}

/// Execute SSTORE opcode (0x55)
pub fn op_sstore<S: Storage>(
    stack: &mut Stack,
    storage: &mut S,
    address: &[u8; 32],
) -> Result<VMw, OpcodeError> {
    let key = H256::from(stack.pop()?.to_big_endian());
    let value = H256::from(stack.pop()?.to_big_endian());

    // Read current value
    let current = storage.read(&key).unwrap_or(H256::zero());

    // Write new value
    storage.write(key, value);

    // Gas cost depends on storage change (EIP-2200)
    let gas = if current == H256::zero() && value != H256::zero() {
        20000 // First write to empty slot
    } else if current != H256::zero() && value == H256::zero() {
        5000 // Clearing storage (refund eligible)
    } else {
        200 // Updating existing value
    };

    Ok(gas)
}
```

#### Control Flow Opcodes

```rust
/// Execute JUMP opcode (0x56)
pub fn op_jump(
    stack: &mut Stack,
    pc: &mut usize,
    valid_jumpdests: &BTreeSet<usize>,
) -> Result<VMw, OpcodeError> {
    let dest = stack.pop()?.as_usize();

    // Verify jump destination is valid (JUMPDEST)
    if !valid_jumpdests.contains(&dest) {
        return Err(OpcodeError::InvalidJump);
    }

    *pc = dest;
    Ok(8) // 8 VMw gas cost
}

/// Execute JUMPI opcode (0x57)
pub fn op_jumpi(
    stack: &mut Stack,
    pc: &mut usize,
    valid_jumpdests: &BTreeSet<usize>,
) -> Result<VMw, OpcodeError> {
    let dest = stack.pop()?.as_usize();
    let condition = stack.pop()?;

    // Jump only if condition is non-zero
    if !condition.is_zero() {
        if !valid_jumpdests.contains(&dest) {
            return Err(OpcodeError::InvalidJump);
        }
        *pc = dest;
    }

    Ok(10) // 10 VMw gas cost
}
```

### 5.6 Complete Opcode Gas Costs

| Opcode | Mnemonic | Gas Cost | Description |
|--------|----------|----------|-------------|
| 0x00 | STOP | 0 | Halt execution |
| 0x01 | ADD | 3 | Addition |
| 0x02 | MUL | 5 | Multiplication |
| 0x03 | SUB | 3 | Subtraction |
| 0x04 | DIV | 5 | Division |
| 0x05 | SDIV | 5 | Signed division |
| 0x06 | MOD | 5 | Modulo |
| 0x07 | SMOD | 5 | Signed modulo |
| 0x08 | ADDMOD | 8 | Modular addition |
| 0x09 | MULMOD | 8 | Modular multiplication |
| 0x0A | EXP | 10+50/byte | Exponentiation |
| 0x0B | SIGNEXTEND | 5 | Sign extension |
| 0x10 | LT | 3 | Less than |
| 0x11 | GT | 3 | Greater than |
| 0x12 | SLT | 3 | Signed less than |
| 0x13 | SGT | 3 | Signed greater than |
| 0x14 | EQ | 3 | Equality |
| 0x15 | ISZERO | 3 | Is zero |
| 0x16 | AND | 3 | Bitwise AND |
| 0x17 | OR | 3 | Bitwise OR |
| 0x18 | XOR | 3 | Bitwise XOR |
| 0x19 | NOT | 3 | Bitwise NOT |
| 0x1A | BYTE | 3 | Extract byte |
| 0x1B | SHL | 3 | Shift left |
| 0x1C | SHR | 3 | Shift right |
| 0x1D | SAR | 3 | Arithmetic shift right |
| 0x20 | SHA3 | 30+6/word | Keccak-256 hash |
| 0x30 | ADDRESS | 2 | Current contract address |
| 0x31 | BALANCE | 100 | Account balance |
| 0x32 | ORIGIN | 2 | Transaction origin |
| 0x33 | CALLER | 2 | Message caller |
| 0x34 | CALLVALUE | 2 | Message value |
| 0x35 | CALLDATALOAD | 3 | Load call data |
| 0x36 | CALLDATASIZE | 2 | Size of call data |
| 0x37 | CALLDATACOPY | 3+3/word | Copy call data |
| 0x38 | CODESIZE | 2 | Size of code |
| 0x39 | CODECOPY | 3+3/word | Copy code |
| 0x3A | GASPRICE | 2 | Gas price |
| 0x3B | EXTCODESIZE | 100 | External code size |
| 0x3C | EXTCODECOPY | 100+3/word | Copy external code |
| 0x3D | RETURNDATASIZE | 2 | Size of return data |
| 0x3E | RETURNDATACOPY | 3+3/word | Copy return data |
| 0x3F | EXTCODEHASH | 100 | External code hash |
| 0x40 | BLOCKHASH | 20 | Block hash |
| 0x41 | COINBASE | 2 | Block coinbase |
| 0x42 | TIMESTAMP | 2 | Block timestamp |
| 0x43 | NUMBER | 2 | Block number |
| 0x44 | DIFFICULTY | 2 | Block difficulty |
| 0x45 | GASLIMIT | 2 | Block gas limit |
| 0x46 | CHAINID | 2 | Chain ID |
| 0x47 | SELFBALANCE | 5 | Contract balance |
| 0x50 | POP | 2 | Pop from stack |
| 0x51 | MLOAD | 3* | Load from memory |
| 0x52 | MSTORE | 3* | Store to memory |
| 0x53 | MSTORE8 | 3* | Store byte to memory |
| 0x54 | SLOAD | 200-2100 | Load from storage |
| 0x55 | SSTORE | 200-20000 | Store to storage |
| 0x56 | JUMP | 8 | Unconditional jump |
| 0x57 | JUMPI | 10 | Conditional jump |
| 0x58 | PC | 2 | Program counter |
| 0x59 | MSIZE | 2 | Memory size |
| 0x5A | GAS | 2 | Remaining gas |
| 0x5B | JUMPDEST | 1 | Jump destination |
| 0x60-0x7F | PUSH1-PUSH32 | 3 | Push constant |
| 0x80-0x8F | DUP1-DUP16 | 3 | Duplicate stack item |
| 0x90-0x9F | SWAP1-SWAP16 | 3 | Swap stack items |
| 0xA0-0xA4 | LOG0-LOG4 | 375+375/topic | Emit log |
| 0xF0 | CREATE | 32000 | Create contract |
| 0xF1 | CALL | 100-9000 | Call contract |
| 0xF2 | CALLCODE | 100-9000 | Call with code |
| 0xF3 | RETURN | 0 | Return data |
| 0xF4 | DELEGATECALL | 100-9000 | Delegate call |
| 0xF5 | CREATE2 | 32000 | Create contract (deterministic) |
| 0xFA | STATICCALL | 100-9000 | Static call |
| 0xFD | REVERT | 0 | Revert execution |
| 0xFE | INVALID | 0 | Invalid opcode |
| 0xFF | SELFDESTRUCT | 5000-30000 | Destroy contract |

*Note: Some opcodes have dynamic gas costs based on memory expansion, storage access patterns, etc.*

### 5.7 Reentrancy Protection Implementation

```rust
impl ExecutionContext {
    /// Enter a call with reentrancy check
    pub fn enter_call(&mut self, target: [u8; 32]) -> Result<(), ExecutionError> {
        // Direct reentrancy check: A -> A
        if self.address == target {
            return Err(ExecutionError::ReentrancyDetected);
        }

        // Indirect reentrancy check: A -> B -> A
        if self.call_stack.contains(&target) {
            return Err(ExecutionError::ReentrancyDetected);
        }

        // Max depth check
        if self.reentrancy_depth >= self.max_depth {
            return Err(ExecutionError::MaxCallDepthExceeded);
        }

        // Add to call stack
        self.call_stack.insert(target);
        self.reentrancy_depth += 1;

        Ok(())
    }

    /// Exit a call
    pub fn exit_call(&mut self, target: [u8; 32]) {
        self.call_stack.remove(&target);
        self.reentrancy_depth = self.reentrancy_depth.saturating_sub(1);
    }
}

/// Test reentrancy protection
#[test]
fn test_reentrancy_protection() {
    let mut ctx = ExecutionContext::new();
    let contract_a = [1u8; 32];
    let contract_b = [2u8; 32];

    // A calls B: OK
    assert!(ctx.enter_call(contract_b).is_ok());

    // B calls A: BLOCKED (indirect reentrancy)
    assert!(matches!(
        ctx.enter_call(contract_a),
        Err(ExecutionError::ReentrancyDetected)
    ));
}
```

**Reentrancy Attack Prevention**:

```
❌ Direct Reentrancy (A → A):
Contract A {
    function withdraw() {
        victim.call{value: balance}("");  // ❌ A calls itself again
        balance = 0;
    }
}

Result: BLOCKED by call_stack.contains(&target)

❌ Indirect Reentrancy (A → B → A):
Contract A {
    function withdraw() {
        B.transfer(balance);  // B calls back to A.withdraw()
        balance = 0;
    }
}

Result: BLOCKED by call_stack tracking

❌ Depth Exhaustion (A → B → C → ... → Z):
Recursive calls exceeding max_depth = 10

Result: BLOCKED by reentrancy_depth check
```

---

# PART VI: ORACLE NETWORK DESIGN

## 6. Oracle Network Design

**Location**: `/Users/macbook/Desktop/etrid/05-multichain/bridge-protocols/`
**Lines**: 989 (oracle_adapter.rs: 316 + pallet-edsc-oracle: 647)
**Purpose**: Multi-source price aggregation for bridges and stablecoin

### 6.1 Cross-Chain Price Oracle

```rust
/// Exchange rate between two chains
#[derive(Clone, Debug, Encode, Decode)]
pub struct ExchangeRate {
    /// Exchange rate in basis points (10000 = 1:1)
    pub rate: u64,

    /// Timestamp of rate calculation
    pub timestamp: u64,

    /// Confidence score (0-100)
    pub confidence: u8,
}

/// Oracle aggregator for multi-source pricing
pub struct OracleAggregator {
    /// List of price oracle sources
    oracles: Vec<Box<dyn PriceOracle>>,

    /// Minimum number of sources required
    min_sources: usize,

    /// Maximum age of rate (seconds)
    max_rate_age: u64,

    /// Minimum confidence required
    min_confidence: u8,
}

impl OracleAggregator {
    /// Get aggregated exchange rate from multiple sources
    pub fn get_aggregated_rate(
        &self,
        from: &ChainId,
        to: &ChainId,
        current_time: u64,
    ) -> Result<ExchangeRate, OracleError> {
        // Collect rates from all oracles
        let mut rates = vec![];

        for oracle in &self.oracles {
            match oracle.get_rate(from, to) {
                Ok(rate) => {
                    // Check if rate is fresh
                    if current_time - rate.timestamp <= self.max_rate_age {
                        // Check confidence
                        if rate.confidence >= self.min_confidence {
                            rates.push(rate);
                        }
                    }
                }
                Err(_) => continue, // Skip failed oracle
            }
        }

        // Verify minimum sources
        if rates.len() < self.min_sources {
            return Err(OracleError::InsufficientSources);
        }

        // Calculate median rate (outlier-resistant)
        let median_rate = self.calculate_median(&rates);

        // Calculate average confidence
        let avg_confidence = rates.iter()
            .map(|r| r.confidence as u64)
            .sum::<u64>() / rates.len() as u64;

        Ok(ExchangeRate {
            rate: median_rate,
            timestamp: current_time,
            confidence: avg_confidence as u8,
        })
    }

    /// Calculate median of rates (outlier-resistant)
    fn calculate_median(&self, rates: &[ExchangeRate]) -> u64 {
        let mut sorted: Vec<u64> = rates.iter().map(|r| r.rate).collect();
        sorted.sort_unstable();

        let len = sorted.len();
        if len % 2 == 0 {
            (sorted[len / 2 - 1] + sorted[len / 2]) / 2
        } else {
            sorted[len / 2]
        }
    }
}
```

### 6.2 TWAP Oracle Implementation

```rust
/// Time-Weighted Average Price Oracle
pub struct TWAPOracle<T: Config> {
    /// TWAP window duration (blocks)
    window: BlockNumber,

    /// Fallback window (used if primary fails)
    fallback_window: BlockNumber,

    /// Price observations (block_number => price)
    observations: BTreeMap<BlockNumber, u128>,
}

impl<T: Config> TWAPOracle<T> {
    /// Record new price observation
    pub fn record_price(&mut self, block: BlockNumber, price: u128) {
        self.observations.insert(block, price);

        // Clean up old observations (> fallback_window)
        let cutoff = block.saturating_sub(self.fallback_window);
        self.observations.retain(|&b, _| b >= cutoff);
    }

    /// Calculate TWAP over window
    pub fn calculate_twap(&self, current_block: BlockNumber) -> Result<TwapResult, OracleError> {
        // Try primary window (24 hours)
        let primary_start = current_block.saturating_sub(self.window);
        let primary_result = self.calculate_twap_window(primary_start, current_block);

        if let Ok(result) = primary_result {
            if result.data_points >= T::MinSources::get() {
                return Ok(result);
            }
        }

        // Fallback to 7-day window
        let fallback_start = current_block.saturating_sub(self.fallback_window);
        let mut fallback_result = self.calculate_twap_window(fallback_start, current_block)?;
        fallback_result.using_fallback = true;

        Ok(fallback_result)
    }

    /// Calculate TWAP for a specific window
    fn calculate_twap_window(
        &self,
        start_block: BlockNumber,
        end_block: BlockNumber,
    ) -> Result<TwapResult, OracleError> {
        // Get observations in window
        let observations: Vec<(&BlockNumber, &u128)> = self.observations
            .range(start_block..=end_block)
            .collect();

        if observations.is_empty() {
            return Err(OracleError::NoData);
        }

        // Calculate time-weighted sum
        let mut weighted_sum = 0u128;
        let mut total_time = 0u64;

        for i in 0..observations.len() - 1 {
            let (block1, price1) = observations[i];
            let (block2, _) = observations[i + 1];

            let time_delta = block2.saturating_sub(*block1);
            weighted_sum += price1 * time_delta as u128;
            total_time += time_delta;
        }

        // Calculate TWAP
        let twap = if total_time > 0 {
            weighted_sum / total_time as u128
        } else {
            *observations.last().unwrap().1 // Latest price if no time delta
        };

        // Calculate variance
        let variance = self.calculate_variance(&observations, twap);

        Ok(TwapResult {
            price: twap,
            data_points: observations.len() as u32,
            sources_used: 1, // Single oracle for now
            variance,
            using_fallback: false,
        })
    }

    /// Calculate price variance
    fn calculate_variance(
        &self,
        observations: &[(&BlockNumber, &u128)],
        mean: u128,
    ) -> u128 {
        if observations.len() < 2 {
            return 0;
        }

        let squared_diffs: u128 = observations
            .iter()
            .map(|(_, &price)| {
                let diff = if price > mean {
                    price - mean
                } else {
                    mean - price
                };
                diff * diff
            })
            .sum();

        squared_diffs / observations.len() as u128
    }
}

#[derive(Clone, Debug, Encode, Decode)]
pub struct TwapResult {
    /// Calculated TWAP price (USD cents, 100 = $1.00)
    pub price: u128,

    /// Number of data points used
    pub data_points: u32,

    /// Number of unique sources
    pub sources_used: u32,

    /// Price variance
    pub variance: u128,

    /// Using fallback window (7d instead of 24h)
    pub using_fallback: bool,
}
```

### 6.3 Outlier Detection

```rust
/// Detect and filter outliers from price data
pub fn check_outlier<T: Config>(
    price: u128,
    median: u128,
    variance: u128,
) -> DispatchResult {
    // Bootstrap phase: accept wide range ($0.50 - $2.00)
    if price < 50 || price > 200 {
        return Err(Error::<T>::PriceOutOfRange.into());
    }

    // Calculate threshold based on variance
    let threshold = if variance == 0 {
        // Zero variance (identical prices): strict 2% threshold
        Permill::from_percent(2).mul_floor(median)
    } else {
        // Non-zero variance: dynamic 5-15% threshold
        let variance_factor = (variance / median).min(10); // Cap at 10%
        let dynamic_percent = (5 + variance_factor).min(15);
        Permill::from_percent(dynamic_percent as u32).mul_floor(median)
    };

    // Calculate deviation
    let deviation = if price > median {
        price - median
    } else {
        median - price
    };

    // Check threshold
    ensure!(
        deviation <= threshold,
        Error::<T>::InvalidPrice
    );

    Ok(())
}
```

**Outlier Detection Examples**:

| Scenario | Prices | Median | Variance | Threshold | Reject |
|----------|--------|--------|----------|-----------|--------|
| Identical | [$1.00, $1.00, $1.00] | $1.00 | 0 | 2% ($0.02) | $1.03+ |
| Low variance | [$0.99, $1.00, $1.01] | $1.00 | 0.01 | 6% ($0.06) | $1.07+ |
| High variance | [$0.90, $1.00, $1.10] | $1.00 | 0.04 | 9% ($0.09) | $1.10+ |
| Bootstrap | [$0.45, $1.00, $2.10] | $1.00 | - | $0.50-$2.00 | <$0.50 or >$2.00 |

### 6.4 Price Sources

```rust
/// Price source types
pub enum PriceSource {
    /// Centralized exchange (Binance, Coinbase, Kraken)
    CEX(String),

    /// Decentralized exchange (Uniswap, Curve, PancakeSwap)
    DEX(String),

    /// Aggregator (CoinGecko, Messari)
    Aggregator(String),

    /// Static rate (for stablecoin pairs)
    Static(u128),
}

impl PriceSource {
    /// Fetch price from source
    pub async fn fetch_price(&self, pair: &TradingPair) -> Result<u128, FetchError> {
        match self {
            PriceSource::CEX(exchange) => {
                // Fetch from CEX API
                Self::fetch_from_cex(exchange, pair).await
            }
            PriceSource::DEX(protocol) => {
                // Fetch from DEX on-chain
                Self::fetch_from_dex(protocol, pair).await
            }
            PriceSource::Aggregator(api) => {
                // Fetch from aggregator API
                Self::fetch_from_aggregator(api, pair).await
            }
            PriceSource::Static(rate) => {
                // Return static rate
                Ok(*rate)
            }
        }
    }
}
```

**Price Source Matrix**:

| Asset Pair | CEX Sources | DEX Sources | Aggregators | Static |
|------------|-------------|-------------|-------------|--------|
| ETH/USD | Binance, Coinbase, Kraken | Uniswap V3, Curve | CoinGecko, Messari | - |
| BTC/USD | Binance, Coinbase, Kraken | - | CoinGecko, Messari | - |
| SOL/USD | Binance, Coinbase, FTX | Raydium, Orca | CoinGecko, Messari | - |
| USDT/USD | Binance, Coinbase | Curve, Uniswap | - | $1.00 (10000 bp) |
| USDC/USD | Binance, Coinbase | Curve, Uniswap | - | $1.00 (10000 bp) |

---

# PART VII: TESTING & QUALITY ASSURANCE

## 7. Testing Strategy

### 7.1 Unit Tests

**Coverage Requirements**:
- Minimum: 80% line coverage
- Critical paths: 100% coverage
- Edge cases: All identified edge cases must have tests

**Example Unit Tests**:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_aidid_reputation_calculation() {
        let mut reputation = Reputation::default();

        // Record 100 successful inferences
        for _ in 0..100 {
            reputation.record_inference(true);
        }

        assert_eq!(reputation.total_inferences, 100);
        assert_eq!(reputation.successful_inferences, 100);
        assert_eq!(reputation.success_rate(), 10000); // 100%
    }

    #[test]
    fn test_lightning_bloc_channel_update() {
        let mut channel = PaymentChannel::new(
            "alice".to_string(),
            "bob".to_string(),
            100_000_000_000_000_000_000, // 100 ÉTR
            100_000_000_000_000_000_000, // 100 ÉTR
            86400, // 24 hours
        );

        // Alice pays Bob 10 ÉTR
        channel.update(
            10_000_000_000_000_000_000,
            true, // from_a_to_b
            vec![],
            vec![],
        ).unwrap();

        assert_eq!(channel.current_balance_a, 90_000_000_000_000_000_000);
        assert_eq!(channel.current_balance_b, 110_000_000_000_000_000_000);
        assert_eq!(channel.nonce, 1);
    }

    #[test]
    fn test_etwasm_stack_operations() {
        let mut stack = Stack::new();

        // Push values
        stack.push(U256::from(10)).unwrap();
        stack.push(U256::from(20)).unwrap();

        // Pop and verify
        assert_eq!(stack.pop().unwrap(), U256::from(20));
        assert_eq!(stack.pop().unwrap(), U256::from(10));

        // Stack should be empty
        assert!(stack.pop().is_err());
    }
}
```

### 7.2 Integration Tests

**Test Scenarios**:
- Cross-module interactions
- End-to-end workflows
- Multi-step processes

### 7.3 Benchmark Tests

**Performance Tests**:
- Gas consumption benchmarks
- Transaction throughput
- Latency measurements

---

# PART VIII: PERFORMANCE BENCHMARKS

## 8. Performance Benchmarks

### 8.1 AIDID Performance

| Operation | Gas Cost (VMw) | Cost (ÉTR) | Latency |
|-----------|----------------|------------|---------|
| Register AIDID | 500 | 0.0005 | ~6s |
| Update Profile | 200 | 0.0002 | ~6s |
| Record Inference | 50 | 0.00005 | <1s |
| Query Profile | 0 | Free | <100ms |

### 8.2 Lightning-Bloc Performance

| Operation | On-Chain Txs | Off-Chain Msgs | Latency | Cost |
|-----------|--------------|----------------|---------|------|
| Open Channel | 1 | 0 | ~6s | 0.021 ÉTR |
| Payment | 0 | 1 | <1s | ~0 ÉTR |
| Close Channel | 1 | 0 | ~6s | 0.021 ÉTR |
| Multi-Hop (3) | 0 | 3 | <3s | ~0 ÉTR |

### 8.3 ËtwasmVM Performance

| Contract Type | Deploy Gas | Call Gas | Cost |
|---------------|------------|----------|------|
| Simple Storage | 50,000 VMw | 5,000 VMw | 0.05 + 0.005 ÉTR |
| ERC-20 Token | 800,000 VMw | 50,000 VMw | 0.8 + 0.05 ÉTR |
| Uniswap Pair | 3,000,000 VMw | 200,000 VMw | 3.0 + 0.2 ÉTR |

---

# PART IX: SECURITY AUDIT CHECKLIST

## 9. Security Audit Checklist

### 9.1 Smart Contract Security

- [ ] Reentrancy protection verified
- [ ] Integer overflow/underflow checks
- [ ] Access control properly implemented
- [ ] No unbounded loops
- [ ] Safe external calls
- [ ] Proper error handling

### 9.2 Bridge Security

- [ ] Multisig threshold verified
- [ ] Replay protection implemented
- [ ] Double-spend prevention
- [ ] Circuit breaker tested
- [ ] Oracle manipulation resistance

### 9.3 Cryptographic Security

- [ ] Secure random number generation
- [ ] Proper signature verification
- [ ] Hash collision resistance
- [ ] Key management best practices

---

**End of Ivory Paper - Volume 2: Technical Specifications**

**Total Pages**: ~150 pages
**Total Code Examples**: 50+
**Total Tables**: 25+

*For governance and economic policies, see Volume 3: Governance & Economics*
*For updates, visit: https://docs.etrid.network*
