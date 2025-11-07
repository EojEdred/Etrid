# XCM Integration Guide: ETH-PBC ↔ FlareChain

## Overview

This guide explains how to complete the XCM integration between ETH-PBC and FlareChain, enabling custom precompiles to query FlareChain services in production.

---

## Current Status

| Component | Status | Notes |
|-----------|--------|-------|
| **Precompile Implementations** | ✅ Complete | Oracle, Governance, Staking |
| **Mock XCM Bridge** | ✅ Complete | Development/testing |
| **FlareChain Query Handler** | ✅ Complete | Pallet ready |
| **Production XCM Bridge** | ⏸️ Stub Created | Needs runtime integration |
| **HRMP Channels** | ⏸️ Not Set Up | Requires relay chain |
| **Async Callback Mechanism** | ⏸️ Design Complete | Needs implementation |

---

## Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                     ETH-PBC                                  │
│                                                              │
│  Solidity Contract                                           │
│         ↓                                                    │
│  IEtridOracle.getPriceInETH("BTC")                          │
│         ↓                                                    │
│  Oracle Precompile (0x800)                                   │
│         ↓                                                    │
│  Production XCM Bridge                                       │
│         ↓                                                    │
│  pallet_xcm::send_xcm(FlareChain, Query)                   │
│         ↓                                                    │
└─────────┼────────────────────────────────────────────────────┘
          │
          │ XCM Message (via HRMP)
          │
┌─────────▼────────────────────────────────────────────────────┐
│                     FLARECHAIN                                │
│                                                              │
│  pallet_xcm receives message                                 │
│         ↓                                                    │
│  pallet_xcm_query_handler::handle_query(OraclePrice)        │
│         ↓                                                    │
│  pallet_oracle_network::get_price("BTC")                    │
│         ↓                                                    │
│  Response: 50000000000000000000000 (50k * 1e18)             │
│         ↓                                                    │
│  pallet_xcm::send_xcm(ETH-PBC, Response)                    │
│         ↓                                                    │
└─────────┼────────────────────────────────────────────────────┘
          │
          │ XCM Response (via HRMP)
          │
┌─────────▼────────────────────────────────────────────────────┐
│                     ETH-PBC                                  │
│                                                              │
│  pallet_xcm receives response                                │
│         ↓                                                    │
│  Update response cache                                       │
│         ↓                                                    │
│  Oracle Precompile returns cached value                      │
│         ↓                                                    │
│  Solidity contract receives: 16670000000000000000 (16.67 ETH)│
└──────────────────────────────────────────────────────────────┘
```

---

## Implementation Steps

### Phase 1: Prerequisites ✅

#### 1.1. Ensure Both Chains are Parachains
```bash
# Check if ETH-PBC and FlareChain are registered as parachains
polkadot-js-api query.paras.parachains

# Expected output:
# [2000, 2001, ...] # 2000=FlareChain, 2001=ETH-PBC (example IDs)
```

#### 1.2. Verify XCM Version Compatibility
- FlareChain: XCM v3
- ETH-PBC: XCM v3
- Both should use same XCM version

---

### Phase 2: HRMP Channel Setup ⏸️

HRMP (Horizontal Relay-routed Message Passing) enables parachains to communicate.

#### 2.1. Open Channel: ETH-PBC → FlareChain

```bash
# From ETH-PBC, request to open channel to FlareChain
polkadot-js-api tx.hrmp.hrmpInitOpenChannel \
    --recipient 2000 \          # FlareChain para ID
    --proposedMaxCapacity 1000 \
    --proposedMaxMessageSize 10240

# FlareChain must accept
polkadot-js-api tx.hrmp.hrmpAcceptOpenChannel \
    --sender 2001               # ETH-PBC para ID
```

#### 2.2. Open Channel: FlareChain → ETH-PBC

```bash
# From FlareChain, request to open channel to ETH-PBC
polkadot-js-api tx.hrmp.hrmpInitOpenChannel \
    --recipient 2001 \          # ETH-PBC para ID
    --proposedMaxCapacity 1000 \
    --proposedMaxMessageSize 10240

