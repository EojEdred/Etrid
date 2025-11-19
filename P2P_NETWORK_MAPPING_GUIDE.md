# P2P Network Mapping via Tailscale

## Overview

Use Tailscale to map, monitor, and visualize the P2P connections between all 22 FlareChain validators in production.

## What You Can Do

### ✓ **Live Monitoring**
- Check which validators are online
- See peer connection counts
- Monitor block heights
- View network health in real-time

### ✓ **Network Topology Mapping**
- Discover P2P peer relationships
- Create network graphs
- Identify connection patterns
- Detect isolated nodes

### ✓ **Performance Analysis**
- Track peer counts over time
- Identify well-connected vs isolated nodes
- Monitor network partition risks
- Analyze ASF consensus participation

---

## Tools Available

### 1. **Live P2P Monitor** (`/tmp/live-p2p-monitor.sh`)

Quick status check of all validators.

**Usage:**
```bash
/tmp/live-p2p-monitor.sh
```

**Shows:**
- Validator status (running/stopped)
- Number of P2P peers connected
- Current best block height
- Network health overview

**Example Output:**
```
ALIAS                NAME                      STATUS     PEER_COUNT      BEST_BLOCK
──────────────────────────────────────────────────────────────────────────────────
ts-val-01            Gizzi-Director-1          ✓ RUNNING  18              #1,234,567
ts-val-02            AuditDev-Director-2       ✓ RUNNING  21              #1,234,567
ts-val-03            Director-3                ✓ RUNNING  19              #1,234,567
...
```

---

### 2. **Full P2P Network Mapper** (`/tmp/map-p2p-network.sh`)

Comprehensive network topology analysis.

**Usage:**
```bash
/tmp/map-p2p-network.sh
```

**Generates:**
1. **Peer Data JSON** - Complete peer list for each validator
2. **Topology Map** - Text-based network map
3. **Graph (DOT file)** - GraphViz visualization

**Output Location:**
`~/Desktop/etrid/p2p-network-map/`

**Files Created:**
- `peers_TIMESTAMP.json` - Raw peer data
- `topology_TIMESTAMP.txt` - Human-readable map
- `network_graph_TIMESTAMP.dot` - GraphViz graph

---

## How It Works

### Query Method

Via Tailscale SSH, the scripts query each validator's Substrate RPC:

```bash
# Get P2P peers
curl -s -H 'Content-Type: application/json' \
     -d '{"id":1,"jsonrpc":"2.0","method":"system_peers","params":[]}' \
     http://localhost:9933/

# Get node identity
curl -s -H 'Content-Type: application/json' \
     -d '{"id":1,"jsonrpc":"2.0","method":"system_localPeerId","params":[]}' \
     http://localhost:9933/

# Get best block
curl -s -H 'Content-Type: application/json' \
     -d '{"id":1,"jsonrpc":"2.0","method":"chain_getBlock","params":[]}' \
     http://localhost:9933/
```

### Data Collected

For each validator:
- **Peer ID** - Unique libp2p identity
- **Connected Peers** - List of peer connections
- **Peer Count** - Number of active connections
- **Block Height** - Current chain tip
- **Node Status** - Running/stopped

---

## Visualization

### Create Network Graph (Requires GraphViz)

```bash
# Install GraphViz (if not already)
brew install graphviz

# Generate network graph
cd ~/Desktop/etrid/p2p-network-map/
dot -Tpng network_graph_*.dot -o network_graph.png
dot -Tsvg network_graph_*.dot -o network_graph.svg

# View the graph
open network_graph.png
```

### Graph Shows:
- **Green boxes** - Oracle Directors (6 nodes)
- **Yellow boxes** - Contabo Validators (16 nodes)
- **Arrows** - P2P connections between nodes
- **Clusters** - Grouped by infrastructure

---

## Use Cases

### 1. **Network Health Monitoring**

Check if all validators are properly connected:

```bash
/tmp/live-p2p-monitor.sh
```

**Look for:**
- All nodes showing "RUNNING"
- Peer counts > 0 (ideally 15-21)
- Block heights synchronized

### 2. **Detect Network Partitions**

Run full mapper to identify isolated nodes:

```bash
/tmp/map-p2p-network.sh
cat ~/Desktop/etrid/p2p-network-map/topology_*.txt
```

**Warning signs:**
- Nodes with 0 peers
- Significantly lower peer counts
- Different block heights

### 3. **ASF Consensus Analysis**

Track which validators are participating in ASF:

```bash
# Query ASF finality status via Tailscale
for i in {1..22}; do
  echo "=== Validator $i ==="
  ssh ts-val-$(printf "%02d" $i) \
    'curl -s http://localhost:9933 -H "Content-Type: application/json" \
     -d "{\"id\":1,\"jsonrpc\":\"2.0\",\"method\":\"grandpa_roundState\",\"params\":[]}"' \
    | jq '.result.best'
done
```

