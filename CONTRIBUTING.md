# Contributing to Ëtrid

Thank you for your interest in contributing to Etrid! This document provides guidelines for contributing to the project.

## Table of Contents
- [Code of Conduct](#code-of-conduct)
- [Getting Started](#getting-started)
- [Development Workflow](#development-workflow)
- [Pull Request Process](#pull-request-process)
- [Code Style Guidelines](#code-style-guidelines)
- [Testing Requirements](#testing-requirements)
- [Documentation](#documentation)

## Code of Conduct

- Be respectful and inclusive
- Welcome newcomers
- Focus on constructive feedback
- Help build a positive community

## Getting Started

1. **Fork the repository**
2. **Clone your fork:**
   ```bash
   git clone https://github.com/YOUR_USERNAME/etrid.git
   cd etrid
   ```
3. **Install dependencies:**
   ```bash
   make install
   ```
4. **Build the project:**
   ```bash
   make build
   ```
5. **Run tests:**
   ```bash
   make test
   ```

## Development Workflow

1. Create a feature branch:
   ```bash
   git checkout -b feature/your-feature-name
   ```
2. Make your changes
3. Run tests: `make test`
4. Run linter: `make lint`
5. Format code: `make format`
6. Commit with descriptive messages
7. Push to your fork
8. Open a Pull Request

## Pull Request Process

1. **Update documentation** for any changed functionality
2. **Add tests** for new features
3. **Ensure all tests pass**: `make test`
4. **Update CHANGELOG.md** with your changes
5. **Request review** from maintainers
6. **Address review feedback**
7. **Squash commits** if requested

### PR Title Format
```
type(scope): short description

Examples:
feat(pallet-oracle): add price aggregation
fix(sdk): resolve connection timeout
docs(user-guide): update staking instructions
```

## Code Style Guidelines

### Rust Code
- Follow official [Rust Style Guide](https://doc.rust-lang.org/1.0.0/style/)
- Run `cargo fmt` before committing
- Run `cargo clippy` and fix warnings
- Use meaningful variable names
- Add comments for complex logic
- Write doc comments for public APIs

### TypeScript/JavaScript
- Use TypeScript for type safety
- Follow [Airbnb Style Guide](https://github.com/airbnb/javascript)
- Run `npm run lint` before committing
- Use async/await over callbacks
- Prefer functional programming patterns

### Documentation
- Use clear, concise language
- Include code examples
- Update affected documentation
- Add screenshots for UI changes

## Testing Requirements

All code must include appropriate tests:

### Rust Tests
```rust
#[test]
fn test_functionality() {
    // Test implementation
}
```

### JavaScript/TypeScript Tests
```typescript
describe('Feature', () => {
  it('should work correctly', () => {
    // Test implementation
  });
});
```

### Test Coverage
- Maintain minimum 80% code coverage
- Test edge cases and error conditions
- Include integration tests for complex features

## Documentation

Update these files as appropriate:
- `docs/API_REFERENCE.md` for API changes
- `docs/USER_GUIDE.md` for user-facing features
- `docs/DEVELOPER_GUIDE.md` for developer features
- `README.md` for major changes
- Inline code comments

## Questions?

- **Discord:** discord.gg/etrid
- **GitHub Discussions:** github.com/etrid/etrid/discussions
- **Email:** dev@etrid.io

Thank you for contributing!
