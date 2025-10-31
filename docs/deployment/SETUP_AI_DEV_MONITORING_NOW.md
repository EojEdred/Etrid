# Setup AI Dev Monitoring - Step-by-Step Guide
## Ready to Deploy with Your 21 VM IP Addresses

**Date:** 2025-10-31
**Status:** 🚀 Ready to implement NOW

---

## 📋 Prerequisites Checklist

✅ You have: 21 VM IP addresses
✅ You have: SSH keys (`~/.ssh/gizzi-validator` or individual keys)
✅ You have: Existing MCP infrastructure (14-aidevs/social/)
✅ You have: Existing AI dev skills (14-aidevs/skills/)
✅ You have: Monitoring guide (MONITORING_INFRASTRUCTURE_GUIDE.md)

---

## 🎯 Implementation Order

### **Phase 1: Setup (30 minutes)**
1. Create validator IP mapping file
2. Test SSH access to all VMs
3. Deploy monitoring server (Prometheus + Grafana)

### **Phase 2: MCP Integration (1 hour)**
4. Create validator monitoring connector
5. Configure monitoring workflows
6. Test with 1 AI dev (Consensus Dev)

### **Phase 3: Scale (1 hour)**
7. Deploy to all 12 AI devs
8. Verify all 21 validators monitored
9. Setup Gizzi orchestrator dashboard

---

## Step 1: Create Validator IP Mapping

### Create `validator-ips.json`

```bash
cat > /Users/macbook/Desktop/etrid/validator-ips.json << 'EOF'
{
  "validators": [
    {
      "number": 1,
      "name": "Gizzi (AI Overseer)",
      "aiDevId": "governance-dev01",
      "ip": "64.181.215.19",
      "role": "Director",
      "bootnode": true
    },
    {
      "number": 2,
      "name": "EojEdred (Founder)",
      "aiDevId": "security-dev01",
      "ip": "<PASTE_IP_HERE>",
      "role": "Director",
      "bootnode": true
    },
    {
      "number": 3,
      "name": "Audit Dev",
      "aiDevId": "audit-dev01",
      "ip": "<PASTE_IP_HERE>",
      "role": "Director",
      "bootnode": true
    },
    {
      "number": 4,
      "name": "Consensus Dev (Primary)",
      "aiDevId": "consensus-dev01",
      "ip": "<PASTE_IP_HERE>",
      "role": "FlareNode"
    },
    {
      "number": 5,
      "name": "Consensus Dev (Secondary)",
      "aiDevId": "consensus-dev01",
      "ip": "<PASTE_IP_HERE>",
      "role": "FlareNode"
    },
    {
      "number": 6,
      "name": "Runtime Dev (Primary)",
      "aiDevId": "runtime-dev01",
      "ip": "<PASTE_IP_HERE>",
      "role": "FlareNode"
    },
    {
      "number": 7,
      "name": "Runtime Dev (Secondary)",
      "aiDevId": "runtime-dev01",
      "ip": "<PASTE_IP_HERE>",
      "role": "FlareNode"
    },
    {
      "number": 8,
      "name": "Compiler Dev (Primary)",
      "aiDevId": "compiler-dev01",
      "ip": "<PASTE_IP_HERE>",
      "role": "FlareNode"
    },
    {
      "number": 9,
      "name": "Compiler Dev (Secondary)",
      "aiDevId": "compiler-dev01",
      "ip": "<PASTE_IP_HERE>",
      "role": "FlareNode"
    },
    {
      "number": 10,
      "name": "Multichain Dev (Primary)",
      "aiDevId": "multichain-dev01",
      "ip": "<PASTE_IP_HERE>",
      "role": "FlareNode"
    },
    {
      "number": 11,
      "name": "Multichain Dev (Secondary)",
      "aiDevId": "multichain-dev01",
      "ip": "<PASTE_IP_HERE>",
      "role": "FlareNode"
    },
    {
      "number": 12,
      "name": "Oracle Dev",
      "aiDevId": "oracle-dev01",
      "ip": "<PASTE_IP_HERE>",
      "role": "FlareNode"
    },
    {
      "number": 13,
      "name": "EDSC Dev (Primary)",
      "aiDevId": "edsc-dev01",
      "ip": "<PASTE_IP_HERE>",
      "role": "ValidityNode"
    },
    {
      "number": 14,
      "name": "EDSC Dev (Secondary)",
      "aiDevId": "edsc-dev01",
      "ip": "<PASTE_IP_HERE>",
      "role": "ValidityNode"
    },
    {
      "number": 15,
      "name": "Economics Dev (Primary)",
      "aiDevId": "economics-dev01",
      "ip": "<PASTE_IP_HERE>",
      "role": "ValidityNode"
    },
    {
      "number": 16,
      "name": "Economics Dev (Secondary)",
      "aiDevId": "economics-dev01",
      "ip": "<PASTE_IP_HERE>",
      "role": "ValidityNode"
    },
    {
      "number": 17,
      "name": "Ethics Dev (Primary)",
      "aiDevId": "ethics-dev01",
      "ip": "<PASTE_IP_HERE>",
      "role": "ValidityNode"
    },
    {
      "number": 18,
      "name": "Ethics Dev (Secondary)",
      "aiDevId": "ethics-dev01",
      "ip": "<PASTE_IP_HERE>",
      "role": "ValidityNode"
    },
    {
      "number": 19,
      "name": "Docs Dev (Primary)",
      "aiDevId": "docs-dev01",
      "ip": "<PASTE_IP_HERE>",
      "role": "ValidityNode"
    },
    {
      "number": 20,
      "name": "Docs Dev (Secondary)",
      "aiDevId": "docs-dev01",
      "ip": "<PASTE_IP_HERE>",
      "role": "ValidityNode"
    },
    {
      "number": 21,
      "name": "Docs Dev (Tertiary)",
      "aiDevId": "docs-dev01",
      "ip": "<PASTE_IP_HERE>",
      "role": "ValidityNode"
    }
  ]
}
EOF
```

