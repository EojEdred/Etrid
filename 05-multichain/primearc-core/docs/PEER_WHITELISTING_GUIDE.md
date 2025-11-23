# Peer Whitelisting / Node Authorization Guide

## Overview

The Ëtrid Primearc Core Chain (Primearc Core Chain) now implements comprehensive peer whitelisting through `pallet-node-authorization`. This ensures that only authorized validator nodes can connect to the network, providing production-grade security.

## Architecture

### Components

1. **pallet-node-authorization**: Substrate's standard node authorization pallet
2. **pallet-validator-committee**: Custom validator management pallet
3. **node-authorization-helper**: Integration layer between the two

### Security Model

- **Bootnode Whitelisting**: Only pre-configured validator PeerIds can connect initially
- **Validator-Only Network**: Non-validators cannot participate in consensus
- **Dynamic Updates**: Authorized nodes can be added/removed via governance
- **Session Integration**: Peer authorization syncs with validator session changes

## Genesis Configuration

### Adding Authorized Peers to Chain Spec

In your genesis preset JSON (e.g., `flarechain_mainnet_v1_pure_asf.json`), add:

```json
{
  "nodeAuthorization": {
    "nodes": [
      [
        "12D3KooWBmAwcd4PJNJvfV89HwE48nwkRmAgo8Vy3uQEyNNHBox2",
        "5Dd8AjjuwKDP8P8sDguiiNKfADAXrACramNbWvLcdLEpGaPJ"
      ],
      [
        "12D3KooWQYV9dGMFoRzNStwpXztXaBUjtPqi6aU76ZgUriHhKust",
        "5HYpUK51E1BzhEfiRikhjkNivJiw2WAEG5Uxsrbj5ZE669EM"
      ]
    ]
  }
}
```

### Field Explanation

- **First element**: PeerId (libp2p multihash-encoded peer identifier)
- **Second element**: AccountId (SS58-encoded validator account)

## Getting PeerIds

### Method 1: From Running Node

If you have a validator node running:

```bash
# Get the node's local PeerId
curl -H "Content-Type: application/json" \
     -d '{"id":1, "jsonrpc":"2.0", "method": "system_localPeerId"}' \
     http://localhost:9944
```

Response:
```json
{
  "jsonrpc": "2.0",
  "result": "12D3KooWBmAwcd4PJNJvfV89HwE48nwkRmAgo8Vy3uQEyNNHBox2",
  "id": 1
}
```

### Method 2: From Node Key File

If you have the node's secret key:

```bash
# Node key is typically stored in:
# /path/to/chain-data/network/secret_ed25519

# Use subkey to derive PeerId from the secret
subkey inspect-node-key --file /path/to/chain-data/network/secret_ed25519
```

### Method 3: From Validator Session Keys

```bash
# Query the validator's session keys from the chain
curl -H "Content-Type: application/json" \
     -d '{
       "id":1,
       "jsonrpc":"2.0",
       "method":"author_hasSessionKeys",
       "params":["YOUR_SESSION_KEYS_HEX"]
     }' \
     http://localhost:9944
```

## Managing Authorized Peers

### Add a Peer (Sudo/Governance)

```bash
# Using Polkadot.js Apps or polkadot-js/api

# 1. Go to Developer > Sudo (or Developer > Extrinsics for governance)
# 2. Select nodeAuthorization.addWellKnownNode
# 3. Parameters:
#    - node: PeerId (e.g., 12D3KooW...)
#    - owner: AccountId of the validator

# CLI example using polkadot-js-api
node -e "
const { ApiPromise, WsProvider } = require('@polkadot/api');
const { Keyring } = require('@polkadot/keyring');

async function addNode() {
  const wsProvider = new WsProvider('ws://localhost:9944');
  const api = await ApiPromise.create({ provider: wsProvider });

  const keyring = new Keyring({ type: 'sr25519' });
  const sudo = keyring.addFromUri('//Alice');

  const peerId = '12D3KooWBmAwcd4PJNJvfV89HwE48nwkRmAgo8Vy3uQEyNNHBox2';
  const owner = '5Dd8AjjuwKDP8P8sDguiiNKfADAXrACramNbWvLcdLEpGaPJ';

  const tx = api.tx.nodeAuthorization.addWellKnownNode(peerId, owner);
  await api.tx.sudo.sudo(tx).signAndSend(sudo);
}

addNode();
"
```

