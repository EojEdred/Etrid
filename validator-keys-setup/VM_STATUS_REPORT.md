# Etrid FlareChain - Complete 21-Validator Network Status

**Report Generated:** 2025-10-31
**Grafana Dashboard:** http://98.71.91.84:3000/d/9544262f-4207-4f22-a9dd-6778b4737a93/
**Live Validators (Grafana):** 19/21

---

## Summary

- **Total VMs:** 21
- **SSH Accessible:** 18
- **FlareChain Running:** 18 confirmed (+ 2 with SSH timeout but VM running per Azure)
- **Total FlareChain Processes:** 22 (some VMs run 2 processes)
- **Issues:** 3 VMs with connectivity problems

---

## Oracle Cloud VMs (2)

### 1. Gizzi - AI Overseer & Bootstrap
- **IP:** 64.181.215.19
- **User:** ubuntu
- **SSH:** `ssh -i ~/.ssh/gizzi-validator ubuntu@64.181.215.19`
- **Hostname:** gizzi-io-validator
- **Status:** ✅ ONLINE
- **FlareChain Processes:** 1
- **Role:** Bootstrap Validator #1, AI Overseer (Director)

### 2. Audit Dev - Oracle Cloud
- **IP:** 129.80.122.34
- **User:** ubuntu
- **SSH:** `ssh -i ~/.ssh/gizzi-validator ubuntu@129.80.122.34`
- **Hostname:** Unknown
- **Status:** ⚠️ SSH TIMEOUT (VM running per Azure)
- **FlareChain Processes:** Unknown
- **Issue:** Possible firewall blocking SSH port 22
- **Note:** This VM likely one of the 2 offline in Grafana

---

## Azure Subscription 2 VMs (3)

### 3. eojedred-validator
- **IP:** 20.69.26.209
- **User:** azureuser
- **SSH:** `ssh -i ~/.ssh/gizzi-validator azureuser@20.69.26.209`
- **Hostname:** eojedred-validator
- **Resource Group:** Etrid-Mainnet
- **Status:** ✅ ONLINE
- **FlareChain Processes:** 2
- **Role:** Validator

### 4. etrid-mainnet
- **IP:** 20.186.91.207
- **User:** azureuser
- **SSH:** `ssh -i ~/.ssh/gizzi-validator azureuser@20.186.91.207`
- **Hostname:** Unknown
- **Resource Group:** ETRID-MAINNET_GROUP
- **Status:** ⚠️ SSH TIMEOUT (VM running per Azure)
- **FlareChain Processes:** Unknown
- **Issue:** Possible firewall blocking SSH port 22
- **Note:** This VM possibly one of the 2 offline in Grafana

### 5. SecurityDev
- **IP:** 52.252.142.146
- **User:** azureuser
- **SSH:** `ssh -i ~/.ssh/gizzi-validator azureuser@52.252.142.146`
- **Hostname:** SecurityDev
- **Resource Group:** ETRID-MAINNET_GROUP
- **Status:** ✅ ONLINE
- **FlareChain Processes:** 2
- **Role:** Validator

---

## Azure Subscription 1 VMs (16)

### 6. Azure VM (Monitoring & Dashboard)
- **IP:** 98.71.91.84
- **User:** ubuntu
- **SSH:** `ssh -i ~/.ssh/gizzi-validator ubuntu@98.71.91.84`
- **Hostname:** Unknown
- **Status:** ❌ SSH KEY PERMISSION DENIED
- **FlareChain Processes:** Unknown
- **Issue:** gizzi-validator SSH key not authorized for ubuntu user
- **Note:** Hosts Grafana dashboard on :3000

### 7. etrid-consensus-dev-secondary
- **IP:** 20.224.104.239
- **User:** azureuser
- **SSH:** `ssh -i ~/.ssh/gizzi-validator azureuser@20.224.104.239`
- **Hostname:** etrid-consensus-dev-secondary
- **Status:** ✅ ONLINE
- **FlareChain Processes:** 1
- **Role:** Validator

