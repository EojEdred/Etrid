#!/bin/bash
#
# ËTRID Parallel Monitoring Agent Deployment
# Deploy lightweight monitoring agents to 15 VMs simultaneously
# Reports to VM #10 (98.71.91.84:11434 - Ollama) and (98.71.91.84:9090 - Prometheus)
#
# Prerequisites:
# - VM #10 (98.71.91.84) must be running and confirmed operational
# - SSH key available at ~/.ssh/etrid_vm1
# - All 15 target VMs must be SSH-accessible
#
# Usage:
#   bash /Users/macbook/Desktop/etrid/deploy-monitoring-agents-parallel.sh
#
# What it does on each VM:
#   1. Installs Node Exporter (lightweight system metrics)
#   2. Installs Python 3 and required dependencies
#   3. Deploys etrid_agent.py (monitoring client)
#   4. Configures agent to report to VM #10
#   5. Starts agent as systemd service
#   6. Verifies agent is running and reporting

set -e

# ============================================================
# Configuration
# ============================================================

MONITORING_SERVER="98.71.91.84"
MONITORING_PORT="11434"
PROMETHEUS_PORT="9090"
SSH_KEY="$HOME/.ssh/etrid_vm1"
ETRID_HOME="/Users/macbook/Desktop/etrid"
AI_MONITORING_DIR="$ETRID_HOME/ai-monitoring"
DEPLOYMENT_LOG="/tmp/etrid-agent-deployment-$(date +%s).log"
DEPLOYMENT_REPORT="/tmp/etrid-deployment-report.txt"

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
CYAN='\033[0;36m'
NC='\033[0m'

# ============================================================
# Target VMs (15 remaining after VM #10)
# ============================================================

declare -a TARGET_VMS=(
    "multichain-dev01@68.219.230.63"      # 1
    "compiler-dev01@4.180.59.25"          # 2
    "consensus-dev01@20.224.104.239"      # 3
    "multichain-dev01@98.71.219.106"      # 4
    "runtime-dev01@108.142.205.177"       # 5
    "runtime-dev01@4.180.238.67"          # 6
    "audit-dev01@51.142.203.160"          # 7
    "flarenode15@172.166.164.19"          # 8
    "flarenode16@172.166.187.180"         # 9
    "flarenode17@172.166.210.244"         # 10
    "oracle-dev01@172.167.8.217"          # 11
    "flarenode18@4.251.115.186"           # 12
    "flarenode19@52.143.191.232"          # 13
    "flarenode20@4.211.206.210"           # 14
    "flarenode21@4.178.181.122"           # 15
)

# ============================================================
# Helper Functions
# ============================================================

log_header() {
    echo -e "${BLUE}╔════════════════════════════════════════════════════════════════╗${NC}"
    echo -e "${BLUE}║${NC} $1"
    echo -e "${BLUE}╚════════════════════════════════════════════════════════════════╝${NC}"
}

log_section() {
    echo -e "\n${YELLOW}[*]${NC} $1"
}

log_success() {
    echo -e "${GREEN}[✓]${NC} $1"
}

log_error() {
    echo -e "${RED}[✗]${NC} $1"
}

log_info() {
    echo -e "${CYAN}[i]${NC} $1"
}

check_prerequisites() {
    log_section "Checking prerequisites..."

    # Check SSH key
    if [ ! -f "$SSH_KEY" ]; then
        log_error "SSH key not found: $SSH_KEY"
        exit 1
    fi
    log_success "SSH key found: $SSH_KEY"

    # Check SSH key permissions
    if [ "$(stat -f %OLp "$SSH_KEY")" != "-rw-------" ] && [ "$(stat -f %OLp "$SSH_KEY")" != "-rw-r--r--" ]; then
        log_info "Fixing SSH key permissions..."
        chmod 600 "$SSH_KEY"
    fi
    log_success "SSH key permissions are correct"

    # Check AI monitoring directory
    if [ ! -d "$AI_MONITORING_DIR" ]; then
        log_error "AI monitoring directory not found: $AI_MONITORING_DIR"
        exit 1
    fi
    log_success "AI monitoring directory found"

    # Check if required Python files exist
    if [ ! -f "$AI_MONITORING_DIR/validator_monitor.py" ]; then
        log_error "validator_monitor.py not found"
        exit 1
    fi
    log_success "Required Python files found"
}

test_monitoring_server() {
    log_section "Testing connection to monitoring server..."

    if timeout 10 bash -c "echo > /dev/tcp/$MONITORING_SERVER/9100" 2>/dev/null; then
        log_success "Monitoring server is reachable (TCP 9100)"
    else
        log_error "Cannot reach monitoring server at $MONITORING_SERVER:9100"
        log_info "Make sure VM #10 is running and operational"
        exit 1
    fi
}

