# Peer Authorization Quick Start Guide

## For Validators

### Quick Commands

#### Get Your Node's PeerId
```bash
curl -s http://localhost:9944 -H "Content-Type: application/json" \
  -d '{"id":1, "jsonrpc":"2.0", "method": "system_localPeerId"}' | jq -r '.result'
```

#### Check If You're Authorized
```bash
# Get your PeerId first, then check
PEER_ID="12D3KooW..."
curl -s http://localhost:9944 -H "Content-Type: application/json" \
  -d "{\"id\":1, \"jsonrpc\":\"2.0\", \"method\": \"state_call\", \"params\":[\"NodeAuthorizationApi_well_known_nodes\", \"0x\"]}" \
  | jq ".result" | grep "$PEER_ID"
```

#### Check Connected Peers
```bash
curl -s http://localhost:9944 -H "Content-Type: application/json" \
  -d '{"id":1, "jsonrpc":"2.0", "method": "system_peers"}' | jq '.result | length'
```

### Common Issues

#### "No Peers Connected"
**Cause**: Your PeerId is not authorized

**Fix**:
1. Get your PeerId: See command above
2. Get your AccountId: `subkey inspect //YourSeed`
3. Contact governance to add your node:
   ```
   nodeAuthorization.addWellKnownNode(YOUR_PEER_ID, YOUR_ACCOUNT_ID)
   ```

#### "Connected but Not Proposing Blocks"
**Cause**: Not in validator committee or session keys not set

**Fix**:
1. Check if in committee:
   ```bash
   curl -s http://localhost:9944 -H "Content-Type: application/json" \
     -d '{"id":1, "jsonrpc":"2.0", "method": "validatorCommittee_validatorInfo", "params":["YOUR_ACCOUNT_ID"]}' \
     | jq
   ```

2. Set session keys if needed:
   ```bash
   # Generate keys
   curl -s http://localhost:9944 -H "Content-Type: application/json" \
     -d '{"id":1, "jsonrpc":"2.0", "method": "author_rotateKeys"}'

   # Set keys (via Polkadot.js Apps)
   # Developer > Extrinsics > session.setKeys(keys, proof)
   ```

## For Node Operators

### Deployment Checklist

- [ ] Generate unique node key: `subkey generate-node-key --file node.key`
- [ ] Record PeerId from output
- [ ] Record your validator AccountId
- [ ] Submit authorization request with PeerId + AccountId
- [ ] Wait for governance approval
- [ ] Start node with: `--node-key-file node.key`
- [ ] Verify connection to network
- [ ] Set session keys if validator

### Starting Your Node

```bash
./primearc-core-node \
  --validator \
  --name "My-Validator" \
  --node-key-file /secure/path/to/node.key \
  --chain flarechain_mainnet_v1 \
  --base-path /data/validator \
  --bootnodes /ip4/BOOTNODE_IP/tcp/30333/p2p/BOOTNODE_PEER_ID
```

### Backup Your Node Key

```bash
# Copy your node key to secure backup
cp /path/to/node.key /secure/backup/location/

# Test restore
./primearc-core-node \
  --node-key-file /secure/backup/location/node.key \
  --validator \
  --chain flarechain_mainnet_v1
```

## For Governance/Sudo

### Add New Validator Node

Via Polkadot.js Apps:
1. Developer > Sudo
2. Select: `nodeAuthorization.addWellKnownNode`
3. Parameters:
   - `node`: PeerId (e.g., `12D3KooW...`)
   - `owner`: AccountId (e.g., `5Dd8Ajj...`)
4. Submit Transaction

### Remove Validator Node

1. Developer > Sudo
2. Select: `nodeAuthorization.removeWellKnownNode`
3. Parameters:
   - `node`: PeerId to remove
4. Submit Transaction

### List All Authorized Nodes

```bash
curl -s http://localhost:9944 -H "Content-Type: application/json" \
  -d '{"id":1, "jsonrpc":"2.0", "method": "state_call", "params":["NodeAuthorizationApi_well_known_nodes", "0x"]}' \
  | jq
```

## Security Best Practices

1. **Never share your node key** - Treat it like a private key
2. **Backup node keys securely** - Use encrypted storage
3. **Monitor unauthorized attempts** - Set up alerts
4. **Rotate node keys periodically** - Via governance process
5. **Use firewall rules** - Limit P2P port access to known IPs

## Support

- Emergency: Discord #validator-support
- Non-urgent: GitHub Issues
- Documentation: https://docs.etrid.io/validators

## Quick Reference: PeerId Format

Valid PeerId examples:
- `12D3KooWEyoppNCUx8Yx66oV9fJnriXwCcXwDDUA2kj6vnc6iDEp`
- `12D3KooWHdiAxVd8uMQR1hGWXccidmfCwLqcMpGwR6QcTP6QRMuD`

Always starts with `12D3KooW` (base58 encoded multihash)

## Monitoring Commands

```bash
# Check sync status
curl -s http://localhost:9944 -H "Content-Type: application/json" \
  -d '{"id":1, "jsonrpc":"2.0", "method": "system_syncState"}' | jq

# Check node health
curl -s http://localhost:9944 -H "Content-Type: application/json" \
  -d '{"id":1, "jsonrpc":"2.0", "method": "system_health"}' | jq

# Check node version
curl -s http://localhost:9944 -H "Content-Type: application/json" \
  -d '{"id":1, "jsonrpc":"2.0", "method": "system_version"}' | jq -r '.result'
```