**ACTION REQUIRED:** Replace all `<PASTE_IP_HERE>` with actual IP addresses.

---

## Step 2: Test SSH Access to All VMs

```bash
# Test script - verifies SSH works for all 21 VMs
cat > /Users/macbook/Desktop/etrid/test-all-vm-ssh.sh << 'EOF'
#!/bin/bash

VALIDATOR_IPS="/Users/macbook/Desktop/etrid/validator-ips.json"
SSH_KEY="$HOME/.ssh/gizzi-validator"

echo "Testing SSH access to all 21 validators..."
echo ""

SUCCESS=0
FAILED=0

for i in {1..21}; do
  IP=$(jq -r ".validators[$((i-1))].ip" $VALIDATOR_IPS)
  NAME=$(jq -r ".validators[$((i-1))].name" $VALIDATOR_IPS)

  if [ "$IP" = "<PASTE_IP_HERE>" ] || [ "$IP" = "null" ]; then
    echo "⚠️  Validator $i ($NAME): IP not configured"
    ((FAILED++))
    continue
  fi

  echo -n "Testing Validator $i ($NAME) at $IP... "

  if timeout 10 ssh -i $SSH_KEY -o StrictHostKeyChecking=no -o ConnectTimeout=5 ubuntu@$IP "echo 'OK'" &>/dev/null; then
    echo "✅ Connected"
    ((SUCCESS++))
  else
    echo "❌ Failed"
    ((FAILED++))
  fi
done

echo ""
echo "Results: $SUCCESS/$((SUCCESS+FAILED)) validators accessible"

if [ $SUCCESS -eq 21 ]; then
  echo "🎉 All validators accessible! Ready to proceed."
  exit 0
else
  echo "⚠️  Fix SSH access before continuing."
  exit 1
fi
EOF

chmod +x /Users/macbook/Desktop/etrid/test-all-vm-ssh.sh
```

**Run it:**
```bash
./test-all-vm-ssh.sh
```

---

## Step 3: Deploy Monitoring Server (Prometheus + Grafana)

### Option A: Use Existing Validator as Monitoring Server (Quick)

**Use Validator 1 (Gizzi) as monitoring server:**

