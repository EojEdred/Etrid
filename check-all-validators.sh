#!/bin/bash
# Complete validator network status check
# Includes: Oracle, Azure, and Contabo validators

echo "╔══════════════════════════════════════════════════════════════╗"
echo "║  Ëtrid FlareChain - Complete Validator Network Status       ║"
echo "╚══════════════════════════════════════════════════════════════╝"
echo ""

TOTAL=0
ACTIVE=0

# Oracle Cloud Validators (2)
echo "ORACLE CLOUD VALIDATORS:"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"

# Gizzi Validator (Bootstrap)
TOTAL=$((TOTAL + 1))
echo -n "Validator-1 (Gizzi) - 64.181.215.19: "
STATUS=$(ssh -i ~/.ssh/gizzi-validator -o ConnectTimeout=5 -o StrictHostKeyChecking=no opc@64.181.215.19 "systemctl is-active flarechain-validator 2>&1" | head -1)
if [ "$STATUS" = "active" ]; then
    ACTIVE=$((ACTIVE + 1))
    echo "✅ ACTIVE"
else
    echo "❌ $STATUS"
fi

# Oracle Audit Dev
TOTAL=$((TOTAL + 1))
echo -n "Validator-5 (Audit Dev) - 129.80.122.34: "
STATUS=$(ssh -i ~/.ssh/gizzi-validator -o ConnectTimeout=5 -o StrictHostKeyChecking=no opc@129.80.122.34 "systemctl is-active flarechain-validator 2>&1" | head -1)
if [ "$STATUS" = "active" ]; then
    ACTIVE=$((ACTIVE + 1))
    echo "✅ ACTIVE"
else
    echo "❌ $STATUS"
fi

echo ""

# Azure Validators (3)
echo "AZURE VALIDATORS:"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"

# V0B-EojEdred
TOTAL=$((TOTAL + 1))
echo -n "Validator-2 (EojEdred) - 20.69.26.209: "
STATUS=$(ssh -i ~/.ssh/gizzi-validator -o ConnectTimeout=5 -o StrictHostKeyChecking=no azureuser@20.69.26.209 "systemctl is-active flarechain-validator 2>&1" | head -1)
if [ "$STATUS" = "active" ]; then
    ACTIVE=$((ACTIVE + 1))
    echo "✅ ACTIVE"
else
    echo "❌ $STATUS"
fi

# V1-Governance
TOTAL=$((TOTAL + 1))
echo -n "Validator-3 (Governance) - 20.186.91.207: "
STATUS=$(ssh -i ~/.ssh/gizzi-validator -o ConnectTimeout=5 -o StrictHostKeyChecking=no azureuser@20.186.91.207 "systemctl is-active flarechain-validator 2>&1" | head -1)
if [ "$STATUS" = "active" ]; then
    ACTIVE=$((ACTIVE + 1))
    echo "✅ ACTIVE"
else
    echo "❌ $STATUS"
fi

# V2-Security
TOTAL=$((TOTAL + 1))
echo -n "Validator-4 (SecurityDev) - 52.252.142.146: "
STATUS=$(ssh -i ~/.ssh/gizzi-validator -o ConnectTimeout=5 -o StrictHostKeyChecking=no azureuser@52.252.142.146 "systemctl is-active flarechain-validator 2>&1" | head -1)
if [ "$STATUS" = "active" ]; then
    ACTIVE=$((ACTIVE + 1))
    echo "✅ ACTIVE"
else
    echo "❌ $STATUS"
fi

echo ""

# Contabo Validators (16)
echo "CONTABO VALIDATORS (Seattle - 5 VMs):"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"

CONTABO_IPS=(
    "85.239.239.194:6:Seattle"
    "85.239.239.193:7:Seattle"
    "85.239.239.190:8:Seattle"
    "85.239.239.189:9:Seattle"
    "85.239.239.188:10:Seattle"
)

for validator in "${CONTABO_IPS[@]}"; do
    IFS=':' read -r ip num location <<< "$validator"
    TOTAL=$((TOTAL + 1))
    echo -n "Validator-$num - $ip: "
    STATUS=$(ssh -i ~/.ssh/contabo-validators -o ConnectTimeout=3 -o StrictHostKeyChecking=no root@$ip "systemctl is-active flarechain-validator 2>&1" | head -1)
    if [ "$STATUS" = "active" ]; then
        ACTIVE=$((ACTIVE + 1))
        echo "✅ ACTIVE"
    else
        echo "❌ $STATUS"
    fi
done

echo ""
echo "CONTABO VALIDATORS (Portsmouth - 6 VMs):"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"

CONTABO_UK_IPS=(
    "80.190.82.186:11:Portsmouth"
    "80.190.82.185:12:Portsmouth"
    "80.190.82.184:13:Portsmouth"
    "80.190.82.183:14:Portsmouth"
    "158.220.83.146:15:Portsmouth"
    "158.220.83.66:16:Portsmouth"
)

for validator in "${CONTABO_UK_IPS[@]}"; do
    IFS=':' read -r ip num location <<< "$validator"
    TOTAL=$((TOTAL + 1))
    echo -n "Validator-$num - $ip: "
    STATUS=$(ssh -i ~/.ssh/contabo-validators -o ConnectTimeout=3 -o StrictHostKeyChecking=no root@$ip "systemctl is-active flarechain-validator 2>&1" | head -1)
    if [ "$STATUS" = "active" ]; then
        ACTIVE=$((ACTIVE + 1))
        echo "✅ ACTIVE"
    else
        echo "❌ $STATUS"
    fi
done

echo ""
echo "CONTABO VALIDATORS (New York - 5 VMs):"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"

CONTABO_NY_IPS=(
    "154.12.250.18:17:NewYork"
    "154.12.250.17:18:NewYork"
    "154.12.250.15:19:NewYork"
    "154.12.249.223:20:NewYork"
    "154.12.249.182:21:NewYork"
)

for validator in "${CONTABO_NY_IPS[@]}"; do
    IFS=':' read -r ip num location <<< "$validator"
    TOTAL=$((TOTAL + 1))
    echo -n "Validator-$num - $ip: "
    STATUS=$(ssh -i ~/.ssh/contabo-validators -o ConnectTimeout=3 -o StrictHostKeyChecking=no root@$ip "systemctl is-active flarechain-validator 2>&1" | head -1)
    if [ "$STATUS" = "active" ]; then
        ACTIVE=$((ACTIVE + 1))
        echo "✅ ACTIVE"
    else
        echo "❌ $STATUS"
    fi
done

echo ""
echo "╔══════════════════════════════════════════════════════════════╗"
echo "║  SUMMARY                                                     ║"
echo "╚══════════════════════════════════════════════════════════════╝"
echo ""
echo "Total Validators:    $TOTAL/21"
echo "Active Validators:   $ACTIVE/21 ($((ACTIVE * 100 / TOTAL))%)"
echo "Required for Consensus: 15/21 (71%)"
echo ""

if [ $ACTIVE -ge 15 ]; then
    echo "Status: ✅ NETWORK CONSENSUS ACHIEVED"
else
    echo "Status: ❌ NETWORK CONSENSUS FAILED (need $((15 - ACTIVE)) more validators)"
fi

echo ""
echo "Offline Validators: $((TOTAL - ACTIVE))"
echo ""
