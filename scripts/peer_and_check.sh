#!/bin/bash

# Peer VMs and Check Health Script
# Peers all active VMs together and reports block height/finalization

SSH_USER="root"
SSH_PASS="G1zzi!Pwr2025$"
SSH_OPTS="-o StrictHostKeyChecking=no -o ConnectTimeout=10 -o UserKnownHostsFile=/dev/null"

# Active VMs (Excluded failed ones)
ACTIVE_VMS=(
    "158.220.83.66" "158.220.83.146"
    "80.190.82.183" "80.190.82.186" "85.239.239.188" "85.239.239.189" "85.239.239.190" "85.239.239.193"
    "85.239.239.194" "154.12.249.182" "154.12.249.223" "154.12.250.15" "154.12.250.18"
    "157.173.200.80" "157.173.200.84" "157.173.200.81"
)

# Associative array to store Peer IDs
declare -A PEER_IDS

echo "🔍 Fetching Peer IDs from ${#ACTIVE_VMS[@]} nodes..."
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"

for ip in "${ACTIVE_VMS[@]}"; do
    echo -n "Fetching ID for $ip... "
    response=$(sshpass -p "$SSH_PASS" ssh $SSH_OPTS $SSH_USER@$ip "curl -s -H 'Content-Type: application/json' -d '{\"id\":1, \"jsonrpc\":\"2.0\", \"method\": \"system_localPeerId\", \"params\":[]}' http://localhost:9944" 2>/dev/null)
    peer_id=$(echo "$response" | grep -o '"result":"[^"]*"' | cut -d'"' -f4)
    
    if [ -n "$peer_id" ]; then
        PEER_IDS[$ip]=$peer_id
        echo "✓ $peer_id"
    else
        echo "✗ Failed"
    fi
done

echo ""
echo "🔗 Peering Nodes (Mesh Topology)..."
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"

for src_ip in "${ACTIVE_VMS[@]}"; do
    if [ -z "${PEER_IDS[$src_ip]}" ]; then continue; fi
    
    echo "Connecting $src_ip to others..."
    
    for dst_ip in "${ACTIVE_VMS[@]}"; do
        if [ "$src_ip" == "$dst_ip" ]; then continue; fi
        if [ -z "${PEER_IDS[$dst_ip]}" ]; then continue; fi
        
        peer_addr="/ip4/$dst_ip/tcp/30333/p2p/${PEER_IDS[$dst_ip]}"
        
        # Add Reserved Peer
        sshpass -p "$SSH_PASS" ssh $SSH_OPTS $SSH_USER@$src_ip "curl -s -H 'Content-Type: application/json' -d '{\"id\":1, \"jsonrpc\":\"2.0\", \"method\": \"system_addReservedPeer\", \"params\":[\"$peer_addr\"]}' http://localhost:9944" >/dev/null 2>&1
    done
done

echo ""
echo "📊 Network Health Report"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
printf "% -16s | % -10s | % -10s | % -10s | % -15s\n" "Node IP" "Block #" "Final #" "Peers" "Hash Prefix"
echo "─────────────────|────────────|────────────|────────────|────────────────"

for ip in "${ACTIVE_VMS[@]}"; do
    # Get Header (Best Block)
    header_res=$(sshpass -p "$SSH_PASS" ssh $SSH_OPTS $SSH_USER@$ip "curl -s -H 'Content-Type: application/json' -d '{\"id\":1, \"jsonrpc\":\"2.0\", \"method\": \"chain_getHeader\", \"params\":[]}' http://localhost:9944" 2>/dev/null)
    block_hex=$(echo "$header_res" | grep -o '"number":"[^"]*"' | cut -d'"' -f4)
    block_num=$((block_hex)) # Convert Hex to Dec
    
    # Get Finalized Head Hash
    final_head_res=$(sshpass -p "$SSH_PASS" ssh $SSH_OPTS $SSH_USER@$ip "curl -s -H 'Content-Type: application/json' -d '{\"id\":1, \"jsonrpc\":\"2.0\", \"method\": \"chain_getFinalizedHead\", \"params\":[]}' http://localhost:9944" 2>/dev/null)
    final_hash=$(echo "$final_head_res" | grep -o '"result":"[^"]*"' | cut -d'"' -f4)
    
    # Get Finalized Header
    final_header_res=$(sshpass -p "$SSH_PASS" ssh $SSH_OPTS $SSH_USER@$ip "curl -s -H 'Content-Type: application/json' -d '{\"id\":1, \"jsonrpc\":\"2.0\", \"method\": \"chain_getHeader\", \"params\":[\"$final_hash\"]}' http://localhost:9944" 2>/dev/null)
    final_hex=$(echo "$final_header_res" | grep -o '"number":"[^"]*"' | cut -d'"' -f4)
    final_num=$((final_hex))
    
    # Get Health (Peers)
    health_res=$(sshpass -p "$SSH_PASS" ssh $SSH_OPTS $SSH_USER@$ip "curl -s -H 'Content-Type: application/json' -d '{\"id\":1, \"jsonrpc\":\"2.0\", \"method\": \"system_health\", \"params\":[]}' http://localhost:9944" 2>/dev/null)
    peers=$(echo "$health_res" | grep -o '"peers":[0-9]*' | cut -d':' -f2)
    
    # Check for Forks (Hash Prefix)
    # Using parentHash of best block as a proxy for chain identity at the tip
    hash_prefix=${final_hash:0:10}...
    
    printf "% -16s | % -10s | % -10s | % -10s | % -15s\n" "$ip" "$block_num" "$final_num" "$peers" "$hash_prefix"
done
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
