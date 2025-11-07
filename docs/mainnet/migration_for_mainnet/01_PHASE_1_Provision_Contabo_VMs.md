# Phase 1: Provision Contabo VMs

**Duration:** 30 minutes
**Goal:** Order 16 VPS instances and record their IPs

---

## Step 1: Create Contabo Account (5 minutes)

1. **Go to:** https://contabo.com
2. **Click:** "Sign Up" or "Register"
3. **Fill in:**
   - Email address
   - Password
   - Company/Personal details
4. **Verify** email address
5. **Add payment method** (credit card or PayPal)

✅ **Checkpoint:** You can log into Contabo customer portal

---

## Step 2: Order 16 VPS Instances (20 minutes)

### Order Process

1. **Log into** Contabo customer portal
2. **Navigate to:** VPS → Order New VPS
3. **Select Plan:** VPS M
   - 6 vCPU cores
   - 12 GB RAM
   - 200 GB NVMe SSD
   - €10.50/month

4. **Select OS:** Ubuntu 22.04 LTS (64-bit)

5. **Select Region:** Choose distributed locations for resilience
   - **Suggested distribution:**
     - 6 VMs in Germany (Nuremberg)
     - 5 VMs in USA (East Coast)
     - 5 VMs in USA (West Coast)
   - **OR:** All in one region if you prefer simplicity

