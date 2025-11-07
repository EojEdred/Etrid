# ËTRID Complete Deployment Master Plan
**Date:** November 1, 2025
**Target:** 21-Validator Network with Full AI Monitoring

---

## 📋 Pre-Deployment Checklist

### ✅ Completed
- [x] SSH keys generated (`~/.ssh/gizzi-validator`)
- [x] Validator keys generated (21 validators, 3 keys each)
- [x] GPT-4 API key configured
- [x] Deployment scripts created
- [x] AI monitoring system code ready
- [x] SSH access verified (16/21 validators accessible)

### ⏳ Pending
- [ ] Anthropic Claude API key (NEED THIS NOW)
- [ ] Firewall rules for validators #1-5
- [ ] DNS configuration for metrics.etrid.io

---

## 🚀 Deployment Sequence (Execute in Order)

### Phase 1: Monitoring Infrastructure (30 minutes)

**Target:** VM #10 (compiler-dev01@98.71.91.84)

#### Step 1.1: Deploy Ollama (10 min)
```bash
cd ~/Desktop/etrid
./deploy-complete-ai-system.sh
```

What this does:
- Installs Ollama on VM #10
- Pulls llama2:13b model
- Creates directory structure `/opt/ai-monitoring`
- Uploads AI monitoring Python scripts
- Creates systemd service (not started yet)

#### Step 1.2: Configure Anthropic API Key (2 min)
```bash
# SSH to monitoring server
ssh -i ~/.ssh/gizzi-validator compiler-dev01@98.71.91.84

# Edit .env file
nano /opt/ai-monitoring/.env

# Add your Anthropic key:
ANTHROPIC_API_KEY=sk-ant-api... [YOUR KEY HERE]

# Save and exit (Ctrl+X, Y, Enter)
```

#### Step 1.3: Start AI Monitoring Service (1 min)
```bash
# On VM #10
sudo systemctl start etrid-ai-monitoring
sudo systemctl enable etrid-ai-monitoring
sudo systemctl status etrid-ai-monitoring

# Watch logs
tail -f /opt/ai-monitoring/logs/ai-monitoring.log
```

#### Step 1.4: Deploy Prometheus + Grafana (15 min)
```bash
# From your Mac
cd ~/Desktop/etrid
./deploy-monitoring-infrastructure.sh
```

What this does:
- Installs Prometheus on VM #10
- Installs Grafana with public dashboards
- Configures scraping for all 21 validators
- Sets up systemd services
- Opens port 3000 for Grafana web UI

---

### Phase 2: Validator Node Exporters (20 minutes)