### 8. etrid-runtime-dev-primary
- **IP:** 108.142.205.177
- **User:** azureuser
- **SSH:** `ssh -i ~/.ssh/gizzi-validator azureuser@108.142.205.177`
- **Hostname:** etrid-runtime-dev-primary
- **Status:** ✅ ONLINE
- **FlareChain Processes:** 1
- **Role:** Validator

### 9. etrid-runtime-dev-secondary
- **IP:** 4.180.238.67
- **User:** azureuser
- **SSH:** `ssh -i ~/.ssh/gizzi-validator azureuser@4.180.238.67`
- **Hostname:** etrid-runtime-dev-secondary
- **Status:** ✅ ONLINE
- **FlareChain Processes:** 1
- **Role:** Validator

### 10. etrid-compiler-dev-primary
- **IP:** 4.180.59.25
- **User:** azureuser
- **SSH:** `ssh -i ~/.ssh/gizzi-validator azureuser@4.180.59.25`
- **Hostname:** etrid-compiler-dev-primary
- **Status:** ✅ ONLINE
- **FlareChain Processes:** 1
- **Role:** Validator

### 11. etrid-multichain-dev-primary
- **IP:** 68.219.230.63
- **User:** azureuser
- **SSH:** `ssh -i ~/.ssh/gizzi-validator azureuser@68.219.230.63`
- **Hostname:** etrid-multichain-dev-primary
- **Status:** ✅ ONLINE
- **FlareChain Processes:** 1
- **Role:** Validator

### 12. etrid-multichain-dev-secondary
- **IP:** 98.71.219.106
- **User:** azureuser
- **SSH:** `ssh -i ~/.ssh/gizzi-validator azureuser@98.71.219.106`
- **Hostname:** etrid-multichain-dev-secondary
- **Status:** ✅ ONLINE
- **FlareChain Processes:** 2
- **Role:** Validator

### 13. etrid-oracle-dev
- **IP:** 172.167.8.217
- **User:** azureuser
- **SSH:** `ssh -i ~/.ssh/gizzi-validator azureuser@172.167.8.217`
- **Hostname:** etrid-oracle-dev
- **Status:** ✅ ONLINE
- **FlareChain Processes:** 1
- **Role:** Validator

### 14. etrid-audit-dev-secondary
- **IP:** 51.142.203.160
- **User:** azureuser
- **SSH:** `ssh -i ~/.ssh/gizzi-validator azureuser@51.142.203.160`
- **Hostname:** etrid-audit-dev-secondary
- **Status:** ✅ ONLINE
- **FlareChain Processes:** 1
- **Role:** Validator

### 15. etrid-flarenode-15
- **IP:** 172.166.164.19
- **User:** azureuser
- **SSH:** `ssh -i ~/.ssh/gizzi-validator azureuser@172.166.164.19`
- **Hostname:** etrid-flarenode-15
- **Status:** ✅ ONLINE
- **FlareChain Processes:** 1
- **Role:** FlareNode Validator

### 16. etrid-flarenode-16
- **IP:** 172.166.187.180
- **User:** azureuser
- **SSH:** `ssh -i ~/.ssh/gizzi-validator azureuser@172.166.187.180`
- **Hostname:** etrid-flarenode-16
- **Status:** ✅ ONLINE
- **FlareChain Processes:** 1
- **Role:** FlareNode Validator

### 17. etrid-flarenode-17
- **IP:** 172.166.210.244
- **User:** azureuser
- **SSH:** `ssh -i ~/.ssh/gizzi-validator azureuser@172.166.210.244`
- **Hostname:** etrid-flarenode-17
- **Status:** ✅ ONLINE
- **FlareChain Processes:** 1
- **Role:** FlareNode Validator

### 18. etrid-flarenode-18
- **IP:** 4.251.115.186
- **User:** azureuser
- **SSH:** `ssh -i ~/.ssh/gizzi-validator azureuser@4.251.115.186`
- **Hostname:** etrid-flarenode-18
- **Status:** ✅ ONLINE
- **FlareChain Processes:** 1
- **Role:** FlareNode Validator

### 19. etrid-flarenode-19
- **IP:** 52.143.191.232
- **User:** azureuser
- **SSH:** `ssh -i ~/.ssh/gizzi-validator azureuser@52.143.191.232`
- **Hostname:** etrid-flarenode-19
- **Status:** ✅ ONLINE
- **FlareChain Processes:** 1
- **Role:** FlareNode Validator