### Remove a Peer (Sudo/Governance)

```bash
# Using Polkadot.js Apps
# 1. Developer > Sudo
# 2. Select nodeAuthorization.removeWellKnownNode
# 3. Parameters:
#    - node: PeerId to remove

# CLI example
const tx = api.tx.nodeAuthorization.removeWellKnownNode(peerId);
await api.tx.sudo.sudo(tx).signAndSend(sudo);
```

### Query Authorized Nodes

```bash
# Get all well-known nodes
curl -H "Content-Type: application/json" \
     -d '{
       "id":1,
       "jsonrpc":"2.0",
       "method":"state_call",
       "params":["NodeAuthorizationApi_well_known_nodes", "0x"]
     }' \
     http://localhost:9944
```

## Integration with Validator Committee

The system automatically integrates with `pallet-validator-committee`:

### On Genesis

- Validators from `validatorCommittee.validators` are converted to authorized nodes
- Their PeerIds are derived from AccountIds (or session keys in production)

### On Session Change

When validator set changes (every epoch):

1. New validators joining the committee should be added via governance
2. Exiting validators can be removed to prevent stale connections

### Manual Sync (if needed)

```rust
// In runtime upgrade or migration
node_authorization_helper::sync_authorized_nodes_with_validators::<Runtime>()?;
```

## Production Deployment Checklist

### 1. Generate Node Keys for All Validators

```bash
# For each validator, generate a unique node key
subkey generate-node-key --file /path/to/validator1/node-key

# This generates:
# - Secret key: saved to file
# - Public key (PeerId): printed to stdout
```

### 2. Collect PeerIds and AccountIds

Create a mapping file (`authorized-peers.json`):

```json
[
  {
    "name": "Validator-1",
    "accountId": "5Dd8AjjuwKDP8P8sDguiiNKfADAXrACramNbWvLcdLEpGaPJ",
    "peerId": "12D3KooWBmAwcd4PJNJvfV89HwE48nwkRmAgo8Vy3uQEyNNHBox2",
    "ip": "203.0.113.1",
    "port": 30333
  },
  {
    "name": "Validator-2",
    "accountId": "5HYpUK51E1BzhEfiRikhjkNivJiw2WAEG5Uxsrbj5ZE669EM",
    "peerId": "12D3KooWQYV9dGMFoRzNStwpXztXaBUjtPqi6aU76ZgUriHhKust",
    "ip": "203.0.113.2",
    "port": 30333
  }
]
```

### 3. Update Genesis Preset

Add `nodeAuthorization` section to your preset:

```json
{
  "nodeAuthorization": {
    "nodes": [
      ["12D3KooWBmAwcd4PJNJvfV89HwE48nwkRmAgo8Vy3uQEyNNHBox2", "5Dd8AjjuwKDP8P8sDguiiNKfADAXrACramNbWvLcdLEpGaPJ"],
      ["12D3KooWQYV9dGMFoRzNStwpXztXaBUjtPqi6aU76ZgUriHhKust", "5HYpUK51E1BzhEfiRikhjkNivJiw2WAEG5Uxsrbj5ZE669EM"]
    ]
  }
}
```

### 4. Configure Bootnodes

In your chain spec or node flags:

```bash
--bootnodes /ip4/203.0.113.1/tcp/30333/p2p/12D3KooWBmAwcd4PJNJvfV89HwE48nwkRmAgo8Vy3uQEyNNHBox2 \
--bootnodes /ip4/203.0.113.2/tcp/30333/p2p/12D3KooWQYV9dGMFoRzNStwpXztXaBUjtPqi6aU76ZgUriHhKust
```

### 5. Start Validators with Node Keys

```bash
# Each validator should use their specific node-key
./primearc-core-node \
  --validator \
  --name "Validator-1" \
  --node-key-file /path/to/validator1/node-key \
  --chain flarechain_mainnet_v1 \
  --base-path /data/validator1 \
  --bootnodes /ip4/203.0.113.2/tcp/30333/p2p/12D3KooWQYV9dGMFoRzNStwpXztXaBUjtPqi6aU76ZgUriHhKust
```

### 6. Verify Connections

```bash
# Check connected peers
curl -H "Content-Type: application/json" \
     -d '{"id":1, "jsonrpc":"2.0", "method": "system_peers"}' \
     http://localhost:9944 | jq '.result | length'

# Expected: Should show only authorized validators
```