### 4. **Continuous Monitoring**

Create a monitoring loop:

```bash
# Watch network status every 30 seconds
watch -n 30 /tmp/live-p2p-monitor.sh
```

### 5. **Historical Tracking**

Save snapshots over time:

```bash
# Run mapper every hour
crontab -e
# Add: 0 * * * * /tmp/map-p2p-network.sh

# Compare changes
cd ~/Desktop/etrid/p2p-network-map/
diff topology_20251118_100000.txt topology_20251118_110000.txt
```

---

## Advanced Queries

### Check Specific Peer Connections

```bash
# Get detailed peer info from a specific validator
ssh ts-val-01 'curl -s http://localhost:9933 \
  -H "Content-Type: application/json" \
  -d "{\"id\":1,\"jsonrpc\":\"2.0\",\"method\":\"system_peers\",\"params\":[]}"' \
  | jq '.result[] | {peerId, roles, bestNumber}'
```

### Monitor Connection Changes

```bash
# Save current peer counts
/tmp/live-p2p-monitor.sh > /tmp/peers_before.txt

# Wait some time
sleep 300

# Compare
/tmp/live-p2p-monitor.sh > /tmp/peers_after.txt
diff /tmp/peers_before.txt /tmp/peers_after.txt
```

### Check Network Latency via Tailscale

```bash
# Ping all validators via Tailscale
for i in {1..22}; do
  alias=$(printf "ts-val-%02d" $i)
  ssh "$alias" 'tailscale ping --c 1 100.96.84.69' | grep time
done
```

---

## Integration with ASF Monitoring

Combine with ASF finality monitoring:

```bash
# Check both P2P and ASF status
/tmp/live-p2p-monitor.sh

# Then check ASF consensus
for i in {1..22}; do
  ssh ts-val-$(printf "%02d" $i) \
    'journalctl -u flarechain-node -n 100 | grep -i "finalized"'
done
```

---

## Troubleshooting

### If peer counts are 0:

1. **Check firewall**: Port 30333 must be open
   ```bash
   ssh ts-val-01 'sudo ufw status | grep 30333'
   ```

2. **Check node is running**:
   ```bash
   ssh ts-val-01 'systemctl status flarechain-node'
   ```

3. **Check bootnodes**:
   ```bash
   ssh ts-val-01 'journalctl -u flarechain-node | grep bootnode'
   ```

### If RPC queries fail:

1. **Check RPC is enabled**:
   ```bash
   ssh ts-val-01 'curl -s http://localhost:9933/health'
   ```

2. **Check node config**:
   ```bash
   ssh ts-val-01 'cat /etc/systemd/system/flarechain-node.service'
   ```

### If Tailscale connection fails:

1. **Check Tailscale status**:
   ```bash
   ssh ts-val-01 'sudo tailscale status'
   ```

2. **Restart if needed**:
   ```bash
   ssh ts-val-01 'sudo tailscale up'
   ```

---

## Benefits of Using Tailscale for P2P Mapping

✓ **Secure** - Encrypted WireGuard connections
✓ **Fast** - Direct peer-to-peer queries
✓ **Reliable** - Works from any network
✓ **Centralized** - Query all validators from one location
✓ **Persistent** - Stable IPs for consistent monitoring

---

## Next Steps

1. **Run live monitor** to get current status:
   ```bash
   /tmp/live-p2p-monitor.sh
   ```

2. **Generate full topology map**:
   ```bash
   /tmp/map-p2p-network.sh
   ```

3. **Create network visualization**:
   ```bash
   cd ~/Desktop/etrid/p2p-network-map/
   dot -Tpng network_graph_*.dot -o network.png
   open network.png
   ```

4. **Set up continuous monitoring**:
   ```bash
   watch -n 60 /tmp/live-p2p-monitor.sh
   ```

---

## Example Workflow

```bash
# 1. Quick status check
/tmp/live-p2p-monitor.sh

# 2. Generate detailed topology
/tmp/map-p2p-network.sh

# 3. View results
cd ~/Desktop/etrid/p2p-network-map/
cat topology_*.txt

# 4. Create visual graph
dot -Tpng network_graph_*.dot -o network.png
open network.png

# 5. Investigate any issues
ssh ts-val-01 'curl -s http://localhost:9933 \
  -H "Content-Type: application/json" \
  -d "{\"id\":1,\"jsonrpc\":\"2.0\",\"method\":\"system_health\",\"params\":[]}"'
```

---

**Your P2P network is now fully mapped via Tailscale!** 🗺️

All 22 validators are accessible for real-time monitoring and analysis.
