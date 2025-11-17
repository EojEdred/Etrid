# Ëtrid SDK Test Plan

Comprehensive testing strategy for JavaScript/TypeScript SDK.

## Test Coverage Goals

- **Unit Tests**: 80%+ coverage
- **Integration Tests**: All critical paths
- **E2E Tests**: Complete workflows
- **Performance Tests**: Benchmarks for key operations

---

## Test Structure

```
tests/
├── unit/                           # Unit tests for each wrapper
│   ├── LightningBlocWrapper.test.ts
│   ├── DistributionPayWrapper.test.ts
│   ├── EtwasmVMWrapper.test.ts
│   ├── AIDidWrapper.test.ts
│   ├── BridgeWrapper.test.ts
│   ├── OracleWrapper.test.ts
│   └── ReserveVaultWrapper.test.ts
├── integration/                    # Integration tests
│   ├── payment-flow.test.ts
│   ├── defi-lending.test.ts
│   └── cross-chain.test.ts
├── mocks/                          # Mock data and fixtures
│   ├── apiMocks.ts
│   ├── eventMocks.ts
│   └── queryMocks.ts
└── utils/                          # Test utilities
    └── testHelpers.ts
```

---

## Unit Test Coverage

### 1. LightningBlocWrapper Tests

**Coverage**: 25 tests

```typescript
describe('LightningBlocWrapper', () => {
  describe('openChannel', () => {
    ✓ Should open channel successfully
    ✓ Should fail with insufficient balance
    ✓ Should emit ChannelOpened event
    ✓ Should validate deposit amounts
    ✓ Should validate duration
  });

  describe('routePayment', () => {
    ✓ Should find route with single hop
    ✓ Should find route with multiple hops
    ✓ Should fail when no route exists
    ✓ Should respect max hops limit
    ✓ Should calculate fees correctly
  });

  describe('closeChannel', () => {
    ✓ Should close cooperatively
    ✓ Should force close with dispute
    ✓ Should emit ChannelClosed event
  });

  describe('getChannel', () => {
    ✓ Should return channel details
    ✓ Should return null for non-existent channel
  });

  describe('estimateRoutingFee', () => {
    ✓ Should calculate base fee
    ✓ Should calculate proportional fee
    ✓ Should return total fee
  });
});
```

### 2. DistributionPayWrapper Tests

**Coverage**: 20 tests

```typescript
describe('DistributionPayWrapper', () => {
  describe('claimReward', () => {
    ✓ Should claim reward successfully
    ✓ Should fail if not eligible
    ✓ Should fail if claimed too early
    ✓ Should emit RewardClaimed event
  });

  describe('getPendingRewards', () => {
    ✓ Should return pending amounts by category
    ✓ Should return total pending
    ✓ Should return zero for ineligible users
  });

  describe('isEligible', () => {
    ✓ Should check voter eligibility
    ✓ Should check staker eligibility
    ✓ Should check validator eligibility
  });

  describe('getEligibleCategories', () => {
    ✓ Should return all eligible categories
    ✓ Should return empty array if none eligible
  });

  describe('estimateNextDistribution', () => {
    ✓ Should calculate estimated amount
    ✓ Should calculate user share
    ✓ Should return next distribution time
  });
});
```

### 3. EtwasmVMWrapper Tests

**Coverage**: 22 tests

```typescript
describe('EtwasmVMWrapper', () => {
  describe('uploadCode', () => {
    ✓ Should upload WASM bytecode
    ✓ Should return code hash
    ✓ Should validate bytecode
    ✓ Should fail with invalid WASM
  });

  describe('instantiate', () => {
    ✓ Should instantiate contract
    ✓ Should return contract address
    ✓ Should pass constructor args
  });

  describe('deployContract', () => {
    ✓ Should deploy in one step
    ✓ Should upload and instantiate
    ✓ Should return deployment result
  });

  describe('callContract', () => {
    ✓ Should call contract method
    ✓ Should handle method parameters
    ✓ Should emit contract events
    ✓ Should fail with insufficient gas
  });

  describe('queryContract', () => {
    ✓ Should query contract state
    ✓ Should not charge gas
    ✓ Should handle query failures
  });

  describe('estimateGas', () => {
    ✓ Should estimate gas usage
    ✓ Should add buffer percentage
    ✓ Should respect max gas limit
  });
});
```

### 4. AIDidWrapper Tests

**Coverage**: 24 tests

```typescript
describe('AIDidWrapper', () => {
  describe('registerAI', () => {
    ✓ Should register AI successfully
    ✓ Should generate unique DID
    ✓ Should validate profile data
    ✓ Should support all AI types
    ✓ Should emit AIRegistered event
  });

  describe('getProfile', () => {
    ✓ Should return AI profile
    ✓ Should return null for non-existent DID
  });

  describe('getReputation', () => {
    ✓ Should return reputation score
    ✓ Should calculate success rate
    ✓ Should track total inferences
  });

  describe('recordInference', () => {
    ✓ Should record successful inference
    ✓ Should record failed inference
    ✓ Should update reputation
  });

  describe('addRating', () => {
    ✓ Should add user rating
    ✓ Should update user rating average
    ✓ Should validate rating range
  });

  describe('grantPermission', () => {
    ✓ Should grant permission
    ✓ Should set conditions
    ✓ Should emit PermissionGranted event
  });

  describe('queryByCapability', () => {
    ✓ Should find matching AIs
    ✓ Should filter by minimum reputation
    ✓ Should rank by match score
  });
});
```

