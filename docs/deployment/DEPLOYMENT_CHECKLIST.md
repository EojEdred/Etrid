# ËTRID Agent Deployment Checklist

**Project:** Rolling out AI monitoring to 15 remaining VMs
**Primary Server:** VM #10 (compiler-dev01@98.71.91.84)
**Deployment Type:** Parallel (all VMs simultaneously)
**Status:** Ready to Deploy

---

## Pre-Deployment Phase

### Prerequisites Verification
- [ ] VM #10 is fully operational and confirmed working
- [ ] SSH key exists: `~/.ssh/etrid_vm1`
- [ ] SSH key has correct permissions: `chmod 600 ~/.ssh/etrid_vm1`
- [ ] Network connectivity to all 15 VMs verified
- [ ] Sufficient disk space on each VM (minimum 1 GB free)
- [ ] Internet access available on all VMs
- [ ] No firewall blocking SSH port 22 on any VM

### Files Verification
- [ ] Deployment script exists: `/Users/macbook/Desktop/etrid/deploy-monitoring-agents-parallel.sh`
- [ ] Deployment guide exists: `/Users/macbook/Desktop/etrid/AGENT_DEPLOYMENT_GUIDE.md`
- [ ] AI monitoring directory exists: `/Users/macbook/Desktop/etrid/ai-monitoring/`
- [ ] Python files present:
  - [ ] `validator_monitor.py`
  - [ ] `orchestrator.py`
  - [ ] `ai_dev_workers.py`

### VM #10 Readiness
- [ ] SSH access confirmed: `ssh -i ~/.ssh/etrid_vm1 compiler-dev01@98.71.91.84`
- [ ] Prometheus running: `systemctl status prometheus`
- [ ] Ollama running: `systemctl status ollama`
- [ ] Grafana running: `systemctl status grafana-server`
- [ ] Node Exporter running: `systemctl status node_exporter`
- [ ] Port 9090 responding: `curl http://98.71.91.84:9090`
- [ ] Port 11434 responding: `curl http://98.71.91.84:11434/api/tags`
- [ ] Port 3000 responding: `curl http://98.71.91.84:3000`

### SSH Connectivity to Target VMs
Test a sample of the 15 VMs:
- [ ] VM #1: `ssh -i ~/.ssh/etrid_vm1 multichain-dev01@68.219.230.63`
- [ ] VM #3: `ssh -i ~/.ssh/etrid_vm1 consensus-dev01@20.224.104.239`
- [ ] VM #8: `ssh -i ~/.ssh/etrid_vm1 flarenode15@172.166.164.19`
- [ ] VM #15: `ssh -i ~/.ssh/etrid_vm1 flarenode21@4.178.181.122`

---

## Deployment Phase

### Pre-Deployment Steps
- [ ] Read entire deployment guide
- [ ] Backup any critical configurations
- [ ] Plan maintenance window (if needed)
- [ ] Notify team of upcoming deployment
- [ ] Set up monitoring dashboard to watch deployment

### Running the Deployment
- [ ] Make script executable: `chmod +x deploy-monitoring-agents-parallel.sh`
- [ ] Execute script: `bash /Users/macbook/Desktop/etrid/deploy-monitoring-agents-parallel.sh`
- [ ] Monitor console output for progress
- [ ] Note any errors (will be displayed in real-time)
- [ ] Expected duration: 15-25 minutes
- [ ] Do NOT interrupt script during deployment

### During Deployment Monitoring
- [ ] Watch for SSH connection errors
- [ ] Monitor network bandwidth usage
- [ ] Check system load on local machine
- [ ] Keep SSH session active
- [ ] Do not close terminal window

---

## Post-Deployment Phase

### Immediate Verification (First 5 minutes)
- [ ] Script completes without critical errors
- [ ] Deployment report generated: `/tmp/etrid-deployment-report.txt`
- [ ] Check report summary:
  ```bash
  cat /tmp/etrid-deployment-report.txt | tail -20
  ```
- [ ] Note any failed or partial deployments

