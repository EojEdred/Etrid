# ËTRID Bridge Configuration and Emergency Procedures

This directory contains critical configuration files and procedures for managing the ËTRID cross-chain bridge system.

## Directory Contents

### 1. emergency-procedures.md
**Comprehensive emergency response guide**

Complete documentation for emergency bridge operations including:
- How to pause/unpause token messenger and attestation services
- Emergency response scenarios and procedures
- Step-by-step commands for different emergency situations
- Post-emergency checklists
- Monitoring and alerting setup
- Testing procedures

**When to Use**: Primary reference during security incidents or operational issues.

### 2. quick-reference.md
**Quick reference card for emergency operations**

Condensed quick-reference guide with:
- One-page emergency pause commands
- Critical contact information
- Response timelines
- Essential checklists
- Quick status checks

**When to Use**: During active emergencies when you need immediate access to commands.

### 3. admin-accounts.json
**Admin account configuration and governance structure**

Contains:
- Emergency admin account addresses (multisig)
- Fee collector account configuration
- Attestation admin accounts
- Domain admin accounts
- Attester registry with public keys
- Governance configuration
- Security procedures and contacts
- Deployment checklists

**When to Use**: Reference for all administrative operations and account management.

### 4. implementation-details.md
**Technical implementation documentation**

Deep-dive technical documentation including:
- Storage structure and implementation
- Extrinsic details and call indices
- Weight calculations
- Security considerations
- Integration between pallets
- Testing recommendations
- Monitoring and observability

**When to Use**: For developers implementing integrations or conducting security audits.

---

## Current Implementation Status

Both pallets have COMPLETE emergency pause functionality already implemented:

### pallet-token-messenger ✅
- Pause Storage: Lines 553-556
- pause_bridge(): Lines 1062-1070
- unpause_bridge(): Lines 1081-1089
- Pause checks in deposit_for_burn() and receive_message()
- Events: BridgePaused, BridgeUnpaused

### pallet-bridge-attestation ✅
- Pause Storage: Lines 253-255
- pause_attestation(): Lines 805-810
- unpause_attestation(): Lines 817-822
- Pause checks in all critical operations
- Events: AttestationPaused, AttestationUnpaused

**No additional implementation required.**

---

For detailed information, see the individual documentation files in this directory.