### 5. BridgeWrapper Tests

**Coverage**: 20 tests

```typescript
describe('BridgeWrapper', () => {
  describe('bridgeTokens', () => {
    ✓ Should initiate bridge transfer
    ✓ Should validate source chain
    ✓ Should validate target chain
    ✓ Should check bridge limits
    ✓ Should emit BridgeInitiated event
  });

  describe('getBridgeStatus', () => {
    ✓ Should return transaction status
    ✓ Should track confirmations
    ✓ Should calculate ETA
  });

  describe('estimateBridgeFee', () => {
    ✓ Should calculate base fee
    ✓ Should calculate variable fee
    ✓ Should return total fee
  });

  describe('getBridgeLimits', () => {
    ✓ Should return min/max amounts
    ✓ Should return daily limit
    ✓ Should track daily usage
  });

  describe('getSupportedChains', () => {
    ✓ Should return all 13 chains
  });
});
```

### 6. OracleWrapper Tests

**Coverage**: 18 tests

```typescript
describe('OracleWrapper', () => {
  describe('getPrice', () => {
    ✓ Should return current price
    ✓ Should include confidence score
    ✓ Should track oracle sources
    ✓ Should fail for unsupported pair
  });

  describe('getTWAP', () => {
    ✓ Should calculate 24h TWAP
    ✓ Should calculate custom range
    ✓ Should require minimum data points
    ✓ Should return statistics
  });

  describe('subscribePriceUpdates', () => {
    ✓ Should subscribe to updates
    ✓ Should call callback on price change
    ✓ Should allow unsubscribe
  });

  describe('getPriceSources', () => {
    ✓ Should return oracle sources
    ✓ Should show source health
    ✓ Should show source weights
  });

  describe('getHistoricalPrices', () => {
    ✓ Should return price history
    ✓ Should respect time range
    ✓ Should use specified interval
  });
});
```

### 7. ReserveVaultWrapper Tests

**Coverage**: 23 tests

```typescript
describe('ReserveVaultWrapper', () => {
  describe('depositCollateral', () => {
    ✓ Should deposit collateral
    ✓ Should update vault balance
    ✓ Should support multiple assets
    ✓ Should emit CollateralDeposited event
  });

  describe('withdrawCollateral', () => {
    ✓ Should withdraw collateral
    ✓ Should maintain minimum ratio
    ✓ Should fail if ratio too low
  });

  describe('getVaultBalance', () => {
    ✓ Should return vault state
    ✓ Should calculate collateral ratio
    ✓ Should determine vault status
  });

  describe('borrow', () => {
    ✓ Should borrow against collateral
    ✓ Should respect LTV limits
    ✓ Should update debt
    ✓ Should fail if insufficient collateral
  });

  describe('repay', () => {
    ✓ Should repay debt
    ✓ Should reduce debt amount
    ✓ Should improve collateral ratio
  });

  describe('getSupportedAssets', () => {
    ✓ Should return collateral assets
    ✓ Should show LTV ratios
    ✓ Should show liquidation thresholds
  });

  describe('getCollateralRatio', () => {
    ✓ Should calculate ratio
    ✓ Should determine health status
  });
});
```

---

## Integration Tests

### Payment Flow Test
- Open channel → Route payment → Close channel
- Multi-hop routing across 3+ nodes
- Concurrent payments through same channel

### DeFi Lending Test
- Deposit → Borrow → Price change → Repay → Withdraw
- Liquidation scenario
- Multi-asset collateral

### Cross-Chain Test
- Bridge tokens → Monitor status → Verify receipt
- Multiple simultaneous bridges
- Failed bridge recovery

---

## Test Execution

### Run All Tests
```bash
npm test
```

### Run Specific Suite
```bash
npm test LightningBlocWrapper
```

### Watch Mode
```bash
npm test -- --watch
```

### Coverage Report
```bash
npm test -- --coverage
```

---

## Continuous Integration

Tests run automatically on:
- Every commit to main
- All pull requests
- Scheduled nightly builds

### CI Requirements
- All tests must pass
- Coverage must be ≥ 80%
- No TypeScript errors
- Linting passes

---

## Performance Benchmarks

Track performance of key operations:

```typescript
describe('Performance', () => {
  it('Channel operations < 100ms', async () => {
    const start = Date.now();
    await lightning.openChannel(...);
    expect(Date.now() - start).toBeLessThan(100);
  });

  it('1000 concurrent price queries', async () => {
    const promises = Array(1000).fill(null).map(() =>
      oracle.getPrice('ETR/USD')
    );
    await Promise.all(promises);
  });
});
```

---

## Maintenance

- Review and update tests monthly
- Add tests for all new features
- Remove tests for deprecated features
- Keep mocks updated with chain changes

---

**Test Coverage Target**: 90%+
**Last Updated**: November 16, 2025
