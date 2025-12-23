# ËTRID Bridge Emergency Pause - Quick Reference

This is a quick reference card for emergency bridge operations. For detailed procedures, see [emergency-procedures.md](./emergency-procedures.md).

## Emergency Pause Status

Both pallets have **COMPLETE** pause functionality already implemented:

- ✅ **pallet-token-messenger** - Lines 553-556, 749, 915, 1062-1089
- ✅ **pallet-bridge-attestation** - Lines 253-255, 384, 550, 656, 805-822, 980

## Quick Commands

### Check Bridge Status

```javascript
// Check if token messenger is paused
const isPaused = await api.query.tokenMessenger.isPaused();
console.log('Token Messenger:', isPaused.toHuman() ? 'PAUSED' : 'ACTIVE');

// Check if attestation is paused
const attestationPaused = await api.query.bridgeAttestation.isPaused();
console.log('Attestation:', attestationPaused.toHuman() ? 'PAUSED' : 'ACTIVE');
```

### Emergency Pause (Both Pallets)

```javascript
// EMERGENCY: Pause both pallets immediately
const tx = api.tx.utility.batchAll([
  api.tx.sudo.sudo(api.tx.tokenMessenger.pauseBridge()),
  api.tx.sudo.sudo(api.tx.bridgeAttestation.pauseAttestation())
]);

const hash = await tx.signAndSend(sudoAccount);
console.log('Emergency pause executed:', hash.toHex());
```

### Pause Token Messenger Only

```javascript
const tx = api.tx.sudo.sudo(api.tx.tokenMessenger.pauseBridge());
await tx.signAndSend(sudoAccount);
```

### Pause Attestation Only

```javascript
const tx = api.tx.sudo.sudo(api.tx.bridgeAttestation.pauseAttestation());
await tx.signAndSend(sudoAccount);
```

### Unpause Token Messenger

```javascript
const tx = api.tx.sudo.sudo(api.tx.tokenMessenger.unpauseBridge());
await tx.signAndSend(sudoAccount);
```

### Unpause Attestation

```javascript
const tx = api.tx.sudo.sudo(api.tx.bridgeAttestation.unpauseAttestation());
await tx.signAndSend(sudoAccount);
```

### Disable Specific Attester

```javascript
const tx = api.tx.sudo.sudo(
  api.tx.bridgeAttestation.disableAttester(attesterId)
);
await tx.signAndSend(sudoAccount);
```

### Disable Specific Domain

```javascript
const tx = api.tx.sudo.sudo(
  api.tx.tokenMessenger.configureDomain(
    domainId,
    false, // disabled
    maxBurnAmount,
    dailyLimit,
    minBurnAmount
  )
);
await tx.signAndSend(sudoAccount);
```

## Admin Accounts (Production)

| Role | Type | Permissions | Account |
|------|------|-------------|---------|
| **Emergency Admin** | 3-of-5 Multisig | All pause/unpause operations | See admin-accounts.json |
| **Fee Collector** | Single Account | Receives bridge fees | Configured in runtime |
| **Attestation Admin** | 3-of-5 Multisig | Manage attester registry | Same as Emergency Admin |
| **Domain Admin** | 3-of-5 Multisig | Configure domains | Same as Emergency Admin |

## Response Timeline

| Severity | Action | Timeline |
|----------|--------|----------|
| **Critical** | Pause bridge immediately | < 5 minutes |
| **High** | Investigate and pause if confirmed | < 30 minutes |
| **Medium** | Monitor and prepare mitigation | < 2 hours |
| **Low** | Address in next maintenance | As scheduled |

## Pre-Pause Checklist

- [ ] Identify root cause
- [ ] Assess impact scope
- [ ] Notify emergency response team
- [ ] Prepare communication for users
- [ ] Document incident details

## Post-Pause Checklist

- [ ] Investigate root cause
- [ ] Develop and test fix
- [ ] Test on testnet
- [ ] Prepare unpause plan
- [ ] Notify team and community
- [ ] Execute unpause
- [ ] Monitor closely for 24h
- [ ] Conduct post-mortem

## Emergency Contacts

- **Security Lead**: security@etrid.network
- **Operations Lead**: ops@etrid.network
- **Emergency Slack**: #bridge-emergency
- **Status Page**: https://status.etrid.network

## Important Notes

1. **Both pallets have independent pause controls**
2. **Pause requires root/sudo origin** (governance in production)
3. **Pause is immediate** (effective in the same block)
4. **Domain config and attester management work during pause**
5. **Always test on testnet first**

## Monitoring Events

Subscribe to these critical events:

```javascript
api.query.system.events((events) => {
  events.forEach(({ event }) => {
    if (event.method === 'BridgePaused') {
      alert('CRITICAL: Token bridge paused!');
    }
    if (event.method === 'AttestationPaused') {
      alert('CRITICAL: Attestation paused!');
    }
  });
});
```

---

**For detailed procedures and scenarios, see [emergency-procedures.md](./emergency-procedures.md)**