### Agent Verification on Sample VMs
For each of 3-5 sample VMs:
- [ ] SSH to VM: `ssh -i ~/.ssh/etrid_vm1 <vm-address>`
- [ ] Agent running: `systemctl status etrid-agent`
- [ ] Node Exporter running: `systemctl status node_exporter`
- [ ] Check agent logs: `journalctl -u etrid-agent -n 10`
- [ ] Exit: `exit`

Example verification commands:
```bash
# VM 1
ssh -i ~/.ssh/etrid_vm1 multichain-dev01@68.219.230.63 \
  "systemctl is-active etrid-agent && echo 'Agent OK' || echo 'Agent FAILED'"

# VM 8
ssh -i ~/.ssh/etrid_vm1 flarenode15@172.166.164.19 \
  "systemctl is-active node_exporter && echo 'Node Exporter OK'"

# VM 15
ssh -i ~/.ssh/etrid_vm1 flarenode21@4.178.181.122 \
  "curl -s http://localhost:9100/metrics | head -1 && echo 'Metrics OK'"
```

### VM #10 Dashboard Verification
- [ ] SSH to VM #10: `ssh -i ~/.ssh/etrid_vm1 compiler-dev01@98.71.91.84`
- [ ] Check GLOBAL_MEMORY: `tail -50 /opt/ai-monitoring/GLOBAL_MEMORY.md`
- [ ] Check orchestrator logs: `journalctl -u ai-dev-monitoring -n 20`
- [ ] Verify agent metrics in Prometheus:
  ```bash
  curl http://localhost:9090/api/v1/targets | jq '.data.activeTargets | length'
  ```
- [ ] Exit: `exit`

### Detailed VM Status (within 1 hour)
For ALL 15 VMs, verify:
- [ ] VM 1: Agent online
- [ ] VM 2: Agent online
- [ ] VM 3: Agent online
- [ ] VM 4: Agent online
- [ ] VM 5: Agent online
- [ ] VM 6: Agent online
- [ ] VM 7: Agent online
- [ ] VM 8: Agent online
- [ ] VM 9: Agent online
- [ ] VM 10: Agent online
- [ ] VM 11: Agent online
- [ ] VM 12: Agent online
- [ ] VM 13: Agent online
- [ ] VM 14: Agent online
- [ ] VM 15: Agent online

Quick batch check script:
```bash
VMS=(
  "multichain-dev01@68.219.230.63"
  "compiler-dev01@4.180.59.25"
  "consensus-dev01@20.224.104.239"
  # ... add all 15 ...
)

for vm in "${VMS[@]}"; do
  status=$(ssh -i ~/.ssh/etrid_vm1 "$vm" "systemctl is-active etrid-agent" 2>&1)
  if [ "$status" == "active" ]; then
    echo "✓ $vm: OK"
  else
    echo "✗ $vm: FAILED"
  fi
done
```

### Grafana Dashboard Setup (within 2 hours)
- [ ] Access Grafana: http://98.71.91.84:3000
- [ ] Login: admin/admin (change password immediately)
- [ ] Add Prometheus data source:
  - [ ] Name: "Prometheus"
  - [ ] URL: "http://localhost:9090"
  - [ ] Save & Test
- [ ] Import/create dashboards:
  - [ ] System metrics (CPU, Memory, Disk)
  - [ ] Network metrics
  - [ ] Agent health status
- [ ] Create alerts for:
  - [ ] Agent offline > 5 minutes
  - [ ] Memory > 80%
  - [ ] Disk > 80%

### API Integration Testing (within 4 hours)
- [ ] Test Ollama API:
  ```bash
  curl -X POST http://98.71.91.84:11434/api/generate \
    -d '{"model":"llama3.2","prompt":"test","stream":false}'
  ```
- [ ] Test Prometheus API:
  ```bash
  curl http://98.71.91.84:9090/api/v1/query?query=up
  ```
