# Testing Guide

Comprehensive guide to testing applications built on ËTRID.

## Testing Environment

ËTRID provides multiple testing environments:

| Environment | Purpose | Reset Frequency |
|-------------|---------|-----------------|
| **Local Node** | Development, unit tests | Manual |
| **Ember Testnet** | Integration testing | Stable |
| **Staging** | Pre-production testing | Weekly |
| **Mainnet** | Production | Never |

---

## Local Development Node

### Quick Start

```bash
# Clone repository
git clone https://github.com/etrid/etrid.git
cd etrid

# Build runtime
cargo build --release

# Run local node
./target/release/etrid --dev --tmp
```

Your local node is now running at:
- WebSocket: `ws://127.0.0.1:9944`
- HTTP RPC: `http://127.0.0.1:9933`

### Benefits
- ✅ Instant block production
- ✅ Pre-funded test accounts
- ✅ Full control over chain state
- ✅ No rate limits
- ✅ Offline development

---

## Ember Testnet

### Connecting to Testnet

**WebSocket Endpoints:**
```
wss://testnet-rpc.etrid.org
wss://testnet-backup.etrid.org
```

**Faucet:**
```
https://faucet.etrid.org
```

### Getting Test Tokens

```bash
# Using CLI
etrid-cli request-tokens --network testnet --address YOUR_ADDRESS

# Or visit faucet website
https://faucet.etrid.org
```

---

## Unit Testing

### Rust Runtime Tests

```rust
// runtime/src/tests.rs
#[cfg(test)]
mod tests {
    use super::*;
    use frame_support::{assert_ok, assert_noop};

    #[test]
    fn transfer_works() {
        new_test_ext().execute_with(|| {
            // Setup
            let alice = 1u64;
            let bob = 2u64;
            let amount = 100;

            // Execute
            assert_ok!(Balances::transfer(
                Origin::signed(alice),
                bob,
                amount
            ));

            // Verify
            assert_eq!(Balances::free_balance(bob), amount);
        });
    }

    #[test]
    fn transfer_insufficient_balance_fails() {
        new_test_ext().execute_with(|| {
            let alice = 1u64;
            let bob = 2u64;
            let amount = 1_000_000; // More than alice has

            assert_noop!(
                Balances::transfer(Origin::signed(alice), bob, amount),
                Error::<Test>::InsufficientBalance
            );
        });
    }
}
```

**Running Tests:**
```bash
cargo test
cargo test --package pallet-my-feature
cargo test test_name
```

---

## Integration Testing

### JavaScript/TypeScript Tests

```typescript
// tests/integration/transfer.test.ts
import { ApiPromise, WsProvider } from '@polkadot/api';
import { Keyring } from '@polkadot/keyring';
import { expect } from 'chai';

describe('Transfer Integration Tests', () => {
  let api: ApiPromise;
  let alice, bob;

  before(async () => {
    const provider = new WsProvider('ws://127.0.0.1:9944');
    api = await ApiPromise.create({ provider });

    const keyring = new Keyring({ type: 'sr25519' });
    alice = keyring.addFromUri('//Alice');
    bob = keyring.addFromUri('//Bob');
  });

  it('should transfer tokens successfully', async () => {
    const amount = 1000000000000; // 1 ÉTR

    const initialBalance = await api.query.system.account(bob.address);

    await api.tx.balances
      .transfer(bob.address, amount)
      .signAndSend(alice);

    // Wait for block
    await new Promise(resolve => setTimeout(resolve, 6000));

    const finalBalance = await api.query.system.account(bob.address);
    expect(finalBalance.data.free.toBigInt()).to.be.greaterThan(
      initialBalance.data.free.toBigInt()
    );
  });

  after(async () => {
    await api.disconnect();
  });
});
```

**Running Tests:**
```bash
npm test
npm run test:integration
```

---

## Smart Contract Testing

### ink! Contract Tests

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use ink_lang as ink;

    #[ink::test]
    fn new_works() {
        let contract = MyContract::new(100);
        assert_eq!(contract.get(), 100);
    }

    #[ink::test]
    fn transfer_works() {
        let mut contract = MyContract::new(100);
        assert_eq!(contract.transfer(20), true);
        assert_eq!(contract.get(), 80);
    }
}
```

**Running Contract Tests:**
```bash
cargo +nightly test
cargo contract test
```

---

## End-to-End Testing

### Playwright E2E Tests

```typescript
// e2e/wallet.spec.ts
import { test, expect } from '@playwright/test';

