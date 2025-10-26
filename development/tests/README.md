# EDSC Bridge Testing Suite

Comprehensive integration and end-to-end tests for the Ëtrid Dollar Stablecoin (EDSC) cross-chain bridge.

## Overview

This testing suite validates the complete bridge functionality:

- **Integration Tests**: Test individual components and flows
  - Ethereum → Ëtrid transfers
  - Ëtrid → Ethereum transfers
  - Attestation service functionality
  - Relayer service functionality

- **E2E Tests**: Test complete user journeys
  - Round-trip transfers
  - High-value transfers
  - Concurrent transfers
  - Bridge health monitoring

## Prerequisites

### Required Software

- **Node.js 18+** and npm
- **Rust & Cargo** (latest stable)
- **Git**
- Unix-like environment (macOS, Linux, WSL)

### System Requirements

- 8GB+ RAM
- 20GB+ free disk space
- Ports 8545, 9944, 3000, 3001 available

## Quick Start

### 1. Install Dependencies

```bash
cd tests
npm install
```

### 2. Setup Local Testnet

This script will:
- Start Hardhat local Ethereum network
- Deploy all Ethereum contracts
- Start EDSC-PBC Substrate node
- Start attestation service
- Start relayer service
- Configure environment variables

```bash
./setup-local-testnet.sh
```

**Note**: First run may take 15-30 minutes to build Substrate node.

### 3. Run Tests

```bash
# Run all tests
npm test

# Run specific test suite
npm run test:integration
npm run test:e2e

# Run with coverage
npm run test:coverage

# Watch mode (for development)
npm run test:watch
```

### 4. Teardown

```bash
./teardown-testnet.sh
```

## Test Suites

### Integration Tests

#### Ethereum → Ëtrid (`ethereum-to-etrid.test.ts`)

Tests transfers from Ethereum to Ëtrid:

```bash
npm test ethereum-to-etrid
```

**Test Cases:**
- ✅ Transfer 100 EDSC from Ethereum to Ëtrid
- ✅ Should not relay duplicate messages
- ✅ Should handle multiple concurrent transfers

**Flow:**
1. User burns EDSC on Ethereum
2. Attestation service detects event
3. Attesters sign message (M-of-N threshold)
4. Relayer submits to Ëtrid
5. User receives EDSC on Ëtrid

#### Ëtrid → Ethereum (`etrid-to-ethereum.test.ts`)

Tests transfers from Ëtrid to Ethereum:

```bash
npm test etrid-to-ethereum
```

**Test Cases:**
- ✅ Transfer 100 EDSC from Ëtrid to Ethereum
- ✅ Should reject invalid signatures
- ✅ Should handle round-trip transfers
- ✅ Should track nonces sequentially

**Flow:**
1. User burns EDSC on Ëtrid
2. Attestation service detects event
3. Attesters sign message
4. Relayer submits to Ethereum
5. User receives EDSC on Ethereum

### End-to-End Tests

#### Complete Bridge Flow (`full-bridge-flow.test.ts`)

Simulates a complete user journey:

```bash
npm test full-bridge-flow
```

**Test Scenario:**
```
User starts with 1000 EDSC on Ethereum
  ↓ Transfer 400 EDSC
Ëtrid (use EDSC in DeFi, payments, etc.)
  ↓ Transfer 150 EDSC back
Ethereum (final balance: 750 EDSC)
Net on Ëtrid: 250 EDSC
```

**Test Cases:**
- ✅ Complete user journey: Eth → Ëtrid → Eth
- ✅ Bridge handles high-value transfers (10,000 EDSC)
- ✅ Bridge statistics and health monitoring

## Test Architecture

### Directory Structure

```
tests/
├── integration/              # Integration tests
│   ├── ethereum-to-etrid.test.ts
│   └── etrid-to-ethereum.test.ts
├── e2e/                      # End-to-end tests
│   └── full-bridge-flow.test.ts
├── utils/                    # Test utilities
│   ├── ethereum-helper.ts    # Ethereum interaction helpers
│   ├── substrate-helper.ts   # Substrate interaction helpers
│   └── service-helper.ts     # Service interaction helpers
├── fixtures/                 # Test data
├── logs/                     # Service logs
├── setup-local-testnet.sh    # Setup script
├── teardown-testnet.sh       # Teardown script
├── package.json
├── jest.config.js
└── README.md
```

