# Firewall Troubleshooting for FlareChain Validators

**Complete guide for diagnosing and fixing port 30333 connectivity issues**

---

## Why This Matters

**Port 30333 MUST be open for FlareChain validators to function.**

Without it:
- ❌ 0 peers (validator is isolated)
- ❌ No block sync (stuck at genesis)
- ❌ No consensus participation
- ❌ Wasted resources

This is the #1 cause of validator deployment failures.

---

## Quick Diagnosis

### Test 1: External Port Check

**From your local machine (NOT the validator VM):**
```bash
nc -zv <validator-ip> 30333
```

**✅ If successful:**
```
Connection to <ip> 30333 port [tcp/*] succeeded!
```

**❌ If blocked:**
```
nc: connect to <ip> port 30333 (tcp) failed: Connection refused
# OR
nc: connect to <ip> port 30333 (tcp) failed: No route to host
```

### Test 2: Check Validator Logs

```bash
ssh -i ~/.ssh/your-key user@<validator-ip> 'journalctl -u flarechain-validator -n 20 | grep peers'
```

**✅ Healthy:**
```
... 15 peers
```

**❌ Firewall issue:**
```
... 0 peers
```

---

## Diagnosis by Cloud Provider

### Contabo (CRITICAL)

**Default:** `iptables policy DROP` - blocks ALL incoming

**Check:**
```bash
ssh -i ~/.ssh/contabo-validators root@<ip> 'sudo iptables -L INPUT -n | grep 30333'
```

**If NO output** → Port is blocked

**Fix:**
```bash
ssh -i ~/.ssh/contabo-validators root@<ip> 'sudo iptables -I INPUT 1 -p tcp --dport 30333 -m comment --comment "FlareChain P2P" -j ACCEPT && DEBIAN_FRONTEND=noninteractive apt-get install -y iptables-persistent && netfilter-persistent save && systemctl restart flarechain-validator'
```

---

### Oracle Cloud

**Two layers: Cloud NSG + VM iptables**

**Check Cloud NSG:**
1. Oracle Cloud Console → Networking → Virtual Cloud Networks
2. Select your VCN → Security Lists
3. Look for Ingress Rule: Port 30333, Source 0.0.0.0/0

**If missing** → Add rule:
```
Source Type: CIDR
Source CIDR: 0.0.0.0/0
IP Protocol: TCP
Destination Port Range: 30333
```

**Check VM iptables:**
```bash
ssh -i ~/.ssh/gizzi-validator ubuntu@<ip> 'sudo iptables -L INPUT -n | grep 30333'
```

**Fix if needed:**
```bash
ssh -i ~/.ssh/gizzi-validator ubuntu@<ip> 'sudo iptables -I INPUT 1 -p tcp --dport 30333 -j ACCEPT && sudo apt-get install -y iptables-persistent && sudo netfilter-persistent save'
```

---

### Azure

**Two layers: NSG + VM iptables**

**Check NSG via Azure CLI:**
```bash
az network nsg rule list \
  --resource-group <resource-group> \
  --nsg-name <nsg-name> \
  --query "[?destinationPortRange=='30333']"
```

**If empty** → Add rule:
```bash
az network nsg rule create \
  --resource-group <resource-group> \
  --nsg-name <nsg-name> \
  --name Allow-FlareChain-P2P \
  --priority 100 \
  --source-address-prefixes '*' \
  --destination-port-ranges 30333 \
  --protocol Tcp \
  --access Allow
```

**Check VM iptables:**
```bash
ssh -i ~/.ssh/azure_validator_key azureuser@<ip> 'sudo iptables -L INPUT -n | grep 30333'
```

**Fix if needed:**
```bash
ssh -i ~/.ssh/azure_validator_key azureuser@<ip> 'sudo iptables -I INPUT 1 -p tcp --dport 30333 -j ACCEPT'
```

---

### DigitalOcean

**Check Cloud Firewall (if using):**
1. DigitalOcean Console → Networking → Firewalls
2. Check Inbound Rules for Port 30333

**Add if missing:**
```
Type: Custom
Protocol: TCP
Port Range: 30333
Sources: All IPv4, All IPv6
```

**Check VM firewall (if using ufw):**
```bash
ssh -i ~/.ssh/do-validator root@<ip> 'sudo ufw status | grep 30333'
```

**Fix:**
```bash
ssh -i ~/.ssh/do-validator root@<ip> 'sudo ufw allow 30333/tcp && sudo ufw reload'
```

---

### AWS

**Check Security Group:**
```bash
aws ec2 describe-security-groups \
  --group-ids <security-group-id> \
  --query "SecurityGroups[0].IpPermissions[?ToPort==\`30333\`]"
```

**If empty** → Add rule:
```bash
aws ec2 authorize-security-group-ingress \
  --group-id <security-group-id> \
  --protocol tcp \
  --port 30333 \
  --cidr 0.0.0.0/0
```

**Check VM iptables:**
```bash
ssh -i ~/.ssh/aws-validator ec2-user@<ip> 'sudo iptables -L INPUT -n | grep 30333'
```

---

## Common Firewall Configurations

### iptables (Most Linux)

**Check current rules:**
```bash
sudo iptables -L INPUT -n -v
```

**Check if port 30333 is open:**
```bash
sudo iptables -L INPUT -n | grep 30333
```

**Add rule if missing:**
```bash
sudo iptables -I INPUT 1 -p tcp --dport 30333 -m comment --comment "FlareChain P2P" -j ACCEPT
```

**Save rules (Ubuntu/Debian):**
```bash
sudo apt-get install -y iptables-persistent
sudo netfilter-persistent save
```

**Save rules (CentOS/RHEL):**
```bash
sudo service iptables save
```

