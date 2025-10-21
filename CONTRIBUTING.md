# Contributing to Ëtrid

Thank you for your interest in contributing to Ëtrid! This document provides guidelines for contributing to the project.

## Table of Contents

- [Code of Conduct](#code-of-conduct)
- [Getting Started](#getting-started)
- [How to Contribute](#how-to-contribute)
- [Development Workflow](#development-workflow)
- [Code Standards](#code-standards)
- [Testing](#testing)
- [Documentation](#documentation)
- [Community](#community)

---

## Code of Conduct

### Our Pledge

We are committed to providing a welcoming and inspiring community for all. We pledge to:

- **Be respectful**: Treat everyone with respect and kindness
- **Be inclusive**: Welcome diverse perspectives and backgrounds
- **Be collaborative**: Work together constructively
- **Be professional**: Focus on technical merit
- **Be considerate**: Think about how your actions affect others

### Unacceptable Behavior

- Harassment, discrimination, or intimidation
- Trolling, insulting comments, or personal attacks
- Public or private harassment
- Publishing others' private information
- Other conduct inappropriate in a professional setting

### Enforcement

Violations of the Code of Conduct may be reported to conduct@etrid.io. All complaints will be reviewed and investigated promptly and fairly.

---

## Getting Started

### Prerequisites

**For Rust/Substrate development**:
- Rust 1.70+ with `wasm32-unknown-unknown` target
- Linux or macOS (Windows via WSL)
- 8GB+ RAM, 50GB+ free disk space

**For JavaScript/TypeScript development**:
- Node.js 18+
- npm or yarn

**For smart contracts**:
- Solidity 0.8.20+
- Hardhat

### Fork and Clone

1. Fork the repository on GitHub
2. Clone your fork:
   ```bash
   git clone https://github.com/YOUR-USERNAME/etrid.git
   cd etrid
   ```

3. Add upstream remote:
   ```bash
   git remote add upstream https://github.com/etrid/etrid.git
   ```

### Build

**Substrate components**:
```bash
# Build EDSC-PBC runtime
cargo build --release -p edsc-pbc-runtime

# Build node
cargo build --release -p edsc-pbc-node
```

**Services**:
```bash
# Attestation service
cd services/attestation-service
npm install && npm run build

# Relayer service
cd ../relayer-service
npm install && npm run build
```

**Smart contracts**:
```bash
cd contracts/ethereum
npm install
npx hardhat compile
```

### Run Tests

```bash
# Rust tests
cargo test --workspace

# Service tests
cd tests
npm install
npm test

# Smart contract tests
cd contracts/ethereum
npx hardhat test
```

---

## How to Contribute

### Ways to Contribute

- 🐛 **Bug Reports**: Found a bug? Report it!
- 💡 **Feature Requests**: Have an idea? Propose it!
- 📝 **Documentation**: Improve docs, write tutorials
- 🧪 **Testing**: Write tests, find edge cases
- 💻 **Code**: Fix bugs, implement features
- 🎨 **Design**: UX/UI improvements
- 🌍 **Translation**: Translate documentation
- 💬 **Community**: Help others, answer questions

### Finding Issues

Good first issues:
- Label: `good first issue`
- Label: `help wanted`
- Label: `documentation`

Browse: https://github.com/etrid/etrid/issues

### Claiming Issues

1. Comment on the issue: "I'd like to work on this"
2. Wait for assignment (maintainers will respond within 48 hours)
3. Fork and start working
4. Submit PR when ready

---

## Development Workflow

### 1. Create a Branch

```bash
# Update main
git checkout main
git pull upstream main

# Create feature branch
git checkout -b feature/your-feature-name
# or
git checkout -b fix/bug-description
```

**Branch naming**:
- `feature/` - New features
- `fix/` - Bug fixes
- `docs/` - Documentation
- `test/` - Test improvements
- `refactor/` - Code refactoring

### 2. Make Changes

- Write clean, readable code
- Follow coding standards (below)
- Add/update tests
- Update documentation
- Commit regularly with clear messages

### 3. Commit Messages

Follow [Conventional Commits](https://www.conventionalcommits.org/):

```
type(scope): subject

body (optional)

footer (optional)
```

**Types**:
- `feat`: New feature
- `fix`: Bug fix
- `docs`: Documentation
- `style`: Code style (formatting)
- `refactor`: Code refactoring
- `test`: Tests
- `chore`: Maintenance

**Examples**:
```
feat(bridge): add support for batched transfers

fix(attestation): resolve signature verification issue

docs(readme): update installation instructions

test(e2e): add round-trip transfer test
```

### 4. Push and Create PR

```bash
# Push to your fork
git push origin feature/your-feature-name

# Create PR on GitHub
```

**PR Template**:
```markdown
## Description
Brief description of changes

## Type of Change
- [ ] Bug fix
- [ ] New feature
- [ ] Breaking change
- [ ] Documentation

## Testing
How was this tested?

## Checklist
- [ ] Code follows style guidelines
- [ ] Tests added/updated
- [ ] Documentation updated
- [ ] All tests pass
- [ ] No new warnings
```

### 5. Code Review

- Be responsive to feedback
- Make requested changes
- Discuss constructively
- Be patient (reviews take time)

### 6. Merge

Once approved:
- Maintainer will merge
- Delete your branch
- Update your fork

---

## Code Standards

### Rust

**Style**:
```bash
# Format code
cargo fmt

# Lint
cargo clippy --all-targets --all-features
```

**Guidelines**:
- Use `rustfmt` defaults
- Fix all `clippy` warnings
- Prefer explicit types for public APIs
- Document public functions with `///`
- Use descriptive variable names

**Example**:
```rust
/// Burns EDSC tokens and emits cross-chain message
///
/// # Parameters
/// - `origin`: Transaction sender
/// - `amount`: Amount to burn (in smallest unit)
/// - `recipient`: Destination address
///
/// # Errors
/// - `InsufficientBalance`: Not enough tokens
/// - `InvalidRecipient`: Invalid destination address
pub fn burn_and_send(
    origin: OriginFor<T>,
    amount: BalanceOf<T>,
    recipient: Vec<u8>,
) -> DispatchResult {
    // Implementation
}
```

### JavaScript/TypeScript

**Style**:
```bash
# Format
npm run format

# Lint
npm run lint
```

**Guidelines**:
- Use ESLint + Prettier
- Prefer `const` over `let`
- Use async/await over promises
- Add JSDoc comments for exported functions
- Descriptive variable names

**Example**:
```typescript
/**
 * Fetches attestation from service by nonce
 * @param sourceDomain - Source chain domain (0=Ethereum, 2=Ëtrid)
 * @param nonce - Message nonce
 * @returns Attestation with signatures
 * @throws Error if not found or network error
 */
async function fetchAttestationByNonce(
  sourceDomain: number,
  nonce: bigint
): Promise<Attestation> {
  // Implementation
}
```

### Solidity

**Style**:
```bash
# Format
npx prettier --write 'contracts/**/*.sol'
```

**Guidelines**:
- Follow Solidity style guide
- Use NatSpec comments
- Explicit visibility modifiers
- Use SafeMath (or Solidity 0.8+ overflow checks)
- Emit events for state changes

**Example**:
```solidity
/// @notice Burns EDSC and sends cross-chain message
/// @param recipient Destination address (bytes32)
/// @param amount Amount to burn
/// @return nonce Message nonce
function burnAndSend(
    bytes calldata recipient,
    uint256 amount
) external whenNotPaused returns (uint64 nonce) {
    // Implementation
    emit MessageSent(destinationDomain, nonce, msg.sender, recipient, amount);
}
```

---

## Testing

### Test Requirements

All PRs must include tests:

- **Unit tests**: Test individual functions
- **Integration tests**: Test component interactions
- **E2E tests**: Test full user flows

### Running Tests

**Rust**:
```bash
# All tests
cargo test --workspace

# Specific pallet
cargo test -p pallet-token-messenger

# With output
cargo test -- --nocapture
```

**Services**:
```bash
cd tests
npm test

# Watch mode
npm run test:watch

# Coverage
npm run test:coverage
```

**Smart contracts**:
```bash
cd contracts/ethereum
npx hardhat test

# Coverage
npx hardhat coverage
```

### Test Guidelines

- **Descriptive names**: `test_burn_with_insufficient_balance_fails()`
- **Clear arrangement**: Given/When/Then pattern
- **Edge cases**: Test boundaries, errors, invalid inputs
- **Mock dependencies**: Use test doubles for external services
- **Fast tests**: Unit tests should run in <1s

**Example**:
```rust
#[test]
fn test_burn_and_send_success() {
    new_test_ext().execute_with(|| {
        // Given: User has sufficient balance
        let account = 1;
        let amount = 100;
        assert_ok!(Balances::set_balance(Origin::root(), account, amount, 0));

        // When: User burns tokens
        assert_ok!(TokenMessenger::burn_and_send(
            Origin::signed(account),
            amount,
            vec![0; 32], // recipient
        ));

        // Then: Balance decreased and event emitted
        assert_eq!(Balances::free_balance(account), 0);
        assert!(System::events().iter().any(|e| {
            matches!(e.event, Event::TokenMessenger(TokenMessengerEvent::BurnMessageSent { .. }))
        }));
    });
}
```

---

## Documentation

### Types of Documentation

1. **Code comments**: Explain *why*, not *what*
2. **API docs**: Public functions, parameters, returns
3. **README files**: Setup, usage, examples
4. **User guides**: End-user tutorials
5. **Technical specs**: Architecture, protocols

### Writing Documentation

**Guidelines**:
- Clear and concise
- Use examples
- Keep up-to-date
- Use proper English
- Link to related docs

**Markdown**:
```markdown
# Clear Heading

Brief introduction paragraph.

## Subsection

- Bullet points for lists
- Use **bold** for emphasis
- Use `code` for technical terms

### Code Examples

\`\`\`rust
// Always include working code examples
fn example() {
    println!("Hello!");
}
\`\`\`

## See Also

- [Related Doc](link)
```

### Documentation Locations

- **Rust docs**: `///` comments above items
- **Service docs**: `README.md` in service directory
- **Contract docs**: NatSpec comments in Solidity
- **User docs**: `docs/` directory
- **API reference**: Auto-generated from code

---

## Community

### Communication Channels

- **Discord**: [discord.gg/etrid](https://discord.gg/etrid)
  - #development - Technical discussions
  - #bridge - Bridge-specific chat
  - #help - Get help
  - #announcements - Project updates

- **GitHub Discussions**: Questions, proposals, ideas

- **Twitter**: [@EtridMultichain](https://twitter.com/EtridMultichain) - Announcements

- **Email**: dev@etrid.io - Direct contact

### Weekly Meetings

**Community Call**:
- When: Every Tuesday 16:00 UTC
- Where: Discord voice channel
- Topics: Project updates, Q&A, demos

**Developer Sync**:
- When: Every Friday 14:00 UTC
- Where: Discord voice channel
- Topics: Technical discussions, PRs, roadmap

### Recognition

Contributors are recognized in:
- `CONTRIBUTORS.md` file
- Release notes
- Social media shoutouts
- NFT contributor badges (coming soon)

---

## Pull Request Process

### Before Submitting

- [ ] Code builds without errors
- [ ] All tests pass
- [ ] No new clippy/lint warnings
- [ ] Documentation updated
- [ ] Commit messages follow convention
- [ ] Branch is up-to-date with main

### PR Checklist

- [ ] Title follows format: `type(scope): description`
- [ ] Description explains changes
- [ ] Links to related issues
- [ ] Screenshots (if UI changes)
- [ ] Breaking changes noted
- [ ] Backward compatibility considered

### Review Process

1. **Automated checks**: CI must pass
2. **Code review**: At least 1 maintainer approval
3. **Testing**: Reviewer tests changes
4. **Discussion**: Address feedback
5. **Approval**: Maintainer approves
6. **Merge**: Maintainer merges

### After Merge

- Branch is deleted
- Issue is closed (if applicable)
- Contributors are thanked
- Changes included in next release

---

## Release Process

### Versioning

We use [Semantic Versioning](https://semver.org/):
- **Major**: Breaking changes (2.0.0)
- **Minor**: New features (1.1.0)
- **Patch**: Bug fixes (1.0.1)

### Release Checklist

- [ ] All PRs merged
- [ ] Version bumped
- [ ] CHANGELOG updated
- [ ] Tests passing
- [ ] Documentation current
- [ ] Security audit (mainnet)
- [ ] Tag created
- [ ] Binaries built
- [ ] Announcement prepared

---

## Security

### Reporting Vulnerabilities

**Do NOT** create public issues for security vulnerabilities.

**Instead**:
1. Email: security@etrid.io
2. Include:
   - Description
   - Steps to reproduce
   - Potential impact
   - Suggested fix (optional)
3. We'll respond within 48 hours
4. Coordinate disclosure timeline

### Security Best Practices

- Never commit secrets
- Use environment variables
- Validate all inputs
- Handle errors properly
- Follow principle of least privilege
- Keep dependencies updated

---

## License

By contributing, you agree that your contributions will be licensed under the MIT License.

---

## Questions?

- 📧 Email: dev@etrid.io
- 💬 Discord: #development channel
- 🐦 Twitter: @EtridMultichain

**Thank you for contributing to Ëtrid!** 🙏

---

<p align="center">
  <strong>Together we build the future of decentralized finance</strong>
</p>
