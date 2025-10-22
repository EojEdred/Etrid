# Contributing to Ëtrid Protocol

Thank you for your interest in contributing to Ëtrid! We're building the future of decentralized democracy through blockchain technology, and we welcome contributions from developers, designers, writers, and blockchain enthusiasts.

This document provides comprehensive guidelines for contributing to the Ëtrid Protocol project.

## Table of Contents

- [Welcome](#welcome)
- [Code of Conduct](#code-of-conduct)
- [How to Contribute](#how-to-contribute)
- [Development Setup](#development-setup)
- [Coding Standards](#coding-standards)
- [Git Workflow](#git-workflow)
- [Testing Guidelines](#testing-guidelines)
- [Documentation Standards](#documentation-standards)
- [Pull Request Checklist](#pull-request-checklist)
- [Review Process](#review-process)
- [Component-Specific Guidelines](#component-specific-guidelines)
- [Getting Help](#getting-help)

---

## Welcome

Ëtrid is a next-generation multichain blockchain implementing Adaptive Stake Finality (ASF) consensus, Partition Burst Chains (PBCs), and annual on-chain Consensus Day governance. Before contributing, please:

1. **Read the [README.md](README.md)** - Understand the project architecture and E³20 protocol
2. **Review [KNOWN_ISSUES.md](KNOWN_ISSUES.md)** - Check current blockers and limitations
3. **Explore the codebase** - Each of the 13 E³20 components has detailed ARCHITECTURE.md files
4. **Join our community** - Connect with other contributors on Discord

**Project Status**: Pre-audit phase, ~97% audit-ready. All 13 E³20 protocol components are implemented with 85-90% test coverage.

---

## Code of Conduct

### Our Pledge

We are committed to providing a welcoming and inspiring community for all. We pledge to:

- **Be respectful**: Treat everyone with respect and kindness
- **Be inclusive**: Welcome diverse perspectives and backgrounds from all cultures and communities
- **Be collaborative**: Work together constructively toward common goals
- **Be professional**: Focus on technical merit and constructive criticism
- **Be considerate**: Think about how your actions affect the community

### Unacceptable Behavior

The following behaviors are unacceptable in our community:

- Harassment, discrimination, or intimidation of any kind
- Trolling, insulting comments, or personal attacks
- Public or private harassment
- Publishing others' private information without explicit permission
- Sexual attention or advances of any kind
- Other conduct inappropriate in a professional setting

### Enforcement

Violations of the Code of Conduct should be reported to **conduct@etrid.io**. All complaints will be reviewed and investigated promptly and fairly. The project team is obligated to maintain confidentiality regarding the reporter of an incident.

**Consequences**: Depending on severity, violations may result in warnings, temporary bans, or permanent expulsion from the project.

---

## How to Contribute

### Ways to Contribute

We welcome many types of contributions:

- **Code**: Fix bugs, implement features, optimize performance
- **Documentation**: Write tutorials, improve docs, translate content
- **Testing**: Write tests, find edge cases, report bugs
- **Bug Reports**: Found a bug? Report it with detailed reproduction steps
- **Feature Requests**: Have an idea? Propose it with clear use cases
- **Design**: UX/UI improvements for wallets and web apps
- **Community**: Answer questions, help others, spread the word

### Areas Needing Help

Current priority areas (check GitHub issues for specific tasks):

- **High Priority**: Property-based tests for critical pallets (consensus, bridge, cryptography)
- **High Priority**: Documentation for SDK implementations (Rust, JS, Python, Swift)
- **Medium Priority**: Frontend development (React wallet-web, Flutter wallet-mobile)
- **Medium Priority**: Runtime pallet development and optimization
- **Low Priority**: Translation of documentation into other languages
- **Low Priority**: Example smart contracts for ËtwasmVM

### Finding Issues

Good starting points:

- **Label: `good first issue`** - Suitable for newcomers
- **Label: `help wanted`** - Community contributions welcome
- **Label: `documentation`** - Documentation improvements
- **Label: `bug`** - Bug fixes needed
- **Label: `enhancement`** - Feature requests

Browse issues: https://github.com/EojEdred/Etrid/issues

### Claiming Issues

1. Comment on the issue: "I'd like to work on this"
2. Wait for assignment (maintainers typically respond within 48 hours)
3. Fork the repository and create a branch
4. Start working and submit a PR when ready
5. If you need more than 7 days, provide a status update

**Note**: If an issue is already assigned, please don't start work on it without coordinating with the assignee.

---

## Development Setup

### Prerequisites

**For Rust/Substrate development**:
- **Rust 1.70+** with `wasm32-unknown-unknown` target
- **Polkadot SDK**: stable2506 (unified across workspace)
- **Linux or macOS** (Windows via WSL2)
- **8GB+ RAM**, 50GB+ free disk space
- **Build tools**: `cmake`, `pkg-config`, `libssl-dev`, `git`, `clang`

**For JavaScript/TypeScript development**:
- **Node.js 18+** (LTS recommended)
- **npm 9+** or **yarn 1.22+**
- **TypeScript 5.0+**

**For Mobile development**:
- **Flutter 3.0+** (for wallet-mobile)
- **Dart SDK 3.0+**
- **Android Studio** or **Xcode** (depending on target platform)

**For Smart Contract development**:
- **Solidity 0.8.20+** (for Ethereum bridge contracts)
- **Hardhat 2.17+**
- **Node.js 18+**

### Fork and Clone

1. **Fork the repository** on GitHub (https://github.com/EojEdred/Etrid)

2. **Clone your fork**:
   ```bash
   git clone https://github.com/YOUR-USERNAME/etrid.git
   cd etrid
   ```

3. **Add upstream remote**:
   ```bash
   git remote add upstream https://github.com/EojEdred/Etrid.git
   ```

4. **Install Rust dependencies**:
   ```bash
   rustup update
   rustup target add wasm32-unknown-unknown
   rustup component add rustfmt clippy
   ```

5. **Install additional tools**:
   ```bash
   cargo install cargo-tarpaulin  # Code coverage
   cargo install cargo-audit      # Security audits
   ```

### Build

**Note**: Due to Polkadot SDK dependency stabilization, some builds may require specific steps. See [KNOWN_ISSUES.md](KNOWN_ISSUES.md) for current status.

**Substrate components** (FlareChain and PBCs):
```bash
# Build FlareChain runtime
cargo build --release -p flare-chain-runtime

# Build FlareChain node
cargo build --release -p flare-chain-node

# Build specific PBC (e.g., EDSC-PBC)
cargo build --release -p edsc-pbc-runtime
cargo build --release -p edsc-pbc-node
```

**Services** (Attestation and Relayer):
```bash
# Attestation service
cd services/attestation-service
npm install && npm run build

# Relayer service
cd ../relayer-service
npm install && npm run build
```

**Smart contracts** (Ethereum bridge):
```bash
cd contracts/ethereum
npm install
npx hardhat compile
```

**SDKs** (4 languages):
```bash
# Rust SDK
cd 13-clients/sdk/rust-etrid-sdk
cargo build

# JavaScript SDK
cd ../js-etrid-sdk
npm install && npm run build

# Python SDK
cd ../python-etrid-sdk
pip install -e .

# Swift SDK (requires macOS)
cd ../swift-etrid-sdk
swift build
```

### Run Tests

**All tests** (comprehensive suite):
```bash
# Rust workspace tests (60/60 passing)
cargo test --workspace

# Property-based tests (57 tests × 1000 cases = 57,000 test cases)
cd tests/property-based
cargo test --release

# Service tests
cd tests
npm install
npm test

# Smart contract tests
cd contracts/ethereum
npx hardhat test
```

### Common Issues

**Issue 1**: `duplicate lang item` errors
- **Solution**: Ensure all dependencies use unified Polkadot SDK version (stable2506)
- **Details**: See [KNOWN_ISSUES.md](KNOWN_ISSUES.md) - Runtime Version Conflict section

**Issue 2**: Build fails with missing features
- **Solution**: Clean build cache and rebuild
  ```bash
  cargo clean
  rm Cargo.lock
  cargo build --release
  ```

**Issue 3**: Node fails to start
- **Solution**: Check chain spec generation and ensure ports are not in use
- **Details**: Default ports are 9944 (WS), 9933 (HTTP), 30333 (P2P)

For more issues, see [KNOWN_ISSUES.md](KNOWN_ISSUES.md) or ask in Discord #development channel.

---

## Coding Standards

### Rust

**Style Guide**: Follow official [Rust style guide](https://rust-lang.github.io/api-guidelines/) and Substrate conventions.

**Formatting**:
```bash
# Format all code (REQUIRED before commit)
cargo fmt --all

# Check formatting without changes
cargo fmt --all -- --check
```

**Linting**:
```bash
# Run Clippy (REQUIRED - must have 0 warnings)
cargo clippy --all-targets --all-features -- -D warnings

# Fix auto-fixable issues
cargo clippy --all-targets --all-features --fix
```

**Guidelines**:
- Use `rustfmt` defaults (no custom configuration)
- Fix **ALL** `clippy` warnings before submitting PR
- Prefer explicit types for public APIs
- Document all public items with `///` doc comments
- Use descriptive variable names (no single-letter except in closures)
- Avoid `.unwrap()` in production code - use proper `Result` handling
- Use `#[cfg(test)]` for test-only code
- Follow Substrate weight conventions for extrinsics

**Example** (pallet extrinsic):
```rust
/// Burns EDSC tokens and emits cross-chain message
///
/// # Parameters
/// - `origin`: Transaction sender (must be signed)
/// - `amount`: Amount to burn in smallest unit (must be > MinBurnAmount)
/// - `recipient`: Destination address on target chain (must be valid)
///
/// # Errors
/// - `InsufficientBalance`: Not enough tokens to burn
/// - `InvalidRecipient`: Recipient address is invalid
/// - `BelowMinimumBurn`: Amount is below minimum threshold
///
/// # Weight
/// O(1) - Single balance update and event emission
#[pallet::call_index(0)]
#[pallet::weight(T::WeightInfo::burn_and_send())]
pub fn burn_and_send(
    origin: OriginFor<T>,
    amount: BalanceOf<T>,
    recipient: Vec<u8>,
) -> DispatchResult {
    let sender = ensure_signed(origin)?;

    // Validate amount
    ensure!(
        amount >= T::MinBurnAmount::get(),
        Error::<T>::BelowMinimumBurn
    );

    // Implementation...

    Ok(())
}
```

### JavaScript/TypeScript

**Style Guide**: Follow [Airbnb JavaScript Style Guide](https://github.com/airbnb/javascript) with TypeScript extensions.

**Formatting**:
```bash
# Format code with Prettier
npm run format

# Check formatting
npm run format:check
```

**Linting**:
```bash
# Run ESLint (REQUIRED before commit)
npm run lint

# Fix auto-fixable issues
npm run lint:fix
```

**Guidelines**:
- Use **ESLint + Prettier** (configuration provided)
- Prefer `const` over `let` (never use `var`)
- Use async/await over promise chains
- Add JSDoc comments for all exported functions
- Use descriptive variable names (camelCase)
- Use TypeScript strict mode
- Prefer functional programming patterns
- Use type guards for runtime type checking

**Example** (attestation service):
```typescript
/**
 * Fetches attestation from service by nonce
 *
 * Retrieves the M-of-N aggregated attestation for a cross-chain message.
 *
 * @param sourceDomain - Source chain domain ID (0=Ethereum, 2=Ëtrid)
 * @param nonce - Unique message nonce
 * @returns Attestation with M-of-N signatures
 * @throws {AttestationNotFoundError} If attestation doesn't exist
 * @throws {NetworkError} If service is unreachable
 *
 * @example
 * const attestation = await fetchAttestationByNonce(0, 123n);
 * console.log(`Signatures: ${attestation.signatures.length}`);
 */
async function fetchAttestationByNonce(
  sourceDomain: number,
  nonce: bigint
): Promise<Attestation> {
  // Validate inputs
  if (sourceDomain < 0) {
    throw new Error('Invalid source domain');
  }

  // Implementation...
}
```

### Solidity

**Style Guide**: Follow [Solidity style guide](https://docs.soliditylang.org/en/latest/style-guide.html) and [ConsenSys best practices](https://consensys.github.io/smart-contract-best-practices/).

**Formatting**:
```bash
# Format with Prettier
npx prettier --write 'contracts/**/*.sol'

# Check formatting
npx prettier --check 'contracts/**/*.sol'
```

**Guidelines**:
- Follow Solidity style guide conventions
- Use NatSpec comments (`@notice`, `@param`, `@return`)
- Explicit visibility modifiers for all functions
- Use Solidity 0.8+ overflow checks (no SafeMath needed)
- Emit events for all state changes
- Use `require` for input validation
- Use `revert` with custom errors for gas efficiency
- Follow checks-effects-interactions pattern
- Use OpenZeppelin contracts where applicable

**Example** (EDSC bridge contract):
```solidity
/// @notice Burns EDSC tokens and sends cross-chain message
/// @dev Emits MessageSent event with nonce for tracking
/// @param recipient Destination address on Ëtrid chain (bytes format)
/// @param amount Amount of EDSC to burn (18 decimals)
/// @return nonce Unique message nonce for attestation tracking
/// @custom:security-note Ensure recipient is valid Ëtrid address
function burnAndSend(
    bytes calldata recipient,
    uint256 amount
) external whenNotPaused nonReentrant returns (uint64 nonce) {
    // Input validation
    require(recipient.length == 32, "Invalid recipient length");
    require(amount > 0, "Amount must be greater than zero");
    require(amount >= minBurnAmount, "Below minimum burn amount");

    // Effects
    _burn(msg.sender, amount);
    nonce = _getNextNonce();

    // Interactions
    emit MessageSent(destinationDomain, nonce, msg.sender, recipient, amount);

    return nonce;
}
```

### Documentation Comments

**Rust** - Use `///` for public items:
```rust
/// Calculates voting power using ASF formula
///
/// Formula: √(stake × coinage)
/// where coinage = time_staked_in_blocks
///
/// # Arguments
/// * `stake` - Amount of ÉTR staked
/// * `coinage` - Number of blocks stake has been held
///
/// # Returns
/// Voting power as u128
///
/// # Examples
/// ```
/// let power = calculate_voting_power(1000, 100800); // 1000 ÉTR for ~7 days
/// assert_eq!(power, 10040); // √(1000 × 100800) ≈ 10040
/// ```
pub fn calculate_voting_power(stake: Balance, coinage: BlockNumber) -> u128 {
    // Implementation
}
```

**TypeScript** - Use JSDoc:
```typescript
/**
 * Submits a governance proposal to the Ëtrid network
 *
 * @param title - Proposal title (max 256 characters)
 * @param description - Proposal description (max 4096 characters)
 * @param proposalType - Type of proposal (InflationRate, NetworkUpgrade, etc.)
 * @returns Transaction hash of submitted proposal
 * @throws {InsufficientBalanceError} If proposer doesn't have deposit amount
 * @throws {InvalidProposalError} If proposal format is invalid
 *
 * @example
 * const hash = await submitProposal(
 *   "Reduce Inflation to 2%",
 *   "This proposal reduces annual inflation from 3% to 2%...",
 *   ProposalType.InflationRate
 * );
 */
```

---

## Git Workflow

### Branch Naming

Create descriptive branches following these conventions:

- `feature/feature-name` - New features (e.g., `feature/consensus-day-voting`)
- `fix/bug-description` - Bug fixes (e.g., `fix/oracle-price-staleness`)
- `docs/documentation-topic` - Documentation (e.g., `docs/sdk-examples`)
- `test/test-description` - Test improvements (e.g., `test/property-based-redemption`)
- `refactor/refactor-description` - Code refactoring (e.g., `refactor/pallet-weights`)
- `chore/maintenance-task` - Maintenance tasks (e.g., `chore/update-dependencies`)

### Workflow Steps

**1. Update your fork**:
```bash
git checkout main
git fetch upstream
git merge upstream/main
git push origin main
```

**2. Create a branch**:
```bash
git checkout -b feature/your-feature-name
```

**3. Make changes**:
- Write clean, readable code
- Follow coding standards
- Add/update tests (REQUIRED)
- Update documentation
- Commit regularly with clear messages

**4. Run checks locally** (REQUIRED before push):
```bash
# Format code
cargo fmt --all
npm run format  # For JS/TS changes

# Lint code
cargo clippy --all-targets --all-features -- -D warnings
npm run lint    # For JS/TS changes

# Run tests
cargo test --workspace
npm test        # For service changes
```

**5. Commit changes**:
Follow [Conventional Commits](https://www.conventionalcommits.org/):

```
type(scope): subject

body (optional)

footer (optional)
```

**Types**:
- `feat`: New feature
- `fix`: Bug fix
- `docs`: Documentation only
- `style`: Code style (formatting, no logic change)
- `refactor`: Code refactoring
- `test`: Adding/updating tests
- `chore`: Maintenance tasks
- `perf`: Performance improvements

**Examples**:
```bash
# Feature commit
git commit -m "feat(bridge): add batched transfer support

Implements batched cross-chain transfers to reduce gas costs.
Adds new extrinsic burn_and_send_batch() with validation.

Closes #123"

# Bug fix commit
git commit -m "fix(attestation): resolve signature verification issue

Fixed ECDSA signature verification for cross-chain messages.
Added test case for edge case where r=0.

Fixes #456"

# Documentation commit
git commit -m "docs(readme): update installation instructions

Added troubleshooting section for common build errors."

# Test commit
git commit -m "test(e2e): add round-trip transfer test

Property-based test for ETH→EDSC→ETH transfers."
```

**6. Push to your fork**:
```bash
git push origin feature/your-feature-name
```

**7. Create Pull Request** on GitHub:
- Use clear, descriptive title following commit convention
- Fill out the PR template completely
- Link related issues
- Add screenshots for UI changes
- Mark as draft if not ready for review

---

## Testing Guidelines

**Ëtrid testing philosophy**: Every critical component must have comprehensive test coverage, including unit tests, integration tests, and property-based tests for financial logic.

### Test Coverage Requirements

- **Target**: **90%+ coverage** for all pallets before mainnet (currently 85-90%)
- **Critical paths**: **100% coverage** for consensus, bridge, cryptography, and token economics
- **Property-based tests**: Required for all financial calculations and state transitions
- **Current status**: 60/60 tests passing, 57 property-based tests with 1000 cases each

### Types of Tests

**1. Unit Tests** (Test individual functions):
```bash
# Run all unit tests
cargo test --lib

# Run specific pallet tests
cargo test -p pallet-edsc-token

# Run with output
cargo test -- --nocapture
```

**2. Integration Tests** (Test component interactions):
```bash
# Run integration tests
cargo test --test '*'

# Run specific integration test
cargo test --test bridge_integration
```

**3. Property-Based Tests** (Test invariants with random inputs):
```bash
# Run all property tests (57 tests × 1000 cases each)
cd tests/property-based
cargo test --release

# Run specific property test file
cargo test --test reserve_ratio_simple --release

# Run with more test cases (default 1000)
PROPTEST_CASES=10000 cargo test --release
```

**4. End-to-End Tests** (Test full user flows):
```bash
cd tests
npm install
npm test

# Run specific E2E test
npm test -- bridge-e2e.test.ts
```

### Property-Based Testing

**Why property-based tests?** They validate invariants across thousands of randomized inputs, catching edge cases that manual tests miss.

**Current property test coverage**:
- **Reserve Ratio**: 23 tests (max collateral, min values, extreme ratios, dust amounts, oracle volatility)
- **Oracle Pricing**: 16 tests (price bounds, staleness, deviation, manipulation detection)
- **Redemption Flows**: 18 tests (amount validation, collateral safety, fee application)

**Total**: **57,000 test cases** passing (57 tests × 1000 cases each)

**Example property-based test**:
```rust
use proptest::prelude::*;

proptest! {
    /// Property: Reserve ratio must always be between 0% and 1000%
    /// Tests across all possible u128 values for total_supply and collateral
    #[test]
    fn reserve_ratio_always_in_valid_range(
        total_supply in 1u128..=u128::MAX,
        collateral_value in 0u128..=u128::MAX,
    ) {
        let ratio = calculate_reserve_ratio(total_supply, collateral_value);

        // Invariant: ratio must be in [0, 1000] (representing 0% to 1000%)
        prop_assert!(ratio >= 0);
        prop_assert!(ratio <= 1000);
    }

    /// Property: Redemptions can never exceed available collateral
    /// Tests flash crash scenarios and extreme market conditions
    #[test]
    fn redemption_never_exceeds_collateral(
        edsc_amount in 1u128..=1_000_000_000_000u128,  // Up to 1T EDSC
        collateral_value in 1u128..=u128::MAX,
        price_per_edsc in 1u128..=10_000u128,          // $0.01 to $100
    ) {
        let redemption_value = calculate_redemption(
            edsc_amount,
            collateral_value,
            price_per_edsc
        );

        // Invariant: redemption value must not exceed available collateral
        prop_assert!(redemption_value <= collateral_value);
    }
}
```

**When to write property-based tests**:
- ✅ Financial calculations (balances, fees, interest)
- ✅ State transitions (must preserve invariants)
- ✅ Oracle pricing (bounds checking, staleness)
- ✅ Cryptographic operations (signature verification)
- ✅ Consensus mechanisms (safety and liveness)

**Property test files location**: `tests/property-based/tests/`

### Test Guidelines

**Structure**: Use Given/When/Then or Arrange/Act/Assert pattern:
```rust
#[test]
fn test_burn_and_send_success() {
    new_test_ext().execute_with(|| {
        // GIVEN: User has sufficient EDSC balance
        let account = AccountId::from([1u8; 32]);
        let amount = 1000 * UNITS;
        assert_ok!(EdscToken::mint(RuntimeOrigin::root(), account, amount));

        // WHEN: User burns tokens to send cross-chain
        assert_ok!(TokenMessenger::burn_and_send(
            RuntimeOrigin::signed(account),
            amount,
            vec![2u8; 32], // Recipient on Ëtrid
        ));

        // THEN: Balance decreased and event emitted
        assert_eq!(EdscToken::balance_of(account), 0);
        System::assert_last_event(
            Event::TokenMessenger(TokenMessengerEvent::BurnMessageSent {
                sender: account,
                amount,
                nonce: 0,
            }).into()
        );
    });
}
```

**Test naming**: Use descriptive names that explain the scenario:
- ✅ `test_burn_with_insufficient_balance_fails()`
- ✅ `test_oracle_price_update_within_deviation_threshold()`
- ✅ `test_consensus_day_voting_power_calculation_with_coinage()`
- ❌ `test_burn()` (too vague)
- ❌ `test1()` (meaningless)

**Edge cases to test**:
- Zero values (amount = 0)
- Minimum values (just above zero)
- Maximum values (u128::MAX)
- Boundary conditions (exactly at threshold)
- Negative cases (insufficient balance, unauthorized, etc.)
- State changes (storage updates, event emissions)

**Mock dependencies**: Use test doubles for external services:
```rust
// Mock oracle for price testing
pub struct MockOracle;
impl PriceProvider for MockOracle {
    fn get_price(asset: AssetId) -> Option<Price> {
        // Return test price
        Some(Price::from_rational(100, 1)) // $100
    }
}
```

**Fast tests**: Unit tests should run in <1 second each. Use `--release` for property tests.

### Coverage Reports

Generate coverage reports with `cargo-tarpaulin`:

```bash
# Install tarpaulin
cargo install cargo-tarpaulin

# Generate coverage report (HTML)
cargo tarpaulin --out Html --output-dir ./target/coverage

# Generate coverage report (terminal)
cargo tarpaulin --out Stdout

# Coverage for specific package
cargo tarpaulin -p pallet-edsc-token --out Html
```

**Coverage targets**:
- **Overall**: 90%+ before mainnet
- **Pallets**: 85%+ minimum
- **Critical pallets** (consensus, bridge, tokens): 95%+
- **Services**: 80%+
- **Contracts**: 90%+

**Current coverage**: 85-90% (see [TEST_COVERAGE_ANALYSIS.md](docs/operations/TEST_COVERAGE_ANALYSIS.md))

---

## Documentation Standards

Good documentation is as important as good code. Every contribution should include appropriate documentation updates.

### Types of Documentation

**1. Code comments**: Explain *why*, not *what*
```rust
// ❌ Bad: Obvious what the code does
// Increment counter
counter += 1;

// ✅ Good: Explains why
// Increment nonce to prevent replay attacks across multiple messages
nonce += 1;
```

**2. API documentation**: Document all public items
```rust
/// ✅ Complete API documentation
/// Validates cross-chain message attestation
///
/// Verifies M-of-N signature threshold and checks for replay attacks.
///
/// # Parameters
/// - `message`: The cross-chain message to validate
/// - `attestation`: M-of-N aggregated signatures
///
/// # Returns
/// - `Ok(())` if attestation is valid
/// - `Err(InvalidAttestation)` if signature threshold not met
/// - `Err(ReplayAttack)` if nonce already used
///
/// # Examples
/// ```
/// let valid = validate_attestation(&message, &attestation)?;
/// ```
pub fn validate_attestation(message: &Message, attestation: &Attestation) -> Result<()>
```

**3. README files**: Every component should have README.md
- Purpose and overview
- Installation/setup
- Usage examples
- API reference (or link to it)
- Contributing guidelines (if component-specific)

**4. Architecture documentation**: See `ARCHITECTURE.md` in each E³20 component
- High-level design
- Component interactions
- Data flows
- Security considerations

**5. User guides**: Tutorials and how-tos in `docs/`
- Step-by-step instructions
- Screenshots/diagrams
- Common issues and solutions

### Documentation Locations

```
etrid/
├── README.md                          # Project overview
├── CONTRIBUTING.md                    # This file
├── KNOWN_ISSUES.md                    # Current issues
├── 01-detr-p2p/ARCHITECTURE.md        # Component architecture
├── docs/
│   ├── whitepaper/                    # Technical specification
│   ├── api/                           # API reference
│   ├── operations/                    # Operational guides
│   └── archive/                       # Historical docs
├── 13-clients/sdk/
│   ├── README.md                      # SDK overview
│   ├── rust-etrid-sdk/README.md       # Rust SDK guide
│   ├── js-etrid-sdk/README.md         # JS SDK guide
│   ├── python-etrid-sdk/README.md     # Python SDK guide
│   └── swift-etrid-sdk/README.md      # Swift SDK guide
└── deployment/README.md               # Deployment guides
```

### Writing Guidelines

- **Clear and concise**: Use simple language, short sentences
- **Use examples**: Show, don't just tell
- **Keep up-to-date**: Update docs when code changes
- **Proper English**: Use spell-check, proper grammar
- **Link related docs**: Help readers find related information
- **Use diagrams**: Visual aids for complex concepts (Mermaid.js)
- **Version information**: Specify which version docs apply to

### Markdown Formatting

```markdown
# Clear Heading (H1 - page title)

Brief introduction paragraph. Keep it concise.

## Subsection (H2 - major sections)

### Sub-subsection (H3 - minor sections)

- Bullet points for lists
- Use **bold** for emphasis
- Use `code` for technical terms
- Use *italics* sparingly

### Code Examples

Always include working code examples:

\`\`\`rust
// Complete, working example
fn example() {
    println!("Hello, Ëtrid!");
}
\`\`\`

### Tables

| Feature | Status | Notes |
|---------|--------|-------|
| ASF Consensus | ✅ Complete | Production-ready |
| EDSC Bridge | ✅ Complete | Audit pending |

### Links

- Internal: [See Architecture](ARCHITECTURE.md)
- External: [Substrate Docs](https://docs.substrate.io/)

### Callouts

**Note**: Informational callout

**Warning**: Important warning

**CRITICAL**: Security-critical information
```

---

## Pull Request Checklist

Before submitting a pull request, ensure you have completed ALL items:

### Pre-Submission Checklist

- [ ] **Code compiles**: `cargo build --release` succeeds with 0 errors
- [ ] **All tests pass**: `cargo test --workspace` succeeds with 0 failures
- [ ] **Clippy warnings resolved**: `cargo clippy -- -D warnings` succeeds with 0 warnings
- [ ] **Code formatted**: `cargo fmt --all` applied
- [ ] **Documentation updated**: README, ARCHITECTURE, or API docs updated as needed
- [ ] **Tests added**: New tests for new functionality, updated tests for changes
- [ ] **CHANGELOG updated**: Added entry to unreleased section (for significant changes)
- [ ] **Branch up-to-date**: Rebased on latest `main` branch
- [ ] **Property tests pass**: `cargo test --release` in `tests/property-based/` succeeds (if applicable)
- [ ] **No `TODO` or `FIXME`**: All TODOs resolved or documented in KNOWN_ISSUES.md

### Pull Request Template

Use this template when creating your PR:

```markdown
## Description

Brief description of what this PR does and why.

Closes #123 (if applicable)

## Type of Change

- [ ] Bug fix (non-breaking change which fixes an issue)
- [ ] New feature (non-breaking change which adds functionality)
- [ ] Breaking change (fix or feature that would cause existing functionality to not work as expected)
- [ ] Documentation update
- [ ] Performance improvement
- [ ] Code refactoring
- [ ] Test improvement

## Component(s) Affected

- [ ] Consensus (09-consensus)
- [ ] Bridge (05-multichain/bridge-protocols)
- [ ] Cryptography (03-security)
- [ ] Smart Contracts (08-etwasm-vm)
- [ ] Pallets (specify: _______)
- [ ] Services (attestation/relayer)
- [ ] SDKs (Rust/JS/Python/Swift)
- [ ] Frontend (wallet-web/wallet-mobile)
- [ ] Documentation
- [ ] Other (specify: _______)

## Testing

How was this tested? Include test commands and results.

```bash
# Test commands run
cargo test -p pallet-edsc-token
cargo test --test reserve_ratio_simple --release
```

### Test Results

- Unit tests: X passing
- Integration tests: Y passing
- Property tests: Z passing (W cases each)
- Manual testing: Describe scenarios tested

## Screenshots (if UI changes)

Add screenshots or screen recordings for visual changes.

## Breaking Changes

If this PR introduces breaking changes, describe:
- What breaks
- Migration path for users
- Version bump required

## Checklist

- [ ] My code follows the style guidelines of this project
- [ ] I have performed a self-review of my own code
- [ ] I have commented my code, particularly in hard-to-understand areas
- [ ] I have made corresponding changes to the documentation
- [ ] My changes generate no new warnings
- [ ] I have added tests that prove my fix is effective or that my feature works
- [ ] New and existing unit tests pass locally with my changes
- [ ] Any dependent changes have been merged and published
- [ ] I have updated KNOWN_ISSUES.md if this resolves any known issues
```

---

## Review Process

### How Reviews Work

1. **Automated Checks** (GitHub Actions CI):
   - ✅ Code compiles successfully
   - ✅ All tests pass
   - ✅ Clippy lints pass
   - ✅ Formatting is correct
   - ✅ No security vulnerabilities detected

2. **Code Review** (Human reviewers):
   - At least **1 maintainer approval** required
   - **2 approvals** for critical components (consensus, cryptography, bridge)
   - Reviewers check: code quality, security, test coverage, documentation

3. **Testing** (Reviewer verification):
   - Reviewer checks out branch and runs tests locally
   - Manual testing for UI/UX changes
   - Performance testing for optimization PRs

4. **Discussion** (Address feedback):
   - Be responsive to review comments
   - Make requested changes promptly
   - Discuss constructively if you disagree
   - Push updates to the same branch

5. **Approval** (Final check):
   - Maintainer approves PR
   - All conversations resolved
   - CI passes

6. **Merge** (Integration):
   - Maintainer merges (usually squash merge)
   - Branch automatically deleted
   - Contributor is thanked

### Review Timeframes

- **Initial review**: Within **48 hours** for most PRs
- **Critical bugs**: Within **24 hours**
- **Large features**: May take **3-5 days** for thorough review
- **Documentation**: Usually **24 hours**

If your PR hasn't been reviewed within these timeframes, ping in Discord #development channel.

### What Reviewers Look For

**Code Quality**:
- Follows coding standards
- Clear and readable
- Properly structured
- No code smells

**Correctness**:
- Logic is sound
- Edge cases handled
- No obvious bugs
- Proper error handling

**Security**:
- No vulnerabilities
- Input validation
- Safe arithmetic (no overflows)
- Proper access control

**Performance**:
- Efficient algorithms
- No unnecessary allocations
- Proper weight calculations (for extrinsics)

**Testing**:
- Adequate test coverage
- Tests actually test the code
- Property tests for financial logic
- No flaky tests

**Documentation**:
- Public items documented
- README updated
- Comments explain complex logic
- Examples provided

### Responding to Feedback

**Do**:
- ✅ Thank reviewers for their time
- ✅ Ask questions if feedback is unclear
- ✅ Make requested changes promptly
- ✅ Explain your reasoning if you disagree
- ✅ Push updates to the same branch
- ✅ Mark conversations as resolved when addressed

**Don't**:
- ❌ Take feedback personally
- ❌ Argue defensively
- ❌ Ignore feedback
- ❌ Force-push (unless asked to rebase)
- ❌ Close PR and open a new one for updates

### After Merge

1. **Branch cleanup**: Your branch will be automatically deleted
2. **Issue closure**: Linked issues will be closed automatically
3. **Recognition**: Contributor credited in release notes
4. **Next release**: Changes included in next version

---

## Component-Specific Guidelines

Some components require extra scrutiny due to their criticality.

### 1. Consensus Changes (09-consensus/)

**Extra Requirements**:
- **2 maintainer approvals** required
- **Formal specification** for consensus algorithm changes
- **Adversarial testing**: Test Byzantine fault scenarios
- **Performance benchmarking**: Measure finality time impact
- **Security review**: External audit may be required

**Key Considerations**:
- ASF consensus safety and liveness properties
- Validator committee rotation logic
- Epoch transition correctness
- Slashing conditions
- Nothing-at-stake prevention

**Test Requirements**:
- **100% test coverage** for consensus logic
- Property-based tests for safety invariants
- Byzantine fault injection tests
- Network partition simulations
- Long-running stress tests (hours)

**Review Process**:
- Initial review by consensus expert
- Secondary review by cryptography expert
- Performance benchmarking required
- Formal verification (if possible)

**Example PR for consensus changes**:
```markdown
## Consensus Change: Improve Epoch Transition

### Specification
Link to formal specification document or include inline:
- Describe algorithm change
- Prove safety and liveness properties
- Show performance impact

### Testing
- 100% coverage achieved ✅
- Property tests: 10 new tests, 10,000 cases each ✅
- Byzantine tests: 5 adversarial scenarios ✅
- Performance: Finality time improved from 15s to 12s ✅

### Security Review
- Internal review complete ✅
- External audit pending (if major change)
```

### 2. Cryptography Changes (03-security/)

**Extra Requirements**:
- **2 maintainer approvals** (one must be cryptography expert)
- **Cryptographic audit** for new primitives or schemes
- **Constant-time operations** for secret handling
- **Test vectors** from reference implementations
- **Side-channel resistance** analysis

**Key Considerations**:
- Use well-established cryptographic libraries (no custom crypto)
- Follow industry standards (ed25519, sr25519, Blake2)
- Proper key generation with sufficient entropy
- Constant-time comparisons for secrets
- Secure memory handling (zeroize after use)

**Test Requirements**:
- **100% test coverage** for cryptographic functions
- Test vectors from official sources (NIST, RFC)
- Negative tests (invalid signatures, tampered data)
- Side-channel timing tests (if applicable)

**Example cryptographic change**:
```rust
// ✅ GOOD: Using standard library, constant-time comparison
use sp_core::sr25519;

pub fn verify_signature(
    public_key: &PublicKey,
    message: &[u8],
    signature: &Signature,
) -> bool {
    sr25519::Pair::verify(signature, message, public_key)
}

// ❌ BAD: Custom crypto, timing attack vulnerable
pub fn verify_signature_bad(
    public_key: &PublicKey,
    message: &[u8],
    signature: &Signature,
) -> bool {
    // Don't roll your own crypto!
    for i in 0..signature.len() {
        if signature[i] != compute_expected_signature(message)[i] {
            return false; // TIMING ATTACK VULNERABLE
        }
    }
    true
}
```

### 3. Smart Contract Changes (08-etwasm-vm/)

**Extra Requirements**:
- **Security audit** required for VM changes
- **Gas metering accuracy** verification
- **Reentrancy protection** testing
- **Storage collision prevention** verification
- **Opcode safety** review

**Key Considerations**:
- Gas costs must prevent DoS attacks
- Storage access properly metered
- No unbounded loops or recursion
- Proper sandboxing (no host system access)
- Deterministic execution across nodes

**Test Requirements**:
- **95%+ test coverage** for VM code
- Fuzzing tests for bytecode execution
- DoS attack scenario tests
- Gas exhaustion tests
- Reentrancy attack tests

**Example VM test**:
```rust
#[test]
fn test_reentrancy_protection() {
    // GIVEN: Malicious contract with reentrant call
    let malicious_contract = compile_contract(r#"
        // Contract that tries to reenter during withdrawal
        fn withdraw() {
            transfer(caller, balance);
            call(this, "withdraw"); // Reentrancy attempt
        }
    "#);

    // WHEN: Contract is executed
    let result = vm.execute(malicious_contract, "withdraw", &[]);

    // THEN: Reentrancy is prevented
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), Error::ReentrancyDetected);
}
```

### 4. Bridge Security (05-multichain/bridge-protocols/edsc-bridge/)

**Extra Requirements**:
- **2 maintainer approvals**
- **Cross-chain message replay prevention** verification
- **Double-spend attack** testing
- **Oracle security** review
- **Custodian authorization** verification

**Key Considerations**:
- Nonce management for replay prevention
- Attestation signature verification (M-of-N threshold)
- Reserve ratio circuit breakers
- Oracle price staleness detection
- Fee mechanism prevents economic attacks

**Test Requirements**:
- **90%+ test coverage**
- Property tests for economic invariants
- Cross-chain integration tests
- Replay attack tests
- Oracle manipulation tests

**Example bridge test**:
```rust
proptest! {
    /// Property: Total EDSC supply on both chains must always equal original supply
    /// This invariant must hold across all transfers and redemptions
    #[test]
    fn bridge_preserves_total_supply(
        transfers in vec(1u128..=1_000_000_000u128, 1..100),
    ) {
        let initial_supply = 1_000_000_000_000u128; // 1T EDSC
        let mut eth_supply = initial_supply;
        let mut etrid_supply = 0u128;

        for amount in transfers {
            // Simulate bridge transfer
            if coin_flip() {
                // ETH -> Ëtrid
                eth_supply -= amount;
                etrid_supply += amount;
            } else {
                // Ëtrid -> ETH
                if etrid_supply >= amount {
                    etrid_supply -= amount;
                    eth_supply += amount;
                }
            }
        }

        // INVARIANT: Total supply preserved
        prop_assert_eq!(eth_supply + etrid_supply, initial_supply);
    }
}
```

### 5. Economic/Financial Logic (06-native-currency/, pallets/)

**Extra Requirements**:
- **Property-based tests** required for all calculations
- **Overflow/underflow protection** verification
- **Economic attack scenarios** testing
- **Fee mechanism** validation

**Key Considerations**:
- All arithmetic uses checked operations (no silent overflow)
- Economic incentives properly aligned
- Fee mechanism prevents spam
- Reserve ratios maintain collateral safety

**Test Requirements**:
- **95%+ test coverage**
- Property tests with 10,000+ cases
- Boundary value tests
- Economic attack simulations

**Example financial property test**:
```rust
proptest! {
    /// Property: Fees can never exceed transaction amount
    #[test]
    fn fees_never_exceed_amount(
        amount in 1u128..=u128::MAX,
        fee_rate in 0u32..=10000u32,  // 0% to 100% (basis points)
    ) {
        let fee = calculate_fee(amount, fee_rate);

        // INVARIANT: Fee ≤ amount
        prop_assert!(fee <= amount);

        // INVARIANT: Amount - fee ≥ 0 (no underflow)
        prop_assert!(amount.checked_sub(fee).is_some());
    }
}
```

---

## Getting Help

### Where to Ask Questions

**1. GitHub Discussions** (https://github.com/EojEdred/Etrid/discussions)
- General questions about the project
- Feature requests and proposals
- Technical architecture discussions
- Best place for async communication

**2. Discord** (https://discord.gg/etrid)
- **#development** - Technical development questions
- **#bridge** - EDSC bridge-specific questions
- **#help** - General help and support
- **#consensus** - ASF consensus discussions
- **#governance** - Governance and Consensus Day
- **#announcements** - Project updates

**3. GitHub Issues** (https://github.com/EojEdred/Etrid/issues)
- Bug reports
- Feature requests
- Task tracking
- Use provided templates

**4. Email**
- **dev@etrid.io** - Development team direct contact
- **security@etrid.io** - Security vulnerability reports (private)
- **conduct@etrid.io** - Code of Conduct violations

### How to Report Bugs

**Good bug report includes**:

1. **Clear title**: Summarize the issue in one line
2. **Environment**: OS, Rust version, branch, commit hash
3. **Steps to reproduce**: Exact steps to trigger the bug
4. **Expected behavior**: What should happen
5. **Actual behavior**: What actually happens
6. **Logs/errors**: Include full error messages
7. **Minimal reproduction**: Simplest case that reproduces the issue

**Example bug report**:
```markdown
### Title
EDSC redemption fails with overflow error for amounts > 1M

### Environment
- OS: macOS 14.0
- Rust: 1.70.0
- Branch: main
- Commit: abc123def

### Steps to Reproduce
1. Mint 2M EDSC to account
2. Call `redeem_edsc(1_500_000 * UNITS)` extrinsic
3. Observe error

### Expected Behavior
Redemption should succeed and return equivalent collateral value

### Actual Behavior
Transaction fails with error: `Arithmetic overflow in redemption calculation`

### Logs
```
Error: Arithmetic overflow
  at pallet_edsc_redemption::calculate_redemption_value (lib.rs:245)
```

### Minimal Reproduction
```rust
#[test]
fn reproduce_redemption_overflow() {
    new_test_ext().execute_with(|| {
        let amount = 1_500_000 * UNITS;
        assert_ok!(redeem_edsc(amount)); // FAILS with overflow
    });
}
```
```

### How to Request Features

**Good feature request includes**:

1. **Problem statement**: What problem does this solve?
2. **Proposed solution**: How should it work?
3. **Alternatives considered**: What other approaches did you consider?
4. **Use cases**: Real-world scenarios where this is useful
5. **Impact**: Who benefits from this feature?

### Weekly Community Calls

**Community Call**:
- **When**: Every Tuesday 16:00 UTC
- **Where**: Discord voice channel
- **Topics**: Project updates, Q&A, demos
- **Duration**: 60 minutes

**Developer Sync**:
- **When**: Every Friday 14:00 UTC
- **Where**: Discord voice channel
- **Topics**: Technical discussions, PR reviews, roadmap planning
- **Duration**: 90 minutes

### Recognition & Rewards

Contributors are recognized through:

1. **CONTRIBUTORS.md file**: Listed as project contributors
2. **Release notes**: Credited in version release notes
3. **Social media**: Shoutouts on Twitter/X for significant contributions
4. **NFT contributor badges**: Coming soon - on-chain proof of contribution
5. **Governance**: Active contributors may receive voting power delegation

---

## Release Process

### Versioning

We use [Semantic Versioning 2.0.0](https://semver.org/):

- **Major version** (X.0.0): Breaking changes, incompatible API changes
- **Minor version** (0.X.0): New features, backward-compatible additions
- **Patch version** (0.0.X): Bug fixes, backward-compatible fixes

**Examples**:
- `1.0.0` → `2.0.0`: Breaking change (e.g., consensus algorithm update)
- `1.0.0` → `1.1.0`: New feature (e.g., new governance proposal type)
- `1.0.0` → `1.0.1`: Bug fix (e.g., fix oracle price staleness check)

### Release Checklist

**Pre-release**:
- [ ] All PRs for release merged to `main`
- [ ] Version bumped in `Cargo.toml` files
- [ ] CHANGELOG.md updated with all changes
- [ ] All tests passing (100% CI green)
- [ ] Documentation updated and current
- [ ] Security audit complete (for mainnet releases)
- [ ] Performance benchmarks run
- [ ] Breaking changes documented

**Release**:
- [ ] Git tag created: `git tag -a v1.0.0 -m "Release v1.0.0"`
- [ ] Tag pushed: `git push origin v1.0.0`
- [ ] GitHub Release created with changelog
- [ ] Binaries built for all platforms (Linux, macOS, Windows)
- [ ] Docker images published
- [ ] Release announcement prepared

**Post-release**:
- [ ] Announcement posted on Discord, Twitter, GitHub Discussions
- [ ] Documentation site updated
- [ ] Contributors thanked
- [ ] Metrics tracked (downloads, usage)

### CHANGELOG Format

Follow [Keep a Changelog](https://keepachangelog.com/) format:

```markdown
# Changelog

All notable changes to Ëtrid will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- New governance proposal type: EmergencyPause
- Property-based tests for redemption flows (18 tests)

### Changed
- Improved oracle staleness detection (now 15 minute threshold)
- Optimized ASF finality latency (12s → 10s average)

### Fixed
- EDSC redemption overflow for amounts > 1M
- Oracle price update race condition

### Security
- Patched reentrancy vulnerability in pallet-etwasm-vm

## [1.0.0] - 2026-06-01

### Added
- Initial mainnet release
- ASF consensus implementation
- 13 Partition Burst Chains
- EDSC stablecoin bridge
- Consensus Day governance

...
```

---

## Security

### Reporting Vulnerabilities

**CRITICAL**: **Do NOT** create public GitHub issues for security vulnerabilities.

**Instead, follow this process**:

1. **Email**: security@etrid.io
2. **Include**:
   - Detailed description of the vulnerability
   - Steps to reproduce
   - Potential impact and severity
   - Suggested fix (if you have one)
   - Your contact information
3. **Response**: We'll respond within **48 hours**
4. **Disclosure**: We'll coordinate responsible disclosure timeline
5. **Recognition**: Security researchers credited (if desired)

### Security Best Practices

**For all contributors**:

- ✅ Never commit secrets, private keys, or passwords
- ✅ Use environment variables for sensitive configuration
- ✅ Validate all inputs (never trust user input)
- ✅ Handle errors properly (no `.unwrap()` in production)
- ✅ Follow principle of least privilege
- ✅ Keep dependencies updated (run `cargo audit` regularly)
- ✅ Use checked arithmetic (avoid overflow/underflow)
- ✅ Test edge cases and boundary conditions
- ✅ Review PRs for security implications
- ✅ Run security linters (Clippy with `-D warnings`)

**For sensitive code** (consensus, cryptography, bridge):

- ✅ Use formal verification when possible
- ✅ Write property-based tests for invariants
- ✅ Request security review from experts
- ✅ Add comprehensive documentation
- ✅ Use constant-time operations for secrets
- ✅ Avoid timing-based side channels

### Bug Bounty Program

**Status**: Coming soon (post-mainnet launch)

**Scope**: All components of Ëtrid Protocol
- Consensus implementation
- Bridge protocols
- Smart contract VM
- Pallets and runtime
- Services (attestation, relayer)
- Frontend applications

**Rewards**: TBD (based on severity)

---

## License

By contributing to Ëtrid Protocol, you agree that your contributions will be licensed under the **MIT License**.

See [LICENSE](LICENSE) file for full license text.

Your contributions are voluntary, and you retain copyright to your work while granting Ëtrid Protocol and users the rights specified in the MIT License.

---

## Additional Resources

### Documentation

- **[README.md](README.md)** - Project overview and quick start
- **[KNOWN_ISSUES.md](KNOWN_ISSUES.md)** - Current blockers and limitations
- **[ROADMAP.md](ROADMAP.md)** - Development roadmap to mainnet
- **[ARCHITECTURE.md files](01-detr-p2p/ARCHITECTURE.md)** - Component architecture (all 13 E³20 components)
- **[Deployment Guide](deployment/README.md)** - Testnet and mainnet deployment
- **[Test Coverage](docs/operations/TEST_COVERAGE_ANALYSIS.md)** - Test coverage analysis
- **[Security Scan](docs/operations/SECURITY_SCAN_SUMMARY.md)** - Vulnerability scan results

### SDK Documentation

- **[SDK Overview](13-clients/sdk/README.md)** - Multi-language SDK guide
- **[Rust SDK](13-clients/sdk/rust-etrid-sdk/README.md)** - Substrate/Tokio async client
- **[JavaScript SDK](13-clients/sdk/js-etrid-sdk/README.md)** - @polkadot/api integration
- **[Python SDK](13-clients/sdk/python-etrid-sdk/README.md)** - Async Python client
- **[Swift SDK](13-clients/sdk/swift-etrid-sdk/README.md)** - iOS/macOS native client

### External Resources

- **[Substrate Documentation](https://docs.substrate.io/)** - FRAME pallet development
- **[Polkadot SDK](https://github.com/paritytech/polkadot-sdk)** - Upstream SDK
- **[Rust Book](https://doc.rust-lang.org/book/)** - Learn Rust programming
- **[Conventional Commits](https://www.conventionalcommits.org/)** - Commit message format
- **[Semantic Versioning](https://semver.org/)** - Versioning scheme

---

## Questions?

Still have questions? We're here to help!

- **Discord**: [discord.gg/etrid](https://discord.gg/etrid) - Real-time chat
- **GitHub Discussions**: [github.com/EojEdred/Etrid/discussions](https://github.com/EojEdred/Etrid/discussions) - Async Q&A
- **Email**: dev@etrid.io - Direct contact
- **Twitter/X**: [@EtridMultichain](https://twitter.com/EtridMultichain) - Announcements

---

## Thank You!

**Thank you for contributing to Ëtrid Protocol!** Your contributions help build the future of decentralized democracy and multichain blockchain technology.

Every contribution matters, whether it's fixing a typo in documentation, adding a test, or implementing a major feature. We appreciate your time and effort.

**Together, we build the free and open decentralized democracy of stakeholders.**

---

<p align="center">
  <strong>Built with passion by the Ëtrid community</strong><br>
  <sub>The Free and Open Decentralized Democracy of Stakeholders</sub>
</p>
