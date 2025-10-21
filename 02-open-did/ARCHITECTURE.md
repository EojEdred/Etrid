# 02. OpenDID - Decentralized Identity System

## Overview

OpenDID provides a W3C DID-compliant decentralized identity system for the Ëtrid blockchain. It includes standard DID functionality for users and organizations, plus the world's first blockchain-native AI identity standard (AIDID) for artificial intelligence agents and models.

**Status:** 🟡 In Progress (Core modules complete, AIDID specification finalized)

## Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                    OpenDID Identity System                   │
├─────────────────────────────────────────────────────────────┤
│                                                               │
│  ┌──────────────────────────────────────────────────────┐   │
│  │                  DID Types & Schemas                  │   │
│  │             (etrid-did-types)                        │   │
│  └────────────┬─────────────────────┬───────────────────┘   │
│               │                     │                        │
│               ↓                     ↓                        │
│  ┌───────────────────┐   ┌──────────────────────┐          │
│  │   DID Registry    │   │    DID Resolver      │          │
│  │  (on-chain)       │   │ (caching + fallback) │          │
│  └─────────┬─────────┘   └──────────┬───────────┘          │
│            │                         │                       │
│            └─────────────┬───────────┘                       │
│                          ↓                                   │
│         ┌────────────────────────────────┐                  │
│         │           AIDID                │                  │
│         │  AI Decentralized Identity     │                  │
│         │  (World's First AI DID)        │                  │
│         └────────────────────────────────┘                  │
│                          ↓                                   │
└──────────────────────────┼───────────────────────────────────┘
                           ↓
                  FlareChain Runtime
                  (Substrate Pallets)
```

## Components

### 1. DID Types (etrid-did-types)

**Location:** `02-open-did/types/`
**Package:** `etrid-did-types`
**Purpose:** DID types and schemas for ÉTRID blockchain

**Description:**
Core type definitions for W3C DID specification compliance, including DID documents, verification methods, service endpoints, and credential structures.

**Key Types:**
- `DidDocument` - W3C-compliant DID document
- `VerificationMethod` - Cryptographic verification methods
- `ServiceEndpoint` - Service endpoint definitions
- `DidUrl` - DID URL parsing and validation

**Dependencies:**
- `serde` - Serialization
- `serde_json` - JSON serialization

**Features:**
- W3C DID Core 1.0 compliance
- JSON-LD context support
- Multi-key support (Ed25519, Secp256k1, etc.)
- Service endpoint management

**Status:** ✅ Complete

---

### 2. DID Registry (etrid-did-registry)

**Location:** `02-open-did/registry/`
**Package:** `etrid-did-registry`
**Purpose:** On-chain DID registration, updates, and access control

**Description:**
Provides on-chain storage and management of DID documents on the Ëtrid blockchain. Handles DID lifecycle (create, update, deactivate, delete) with access control.

**Key Features:**
- On-chain DID document storage
- DID lifecycle management
- Access control (controller-based)
- Version history tracking
- Event emission for changes

**API:**
```rust
// Register new DID
pub fn register_did(did: Did, document: DidDocument) -> Result<()>;

// Update DID document
pub fn update_did(did: Did, document: DidDocument) -> Result<()>;

// Deactivate DID
pub fn deactivate_did(did: Did) -> Result<()>;

// Get DID document
pub fn get_did_document(did: &Did) -> Option<DidDocument>;

// Check if controller
pub fn is_controller(did: &Did, account: &AccountId) -> bool;
```

**Status:** 🟡 Implemented, needs runtime integration

---

### 3. DID Resolver (etrid-did-resolver)

**Location:** `02-open-did/resolver/`
**Package:** `etrid-did-resolver`
**Purpose:** DID resolver with caching, fallback mechanisms, and error handling

**Description:**
Resolves DIDs to DID documents with multiple resolution strategies, caching for performance, and fallback mechanisms for reliability.

**Key Features:**
- Multi-strategy resolution (on-chain, off-chain, cache)
- LRU cache for performance
- Fallback mechanisms
- Async resolution
- Error handling and retry logic

**Resolution Strategy:**
```
1. Check local cache
   ↓ (miss)
2. Query on-chain registry
   ↓ (not found)
3. Try HTTP(S) resolver (if configured)
   ↓ (failed)
4. Return error with suggestions
```

**API:**
```rust
#[async_trait]
pub trait DidResolver {
    async fn resolve(&self, did: &str) -> Result<DidDocument>;
    async fn resolve_with_metadata(&self, did: &str)
        -> Result<(DidDocument, Metadata)>;
}

pub struct CachedResolver {
    cache: LruCache<String, DidDocument>,
    registry: Arc<dyn DidRegistry>,
    http_fallback: Option<HttpResolver>,
}
```

**Status:** ✅ Complete

---

### 4. AIDID - AI Decentralized Identity

**Location:** `02-open-did/aidid/`
**Package:** `aidid`
**Version:** 1.0.0
**Purpose:** **World's first blockchain-native AI identity standard**

**Description:**
AIDID extends W3C DID to provide decentralized identities for AI agents, models, and systems. It addresses identity, provenance, capabilities, authorization, liability, trust, and interoperability for AI entities.

**DID Format:**
```
did:etrid:ai:{type}:{identifier}
```

**AI Types:**
| Type | Description | Example |
|------|-------------|---------|
| `llm` | Large Language Model | `did:etrid:ai:llm:gpt4-turbo` |
| `vision` | Computer Vision Model | `did:etrid:ai:vision:yolo-v8` |
| `audio` | Audio Processing | `did:etrid:ai:audio:whisper-v3` |
| `multimodal` | Multi-modal AI | `did:etrid:ai:multimodal:gpt4v` |
| `agent` | Autonomous Agent | `did:etrid:ai:agent:trader-bot-001` |
| `ensemble` | Combined Models | `did:etrid:ai:ensemble:medical-dx` |

**AI Profile Structure:**
```json
{
  "aiProfile": {
    "type": "llm",
    "version": "4.0",
    "modelArchitecture": "transformer",
    "trainingData": {
      "sources": ["web", "books", "papers"],
      "cutoffDate": "2024-01-01"
    },
    "capabilities": {
      "maxTokens": 128000,
      "languages": ["en", "es", "fr", "de", "zh"],
      "modalities": ["text"]
    },
    "safety": {
      "contentFilter": true,
      "alignmentApproach": "RLHF"
    },
    "performance": {
      "latency": "~500ms",
      "throughput": "100 req/s"
    }
  }
}
```

**Key Features:**
- AI agent unique identification
- Model provenance tracking
- Capability declaration
- Authorization framework
- Reputation system
- On-chain attestation
- Cross-platform interoperability

**Substrate Integration:**
```rust
use frame_support::pallet;
use sp_std::vec::Vec;

#[pallet]
pub mod pallet {
    use super::*;

    #[pallet::storage]
    pub type AIDIDs<T: Config> = StorageMap<
        _,
        Blake2_128Concat,
        Vec<u8>,  // AIDID
        AIDIDDocument,
        OptionQuery,
    >;

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        pub fn register_ai(
            origin: OriginFor<T>,
            aidid: Vec<u8>,
            document: AIDIDDocument,
        ) -> DispatchResult;

        pub fn attest_model(
            origin: OriginFor<T>,
            aidid: Vec<u8>,
            attestation: Attestation,
        ) -> DispatchResult;
    }
}
```

**Status:** ✅ Specification complete, implementation ready for runtime integration

**Reference:** See `AIDID_SPECIFICATION.md` for complete specification

---

## Protocol Layers

### Layer 1: Type System
- W3C DID Core types
- AIDID extensions
- JSON-LD schemas
- Verification methods

### Layer 2: Storage Layer
- On-chain registry (FlareChain)
- DID document storage
- Access control
- Version history

### Layer 3: Resolution Layer
- DID resolver
- Caching mechanisms
- Fallback strategies
- Multi-source resolution

### Layer 4: Application Layer
- AIDID management
- Attestation service
- Reputation tracking
- Capability verification

## DID Methods Supported

### did:etrid (Standard DIDs)
```
did:etrid:user:{identifier}
did:etrid:org:{identifier}
did:etrid:service:{identifier}
```

### did:etrid:ai (AI DIDs - AIDID)
```
did:etrid:ai:llm:{identifier}
did:etrid:ai:vision:{identifier}
did:etrid:ai:audio:{identifier}
did:etrid:ai:multimodal:{identifier}
did:etrid:ai:agent:{identifier}
did:etrid:ai:ensemble:{identifier}
```

## API Design

### Core DID Operations

```rust
use etrid_did_types::{Did, DidDocument};
use etrid_did_registry::Registry;
use etrid_did_resolver::Resolver;

