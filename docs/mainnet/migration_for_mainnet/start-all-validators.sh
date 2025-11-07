#!/bin/bash
# Start all validators on Contabo VMs

set -e

SSH_KEY="$HOME/.ssh/contabo-validators"

# VM IP to Validator Number mapping
declare -a VMS=(
    "85.239.239.194:6"
    "85.239.239.193:7"
    "85.239.239.190:8"
    "85.239.239.189:9"
    "85.239.239.188:10"
    "80.190.82.186:11"
    "80.190.82.185:12"
    "80.190.82.184:13"
    "80.190.82.183:14"
    "158.220.83.146:15"
    "158.220.83.66:16"
)

echo "╔══════════════════════════════════════════════════════════════╗"
echo "║  Ëtrid FlareChain - Start All Validators                    ║"
echo "╚══════════════════════════════════════════════════════════════╝"
echo ""
echo "Starting 11 validators..."
echo ""

# Start all validators in parallel
for vm in "${VMS[@]}"; do
    IFS=':' read -r ip validator_num <<< "$vm"

    echo "[VM $ip] Starting Validator $validator_num..."
    ssh -i "$SSH_KEY" root@$ip "systemctl start flarechain-validator" &
done

# Wait for all to start
wait

echo ""
echo "Waiting 10 seconds for services to initialize..."
sleep 10
echo ""

echo "╔══════════════════════════════════════════════════════════════╗"
echo "║  Checking Validator Status                                   ║"
echo "╚══════════════════════════════════════════════════════════════╝"
echo ""

# Check status of each validator
for vm in "${VMS[@]}"; do
    IFS=':' read -r ip validator_num <<< "$vm"

    echo "Validator $validator_num ($ip):"
    ssh -i "$SSH_KEY" root@$ip bash <<'STATUS'
STATUS=$(systemctl is-active flarechain-validator)
if [ "$STATUS" = "active" ]; then
    echo "  Status: ✅ RUNNING"
    # Show last 3 log lines
    journalctl -u flarechain-validator -n 3 --no-pager 2>/dev/null | tail -3 | sed 's/^/  /'
else
    echo "  Status: ❌ NOT RUNNING ($STATUS)"
fi
STATUS
    echo ""
done

echo ""
echo "╔══════════════════════════════════════════════════════════════╗"
echo "║  All Validators Started!                                     ║"
echo "╚══════════════════════════════════════════════════════════════╝"
echo ""
echo "Next steps:"
echo "  1. Monitor sync progress: ./monitor-validators.sh"
echo "  2. Check network consensus: ./check-network-health.sh"
echo ""
echo "Important notes:"
echo "  - Validators will sync from genesis (this may take 30-60 minutes)"
echo "  - Network needs 15/21 validators for consensus"
echo "  - You also need to start the 2 Oracle validators"
echo ""
