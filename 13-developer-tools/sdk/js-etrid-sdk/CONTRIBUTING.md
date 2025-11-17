# Contributing to Ëtrid SDK

Thank you for your interest in contributing to the Ëtrid SDK! This guide will help you get started.

## Code of Conduct

Please read and follow our [Code of Conduct](../../CODE_OF_CONDUCT.md).

## Getting Started

### Prerequisites

- Node.js 16+
- npm or yarn
- Git
- TypeScript knowledge
- Basic blockchain understanding

### Development Setup

1. **Fork and Clone**
   ```bash
   git clone https://github.com/YOUR_USERNAME/etrid-protocol
   cd etrid-protocol/13-developer-tools/sdk/js-etrid-sdk
   ```

2. **Install Dependencies**
   ```bash
   npm install
   ```

3. **Build**
   ```bash
   npm run build
   ```

4. **Run Tests**
   ```bash
   npm test
   ```

## Development Workflow

### 1. Create a Branch

```bash
git checkout -b feature/your-feature-name
# or
git checkout -b fix/your-bug-fix
```

Branch naming:
- `feature/` - New features
- `fix/` - Bug fixes
- `docs/` - Documentation
- `test/` - Test improvements
- `refactor/` - Code refactoring

### 2. Make Changes

Follow our [coding standards](#coding-standards).

### 3. Test Your Changes

```bash
# Run all tests
npm test

# Run specific test
npm test -- StakingWrapper.test.ts

# Run with coverage
npm run test:coverage

# Lint
npm run lint

# Fix lint issues
npm run lint:fix
```

### 4. Commit

We use conventional commits:

```bash
git commit -m "feat: add new method to StakingWrapper"
git commit -m "fix: resolve issue with channel closing"
git commit -m "docs: update README with new examples"
git commit -m "test: add tests for OracleWrapper"
```

**Commit Types**:
- `feat:` - New feature
- `fix:` - Bug fix
- `docs:` - Documentation
- `test:` - Tests
- `refactor:` - Code refactoring
- `perf:` - Performance
- `chore:` - Maintenance

### 5. Push and Create PR

```bash
git push origin feature/your-feature-name
```

Then create a Pull Request on GitHub.

## Coding Standards

### TypeScript

```typescript
// ✅ Good
async getBalance(address: string): Promise<Balance> {
  this.ensureConnected();
  this.validateAddress(address);
  
  try {
    const account = await this.api.query.system.account(address);
    return {
      free: account.data.free.toBigInt(),
      reserved: account.data.reserved.toBigInt(),
    };
  } catch (error) {
    throw new AccountError('Failed to get balance', { address, error });
  }
}

// ❌ Bad
async getBalance(address) {
  const account = await this.api.query.system.account(address);
  return account.data.free;
}
```

**Rules**:
- Always use TypeScript, never `any` unless absolutely necessary
- Always validate inputs
- Always handle errors with custom error classes
- Always add JSDoc comments
- Always use BigInt for token amounts
- Always ensure API is connected

### Documentation

Every public method needs JSDoc:

```typescript
/**
 * Get account balance.
 * 
 * @param address - Account address in SS58 format
 * @returns Balance information with free and reserved amounts
 * @throws {InvalidAddressError} If address format is invalid
 * @throws {NotConnectedError} If API is not connected
 * 
 * @example
 * ```typescript
 * const balance = await accounts.getBalance(address);
 * console.log('Free:', balance.free / 10n**18n, 'ÉTR');
 * ```
 */
async getBalance(address: string): Promise<Balance> {
  // ...
}
```

### Testing

Every new method needs tests:

```typescript
describe('StakingWrapper', () => {
  describe('getValidators', () => {
    it('should return all active validators', async () => {
      // Arrange
      const mockValidators = [address1, address2];
      mockApi.query.session.validators = jest.fn()
        .mockResolvedValue(mockValidators);
      
      // Act
      const result = await wrapper.getValidators();
      
      // Assert
      expect(result).toHaveLength(2);
      expect(result).toContain(address1);
    });
    
    it('should throw error when not connected', async () => {
      mockApi.websocket.connected = false;
      
      await expect(
        wrapper.getValidators()
      ).rejects.toThrow(NotConnectedError);
    });
  });
});
```

**Test Requirements**:
- Test happy path
- Test error cases
- Test edge cases
- Maintain 80%+ coverage
- Use descriptive test names

### Error Handling

Create custom errors:

```typescript
// errors/EtridErrors.ts
export class StakingError extends TransactionError {
  constructor(message: string, context?: any) {
    super(message);
    this.name = 'StakingError';
    this.context = context;
  }
}

// Usage
throw new StakingError('Failed to bond tokens', { 
  address, 
  amount, 
  error 
});
```

## Project Structure

```
src/
├── index.ts              # Main exports
├── client/
│   └── EtridClient.ts   # Main client
├── wrappers/
│   ├── LightningBlocWrapper.ts
│   ├── StakingWrapper.ts
│   └── ...
├── builders/
│   └── TransactionBuilder.ts
├── types/
│   └── enhanced.ts       # Type definitions
└── errors/
    └── EtridErrors.ts    # Error classes

tests/
├── unit/                 # Unit tests
│   ├── LightningBlocWrapper.test.ts
│   └── ...
├── integration/          # Integration tests
│   ├── accounts.integration.test.ts
│   └── ...
└── utils/
    └── testHelpers.ts    # Test utilities

examples/
├── lightning-bloc-payment.ts
└── ...

docs/
├── tutorials/
│   ├── 01-getting-started.md
│   └── ...
└── NPM_PUBLICATION_GUIDE.md
```

## Adding a New Wrapper

1. **Create wrapper file**
   ```bash
   touch src/wrappers/MyPalletWrapper.ts
   ```

2. **Implement wrapper**
   ```typescript
   import { SubstrateInterface } from '@polkadot/api';
   import { Keypair } from '@polkadot/keyring/types';
   import { NotConnectedError, MyPalletError } from '../errors';
   
   export class MyPalletWrapper {
     constructor(private api: SubstrateInterface) {}
     
     private ensureConnected(): void {
       if (!this.api.isConnected) {
         throw new NotConnectedError();
       }
     }
     
     async myMethod(): Promise<void> {
       this.ensureConnected();
       // Implementation
     }
   }
   ```

3. **Add types**
   ```typescript
   // src/types/enhanced.ts
   export interface MyPalletData {
     field1: string;
     field2: bigint;
   }
   ```

4. **Create tests**
   ```bash
   touch tests/unit/MyPalletWrapper.test.ts
   ```

5. **Add to exports**
   ```typescript
   // src/index.ts
   export { MyPalletWrapper } from './wrappers/MyPalletWrapper';
   ```

6. **Create example**
   ```bash
   touch examples/my-pallet-example.ts
   ```

7. **Update documentation**
   - Add to README
   - Add JSDoc comments
   - Create tutorial if complex

## Pull Request Guidelines

### PR Title

Follow conventional commits:
- `feat: add governance delegation support`
- `fix: resolve channel closing issue`
- `docs: update StakingWrapper examples`

### PR Description Template

```markdown
## Description
Brief description of changes

## Type of Change
- [ ] Bug fix
- [ ] New feature
- [ ] Breaking change
- [ ] Documentation update

## Testing
- [ ] All tests pass
- [ ] New tests added
- [ ] Coverage maintained/improved

## Checklist
- [ ] Code follows style guidelines
- [ ] Self-review completed
- [ ] Comments added for complex code
- [ ] Documentation updated
- [ ] No new warnings
- [ ] Tests added
- [ ] All tests pass
```

### Review Process

1. **Automated Checks**
   - Tests must pass
   - Lint must pass
   - Coverage must be 80%+

2. **Code Review**
   - At least one approval required
   - Address all comments
   - Keep commits clean

3. **Merge**
   - Squash and merge preferred
   - Delete branch after merge

## Release Process

1. **Update version**
   ```bash
   npm version patch  # or minor/major
   ```

2. **Update CHANGELOG**
   - Document all changes
   - Follow Keep a Changelog format

3. **Create release**
   ```bash
   git tag v0.1.1
   git push origin v0.1.1
   ```

4. **Publish to NPM**
   ```bash
   npm publish --access public
   ```

## Style Guide

### Code Formatting

We use ESLint and Prettier:

```bash
# Check formatting
npm run lint

# Auto-fix
npm run lint:fix
```

### Naming Conventions

- **Files**: `PascalCase.ts` for classes, `camelCase.ts` for utilities
- **Classes**: `PascalCase`
- **Methods**: `camelCase`
- **Constants**: `UPPER_SNAKE_CASE`
- **Types/Interfaces**: `PascalCase`

### Comments

```typescript
// ✅ Good - explain WHY
// Using TWAP to avoid price manipulation
const price = await oracle.getTWAP(pair, 3600);

// ❌ Bad - explain WHAT (obvious from code)
// Get TWAP
const price = await oracle.getTWAP(pair, 3600);
```

## Performance Guidelines

- Use BigInt for all token amounts
- Avoid unnecessary API calls
- Cache when appropriate
- Use subscriptions for real-time data
- Batch transactions when possible

## Security Guidelines

- Never log private keys
- Validate all inputs
- Use parameterized queries
- Handle errors gracefully
- Don't expose internal errors to users

## Getting Help

- **Discord**: https://discord.gg/etrid
- **GitHub Discussions**: https://github.com/etrid/etrid-protocol/discussions
- **Email**: dev@etrid.io

## Recognition

Contributors will be:
- Listed in CHANGELOG
- Mentioned in release notes
- Added to CONTRIBUTORS.md

## License

By contributing, you agree that your contributions will be licensed under Apache-2.0.

---

**Thank you for contributing to Ëtrid SDK! 🚀**