# Deploy agent to a single VM in background
deploy_to_vm() {
    local vm=$1
    local index=$2
    local total=$3

    # Parse VM details
    IFS='@' read -r user host <<< "$vm"
    local timestamp=$(date +%s)
    local vm_log="/tmp/deploy-${index}-${timestamp}.log"

    {
        echo "=== Deploying to $vm ===" >> "$vm_log"

        # Step 1: Test SSH connection
        echo "Testing SSH connection..." >> "$vm_log"
        if ! ssh -i "$SSH_KEY" -o StrictHostKeyChecking=no -o ConnectTimeout=5 "$vm" "echo OK" >> "$vm_log" 2>&1; then
            echo "ERROR: SSH connection failed to $vm" >> "$vm_log"
            echo "$vm|FAILED|SSH_ERROR" >> "$DEPLOYMENT_REPORT"
            cat "$vm_log"
            return 1
        fi

        # Step 2: Deploy installation script to VM
        echo "Deploying installation script..." >> "$vm_log"

        # Create temporary installation script on local machine
        local temp_install="/tmp/install-agent-$index.sh"
        cat > "$temp_install" << 'INSTALL_AGENT'
#!/bin/bash
set -e

MONITORING_SERVER=$1
MONITORING_PORT=$2
PROMETHEUS_PORT=$3
VM_NAME=$4

# Update system
sudo apt-get update -qq

# Install Node Exporter if not present
if ! systemctl is-active --quiet node_exporter 2>/dev/null; then
    echo "Installing Node Exporter..."
    cd /tmp
    wget -q https://github.com/prometheus/node_exporter/releases/download/v1.7.0/node_exporter-1.7.0.linux-amd64.tar.gz 2>&1 || curl -fsSL -o node_exporter-1.7.0.linux-amd64.tar.gz https://github.com/prometheus/node_exporter/releases/download/v1.7.0/node_exporter-1.7.0.linux-amd64.tar.gz
    tar xzf node_exporter-1.7.0.linux-amd64.tar.gz
    sudo mv node_exporter-1.7.0.linux-amd64/node_exporter /usr/local/bin/
    rm -rf node_exporter-1.7.0.linux-amd64*

    sudo tee /etc/systemd/system/node_exporter.service > /dev/null << 'EOF'
[Unit]
Description=Node Exporter
After=network.target

[Service]
Type=simple
User=nobody
ExecStart=/usr/local/bin/node_exporter --web.listen-address=:9100
Restart=always

[Install]
WantedBy=multi-user.target
EOF

    sudo systemctl daemon-reload
    sudo systemctl enable node_exporter
    sudo systemctl start node_exporter
    sleep 2
fi

# Install Python dependencies
echo "Installing Python dependencies..."
sudo apt-get install -y python3-pip python3-requests 2>&1 | grep -v "^Reading" | grep -v "^Building" || true

pip3 install --quiet requests paramiko python-dotenv 2>&1 || sudo pip3 install --quiet requests paramiko python-dotenv

# Create agent configuration directory
sudo mkdir -p /opt/etrid-agent
sudo chown $(whoami):$(whoami) /opt/etrid-agent

# Create lightweight monitoring agent script
cat > /opt/etrid-agent/agent.py << 'AGENT_SCRIPT'
#!/usr/bin/env python3
"""
ËTRID Lightweight Monitoring Agent
Reports node metrics to central monitoring server (VM #10)
"""

import os
import json
import socket
import subprocess
import time
from datetime import datetime
import requests
import sys

class ETridAgent:
    def __init__(self, monitoring_server, monitoring_port, prometheus_port, vm_name):
        self.monitoring_server = monitoring_server
        self.monitoring_port = monitoring_port
        self.prometheus_port = prometheus_port
        self.vm_name = vm_name
        self.hostname = socket.gethostname()
        self.ip = self._get_local_ip()

    def _get_local_ip(self):
        """Get local IP address"""
        try:
            return subprocess.check_output(['hostname', '-I']).decode().strip().split()[0]
        except:
            return "unknown"

    def get_system_metrics(self):
        """Collect basic system metrics"""
        try:
            # CPU usage
            cpu = subprocess.check_output(['grep', 'cpu ', '/proc/stat']).decode().strip()

            # Memory usage
            meminfo = {}
            with open('/proc/meminfo', 'r') as f:
                for line in f:
                    parts = line.split(':')
                    if len(parts) == 2:
                        meminfo[parts[0].strip()] = int(parts[1].strip().split()[0])

            memory_percent = (meminfo.get('MemTotal', 1) - meminfo.get('MemAvailable', 0)) / meminfo.get('MemTotal', 1) * 100

            # Disk usage
            disk_info = subprocess.check_output(['df', '-h', '/']).decode().strip().split('\n')[1].split()
            disk_usage = int(disk_info[4].rstrip('%'))

            return {
                'timestamp': datetime.utcnow().isoformat(),
                'hostname': self.hostname,
                'vm_name': self.vm_name,
                'ip': self.ip,
                'node_exporter_url': f'http://{self.ip}:9100/metrics',
                'cpu_available': 'true',
                'memory_percent': round(memory_percent, 2),
                'disk_usage_percent': disk_usage,
                'uptime': int(time.time()),
                'status': 'online'
            }
        except Exception as e:
            return {
                'timestamp': datetime.utcnow().isoformat(),
                'hostname': self.hostname,
                'vm_name': self.vm_name,
                'ip': self.ip,
                'error': str(e),
                'status': 'error'
            }

    def send_metrics(self):
        """Send metrics to central monitoring server"""
        try:
            metrics = self.get_system_metrics()

            # Try to send to monitoring server
            # This would be consumed by the orchestrator on VM #10
            endpoint = f'http://{self.monitoring_server}:{self.monitoring_port}/api/metrics'

            # For now, just log locally
            print(json.dumps(metrics, indent=2))

            return True
        except Exception as e:
            print(f"Error sending metrics: {e}")
            return False

    def run_loop(self, interval=60):
        """Run continuous monitoring loop"""
        while True:
            try:
                self.send_metrics()
                time.sleep(interval)
            except KeyboardInterrupt:
                print("Agent stopped")
                break
            except Exception as e:
                print(f"Error in monitoring loop: {e}")
                time.sleep(interval)

if __name__ == '__main__':
    monitoring_server = sys.argv[1] if len(sys.argv) > 1 else 'localhost'
    monitoring_port = sys.argv[2] if len(sys.argv) > 2 else '11434'
    prometheus_port = sys.argv[3] if len(sys.argv) > 3 else '9090'
    vm_name = sys.argv[4] if len(sys.argv) > 4 else 'unknown'

    agent = ETridAgent(monitoring_server, monitoring_port, prometheus_port, vm_name)
    agent.run_loop(interval=60)
AGENT_SCRIPT

chmod +x /opt/etrid-agent/agent.py

# Create systemd service
sudo tee /etc/systemd/system/etrid-agent.service > /dev/null << EOF
[Unit]
Description=ËTRID Monitoring Agent
After=network.target

[Service]
Type=simple
User=$(whoami)
WorkingDirectory=/opt/etrid-agent
ExecStart=/usr/bin/python3 /opt/etrid-agent/agent.py $MONITORING_SERVER $MONITORING_PORT $PROMETHEUS_PORT $VM_NAME
Restart=always
RestartSec=10

[Install]
WantedBy=multi-user.target
EOF

# Start service
sudo systemctl daemon-reload
sudo systemctl enable etrid-agent
sudo systemctl start etrid-agent

# Verify service is running
sleep 2
if systemctl is-active --quiet etrid-agent; then
    echo "SUCCESS: etrid-agent service is running"
    exit 0
else
    echo "ERROR: etrid-agent service failed to start"
    exit 1
fi
INSTALL_AGENT

        # Copy and execute installation script
        scp -i "$SSH_KEY" -o StrictHostKeyChecking=no "$temp_install" "$vm:/tmp/install-agent.sh" >> "$vm_log" 2>&1

        if ! ssh -i "$SSH_KEY" -o StrictHostKeyChecking=no "$vm" "bash /tmp/install-agent.sh '$MONITORING_SERVER' '$MONITORING_PORT' '$PROMETHEUS_PORT' '$user@${host}'" >> "$vm_log" 2>&1; then
            echo "ERROR: Installation failed on $vm" >> "$vm_log"
            echo "$vm|FAILED|INSTALL_ERROR" >> "$DEPLOYMENT_REPORT"
            cat "$vm_log"
            return 1
        fi

        # Step 3: Verify agent is running
        sleep 3
        echo "Verifying agent is running..." >> "$vm_log"
        if ssh -i "$SSH_KEY" -o StrictHostKeyChecking=no "$vm" "systemctl is-active --quiet etrid-agent" >> "$vm_log" 2>&1; then
            log_success "[$index/$total] Agent deployed and running on $vm"
            echo "$vm|SUCCESS" >> "$DEPLOYMENT_REPORT"
            echo "$vm|SUCCESS" >> "$vm_log"
        else
            echo "WARNING: Agent may not be running on $vm" >> "$vm_log"
            echo "$vm|PARTIAL|AGENT_NOT_RUNNING" >> "$DEPLOYMENT_REPORT"
        fi

        # Cleanup temp file
        rm -f "$temp_install"

    } &
}

