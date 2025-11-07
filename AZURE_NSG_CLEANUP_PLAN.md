# Azure NSG SSH Rules Cleanup Plan
**Date:** November 7, 2025
**NSG:** eojedred-validator-nsg
**Resource Group:** ETRID-MAINNET
**Current Public IP:** 206.188.236.130

---

## Current SSH Rules (4 rules on port 22)

| Priority | Name | Source IP | Access | Status |
|----------|------|-----------|--------|--------|
| 100 | SSH-Access | 172.56.11.73 | Allow | ❓ Keep? |
| 102 | Temp-SSH-Claude | 216.165.151.59 | Allow | 🗑️ Temporary - Remove |
| 310 | Allow-SSH-From-New-IP | 172.58.12.128 | Allow | 🗑️ Temporary - Remove |
| 321 | SSH-IP-73-185-170-6 | 73.185.170.6/32 | Allow | ✅ Keep (likely permanent) |

---

## Proposed Cleanup Solution

### Option 1: Single Consolidated Rule (Recommended)
Delete all 4 rules and create ONE rule with multiple source IPs:

**Rule Details:**
- **Name:** SSH-Consolidated
- **Priority:** 100
- **Source:** 73.185.170.6/32, 206.188.236.130/32
- **Source Port:** *
- **Destination:** *
- **Destination Port:** 22
- **Protocol:** TCP
- **Access:** Allow
- **Description:** Consolidated SSH access for admin IPs

**Benefits:**
- Single rule = easier management
- Clear source IPs listed
- No confusion from temporary rules

---

### Option 2: Keep Existing Priority 321, Add Current IP
Keep only the permanent IP rule and add current IP:

**Keep:**
- Priority 321: SSH-IP-73-185-170-6 (73.185.170.6/32)

**Add:**
- Priority 100: SSH-Current-IP (206.188.236.130/32)

**Delete:**
- Priority 102: Temp-SSH-Claude
- Priority 310: Allow-SSH-From-New-IP
- Priority 100: SSH-Access (unless 172.56.11.73 is still needed)

---

## Execution Steps (Azure Portal)

### Step 1: Login to Azure Portal
1. Go to https://portal.azure.com
2. Login with: **eojedredbitepubkey1@proton.me**

### Step 2: Navigate to NSG
1. Search for "Network security groups"
2. Select **eojedred-validator-nsg**
3. Click **Inbound security rules** in left menu

### Step 3: Delete Temporary Rules
Delete these rules by clicking them and selecting "Delete":
- ✅ **Temp-SSH-Claude** (Priority 102)
- ✅ **Allow-SSH-From-New-IP** (Priority 310)

### Step 4: Update or Add Current IP Rule

**If keeping Priority 321 (Option 2):**
1. Click **SSH-IP-73-185-170-6** (Priority 321)
2. Update **Source** to: `73.185.170.6,206.188.236.130`
3. Update **Description** to: "SSH access for admin IPs"
4. Click **Save**

**OR for consolidated rule (Option 1):**
1. Delete all 4 existing SSH rules
2. Click **+ Add** to create new rule
3. Fill in:
   - Source: **IP Addresses**
   - Source IP addresses: `73.185.170.6,206.188.236.130`
   - Source port ranges: `*`
   - Destination: **Any**
   - Destination port ranges: `22`
   - Protocol: **TCP**
   - Action: **Allow**
   - Priority: `100`
   - Name: `SSH-Consolidated`
   - Description: `Consolidated SSH access for admin IPs`
4. Click **Add**

### Step 5: Test SSH Access
```bash
ssh -i ~/.ssh/gizzi-validator azureuser@20.69.26.209    # V0B-EojEdred
ssh -i ~/.ssh/gizzi-validator azureuser@52.252.142.146  # SecurityDev
```

---

## Execution Steps (Azure CLI)

If you have access to Azure Cloud Shell or CLI logged into the correct subscription:

```bash
# Get NSG name
NSG_NAME="eojedred-validator-nsg"
RG_NAME="ETRID-MAINNET"

# Option 1: Delete temporary rules
az network nsg rule delete --resource-group $RG_NAME --nsg-name $NSG_NAME --name "Temp-SSH-Claude"
az network nsg rule delete --resource-group $RG_NAME --nsg-name $NSG_NAME --name "Allow-SSH-From-New-IP"

# Option 2A: Update existing rule with multiple IPs
az network nsg rule update \
  --resource-group $RG_NAME \
  --nsg-name $NSG_NAME \
  --name "SSH-IP-73-185-170-6" \
  --source-address-prefixes 73.185.170.6 206.188.236.130 \
  --description "SSH access for admin IPs"

# Option 2B: Or create consolidated rule (delete all first)
az network nsg rule delete --resource-group $RG_NAME --nsg-name $NSG_NAME --name "SSH-Access"
az network nsg rule delete --resource-group $RG_NAME --nsg-name $NSG_NAME --name "SSH-IP-73-185-170-6"

az network nsg rule create \
  --resource-group $RG_NAME \
  --nsg-name $NSG_NAME \
  --name "SSH-Consolidated" \
  --priority 100 \
  --source-address-prefixes 73.185.170.6 206.188.236.130 \
  --source-port-ranges '*' \
  --destination-address-prefixes '*' \
  --destination-port-ranges 22 \
  --access Allow \
  --protocol Tcp \
  --description "Consolidated SSH access for admin IPs"
```

---

## Question: Is 172.56.11.73 Still Needed?

**Rule:** SSH-Access (Priority 100)
**Source IP:** 172.56.11.73

**Action Required:** Determine if this IP is still needed:
- ✅ **Keep** if this is a permanent admin IP or office network
- 🗑️ **Delete** if this was temporary or no longer used

---

## Recommended Final State

After cleanup, you should have **ONE** SSH rule:

```
Priority: 100
Name: SSH-Consolidated
Source: 73.185.170.6, 206.188.236.130 (+ 172.56.11.73 if needed)
Destination Port: 22
Protocol: TCP
Access: Allow
```

All other SSH rules (102, 310) deleted as temporary.

---

## Why This Matters

### Current Issues:
1. **4 rules = confusion** - which one is active?
2. **Temporary rules left behind** - "Temp-SSH-Claude", "Allow-SSH-From-New-IP"
3. **Current IP not in list** - 206.188.236.130 not in any rule
4. **Priority conflicts** - Multiple rules with different priorities

### After Cleanup:
1. ✅ **Single source of truth** - one rule for SSH
2. ✅ **Clear IP list** - all authorized IPs in one place
3. ✅ **Current IP included** - SSH access restored
4. ✅ **No conflicts** - single priority, clear intent

---

## Verification Checklist

After making changes, verify:
- [ ] SSH access works from current IP (206.188.236.130)
- [ ] SSH access works from permanent IP (73.185.170.6)
- [ ] Only necessary SSH rules remain (1-2 rules max)
- [ ] No temporary rules left (Temp-SSH-Claude, Allow-SSH-From-New-IP)
- [ ] Rule priority is clear (recommend 100)

---

## Need Help?

If SSH access is completely lost after changes:
1. Use **Azure Bastion** to connect via browser
2. Or use Azure Portal **Run Command** to re-add IP:
   ```bash
   # This runs directly on the VM
   sudo ufw allow from 206.188.236.130 to any port 22
   ```

---

**Status:** 📋 **READY FOR EXECUTION**
**Recommendation:** **Option 1 (Single Consolidated Rule)**
**Time Estimate:** 5 minutes via Azure Portal

---

*Generated for Azure NSG cleanup on November 7, 2025*
