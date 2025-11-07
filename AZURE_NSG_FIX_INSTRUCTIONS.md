# Azure NSG Fix Instructions for SSH Access

## Issue
SSH access is blocked on 2 Azure VMs even though they are running:
- eojedred-validator (20.69.26.209)
- SecurityDev (52.252.142.146)

## Impact
- ⚠️ Cannot SSH to these VMs for management
- ✅ Validators ARE running (confirmed via blockchain P2P network)
- ✅ Network is fully operational

---

## Fix Option 1: Azure Portal (Recommended - Easy)

### For eojedred-validator (20.69.26.209):

1. **Login to Azure Portal**
   - Go to https://portal.azure.com
   - Select the subscription containing these VMs

2. **Navigate to VM**
   - Search for "Virtual machines" or click "Virtual machines" in left menu
   - Find and click "eojedred-validator"

3. **Open Networking Settings**
   - In left menu, click "Networking" (under "Settings")
   - Click "Network settings"

4. **Add Inbound Rule**
   - Click "+ Add inbound port rule" or "Add inbound security rule"
   - Configure the rule:
     ```
     Source: IP Addresses
     Source IP: [Your IP address] or 0.0.0.0/0 (any IP - less secure)
     Source port ranges: *
     Destination: Any
     Service: SSH
     Destination port ranges: 22
     Protocol: TCP
     Action: Allow
     Priority: 100
     Name: Allow-SSH-MyIP
     Description: Allow SSH access from my IP
     ```
   - Click "Add"

5. **Wait 30 seconds** for rule to apply

6. **Test SSH**
   ```bash
   ssh -i ~/.ssh/gizzi-validator azureuser@20.69.26.209
   ```

### For SecurityDev (52.252.142.146):

Repeat the same steps above but select "SecurityDev" VM instead.

---

## Fix Option 2: Azure CLI

If you have Azure CLI configured:

```bash
# Get your current IP
MY_IP=$(curl -s https://api.ipify.org)

# Fix eojedred-validator
az network nsg rule create \
  --resource-group ETRID-MAINNET \
  --nsg-name eojedred-validator-nsg \
  --name Allow-SSH-MyIP \
  --priority 100 \
  --source-address-prefixes $MY_IP/32 \
  --destination-port-ranges 22 \
  --access Allow \
  --protocol Tcp \
  --description "Allow SSH from my IP"

# Fix SecurityDev  
az network nsg rule create \
  --resource-group etrid-mainnet_group \
  --nsg-name SecurityDev-nsg \
  --name Allow-SSH-MyIP \
  --priority 100 \
  --source-address-prefixes $MY_IP/32 \
  --destination-port-ranges 22 \
  --access Allow \
  --protocol Tcp \
  --description "Allow SSH from my IP"
```

---

## Fix Option 3: Azure Cloud Shell

1. Go to https://shell.azure.com
2. Select "Bash"
3. Run the CLI commands from Option 2 above

---

## Verify NSG Rule Was Added

After adding the rule, verify it appears in the NSG:

1. Go to VM → Networking → Network settings
2. Look for "Allow-SSH-MyIP" in the inbound rules list
3. Should show:
   - Priority: 100
   - Port: 22
   - Protocol: TCP
   - Action: Allow

---

## Security Note

**Best Practice:** Use specific IP address (your IP) instead of 0.0.0.0/0

To get your current IP:
```bash
curl https://api.ipify.org
```

Then use that IP with /32 CIDR notation (e.g., 73.185.170.6/32)

---

## Alternative: Use Azure Bastion (No NSG Changes Needed)

If you don't want to modify NSG rules:

1. Go to Azure Portal → VM
2. Click "Connect" → "Bastion"
3. Enter username: azureuser
4. Use SSH private key authentication
5. Paste your private key content

This connects through Azure's internal network, bypassing NSG rules.

---

## Troubleshooting

### Still can't SSH after adding rule?

1. **Check rule priority** - Must be lower number than any deny rule
2. **Check rule status** - Should show as "Active"
3. **Wait 1-2 minutes** - Rules take time to propagate
4. **Verify your IP** - Make sure you used correct source IP
5. **Check VM is running** - Verify in Azure Portal
6. **Try different SSH key** - Might be using wrong key

### Test from Azure VM:

Since V1-Governance (20.186.91.207) is accessible, you can SSH from there:

```bash
# SSH to V1-Governance first
ssh -i ~/.ssh/gizzi-validator azureuser@20.186.91.207

# Then from V1-Governance, try to reach the other VMs
ssh azureuser@20.69.26.209
ssh azureuser@52.252.142.146
```

If this works, the NSG might be blocking external IPs only.

---

## Current Validator Status (Even Without SSH)

Both validators ARE running and participating in consensus:
- ✅ P2P port 30333 is open and responding
- ✅ SecurityDev confirmed in blockchain peer list
- ✅ Both VMs show as "Running" in Azure Portal
- ✅ Network has 20/21 validators online

**SSH access is for management only - not required for validator operation!**
