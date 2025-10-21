# AIDID Specification - AI Decentralized Identity

**Version**: 1.0.0-draft
**Status**: Draft
**Authors**: Ëtrid Foundation
**Date**: October 20, 2025

## Abstract

AIDID (AI Decentralized Identity) extends the W3C DID specification to provide decentralized identities for Artificial Intelligence agents, models, and systems. This specification defines how AI entities can be uniquely identified, authenticated, and authorized within decentralized networks.

## Motivation

As AI systems become more autonomous and prevalent, there is a critical need for:

1. **Identity**: Uniquely identify AI agents separate from human identities
2. **Provenance**: Track AI model origins, training data, and versions
3. **Capabilities**: Declare what an AI can and cannot do
4. **Authorization**: Control which AI systems can perform specific actions
5. **Liability**: Attribute actions and decisions to specific AI agents
6. **Trust**: Build reputation and trust scores for AI systems
7. **Interoperability**: Allow AI agents to interact across platforms

## DID Format

### Structure

AIDID follows the W3C DID specification with AI-specific extensions:

```
did:etrid:ai:{type}:{identifier}
```

**Components**:
- `did` - Standard DID prefix
- `etrid` - DID method (Ëtrid blockchain)
- `ai` - AI-specific namespace
- `{type}` - AI agent type (see below)
- `{identifier}` - Unique identifier (64 characters max, base58)

### AI Types

| Type | Description | Example |
|------|-------------|---------|
| `llm` | Large Language Model | `did:etrid:ai:llm:gpt4-20240101` |
| `vision` | Computer Vision Model | `did:etrid:ai:vision:yolo-v8` |
| `audio` | Audio Processing Model | `did:etrid:ai:audio:whisper-v3` |
| `multimodal` | Multi-modal AI System | `did:etrid:ai:multimodal:gpt4v` |
| `agent` | Autonomous AI Agent | `did:etrid:ai:agent:trader-bot-001` |
| `ensemble` | Multiple AI Models Combined | `did:etrid:ai:ensemble:medical-diagnosis` |

### Examples

```
# OpenAI GPT-4 model
did:etrid:ai:llm:openai-gpt4-turbo

# Autonomous trading agent
did:etrid:ai:agent:etrid-dex-arbitrage-v2

# Vision model for medical imaging
did:etrid:ai:vision:medical-xray-classifier

# Multimodal content moderation
did:etrid:ai:multimodal:content-moderator-v3
```

## DID Document Structure

### Standard Fields

```json
{
  "@context": [
    "https://www.w3.org/ns/did/v1",
    "https://etrid.network/aidid/v1"
  ],
  "id": "did:etrid:ai:llm:gpt4-turbo",
  "controller": "did:etrid:org:openai",
  "created": "2024-01-15T00:00:00Z",
  "updated": "2024-10-01T00:00:00Z",

  "verificationMethod": [{
    "id": "did:etrid:ai:llm:gpt4-turbo#key-1",
    "type": "Ed25519VerificationKey2020",
    "controller": "did:etrid:org:openai",
    "publicKeyMultibase": "z6MkhaXgBZDvotDkL5257faiztiGiC2QtKLGpbnnEGta2doK"
  }],

  "authentication": ["did:etrid:ai:llm:gpt4-turbo#key-1"],
  "assertionMethod": ["did:etrid:ai:llm:gpt4-turbo#key-1"]
}
```

### AI-Specific Extensions

```json
{
  "aiProfile": {
    "type": "llm",
    "version": "gpt-4-turbo-2024-01-15",
    "architecture": "transformer",
    "parameters": "1.76T",
    "contextWindow": 128000,

    "capabilities": [
      "text-generation",
      "code-generation",
      "reasoning",
      "multi-language"
    ],

    "restrictions": [
      "no-real-time-data",
      "no-image-generation",
      "knowledge-cutoff-2023-12"
    ],

    "safety": {
      "alignment": "rlhf",
      "contentFiltering": true,
      "biasEvaluation": "evaluated",
      "toxicityScore": 0.02
    }
  },

  "modelAttestation": {
    "trainingDataHash": "QmXyz...",
    "modelHashAlgorithm": "sha256",
    "modelHash": "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855",
    "checkpointHash": "b7852b855e3b0c44298fc1c149afbf4c8996fb924",
    "reproducible": false,
    "trainingEnvironment": {
      "framework": "PyTorch 2.0",
      "hardware": "NVIDIA A100 (10,000 GPUs)",
      "trainingTime": "43200 hours"
    }
  },

  "provenance": {
    "creator": "did:etrid:org:openai",
    "baseModel": "did:etrid:ai:llm:gpt4-base",
    "derivedFrom": [],
    "trainingDate": "2024-01-15",
    "releaseDate": "2024-01-25"
  },

  "pricing": {
    "currency": "EDSC",
    "inputTokenPrice": "0.000001",
    "outputTokenPrice": "0.000003",
    "billingMethod": "per-token"
  },

  "endpoint": {
    "apiUrl": "https://api.openai.com/v1/chat/completions",
    "protocol": "openai-compatible",
    "authentication": "bearer-token"
  }
}
```