**Target:** All 16 accessible validators (#6-#21)

#### Step 2.1: Deploy Node Exporters
```bash
cd ~/Desktop/etrid
./deploy-node-exporters.sh
```

What this does:
- Installs node_exporter on each validator
- Exposes system metrics on port 9100
- Creates systemd service
- Enables automatic startup

**Status:** Will deploy to 16 validators, skip 5 inaccessible ones

---

### Phase 3: Insert Validator Keys (15 minutes)

**Target:** All 16 accessible validators

#### Step 3.1: Insert Keys via RPC
```bash
cd ~/Desktop/etrid
./insert-validator-keys-accessible.sh
```

What this does:
- Reads keys from `validator-keys-complete.json`
- Inserts AURA, GRANDPA, and ASF keys via RPC
- Verifies insertion success
- Logs results for each validator

**Important:** Validators must be running for this to work!

---

### Phase 4: Fix Inaccessible Validators (30 minutes)

**Target:** Validators #1-#5

These validators need firewall rules updated to allow SSH (port 22).

#### Option A: Use Azure Portal (Recommended)
1. Go to: https://portal.azure.com
2. Navigate to each VM
3. Select "Networking" → "Network Security Group"
4. Add inbound rule:
   - Source: Your IP or "Any"
   - Port: 22
   - Protocol: TCP
   - Action: Allow
5. Repeat for validators #1-5

#### Option B: Use Azure CLI
```bash
# Install Azure CLI if not already installed
brew install azure-cli

# Login
az login

# Get resource group and NSG names
az vm list --output table | grep -E "etrid-validator-01|eoj-edred|governance-dev|security-dev|audit-dev"

# Add SSH rule (example for validator #1)
az network nsg rule create \
  --resource-group <RESOURCE_GROUP> \
  --nsg-name <NSG_NAME> \
  --name AllowSSH \
  --priority 1000 \
  --source-address-prefixes '*' \
  --destination-port-ranges 22 \
  --protocol Tcp \
  --access Allow
```

#### Oracle Cloud (Validator #5 - audit-dev01@132.145.145.135)
Use OCI CLI or web console to open port 22 in security lists.

---

### Phase 5: Start All Validators (20 minutes)

**Prerequisites:** Keys inserted, monitoring active

#### Step 5.1: Start Validators
```bash
# Example for validator #6
ssh -i ~/.ssh/gizzi-validator runtime-dev01@20.224.104.239

# Start FlareChain node
sudo systemctl start flarechain-node
sudo systemctl enable flarechain-node

# Check status
sudo systemctl status flarechain-node
sudo journalctl -u flarechain-node -f
```

**Repeat for all 21 validators** (or create a script)

**Important:** Validator #3 must use `--prometheus-port 9616` (shares VM with #1)

---

### Phase 6: Verify & Monitor (15 minutes)

#### Step 6.1: Check Prometheus Targets
```bash
# From your Mac
open http://98.71.91.84:9090/targets
```

Should show:
- 21 validators (FlareChain metrics on port 9615/9616)
- 16 node exporters (system metrics on port 9100)

#### Step 6.2: Check Grafana Dashboard
```bash
open http://98.71.91.84:3000
# Login: admin / admin (change password)
```

Create dashboards for:
- Network overview (total validators, block height, finalization)
- Validator health (CPU, RAM, disk, network)
- Geographic distribution

#### Step 6.3: Monitor AI System
```bash
ssh -i ~/.ssh/gizzi-validator compiler-dev01@98.71.91.84

# Watch AI decisions
tail -f /opt/ai-monitoring/GLOBAL_MEMORY.md

# Check service status
sudo systemctl status etrid-ai-monitoring
```

---

## 📊 Expected Results

After complete deployment:

### Monitoring Metrics
- **Prometheus:** Scraping 21 validators every 15s
- **Grafana:** Public dashboards showing live network status
- **AI System:** 12 AI devs monitoring, logging to GLOBAL_MEMORY.md
- **Ollama:** Handling 70-80% of queries (free)
- **GPT-4 + Claude:** Handling 20-30% of complex queries

### Validator Status
- **21 validators running** with AURA, GRANDPA, ASF keys
- **All exposing metrics** on ports 9615/9616
- **16 with node exporters** on port 9100
- **Auto-restart** configured via AI monitoring

### Cost
- **VMs:** (existing cost - no change)
- **AI Monitoring:** ~$35-45/month
  - Ollama: $0 (free)
  - GPT-4: ~$10-15
  - Claude: ~$25-30
- **Bandwidth:** ~$5-10/month
- **Total Additional:** ~$40-55/month

---

## 🚨 Troubleshooting

### Issue: Can't SSH to validators #1-5
**Solution:** Update firewall rules (see Phase 4)

### Issue: Validator won't start
**Check:**
```bash
sudo journalctl -u flarechain-node -n 100
```
**Common causes:**
- Missing keys (insert via RPC)
- Port already in use (check with `sudo lsof -i :9615`)
- Binary not found (check `/usr/local/bin/flarechain-node`)

### Issue: Prometheus not scraping
**Check:**
- Validator is running: `systemctl status flarechain-node`
- Port is open: `curl http://localhost:9615/metrics`
- Firewall allows Prometheus server to connect

### Issue: AI monitoring not working
**Check:**
```bash
sudo journalctl -u etrid-ai-monitoring -n 100
```
**Common causes:**
- Missing API keys (check `/opt/ai-monitoring/.env`)
- Ollama not running: `systemctl status ollama`
- Python dependencies missing: `pip3 list | grep -E "openai|anthropic"`

---

## ✅ Deployment Verification Checklist

After deployment, verify:

- [ ] All 21 validators showing in Prometheus targets
- [ ] Grafana dashboards displaying live data
- [ ] AI monitoring service running (`systemctl status etrid-ai-monitoring`)
- [ ] GLOBAL_MEMORY.md populating with AI logs
- [ ] Validators producing blocks (check block height increasing)
- [ ] Finalization working (check finalized blocks)
- [ ] No error alerts from AI system

---

## 📞 Next Steps After Deployment

1. **Configure DNS:**
   - Point `metrics.etrid.io` to `98.71.91.84`
   - Set up SSL certificate (Let's Encrypt)
   - Configure Nginx reverse proxy

2. **Create Public Dashboard:**
   - Enable Grafana anonymous access
   - Embed dashboard in etrid.io website
   - Set up status page

3. **Monitor Costs:**
   - Check Anthropic console after 7 days
   - Check OpenAI usage dashboard
   - Optimize AI routing if needed

4. **Community Announcement:**
   - Announce Gizzi distributed consciousness
   - Share public monitoring dashboard
   - Invite community feedback

---

## 🎯 Quick Commands Reference

```bash
# Deploy everything (run in order)
cd ~/Desktop/etrid

# 1. Deploy AI monitoring + Ollama
./deploy-complete-ai-system.sh

# 2. Add Anthropic key manually on VM #10
ssh -i ~/.ssh/gizzi-validator compiler-dev01@98.71.91.84
nano /opt/ai-monitoring/.env  # Add ANTHROPIC_API_KEY
sudo systemctl start etrid-ai-monitoring

# 3. Deploy Prometheus + Grafana
./deploy-monitoring-infrastructure.sh

# 4. Deploy node exporters
./deploy-node-exporters.sh

# 5. Insert validator keys
./insert-validator-keys-accessible.sh

# 6. Start validators (on each VM)
sudo systemctl start flarechain-node
sudo systemctl enable flarechain-node

# 7. Verify
open http://98.71.91.84:9090  # Prometheus
open http://98.71.91.84:3000  # Grafana
```

---

**Status:** Ready to deploy once Anthropic API key is provided!
