# Node Authorization Implementation Summary

## Overview

This document describes the implementation of peer whitelisting/node authorization for the Ëtrid Primearc Core Chain (Primearc Core Chain). The implementation provides production-grade security by ensuring only authorized validator nodes can connect to the network.

## Architecture

### Components Implemented

1. **pallet-node-authorization**
   - Location: Runtime dependency from polkadot-sdk
   - Purpose: Core node authorization functionality
   - Configuration: `/05-multichain/primearc-core/runtime/src/lib.rs` lines 386-406

2. **node-authorization-helper**
   - Location: `/05-multichain/primearc-core/runtime/src/node_authorization_helper.rs`
   - Purpose: Integration layer between node authorization and validator committee
   - Functions:
     - `account_id_to_peer_id()`: Converts AccountId to PeerId
     - `sync_authorized_nodes_with_validators()`: Syncs authorized nodes with validator set
     - `get_authorized_peer_ids()`: Retrieves current authorized peers
     - `initialize_genesis_authorized_nodes()`: Sets up genesis authorized nodes

3. **Genesis Configuration**
   - Location: Runtime presets (e.g., `development_with_auth.json`)
   - Format: Array of [PeerId, AccountId] tuples
   - Purpose: Initialize whitelisted nodes at genesis

### Runtime Integration

#### Cargo.toml Changes

```toml
# Added dependencies
pallet-node-authorization = { git = "https://github.com/paritytech/polkadot-sdk", tag = "polkadot-stable2509", default-features = false }

# Added to std feature
"pallet-node-authorization/std",
```

#### Runtime Configuration

```rust
impl pallet_node_authorization::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type MaxWellKnownNodes = MaxWellKnownNodes;  // 100 nodes max
    type MaxPeerIdLength = MaxPeerIdLength;      // 128 bytes max
    type AddOrigin = EnsureRoot<AccountId>;      // Only root can add
    type RemoveOrigin = EnsureRoot<AccountId>;   // Only root can remove
    type SwapOrigin = EnsureRoot<AccountId>;     // Only root can swap
    type ResetOrigin = EnsureRoot<AccountId>;    // Only root can reset
    type WeightInfo = ();
}
```

#### construct_runtime! Addition

```rust
NodeAuthorization: pallet_node_authorization,
```

## Security Model

### Access Control

- **Add/Remove Nodes**: Requires `Root` origin (sudo or governance)
- **Genesis Initialization**: Only predefined validators in genesis preset
- **Session Integration**: Can be integrated with session changes for dynamic updates

### Threat Mitigation

| Attack Vector | Mitigation |
|--------------|------------|
| Eclipse Attack | Only whitelisted nodes can connect |
| Sybil Attack | Governance approval required for new nodes |
| Network Partition | Stable bootnode set prevents isolation |
| Unauthorized Access | PeerId validation at connection time |

## Usage

### 1. Genesis Configuration

Add to your genesis preset JSON:

```json
{
  "nodeAuthorization": {
    "nodes": [
      ["12D3KooWEyoppNCUx8Yx66oV9fJnriXwCcXwDDUA2kj6vnc6iDEp", "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY"],
      ["12D3KooWHdiAxVd8uMQR1hGWXccidmfCwLqcMpGwR6QcTP6QRMuD", "5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty"]
    ]
  }
}
```

### 2. Runtime Extrinsics

#### Add Well-Known Node

```rust
// Requires Root origin
nodeAuthorization.addWellKnownNode(peerId, owner)
```

#### Remove Well-Known Node

```rust
// Requires Root origin
nodeAuthorization.removeWellKnownNode(peerId)
```

#### Swap Well-Known Node

```rust
// Requires Root origin
nodeAuthorization.swapWellKnownNode(remove, add)
```

### 3. Node Operation

Start validator with authorized node key:

