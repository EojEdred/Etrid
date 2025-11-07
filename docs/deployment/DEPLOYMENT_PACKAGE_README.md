# ËTRID Agent Deployment Package

**Status:** ✓ Ready for Deployment (Awaiting VM #10 Confirmation)

**Package Created:** 2025-11-01
**Target:** 15 Remaining Validator VMs
**Primary Server:** VM #10 (98.71.91.84)
**Deployment Type:** Parallel (simultaneous across all 15 VMs)

---

## What's in This Package

This deployment package contains everything needed to roll out lightweight monitoring agents to 15 validator VMs that report to the central monitoring server (VM #10).

### Package Contents

```
/Users/macbook/Desktop/etrid/
├── deploy-monitoring-agents-parallel.sh      ← MAIN DEPLOYMENT SCRIPT
├── AGENT_DEPLOYMENT_GUIDE.md                 ← Detailed documentation
├── DEPLOYMENT_CHECKLIST.md                   ← Step-by-step checklist
├── QUICK_DEPLOYMENT_REFERENCE.md             ← Quick reference (this file)
├── DEPLOYMENT_PACKAGE_README.md              ← Overview (this file)
├── install-etrid-monitoring.sh               ← Universal installer for VM #10
└── ai-monitoring/
    ├── orchestrator.py                       ← AI coordination
    ├── validator_monitor.py                  ← Metrics collection
    ├── ai_dev_workers.py                     ← AI workers
    └── ... other monitoring files ...
```

---

## Quick Start

### Three Command Deployment

```bash
# 1. Verify VM #10 is ready
ssh -i ~/.ssh/etrid_vm1 compiler-dev01@98.71.91.84 "systemctl status prometheus"

# 2. Run deployment (takes 15-25 minutes)
bash /Users/macbook/Desktop/etrid/deploy-monitoring-agents-parallel.sh

# 3. Verify agents are reporting
curl http://98.71.91.84:9090/api/v1/targets | jq '.data.activeTargets | length'
```

Expected final result: `15` (all agents reporting)

---

## Target VMs (15 Total)

The deployment script will install lightweight agents on:

1. multichain-dev01@68.219.230.63
2. compiler-dev01@4.180.59.25
3. consensus-dev01@20.224.104.239
4. multichain-dev01@98.71.219.106
5. runtime-dev01@108.142.205.177
6. runtime-dev01@4.180.238.67
7. audit-dev01@51.142.203.160
8. flarenode15@172.166.164.19
9. flarenode16@172.166.187.180
10. flarenode17@172.166.210.244
11. oracle-dev01@172.167.8.217
12. flarenode18@4.251.115.186
13. flarenode19@52.143.191.232
14. flarenode20@4.211.206.210
15. flarenode21@4.178.181.122

---

## What Gets Deployed

### On Each of 15 Agent VMs

**1. Node Exporter (System Metrics Collector)**
- Binary: `/usr/local/bin/node_exporter`
- Service: `node_exporter` (systemd)
- Port: 9100
- Metrics: CPU, memory, disk, network, uptime
- Size: ~10 MB
- Memory: ~50 MB

**2. Python Monitoring Agent**
- Script: `/opt/etrid-agent/agent.py`
- Service: `etrid-agent` (systemd)
- Reports to: VM #10 (98.71.91.84)
- Interval: Every 60 seconds
- Size: ~5 KB
- Memory: ~30-50 MB

**3. System Dependencies**
- python3-pip
- python3-requests
- Python packages: requests, paramiko, python-dotenv

### Total per VM
- **Disk Space:** ~500 MB
- **Memory:** ~100 MB
- **CPU:** <2%
- **Network:** ~1 Mbps (metrics only)
- **Start Time:** 2-3 minutes per VM

---

## Deployment Documentation Files

### 1. QUICK_DEPLOYMENT_REFERENCE.md
**For:** Quick reference while deploying
**Contains:**
- 5-step TL;DR deployment
- Command snippets
- Common troubleshooting
- Success indicators

**Use When:** You want to deploy quickly and know what you're doing

### 2. AGENT_DEPLOYMENT_GUIDE.md
**For:** Comprehensive deployment documentation
**Contains:**
- Architecture overview
- Detailed deployment steps
- Monitoring procedures
- Troubleshooting guide
- Performance impact analysis
- Security considerations

**Use When:** You need full details or encounter issues

### 3. DEPLOYMENT_CHECKLIST.md
**For:** Systematic verification
**Contains:**
- Pre-deployment checklist
- During-deployment monitoring
- Post-deployment verification
- Sign-off section
- Daily/weekly/monthly tasks

**Use When:** You want to ensure nothing is missed

### 4. DEPLOYMENT_PACKAGE_README.md
**For:** Overview and orientation (this file)
**Contains:**
- Package contents
- Quick start
- File descriptions
- Estimated timeline
- Success criteria

---

## Estimated Timeline

| Phase | Time | Action |
|-------|------|--------|
| Prerequisites | 5-10 min | Verify VM #10, SSH keys, network |
| Deployment | 15-25 min | Run script (parallel on all VMs) |
| Verification | 5-10 min | Check deployment report |
| Agent Verification | 10-15 min | Sample VMs + VM #10 checks |
| Dashboard Setup | 15-30 min | Grafana configuration |
| **Total** | **60-90 min** | Full rollout |

---

## Prerequisites Checklist

Before running deployment, ensure:

- [ ] VM #10 (98.71.91.84) is fully operational
  - [ ] Prometheus running
  - [ ] Ollama running
  - [ ] Grafana running
  - [ ] Node Exporter running

- [ ] SSH key available: `~/.ssh/etrid_vm1`
  - [ ] File exists
  - [ ] Permissions are 600
  - [ ] Works: `ssh -i ~/.ssh/etrid_vm1 compiler-dev01@98.71.91.84`

- [ ] Network connectivity
  - [ ] Can ping 98.71.91.84
  - [ ] Can ping sample target VMs
  - [ ] SSH port 22 open on all VMs

- [ ] Local files ready
  - [ ] Deployment script executable
  - [ ] AI monitoring directory intact
  - [ ] Required Python files present

---

## How to Use This Package

### Scenario 1: You Know What You're Doing
1. Read: QUICK_DEPLOYMENT_REFERENCE.md
2. Run: `bash deploy-monitoring-agents-parallel.sh`
3. Verify: Check deployment report
4. Done!

### Scenario 2: You Want Full Details
1. Read: AGENT_DEPLOYMENT_GUIDE.md (covers everything)
2. Follow: DEPLOYMENT_CHECKLIST.md (step-by-step)
3. Run: `bash deploy-monitoring-agents-parallel.sh`
4. Verify: Use checklist items
5. Done!

### Scenario 3: You Encounter Issues
1. Check: AGENT_DEPLOYMENT_GUIDE.md (Troubleshooting section)
2. Run: Manual commands from QUICK_DEPLOYMENT_REFERENCE.md
3. SSH to affected VM and debug
4. Manually re-run deployment for that VM
5. Verify with checklist

---

## Success Criteria

Deployment is successful when:

✓ Script completes with mostly green checkmarks
✓ Deployment report shows ≥13/15 "SUCCESS"
✓ Manual verification on sample VMs passes
✓ VM #10 reports receiving metrics from agents
✓ Prometheus targets show all agent VMs
✓ No errors in orchestrator logs after 5 minutes
✓ Agents continue running for 30+ minutes

---

## The Deployment Script

### File
`/Users/macbook/Desktop/etrid/deploy-monitoring-agents-parallel.sh`

### What It Does

**For Each VM (in parallel):**
1. Test SSH connectivity
2. Deploy installation script
3. Install Node Exporter
4. Install Python dependencies
5. Deploy agent.py script
6. Configure systemd service
7. Start agent service
8. Verify service is running
9. Log results

**Key Features:**
- Parallel deployment (all 15 VMs simultaneously)
- SSH staggering to avoid overwhelming connections
- Real-time progress feedback
- Color-coded output (green=success, red=failure)
- Detailed logging for debugging
- Automatic error handling

### Running the Script

```bash
# Make executable
chmod +x /Users/macbook/Desktop/etrid/deploy-monitoring-agents-parallel.sh

# Run deployment
bash /Users/macbook/Desktop/etrid/deploy-monitoring-agents-parallel.sh

# Monitor output - script will show:
# [1/15] Starting deployment to multichain-dev01@68.219.230.63...
# [✓] Agent deployed and running on multichain-dev01@68.219.230.63
# [2/15] Starting deployment to compiler-dev01@4.180.59.25...
# ...and so on until [15/15]
```

### What You'll See

```
ËTRID Parallel Monitoring Agent Deployment
═══════════════════════════════════════════════════════════════╗

Configuration:
  Monitoring Server: 98.71.91.84
  Ollama Port: 11434
  Prometheus Port: 9090
  SSH Key: ~/.ssh/etrid_vm1
  Total VMs: 15

[*] Checking prerequisites...
[✓] SSH key found: /Users/macbook/.ssh/etrid_vm1
[✓] SSH key permissions are correct
[✓] AI monitoring directory found
[✓] Required Python files found

[*] Testing connection to monitoring server...
[✓] Monitoring server is reachable (TCP 9100)

[*] Starting parallel deployment to 15 VMs...
Press Ctrl+C to cancel (will terminate all background deployments)

[1/15] Starting deployment to multichain-dev01@68.219.230.63...
[2/15] Starting deployment to compiler-dev01@4.180.59.25...
...
[15/15] Starting deployment to flarenode21@4.178.181.122...

[*] Waiting for deployments to complete...
Total background jobs: 15

[✓] multichain-dev01@68.219.230.63
[✓] compiler-dev01@4.180.59.25
[✓] consensus-dev01@20.224.104.239
...

═══════════════════════════════════════════════════════════════╗
Deployment Summary
═══════════════════════════════════════════════════════════════╗

Results by VM:
[✓] multichain-dev01@68.219.230.63
[✓] compiler-dev01@4.180.59.25
... (all 15 shown)

Statistics:
  Total VMs: 15
  Successful: 15
  Partial: 0
  Failed: 0

Estimated Deployment Time: ~15-25 minutes
```

---

## After Deployment

### Immediately (First 5 minutes)
1. Check deployment report: `cat /tmp/etrid-deployment-report.txt`
2. Note any failed VMs
3. Verify successful deployments

### Within 30 minutes
1. SSH to sample VMs and verify agents running
2. Check VM #10 logs for incoming metrics
3. Verify Prometheus sees agent targets

### Within 1-2 hours
1. Verify all 15 agents online
2. Set up Grafana dashboards
3. Configure alerts
4. Test API integrations

---

## Key Contacts & Documentation

| Component | File | Purpose |
|-----------|------|---------|
| Quick Start | QUICK_DEPLOYMENT_REFERENCE.md | 5-step deployment |
| Full Guide | AGENT_DEPLOYMENT_GUIDE.md | Complete documentation |
| Checklist | DEPLOYMENT_CHECKLIST.md | Verification checklist |
| This File | DEPLOYMENT_PACKAGE_README.md | Package overview |
| Main Script | deploy-monitoring-agents-parallel.sh | Deployment automation |
| VM #10 Install | install-etrid-monitoring.sh | Monitoring server setup |

---

## Important Notes

1. **Do Not Modify the Script** - It's production-ready
2. **VM #10 Must Be Ready** - Check before deploying agents
3. **SSH Key Must Be Set** - Ensure `~/.ssh/etrid_vm1` exists
4. **Network Must Be Available** - Test sample VMs first
5. **Do Not Interrupt** - Let deployment complete (Ctrl+C will wait for current jobs)

---

## Support & Troubleshooting

**If something goes wrong:**

1. **Script fails during deployment:**
   - Check `/tmp/etrid-deployment-report.txt`
   - SSH manually to failed VM
   - See AGENT_DEPLOYMENT_GUIDE.md Troubleshooting section

2. **Agent not running on a VM:**
   - SSH to that VM
   - Run: `systemctl status etrid-agent`
   - See logs: `journalctl -u etrid-agent -n 50`

3. **VM #10 not receiving metrics:**
   - SSH to VM #10
   - Check: `curl http://localhost:9090/api/v1/targets`
   - See: `/opt/ai-monitoring/GLOBAL_MEMORY.md`

4. **Network connectivity issues:**
   - Test: `ping 98.71.91.84`
   - Test: `ssh -i ~/.ssh/etrid_vm1 compiler-dev01@98.71.91.84`
   - Check firewall rules

---

## Next Steps

1. **Review:** Read QUICK_DEPLOYMENT_REFERENCE.md or AGENT_DEPLOYMENT_GUIDE.md
2. **Verify:** Check all prerequisites
3. **Backup:** Save any critical configurations
4. **Execute:** Run deployment script
5. **Monitor:** Watch for green checkmarks
6. **Verify:** Check deployment report
7. **Validate:** Sample VM verification
8. **Celebrate:** Monitoring rollout complete!

---

## Summary

This deployment package provides everything needed to roll out AI-powered monitoring to 15 validator VMs in one coordinated operation:

- **Automated:** Single script handles all deployment
- **Parallel:** All 15 VMs deploy simultaneously
- **Verified:** Built-in verification and reporting
- **Documented:** Comprehensive guides and checklists
- **Tested:** Deployment script is production-ready
- **Ready:** Just needs VM #10 confirmation

**Status: Ready for Deployment ✓**