// Create DID
let did = Did::new("etrid", "user", "alice123");

// Create DID document
let doc = DidDocument::builder()
    .id(did.clone())
    .controller(did.clone())
    .verification_method(vm)
    .authentication(vec![vm.id()])
    .build()?;

// Register on-chain
registry.register_did(did.clone(), doc.clone()).await?;

// Resolve DID
let resolved = resolver.resolve(&did.to_string()).await?;

// Update DID document
doc.add_service_endpoint(service);
registry.update_did(did, doc).await?;
```

### AIDID Operations

```rust
use aidid::{AIDID, AIDIDDocument, AIProfile};

// Create AI DID
let aidid = AIDID::new("llm", "gpt4-turbo")?;

// Create AI profile
let profile = AIProfile {
    ai_type: AIType::LLM,
    version: "4.0".to_string(),
    capabilities: Capabilities {
        max_tokens: 128000,
        languages: vec!["en", "es", "fr"],
        modalities: vec!["text"],
    },
    safety: SafetyProfile {
        content_filter: true,
        alignment_approach: "RLHF".to_string(),
    },
};

// Create AIDID document
let doc = AIDIDDocument::builder()
    .id(aidid.clone())
    .controller("did:etrid:org:openai")
    .ai_profile(profile)
    .build()?;

