#!/bin/bash
# Test SSH Access to All Validators
# Tests connectivity to all 21 validators (18 with known IPs)

SSH_KEY="/Users/macbook/.ssh/gizzi-validator"
VALIDATORS_JSON="/Users/macbook/Desktop/etrid/validator-ips.json"

echo "========================================"
echo "Testing SSH Access to All Validators"
echo "========================================"
echo ""

# Colors for output
GREEN='\033[0;32m'
RED='\033[0;31m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Test SSH to a single validator
test_ssh() {
    local num=$1
    local name=$2
    local ip=$3

    if [ "$ip" = "NEEDS_IP" ]; then
        echo -e "${YELLOW}[SKIP] Validator $num ($name): No IP assigned${NC}"
        return 2
    fi

    # Try SSH with 5 second timeout (using SSH's built-in timeout)
    if ssh -i "$SSH_KEY" -o StrictHostKeyChecking=no -o ConnectTimeout=5 -o BatchMode=yes ubuntu@$ip "echo 'OK'" &> /dev/null; then
        echo -e "${GREEN}[OK] Validator $num ($name): $ip${NC}"
        return 0
    else
        echo -e "${RED}[FAIL] Validator $num ($name): $ip${NC}"
        return 1
    fi
}

# Parse JSON and test each validator
success_count=0
fail_count=0
skip_count=0

# Validator 1 - Gizzi
test_ssh 1 "Gizzi (AI Overseer)" "64.181.215.19"
result=$?
[ $result -eq 0 ] && ((success_count++)) || [ $result -eq 1 ] && ((fail_count++)) || ((skip_count++))

# Validator 2 - EojEdred (NEEDS_IP)
test_ssh 2 "EojEdred (Founder)" "NEEDS_IP"
result=$?
[ $result -eq 0 ] && ((success_count++)) || [ $result -eq 1 ] && ((fail_count++)) || ((skip_count++))

# Validator 3 - Audit Dev (NEEDS_IP)
test_ssh 3 "Audit Dev" "NEEDS_IP"
result=$?
[ $result -eq 0 ] && ((success_count++)) || [ $result -eq 1 ] && ((fail_count++)) || ((skip_count++))

# Validator 4 - Consensus Dev Primary (NEEDS_IP)
test_ssh 4 "Consensus Dev (Primary)" "NEEDS_IP"
result=$?
[ $result -eq 0 ] && ((success_count++)) || [ $result -eq 1 ] && ((fail_count++)) || ((skip_count++))

# Validator 5 - Consensus Dev Secondary
test_ssh 5 "Consensus Dev (Secondary)" "129.80.122.34"
result=$?
[ $result -eq 0 ] && ((success_count++)) || [ $result -eq 1 ] && ((fail_count++)) || ((skip_count++))

# Validator 6 - Runtime Dev Primary
test_ssh 6 "Runtime Dev (Primary)" "20.224.104.239"
result=$?
[ $result -eq 0 ] && ((success_count++)) || [ $result -eq 1 ] && ((fail_count++)) || ((skip_count++))

# Validator 7 - Runtime Dev Secondary
test_ssh 7 "Runtime Dev (Secondary)" "108.142.205.177"
result=$?
[ $result -eq 0 ] && ((success_count++)) || [ $result -eq 1 ] && ((fail_count++)) || ((skip_count++))

# Validator 8 - Compiler Dev Primary
test_ssh 8 "Compiler Dev (Primary)" "4.180.238.67"
result=$?
[ $result -eq 0 ] && ((success_count++)) || [ $result -eq 1 ] && ((fail_count++)) || ((skip_count++))

# Validator 9 - Compiler Dev Secondary
test_ssh 9 "Compiler Dev (Secondary)" "4.180.59.25"
result=$?
[ $result -eq 0 ] && ((success_count++)) || [ $result -eq 1 ] && ((fail_count++)) || ((skip_count++))