# ETH-PBC must accept
polkadot-js-api tx.hrmp.hrmpAcceptOpenChannel \
    --sender 2000               # FlareChain para ID
```

#### 2.3. Verify Channels are Open

```bash
# Check HRMP channels
polkadot-js-api query.hrmp.hrmpChannels 2001 2000  # ETH-PBC -> FlareChain
polkadot-js-api query.hrmp.hrmpChannels 2000 2001  # FlareChain -> ETH-PBC

# Both should return channel configuration
```

---

### Phase 3: Runtime Configuration ⏸️

#### 3.1. Configure ETH-PBC Runtime

Add to `/eth-pbc/runtime/Cargo.toml`:
```toml
# XCM dependencies
xcm = { package = "staging-xcm", git = "https://github.com/paritytech/polkadot-sdk", tag = "polkadot-stable2506", default-features = false }
pallet-xcm = { git = "https://github.com/paritytech/polkadot-sdk", tag = "polkadot-stable2506", default-features = false }
cumulus-pallet-xcm = { git = "https://github.com/paritytech/polkadot-sdk", tag = "polkadot-stable2506", default-features = false }
cumulus-primitives-core = { git = "https://github.com/paritytech/polkadot-sdk", tag = "polkadot-stable2506", default-features = false }
```

Add to `/eth-pbc/runtime/src/lib.rs`:
```rust
// XCM Configuration
parameter_types! {
    pub const RelayLocation: MultiLocation = MultiLocation::parent();
    pub const FlareChainLocation: MultiLocation = MultiLocation::new(1, X1(Parachain(2000)));
}

impl pallet_xcm::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type SendXcmOrigin = EnsureXcmOrigin<RuntimeOrigin, ()>;
    type XcmRouter = XcmRouter;
    // ... other config
}

impl cumulus_pallet_xcm::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type XcmExecutor = XcmExecutor<XcmConfig>;
}
```

#### 3.2. Configure FlareChain Runtime

Add to `/flare-chain/runtime/Cargo.toml`:
```toml
# XCM Query Handler
pallet-xcm-query-handler = { path = "../pallets/pallet-xcm-query-handler", default-features = false }
```

Add to `/flare-chain/runtime/src/lib.rs`:
```rust
// XCM Query Handler Configuration
impl pallet_xcm_query_handler::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type Oracle = pallet_oracle_network::Pallet<Runtime>;
    type Governance = pallet_governance::Pallet<Runtime>;
    type Staking = pallet_etrid_staking::Pallet<Runtime>;
}

construct_runtime!(
    pub struct Runtime {
        // ... existing pallets
        XcmQueryHandler: pallet_xcm_query_handler,
    }
);
```

---

### Phase 4: Production XCM Bridge Implementation ⏸️

#### 4.1. Replace Mock with Production

In `/eth-pbc/runtime/src/precompiles.rs`:
```rust
// Change from:
pub type FrontierPrecompiles<R> = EtridPrecompiles<R, MockXcmBridge>;

// To:
pub type FrontierPrecompiles<R> = EtridPrecompiles<R, ProductionXcmBridge>;
```

#### 4.2. Implement XCM Sending

Update `/eth-pbc/runtime/src/precompiles/xcm_bridge_production.rs`:
```rust
impl XcmBridge for ProductionXcmBridge {
    fn query_flarechain(query: FlareChainQuery) -> Result<FlareChainResponse, Vec<u8>> {
        // 1. Generate unique query ID
        let query_id = Self::next_query_id();

        // 2. Construct XCM message
        let xcm_message = Xcm(vec![
            WithdrawAsset((Here, 1_000_000_000u128).into()),
            BuyExecution {
                fees: (Here, 1_000_000_000u128).into(),
                weight_limit: WeightLimit::Unlimited,
            },
            Transact {
                origin_kind: OriginKind::Native,
                require_weight_at_most: Weight::from_parts(1_000_000_000, 64 * 1024),
                call: query.encode().into(),
            },
        ]);

        // 3. Send to FlareChain
        pallet_xcm::Pallet::<Runtime>::send_xcm(
            Here,
            FlareChainLocation::get(),
            xcm_message
        ).map_err(|_| b"XCM send failed".to_vec())?;

        // 4. Store pending query
        PendingQueries::insert(query_id, PendingQuery {
            query_id,
            query,
            sent_at_block: frame_system::Pallet::<Runtime>::block_number(),
            timeout_at_block: frame_system::Pallet::<Runtime>::block_number() + 100,
        });

        // 5. Wait for response (or return cached if view function)
        Self::wait_for_response(query_id)
    }
}
```

---

### Phase 5: Async Callback Mechanism ⏸️

#### 5.1. Create Response Handler Pallet (ETH-PBC)

```rust
#[pallet::pallet]
pub struct Pallet<T>(_);

