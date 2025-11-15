# Changelog

All notable changes to the ASF (Ascending Scale of Finality) consensus algorithm will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- `MaxEncodedLen` trait support for FRAME storage compatibility (2025-11-15)
  - Added to `ConsensusPhase` enum (4 variants: Prepare, PreCommit, Commit, Decide)
  - Added to `FinalityLevel` enum (5 levels: None, Weak, Moderate, Strong, Irreversible)
  - Added to `Signature` enum (Sr25519/Ed25519 wrappers with fixed 64-byte size)
- Comprehensive `FRAME_COMPATIBILITY.md` documentation
  - Integration strategies for FRAME pallets
  - Migration path from unbounded to bounded types
  - Examples of all three integration approaches

### Changed
- Imported `MaxEncodedLen` trait from `codec` in core modules

### Known Limitations
- Complex types containing `Vec` collections cannot yet derive `MaxEncodedLen`:
  - `Vote`: Contains signature and metadata
  - `ValidityCertificate`: Contains aggregate votes and signatures
  - `VoteAggregate`: Contains unbounded `Vec<ValidatorId>`
  - `AggregateSignature`: Contains unbounded `Vec<Signature>` and `Vec<ValidatorId>`
- Future work required: Refactor to use `BoundedVec` for full FRAME compatibility

### Testing
- ✅ All 87 library tests passing
- ✅ `cargo check` passes without errors
- ✅ `cargo build --release` for flarechain-node: SUCCESS
- ✅ Backward compatibility maintained

## [0.1.0] - 2025-11-14

### Added
- Initial ASF consensus algorithm implementation
- HotStuff 4-phase Byzantine consensus
- PPFA (Proposing Panel for Attestation) rotation
- Validity certificate generation and aggregation
- Ascending Scale of Finality (5 levels: 0-4)
- Cryptographic signature support (Sr25519, Ed25519)
- Byzantine fault detection and graduated slashing
- Network message authentication with replay protection
- Comprehensive test suite (87 tests)

### Security Features
- Cryptographic signature verification for all votes
- Aggregate signature support for certificates
- Nonce-based replay protection
- Timestamp-based message freshness checks
- Byzantine validator detection
- Graduated slashing system (5%, 15%, 40%, 100%)
- Permanent validator exclusion after 5 incidents

### Documentation
- API documentation for all public modules
- Integration tests with production scenarios
- Security model documentation
- GRANDPA migration guide

---

## Git Commit History

### 2025-11-15
- **88efd6f7** - feat(asf-algorithm): Add MaxEncodedLen trait for FRAME storage compatibility

### 2025-11-14
- **10b4d6f1** - Previous ASF implementation work