- [ ] Test orchestrator:
  ```bash
  ssh -i ~/.ssh/etrid_vm1 compiler-dev01@98.71.91.84 \
    "curl http://localhost:11434/api/tags | jq '.models | length'"
  ```

---

## Troubleshooting Phase

### If Any VM Failed During Deployment
- [ ] Identify which VM(s) failed from report
- [ ] Check SSH connectivity to failed VM
- [ ] Manually verify prerequisites on that VM
- [ ] Re-run deployment for just that VM:
  ```bash
  # Modify script to deploy only to failed VM
  # Or manually run install steps
  ```
- [ ] Verify successful deployment before moving on

### If Agent Not Running on a VM
- [ ] SSH to the VM
- [ ] Check systemd service:
  ```bash
  sudo systemctl status etrid-agent
  journalctl -u etrid-agent -n 50
  ```
- [ ] Verify Python installation:
  ```bash
  python3 --version
  pip3 list | grep requests
  ```
- [ ] Manually restart service:
  ```bash
  sudo systemctl restart etrid-agent
  sleep 2
  sudo systemctl status etrid-agent
  ```

### If Node Exporter Not Running
- [ ] SSH to the VM
- [ ] Check service:
  ```bash
  sudo systemctl status node_exporter
  ```
- [ ] Start if stopped:
  ```bash
  sudo systemctl start node_exporter
  sudo systemctl enable node_exporter
  ```
- [ ] Verify metrics:
  ```bash
  curl http://localhost:9100/metrics | wc -l
  ```

### If VM #10 Services Down
- [ ] SSH to VM #10
- [ ] Check each service:
  ```bash
  systemctl status prometheus
  systemctl status ollama
  systemctl status grafana-server
  systemctl status node_exporter
  ```
- [ ] Restart if needed:
  ```bash
  sudo systemctl restart <service-name>
  ```

---

## Success Criteria

Deployment is successful when:

- [ ] All 15 agents deployed without critical errors
- [ ] At least 13/15 agents show "SUCCESS" status
- [ ] Sample VM verification passes
- [ ] VM #10 reports receiving metrics from agents
- [ ] Prometheus shows targets from all agent VMs
- [ ] No errors in orchestrator logs
- [ ] Grafana dashboards display agent metrics
- [ ] Agents continue running for 30+ minutes without restart

---

## Documentation & Rollback

### Documentation
- [ ] Save deployment report: `/tmp/etrid-deployment-report.txt`
- [ ] Document any issues encountered
- [ ] Note any VMs that required manual intervention
- [ ] Create deployment summary

### Rollback Plan
- [ ] Backup: Current GLOBAL_MEMORY.md from VM #10
- [ ] If critical issues: Have rollback script ready:
  ```bash
  # For each VM:
  ssh -i ~/.ssh/etrid_vm1 <vm-address> \
    "sudo systemctl stop etrid-agent && \
     sudo systemctl disable etrid-agent && \
     sudo rm -rf /opt/etrid-agent"
  ```
- [ ] Node Exporter can remain (useful for monitoring)

---

## Post-Deployment Operations

### Daily Monitoring
- [ ] Check agent status: `curl http://98.71.91.84:9090/api/v1/targets`
- [ ] Review GLOBAL_MEMORY for AI insights: `tail -f /opt/ai-monitoring/GLOBAL_MEMORY.md`
- [ ] Monitor Grafana dashboards
- [ ] Check for any error alerts

### Weekly Checks
- [ ] Verify all 15 agents still reporting
- [ ] Review performance trends
- [ ] Check for any network issues
- [ ] Validate backup procedures

### Monthly Tasks
- [ ] Update Node Exporter versions (if needed)
- [ ] Review and optimize Prometheus retention
- [ ] Update AI monitoring scripts (if new versions available)
- [ ] Plan capacity upgrades (if needed)

---

## Sign-Off

| Role | Name | Date | Signature |
|------|------|------|-----------|
| Deployment Lead | | | |
| Verification | | | |
| Operations | | | |

---

## Notes & Comments

```
[Space for deployment notes, issues, and observations]

```
