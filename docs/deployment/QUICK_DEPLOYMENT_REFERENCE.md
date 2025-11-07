# Quick Deployment Reference

**TL;DR - Agent Deployment in 5 Steps**

---

## Step 1: Verify VM #10 is Ready

```bash
# SSH to VM #10
ssh -i ~/.ssh/etrid_vm1 compiler-dev01@98.71.91.84

# Check services (all should be "active (running)")
systemctl status prometheus
systemctl status ollama
systemctl status grafana-server
systemctl status node_exporter

# Exit
exit
```

**Expected Output:** All services active
**If Issues:** Run `/opt/ai-monitoring/DEPLOYMENT_GUIDE.md` steps to fix VM #10

---

## Step 2: Test SSH to Sample VMs

```bash
# Test VM 1
ssh -i ~/.ssh/etrid_vm1 multichain-dev01@68.219.230.63 "echo OK"

# Test VM 15
ssh -i ~/.ssh/etrid_vm1 flarenode21@4.178.181.122 "echo OK"
```

**Expected Output:** `OK` (on both lines)
**If Issues:** Check SSH key and VM network connectivity

---

## Step 3: Run Deployment Script

```bash
# Execute deployment (takes 15-25 minutes)
bash /Users/macbook/Desktop/etrid/deploy-monitoring-agents-parallel.sh
```

**What to Watch For:**
- Green `[✓]` checkmarks = good
- Red `[✗]` marks = check logs
- Progress counter `[N/15]`

**Press Ctrl+C to Cancel** (will wait for current deployments)

---

## Step 4: Check Deployment Report

```bash
# View summary
cat /tmp/etrid-deployment-report.txt

# Quick stats
grep "SUCCESS\|FAILED" /tmp/etrid-deployment-report.txt | sort | uniq -c
```

**Expected Output:**
```
15 SUCCESS
```

or acceptable:
```
13 SUCCESS
1 PARTIAL
1 FAILED
```

---

## Step 5: Verify Agents are Running

```bash
# Quick batch verification (all 15 VMs)
bash << 'EOF'
VMS=(
  "multichain-dev01@68.219.230.63"
  "compiler-dev01@4.180.59.25"
  "consensus-dev01@20.224.104.239"
  "multichain-dev01@98.71.219.106"
  "runtime-dev01@108.142.205.177"
  "runtime-dev01@4.180.238.67"
  "audit-dev01@51.142.203.160"
  "flarenode15@172.166.164.19"
  "flarenode16@172.166.187.180"
  "flarenode17@172.166.210.244"
  "oracle-dev01@172.167.8.217"
  "flarenode18@4.251.115.186"
  "flarenode19@52.143.191.232"
  "flarenode20@4.211.206.210"
  "flarenode21@4.178.181.122"
)

echo "Checking agent status on all VMs..."
success=0
failed=0

for vm in "${VMS[@]}"; do
  status=$(ssh -i ~/.ssh/etrid_vm1 "$vm" "systemctl is-active etrid-agent" 2>/dev/null)
  if [ "$status" == "active" ]; then
    ((success++))
  else
    echo "✗ $vm: $status"
    ((failed++))
  fi
done

echo ""
echo "Results: $success/15 agents running"
if [ $failed -eq 0 ]; then
  echo "Status: ALL AGENTS ONLINE ✓"
else
  echo "Status: $failed agents need attention"
fi
EOF
```

---

## Deployment Complete ✓

When all 5 steps pass, deployment is ready.

---

## What Gets Deployed on Each VM

1. **Node Exporter** (~10 MB)
   - Collects system metrics (CPU, memory, disk)
   - Running on port 9100
   - Service: `node_exporter`

2. **Python Agent** (~5 MB)
   - Reports to VM #10
   - Service: `etrid-agent`
   - Interval: Every 60 seconds

3. **Dependencies**
   - python3-pip
   - requests library
   - paramiko library

---

## Common Commands After Deployment

### Check Agent Status on a VM
```bash
ssh -i ~/.ssh/etrid_vm1 <vm-address> "systemctl status etrid-agent"
```

### View Agent Logs
```bash
ssh -i ~/.ssh/etrid_vm1 <vm-address> "journalctl -u etrid-agent -n 50"
```

