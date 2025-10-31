#!/bin/bash
# Deploy ollama_client.py to all validators

set -e

SSH_KEY="$HOME/.ssh/gizzi-validator"

# Colors
GREEN='\033[0;32m'
BLUE='\033[0;34m'
NC='\033[0m'

echo -e "${BLUE}╔═════════════════════════════════════════════════════╗${NC}"
echo -e "${BLUE}║   Deploying Ollama Client to All Validators        ║${NC}"
echo -e "${BLUE}╚═════════════════════════════════════════════════════╝${NC}"
echo ""

# Validator list
VALIDATORS=(
    "ubuntu@64.181.215.19"           # Gizzi
    "ubuntu@129.80.122.34"           # Consensus 5
    "runtime-dev01@20.224.104.239"   # Runtime 6
    "runtime-dev01@108.142.205.177"  # Runtime 7
    "compiler-dev01@4.180.238.67"    # Compiler 8
    "compiler-dev01@4.180.59.25"     # Compiler 9
    "multichain-dev01@98.71.91.84"   # Multichain 10
    "multichain-dev01@68.219.230.63" # Multichain 11
    "oracle-dev01@98.71.219.106"     # Oracle 12
    "edsc-dev01@172.167.8.217"       # EDSC 13
    "edsc-dev01@51.142.203.160"      # EDSC 14
    "economics-dev01@172.166.164.19" # Economics 15
    "economics-dev01@172.166.187.180" # Economics 16
    "ethics-dev01@172.166.210.244"   # Ethics 17
    "ethics-dev01@4.251.115.186"     # Ethics 18
    "docs-dev01@52.143.191.232"      # Docs 19
    "docs-dev01@4.211.206.210"       # Docs 20
    "docs-dev01@4.178.181.122"       # Docs 21
)

TOTAL=${#VALIDATORS[@]}
CURRENT=0

for validator in "${VALIDATORS[@]}"; do
    CURRENT=$((CURRENT + 1))
    echo -e "${BLUE}[$CURRENT/$TOTAL]${NC} Deploying to $validator..."

    # Copy client library
    scp -i "$SSH_KEY" ollama_client.py "$validator:/tmp/"

    # Create directory and move file
    ssh -i "$SSH_KEY" "$validator" "sudo mkdir -p /opt/validator && \
                                     sudo mv /tmp/ollama_client.py /opt/validator/ && \
                                     sudo chmod +x /opt/validator/ollama_client.py && \
                                     pip3 install requests 2>/dev/null || sudo apt-get install -y python3-pip && pip3 install requests"

    echo -e "${GREEN}✅${NC} Deployed to $validator"
done

echo ""
echo -e "${GREEN}╔═════════════════════════════════════════════════════╗${NC}"
echo -e "${GREEN}║            Deployment Complete! 🎉                  ║${NC}"
echo -e "${GREEN}╚═════════════════════════════════════════════════════╝${NC}"
echo ""
echo "Test from any validator:"
echo ""
echo "  ssh -i ~/.ssh/gizzi-validator runtime-dev01@20.224.104.239"
echo "  python3 /opt/validator/ollama_client.py health 6"
echo ""