## Security Considerations

### Access Control

- **Origin Requirements**: All node authorization extrinsics require `Root` origin
- **Governance**: In production, use `EnsureRoot` with a multisig or governance pallet
- **Sudo**: Only for development/testing - remove in production

### Attack Vectors

1. **Eclipse Attacks**: Prevented by whitelisting - only known validators can connect
2. **Sybil Attacks**: Prevented by requiring governance approval for new nodes
3. **Network Partitioning**: Mitigated by having stable bootnode set

### Monitoring

Monitor these metrics:

```bash
# Peer count (should match authorized validator count)
etrid_sub_libp2p_peers_count

# Unauthorized connection attempts (should be 0 or very low)
etrid_sub_libp2p_rejected_connections_total
```

## Troubleshooting

### Node Can't Connect to Network

**Symptom**: Node shows "Discovered new external address" but no peers

**Diagnosis**:
```bash
# Check if node's PeerId is authorized
curl -H "Content-Type: application/json" \
     -d '{
       "id":1,
       "jsonrpc":"2.0",
       "method":"state_getStorage",
       "params":["0x..."] // Storage key for well-known nodes
     }' \
     http://localhost:9944
```

**Solution**:
1. Verify node is using correct node-key file
2. Check PeerId is in genesis or added via governance
3. Ensure bootnodes are correct

### Validator Can't Propose Blocks

**Symptom**: Validator is in committee but not proposing

**Diagnosis**:
```bash
# Check if validator is in active committee
curl -H "Content-Type: application/json" \
     -d '{
       "id":1,
       "jsonrpc":"2.0",
       "method":"validatorCommittee_validatorInfo",
       "params":["VALIDATOR_ACCOUNT_ID"]
     }' \
     http://localhost:9944
```

**Solution**:
1. Verify validator has session keys set
2. Check validator is in both `validatorCommittee` and `nodeAuthorization`
3. Ensure stake meets minimum requirement

### New Validator Can't Join

**Symptom**: Added via governance but can't connect

**Solution**:
1. Add to `nodeAuthorization` first (via sudo/governance)
2. Then add to `validatorCommittee`
3. Wait for session change to take effect

## Testing

### Local Development

For local testing with Alice/Bob:

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

### Integration Testing

```bash
# 1. Build runtime
cargo build --release -p primearc-runtime

# 2. Generate chain spec with node authorization
./target/release/primearc-core-node build-spec \
  --chain flarechain_mainnet_v1 \
  --raw \
  > chainspec-with-auth.json

# 3. Start first validator
./target/release/primearc-core-node \
  --alice \
  --validator \
  --chain chainspec-with-auth.json \
  --base-path /tmp/alice

# 4. Start second validator (should connect)
./target/release/primearc-core-node \
  --bob \
  --validator \
  --chain chainspec-with-auth.json \
  --base-path /tmp/bob \
  --bootnodes /ip4/127.0.0.1/tcp/30333/p2p/ALICE_PEER_ID

# 5. Try to connect unauthorized node (should fail)
./target/release/primearc-core-node \
  --charlie \
  --chain chainspec-with-auth.json \
  --base-path /tmp/charlie \
  --bootnodes /ip4/127.0.0.1/tcp/30333/p2p/ALICE_PEER_ID
```

## Future Enhancements

### Planned Features

1. **Automatic PeerId Extraction**: Derive PeerId from session keys automatically
2. **Dynamic Sync**: Auto-update authorized nodes on session changes
3. **Reputation System**: Track and penalize malicious connection attempts
4. **Connection Limits**: Enforce max connections per validator
5. **Geographic Distribution**: Ensure validator diversity across regions

### Integration with Other Pallets

- **pallet-session**: Sync with session key rotations
- **pallet-staking**: Require minimum stake for authorization
- **pallet-governance**: Decentralized authorization management

## References

- [Substrate Node Authorization Pallet](https://docs.substrate.io/reference/how-to-guides/basics/configure-genesis-state/#authorize-specific-nodes)
- [libp2p PeerId Specification](https://github.com/libp2p/specs/blob/master/peer-ids/peer-ids.md)
- [Polkadot Network Security Model](https://wiki.polkadot.network/docs/learn-security)

## Support

For issues or questions:
- GitHub Issues: https://github.com/etrid/etrid
- Discord: https://discord.gg/etrid
- Documentation: https://docs.etrid.io
