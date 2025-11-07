#!/bin/bash
#
# FlareChain Committee Structure Mapper
# Maps peer IDs to validators and visualizes committee structure
#

set -e

BLUE='\033[0;34m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
NC='\033[0m'

RPC_URL="${1:-http://129.80.122.34:9944}"
OUTPUT_DIR="/tmp/committee-mapping"

echo -e "${BLUE}╔════════════════════════════════════════════════════════════════╗${NC}"
echo -e "${BLUE}║      FlareChain Committee Structure Mapper                    ║${NC}"
echo -e "${BLUE}╚════════════════════════════════════════════════════════════════╝${NC}"
echo ""

mkdir -p "$OUTPUT_DIR"

# Step 1: Get all connected peers
echo -e "${YELLOW}[1/6] Fetching connected peers...${NC}"
PEERS_JSON=$(curl -s -H "Content-Type: application/json" \
  -d '{"id":1, "jsonrpc":"2.0", "method": "system_peers"}' \
  "$RPC_URL")

echo "$PEERS_JSON" | jq '.' > "$OUTPUT_DIR/peers-raw.json"

PEER_COUNT=$(echo "$PEERS_JSON" | jq '.result | length')
echo -e "${GREEN}✓ Found $PEER_COUNT connected peers${NC}"
echo ""

# Step 2: Extract peer IDs and best blocks
echo -e "${YELLOW}[2/6] Extracting peer information...${NC}"
echo "$PEERS_JSON" | jq -r '.result[] | "\(.peerId)|\(.bestNumber)|\(.bestHash)"' > "$OUTPUT_DIR/peers-list.txt"

cat > "$OUTPUT_DIR/peers-formatted.txt" << 'EOF'
╔═══════════════════════════════════════════════════════════════════════════════╗
║                        CONNECTED PEERS ANALYSIS                               ║
╚═══════════════════════════════════════════════════════════════════════════════╝

EOF

while IFS='|' read -r peer_id best_num best_hash; do
  printf "Peer: %-55s Block: #%-6s\n" "$peer_id" "$best_num" >> "$OUTPUT_DIR/peers-formatted.txt"
done < "$OUTPUT_DIR/peers-list.txt"

cat "$OUTPUT_DIR/peers-formatted.txt"
echo ""

# Step 3: Get network health and sync state
echo -e "${YELLOW}[3/6] Fetching network health...${NC}"
HEALTH=$(curl -s -H "Content-Type: application/json" \
  -d '{"id":1, "jsonrpc":"2.0", "method": "system_health"}' \
  "$RPC_URL" | jq '.result')

SYNC_STATE=$(curl -s -H "Content-Type: application/json" \
  -d '{"id":1, "jsonrpc":"2.0", "method": "system_syncState"}' \
  "$RPC_URL" | jq '.result')

echo "$HEALTH" > "$OUTPUT_DIR/network-health.json"
echo "$SYNC_STATE" > "$OUTPUT_DIR/sync-state.json"

PEERS=$(echo "$HEALTH" | jq -r '.peers')
IS_SYNCING=$(echo "$HEALTH" | jq -r '.isSyncing')
CURRENT_BLOCK=$(echo "$SYNC_STATE" | jq -r '.currentBlock')

echo -e "${GREEN}✓ Network Health:${NC}"
echo "  Peers: $PEERS"
echo "  Syncing: $IS_SYNCING"
echo "  Best Block: #$CURRENT_BLOCK"
echo ""

# Step 4: Query GRANDPA authority set (on-chain)
echo -e "${YELLOW}[4/6] Querying GRANDPA authority set...${NC}"

# Get current GRANDPA authorities from runtime
AUTHORITIES=$(curl -s -H "Content-Type: application/json" \
  -d '{"id":1, "jsonrpc":"2.0", "method": "state_call", "params": ["GrandpaApi_grandpa_authorities", "0x"]}' \
  "$RPC_URL" 2>/dev/null | jq -r '.result' || echo "")