```bash
./primearc-core-node \
  --validator \
  --node-key-file /path/to/node-key \
  --chain flarechain_mainnet_v1 \
  --bootnodes /ip4/IP/tcp/30333/p2p/PEER_ID
```

## Integration with Validator Committee

### Current Implementation

The `node-authorization-helper` module provides integration hooks:

```rust
// Get authorized peers from current validator set
let authorized_peers = node_authorization_helper::get_authorized_peer_ids::<Runtime>();

// Sync authorization with validator changes
node_authorization_helper::sync_authorized_nodes_with_validators::<Runtime>()?;
```

### Future Enhancement: Automatic Sync

To enable automatic synchronization on session changes:

```rust
// In pallet-validator-committee's SessionManager implementation
impl<T: Config> SessionManager<AccountId> for Pallet<T> {
    fn new_session(new_index: SessionIndex) -> Option<Vec<AccountId>> {
        // ... existing logic ...

        // Sync node authorization with new validator set
        let _ = node_authorization_helper::sync_authorized_nodes_with_validators::<T>();

        Some(validators)
    }
}
```

## Testing

### Development Testing

1. **Local Development** (Alice & Bob):
   ```bash
   # Use development_with_auth.json preset
   ./primearc-core-node build-spec --chain development --raw > dev-auth-spec.json

   # Start Alice (authorized)
   ./primearc-core-node --alice --validator --chain dev-auth-spec.json

   # Start Bob (authorized)
   ./primearc-core-node --bob --validator --chain dev-auth-spec.json --port 30334

   # Try Charlie (unauthorized - should fail to connect)
   ./primearc-core-node --charlie --chain dev-auth-spec.json --port 30335
   ```

2. **Verify Authorization**:
   ```bash
   # Check well-known nodes
   curl -H "Content-Type: application/json" \
        -d '{"id":1, "jsonrpc":"2.0", "method":"state_call", "params":["NodeAuthorizationApi_well_known_nodes", "0x"]}' \
        http://localhost:9944
   ```

### Integration Testing

See `PEER_WHITELISTING_GUIDE.md` for comprehensive testing procedures.

## Production Deployment

### Pre-Deployment Checklist

- [ ] Generate unique node keys for all validators
- [ ] Collect PeerIds and AccountIds for all validators
- [ ] Update genesis preset with nodeAuthorization section
- [ ] Configure bootnodes with authorized validators
- [ ] Test connection authorization in staging environment
- [ ] Set up monitoring for unauthorized connection attempts
- [ ] Prepare governance process for adding/removing nodes
- [ ] Document node key backup procedures

### Deployment Steps

1. **Generate Node Keys**:
   ```bash
   # For each validator
   subkey generate-node-key --file validator-N-node-key
   # Save the output PeerId
   ```

2. **Create Genesis Configuration**:
   ```bash
   # Use the helper script
   ./scripts/generate-node-authorization-genesis.sh
   ```

3. **Build Chain Spec**:
   ```bash
   ./primearc-core-node build-spec \
     --chain flarechain_mainnet_v1 \
     --raw \
     > chainspec-with-auth.json
   ```

4. **Deploy Validators**:
   ```bash
   # Each validator uses their specific node key
   ./primearc-core-node \
     --validator \
     --node-key-file validator-N-node-key \
     --chain chainspec-with-auth.json \
     --bootnodes /ip4/IP1/tcp/30333/p2p/PEER_ID1 \
     --bootnodes /ip4/IP2/tcp/30333/p2p/PEER_ID2
   ```

5. **Verify Network**:
   ```bash
   # Check peer count matches authorized validators
   # Monitor for unauthorized connection attempts
   ```

## Monitoring and Operations

### Key Metrics

- `etrid_sub_libp2p_peers_count`: Should equal number of authorized validators
- `etrid_sub_libp2p_rejected_connections_total`: Monitor for attack attempts
- `nodeAuthorization.wellKnownNodes`: On-chain storage of authorized nodes

### Common Operations

#### Add New Validator

