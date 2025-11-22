#!/bin/bash
# Collect peer IDs and IPs from all validators

SSH_KEY="$HOME/.ssh/contabo-validators"

VMS=(
  vmi2896906
  vmi2896907
  vmi2896908
  vmi2896909
  vmi2896910
  vmi2896911
  vmi2896914
  vmi2896915
  vmi2896916
  vmi2896917
  vmi2896918
  vmi2896921
  vmi2896922
  vmi2896923
  vmi2896924
  vmi2896925
  vmi2897381
  vmi2897382
  vmi2897383
  vmi2897384
)

echo "Collecting peer IDs and Tailscale IPs from all validators..."
echo ""

> /tmp/bootnode-list.txt

for VM in "${VMS[@]}"; do
    # Get peer ID from logs
    PEER_ID=$(ssh -i "$SSH_KEY" -o StrictHostKeyChecking=no root@$VM \
        'journalctl -u primearc-validator --since "10 minutes ago" --no-pager 2>/dev/null | grep "Local node identity is:" | tail -1 | grep -oP "12D3[a-zA-Z0-9]+"' 2>/dev/null)

    # Get Tailscale IP
    TAILSCALE_IP=$(tailscale status | grep "$VM" | awk '{print $1}')

    if [ -n "$PEER_ID" ] && [ -n "$TAILSCALE_IP" ]; then
        echo "$VM: /ip4/$TAILSCALE_IP/tcp/30333/p2p/$PEER_ID"
        echo "/ip4/$TAILSCALE_IP/tcp/30333/p2p/$PEER_ID" >> /tmp/bootnode-list.txt
    else
        echo "$VM: Failed to get peer ID or IP"
    fi
done

echo ""
echo "Bootnode list saved to /tmp/bootnode-list.txt"
echo ""
echo "First 5 bootnodes:"
head -5 /tmp/bootnode-list.txt