### Test Helpers

#### EthereumHelper (`utils/ethereum-helper.ts`)

Utilities for interacting with Ethereum:

```typescript
const ethereumHelper = new EthereumHelper(rpcUrl, privateKey);
await ethereumHelper.connectContracts(addresses);

// Get balance
const balance = await ethereumHelper.getBalance(address);

// Burn and send
const { nonce, txHash } = await ethereumHelper.burnAndSend(
  recipientAddress,
  amount
);

// Check if message received
const isReceived = await ethereumHelper.isMessageReceived(messageHash);
```

#### SubstrateHelper (`utils/substrate-helper.ts`)

Utilities for interacting with Substrate:

```typescript
const substrateHelper = new SubstrateHelper(wsUrl, accountUri);
await substrateHelper.connect();

// Get balance
const balance = await substrateHelper.getBalance(address);

// Burn and send
const { nonce, blockHash } = await substrateHelper.burnAndSend(
  recipientAddress,
  amount,
  destinationDomain
);

// Check if message received
const isReceived = await substrateHelper.isMessageReceived(messageHash);
```

#### AttestationServiceHelper (`utils/service-helper.ts`)

Utilities for interacting with attestation service:

```typescript
const attestationService = new AttestationServiceHelper(serviceUrl);

// Wait for service
await attestationService.waitForHealthy(30000);

// Get attestation
const attestation = await attestationService.getAttestation(messageHash);

// Wait for attestation to be ready
const attestation = await attestationService.waitForAttestationByNonce(
  sourceDomain,
  nonce,
  timeout
);
```

## Configuration

### Environment Variables

Copy `.env.example` to `.env` and configure:

```bash
# Chain connections
ETHEREUM_RPC_URL=http://localhost:8545
SUBSTRATE_WS_URL=ws://localhost:9944
ATTESTATION_SERVICE_URL=http://localhost:3000

# Test account
TEST_PRIVATE_KEY=0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80

# Contract addresses (auto-populated by setup script)
EDSC_ADDRESS=
ATTESTER_REGISTRY_ADDRESS=
MESSAGE_TRANSMITTER_ADDRESS=
TOKEN_MESSENGER_ADDRESS=

# Test configuration
TEST_TIMEOUT=240000
DEBUG=false
```

### Jest Configuration

Test settings in `jest.config.js`:

- **Timeout**: 120 seconds (for slow blockchain operations)
- **Environment**: Node.js
- **Coverage**: Text, LCOV, HTML reports

## Manual Testing

### Setup Each Component Manually

If you prefer manual setup for debugging:

#### 1. Start Ethereum (Hardhat)

```bash
cd ../contracts/ethereum
npx hardhat node
```

#### 2. Deploy Contracts

```bash
cd ../contracts/ethereum
npx hardhat run scripts/deploy.js --network localhost
```

#### 3. Start Substrate

```bash
cd ..
./target/release/edsc-pbc-node --dev --tmp
```

#### 4. Start Attestation Service

```bash
cd services/attestation-service
npm start
```

#### 5. Start Relayer Service

```bash
cd services/relayer-service
npm start
```

### Monitoring

#### View Logs

```bash
# Hardhat
tail -f tests/logs/hardhat.log

# Substrate
tail -f tests/logs/substrate.log

# Attestation Service
tail -f tests/logs/attestation.log

# Relayer Service
tail -f tests/logs/relayer.log
```

#### Check Service Health

```bash
# Attestation service
curl http://localhost:3000/health | jq

# Get ready attestations
curl http://localhost:3000/attestations/ready | jq

# Get statistics
curl http://localhost:3000/stats | jq
```

#### Check Blockchain State

```bash
# Ethereum block number
curl -X POST http://localhost:8545 \
  -H "Content-Type: application/json" \
  -d '{"jsonrpc":"2.0","method":"eth_blockNumber","params":[],"id":1}' | jq

# Substrate block number
curl -X POST ws://localhost:9944 \
  -H "Content-Type: application/json" \
  -d '{"jsonrpc":"2.0","method":"chain_getHeader","params":[],"id":1}' | jq
```

## Troubleshooting

### Tests Fail to Connect

**Problem**: Cannot connect to Ethereum or Substrate