# Validator 10 - Multichain Dev Primary
test_ssh 10 "Multichain Dev (Primary)" "98.71.91.84"
result=$?
[ $result -eq 0 ] && ((success_count++)) || [ $result -eq 1 ] && ((fail_count++)) || ((skip_count++))

# Validator 11 - Multichain Dev Secondary
test_ssh 11 "Multichain Dev (Secondary)" "68.219.230.63"
result=$?
[ $result -eq 0 ] && ((success_count++)) || [ $result -eq 1 ] && ((fail_count++)) || ((skip_count++))

# Validator 12 - Oracle Dev
test_ssh 12 "Oracle Dev" "98.71.219.106"
result=$?
[ $result -eq 0 ] && ((success_count++)) || [ $result -eq 1 ] && ((fail_count++)) || ((skip_count++))

# Validator 13 - EDSC Dev Primary
test_ssh 13 "EDSC Dev (Primary)" "172.167.8.217"
result=$?
[ $result -eq 0 ] && ((success_count++)) || [ $result -eq 1 ] && ((fail_count++)) || ((skip_count++))

# Validator 14 - EDSC Dev Secondary
test_ssh 14 "EDSC Dev (Secondary)" "51.142.203.160"
result=$?
[ $result -eq 0 ] && ((success_count++)) || [ $result -eq 1 ] && ((fail_count++)) || ((skip_count++))

# Validator 15 - Economics Dev Primary
test_ssh 15 "Economics Dev (Primary)" "172.166.164.19"
result=$?
[ $result -eq 0 ] && ((success_count++)) || [ $result -eq 1 ] && ((fail_count++)) || ((skip_count++))

# Validator 16 - Economics Dev Secondary
test_ssh 16 "Economics Dev (Secondary)" "172.166.187.180"
result=$?
[ $result -eq 0 ] && ((success_count++)) || [ $result -eq 1 ] && ((fail_count++)) || ((skip_count++))

# Validator 17 - Ethics Dev Primary
test_ssh 17 "Ethics Dev (Primary)" "172.166.210.244"
result=$?
[ $result -eq 0 ] && ((success_count++)) || [ $result -eq 1 ] && ((fail_count++)) || ((skip_count++))

# Validator 18 - Ethics Dev Secondary
test_ssh 18 "Ethics Dev (Secondary)" "4.251.115.186"
result=$?
[ $result -eq 0 ] && ((success_count++)) || [ $result -eq 1 ] && ((fail_count++)) || ((skip_count++))

# Validator 19 - Docs Dev Primary
test_ssh 19 "Docs Dev (Primary)" "52.143.191.232"
result=$?
[ $result -eq 0 ] && ((success_count++)) || [ $result -eq 1 ] && ((fail_count++)) || ((skip_count++))

# Validator 20 - Docs Dev Secondary
test_ssh 20 "Docs Dev (Secondary)" "4.211.206.210"
result=$?
[ $result -eq 0 ] && ((success_count++)) || [ $result -eq 1 ] && ((fail_count++)) || ((skip_count++))

# Validator 21 - Docs Dev Tertiary
test_ssh 21 "Docs Dev (Tertiary)" "4.178.181.122"
result=$?
[ $result -eq 0 ] && ((success_count++)) || [ $result -eq 1 ] && ((fail_count++)) || ((skip_count++))

# Summary
echo ""
echo "========================================"
echo "SSH Test Summary"
echo "========================================"
echo -e "${GREEN}Success: $success_count${NC}"
echo -e "${RED}Failed: $fail_count${NC}"
echo -e "${YELLOW}Skipped (No IP): $skip_count${NC}"
echo ""

if [ $fail_count -gt 0 ]; then
    echo "⚠️  Some validators failed SSH test. Check firewall rules and SSH keys."
    exit 1
elif [ $skip_count -gt 0 ]; then
    echo "⚠️  Some validators need IP addresses assigned."
    echo "    Validators 2, 3, 4 are marked as NEEDS_IP"
    exit 2
else
    echo "✅ All validators with IPs are accessible!"
    exit 0
fi