// Register AI identity
aidid_registry.register_ai(aidid, doc).await?;

// Attest model
let attestation = Attestation {
    attester: "did:etrid:org:audit-firm",
    claim: "safety-verified",
    evidence: "report-hash-xyz",
    timestamp: now(),
};
aidid_registry.attest_model(&aidid, attestation).await?;
```

## Integration with FlareChain

### Substrate Pallet Integration

```rust
// In FlareChain runtime
impl pallet_did_registry::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type MaxDocumentSize = ConstU32<10240>;
    type MaxControllers = ConstU32<10>;
}

impl pallet_aidid::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type MaxAIProfiles = ConstU32<100>;
    type AttestationProvider = AttestationOracle;
}

construct_runtime!(
    pub enum Runtime where
        Block = Block,
        NodeBlock = opaque::Block,
        UncheckedExtrinsic = UncheckedExtrinsic
    {
        // ... other pallets
        DIDRegistry: pallet_did_registry,
        AIDID: pallet_aidid,
    }
);
```

## Security Features

### Identity Security
- Cryptographic verification (Ed25519, Secp256k1)
- Controller-based access control
- Multi-signature support
- Key rotation mechanisms

### AI Identity Security
- Model attestation (third-party verification)
- Capability verification
- Provenance tracking
- Safety profile validation

### On-Chain Security
- Access control enforcement
- State transition validation
- Event auditing
- DDoS protection (rate limiting)

## Use Cases

### Standard DID Use Cases
1. **User Identity:** Decentralized user authentication
2. **Organization Identity:** Verifiable organizational credentials
3. **Service Identity:** API service authentication
4. **IoT Identity:** Device identity and authorization

### AIDID Use Cases
1. **AI Model Registry:** Track deployed AI models
2. **Autonomous Agents:** Identify and authorize trading bots, automation agents
3. **Content Moderation:** Attribute moderation decisions to specific AI systems
4. **Medical AI:** Track diagnostic AI with liability attribution
5. **Creative AI:** Attribute generated content to specific AI models
6. **AI Marketplace:** Buy/sell AI services with verified identities
7. **Multi-Agent Systems:** Coordinate multiple AI agents with verified identities

## Performance Characteristics

### Resolution Performance
- **Cache hit:** <1ms
- **On-chain query:** ~100ms (block time dependent)
- **HTTP fallback:** ~500ms (network dependent)

### Storage
- **DID Document Size:** ~1-10 KB (depends on keys and services)
- **AIDID Document Size:** ~2-20 KB (includes AI profile)

### Scalability
- **DIDs per block:** Limited by block size/weight
- **Total DIDs:** Unlimited (map storage)

## Testing

### Unit Tests
```bash
# Test DID types
cargo test -p etrid-did-types