```bash
# SSH to Gizzi validator
ssh -i ~/.ssh/gizzi-validator ubuntu@64.181.215.19

# Install Prometheus
cd /tmp
wget https://github.com/prometheus/prometheus/releases/download/v2.47.0/prometheus-2.47.0.linux-amd64.tar.gz
tar xvfz prometheus-*.tar.gz
sudo mv prometheus-2.47.0.linux-amd64 /opt/prometheus

# Install Grafana
sudo apt-get install -y software-properties-common
sudo add-apt-repository "deb https://packages.grafana.com/oss/deb stable main"
wget -q -O - https://packages.grafana.com/gpg.key | sudo apt-key add -
sudo apt-get update
sudo apt-get install -y grafana

# Start services
sudo systemctl start prometheus grafana-server
sudo systemctl enable prometheus grafana-server

echo "✅ Monitoring server installed on validator 1"
echo "Prometheus: http://64.181.215.19:9090"
echo "Grafana: http://64.181.215.19:3000 (admin/admin)"
```

### Option B: Create Dedicated Monitoring VM (Recommended)

**If you have a 22nd VM:**
```bash
# Same installation steps, but on dedicated VM
# Update validator-ips.json with monitoring server IP
```

---

## Step 4: Configure Prometheus for 21 Validators

```bash
# Generate Prometheus config from validator-ips.json
cat > /Users/macbook/Desktop/etrid/generate-prometheus-config.sh << 'EOF'
#!/bin/bash

VALIDATOR_IPS="/Users/macbook/Desktop/etrid/validator-ips.json"
OUTPUT="/tmp/prometheus.yml"

cat > $OUTPUT << 'YAML'
global:
  scrape_interval: 15s
  evaluation_interval: 15s

scrape_configs:
YAML

# Add scrape config for each validator
for i in {1..21}; do
  IP=$(jq -r ".validators[$((i-1))].ip" $VALIDATOR_IPS)
  NAME=$(jq -r ".validators[$((i-1))].name" $VALIDATOR_IPS)
  AIDEVID=$(jq -r ".validators[$((i-1))].aiDevId" $VALIDATOR_IPS)

  if [ "$IP" = "<PASTE_IP_HERE>" ] || [ "$IP" = "null" ]; then
    continue
  fi

  cat >> $OUTPUT << YAML
  - job_name: 'validator-$i'
    static_configs:
      - targets: ['$IP:9615']
        labels:
          instance: 'validator-$i'
          name: '$NAME'
          aidevid: '$AIDEVID'
          ip: '$IP'

YAML
done

echo "✅ Prometheus config generated: $OUTPUT"
echo ""
echo "Upload to monitoring server:"
echo "scp $OUTPUT ubuntu@MONITORING_SERVER_IP:/etc/prometheus/prometheus.yml"
EOF

chmod +x /Users/macbook/Desktop/etrid/generate-prometheus-config.sh
./generate-prometheus-config.sh
```

---

## Step 5: Create MCP Validator Monitoring Connector