if [ -n "$AUTHORITIES" ] && [ "$AUTHORITIES" != "null" ]; then
  echo "$AUTHORITIES" > "$OUTPUT_DIR/grandpa-authorities-raw.hex"
  echo -e "${GREEN}✓ GRANDPA authorities retrieved${NC}"
else
  echo -e "${YELLOW}⚠ Could not retrieve GRANDPA authorities (runtime API may differ)${NC}"
fi
echo ""

# Step 5: Create committee mapping report
echo -e "${YELLOW}[5/6] Creating committee structure report...${NC}"

cat > "$OUTPUT_DIR/committee-structure.md" << EOREPORT
# FlareChain Mainnet - Committee Structure Report

**Generated:** $(date)
**RPC Endpoint:** $RPC_URL

---

## Network Overview

| Metric | Value |
|--------|-------|
| Connected Peers | $PEERS |
| Is Syncing | $IS_SYNCING |
| Current Block | #$CURRENT_BLOCK |
| Total Validators (Genesis) | 21 |
| Active Validators (Estimated) | ~$PEERS |
| Inactive Validators | ~$((21 - PEERS)) |

---

## Connected Peer IDs

EOREPORT

i=1
while IFS='|' read -r peer_id best_num best_hash; do
  lag=$((CURRENT_BLOCK - best_num))
  if [ $lag -le 5 ]; then
    status="✅ IN_SYNC"
  elif [ $lag -le 20 ]; then
    status="⚠️ LAGGING"
  else
    status="🔴 STALLED"
  fi

  echo "**$i. Peer:** \`$peer_id\`" >> "$OUTPUT_DIR/committee-structure.md"
  echo "   - Best Block: #$best_num ($status, lag: $lag blocks)" >> "$OUTPUT_DIR/committee-structure.md"
  echo "   - Best Hash: \`${best_hash:0:20}...\`" >> "$OUTPUT_DIR/committee-structure.md"
  echo "" >> "$OUTPUT_DIR/committee-structure.md"

  i=$((i + 1))
done < "$OUTPUT_DIR/peers-list.txt"

cat >> "$OUTPUT_DIR/committee-structure.md" << 'EOREPORT'

---

## Known Validators Mapping

To identify which specific validators (1-21) are active, we need to match:
1. **Peer IDs** (from network) ↔ **Validator Accounts** (from genesis)
2. Use network key derivation or manual tracking

### Known Validators:

| # | Name | Account ID | Status | Peer ID |
|---|------|------------|--------|---------|
| 1 | Gizzi (AI Overseer) | 5Dd8AjjuwKDP8P8sDgui... | ✅ DEPLOYED | 12D3KooWDQhL88sDaMCbtWfFFnCAyTYgyRcQumNbhfi2tmiP7ckm |
| 5 | AuditDev | 5DJj4b331JKDTuwQegXS... | ✅ DEPLOYED | 12D3KooWGjGCJzexrJct6nGCSDnj7vaJtMohpagFUPBhPgpZqvpd |
| 2-4, 6-21 | Various | ... | ❌ NOT DEPLOYED | Unknown |

---

## Committee Structure Analysis

### GRANDPA Finality Committee
- **Required for Supermajority:** 15 validators (2/3+1 of 21)
- **Currently Active:** ~PEERS_COUNT validators
- **Status:** PARTIAL_OR_FULL

### ASF Finality Committee
- **Total Committee Size:** 21 validators
- **Active Members:** ~PEERS_COUNT validators
- **PPFA Consensus:** ACTIVE_OR_WAITING

---

## Next Steps to Map Full Committee

### Method 1: Network Key Matching
1. For each validator in `validator-keys-complete.json`
2. Derive Peer ID from network secret key
3. Match against connected peer IDs
4. Identify which validators are active

### Method 2: Session Key Rotation Events
1. Query session key rotation events from chain
2. Match validator accounts to session changes
3. Track when validators joined/left

