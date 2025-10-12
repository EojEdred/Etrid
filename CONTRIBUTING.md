# Contributing to Ëtrid

First off, **thank you** for considering contributing to Ëtrid! This project is driven by the community, and we welcome contributions of all kinds.

---

## 📜 Code of Conduct

### Our Pledge
We are committed to providing a welcoming and inspiring community for all. Please be respectful, constructive, and collaborative.

### Unacceptable Behavior
- Harassment, trolling, or discriminatory language
- Personal attacks or insults
- Spam or off-topic discussions
- Publishing others' private information

**Violations**: Report to conduct@etrid.io. Offenders may be banned.

---

## 🚀 How Can I Contribute?

### 1. Report Bugs
Found a bug? Create an issue on GitHub with:
- **Title**: Clear, concise description
- **Steps to Reproduce**: Exact steps that trigger the bug
- **Expected Behavior**: What should happen
- **Actual Behavior**: What actually happens
- **Environment**: OS, Rust version, browser, etc.
- **Screenshots**: If applicable

**Label**: `bug`

---

### 2. Suggest Features
Have an idea? Create an issue with:
- **Title**: Feature name
- **Problem**: What problem does this solve?
- **Proposed Solution**: How should it work?
- **Alternatives**: Other approaches considered
- **Additional Context**: Mockups, diagrams, etc.

**Label**: `enhancement`

---

### 3. Improve Documentation
Documentation is always a priority! You can:
- Fix typos or unclear explanations
- Add examples and tutorials
- Translate docs to other languages
- Create videos or blog posts

**No code review needed** for minor doc fixes—just submit a PR!

---