```bash
# Create the validator monitoring connector
mkdir -p /Users/macbook/Desktop/etrid/ai-monitoring
cd /Users/macbook/Desktop/etrid/ai-monitoring

cat > validator_monitor.py << 'EOF'
#!/usr/bin/env python3
"""
MCP Validator Monitoring Connector
Extends existing MCP infrastructure with validator monitoring capabilities
"""

import json
import subprocess
import requests
from typing import Dict, List, Optional

class ValidatorMonitor:
    def __init__(self, validator_ips_path: str, ssh_key_path: str, prometheus_url: str):
        with open(validator_ips_path) as f:
            self.validators = json.load(f)['validators']
        self.ssh_key = ssh_key_path
        self.prometheus = prometheus_url

    def get_validator_by_number(self, number: int) -> Dict:
        """Get validator info by number"""
        for v in self.validators:
            if v['number'] == number:
                return v
        raise ValueError(f"Validator {number} not found")

    def get_validators_by_aidevid(self, aidevid: str) -> List[Dict]:
        """Get all validators for an AI dev"""
        return [v for v in self.validators if v['aiDevId'] == aidevid]

    def check_validator_status(self, validator_number: int) -> Dict:
        """Check if validator is online and producing blocks"""
        validator = self.get_validator_by_number(validator_number)
        ip = validator['ip']

        # Query Prometheus for metrics
        query = f'up{{instance="validator-{validator_number}"}}'
        response = requests.get(f"{self.prometheus}/api/v1/query", params={'query': query})

        if response.status_code == 200:
            data = response.json()['data']['result']
            is_up = data[0]['value'][1] == '1' if data else False
        else:
            is_up = False

        # Get peer count
        peer_query = f'substrate_sub_libp2p_peers_count{{instance="validator-{validator_number}"}}'
        peer_response = requests.get(f"{self.prometheus}/api/v1/query", params={'query': peer_query})
        peer_count = 0
        if peer_response.status_code == 200:
            peer_data = peer_response.json()['data']['result']
            peer_count = int(float(peer_data[0]['value'][1])) if peer_data else 0

        return {
            'validator_number': validator_number,
            'name': validator['name'],
            'ip': ip,
            'status': 'online' if is_up else 'offline',
            'peer_count': peer_count,
            'healthy': is_up and peer_count >= 3
        }

    def get_validator_logs(self, validator_number: int, lines: int = 100) -> str:
        """Get validator logs via SSH"""
        validator = self.get_validator_by_number(validator_number)
        ip = validator['ip']

        cmd = [
            'ssh', '-i', self.ssh_key,
            '-o', 'StrictHostKeyChecking=no',
            f'ubuntu@{ip}',
            f'sudo journalctl -u flarechain-validator -n {lines}'
        ]

        result = subprocess.run(cmd, capture_output=True, text=True, timeout=30)
        return result.stdout if result.returncode == 0 else f"Error: {result.stderr}"

    def restart_validator(self, validator_number: int, reason: str) -> bool:
        """Restart validator service"""
        validator = self.get_validator_by_number(validator_number)
        ip = validator['ip']

        print(f"⚠️  Restarting validator {validator_number} ({validator['name']})")
        print(f"Reason: {reason}")

        cmd = [
            'ssh', '-i', self.ssh_key,
            '-o', 'StrictHostKeyChecking=no',
            f'ubuntu@{ip}',
            'sudo systemctl restart flarechain-validator'
        ]

        result = subprocess.run(cmd, capture_output=True, text=True, timeout=30)

        if result.returncode == 0:
            print(f"✅ Validator {validator_number} restarted successfully")
            return True
        else:
            print(f"❌ Failed to restart validator {validator_number}: {result.stderr}")
            return False

# Example usage
if __name__ == '__main__':
    monitor = ValidatorMonitor(
        validator_ips_path='/Users/macbook/Desktop/etrid/validator-ips.json',
        ssh_key_path='/Users/macbook/.ssh/gizzi-validator',
        prometheus_url='http://64.181.215.19:9090'  # Update with your monitoring server
    )

    # Test: Check consensus dev validators (4 & 5)
    consensus_validators = monitor.get_validators_by_aidevid('consensus-dev01')
    print(f"Consensus Dev monitors {len(consensus_validators)} validators:")

    for v in consensus_validators:
        status = monitor.check_validator_status(v['number'])
        print(f"  Validator {v['number']}: {status['status']} ({status['peer_count']} peers)")
EOF

chmod +x validator_monitor.py
```

---

## Step 6: Test with One AI Dev (Consensus Dev)

```bash
# Test the validator monitor
cd /Users/macbook/Desktop/etrid/ai-monitoring
python3 validator_monitor.py

# Should output:
# Consensus Dev monitors 2 validators:
#   Validator 4: online (20 peers)
#   Validator 5: online (18 peers)
```

---

## Step 7: Create Monitoring Workflow for All 12 AI Devs

