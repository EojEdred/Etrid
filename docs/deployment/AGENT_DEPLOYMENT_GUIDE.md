# ËTRID Monitoring Agent Deployment Guide

**Status:** Ready for deployment (waiting for VM #10 confirmation)

## Overview

This guide covers the deployment of lightweight monitoring agents to 15 validator VMs, all reporting to the central monitoring server (VM #10 at 98.71.91.84).

## Architecture

```
┌─────────────────────────────────────────────────────────────┐
│  15 Validator VMs (Agents)                                  │
├─────────────────────────────────────────────────────────────┤
│  ├─ multichain-dev01@68.219.230.63 (Node Exporter + Agent) │
│  ├─ compiler-dev01@4.180.59.25 (Node Exporter + Agent)     │
│  ├─ consensus-dev01@20.224.104.239 (Node Exporter + Agent) │
│  └─ ... 12 more VMs ...                                     │
└──────────────────────────┬──────────────────────────────────┘
                           │
                           │ Reports metrics every 60 seconds
                           │
                           ▼
        ┌──────────────────────────────────────┐
        │   VM #10 (98.71.91.84)               │
        │   Central Monitoring Server          │
        ├──────────────────────────────────────┤
        │ • Prometheus (port 9090)             │
        │ • Grafana (port 3000)                │
        │ • Ollama (port 11434)                │
        │ • AI Orchestrator                    │
        │ • 12 AI Dev Workers                  │
        └──────────────────────────────────────┘
```

## What Gets Deployed on Each VM

### 1. Node Exporter
- **Purpose:** Collect system metrics (CPU, memory, disk, network)
- **Port:** 9100
- **Service:** `node_exporter` (systemd)
- **Metrics exposed at:** `http://<vm-ip>:9100/metrics`
- **Size:** ~10 MB
- **Memory:** ~50 MB

### 2. Python Monitoring Agent
- **File:** `/opt/etrid-agent/agent.py`
- **Purpose:** Lightweight agent that collects metrics and prepares data for VM #10
- **Reports to:** VM #10 (98.71.91.84)
- **Port:** Uses SSH + HTTP for communication
- **Service:** `etrid-agent` (systemd)
- **Interval:** Every 60 seconds
- **Memory:** ~30-50 MB

### 3. Dependencies
```
- python3-pip
- python3-requests
- Python packages: requests, paramiko, python-dotenv
```

## Deployment Script

**Location:** `/Users/macbook/Desktop/etrid/deploy-monitoring-agents-parallel.sh`

### Features

- **Parallel Deployment:** All 15 VMs deploy simultaneously
- **SSH Staggering:** Slight delays between starts to avoid overwhelming SSH
- **Automatic Retry:** Failed deployments are reported
- **Verification:** Confirms agent is running on each VM
- **Comprehensive Logging:** Detailed logs for debugging
- **Progress Tracking:** Real-time feedback during deployment

### What the Script Does

```bash
For each VM in parallel:
  1. Test SSH connectivity
  2. Deploy installation script via SCP
  3. Install Node Exporter (if not present)
  4. Install Python dependencies
  5. Create /opt/etrid-agent directory
  6. Deploy agent.py script
  7. Configure systemd service
  8. Start etrid-agent service
  9. Verify service is running
  10. Generate deployment report
```

## Prerequisites Checklist

Before running the deployment script:

- [ ] VM #10 (98.71.91.84) is fully operational
- [ ] SSH key exists: `~/.ssh/etrid_vm1`
- [ ] SSH key permissions are 600: `chmod 600 ~/.ssh/etrid_vm1`
- [ ] All 15 target VMs are SSH-accessible
- [ ] Network connectivity to all VMs
- [ ] Sufficient disk space on each VM (~1 GB free)
- [ ] Internet access on all VMs (for downloading Node Exporter)

### Verify Prerequisites

```bash
# Check SSH key
ls -la ~/.ssh/etrid_vm1

# Test SSH to VM #10
ssh -i ~/.ssh/etrid_vm1 compiler-dev01@98.71.91.84 "echo OK"

# Test SSH to a few agent VMs
ssh -i ~/.ssh/etrid_vm1 multichain-dev01@68.219.230.63 "echo OK"
ssh -i ~/.ssh/etrid_vm1 consensus-dev01@20.224.104.239 "echo OK"
```

## Target VMs (15 Total)

| # | Name | User | IP | Purpose |
|---|------|------|----|-----------  |
| 1 | multichain-dev01 | multichain-dev01 | 68.219.230.63 | Multichain |
| 2 | compiler-dev01 | compiler-dev01 | 4.180.59.25 | Compiler |
| 3 | consensus-dev01 | consensus-dev01 | 20.224.104.239 | Consensus |
| 4 | multichain-dev01 | multichain-dev01 | 98.71.219.106 | Multichain |
| 5 | runtime-dev01 | runtime-dev01 | 108.142.205.177 | Runtime |
| 6 | runtime-dev01 | runtime-dev01 | 4.180.238.67 | Runtime |
| 7 | audit-dev01 | audit-dev01 | 51.142.203.160 | Audit |
| 8 | flarenode15 | flarenode15 | 172.166.164.19 | Validator |
| 9 | flarenode16 | flarenode16 | 172.166.187.180 | Validator |
| 10 | flarenode17 | flarenode17 | 172.166.210.244 | Validator |
| 11 | oracle-dev01 | oracle-dev01 | 172.167.8.217 | Oracle |
| 12 | flarenode18 | flarenode18 | 4.251.115.186 | Validator |
| 13 | flarenode19 | flarenode19 | 52.143.191.232 | Validator |
| 14 | flarenode20 | flarenode20 | 4.211.206.210 | Validator |
| 15 | flarenode21 | flarenode21 | 4.178.181.122 | Validator |

## Deployment Steps

### Step 1: Verify VM #10 is Operational

```bash
# SSH to VM #10
ssh -i ~/.ssh/etrid_vm1 compiler-dev01@98.71.91.84

# Check if services are running
systemctl status prometheus
systemctl status ollama
systemctl status grafana-server

# Exit
exit
```

### Step 2: Run Deployment Script

```bash
# Make script executable (if not already)
chmod +x /Users/macbook/Desktop/etrid/deploy-monitoring-agents-parallel.sh

# Run the deployment
bash /Users/macbook/Desktop/etrid/deploy-monitoring-agents-parallel.sh
```

### Step 3: Monitor Deployment Progress

The script will show:
- Real-time deployment status for each VM
- Progress counter: `[current/total]`
- Color-coded results (green for success, red for failure)
- Summary at the end

### Step 4: Verify Deployment Success

```bash
# View detailed report
cat /tmp/etrid-deployment-report.txt

# Check a specific VM
ssh -i ~/.ssh/etrid_vm1 multichain-dev01@68.219.230.63
  systemctl status etrid-agent
  systemctl status node_exporter
  curl http://localhost:9100/metrics | head -20
  exit

# Check another VM
ssh -i ~/.ssh/etrid_vm1 consensus-dev01@20.224.104.239
  journalctl -u etrid-agent -n 20
  exit
```

## Estimated Deployment Time

- **Total Time:** 15-25 minutes for all 15 VMs (parallel)
- **Per VM:** 2-3 minutes (sequential, but running in parallel)
- **Network:** Depends on internet speed (downloading Node Exporter ~5 MB)
- **SSH:** Staggered to avoid overwhelming connection limits

### Timeline Breakdown

```
0:00 - Script starts
0:30 - Node Exporter installation begins on all VMs (parallel)
3:00 - Node Exporter installed on most VMs
5:00 - Python agent deployment begins
8:00 - Agent services starting on all VMs
10:00 - First metric reports arriving
15:00 - All deployments complete
20:00 - Final verification and reporting
```

## Monitoring Agent Operations

### Agent Service Management

On any VM after deployment:

```bash
# Check agent status
systemctl status etrid-agent

# View recent logs
journalctl -u etrid-agent -n 50

# View agent logs in real-time
journalctl -u etrid-agent -f

# Restart agent (if needed)
sudo systemctl restart etrid-agent

# Stop agent
sudo systemctl stop etrid-agent

# Start agent
sudo systemctl start etrid-agent
```

### Node Exporter Metrics

```bash
# Check if Node Exporter is running
systemctl status node_exporter

# View metrics being collected
curl http://localhost:9100/metrics | head -20

# Count total metrics
curl -s http://localhost:9100/metrics | grep -v '^#' | wc -l
```

### Agent Metrics Flow

```
Agent (every 60 seconds):
  1. Reads /proc/stat (CPU)
  2. Reads /proc/meminfo (Memory)
  3. Runs 'df' command (Disk)
  4. Gets local IP via hostname
  5. Checks Node Exporter availability
  6. Reports to VM #10 orchestrator
```

## Monitoring on VM #10

Once agents are deployed, they start reporting to VM #10:

### View Agent Reports

```bash
ssh -i ~/.ssh/etrid_vm1 compiler-dev01@98.71.91.84

# Check if agents are being tracked
tail -f /opt/ai-monitoring/GLOBAL_MEMORY.md

# Check Prometheus for agent metrics
curl http://localhost:9090/api/v1/targets

# View Grafana dashboards
# Access at: http://98.71.91.84:3000
# Default login: admin/admin
```

### Orchestrator Awareness of Agents

The AI Dev Orchestrator on VM #10 will:
1. Detect new metric sources as agents come online
2. Add agents to monitoring pool
3. Query agent metrics alongside validator metrics
4. Generate insights about agent health
5. Alert if agents stop reporting

## Troubleshooting

### Issue: Script fails with "SSH key not found"

```bash
# Solution: Ensure SSH key exists
ls -la ~/.ssh/etrid_vm1

# If missing, copy from wherever it's stored
cp /path/to/etrid_vm1 ~/.ssh/etrid_vm1
chmod 600 ~/.ssh/etrid_vm1
```

### Issue: Deployment hangs or times out

```bash
# Solution: Check network connectivity
ping 98.71.91.84

# Test SSH manually
ssh -i ~/.ssh/etrid_vm1 -v compiler-dev01@98.71.91.84

# Check firewall
# Ensure TCP 22 (SSH) is open on all VMs
```

### Issue: Agent not starting on a VM

```bash
# SSH to the VM
ssh -i ~/.ssh/etrid_vm1 <vm-address>

# Check agent status
systemctl status etrid-agent

# View agent error logs
journalctl -u etrid-agent -n 100

# Check if Python is installed
python3 --version

# Reinstall if needed
sudo apt-get install -y python3-pip
pip3 install requests paramiko python-dotenv

# Restart agent
sudo systemctl restart etrid-agent
```

### Issue: Node Exporter not running

```bash
# SSH to the VM
ssh -i ~/.ssh/etrid_vm1 <vm-address>

# Check service
systemctl status node_exporter

# Manually start if needed
sudo systemctl start node_exporter
sudo systemctl enable node_exporter

# Verify it's working
curl http://localhost:9100/metrics | head -10
```

### Issue: Cannot connect to VM #10

```bash
# Test connectivity to VM #10
ping 98.71.91.84

# Test Prometheus port
curl http://98.71.91.84:9090

# Test Ollama port
curl http://98.71.91.84:11434/api/tags

# If failing, VM #10 may not be ready
# Check VM #10 status:
ssh -i ~/.ssh/etrid_vm1 compiler-dev01@98.71.91.84
  sudo systemctl status prometheus
  sudo systemctl status ollama
  exit
```

## Rollback / Uninstall

If you need to remove agents from VMs:

```bash
# On each affected VM:
ssh -i ~/.ssh/etrid_vm1 <vm-address>

# Stop and disable agent
sudo systemctl stop etrid-agent
sudo systemctl disable etrid-agent

# Remove agent files
sudo rm -rf /opt/etrid-agent
sudo rm /etc/systemd/system/etrid-agent.service

# Reload systemd
sudo systemctl daemon-reload

# Note: Node Exporter can be left running
# It's useful for general system monitoring
exit
```

## Performance Impact

Expected impact on each VM:

| Metric | Impact |
|--------|--------|
| CPU Usage | +1-3% |
| Memory | +50-100 MB |
| Network | ~1 Mbps (metrics transmission) |
| Disk I/O | Minimal (~5 MB/day) |
| Disk Space | ~500 MB (Node Exporter + agent) |

## Security Considerations

1. **SSH Key:** Agent uses SSH key at `~/.ssh/etrid_vm1` (already secure)
2. **Node Exporter:** Binds to 127.0.0.1:9100 (localhost only by default)
3. **Agent Script:** Runs with standard user privileges (not root)
4. **Metrics:** No sensitive data in Node Exporter metrics
5. **Credentials:** Agent doesn't store or transmit credentials

## Next Steps After Deployment

1. **Monitor Metrics Flow:** Check that VM #10 receives metrics from all 15 agents
2. **Create Dashboards:** Use Grafana to visualize agent metrics
3. **Set Alerts:** Configure alerts for anomalies in agent health
4. **Test AI Automation:** Verify that AI dev workers can access agent metrics
5. **Document Issues:** Log any agents that behave abnormally
6. **Plan Updates:** Schedule regular updates to Node Exporter and agents

## File Locations

| Component | Location |
|-----------|----------|
| Deployment Script | `/Users/macbook/Desktop/etrid/deploy-monitoring-agents-parallel.sh` |
| This Guide | `/Users/macbook/Desktop/etrid/AGENT_DEPLOYMENT_GUIDE.md` |
| AI Monitoring | `/Users/macbook/Desktop/etrid/ai-monitoring/` |
| Orchestrator | `/Users/macbook/Desktop/etrid/ai-monitoring/orchestrator.py` |
| Validator Monitor | `/Users/macbook/Desktop/etrid/ai-monitoring/validator_monitor.py` |
| Install Script | `/Users/macbook/Desktop/etrid/install-etrid-monitoring.sh` |

## References

- **Node Exporter:** https://github.com/prometheus/node_exporter
- **Prometheus:** https://prometheus.io/
- **Ollama:** https://ollama.ai/
- **Anthropic Claude API:** https://console.anthropic.com/