#[pallet::storage]
pub type PendingQueries<T: Config> =
    StorageMap<_, Blake2_128Concat, u64, PendingQuery>;

#[pallet::storage]
pub type ResponseCache<T: Config> =
    StorageMap<_, Blake2_128Concat, Vec<u8>, CachedResponse>;

#[pallet::hooks]
impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
    fn on_initialize(n: BlockNumberFor<T>) -> Weight {
        // Clean up expired queries and cached responses
        Self::cleanup_expired(n)
    }
}

impl<T: Config> Pallet<T> {
    /// Called when XCM response arrives
    pub fn handle_response(query_id: u64, response: FlareChainResponse) {
        // 1. Match with pending query
        if let Some(pending) = PendingQueries::<T>::take(query_id) {
            // 2. Cache response
            Self::cache_response(pending.query, response.clone());

            // 3. Emit event (contracts can listen)
            Self::deposit_event(Event::ResponseReceived { query_id, response });
        }
    }

    /// Cache response for view functions
    fn cache_response(query: FlareChainQuery, response: FlareChainResponse) {
        let cache_key = query.encode();
        let cached = CachedResponse {
            response,
            cached_at_block: frame_system::Pallet::<T>::block_number(),
            expires_at_block: frame_system::Pallet::<T>::block_number() + 1000,
        };
        ResponseCache::<T>::insert(cache_key, cached);
    }
}
```

#### 5.2. XCM Executor Configuration

```rust
pub struct XcmConfig;
impl xcm_executor::Config for XcmConfig {
    // ... other config

    type CallDispatcher = RuntimeCall;

    // Handle incoming Transact with response data
    fn should_execute<Call>(
        origin: &MultiLocation,
        message: &mut Xcm<Call>,
        max_weight: Weight,
        weight_credit: &mut Weight,
    ) -> Result<(), ProcessMessageError> {
        // Parse response from Transact call
        if let Some(Transact { call, .. }) = message.inner().get(0) {
            if let Ok(response) = FlareChainResponse::decode(&mut &call[..]) {
                // Extract query_id from origin or message
                let query_id = Self::extract_query_id(origin, message);
                pallet_xcm_response_handler::Pallet::<Runtime>::handle_response(
                    query_id,
                    response
                );
            }
        }
        Ok(())
    }
}
```

---

### Phase 6: Testing ⏸️

#### 6.1. Unit Tests

```rust
#[test]
fn test_xcm_query_send() {
    new_test_ext().execute_with(|| {
        let query = FlareChainQuery::OraclePrice {
            symbol: *b"BTC\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            quote_currency: *b"ETH\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
        };

        let result = ProductionXcmBridge::query_flarechain(query);
        assert!(result.is_ok());
    });
}
```

#### 6.2. Integration Tests

```javascript
// Test with Zombienet (local parachain network)
describe("XCM Precompile Integration", () => {
    it("Should query FlareChain oracle via XCM", async () => {
        // 1. Deploy contract on ETH-PBC
        const oracle = await ethers.getContractAt("IEtridOracle", "0x800");

        // 2. Query price (triggers XCM)
        const price = await oracle.getPriceInETH(
            ethers.utils.formatBytes32String("BTC")
        );

        // 3. Wait for XCM response
        await new Promise(r => setTimeout(r, 12000)); // 2 blocks

        // 4. Verify price was updated
        expect(price).to.be.gt(0);
    });
});
```

#### 6.3. Zombienet Configuration

```toml
# zombienet.toml
[relaychain]
default_command = "polkadot"
chain = "rococo-local"

  [[relaychain.nodes]]
  name = "alice"
  validator = true

  [[relaychain.nodes]]
  name = "bob"
  validator = true

