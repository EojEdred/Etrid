# Ëtrid SDK Language Bindings

This directory contains language-specific bindings for the Ëtrid SDK.

## Structure

```
sdk/
├── src/              - Core Rust SDK implementation (714 lines)
├── Cargo.toml        - Rust dependencies and features
└── bindings/         - Language-specific bindings (THIS DIRECTORY)
    ├── js-etrid-sdk/       - JavaScript/TypeScript SDK
    ├── python-etrid-sdk/   - Python SDK
    ├── rust-etrid-sdk/     - Pure Rust SDK (re-export)
    └── swift-etrid-sdk/    - Swift SDK for iOS/macOS
```

## Implementation Status

All language bindings are **PLANNED** for implementation after mainnet deployment.

| Language | Status | Priority | Target Release |
|----------|--------|----------|----------------|
| Rust | ✅ COMPLETE | Critical | v1.0 (Current) |
| JavaScript/TypeScript | 📋 Planned | High | v1.1 (Post-mainnet) |
| Python | 📋 Planned | Medium | v1.2 |
| Swift | 📋 Planned | Low | v1.3 |

## Architecture

The Ëtrid SDK follows a **Rust-first** architecture:

1. **Core SDK** (`/sdk/src/`) - Implemented in Rust
2. **FFI Layer** - C-compatible interface for cross-language support
3. **Language Bindings** - Wrappers around FFI in each target language

This approach ensures:
- Single source of truth (Rust implementation)
- Type safety and performance
- Automatic bindings generation
- Consistency across languages

## Implementation Roadmap

### Phase 1: Rust SDK (✅ COMPLETE)

**Location:** `/sdk/src/lib.rs`

**Features:**
- Wallet operations
- Validator management
- DAO governance
- Transaction building
- Account management
- Multichain support (FlareChain + 13 PBCs)

**Lines of Code:** 714

### Phase 2: JavaScript/TypeScript SDK (📋 Planned for v1.1)

**Target:** `bindings/js-etrid-sdk/`

**Approach:**
- Use `wasm-bindgen` to compile Rust to WASM
- Generate TypeScript type definitions
- NPM package: `@etrid/sdk`

**Features:**
- Browser and Node.js support
- TypeScript-first with full type safety
- Promise-based async API
- Integration with Polkadot.js (already exists in `apps/wallet-web/`)

**Estimated Effort:** 2-3 weeks

### Phase 3: Python SDK (📋 Planned for v1.2)

**Target:** `bindings/python-etrid-sdk/`

**Approach:**
- Use `PyO3` or `maturin` for Python bindings
- PyPI package: `etrid-sdk`

**Features:**
- Python 3.8+ support
- Type hints and mypy compatibility
- Async/await support
- Integration with data science libraries (pandas, jupyter)

**Estimated Effort:** 2-3 weeks

### Phase 4: Swift SDK (📋 Planned for v1.3)

**Target:** `bindings/swift-etrid-sdk/`

**Approach:**
- Use Swift FFI or `swift-bridge`
- CocoaPods/SPM package

**Features:**
- iOS and macOS support
- Swift 5.5+ with async/await
- Integration with existing Flutter mobile wallet

**Estimated Effort:** 3-4 weeks

### Phase 5: Additional Languages (📋 Future)

Potential additional bindings:
- **Go** - For backend services
- **Java/Kotlin** - For Android native development
- **C#/.NET** - For enterprise integrations

## Development Guidelines

When implementing language bindings:

1. **Use the Rust SDK as source of truth**
   - All logic lives in `/sdk/src/`
   - Bindings are thin wrappers only

2. **Follow language conventions**
   - Use idiomatic naming (camelCase for JS, snake_case for Python, etc.)
   - Provide language-native async patterns
   - Write comprehensive documentation

3. **Generate bindings automatically where possible**
   - Use code generation tools (wasm-bindgen, PyO3, etc.)
   - Don't manually maintain FFI code

4. **Maintain feature parity**
   - All bindings should expose the same functionality
   - Version numbers should match core SDK

5. **Test thoroughly**
   - Unit tests for each binding
   - Integration tests against live chains
   - CI/CD for all languages

## Current Integration

While these bindings are planned, the Ëtrid ecosystem already has working integrations:

- **Web Wallet:** Uses Polkadot.js (`apps/wallet-web/etrid-crypto-website/lib/api/flarechain.ts`)
- **Mobile Wallet:** Uses Substrate Connect (`apps/wallet-mobile/etrid-wallet/`)
- **Governance UI:** Uses Snapshot SDK (`apps/governance-ui/`)

The new SDK bindings will provide a **unified, consistent interface** across all platforms.

## Contributing

SDK development will begin after mainnet deployment. To contribute:

1. Wait for the implementation phase to begin
2. Check GitHub issues for SDK tasks
3. Follow the contribution guidelines in `/CONTRIBUTING.md`
4. Join the #sdk channel in Discord

## Contact

For SDK-related questions:
- Discord: #sdk-development
- Email: sdk@etrid.io
- Docs: https://docs.etrid.io/sdk

---

**Last Updated:** October 21, 2025
**Status:** Bindings planned for post-mainnet implementation
