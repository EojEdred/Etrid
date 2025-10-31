#!/bin/bash
################################################################################
# Generate Deployment Report
#
# Creates a comprehensive report of all deployed contracts and their details
################################################################################

set -e

echo "╔════════════════════════════════════════════════════════════╗"
echo "║            ËTRID DEX DEPLOYMENT REPORT                    ║"
echo "╚════════════════════════════════════════════════════════════╝"
echo ""

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
DEX_DIR="$(dirname "$SCRIPT_DIR")"
REPORT_FILE="$DEX_DIR/DEPLOYMENT_REPORT_$(date +%Y%m%d_%H%M%S).md"

# Start report
cat > "$REPORT_FILE" << 'EOF'
# ËTRID DEX Deployment Report

**Generated:** $(date)
**Status:** Production Deployment

---

## 📊 Deployment Summary

EOF

echo "Generating deployment report..."

# Function to add chain info to report
add_chain_report() {
  local chain=$1
  local deployment_file=$2
  local explorer_base=$3

  if [ -f "$deployment_file" ]; then
    local address=$(jq -r '.contractAddress' "$deployment_file")
    local deployer=$(jq -r '.deployerAddress' "$deployment_file")
    local tx_hash=$(jq -r '.deploymentHash' "$deployment_file")
    local timestamp=$(jq -r '.timestamp' "$deployment_file")
    local supply=$(jq -r '.contractDetails.initialSupply' "$deployment_file")

    cat >> "$REPORT_FILE" << EOF

### $chain

**Status:** ✅ DEPLOYED

| Detail | Value |
|--------|-------|
| **Contract Address** | \`$address\` |
| **Deployer** | \`$deployer\` |
| **Deployment TX** | \`$tx_hash\` |
| **Timestamp** | $timestamp |
| **Initial Supply** | $supply ÉTR |
| **Explorer** | [$explorer_base/address/$address]($explorer_base/address/$address) |

EOF
    echo "✅ Added $chain to report"
  else
    cat >> "$REPORT_FILE" << EOF

### $chain

**Status:** ⏳ NOT YET DEPLOYED

EOF
    echo "⏳ $chain not yet deployed"
  fi
}

# Add each chain
add_chain_report "BSC (BEP-20)" \
  "$DEX_DIR/bsc/deployments/bscMainnet-deployment.json" \
  "https://bscscan.com"

add_chain_report "Ethereum (ERC-20)" \
  "$DEX_DIR/ethereum/deployments/mainnet-deployment.json" \
  "https://etherscan.io"

add_chain_report "Polygon (ERC-20)" \
  "$DEX_DIR/polygon/deployments/polygon-deployment.json" \
  "https://polygonscan.com"

# Solana (different format)
if [ -f "$DEX_DIR/solana/deployments/solana-deployment.json" ]; then
  SOL_MINT=$(jq -r '.tokenMint' "$DEX_DIR/solana/deployments/solana-deployment.json")
  SOL_TIMESTAMP=$(jq -r '.timestamp' "$DEX_DIR/solana/deployments/solana-deployment.json")

  cat >> "$REPORT_FILE" << EOF

### Solana (SPL Token)

**Status:** ✅ DEPLOYED

| Detail | Value |
|--------|-------|
| **Token Mint** | \`$SOL_MINT\` |
| **Timestamp** | $SOL_TIMESTAMP |
| **Explorer** | [https://solscan.io/token/$SOL_MINT](https://solscan.io/token/$SOL_MINT) |

EOF
  echo "✅ Added Solana to report"
else
  cat >> "$REPORT_FILE" << EOF

### Solana (SPL Token)

**Status:** ⏳ NOT YET DEPLOYED

EOF
  echo "⏳ Solana not yet deployed"
fi

# Add DEX pools section
cat >> "$REPORT_FILE" << 'EOF'

---

## 🏊 Liquidity Pools

### Phase 1 DEXes

| DEX | Chain | Status | Pool Address | Liquidity |
|-----|-------|--------|--------------|-----------|
| PancakeSwap V3 | BSC | ⏳ Pending | TBD | 25M ÉTR + $2M BNB |
| Raydium CLMM | Solana | ⏳ Pending | TBD | 25M ÉTR + $2M SOL |
| Uniswap V3 | Ethereum | ⏳ Pending | TBD | 25M ÉTR + $2M ETH |
| QuickSwap V3 | Polygon | ⏳ Pending | TBD | 15M ÉTR + $1M MATIC |

---

## 📋 Next Steps

### Immediate Actions

- [ ] Transfer ownership to Foundation multisig
- [ ] Create liquidity pools on each DEX
- [ ] Add initial liquidity (90M ÉTR + $7M native tokens)
- [ ] Test swaps on all DEXes
- [ ] Submit to CoinGecko
- [ ] Submit to CoinMarketCap
- [ ] Announce on social media

### Week 1

- [ ] Monitor trading volume
- [ ] Adjust liquidity ranges if needed
- [ ] Launch LP rewards program
- [ ] Community AMAs

### Month 1

- [ ] Deploy to Phase 2 chains (Avalanche, Arbitrum, Base)
- [ ] Apply to Gate.io and KuCoin
- [ ] Cross-chain bridge integration

---

## 🔐 Security Checklist

- [ ] All contracts verified on block explorers
- [ ] Ownership transferred to Foundation multisig (6-of-9)
- [ ] Bridge contracts configured
- [ ] Emergency pause functionality tested
- [ ] Max supply cap enforced (1B ÉTR)
- [ ] Foundation approval documented

---

## 📞 Support

- **Discord:** #dex-deployment
- **Email:** dev@etrid.org
- **Emergency:** Foundation Directors (7-of-9 multisig)

---

**Report Generated:** $(date)

EOF

echo ""
echo "╔════════════════════════════════════════════════════════════╗"
echo "║              DEPLOYMENT REPORT GENERATED ✅                ║"
echo "╚════════════════════════════════════════════════════════════╝"
echo ""
echo "📄 Report saved to: $REPORT_FILE"
echo ""
echo "You can:"
echo "  - Review: cat $REPORT_FILE"
echo "  - Share with Foundation Directors for approval"
echo "  - Include in quarterly reports"
echo ""