### Watch Agent Logs Live
```bash
ssh -i ~/.ssh/etrid_vm1 <vm-address> "journalctl -u etrid-agent -f"
```

### Restart Agent
```bash
ssh -i ~/.ssh/etrid_vm1 <vm-address> "sudo systemctl restart etrid-agent"
```

### Check Node Exporter Metrics
```bash
ssh -i ~/.ssh/etrid_vm1 <vm-address> "curl http://localhost:9100/metrics | head -10"
```

### VM #10: Check Incoming Metrics
```bash
ssh -i ~/.ssh/etrid_vm1 compiler-dev01@98.71.91.84 "curl http://localhost:9090/api/v1/targets | jq '.data.activeTargets | length'"
```

### VM #10: View AI Insights
```bash
ssh -i ~/.ssh/etrid_vm1 compiler-dev01@98.71.91.84 "tail -100 /opt/ai-monitoring/GLOBAL_MEMORY.md"
```

---

## Troubleshooting

### Agent won't start on a VM
```bash
ssh -i ~/.ssh/etrid_vm1 <vm-address>
sudo systemctl restart etrid-agent
sleep 2
systemctl status etrid-agent
exit
```

### No metrics from an agent
```bash
ssh -i ~/.ssh/etrid_vm1 <vm-address>
# Check if Node Exporter is running
systemctl status node_exporter

# Check if Python is working
python3 --version

# Check agent logs
journalctl -u etrid-agent -n 100 | tail -20
exit
```

### VM #10 not receiving metrics
```bash
ssh -i ~/.ssh/etrid_vm1 compiler-dev01@98.71.91.84

# Check Prometheus targets
curl http://localhost:9090/api/v1/targets

# Check orchestrator
systemctl status ai-dev-monitoring
journalctl -u ai-dev-monitoring -n 50

exit
```

---

## Key Files

| File | Purpose | Location |
|------|---------|----------|
| Deployment Script | Deploy agents to all 15 VMs | `/Users/macbook/Desktop/etrid/deploy-monitoring-agents-parallel.sh` |
| Detailed Guide | Full documentation | `/Users/macbook/Desktop/etrid/AGENT_DEPLOYMENT_GUIDE.md` |
| Deployment Checklist | Step-by-step checklist | `/Users/macbook/Desktop/etrid/DEPLOYMENT_CHECKLIST.md` |
| Installation Script | Universal installer | `/Users/macbook/Desktop/etrid/install-etrid-monitoring.sh` |
| Orchestrator | AI coordination | `/Users/macbook/Desktop/etrid/ai-monitoring/orchestrator.py` |

---

## Timeline

```
T+0:00   - Deployment starts
T+1:00   - Node Exporter installing (all VMs)
T+5:00   - Python agent deploying
T+10:00  - Agents starting
T+15:00  - First metrics arriving at VM #10
T+20:00  - All agents reporting
T+25:00  - Deployment complete
```

---

## Success Indicators

✓ All 15 agents show "SUCCESS" in report
✓ Sample VMs confirm agent running
✓ VM #10 receiving metrics
✓ Prometheus targets list includes all agent VMs
✓ No errors in orchestrator logs
✓ Grafana displaying agent metrics

---

## When in Doubt

1. Read: `/Users/macbook/Desktop/etrid/AGENT_DEPLOYMENT_GUIDE.md`
2. Check: `/tmp/etrid-deployment-report.txt`
3. Verify: Manual SSH to a few VMs
4. Review: Orchestrator logs on VM #10
5. Ask: Check documentation/troubleshooting section

---

## Emergency Contacts

**If Something Goes Wrong:**

1. Stop the deployment: Press Ctrl+C
2. Check logs: `cat /tmp/etrid-deployment-report.txt`
3. Verify network: `ping 98.71.91.84`
4. SSH manual test: `ssh -i ~/.ssh/etrid_vm1 compiler-dev01@98.71.91.84`
5. Review guide: Read AGENT_DEPLOYMENT_GUIDE.md

**Rollback:** Delete `/opt/etrid-agent` on affected VMs (agents stop automatically)