```bash
# 1. Generate node key for new validator
subkey generate-node-key --file new-validator-node-key

# 2. Submit governance proposal or use sudo
polkadot-js-api \
  --ws ws://localhost:9944 \
  --seed "//Alice" \
  --sudo \
  tx.nodeAuthorization.addWellKnownNode PEER_ID ACCOUNT_ID

# 3. New validator can now connect
```

#### Remove Validator

```bash
# 1. Submit governance proposal or use sudo
polkadot-js-api \
  --ws ws://localhost:9944 \
  --seed "//Alice" \
  --sudo \
  tx.nodeAuthorization.removeWellKnownNode PEER_ID

# 2. Validator will be disconnected
```

### Troubleshooting

See `PEER_WHITELISTING_GUIDE.md` for detailed troubleshooting procedures.

## Files Modified

### Runtime Files

1. `/05-multichain/primearc-core/runtime/Cargo.toml`
   - Added `pallet-node-authorization` dependency
   - Added to `std` feature list

2. `/05-multichain/primearc-core/runtime/src/lib.rs`
   - Added `node_authorization_helper` module declaration
   - Added `pallet_node_authorization::Config` implementation
   - Added `NodeAuthorization` to `construct_runtime!`

3. `/05-multichain/primearc-core/runtime/src/node_authorization_helper.rs`
   - New file: Integration helpers

### Documentation Files

1. `/05-multichain/primearc-core/docs/PEER_WHITELISTING_GUIDE.md`
   - Comprehensive user guide

2. `/05-multichain/primearc-core/docs/NODE_AUTHORIZATION_IMPLEMENTATION.md`
   - This implementation summary

### Utility Files

1. `/05-multichain/primearc-core/scripts/generate-node-authorization-genesis.sh`
   - Helper script for genesis generation

2. `/05-multichain/primearc-core/runtime/presets/development_with_auth.json`
   - Example genesis preset with node authorization

## Future Enhancements

### Phase 2: Automatic PeerId Derivation

Currently, PeerIds are manually configured. Future enhancement:

```rust
// Derive PeerId from session keys automatically
impl pallet_session::SessionHandler<AccountId> for AsfSessionHandler {
    fn on_new_session<Ks: OpaqueKeys>(
        _changed: bool,
        validators: &[(AccountId, Ks)],
        _queued_validators: &[(AccountId, Ks)],
    ) {
        // Extract PeerId from session keys
        // Update nodeAuthorization automatically
    }
}
```

### Phase 3: Integration with Staking

```rust
// Require minimum stake for node authorization
impl pallet_node_authorization::Config for Runtime {
    type AddOrigin = EnsureStaked<MinValidatorStake>;
}
```

### Phase 4: Reputation System

Track and penalize malicious connection attempts:

```rust
// Track unauthorized connection attempts
// Slash validators that repeatedly attempt unauthorized connections
```

## Compliance and Auditing

### Security Considerations

- **Origin Safety**: All node authorization changes require Root origin
- **Genesis Validation**: Ensure genesis preset matches validator set
- **Key Management**: Secure storage and backup of node keys
- **Network Monitoring**: Continuous monitoring for unauthorized attempts

### Audit Trail

All node authorization changes are recorded on-chain via events:

- `NodeAdded(PeerId, AccountId)`
- `NodeRemoved(PeerId)`
- `NodeSwapped(PeerId, PeerId)`

Query historical changes via block explorer or chain state queries.

## References

- Substrate Node Authorization: https://docs.substrate.io/reference/how-to-guides/basics/configure-genesis-state/#authorize-specific-nodes
- libp2p Specifications: https://github.com/libp2p/specs
- Polkadot Network Security: https://wiki.polkadot.network/docs/learn-security
- ËTRID Documentation: https://docs.etrid.io

## Support

- GitHub: https://github.com/etrid/etrid
- Discord: https://discord.gg/etrid
- Email: support@etrid.io