## AI Capabilities Declaration

### Format

Capabilities define what an AI system can do:

```json
{
  "capabilities": {
    "modalities": {
      "input": ["text", "image", "audio"],
      "output": ["text", "structured-data"]
    },

    "tasks": [
      "text-classification",
      "question-answering",
      "code-generation",
      "translation",
      "summarization"
    ],

    "languages": [
      "en", "es", "fr", "de", "ja", "zh", "ko", "ar"
    ],

    "performance": {
      "latency": "200ms (p50), 500ms (p99)",
      "throughput": "1000 tokens/sec",
      "availability": "99.9%"
    },

    "limitations": {
      "maxTokens": 4096,
      "rateLimit": "10000 requests/min",
      "concurrency": 100
    }
  }
}
```

## Model Attestation

### Purpose

Model attestation provides cryptographic proof of:
- Model architecture and weights
- Training data provenance
- Model version and lineage
- Performance benchmarks

### Attestation Format

```json
{
  "attestation": {
    "version": "1.0",
    "timestamp": "2024-10-20T00:00:00Z",
    "attester": "did:etrid:org:openai",

    "modelFingerprint": {
      "algorithm": "sha256",
      "hash": "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855",
      "weights": {
        "format": "safetensors",
        "size": "350GB",
        "hash": "b7852b855e3b0c44298fc1c149afbf4c8996fb924"
      }
    },

    "trainingData": {
      "datasetHash": "QmXyz...",
      "datasetSize": "13TB",
      "sources": ["web-crawl", "books", "arxiv", "github"],
      "cutoffDate": "2023-12-31",
      "pii": "filtered",
      "licenses": ["cc-by-4.0", "mit", "apache-2.0"]
    },

    "benchmarks": {
      "mmlu": 0.867,
      "humaneval": 0.730,
      "gsm8k": 0.924,
      "truthfulqa": 0.812
    },

    "signature": {
      "type": "Ed25519Signature2020",
      "creator": "did:etrid:org:openai#key-1",
      "signatureValue": "z3sXGC4..."
    }
  }
}
```

## AI Agent Authorization

### Authorization Matrix

Define which actions an AI agent is authorized to perform:

```json
{
  "authorization": {
    "permissions": [
      {
        "action": "read-user-data",
        "resource": "etrid:user-profiles",
        "condition": "user-consent-given"
      },
      {
        "action": "execute-trade",
        "resource": "etrid:dex",
        "condition": "value < 1000 EDSC"
      },
      {
        "action": "modify-data",
        "resource": "none",
        "condition": "never"
      }
    ],

    "restrictions": [
      "no-real-world-actions",
      "no-financial-decisions-above-limit",
      "require-human-approval-for-critical-ops"
    ],

    "auditLog": true,
    "supervisionRequired": false,
    "killSwitch": "did:etrid:user:admin#emergency"
  }
}
```

## Trust and Reputation

### Reputation Score

```json
{
  "reputation": {
    "score": 0.92,
    "totalInferences": 1000000,
    "successRate": 0.98,
    "userRating": 4.7,
    "uptime": 0.999,

    "trustSignals": [
      {
        "signal": "verified-by-etrid-foundation",
        "timestamp": "2024-01-20T00:00:00Z"
      },
      {
        "signal": "audited-by-security-firm",
        "auditor": "did:etrid:org:security-co",
        "timestamp": "2024-03-15T00:00:00Z"
      }
    ],

    "incidents": [],
    "disputes": 0,
    "resolved": 0
  }
}
```

## Liability Attribution

### Responsibility Chain