---

### ufw (Ubuntu Firewall)

**Check status:**
```bash
sudo ufw status
```

**Allow port 30333:**
```bash
sudo ufw allow 30333/tcp
sudo ufw reload
```

**Verify:**
```bash
sudo ufw status | grep 30333
```

---

### firewalld (CentOS/Fedora)

**Check if running:**
```bash
sudo firewall-cmd --state
```

**Allow port 30333:**
```bash
sudo firewall-cmd --permanent --add-port=30333/tcp
sudo firewall-cmd --reload
```

**Verify:**
```bash
sudo firewall-cmd --list-ports
```

---

## Advanced Diagnostics

### Check if Process is Listening

```bash
sudo ss -tlnp | grep 30333
```

**✅ Should show:**
```
LISTEN 0 128 0.0.0.0:30333 0.0.0.0:* users:(("flarechain-node",pid=1234,fd=42))
```

**❌ If nothing** → Validator service not running or using wrong port

### Check Connections

```bash
sudo ss -tnp | grep 30333
```

**Shows active P2P connections to other validators**

### Packet Capture

**Test if traffic is reaching the VM:**
```bash
sudo tcpdump -i any port 30333 -n
```

**While running, try to connect from outside:**
```bash
# From another machine
nc -v <validator-ip> 30333
```

**✅ If tcpdump shows traffic** → Firewall is allowing packets
**❌ If tcpdump shows nothing** → External firewall blocking

---

## Testing Connectivity

### Test from Another Validator

```bash
# From Validator A to Validator B
ssh user@validator-a-ip 'nc -zv validator-b-ip 30333'
```

### Test from Multiple Locations

```bash
# Test from your local machine
nc -zv <validator-ip> 30333

# Test from another cloud region
ssh user@different-region-vm 'nc -zv <validator-ip> 30333'
```

---

## Fix Verification

After applying a fix, verify:

### 1. External Port Test

```bash
nc -zv <validator-ip> 30333
```

### 2. Restart Validator

```bash
ssh user@<validator-ip> 'sudo systemctl restart flarechain-validator'
```

### 3. Wait 30 Seconds

```bash
sleep 30
```

### 4. Check Peer Count

```bash
ssh user@<validator-ip> 'journalctl -u flarechain-validator -n 5 | grep peers'
```

**Expected progression:**
- 1 min: 0-2 peers
- 5 min: 5-10 peers
- 30 min: 10-20 peers

---

## Emergency Quick Fixes

### Contabo - One Liner

```bash
ssh -i ~/.ssh/contabo-validators root@<ip> 'sudo iptables -I INPUT 1 -p tcp --dport 30333 -j ACCEPT && DEBIAN_FRONTEND=noninteractive apt-get install -y iptables-persistent && netfilter-persistent save && systemctl restart flarechain-validator'
```

### Oracle Cloud - One Liner

```bash
ssh -i ~/.ssh/gizzi-validator ubuntu@<ip> 'sudo iptables -I INPUT 1 -p tcp --dport 30333 -j ACCEPT && sudo apt-get install -y iptables-persistent && sudo netfilter-persistent save && sudo systemctl restart flarechain-validator'
```

### Azure - One Liner

```bash
ssh -i ~/.ssh/azure_validator_key azureuser@<ip> 'sudo iptables -I INPUT 1 -p tcp --dport 30333 -j ACCEPT && sudo systemctl restart flarechain-validator'
```

---

## Preventing Future Issues

### 1. Document Firewall Rules

Create `/etc/flarechain/firewall-rules.txt`:
```bash
# FlareChain required ports
30333/tcp  - P2P networking (REQUIRED)
9615/tcp   - Prometheus metrics (optional)
9944/tcp   - WebSocket RPC (optional, local only)
9933/tcp   - HTTP RPC (optional, local only)
```

### 2. Use iptables-persistent

**Always install on new VMs:**
```bash
DEBIAN_FRONTEND=noninteractive apt-get install -y iptables-persistent
```

**After adding rules:**
```bash
netfilter-persistent save
```

### 3. Test Before Deploying Validator

```bash
# 1. Open port
sudo iptables -I INPUT 1 -p tcp --dport 30333 -j ACCEPT

# 2. Test from outside
nc -zv <vm-ip> 30333

# 3. If successful, make persistent
sudo netfilter-persistent save

# 4. Then deploy validator
```

---

## Firewall Rule Priority

Rules are evaluated in order. Port 30333 should be near the top:

```bash
# Insert at position 1 (highest priority)
sudo iptables -I INPUT 1 -p tcp --dport 30333 -j ACCEPT

# Verify position
sudo iptables -L INPUT -n --line-numbers | head -10
```

**Should show:**
```
Chain INPUT (policy DROP)
num  target     prot opt source               destination
1    ACCEPT     tcp  --  0.0.0.0/0            0.0.0.0/0            tcp dpt:30333
```

---

## Troubleshooting Checklist

- [ ] External port test (nc -zv)
- [ ] Check cloud provider firewall (NSG/SG/Cloud Firewall)
- [ ] Check VM iptables rules
- [ ] Check validator service is running
- [ ] Check validator is listening on 30333
- [ ] Restart validator after firewall changes
- [ ] Wait 30 seconds for peer discovery
- [ ] Check peer count in logs
- [ ] Make firewall rules persistent

---

## Get Help

If port 30333 is open but you still have 0 peers:

1. Check [Peer Discovery Troubleshooting](./PEER_DISCOVERY.md)
2. Verify genesis hash matches mainnet
3. Check network key exists
4. Review systemd service configuration

---

**Last Updated:** November 9, 2025
**Context:** Firewall issue discovered on all 20 Contabo validators
