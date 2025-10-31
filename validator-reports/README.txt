################################################################################
# ËTRID VALIDATOR REPORTS - FOLDER CONTENTS
# Location: /Users/macbook/Desktop/etrid/validator-reports/
# Updated: October 31, 2025
################################################################################

This folder contains comprehensive documentation of all Ëtrid FlareChain
validator infrastructure.

================================================================================
FILES IN THIS FOLDER:
================================================================================

1. VALIDATOR_INFRASTRUCTURE_SUMMARY.txt (MAIN SUMMARY)
   ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
   📋 WHAT: Master summary of all 21 validators

   📊 INCLUDES:
      • Correct cloud distribution (2 Azure accounts + 2 Oracle)
      • All validator details organized by cloud provider
      • SSH access information
      • Monitoring status
      • Quick IP reference
      • Action items

   ⭐ START HERE for current infrastructure overview

2. VM_STATUS_COMPREHENSIVE.txt (DETAILED REPORT)
   ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
   📋 WHAT: Extremely detailed VM status report

   📊 INCLUDES:
      • Individual VM cards with all technical details
      • Power states, SSH, Node Exporter, Prometheus status
      • Resource groups and regions
      • Firewall configurations
      • Issues and troubleshooting notes

   💡 USE THIS for deep-dive troubleshooting

3. bootnodes_UPDATED.txt (BOOTNODE CONFIG)
   ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
   📋 WHAT: FlareChain bootnode configuration

   📊 INCLUDES:
      • Bootnode validator details (V#1, V0B, V#3)
      • Network connectivity information
      • All validator IPs by region
      • Bootnode activation instructions
      • Peer ID placeholders (to be updated when chain starts)

   🔗 USE THIS when configuring FlareChain network connectivity

4. README.txt (THIS FILE)
   ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
   📋 WHAT: This index file explaining folder contents

================================================================================
INFRASTRUCTURE SUMMARY:
================================================================================

Total Validators:    21

Cloud Distribution:
  • Oracle Cloud:    2 VMs
    └─ Validator #1 (Gizzi) - gizziio@64.181.215.19
    └─ Validator #3 (Audit) - aed2020@129.80.122.34

  • Azure (etridfoundation@proton.me):    16 VMs
    └─ Validators #6-21 (FlareNodes & ValidityNodes)
    └─ Regions: West Europe, North Europe, UK South, France Central

  • Azure (eojedredbitepubkey1@proton.me):    3 VMs
    └─ Validator 0B (EojEdred) - 20.69.26.209
    └─ Validator 1 (Governance) - 20.186.91.207
    └─ Validator 2 (Security) - 52.252.142.146

Monitoring Status:   19-20/21 validators (90-95%)
Monitoring Server:   98.71.91.84 (Validator #10)
Dashboard:           http://98.71.91.84/

================================================================================
QUICK ACTIONS:
================================================================================

🔴 URGENT:
  1. Install Node Exporter on Oracle Audit VM (aed2020@129.80.122.34)
  2. Verify Validator 1 firewall (20.186.91.207 port 9100)

⚠️  TODO:
  3. Update validator-ips.json with correct IPs
  4. Start FlareChain validators for blockchain metrics

================================================================================
CONTACT/ACCESS:
================================================================================

Azure Account 1:  etridfoundation@proton.me (16 VMs)
Azure Account 2:  eojedredbitepubkey1@proton.me (3 VMs)
Oracle Account 1: gizziio (Validator #1 - Gizzi)
Oracle Account 2: aed2020 (Validator #3 - Audit)

SSH Key: ~/.ssh/gizzi-validator

================================================================================
FOLDER STRUCTURE:
================================================================================

/Users/macbook/Desktop/etrid/validator-reports/
  ├── README.txt (this file)
  ├── VALIDATOR_INFRASTRUCTURE_SUMMARY.txt (main summary)
  ├── VM_STATUS_COMPREHENSIVE.txt (detailed report)
  └── bootnodes_UPDATED.txt (bootnode config)

================================================================================
LAST UPDATED: October 31, 2025
================================================================================