```json
{
  "liability": {
    "owner": "did:etrid:org:openai",
    "operator": "did:etrid:org:inference-provider",
    "user": "did:etrid:user:alice",

    "terms": {
      "ownerResponsibility": [
        "model-accuracy",
        "bias-mitigation",
        "safety-alignment"
      ],
      "operatorResponsibility": [
        "uptime",
        "data-privacy",
        "access-control"
      ],
      "userResponsibility": [
        "appropriate-use",
        "output-validation",
        "harm-prevention"
      ]
    },

    "insurance": {
      "provider": "did:etrid:org:insurance-dao",
      "coverage": "1000000 EDSC",
      "type": "liability"
    }
  }
}
```

## Inter-AI Verification

### AI-to-AI Authentication

When one AI agent calls another:

```json
{
  "callerAI": "did:etrid:ai:agent:orchestrator",
  "calleeAI": "did:etrid:ai:llm:specialist",
  "timestamp": "2024-10-20T12:00:00Z",
  "nonce": "abc123",
  "signature": "z3sXGC4...",

  "context": {
    "taskId": "task-12345",
    "purpose": "sub-task-execution",
    "userConsent": "did:etrid:user:alice#consent-xyz"
  }
}
```

## Implementation on Ëtrid

### Registry Pallet

```rust
pub struct AIIdentity {
    pub did: Vec<u8>,
    pub ai_type: AIType,
    pub version: Vec<u8>,
    pub controller: AccountId,
    pub model_hash: H256,
    pub capabilities: BoundedVec<Capability>,
    pub restrictions: BoundedVec<Restriction>,
    pub reputation_score: u32, // 0-10000 (0-100.00%)
    pub created_at: BlockNumber,
    pub updated_at: BlockNumber,
}

pub enum AIType {
    LLM,
    Vision,
    Audio,
    Multimodal,
    Agent,
    Ensemble,
}
```

### Extrinsics

```rust
// Register new AI identity
register_ai(origin, did, ai_type, model_hash, capabilities)

// Update AI profile
update_ai_profile(origin, did, new_capabilities, new_restrictions)

// Attest model
attest_model(origin, did, attestation_data, signature)

// Record inference
record_inference(origin, ai_did, user_did, success, cost)

// Update reputation
update_reputation(ai_did, score_delta, reason)

// Grant authorization
authorize_action(origin, ai_did, action, resource, conditions)

// Revoke authorization
revoke_authorization(origin, ai_did, action)
```

## Security Considerations

1. **Key Management**: AI private keys must be protected (HSM recommended)
2. **Model Extraction**: Model hashes don't expose weights
3. **Poisoning**: Attestation includes data provenance checks
4. **Impersonation**: Signatures required for all AI actions
5. **Privacy**: Training data details can be masked with ZK proofs
6. **Replay**: Nonces and timestamps prevent replay attacks

## Privacy Considerations

1. **Training Data**: Hash-only disclosure protects proprietary data
2. **Performance**: Aggregated stats instead of per-query details
3. **User Data**: AI should not store user data without consent
4. **Differential Privacy**: Training should use DP techniques

## Compliance

### AI Regulations

AIDID supports compliance with:
- EU AI Act (high-risk AI systems)
- US AI Executive Order
- NIST AI Risk Management Framework
- ISO/IEC 42001 (AI Management System)

### Required Disclosures

For high-risk AI systems:
- Training data sources and biases
- Performance benchmarks
- Safety testing results
- Human oversight mechanisms
- Incident response procedures

## Future Extensions

1. **Federated Learning**: Track models trained across multiple parties
2. **Model Lineage**: Full DAG of model derivation
3. **Explainability**: Attach explanation methods to DID
4. **Continuous Monitoring**: Real-time performance tracking
5. **Multi-party Attestation**: Multiple attesters for high-stakes AI

## References

- [W3C DID Specification](https://www.w3.org/TR/did-core/)
- [W3C Verifiable Credentials](https://www.w3.org/TR/vc-data-model/)
- [MLCommons Model Cards](https://modelcards.withgoogle.com)
- [EU AI Act](https://artificialintelligenceact.eu/)
- [NIST AI RMF](https://www.nist.gov/itl/ai-risk-management-framework)

## Changelog

- **1.0.0-draft** (2024-10-20): Initial specification

---

**License**: Apache 2.0
**Contact**: foundation@etrid.network