### Method 3: Manual Tracking
1. Track deployment timestamps
2. Note Peer IDs during deployment
3. Maintain mapping table

---

## Recommended Actions

Based on current peer count of PEERS_COUNT:

EOREPORT

if [ "$PEERS" -lt 15 ]; then
  NEEDED=$((15 - PEERS))
  cat >> "$OUTPUT_DIR/committee-structure.md" << EOREPORT
🔴 **NEED MORE VALIDATORS FOR BFT**

- Need to deploy: **$NEEDED more validators**
- Priority: Deploy validators 3, 4, 6, 7, 8, 9, 10
- Target: Reach 15 active validators for GRANDPA supermajority

**Action:** Run deployment script for $NEEDED validators
\`\`\`bash
cd /Users/macbook/Desktop/etrid/scripts
./deploy-validator.sh <index> <ip> "<name>"
\`\`\`
EOREPORT
else
  cat >> "$OUTPUT_DIR/committee-structure.md" << EOREPORT
✅ **BFT THRESHOLD REACHED!**

- Active validators: **$PEERS** (≥15 required)
- GRANDPA supermajority: **ACHIEVED**
- Byzantine tolerance: Can tolerate $((PEERS - (PEERS * 2 / 3 + 1))) faulty validators
- Network status: **PRODUCTION READY**

**Next Steps:**
1. Monitor network stability for 24 hours
2. Deploy remaining $((21 - PEERS)) validators to reach full 21-validator set
3. Enable governance and DAO treasury
EOREPORT
fi

cat >> "$OUTPUT_DIR/committee-structure.md" << 'EOREPORT'

---

## Files Generated

- `peers-raw.json` - Raw peer data from RPC
- `peers-list.txt` - Formatted peer list
- `network-health.json` - Network health metrics
- `sync-state.json` - Sync state data
- `committee-structure.md` - This report
- `peer-to-validator-mapping.json` - Peer ID to validator mapping (if available)

---

**Report Generated by:** FlareChain Committee Mapper
**Genesis Hash:** `0xca40bbf4f8367f63ea110afd54cf5fd38c44df100f9454b62135bfc09df74da8`

EOREPORT

# Replace placeholders
sed -i.bak "s/PEERS_COUNT/$PEERS/g" "$OUTPUT_DIR/committee-structure.md"
sed -i.bak "s/PARTIAL_OR_FULL/$([ $PEERS -ge 15 ] && echo '✅ SUPERMAJORITY' || echo '🟡 PARTIAL')/g" "$OUTPUT_DIR/committee-structure.md"
sed -i.bak "s/ACTIVE_OR_WAITING/$([ $PEERS -ge 15 ] && echo '✅ ACTIVE' || echo '⚠️ WAITING FOR MORE VALIDATORS')/g" "$OUTPUT_DIR/committee-structure.md"

echo -e "${GREEN}✓ Committee structure report created${NC}"
echo ""

# Step 6: Generate peer-to-validator mapping
echo -e "${YELLOW}[6/6] Creating peer-to-validator mapping tool...${NC}"

cat > "$OUTPUT_DIR/identify-validators.py" << 'EOPY'
#!/usr/bin/env python3
"""
Map Peer IDs to Validator Identities
Requires: validator-keys-complete.json and network peer data
"""

import json
import sys
from pathlib import Path

# Known mappings (from deployment tracking)
KNOWN_MAPPINGS = {
    "12D3KooWDQhL88sDaMCbtWfFFnCAyTYgyRcQumNbhfi2tmiP7ckm": {
        "validator_index": 1,
        "name": "Gizzi (AI Overseer)",
        "account": "5Dd8AjjuwKDP8P8sDguiiNKfADAXrACramNbWvLcdLEpGaPJ"
    },
    "12D3KooWGjGCJzexrJct6nGCSDnj7vaJtMohpagFUPBhPgpZqvpd": {
        "validator_index": 5,
        "name": "AuditDev",
        "account": "5DJj4b331JKDTuwQegXSEVFC2yPjtJBW1QW4tBRBsnC9Bxgb"
    }
}

def load_peers(peers_file):
    """Load peer IDs from peers-list.txt"""
    peers = []
    with open(peers_file) as f:
        for line in f:
            if '|' in line:
                peer_id, best_num, best_hash = line.strip().split('|')
                peers.append({
                    'peer_id': peer_id,
                    'best_block': int(best_num),
                    'best_hash': best_hash
                })
    return peers

def map_peers_to_validators(peers):
    """Map peer IDs to known validators"""
    mapped = []
    unknown = []

    for peer in peers:
        peer_id = peer['peer_id']
        if peer_id in KNOWN_MAPPINGS:
            mapped.append({
                **peer,
                **KNOWN_MAPPINGS[peer_id],
                'status': 'IDENTIFIED'
            })
        else:
            unknown.append({
                **peer,
                'validator_index': None,
                'name': 'Unknown Validator',
                'account': 'Unknown',
                'status': 'UNIDENTIFIED'
            })

    return mapped, unknown

def main():
    peers_file = Path("/tmp/committee-mapping/peers-list.txt")
    output_file = Path("/tmp/committee-mapping/peer-to-validator-mapping.json")

    if not peers_file.exists():
        print("Error: peers-list.txt not found. Run map-committee-structure.sh first.")
        sys.exit(1)

    peers = load_peers(peers_file)
    mapped, unknown = map_peers_to_validators(peers)

    result = {
        'total_peers': len(peers),
        'identified': len(mapped),
        'unidentified': len(unknown),
        'mapped_validators': mapped,
        'unknown_validators': unknown
    }

    with open(output_file, 'w') as f:
        json.dump(result, f, indent=2)

    print(f"✓ Mapping complete: {len(mapped)} identified, {len(unknown)} unknown")
    print(f"✓ Saved to: {output_file}")

    # Print summary
    print("\n" + "="*70)
    print("IDENTIFIED VALIDATORS:")
    print("="*70)
    for v in mapped:
        print(f"  #{v['validator_index']:2} | {v['name']:30} | Block: #{v['best_block']}")

    print("\n" + "="*70)
    print(f"UNIDENTIFIED VALIDATORS: {len(unknown)}")
    print("="*70)
    for i, v in enumerate(unknown, 1):
        print(f"  {i}. {v['peer_id'][:40]}... | Block: #{v['best_block']}")

    print("\n" + "="*70)
    print("To identify unknown validators:")
    print("1. Check deployment logs for peer IDs")
    print("2. Add mappings to KNOWN_MAPPINGS in this script")
    print("3. Re-run to update mapping")
    print("="*70)

if __name__ == '__main__':
    main()
EOPY

chmod +x "$OUTPUT_DIR/identify-validators.py"
python3 "$OUTPUT_DIR/identify-validators.py" 2>/dev/null || echo -e "${YELLOW}⚠ Python mapping tool created (run separately if needed)${NC}"

echo ""
echo -e "${BLUE}═══════════════════════════════════════════════════════════════${NC}"
echo -e "${GREEN}✓ Committee structure mapping complete!${NC}"
echo -e "${BLUE}═══════════════════════════════════════════════════════════════${NC}"
echo ""
echo -e "${YELLOW}📊 Reports Generated:${NC}"
echo "  • Committee Structure: $OUTPUT_DIR/committee-structure.md"
echo "  • Peer List: $OUTPUT_DIR/peers-list.txt"
echo "  • Raw Data: $OUTPUT_DIR/peers-raw.json"
echo "  • Mapping Tool: $OUTPUT_DIR/identify-validators.py"
echo ""
echo -e "${YELLOW}📖 View Report:${NC}"
echo "  cat $OUTPUT_DIR/committee-structure.md"
echo ""
echo -e "${YELLOW}🔍 Identify Validators:${NC}"
echo "  python3 $OUTPUT_DIR/identify-validators.py"
echo ""
