# Automated AI Dev Monitoring Deployment

## Instructions for Claude (Terminal Session)

**COPY EVERYTHING BELOW THIS LINE AND PASTE INTO A NEW CLAUDE TERMINAL SESSION:**

---

I need you to deploy the AI Dev monitoring system to the Gizzi validator VM (64.181.215.19). This system will run 24/7 and monitor all 21 validators using Claude API.

**Context:**
- System: 12 AI dev workers using Claude API to monitor 21 blockchain validators
- Target: Gizzi validator VM (ubuntu@64.181.215.19)
- SSH Key: /Users/macbook/.ssh/gizzi-validator
- Python files: Already created in /Users/macbook/Desktop/etrid/ai-monitoring/
- validator-ips.json: /Users/macbook/Desktop/etrid/validator-ips.json

**Anthropic API Key:**
```
```

**Your tasks:**
1. Copy all files to the VM
2. Install dependencies (anthropic, paramiko, requests)
3. Set up directory structure (/opt/ai-monitoring/)
4. Create systemd service for 24/7 operation (use API key above)
5. Start the service
6. Verify it's working

**Files to deploy:**
- validator_monitor.py
- ai_dev_workers.py
- orchestrator.py
- validator-ips.json

**Important:**
- The user will provide the ANTHROPIC_API_KEY - ask for it before starting
- Use optimized mode (only call Claude when issues detected) to save costs (~$56/month instead of $560)
- Set monitoring interval to 300 seconds (5 minutes)
- Service should restart automatically if it crashes
- Log everything to /var/log/ai-dev-monitoring.log

**Configuration:**
- Prometheus URL: http://localhost:9090 (we'll set this up later if not running)
- SSH key on VM: /home/ubuntu/.ssh/gizzi-validator
- Memory log: /opt/ai-monitoring/GLOBAL_MEMORY.md
- User: ubuntu

**Steps:**

1. **Ask user for API key**
   - Tell them to get it from: https://console.anthropic.com/settings/keys
   - Explain it's separate from Claude Desktop subscription
   - Cost: ~$56/month optimized

2. **Copy files to VM:**
   ```bash
   scp -i ~/.ssh/gizzi-validator /Users/macbook/Desktop/etrid/ai-monitoring/*.py ubuntu@64.181.215.19:/tmp/
   scp -i ~/.ssh/gizzi-validator /Users/macbook/Desktop/etrid/validator-ips.json ubuntu@64.181.215.19:/tmp/
   ```

3. **SSH to VM and set up:**
   ```bash
   ssh -i ~/.ssh/gizzi-validator ubuntu@64.181.215.19
   ```

4. **On the VM, run these commands:**
   ```bash
   # Create directory
   sudo mkdir -p /opt/ai-monitoring
   sudo chown ubuntu:ubuntu /opt/ai-monitoring

   # Move files
   mv /tmp/*.py /opt/ai-monitoring/
   mv /tmp/validator-ips.json /opt/ai-monitoring/

   # Install dependencies
   sudo apt-get update -y
   sudo apt-get install -y python3-pip
   pip3 install anthropic paramiko requests

   # Set up SSH key for monitoring
   if [ ! -f ~/.ssh/gizzi-validator ]; then
     cp ~/.ssh/authorized_keys ~/.ssh/gizzi-validator
     chmod 600 ~/.ssh/gizzi-validator
   fi

   # Create memory log
   touch /opt/ai-monitoring/GLOBAL_MEMORY.md
   ```

5. **Create systemd service:**
   ```bash
   sudo tee /etc/systemd/system/ai-dev-monitoring.service > /dev/null <<'EOF'
   [Unit]
   Description=AI Dev Blockchain Monitoring
   After=network.target

   [Service]
   Type=simple
   User=ubuntu
   WorkingDirectory=/opt/ai-monitoring
   Environment="ANTHROPIC_API_KEY=PLACEHOLDER_API_KEY"
   Environment="VALIDATOR_IPS_PATH=/opt/ai-monitoring/validator-ips.json"
   Environment="SSH_KEY_PATH=/home/ubuntu/.ssh/gizzi-validator"
   Environment="PROMETHEUS_URL=http://localhost:9090"
   Environment="MEMORY_PATH=/opt/ai-monitoring/GLOBAL_MEMORY.md"
   Environment="MONITOR_INTERVAL=300"
   Environment="OPTIMIZED=true"
   ExecStart=/usr/bin/python3 /opt/ai-monitoring/orchestrator.py
   Restart=always
   RestartSec=10
   StandardOutput=append:/var/log/ai-dev-monitoring.log
   StandardError=append:/var/log/ai-dev-monitoring-error.log

   [Install]
   WantedBy=multi-user.target
   EOF
   ```

6. **Set the actual API key:**
   ```bash
   sudo sed -i "s/PLACEHOLDER_API_KEY/THE_ACTUAL_API_KEY_HERE/" /etc/systemd/system/ai-dev-monitoring.service
   ```

   Replace `THE_ACTUAL_API_KEY_HERE` with the API key the user provides.

7. **Start the service:**
   ```bash
   sudo systemctl daemon-reload
   sudo systemctl enable ai-dev-monitoring
   sudo systemctl start ai-dev-monitoring
   ```

8. **Verify it's working:**
   ```bash
   # Check status
   sudo systemctl status ai-dev-monitoring

   # Watch logs
   sudo tail -f /var/log/ai-dev-monitoring.log
   ```

   Let it run for 1-2 minutes, then press Ctrl+C.

9. **Report to user:**
   - Service status (running/failed)
   - Any errors from logs
   - How to monitor: `sudo journalctl -u ai-dev-monitoring -f`
   - How to check memory: `tail -f /opt/ai-monitoring/GLOBAL_MEMORY.md`
   - Expected cost: ~$56/month
   - Monitoring interval: Every 5 minutes

**If Prometheus is not running:**
Tell the user:
- AI dev monitoring is deployed but waiting for Prometheus
- Prometheus needed to get validator metrics
- Service will keep retrying every 5 minutes
- Once Prometheus is set up, monitoring will start automatically

**Expected output in logs:**
```
AI DEV ORCHESTRATOR - Monitoring Cycle
[governance-dev01] Starting monitoring cycle...
[governance-dev01] All 1 validators healthy
[consensus-dev01] Starting monitoring cycle...
[consensus-dev01] All 2 validators healthy
...
```

**Troubleshooting:**
- If service fails: Check `/var/log/ai-dev-monitoring-error.log`
- If API auth fails: Double-check API key is correct
- If SSH fails: Check ~/.ssh/gizzi-validator permissions (should be 600)
- If Prometheus fails: That's OK, will be set up later

**After deployment:**
Tell user how to:
1. Check logs: `sudo journalctl -u ai-dev-monitoring -f`
2. See AI decisions: `tail -f /opt/ai-monitoring/GLOBAL_MEMORY.md`
3. Restart service: `sudo systemctl restart ai-dev-monitoring`
4. Stop service: `sudo systemctl stop ai-dev-monitoring`

---

Please proceed with the deployment step by step. Ask the user for their Anthropic API key first, then continue with the deployment.