**Solutions**:
1. Check services are running: `lsof -i :8545` and `lsof -i :9944`
2. Check logs for errors
3. Restart services with `./teardown-testnet.sh` and `./setup-local-testnet.sh`

### Timeouts

**Problem**: Tests timeout waiting for attestations or relays

**Solutions**:
1. Increase timeout in test files (default 240s)
2. Check attestation service is processing events
3. Check relayer service is polling and submitting
4. View logs for errors

### Contract Not Deployed

**Problem**: `EDSC_ADDRESS not configured`

**Solutions**:
1. Run `./setup-local-testnet.sh` which auto-deploys
2. Or manually deploy and update `.env`:
   ```bash
   cd ../contracts/ethereum
   npx hardhat run scripts/deploy.js --network localhost
   ```
3. Copy addresses to `tests/.env`

### Port Already in Use

**Problem**: `EADDRINUSE` error

**Solutions**:
```bash
# Kill processes on ports
lsof -ti:8545 | xargs kill
lsof -ti:9944 | xargs kill
lsof -ti:3000 | xargs kill

# Or use teardown script
./teardown-testnet.sh
```

### Substrate Build Fails

**Problem**: Cargo build errors

**Solutions**:
1. Update Rust: `rustup update stable`
2. Clean build: `cargo clean`
3. Check dependencies in `Cargo.toml`
4. View full error in logs

### Low Balance Errors

**Problem**: Insufficient balance for transfers

**Solutions**:
1. Tests auto-mint tokens when needed
2. Check minting permissions
3. Verify token contract is deployed correctly

## Writing New Tests

### Example Test Structure

```typescript
import { EthereumHelper } from '../utils/ethereum-helper';
import { SubstrateHelper } from '../utils/substrate-helper';
import { AttestationServiceHelper } from '../utils/service-helper';

describe('My Test Suite', () => {
  let ethereumHelper: EthereumHelper;
  let substrateHelper: SubstrateHelper;
  let attestationService: AttestationServiceHelper;

  beforeAll(async () => {
    // Setup
    ethereumHelper = new EthereumHelper(RPC_URL, PRIVATE_KEY);
    await ethereumHelper.connectContracts(ADDRESSES);

    substrateHelper = new SubstrateHelper(WS_URL);
    await substrateHelper.connect();

    attestationService = new AttestationServiceHelper(SERVICE_URL);
    await attestationService.waitForHealthy();
  }, 60000);

  afterAll(async () => {
    // Cleanup
    ethereumHelper.disconnect();
    await substrateHelper.disconnect();
  });

  test('My test case', async () => {
    // Test logic
  }, 240000);
});
```

### Best Practices

1. **Use helpers**: Don't interact with chains directly
2. **Set timeouts**: Blockchain operations are slow
3. **Clean up**: Disconnect in `afterAll`
4. **Check balances**: Verify before and after
5. **Wait for confirmations**: Use `waitFor` utilities
6. **Log progress**: Help debugging with console.log
7. **Handle errors**: Expect and test error cases

## CI/CD Integration

### GitHub Actions Example

```yaml
name: Bridge Tests

on: [push, pull_request]

jobs:
  test:
    runs-on: ubuntu-latest

    steps:
      - uses: actions/checkout@v3

      - name: Setup Node.js
        uses: actions/setup-node@v3
        with:
          node-version: '18'

      - name: Setup Rust
        uses: actions-rs/toolchain@v1
        with:
          toolchain: stable

      - name: Install dependencies
        run: |
          cd tests
          npm install

      - name: Setup testnet
        run: |
          cd tests
          ./setup-local-testnet.sh

      - name: Run tests
        run: |
          cd tests
          npm test

      - name: Upload coverage
        uses: codecov/codecov-action@v3
        with:
          files: ./tests/coverage/lcov.info
```

## Performance Benchmarks

Expected test times on modern hardware:

- **Setup**: 5-10 minutes (first time: 20-30 min for Substrate build)
- **Single transfer test**: 30-60 seconds
- **Integration suite**: 5-10 minutes
- **Full E2E suite**: 10-15 minutes
- **All tests**: 15-20 minutes

## Support

For issues and questions:
- GitHub Issues: [etrid/etrid](https://github.com/etrid/etrid/issues)
- Documentation: [docs.etrid.io](https://docs.etrid.io)
- Discord: [discord.gg/etrid](https://discord.gg/etrid)

## License

Apache-2.0