# Test registry
cargo test -p etrid-did-registry

# Test resolver
cargo test -p etrid-did-resolver

# Test AIDID
cargo test -p aidid
```

### Integration Tests
```bash
# Full DID lifecycle test
cargo test --test did_lifecycle

# AIDID integration test
cargo test --test aidid_integration
```

## Known Issues

1. **Runtime Integration Pending** - AIDID pallet needs FlareChain runtime integration
2. **HTTP Resolver** - External HTTP DID resolution not yet implemented
3. **DID Deactivation** - Deactivation logic needs enhancement
4. **Cache Eviction** - LRU cache eviction policy needs tuning

## Roadmap

### Phase 1: Core DID Functionality (✅ Complete)
- [x] DID type definitions
- [x] DID registry implementation
- [x] DID resolver with caching
- [x] AIDID specification

### Phase 2: Runtime Integration (🟡 In Progress)
- [ ] Integrate DID registry pallet to FlareChain
- [ ] Integrate AIDID pallet to FlareChain
- [ ] On-chain tests
- [ ] RPC endpoints

### Phase 3: AIDID Ecosystem (Planned)
- [ ] AIDID attestation service
- [ ] AI model verification
- [ ] Reputation oracle
- [ ] AIDID marketplace

### Phase 4: Advanced Features (Planned)
- [ ] HTTP DID resolver
- [ ] DID rotation mechanisms
- [ ] Cross-chain DID resolution
- [ ] Zero-knowledge proofs for privacy

## Comparison with Other DID Systems

| Feature | Ëtrid OpenDID | Other Blockchain DIDs |
|---------|---------------|----------------------|
| W3C Compliance | ✅ Full | ✅ Varies |
| AI Identity (AIDID) | ✅ **First in world** | ❌ None |
| On-chain Registry | ✅ Yes | ✅ Yes |
| Caching | ✅ LRU cache | 🟡 Varies |
| Multi-key Support | ✅ Ed25519, Secp256k1 | ✅ Yes |
| Attestation | ✅ On-chain | 🟡 Varies |

## References

### Standards
- **W3C DID Core 1.0:** https://www.w3.org/TR/did-core/
- **JSON-LD:** https://www.w3.org/TR/json-ld11/
- **Verifiable Credentials:** https://www.w3.org/TR/vc-data-model/

### Ëtrid Documentation
- **AIDID Specification:** `AIDID_SPECIFICATION.md`
- **Substrate DID Pallet:** `registry/src/lib.rs`
- **Type Definitions:** `types/src/lib.rs`

### External Resources
- **DIF (Decentralized Identity Foundation):** https://identity.foundation/
- **Substrate Identity Pallet:** https://docs.substrate.io/reference/frame-pallets/identity/

---

**Component:** 02-open-did
**Version:** 0.1.0 (types, registry, resolver), 1.0.0 (aidid)
**Status:** In Progress (Core complete, runtime integration pending)
**Innovation:** World's first blockchain-native AI identity standard (AIDID)
**Last Updated:** October 20, 2025
