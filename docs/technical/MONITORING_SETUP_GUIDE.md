# Centralized Monitoring Setup for 21 FlareChain Validators

**Purpose:** Monitor all 21 validators from a single Prometheus + Grafana server

---

## Overview

```
┌────────────────────────────────┐
│  Monitoring Server (NEW VM)    │
│  - Prometheus (port 9090)      │  ← You access this
│  - Grafana (port 3000)         │  ← Dashboard UI
└────────────────────────────────┘
         ↓ Scrapes metrics
┌────────────────────────────────────────┐
│  21 Validators (port 9615 exposed)     │
│  64.181.215.19:9615 (Gizzi)            │
│  <IP2>:9615 (EojEdred)                 │
│  <IP3-21>:9615 (Others)                │
└────────────────────────────────────────┘
```

**Key Concept:** Prometheus **PULLS** metrics FROM validators. Validators don't send anything - they just expose an HTTP endpoint at port 9615.

---

## Step 1: Create Monitoring Server VM

### VM Specs

| Spec | Requirement |
|------|-------------|
| Provider | Azure/Oracle/AWS |
| vCPU | 2-4 |
| RAM | 8 GB |
| Storage | 200 GB SSD |
| OS | Ubuntu 22.04 LTS |
| Region | Any (recommend same as validators) |

### Create VM and Note IP Address

After creating the VM, you'll get:
```
Monitoring Server IP: <MONITORING_SERVER_IP>
```

Save this IP - you'll use it in NSG rules for all 21 validators.

---

## Step 2: Configure NSG Rules on ALL 21 Validators

Each validator needs to **allow inbound connections from the monitoring server** on port 9615.

### Azure NSG Rule (Per Validator)

```bash
# Replace with your actual monitoring server IP
MONITORING_SERVER_IP="<your_monitoring_server_ip>"

# For each validator's NSG (repeat 21 times or use script)
az network nsg rule create \
  --resource-group your-resource-group \
  --nsg-name validator-1-nsg \
  --name AllowPrometheusMetrics \
  --priority 500 \
  --source-address-prefixes $MONITORING_SERVER_IP \
  --destination-port-ranges 9615 \
  --protocol Tcp \
  --access Allow
```

### What This Does

- **Source IP:** Only the monitoring server can connect
- **Destination Port:** 9615 (Substrate metrics endpoint)
- **Effect:** Prometheus can scrape metrics from this validator

**Repeat for all 21 validators** (or use automation script below)

---

## Step 3: Install Monitoring Stack

### SSH into Monitoring Server

```bash
ssh ubuntu@<MONITORING_SERVER_IP>
```

### Run Installation Script

```bash
# Upload setup script
scp setup-monitoring-server.sh ubuntu@<MONITORING_SERVER_IP>:~/
ssh ubuntu@<MONITORING_SERVER_IP>

# Run installer
chmod +x setup-monitoring-server.sh
sudo ./setup-monitoring-server.sh
```

This installs:
- ✅ Prometheus (metrics collector)
- ✅ Grafana (dashboard UI)
- ✅ Node Exporter (system metrics)

---

## Step 4: Configure Prometheus

### Upload Prometheus Config

```bash
# Edit prometheus-config-21-validators.yml locally
# Replace all <VALIDATOR_XX_IP> with actual IPs

# Upload to monitoring server
scp prometheus-config-21-validators.yml ubuntu@<MONITORING_SERVER_IP>:~/

# SSH to monitoring server
ssh ubuntu@<MONITORING_SERVER_IP>

# Deploy config
sudo mkdir -p /etc/prometheus
sudo mv ~/prometheus-config-21-validators.yml /etc/prometheus/prometheus.yml
sudo chown prometheus:prometheus /etc/prometheus/prometheus.yml
```

### Start Prometheus

```bash
sudo systemctl start prometheus
sudo systemctl enable prometheus
sudo systemctl status prometheus
```

### Verify Prometheus is Scraping

```bash
# Check Prometheus logs
sudo journalctl -u prometheus -f

# Should see:
# "Server is ready to receive web requests"
```

Access Prometheus UI:
```
http://<MONITORING_SERVER_IP>:9090
```

Go to **Status > Targets** to see all 21 validators and their scrape status.

---

## Step 5: Configure Grafana

### Start Grafana

```bash
sudo systemctl start grafana-server
sudo systemctl enable grafana-server
sudo systemctl status grafana-server
```

### Access Grafana

Open in browser:
```
http://<MONITORING_SERVER_IP>:3000
```

Default login:
- Username: `admin`
- Password: `admin` (will prompt to change)

### Add Prometheus Data Source

1. Go to **Configuration** > **Data Sources**
2. Click **Add data source**
3. Select **Prometheus**
4. Set URL: `http://localhost:9090`
5. Click **Save & Test**

### Import Substrate Dashboard

1. Go to **Dashboards** > **Import**
2. Use dashboard ID: `13840` (Substrate Node Metrics)
3. Select Prometheus data source
4. Click **Import**

You'll now see metrics for all 21 validators!

---

## Step 6: Configure NSG for Monitoring Server

Allow yourself to access Prometheus and Grafana:

```bash
YOUR_IP="<your_admin_ip>"
MONITORING_NSG="monitoring-server-nsg"

# Allow SSH
az network nsg rule create \
  --resource-group $RESOURCE_GROUP \
  --nsg-name $MONITORING_NSG \
  --name AllowSSH \
  --priority 100 \
  --source-address-prefixes $YOUR_IP \
  --destination-port-ranges 22 \
  --protocol Tcp \
  --access Allow

# Allow Grafana UI
az network nsg rule create \
  --resource-group $RESOURCE_GROUP \
  --nsg-name $MONITORING_NSG \
  --name AllowGrafana \
  --priority 200 \
  --source-address-prefixes $YOUR_IP \
  --destination-port-ranges 3000 \
  --protocol Tcp \
  --access Allow

# Allow Prometheus UI (optional)
az network nsg rule create \
  --resource-group $RESOURCE_GROUP \
  --nsg-name $MONITORING_NSG \
  --name AllowPrometheus \
  --priority 300 \
  --source-address-prefixes $YOUR_IP \
  --destination-port-ranges 9090 \
  --protocol Tcp \
  --access Allow
```

---

## Automation: Bulk NSG Configuration

Create a script to configure all 21 validators at once:

```bash
#!/bin/bash
# configure-all-validator-nsgs.sh

MONITORING_SERVER_IP="<your_monitoring_server_ip>"
RESOURCE_GROUP="your-resource-group"

# Array of validator NSG names
VALIDATOR_NSGS=(
  "validator-01-nsg"
  "validator-02-nsg"
  "validator-03-nsg"
  # ... add all 21
)

for nsg in "${VALIDATOR_NSGS[@]}"; do
  echo "Configuring $nsg..."

  az network nsg rule create \
    --resource-group $RESOURCE_GROUP \
    --nsg-name $nsg \
    --name AllowPrometheusMetrics \
    --priority 500 \
    --source-address-prefixes $MONITORING_SERVER_IP \
    --destination-port-ranges 9615 \
    --protocol Tcp \
    --access Allow

  echo "✅ $nsg configured"
done

echo "✅ All 21 validators configured!"
```

---

## What You'll See in Grafana

### Key Metrics Per Validator:

- **Block Height** - Current block number
- **Finalized Block** - Last finalized block
- **Peers Connected** - Network connectivity
- **Block Production** - Blocks authored by this validator
- **CPU/Memory Usage** - System resources
- **Database Size** - Chain data growth

### Alerts You Can Set:

- Validator offline (no metrics for 1 minute)
- Block production stopped
- Less than 5 peers connected
- High CPU/memory usage
- Disk space low

---

## Updating Validator IPs

After all 21 VMs are provisioned, update the Prometheus config:

```bash
# Edit locally
nano prometheus-config-21-validators.yml

# Replace all <VALIDATOR_XX_IP> with actual IPs
# Example:
- targets: ['64.181.215.19:9615']    # Validator 1
- targets: ['<actual_ip>:9615']      # Validator 2

# Re-upload and reload
scp prometheus-config-21-validators.yml ubuntu@<MONITORING_SERVER_IP>:~/
ssh ubuntu@<MONITORING_SERVER_IP>
sudo mv ~/prometheus-config-21-validators.yml /etc/prometheus/prometheus.yml
sudo systemctl reload prometheus
```

No restart needed - Prometheus will pick up changes automatically.

---

## Summary of Network Ports

| Service | Port | Purpose | Exposed To |
|---------|------|---------|------------|
| Prometheus | 9090 | Metrics UI | Your IP only |
| Grafana | 3000 | Dashboard UI | Your IP only |
| Node Exporter | 9100 | System metrics | Localhost only |
| Validator Metrics | 9615 | Substrate metrics | Monitoring server only |

---

## Quick Reference

**Access Monitoring:**
```
Grafana:    http://<MONITORING_SERVER_IP>:3000
Prometheus: http://<MONITORING_SERVER_IP>:9090
```

**Restart Services:**
```bash
sudo systemctl restart prometheus
sudo systemctl restart grafana-server
```

**Check Status:**
```bash
sudo systemctl status prometheus
sudo systemctl status grafana-server
sudo journalctl -u prometheus -f
```

**Update Config:**
```bash
sudo nano /etc/prometheus/prometheus.yml
sudo systemctl reload prometheus
```

---

## Troubleshooting

### Validator shows as "DOWN" in Prometheus

**Check:**
1. Is the validator running? `ssh validator "sudo systemctl status flarechain-validator"`
2. Is port 9615 open in NSG? Check Azure portal
3. Can monitoring server reach validator? `telnet VALIDATOR_IP 9615`
4. Is validator exposing metrics? `curl http://VALIDATOR_IP:9615/metrics`

### Prometheus won't start

**Check logs:**
```bash
sudo journalctl -u prometheus -n 50
```

Common issues:
- Config file syntax error: Run `promtool check config /etc/prometheus/prometheus.yml`
- Permission issues: `sudo chown -R prometheus:prometheus /etc/prometheus`

### Grafana dashboard shows no data

**Check:**
1. Is Prometheus running? `sudo systemctl status prometheus`
2. Is data source configured? Grafana > Configuration > Data Sources
3. Are targets UP? Prometheus > Status > Targets

---

**Next Step:** Create monitoring server VM and get its IP address to begin setup.