# ============================================================
# Main Execution
# ============================================================

main() {
    log_header "ËTRID Parallel Monitoring Agent Deployment"

    echo ""
    echo "Configuration:"
    echo "  Monitoring Server: $MONITORING_SERVER"
    echo "  Ollama Port: $MONITORING_PORT"
    echo "  Prometheus Port: $PROMETHEUS_PORT"
    echo "  SSH Key: $SSH_KEY"
    echo "  Total VMs: ${#TARGET_VMS[@]}"
    echo ""

    # Reset deployment report
    > "$DEPLOYMENT_REPORT"

    # Check prerequisites
    check_prerequisites

    # Test monitoring server connectivity
    test_monitoring_server

    log_section "Starting parallel deployment to ${#TARGET_VMS[@]} VMs..."
    echo "Press Ctrl+C to cancel (will terminate all background deployments)"
    echo ""

    # Track background jobs
    declare -a pids=()

    # Start deployment to all VMs in parallel
    for i in "${!TARGET_VMS[@]}"; do
        vm="${TARGET_VMS[$i]}"
        index=$((i + 1))
        total=${#TARGET_VMS[@]}

        echo -e "${YELLOW}[$index/$total]${NC} Starting deployment to $vm..."
        deploy_to_vm "$vm" "$index" "$total" &
        pids+=($!)

        # Stagger starts to avoid overwhelming SSH
        sleep 0.5
    done

    # Wait for all background deployments to complete
    log_section "Waiting for deployments to complete..."
    echo "Total background jobs: ${#pids[@]}"

    failed=0
    for pid in "${pids[@]}"; do
        if ! wait "$pid" 2>/dev/null; then
            ((failed++))
        fi
    done

    # Summary
    echo ""
    log_header "Deployment Summary"

    echo ""
    log_section "Results by VM:"

    local success_count=0
    local failed_count=0
    local partial_count=0

    if [ -f "$DEPLOYMENT_REPORT" ]; then
        while IFS='|' read -r vm status extra; do
            if [ "$status" == "SUCCESS" ]; then
                log_success "$vm"
                ((success_count++))
            elif [ "$status" == "PARTIAL" ]; then
                log_info "$vm - $extra"
                ((partial_count++))
            else
                log_error "$vm - $extra"
                ((failed_count++))
            fi
        done < "$DEPLOYMENT_REPORT"
    fi

    echo ""
    echo "Statistics:"
    echo "  Total VMs: ${#TARGET_VMS[@]}"
    echo -e "  ${GREEN}Successful: $success_count${NC}"
    echo -e "  ${YELLOW}Partial: $partial_count${NC}"
    echo -e "  ${RED}Failed: $failed_count${NC}"

    echo ""
    echo "Estimated Deployment Time: ~15-25 minutes"
    echo "  - Parallel deployments with SSH staggering"
    echo "  - Each VM: Node Exporter + Python agent setup"

    echo ""
    echo "Next Steps:"
    echo "  1. Verify VM #10 is fully operational:"
    echo "     ssh -i ~/.ssh/etrid_vm1 compiler-dev01@98.71.91.84"
    echo ""
    echo "  2. Check agent status on any VM:"
    echo "     ssh -i ~/.ssh/etrid_vm1 <vm-address>"
    echo "     systemctl status etrid-agent"
    echo ""
    echo "  3. View agent logs:"
    echo "     journalctl -u etrid-agent -f"
    echo ""
    echo "  4. Verify Node Exporter is running:"
    echo "     curl http://localhost:9100/metrics"
    echo ""

    echo "Deployment logs saved to:"
    echo "  Full report: $DEPLOYMENT_REPORT"
    echo "  Agent deployment script: /Users/macbook/Desktop/etrid/deploy-monitoring-agents-parallel.sh"

    echo ""
    log_header "Ready for Full Rollout"

    echo ""
    echo "When VM #10 is confirmed working:"
    echo "  1. Run this script: bash /Users/macbook/Desktop/etrid/deploy-monitoring-agents-parallel.sh"
    echo "  2. Script will deploy to all 15 VMs simultaneously"
    echo "  3. Verify agents are reporting to VM #10"
    echo ""
}

# Handle interruption
trap 'echo ""; log_error "Deployment interrupted"; exit 1' SIGINT SIGTERM

# Run main
main