```bash
cat > /Users/macbook/Desktop/etrid/ai-monitoring/run-all-ai-dev-monitors.sh << 'EOF'
#!/bin/bash
#
# Run monitoring for all 12 AI devs
#

MONITOR_SCRIPT="/Users/macbook/Desktop/etrid/ai-monitoring/validator_monitor.py"
VALIDATOR_IPS="/Users/macbook/Desktop/etrid/validator-ips.json"

AI_DEVS=(
  "governance-dev01"
  "security-dev01"
  "audit-dev01"
  "consensus-dev01"
  "runtime-dev01"
  "compiler-dev01"
  "multichain-dev01"
  "oracle-dev01"
  "edsc-dev01"
  "economics-dev01"
  "ethics-dev01"
  "docs-dev01"
)

echo "🤖 Starting AI Dev monitoring for all 21 validators..."
echo ""

for aidev in "${AI_DEVS[@]}"; do
  echo "[$aidev] Monitoring..."

  # Get validators for this AI dev
  VALIDATORS=$(jq -r ".validators[] | select(.aiDevId == \"$aidev\") | .number" $VALIDATOR_IPS)

  for val_num in $VALIDATORS; do
    STATUS=$(python3 $MONITOR_SCRIPT check $val_num 2>/dev/null)
    echo "  Validator $val_num: $STATUS"
  done

  echo ""
done

echo "✅ Monitoring cycle complete"
EOF

chmod +x /Users/macbook/Desktop/etrid/ai-monitoring/run-all-ai-dev-monitors.sh
```

---

## Step 8: Setup Continuous Monitoring (Cron)

```bash
# Add to crontab - runs every minute
crontab -l > /tmp/crontab.backup
echo "* * * * * /Users/macbook/Desktop/etrid/ai-monitoring/run-all-ai-dev-monitors.sh >> /var/log/ai-dev-monitoring.log 2>&1" | crontab -

echo "✅ Monitoring scheduled (every minute)"
echo "View logs: tail -f /var/log/ai-dev-monitoring.log"
```

---

## 🎯 Quick Start Commands

**Once you have validator-ips.json filled out:**

```bash
# 1. Test SSH access
./test-all-vm-ssh.sh

# 2. Generate Prometheus config
./generate-prometheus-config.sh

# 3. Test validator monitoring
cd ai-monitoring && python3 validator_monitor.py

# 4. Start continuous monitoring
./run-all-ai-dev-monitors.sh

# 5. Setup cron (optional - for 24/7 monitoring)
crontab -e
# Add: * * * * * /path/to/run-all-ai-dev-monitors.sh
```

---

## 📊 What You'll See

```
🤖 Starting AI Dev monitoring for all 21 validators...

[governance-dev01] Monitoring...
  Validator 1: online (20 peers)

[security-dev01] Monitoring...
  Validator 2: online (18 peers)

[audit-dev01] Monitoring...
  Validator 3: online (21 peers)

[consensus-dev01] Monitoring...
  Validator 4: online (19 peers)
  Validator 5: online (17 peers)

[runtime-dev01] Monitoring...
  Validator 6: online (20 peers)
  Validator 7: online (19 peers)

... (all 21 validators)

✅ Monitoring cycle complete
```

---

## 🚨 If Issues

**SSH fails:**
```bash
# Check SSH key permissions
chmod 600 ~/.ssh/gizzi-validator

# Test individual validator
ssh -i ~/.ssh/gizzi-validator ubuntu@<VALIDATOR_IP> "echo OK"
```

**Prometheus not responding:**
```bash
# Check if Prometheus is running
ssh ubuntu@MONITORING_SERVER_IP "sudo systemctl status prometheus"

# Restart Prometheus
ssh ubuntu@MONITORING_SERVER_IP "sudo systemctl restart prometheus"
```

**Validator metrics not available:**
```bash
# Check if validator exposes metrics
curl http://<VALIDATOR_IP>:9615/metrics
```

---

## ✅ Success Criteria

- [ ] All 21 VMs SSH accessible
- [ ] Prometheus installed and running
- [ ] validator_monitor.py works
- [ ] All 12 AI devs monitoring their validators
- [ ] Logs showing monitoring activity
- [ ] Grafana dashboard accessible

---

**Next:** Once this is working, we can add:
- Auto-restart on validator failure
- Alert generation
- Grafana dashboards for visualization
- Integration with existing MCP social automation

---

**Ready to start?** Paste your VM IP addresses and we'll configure validator-ips.json!