[[parachains]]
id = 2000
chain = "flarechain-local"

  [[parachains.collators]]
  name = "flarechain-collator"
  command = "flarechain-node"

[[parachains]]
id = 2001
chain = "eth-pbc-local"

  [[parachains.collators]]
  name = "eth-pbc-collator"
  command = "eth-pbc-node"
```

Run test network:
```bash
zombienet spawn zombienet.toml
```

---

## Gas Costs

### Mock Mode (Current)
- **View functions**: ~2,100 gas
- **Write functions**: ~21,000 gas

### Production XCM Mode (After Integration)
- **View functions (cached)**: ~5,000 gas (read from cache)
- **View functions (uncached)**: ~150,000 gas (XCM round-trip)
- **Write functions**: ~250,000 gas (XCM submission + confirmation)

---

## Deployment Checklist

- [ ] Deploy FlareChain with pallet_xcm_query_handler
- [ ] Deploy ETH-PBC with production XCM bridge
- [ ] Set up HRMP channels (both directions)
- [ ] Verify XCM messages can be sent
- [ ] Test oracle price queries end-to-end
- [ ] Test governance proposal submission
- [ ] Test staking queries
- [ ] Monitor gas costs and optimize
- [ ] Security audit XCM integration
- [ ] Deploy to testnet
- [ ] Deploy to mainnet

---

## Troubleshooting

### "XCM send failed"
- **Cause**: HRMP channel not open or insufficient funds
- **Fix**: Verify channel is open with `query.hrmp.hrmpChannels`

### "Response timeout"
- **Cause**: FlareChain not responding or XCM executor issue
- **Fix**: Check FlareChain logs, verify pallet_xcm_query_handler is working

### "Invalid query format"
- **Cause**: Query encoding mismatch
- **Fix**: Ensure both chains use same FlareChainQuery/Response types

### "Duplicate sp_io lang item"
- **Cause**: Version mismatch between stable2506 (ETH-PBC) and stable2509 (FlareChain)
- **Fix**: Align both to same Polkadot SDK version

---

## Performance Optimization

### 1. Cache Aggressively
- Cache oracle prices for 10 blocks (~1 minute)
- Cache validator data for 100 blocks (~10 minutes)
- Cache governance status for 10 blocks

### 2. Batch Queries
- Group multiple oracle queries into single XCM message
- Reduces XCM overhead

### 3. Prefetch
- Have FlareChain push updates to ETH-PBC proactively
- ETH-PBC caches updates, precompiles read from cache
- Eliminates query latency

---

## Security Considerations

1. **Origin Verification**: Ensure XCM messages are from trusted parachains
2. **Rate Limiting**: Limit XCM queries to prevent DoS
3. **Weight Limits**: Set appropriate XCM weight limits
4. **Timeout Handling**: Clean up expired queries
5. **Replay Protection**: Use unique query IDs
6. **Authorization**: Verify caller has permission for governance operations

---

## References

- [XCM Format Specification](https://github.com/paritytech/xcm-format)
- [HRMP Documentation](https://wiki.polkadot.network/docs/learn-xcm-transport#hrmp)
- [Zombienet Testing](https://github.com/paritytech/zombienet)
- [Custom Precompiles Guide](./CUSTOM_PRECOMPILES_GUIDE.md)

---

**Document Version**: 1.0
**Last Updated**: 2025-11-05
**Status**: ⏸️ Implementation Guide (Production XCM Pending)
