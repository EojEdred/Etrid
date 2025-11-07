#!/bin/bash
# Check status of ALL 21 validators

echo "╔══════════════════════════════════════════════════════════════╗"
echo "║  Ëtrid FlareChain - Check All 21 Validators                ║"
echo "╚══════════════════════════════════════════════════════════════╝"
echo ""

check_validator() {
    local name=$1
    local ip=$2
    local ssh_key=$3
    local ssh_user=$4

    echo -n "[$name] $ip: "

    # Try to connect and check service
    timeout 10 ssh -i "$ssh_key" -o ConnectTimeout=5 -o StrictHostKeyChecking=no ${ssh_user}@${ip} "systemctl is-active flarechain-validator 2>/dev/null" 2>/dev/null
    local status=$?

    if [ $status -eq 0 ]; then
        # Get some sync info
        ssh -i "$ssh_key" ${ssh_user}@${ip} "journalctl -u flarechain-validator -n 5 --no-pager 2>/dev/null | grep -E '(Syncing|Imported|Idle)' | tail -1" 2>/dev/null | sed 's/.*flarechain-node\[.*\]: /  /'
    elif [ $status -eq 124 ]; then
        echo "⏱️  TIMEOUT (connection timed out)"
    else
        echo "❌ UNREACHABLE or SERVICE NOT RUNNING"
    fi
}

echo "═══════════════════════════════════════════════════════════════"
echo "ORACLE CLOUD VALIDATORS (2)"
echo "═══════════════════════════════════════════════════════════════"
check_validator "Validator-1 (Gizzi)" "64.181.215.19" "$HOME/.ssh/gizzi-validator" "ubuntu"
check_validator "Validator-5 (Audit)" "129.80.122.34" "$HOME/.ssh/gizzi-validator" "ubuntu"
echo ""

echo "═══════════════════════════════════════════════════════════════"
echo "AZURE VALIDATORS (3)"
echo "═══════════════════════════════════════════════════════════════"
check_validator "Validator-2 (EojEdred)" "20.69.26.209" "$HOME/.ssh/gizzi-validator" "azureuser"
check_validator "Validator-3 (governance)" "20.186.91.207" "$HOME/.ssh/gizzi-validator" "azureuser"
check_validator "Validator-4 (security)" "52.252.142.146" "$HOME/.ssh/gizzi-validator" "azureuser"
echo ""

echo "═══════════════════════════════════════════════════════════════"
echo "CONTABO US WEST - SEATTLE (5)"
echo "═══════════════════════════════════════════════════════════════"
check_validator "Validator-6" "85.239.239.194" "$HOME/.ssh/contabo-validators" "root"
check_validator "Validator-7" "85.239.239.193" "$HOME/.ssh/contabo-validators" "root"
check_validator "Validator-8" "85.239.239.190" "$HOME/.ssh/contabo-validators" "root"
check_validator "Validator-9" "85.239.239.189" "$HOME/.ssh/contabo-validators" "root"
check_validator "Validator-10" "85.239.239.188" "$HOME/.ssh/contabo-validators" "root"
echo ""

echo "═══════════════════════════════════════════════════════════════"
echo "CONTABO UNITED KINGDOM - PORTSMOUTH (6)"
echo "═══════════════════════════════════════════════════════════════"
check_validator "Validator-11" "80.190.82.186" "$HOME/.ssh/contabo-validators" "root"
check_validator "Validator-12" "80.190.82.185" "$HOME/.ssh/contabo-validators" "root"
check_validator "Validator-13" "80.190.82.184" "$HOME/.ssh/contabo-validators" "root"
check_validator "Validator-14" "80.190.82.183" "$HOME/.ssh/contabo-validators" "root"
check_validator "Validator-15" "158.220.83.146" "$HOME/.ssh/contabo-validators" "root"
check_validator "Validator-16" "158.220.83.66" "$HOME/.ssh/contabo-validators" "root"
echo ""

echo "═══════════════════════════════════════════════════════════════"
echo "CONTABO US EAST - NEW YORK (5)"
echo "═══════════════════════════════════════════════════════════════"
check_validator "Validator-17" "154.12.250.18" "$HOME/.ssh/contabo-validators" "root"
check_validator "Validator-18" "154.12.250.17" "$HOME/.ssh/contabo-validators" "root"
check_validator "Validator-19" "154.12.250.15" "$HOME/.ssh/contabo-validators" "root"
check_validator "Validator-20" "154.12.249.223" "$HOME/.ssh/contabo-validators" "root"
check_validator "Validator-21" "154.12.249.182" "$HOME/.ssh/contabo-validators" "root"
echo ""

echo "╔══════════════════════════════════════════════════════════════╗"
echo "║  Status Check Complete                                       ║"
echo "╚══════════════════════════════════════════════════════════════╝"
echo ""
echo "Summary will show how many validators are running."
echo "Consensus requires 15/21 validators (71%)"
echo ""