### 4. Write Code
See [Development Setup](#development-setup) below.

**Good First Issues**: Look for issues labeled `good first issue` or `help wanted`.

**Major Features**: Discuss in an issue first before starting work.

---

## 💻 Development Setup

### Prerequisites
- **Rust** 1.70+ with `wasm32-unknown-unknown` target
- **Node.js** 18+ and npm/yarn (for web apps)
- **Flutter** 3.0+ (for mobile wallet)
- **Git**

### Setup Steps

```bash
# 1. Fork the repo on GitHub, then clone your fork
git clone https://github.com/YOUR_USERNAME/Etrid.git
cd Etrid

# 2. Add upstream remote
git remote add upstream https://github.com/EojEdred/Etrid.git

# 3. Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
rustup update
rustup target add wasm32-unknown-unknown

# 4. Build (when dependencies stabilize)
cargo build

# 5. Run tests
cargo test --workspace

# 6. Create a branch
git checkout -b feature/my-awesome-feature
```

---

## 🏗️ Project Structure

```
etrid/
├── 01-detr-p2p/          # Networking (Rust)
├── 04-accounts/pallet/   # Accounts pallet (Rust)
├── 08-etwasm-vm/pallet/  # VM pallet (Rust)
├── 09-consensus/pallet/  # Consensus pallet (Rust)
├── 10-foundation/        # Governance pallet (Rust)
├── 13-clients/
│   ├── cli/              # CLI tool (Rust)
│   ├── web/              # Web dashboard (React/TS)
│   └── mobile/           # Mobile wallet (Flutter)
├── apps/                 # Frontend apps
├── docs/                 # Documentation
└── scripts/              # Build scripts
```

**Focus Areas:**
- **Rust**: Pallets, consensus, VM
- **Frontend**: React (web), Flutter (mobile)
- **Docs**: Markdown, tutorials, guides

---

## 🧪 Testing

### Rust Tests
```bash
# Run all tests
cargo test --workspace

# Run tests for specific pallet
cargo test -p pallet-accounts

# Run with output
cargo test -- --nocapture
```

### Frontend Tests
```bash
# Web (in apps/web/)
npm test

# Mobile (in 13-clients/mobile/)
flutter test
```

### Integration Tests
```bash
# Run full stack tests
./scripts/test-integration.sh
```

---

## 📝 Coding Standards

### Rust
- **Style**: Use `rustfmt` (run `cargo fmt` before committing)
- **Linting**: Use `clippy` (run `cargo clippy`)
- **Documentation**: Every public function must have doc comments
- **Tests**: Write unit tests for all logic
- **Error Handling**: Use `Result<T, E>`, no `unwrap()` in production code

**Example:**
```rust
/// Transfers tokens from one account to another.
///
/// # Arguments
/// * `from` - Source account
/// * `to` - Destination account
/// * `amount` - Amount to transfer
///
/// # Errors
/// Returns `InsufficientBalance` if `from` has insufficient funds.
pub fn transfer(from: AccountId, to: AccountId, amount: Balance) -> Result<(), Error> {
    // Implementation
}
```

---

### TypeScript/JavaScript
- **Style**: Use Prettier (run `npm run format`)
- **Linting**: Use ESLint (run `npm run lint`)
- **Types**: TypeScript preferred, avoid `any`
- **Components**: Functional components with hooks
- **State**: Use Context API or lightweight state lib (Zustand)

---

### Dart/Flutter
- **Style**: Use `dart format`
- **Linting**: Follow `analysis_options.yaml`
- **Architecture**: BLoC pattern for state management
- **Widgets**: Prefer stateless widgets

---

## 🔀 Git Workflow

### Branching Strategy
- `main` - Stable, production-ready code
- `develop` - Integration branch for features
- `feature/xxx` - New features
- `fix/xxx` - Bug fixes
- `docs/xxx` - Documentation updates

### Commit Messages
Follow [Conventional Commits](https://www.conventionalcommits.org/):

```
<type>(<scope>): <subject>

<body>

<footer>
```

**Types:**
- `feat`: New feature
- `fix`: Bug fix
- `docs`: Documentation only
- `style`: Formatting, no code change
- `refactor`: Code restructure, no behavior change
- `test`: Add or update tests
- `chore`: Build scripts, dependencies

**Examples:**
```
feat(accounts): add multi-sig support

Implement 2-of-3 multi-signature accounts with threshold voting.

Closes #123
```

```
fix(consensus): correct coinage calculation

Coinage was not resetting after vote. Fixed by clearing coinage storage.
```

---

## 📤 Pull Request Process

### Before Submitting
1. ✅ Ensure code compiles: `cargo build`
2. ✅ Run tests: `cargo test --workspace`
3. ✅ Run formatter: `cargo fmt`
4. ✅ Run linter: `cargo clippy`
5. ✅ Update docs if API changed
6. ✅ Rebase on latest `develop`: `git rebase upstream/develop`

### Submitting PR
1. Push to your fork
2. Open PR against `develop` (not `main`)
3. Fill out PR template:
   - **Title**: Clear description
   - **Description**: What does this PR do?
   - **Related Issues**: `Closes #123`, `Fixes #456`
   - **Testing**: How was this tested?
   - **Screenshots**: If UI changes

### Review Process
- **Code Review**: At least 1 maintainer approval required
- **CI Checks**: All tests must pass
- **Merge**: Squash and merge to keep history clean

**Timeline**: Expect review within 3-5 business days.

---

## 🏷️ Issue Labels

| Label | Description |
|-------|-------------|
| `bug` | Something isn't working |
| `enhancement` | New feature request |
| `documentation` | Improvements to docs |
| `good first issue` | Easy for newcomers |
| `help wanted` | Need community assistance |
| `priority: high` | Urgent, blocking |
| `priority: low` | Nice to have |
| `wontfix` | Will not be addressed |
| `duplicate` | Already reported |

---

## 🎨 Design Guidelines

### UI/UX Principles
- **Simple**: Don't make users think
- **Consistent**: Follow design system
- **Accessible**: WCAG 2.1 AA compliance
- **Responsive**: Mobile-first design

### Brand Assets
- **Colors**: See `docs/design/colors.md`
- **Fonts**: Inter (sans-serif), JetBrains Mono (mono)
- **Logo**: Download from `docs/design/logo/`

---

## 🌍 Internationalization (i18n)

We support multiple languages:
- English (primary)
- Spanish, French, German (in progress)

**Contributing Translations:**
1. Find translation files in `locales/`
2. Add your language: `locales/fr.json`
3. Submit PR with translations

---

## 📞 Communication Channels

- **GitHub Issues**: Bug reports, feature requests
- **Discord**: Real-time chat, questions (https://discord.gg/etrid)
- **Forum**: Long-form discussions (https://forum.etrid.io)
- **Email**: hello@etrid.io (for sensitive matters)

---

## 🏆 Recognition

**Contributors are recognized in:**
- `CONTRIBUTORS.md` file
- Release notes
- "Built by" section on website

**Top contributors may receive:**
- ÉTR token grants
- Exclusive NFTs
- Invitation to core team

---

## 📚 Resources

### Learning Materials
- [Substrate Documentation](https://docs.substrate.io)
- [Rust Book](https://doc.rust-lang.org/book/)
- [React Documentation](https://react.dev)
- [Flutter Documentation](https://flutter.dev/docs)

### Ëtrid Resources
- [Whitepaper](docs/whitepaper/)
- [Architecture Guide](docs/architecture/ARCHITECTURE.md)
- [API Reference](docs/api/)

---

## ⚖️ License

By contributing, you agree that your contributions will be licensed under the [MIT License](LICENSE).

---

## 🙏 Thank You!

Every contribution, no matter how small, makes a difference. Thank you for helping build the future of decentralized governance!

---

**Questions?** Open a discussion on GitHub or ask in Discord.

**Ready to contribute?** Pick an issue and let's build! 🚀