test.describe('Wallet E2E Tests', () => {
  test('should create new account', async ({ page }) => {
    await page.goto('https://wallet.etrid.org');

    await page.click('text=Create Account');

    // Verify recovery phrase displayed
    const phrase = await page.locator('.recovery-phrase');
    await expect(phrase).toBeVisible();

    // Save and continue
    await page.click('text=I have saved my recovery phrase');
    await page.click('text=Continue');

    // Set password
    await page.fill('[name="password"]', 'TestPassword123!');
    await page.fill('[name="confirmPassword"]', 'TestPassword123!');
    await page.click('text=Create Account');

    // Verify account created
    await expect(page.locator('.account-address')).toBeVisible();
  });

  test('should send transaction', async ({ page }) => {
    // ... test implementation
  });
});
```

**Running E2E Tests:**
```bash
npx playwright test
npx playwright test --headed
npx playwright test --debug
```

---

## Performance Testing

### Benchmark Tests

```rust
#[bench]
fn bench_transfer(b: &mut Bencher) {
    new_test_ext().execute_with(|| {
        let alice = 1u64;
        let bob = 2u64;

        b.iter(|| {
            Balances::transfer(Origin::signed(alice), bob, 100).unwrap();
        });
    });
}
```

**Running Benchmarks:**
```bash
cargo bench
cargo test --release -- --bench
```

### Load Testing

```typescript
// loadtest/transactions.js
import { check } from 'k6';
import { ApiPromise, WsProvider } from '@polkadot/api';

export default async function() {
  const provider = new WsProvider('wss://testnet-rpc.etrid.org');
  const api = await ApiPromise.create({ provider });

  // Send 1000 transactions
  for (let i = 0; i < 1000; i++) {
    const tx = api.tx.balances.transfer(recipient, amount);
    await tx.signAndSend(sender);
  }
}
```

**Running Load Tests:**
```bash
k6 run loadtest/transactions.js
```

---

## Test Automation

### CI/CD Pipeline

```yaml
# .github/workflows/test.yml
name: Tests

on: [push, pull_request]

jobs:
  test:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3

      - name: Install Rust
        uses: actions-rs/toolchain@v1

      - name: Run unit tests
        run: cargo test

      - name: Run integration tests
        run: npm test

      - name: Run E2E tests
        run: npx playwright test

      - name: Upload coverage
        uses: codecov/codecov-action@v3
```

---

## Best Practices

### Test Structure

✅ **Follow AAA Pattern:**
- **Arrange:** Set up test conditions
- **Act:** Execute the code being tested
- **Assert:** Verify the results

✅ **Write Descriptive Test Names:**
```rust
// Good
#[test]
fn transfer_with_insufficient_balance_should_fail()

// Bad
#[test]
fn test1()
```

✅ **Test Edge Cases:**
- Zero values
- Maximum values
- Boundary conditions
- Error scenarios

✅ **Keep Tests Independent:**
- No shared state
- Each test should run in isolation
- Use setup/teardown properly

### Code Coverage

**Target:** Minimum 80% code coverage

```bash
# Generate coverage report
cargo tarpaulin --out Html --output-dir coverage

# View report
open coverage/index.html
```

---

## Common Testing Pitfalls

### ❌ Avoid

1. **Flaky Tests**
   - Tests that pass/fail randomly
   - Usually due to timing issues
   - Fix: Use proper async/await, increase timeouts

2. **Testing Implementation Instead of Behavior**
   - Test what code does, not how it does it
   - Tests should survive refactoring

3. **Overly Complex Tests**
   - Keep tests simple and focused
   - One assertion per test (when possible)

4. **Ignoring Failed Tests**
   - Fix immediately, don't accumulate
   - Broken tests = broken trust

5. **No Negative Tests**
   - Test error conditions
   - Verify failures fail correctly

---

## Testing Checklist

Before deployment, ensure:

- [ ] All unit tests pass
- [ ] Integration tests pass
- [ ] E2E tests pass
- [ ] Code coverage > 80%
- [ ] Performance benchmarks acceptable
- [ ] Security tests pass
- [ ] Tested on testnet
- [ ] Load testing completed
- [ ] Documentation updated

---

## Resources

**Tools:**
- [Rust Testing](https://doc.rust-lang.org/book/ch11-00-testing.html)
- [Polkadot.js API](https://polkadot.js.org/docs/)
- [Playwright](https://playwright.dev)
- [k6 Load Testing](https://k6.io)

**Documentation:**
- [Developer Guide](DEVELOPER_GUIDE.md)
- [API Reference](API_REFERENCE.md)
- [Smart Contract Development](DEVELOPER_GUIDE.md#smart-contracts)

**Support:**
- 💬 [Discord #dev-testing](https://discord.gg/etrid)
- 📧 Email: dev-support@etrid.org