### 20. etrid-flarenode-20
- **IP:** 4.211.206.210
- **User:** azureuser
- **SSH:** `ssh -i ~/.ssh/gizzi-validator azureuser@4.211.206.210`
- **Hostname:** etrid-flarenode-20
- **Status:** ✅ ONLINE
- **FlareChain Processes:** 1
- **Role:** FlareNode Validator

### 21. etrid-flarenode-21
- **IP:** 4.178.181.122
- **User:** azureuser
- **SSH:** `ssh -i ~/.ssh/gizzi-validator azureuser@4.178.181.122`
- **Hostname:** etrid-flarenode-21
- **Status:** ✅ ONLINE
- **FlareChain Processes:** 1
- **Role:** FlareNode Validator

---

## Issues Requiring Resolution

### Critical Issues (Blocking Mainnet Launch)

1. **Audit Dev (129.80.122.34) - SSH Timeout**
   - Cannot verify FlareChain status
   - Likely one of the 2 offline validators in Grafana
   - **Action Required:** Configure Oracle Cloud firewall to allow SSH from management IP
   - **Alternative:** Use Oracle Cloud Console to verify status

2. **etrid-mainnet (20.186.91.207) - SSH Timeout**
   - Cannot verify FlareChain status
   - Possibly one of the 2 offline validators in Grafana
   - **Action Required:** Configure Azure NSG to allow SSH from management IP
   - **Alternative:** Use Azure Cloud Shell or Serial Console

3. **VM (98.71.91.84) - SSH Key Permission Denied**
   - Hosts Grafana monitoring dashboard (confirmed accessible on :3000)
   - Cannot verify FlareChain validator status
   - **Action Required:** Add gizzi-validator public key to ubuntu user's authorized_keys
   - **Azure CLI Command:**
     ```bash
     az vm user update --resource-group <RG> --name <VM_NAME> \\
       --username ubuntu \\
       --ssh-key-value "$(cat ~/.ssh/gizzi-validator.pub)"
     ```

### Likely Offline Validators in Grafana

Based on connectivity issues, the 2 offline validators (Grafana shows 19/21) are most likely:
1. **Audit Dev (129.80.122.34)** - Oracle Cloud VM with SSH timeout
2. **Either:**
   - etrid-mainnet (20.186.91.207) - Azure VM with SSH timeout, OR
   - VM (98.71.91.84) - Azure VM with SSH key issue (though Grafana is accessible, suggesting network is working)

---

## Next Steps

### Before Mainnet Launch:

1. **Fix SSH Connectivity Issues**
   - Resolve firewall/NSG rules for 129.80.122.34 and 20.186.91.207
   - Add SSH key for ubuntu@98.71.91.84

2. **Verify Validator Keys**
   - Ensure all 21 VMs have their session keys (AURA, GRANDPA, ASF) properly inserted
   - Cross-reference with `/Users/macbook/Desktop/etrid/validator-keys-setup/generated-keys/COMPLETE_VALIDATOR_NETWORK_MAP.md`

3. **Sync Repository**
   - Push latest changes to git
   - Pull latest code on all 21 VMs
   - Verify consistent commit hash across all validators

4. **Verify Grafana Shows 21/21 Live**
   - After fixing SSH and connectivity issues
   - All validators should show as online

5. **Launch Mainnet**
   - Start genesis block production
   - Monitor initial block creation and finalization
   - Verify consensus across all 21 validators

---

## Reference Files

- **Complete Validator Key Map:** `/Users/macbook/Desktop/etrid/validator-keys-setup/generated-keys/COMPLETE_VALIDATOR_NETWORK_MAP.md`
- **Validator Keys JSON:** `/Users/macbook/Desktop/etrid/mainnet-deployment-package/validator-keys-complete.json`
- **Deployment Package:** `/Users/macbook/Desktop/etrid/mainnet-deployment-package/`
- **This Report:** `/Users/macbook/Desktop/etrid/validator-keys-setup/VM_STATUS_REPORT.md`