6. **Additional Options:**
   - **Backup:** Optional (not required for validators)
   - **Managed Service:** No (we'll manage ourselves)
   - **Additional Storage:** No (200 GB is sufficient)

7. **Quantity:** Order in batches
   - First order: 6 VMs
   - Second order: 5 VMs
   - Third order: 5 VMs
   - *Or order all 16 at once if Contabo allows*

8. **Complete payment**

9. **Wait for provisioning** (5-15 minutes per batch)

✅ **Checkpoint:** You receive confirmation emails with VM credentials

---

## Step 3: Record All VM Information (5 minutes)

As VMs are provisioned, you'll receive emails with:
- Public IP address
- Root password
- SSH access details

**Create a tracking file:**

```bash
cd /Users/macbook/Desktop/etrid/docs/mainnet/migration_for_mainnet
nano vm-inventory.txt
```

**Format:**
```
# Ëtrid FlareChain - Contabo VM Inventory
# Date: November 7, 2025

VM01 | NEW-VALIDATOR-06 | IP: _____________ | Region: _______ | Root PW: ____________
VM02 | NEW-VALIDATOR-07 | IP: _____________ | Region: _______ | Root PW: ____________
VM03 | NEW-VALIDATOR-08 | IP: _____________ | Region: _______ | Root PW: ____________
VM04 | NEW-VALIDATOR-09 | IP: _____________ | Region: _______ | Root PW: ____________
VM05 | NEW-VALIDATOR-10 | IP: _____________ | Region: _______ | Root PW: ____________
VM06 | NEW-VALIDATOR-11 | IP: _____________ | Region: _______ | Root PW: ____________
VM07 | NEW-VALIDATOR-12 | IP: _____________ | Region: _______ | Root PW: ____________
VM08 | NEW-VALIDATOR-13 | IP: _____________ | Region: _______ | Root PW: ____________
VM09 | NEW-VALIDATOR-14 | IP: _____________ | Region: _______ | Root PW: ____________
VM10 | NEW-VALIDATOR-15 | IP: _____________ | Region: _______ | Root PW: ____________
VM11 | NEW-VALIDATOR-16 | IP: _____________ | Region: _______ | Root PW: ____________
VM12 | NEW-VALIDATOR-17 | IP: _____________ | Region: _______ | Root PW: ____________
VM13 | NEW-VALIDATOR-18 | IP: _____________ | Region: _______ | Root PW: ____________
VM14 | NEW-VALIDATOR-19 | IP: _____________ | Region: _______ | Root PW: ____________
VM15 | NEW-VALIDATOR-20 | IP: _____________ | Region: _______ | Root PW: ____________
VM16 | NEW-VALIDATOR-21 | IP: _____________ | Region: _______ | Root PW: ____________

# Session Key Mapping (from /Users/macbook/Desktop/etrid/secrets/validator-keys/)
# VM01 → Validator 6 keys
# VM02 → Validator 7 keys
# ... etc
```

**Fill this in as each VM is provisioned.**

✅ **Checkpoint:** All 16 IPs recorded

---

## Step 4: Test SSH Access to First VM (5 minutes)

Once you have at least 1 VM provisioned:

```bash
# Test SSH access
ssh root@FIRST_VM_IP

# You'll be prompted for root password (from email)
# Type it in (won't show as you type)

# First login may require password change
# Set a new secure password

# Test connectivity
ping -c 3 google.com

# Check disk space
df -h

# Check system info
uname -a
cat /etc/os-release

# Exit
exit
```

✅ **Checkpoint:** Successfully logged into first VM

---

## Step 5: Batch Setup - Initial Security (10 minutes)

For each VM as it becomes available:

```bash
# 1. SSH in
ssh root@VM_IP

# 2. Update system
apt update && apt upgrade -y

# 3. Install essentials
apt install -y curl wget git ufw net-tools

# 4. Configure firewall
ufw allow 22/tcp
ufw allow 30333/tcp
ufw allow 9944/tcp
ufw allow 9615/tcp
ufw --force enable

# 5. Set timezone (optional)
timedatectl set-timezone America/Chicago  # Or your timezone

# 6. Verify setup
ufw status
date

# 7. Exit
exit
```

**Repeat for all 16 VMs.**

**TIP:** Open multiple terminal windows to do this in parallel!

✅ **Checkpoint:** All VMs have basic security configured

---

## Step 6: Create SSH Key for Easy Access (5 minutes)

**On your local machine:**

```bash
# Generate new SSH key for Contabo VMs (if you want)
ssh-keygen -t ed25519 -f ~/.ssh/contabo-validators -C "etrid-validators"

# Or use your existing key
# ~/.ssh/gizzi-validator
```

**Copy key to each VM:**

```bash
# For each VM
ssh-copy-id -i ~/.ssh/contabo-validators root@VM_IP

# Or manually copy
ssh root@VM_IP "mkdir -p ~/.ssh && chmod 700 ~/.ssh"
cat ~/.ssh/contabo-validators.pub | ssh root@VM_IP "cat >> ~/.ssh/authorized_keys && chmod 600 ~/.ssh/authorized_keys"
```

**Test passwordless login:**

```bash
ssh -i ~/.ssh/contabo-validators root@VM_IP
```

✅ **Checkpoint:** Passwordless SSH working for all VMs

---

## Phase 1 Complete! ✅

You should now have:

- [x] 16 Contabo VPS instances provisioned
- [x] All IP addresses recorded
- [x] SSH access configured
- [x] Basic firewall rules applied
- [x] VMs ready for software deployment

**Total Cost:** €168/month (~$180/month)

---

## Next Step

**Open:** `02_PHASE_2_Deploy_Software.md`

That guide will walk you through:
- Copying node binary to all VMs
- Deploying chainspec file
- Installing session keys
- Creating systemd services

---

## Troubleshooting

### "Can't connect via SSH"
- Check IP address is correct
- Verify firewall rules (port 22 must be open)
- Try from different network (some ISPs block port 22)
- Contact Contabo support if persistent

### "VM provisioning taking too long"
- Contabo typically provisions in 5-15 minutes
- Check spam folder for emails
- Log into customer portal to check status
- Contact support if > 30 minutes

### "How many VMs should I order at once?"
- Contabo may limit initial orders
- Start with 5-6 VMs
- Order more as those provision
- All 16 should be ready within 1 hour

---

**Phase 1 Duration:** ~30 minutes
**Status:** Ready to proceed to Phase 2

