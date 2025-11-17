# Integration Tests

Integration tests verify end-to-end functionality with a real Ëtrid node.

## Prerequisites

1. **Running Ëtrid Node**:
   ```bash
   ./target/release/flarechain-node --dev --tmp
   ```

2. **Test Accounts with Funds**:
   - Alice (//Alice) - Pre-funded in dev mode
   - Bob (//Bob) - Pre-funded in dev mode

## Running Tests

```bash
# All integration tests
npm run test:integration

# Specific test file
npm run test:integration -- lightning-bloc.integration.test.ts

# With coverage
npm run test:integration -- --coverage
```

## Test Structure

Each integration test should:
1. Connect to a real node
2. Use test accounts (Alice, Bob, etc.)
3. Perform actual transactions
4. Verify on-chain state
5. Clean up resources

## Example Integration Test

```typescript
import { EtridClient, LightningBlocWrapper } from '../../src';
import { Keyring } from '@polkadot/keyring';

describe('Lightning-Bloc Integration', () => {
  let client: EtridClient;
  let lightning: LightningBlocWrapper;
  let alice, bob;
  
  beforeAll(async () => {
    client = new EtridClient('ws://127.0.0.1:9944');
    lightning = new LightningBlocWrapper(client.api);
    
    const keyring = new Keyring({ type: 'sr25519' });
    alice = keyring.addFromUri('//Alice');
    bob = keyring.addFromUri('//Bob');
    
    // Wait for connection
    await client.api.isReady;
  });
  
  afterAll(() => {
    client.close();
  });
  
  it('should open, use, and close channel', async () => {
    // Open channel
    const channel = await lightning.openChannel(
      alice,
      bob.address,
      1000n * 10n**18n
    );
    
    expect(channel.channelId).toBeDefined();
    
    // Verify channel exists
    const info = await lightning.getChannel(channel.channelId);
    expect(info).not.toBeNull();
    expect(info.from).toBe(alice.address);
    expect(info.to).toBe(bob.address);
    
    // Send payment
    const payment = await lightning.sendPayment(
      alice,
      channel.channelId,
      100n * 10n**18n
    );
    
    expect(payment.txHash).toBeDefined();
    
    // Close channel
    const closeHash = await lightning.closeChannel(alice, channel.channelId);
    expect(closeHash).toBeDefined();
  }, 30000); // 30 second timeout
});
```

## Test Coverage Goals

- Lightning-Bloc: Full channel lifecycle
- Distribution Pay: Claim flow
- ETWASM VM: Deploy and call contract
- AI DID: Register and manage AI
- Bridge: Cross-chain transfer (mocked)
- Oracle: Price feed queries
- Reserve Vault: Create, borrow, repay
- Staking: Bond, nominate, unbond
- Governance: Propose, vote, execute

## Best Practices

1. **Use realistic timeouts**: Blockchain operations take time
2. **Clean up**: Close channels, vaults, etc.
3. **Check balances**: Verify funds before/after
4. **Handle errors**: Test failure cases too
5. **Isolate tests**: Each test should be independent

## CI/CD Integration

GitHub Actions workflow for integration tests:

```yaml
name: Integration Tests

on: [push, pull_request]

jobs:
  integration:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      
      - name: Setup Node
        uses: actions/setup-node@v3
        with:
          node-version: '18'
      
      - name: Install dependencies
        run: npm ci
      
      - name: Build Ëtrid node
        run: cargo build --release
        
      - name: Start node
        run: |
          ./target/release/flarechain-node --dev --tmp &
          sleep 10
          
      - name: Run integration tests
        run: npm run test:integration
```

## Troubleshooting

### "Connection refused"

```bash
# Make sure node is running
ps aux | grep flarechain-node

# Start node
./target/release/flarechain-node --dev --tmp
```

### "Insufficient balance"

```bash
# Use pre-funded test accounts
# Alice and Bob have funds in dev mode
```

### "Timeout"

```bash
# Increase Jest timeout
jest.setTimeout(60000);  // 60 seconds
```
