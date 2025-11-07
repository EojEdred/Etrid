# Etrid Validator Key Deployment Summary

**Date:** November 3, 2025
**Status:** ✓ SUCCESSFUL - All 21 validators deployed across 17 Azure VMs

## Deployment Overview

Successfully distributed and deployed validator keys for 21 validators across 17 Azure VMs using automated Python deployment script.

### Key Statistics
- **Total Validators:** 21
- **Total VMs:** 17
- **Successful Deployments:** 17/17 (100%)
- **Failed Deployments:** 0/17 (0%)
- **Keys Inserted per Validator:** 2 (AURA + GRANDPA)

## Validator Distribution by VM

### Single Validator VMs (VMs 1-11)
Each VM hosts exactly one validator:

| VM # | IP Address       | Validator(s)                  | Validator Index |
|------|------------------|-------------------------------|-----------------|
| 1    | 98.71.91.84      | Gizzi (AI Overseer)          | V1              |
| 2    | 68.219.230.63    | EojEdred (Founder)           | V2              |
| 3    | 4.180.59.25      | validator-3                  | V3              |
| 4    | 20.224.104.239   | validator-4                  | V4              |
| 5    | 98.71.219.106    | validator-5                  | V5              |
| 6    | 108.142.205.177  | validator-6                  | V6              |
| 7    | 4.180.238.67     | validator-7                  | V7              |
| 8    | 51.142.203.160   | validator-8                  | V8              |
| 9    | 172.166.164.19   | validator-9                  | V9              |
| 10   | 172.166.187.180  | validator-10                 | V10             |
| 11   | 172.166.210.244  | validator-11                 | V11             |

### Multi-Validator VMs (VMs 12-17)
These VMs host 2 validators each (except VM 16 and 17 with 1 each):

| VM # | IP Address       | Validator(s)                       | Validator Indices |
|------|------------------|------------------------------------|-------------------|
| 12   | 172.167.8.217    | validator-12, validator-13        | V12, V13          |
| 13   | 4.251.115.186    | validator-14, validator-15        | V14, V15          |
| 14   | 52.143.191.232   | validator-16, validator-17        | V16, V17          |
| 15   | 4.211.206.210    | validator-18, validator-19        | V18, V19          |
| 16   | 4.178.181.122    | validator-20                      | V20               |
| 17   | 4.233.88.42      | validator-21                      | V21               |

## Key Types Deployed

For each validator, the following keys were inserted:

### AURA Keys (Block Production)
- **Scheme:** Sr25519
- **Key Type:** `aura`
- **Purpose:** Block authoring and consensus participation

### GRANDPA Keys (Finality)
- **Scheme:** Ed25519
- **Key Type:** `gran`
- **Purpose:** Block finalization in GRANDPA consensus

### Note on ASF Keys
ASF (Advanced Secure Finality) keys are handled by the AURA consensus mechanism in this runtime implementation. The `asfKey` field in the master keys JSON is equivalent to the `auraKey` for the current FlareChain implementation.

## Deployment Details

### SSH Configuration
- **SSH User:** `audit-dev01`
- **SSH Key:** `~/.ssh/etrid_vm1`
- **Base Path:** `~/.local/share/flarechain`
- **Node Binary:** `~/etrid/target/release/flarechain-node`

### Key Insert Command Format
```bash
~/etrid/target/release/flarechain-node key insert \
  --base-path ~/.local/share/flarechain \
  --scheme [Sr25519|Ed25519] \
  --suri "<hex_seed>" \
  --key-type [aura|gran]
```

## Validator Key Details

### Bootstrap Validators
1. **Validator 1 - Gizzi (AI Overseer)**
   - Account ID: `5Dd8AjjuwKDP8P8sDguiiNKfADAXrACramNbWvLcdLEpGaPJ`
   - Role: Director
   - Stake: 128 ETR
   - Bootstrap Order: 1
   - VM: 1 (98.71.91.84)

2. **Validator 2 - EojEdred (Founder)**
   - Account ID: `5HYpUK51E1BzhEfiRikhjkNivJiw2WAEG5Uxsrbj5ZE669EM`
   - Role: Director
   - Stake: 128 ETR
   - Bootstrap Order: 2
   - VM: 2 (68.219.230.63)

### Non-Bootstrap Validators (V3-V21)
All deployed with Director (V3-V5) or FlareNode (V6-V11) or ValidityNode (V12-V21) roles.

## Files Generated

### Deployment Scripts
Individual deployment scripts created for each VM:
- `/tmp/deploy_vm1_98_71_91_84.sh` through `/tmp/deploy_vm17_4_233_88_42.sh`

### Result Files
- **Deployment Log:** `/Users/macbook/Desktop/etrid/deployment_log.txt`
- **JSON Results:** `/Users/macbook/Desktop/etrid/deployment_results.json`
- **Python Script:** `/Users/macbook/Desktop/etrid/deploy_validators_to_azure.py`

## Verification

Keys were successfully inserted into the keystore on each VM. Example verification:

### VM 1 Keystore
```
~/.local/share/flarechain/chains/*/keystore/
├── 6175726152a5b54c0d313fdefb38dc3444f7668c3e96dabbacd0da8f66040cd8459f0270  (aura key)
└── 6772616eb819e158f66c786ca3c5fcd86033dea54c62f95f6d016c58b5e8bb3442613882  (gran key)
```

### VM 12 Keystore (2 validators)
```
~/.local/share/flarechain/chains/*/keystore/
├── 617572611e741768de3e7b3ea8e28647b7c75f55ec5110508ef31cecf309169731cb9025  (aura key - V12)
├── 61757261fa2e27083e33d7243553871c827158db6ba6bca18cb91dde951f59ac9ad79d69  (aura key - V13)
├── 6772616e8c7b5b24f2e69639a838b2b95d4de3049445b57ab142049fcd6f953dcbe989b2  (gran key - V12)
└── 6772616eec311e69bb875773a2850d11bc4825d186417628f0140a3ea6c7e5f0fa1f51b7  (gran key - V13)
```

## Next Steps

To start the validators:

1. **Start the FlareChain nodes** on each VM with the validator flag:
   ```bash
   ~/etrid/target/release/flarechain-node \
     --validator \
     --base-path ~/.local/share/flarechain \
     --name "ValidatorName" \
     --chain mainnet-raw.json
   ```

2. **Verify validator participation** using telemetry or chain state queries

3. **Monitor validator performance** through the validator dashboard

## Security Notes

- All private keys remain secure on their respective VMs
- Keys are stored with restricted permissions (600) in the keystore
- SSH access is secured via key-based authentication
- Master key JSON file should be kept offline in encrypted storage

## Deployment Success Confirmation

✓ All 17 VMs successfully received their assigned validator keys
✓ All 21 validators are ready for network participation
✓ AURA and GRANDPA keys properly inserted for all validators
✓ No errors encountered during deployment

---

**Deployment completed successfully on:** November 3, 2025 00:36 UTC
