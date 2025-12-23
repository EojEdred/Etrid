# ËTRID Bridge Emergency Procedures

This document outlines emergency procedures for the ËTRID cross-chain bridge system, including how to pause and unpause bridge operations in case of security incidents or operational issues.

## Overview

The ËTRID bridge consists of two critical pallets with independent emergency pause capabilities:

1. **pallet-token-messenger** - Handles burn-and-mint token transfers
2. **pallet-bridge-attestation** - Manages M-of-N signature verification for cross-chain messages

Both pallets implement emergency pause functionality that can be activated by governance (sudo/root) to immediately halt operations.

---

## 1. pallet-token-messenger Emergency Pause

### Current Implementation Status

The pallet-token-messenger **already has** emergency pause functionality implemented:

- **Storage**: `IsPaused` (line 553-556 in lib.rs)
- **Extrinsics**:
  - `pause_bridge()` (line 1062-1070)
  - `unpause_bridge()` (line 1081-1089)
- **Protection**: Both `deposit_for_burn()` and `receive_message()` check pause status (lines 749, 915)
- **Events**: `BridgePaused` and `BridgeUnpaused` (lines 612-614)

### How to Pause Token Messenger

#### Using Polkadot.js Apps UI:

1. Navigate to: **Developer** → **Extrinsics**
2. Select account: **Root/Sudo account** (or governance multisig)
3. Select pallet: **tokenMessenger** (or your runtime's pallet name)
4. Select extrinsic: **pauseBridge()**
5. Click **Submit Transaction**
6. Sign and confirm the transaction

#### Using CLI (if configured):

```bash
# For FlareChain or PBC nodes
./flarechain-node \
  --base-path /data/chain \
  --rpc-methods Unsafe \
  --rpc-cors all \
  -- \
  sudo \
  tokenMessenger.pauseBridge

# Or using polkadot-js-api CLI:
polkadot-js-api \
  --ws ws://localhost:9944 \
  --sudo \
  tx.tokenMessenger.pauseBridge()
```

#### Using Substrate API (programmatic):

```javascript
const { ApiPromise, WsProvider } = require('@polkadot/api');
const { Keyring } = require('@polkadot/keyring');

async function pauseBridge() {
  const wsProvider = new WsProvider('ws://localhost:9944');
  const api = await ApiPromise.create({ provider: wsProvider });

  const keyring = new Keyring({ type: 'sr25519' });
  const sudoAccount = keyring.addFromUri('//Alice'); // Replace with actual sudo key

  // Pause the bridge
  const tx = api.tx.sudo.sudo(
    api.tx.tokenMessenger.pauseBridge()
  );

  const hash = await tx.signAndSend(sudoAccount);
  console.log('Bridge paused, tx hash:', hash.toHex());
}

pauseBridge().catch(console.error);
```

### Effects of Pausing Token Messenger

When paused, the following operations are **blocked**:

1. **deposit_for_burn()** - Users cannot initiate new cross-chain transfers
2. **receive_message()** - Relayers cannot deliver messages and mint tokens

**Note**: Domain configuration changes (`configure_domain`, `remove_domain`) are **not** blocked during pause. This allows governance to update configurations even during emergency.

### How to Unpause Token Messenger

Follow the same process as pausing, but use the **unpause_bridge()** extrinsic instead:

```javascript
// Using polkadot.js
const tx = api.tx.sudo.sudo(
  api.tx.tokenMessenger.unpauseBridge()
);
```

---

## 2. pallet-bridge-attestation Emergency Pause

### Current Implementation Status

The pallet-bridge-attestation **already has** emergency pause functionality implemented:

- **Storage**: `IsPaused` (line 253-255 in lib.rs)
- **Extrinsics**:
  - `pause_attestation()` (line 805-810)
  - `unpause_attestation()` (line 817-822)
- **Protection**: All critical operations check pause status (lines 384, 550, 656, 980)
- **Events**: `AttestationPaused` and `AttestationUnpaused` (lines 322-324)

### How to Pause Attestation Service

#### Using Polkadot.js Apps UI:

1. Navigate to: **Developer** → **Extrinsics**
2. Select account: **Root/Sudo account**
3. Select pallet: **bridgeAttestation**
4. Select extrinsic: **pauseAttestation()**
5. Submit and sign

#### Using CLI:

```bash
polkadot-js-api \
  --ws ws://localhost:9944 \
  --sudo \
  tx.bridgeAttestation.pauseAttestation()
```

#### Using Substrate API:

```javascript
const tx = api.tx.sudo.sudo(
  api.tx.bridgeAttestation.pauseAttestation()
);

const hash = await tx.signAndSend(sudoAccount);
console.log('Attestation paused, tx hash:', hash.toHex());
```

### Effects of Pausing Attestation Service

When paused, the following operations are **blocked**:

1. **register_attester()** - Cannot register new attesters
2. **submit_signature()** - Attesters cannot submit signatures
3. **verify_attestation()** - Cannot verify attestations (blocks message processing)
4. **verify_attestation_for_message()** - Internal verification function also checks pause

**Note**: Attester management operations (`disable_attester`, `enable_attester`, `remove_attester`) still work during pause, allowing governance to manage the attester set.

### How to Unpause Attestation Service

```javascript
const tx = api.tx.sudo.sudo(
  api.tx.bridgeAttestation.unpauseAttestation()
);
```

---

## 3. Emergency Response Scenarios

### Scenario A: Security Vulnerability Detected

**Action**: Immediately pause BOTH pallets

```javascript
// Batch pause both pallets in a single transaction
const tx = api.tx.utility.batchAll([
  api.tx.sudo.sudo(api.tx.tokenMessenger.pauseBridge()),
  api.tx.sudo.sudo(api.tx.bridgeAttestation.pauseAttestation())
]);

await tx.signAndSend(sudoAccount);
```

**Timeline**:
1. **T+0 min**: Pause both pallets immediately
2. **T+10 min**: Notify team and investigate issue
3. **T+1 hour**: Assess impact and determine fix
4. **T+4 hours**: Deploy patch/update if needed
5. **T+24 hours**: Comprehensive audit before unpause
6. **Unpause**: Only after confirming fix and security

### Scenario B: Attester Compromise

**Action**: Pause attestation, disable compromised attester(s), reconfigure threshold

```javascript
// 1. Pause attestation
await api.tx.sudo.sudo(
  api.tx.bridgeAttestation.pauseAttestation()
).signAndSend(sudoAccount);

// 2. Disable compromised attester
await api.tx.sudo.sudo(
  api.tx.bridgeAttestation.disableAttester(compromisedAttesterId)
).signAndSend(sudoAccount);

// 3. Update threshold if needed
await api.tx.sudo.sudo(
  api.tx.bridgeAttestation.updateThreshold(newThreshold)
).signAndSend(sudoAccount);

// 4. Unpause after verification
await api.tx.sudo.sudo(
  api.tx.bridgeAttestation.unpauseAttestation()
).signAndSend(sudoAccount);
```

### Scenario C: Excessive Volume/Rate Limit Attack

**Action**: Pause token messenger, adjust domain limits

```javascript
// 1. Pause bridge
await api.tx.sudo.sudo(
  api.tx.tokenMessenger.pauseBridge()
).signAndSend(sudoAccount);

// 2. Reconfigure domain limits
await api.tx.sudo.sudo(
  api.tx.tokenMessenger.configureDomain(
    domainId,
    true, // enabled
    newMaxBurnAmount,
    newDailyLimit,
    newMinBurnAmount
  )
).signAndSend(sudoAccount);

// 3. Unpause
await api.tx.sudo.sudo(
  api.tx.tokenMessenger.unpauseBridge()
).signAndSend(sudoAccount);
```

### Scenario D: Smart Contract Vulnerability on External Chain

**Action**: Disable specific domain

```javascript
// Option 1: Disable domain without full pause
await api.tx.sudo.sudo(
  api.tx.tokenMessenger.configureDomain(
    vulnerableDomainId,
    false, // disabled
    existingMaxBurnAmount,
    existingDailyLimit,
    existingMinBurnAmount
  )
).signAndSend(sudoAccount);

// Option 2: Remove domain entirely
await api.tx.sudo.sudo(
  api.tx.tokenMessenger.removeDomain(vulnerableDomainId)
).signAndSend(sudoAccount);
```

---

## 4. Admin Accounts and Permissions

### Required Roles

1. **Emergency Admin (Sudo)**
   - Can pause/unpause both pallets
   - Can disable attesters
   - Should be a multisig or governance collective
   - **CRITICAL**: Keep keys secure in HSM or multi-party custody

2. **Fee Admin**
   - Receives bridge fees (configured in runtime)
   - Cannot pause operations
   - Account: Configured via `FeeCollector` constant

3. **Attestation Admin**
   - Can manage attester registry (add/remove/enable/disable)
   - Should be governance or multisig
   - Same permissions as Emergency Admin (requires root)

### Production Configuration

For production, **replace sudo with governance**:

```rust
// In your runtime configuration (lib.rs)

// DEVELOPMENT: Direct sudo access
type EnsureRoot = frame_system::EnsureRoot<AccountId>;

// PRODUCTION: Governance collective
type EnsureRoot = pallet_collective::EnsureProportionAtLeast<AccountId, CouncilCollective, 2, 3>;
```

Or use a **time-locked multisig**:

```rust
type EnsureRoot = pallet_multisig::EnsureMultisig<AccountId, 3, 5>; // 3-of-5 multisig
```

---

## 5. Monitoring and Alerts

### Events to Monitor

#### Token Messenger Events:
- `BridgePaused` - Bridge was paused
- `BridgeUnpaused` - Bridge was unpaused
- `DailyLimitExceeded` - Rate limit triggered
- `FeeCollected` - Bridge fees collected

#### Attestation Events:
- `AttestationPaused` - Attestation service paused
- `AttestationUnpaused` - Attestation service unpaused
- `AttesterStatusChanged` - Attester enabled/disabled
- `AttesterRemoved` - Attester removed from registry

### Recommended Monitoring Setup

```javascript
// Subscribe to critical events
api.query.system.events((events) => {
  events.forEach((record) => {
    const { event } = record;

    // Alert on pause events
    if (event.section === 'tokenMessenger' && event.method === 'BridgePaused') {
      sendAlert('CRITICAL: Token bridge paused!');
    }

    if (event.section === 'bridgeAttestation' && event.method === 'AttestationPaused') {
      sendAlert('CRITICAL: Attestation service paused!');
    }

    // Alert on daily limits
    if (event.section === 'tokenMessenger' && event.method === 'DailyLimitExceeded') {
      sendAlert('WARNING: Daily limit exceeded', event.data);
    }
  });
});
```

### Health Check Script

```javascript
async function checkBridgeHealth() {
  const tokenMessengerPaused = await api.query.tokenMessenger.isPaused();
  const attestationPaused = await api.query.bridgeAttestation.isPaused();

  return {
    tokenMessenger: {
      paused: tokenMessengerPaused.toHuman(),
      status: tokenMessengerPaused.toHuman() ? 'PAUSED' : 'OPERATIONAL'
    },
    attestation: {
      paused: attestationPaused.toHuman(),
      status: attestationPaused.toHuman() ? 'PAUSED' : 'OPERATIONAL'
    }
  };
}

// Run every 30 seconds
setInterval(async () => {
  const health = await checkBridgeHealth();
  console.log('Bridge Health:', health);
}, 30000);
```

---

## 6. Post-Emergency Checklist

Before unpausing the bridge after an emergency, verify:

- [ ] Root cause identified and documented
- [ ] Fix/patch deployed and tested on testnet
- [ ] All active attesters verified and operational
- [ ] Signature threshold configured correctly
- [ ] Domain configurations reviewed and updated
- [ ] Rate limits adjusted if needed
- [ ] Monitoring and alerts functioning
- [ ] Team notified of unpause plan
- [ ] Post-mortem scheduled
- [ ] User communication prepared

---

## 7. Contact Information

### Emergency Response Team

- **Lead Engineer**: [Contact info]
- **Security Lead**: [Contact info]
- **DevOps Lead**: [Contact info]
- **Governance Coordinator**: [Contact info]

### Communication Channels

- **Emergency Slack**: #bridge-emergency
- **Status Page**: https://status.etrid.network
- **Community Updates**: Twitter @etrid_network

---

## 8. Testing Emergency Procedures

### Regular Drills (Monthly)

```bash
# 1. Test pause on testnet
polkadot-js-api --ws wss://testnet-rpc.etrid.network --sudo tx.tokenMessenger.pauseBridge()

# 2. Verify pause works (should fail)
polkadot-js-api tx.tokenMessenger.depositForBurn(100000, 0, "0x...")

# 3. Test unpause
polkadot-js-api --sudo tx.tokenMessenger.unpauseBridge()

# 4. Verify unpause works (should succeed)
polkadot-js-api tx.tokenMessenger.depositForBurn(100000, 0, "0x...")
```

### Test on Testnet Before Mainnet

Always test emergency procedures on testnet before executing on mainnet to ensure:
- Correct account permissions
- Proper transaction construction
- Expected behavior after pause/unpause
- No unintended side effects

---

## Summary

Both `pallet-token-messenger` and `pallet-bridge-attestation` have **complete emergency pause functionality** already implemented. The pallets can be paused independently or together, providing fine-grained control during security incidents.

**Key Takeaways**:
1. Emergency pause requires **root/sudo** origin (governance in production)
2. Pausing is **immediate** - operations are blocked in the same block
3. Domain configuration and attester management still work during pause
4. Always test on testnet before mainnet operations
5. Monitor events continuously for early warning
6. Document all emergency actions in incident reports

For any questions or clarifications, contact the ËTRID bridge security team.
