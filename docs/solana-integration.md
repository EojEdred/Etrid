# Solana Integration

ËTRID's Partition Burst Chain (PBC) for Solana enables seamless bridging of SOL and SPL tokens.

## Overview

**PBC-SOL** connects ËTRID to the Solana blockchain, enabling:
- Bridge SOL tokens to/from ËTRID
- Bridge SPL tokens (USDC, USDT, etc.)
- Cross-chain DeFi between Solana and ËTRID
- Fast finality (5-10 minutes)

---

## Supported Assets

### Native Token
- **SOL** - Solana's native token

### SPL Tokens
- **USDC** - USD Coin
- **USDT** - Tether USD
- **RAY** - Raydium
- **SRM** - Serum
- **More coming soon**

Check [bridge.etrid.org/assets](https://bridge.etrid.org/assets) for complete list.

---

## Bridging SOL to ËTRID

### Step 1: Connect Wallets

1. Visit [bridge.etrid.org](https://bridge.etrid.org)
2. Select:
   - **From:** Solana
   - **To:** ËTRID (PBC-SOL)
3. Connect Solana wallet (Phantom, Solflare, etc.)
4. Connect ËTRID wallet

### Step 2: Initiate Bridge

```
┌─────────────────────────────────────────────┐
│ Bridge SOL to ËTRID                        │
├─────────────────────────────────────────────┤
│ From: Solana Mainnet                        │
│ To: ËTRID PBC-SOL                          │
│                                              │
│ Amount: 10.0 SOL                            │
│ Bridge Fee: 0.01 SOL (~$0.50)              │
│                                              │
│ You will receive: 9.99 SOL on ËTRID       │
│ Estimated time: 5-10 minutes                │
│                                              │
│ [ Review Transaction ]                      │
└─────────────────────────────────────────────┘
```

### Step 3: Wait for Confirmations

**Bridge Process:**
1. **Solana confirmation:** 32 slots (~15 seconds)
2. **Relay to ËTRID:** Light client verification (~2 min)
3. **PBC minting:** Issue wrapped SOL (~1 min)
4. **Finalization:** 3 ËTRID blocks (~15 sec)

**Total Time:** 5-10 minutes

### Step 4: Use Bridged SOL

Bridged SOL appears in your ËTRID wallet on PBC-SOL chain.

**Use for:**
- Trading on ËtridSwap
- Providing liquidity
- Staking in DeFi protocols
- Cross-chain arbitrage

---

## Bridging SOL from ËTRID

### Reverse Bridge

1. Navigate to [bridge.etrid.org](https://bridge.etrid.org)
2. Select:
   - **From:** ËTRID (PBC-SOL)
   - **To:** Solana Mainnet
3. Enter amount and Solana destination address
4. Pay bridge fee (ÉTR or SOL)
5. Wait for multi-sig approval (~5 min)
6. Receive SOL on Solana Mainnet

---

## Bridging SPL Tokens

### Example: USDC

**Same process as SOL:**
1. Select USDC token in bridge interface
2. Enter amount
3. Confirm transaction in Solana wallet
4. Wait for bridge confirmation
5. Receive wrapped USDC on ËTRID

**Supported SPL Tokens:**
- USDC (USD Coin)
- USDT (Tether)
- RAY (Raydium)
- SRM (Serum)

---

## Bridge Fees

| Asset | Bridge In | Bridge Out |
|-------|-----------|------------|
| SOL | 0.1% (min 0.01 SOL) | 0.1% (min 0.01 SOL) |
| USDC | 0.1% (min $0.50) | 0.1% (min $0.50) |
| Other SPL | 0.1% (min $0.50) | 0.1% (min $0.50) |

---

## Technical Architecture

### Light Client

ËTRID maintains a Solana light client on PBC-SOL that:
- Verifies Solana block headers
- Validates proof of finality
- Confirms transaction inclusion
- Ensures security without full node

### Multi-Sig Custodians

Bridge uses 5-of-7 multi-signature scheme:
- 7 independent custodians
- 5 signatures required for withdrawals
- Distributed geographically
- Regular audits

### Security Measures

✅ **Light Client Verification**
- Cryptographic proofs
- No trust in custodians for deposits

✅ **Time Locks**
- 2-hour delay for large withdrawals
- Emergency pause functionality

✅ **Rate Limits**
- Maximum daily bridge volume
- Per-user limits

---

## Using Bridged SOL in DeFi

### ËtridSwap

**Trade:**
```
SOL → ÉTR
SOL → USDC
SOL → other assets
```

**Provide Liquidity:**
```
Add to SOL-ÉTR pool
Earn trading fees + rewards
```

### Lending Markets

**Supply SOL:**
- Earn interest on deposited SOL
- Use as collateral for borrowing

**Borrow Against SOL:**
- Borrow stablecoins
- Borrow ÉTR for staking

### Yield Farming

**Farm with SOL:**
- Stake in liquidity pools
- Earn ÉTR rewards
- Compound rewards

---

## Developer Integration

### JavaScript SDK

```javascript
import { EtridBridge } from '@etrid/bridge-sdk';

// Initialize bridge
const bridge = new EtridBridge({
  network: 'mainnet',
  solanaRpc: 'https://api.mainnet-beta.solana.com',
  etridWs: 'wss://rpc.etrid.org'
});

// Bridge SOL to ËTRID
const tx = await bridge.deposit({
  from: 'solana',
  to: 'etrid',
  asset: 'SOL',
  amount: '10.0',
  recipient: 'ETRID_ADDRESS'
});

console.log('Transaction:', tx.signature);

// Monitor status
bridge.on('confirmed', (txHash) => {
  console.log('Bridge confirmed:', txHash);
});
```

### Rust SDK

```rust
use etrid_bridge::{Bridge, Chain};

#[tokio::main]
async fn main() {
    let bridge = Bridge::new("mainnet").await;

    let tx = bridge.deposit(
        Chain::Solana,
        Chain::Etrid,
        "SOL",
        10_000_000_000, // 10 SOL (in lamports)
        "ETRID_ADDRESS"
    ).await?;

    println!("Transaction: {}", tx.signature());
}
```

---

## Monitoring

### Bridge Explorer

Track all Solana ↔ ËTRID bridge activity:

**URL:** [explorer.etrid.org/bridges/solana](https://explorer.etrid.org/bridges/solana)

**View:**
- Real-time bridge transactions
- Volume statistics
- Bridge TVL (Total Value Locked)
- Historical charts

### Bridge Status

**Check bridge health:**
[status.etrid.org/solana](https://status.etrid.org/solana)

**Indicators:**
- ✅ Operational
- ⚠️ Degraded Performance
- ❌ Maintenance Mode

---

## Troubleshooting

### Transaction Stuck

**Symptoms:** Bridge transaction pending > 30 minutes

**Solutions:**
1. Check Solana network status (congestion?)
2. Verify transaction confirmed on Solana
3. Check bridge status page
4. Contact support with transaction signature

### Assets Not Received

**Symptoms:** Solana transaction confirmed but no assets on ËTRID

**Solutions:**
1. Wait up to 1 hour (may be custodian delay)
2. Check correct PBC-SOL chain selected in wallet
3. Refresh wallet (hard refresh)
4. Contact support with both transaction hashes

---

## Solana Wallet Support

### Recommended Wallets

| Wallet | Platform | Bridge Support |
|--------|----------|----------------|
| **Phantom** | Browser, Mobile | ✅ Full support |
| **Solflare** | Browser, Mobile | ✅ Full support |
| **Backpack** | Browser | ✅ Full support |
| **Ledger** | Hardware | ✅ Supported |

---

## Fees Comparison

| Bridge | SOL Fee | USDC Fee | Time |
|--------|---------|----------|------|
| **ËTRID** | 0.1% | 0.1% | 5-10 min |
| Wormhole | 0.1% | $1-5 | 10-20 min |
| Allbridge | 0.3% | 0.3% | 5-15 min |

---

## Roadmap

**Q1 2026:**
- ✅ SOL bridging live
- ✅ USDC/USDT support
- [ ] Additional SPL tokens

**Q2 2026:**
- [ ] NFT bridging
- [ ] Direct swaps (skip wrapping)
- [ ] Reduced fees (0.05%)

**Q3 2026:**
- [ ] Solana smart contract calls from ËTRID
- [ ] Cross-chain messaging

---

## Resources

**Bridge:**
- [Bridge Interface](https://bridge.etrid.org)
- [Bridge Explorer](https://explorer.etrid.org/bridges/solana)
- [Bridge Documentation](bridges.md)

**Developer:**
- [Bridge SDK](https://github.com/etrid/bridge-sdk)
- [API Reference](API_REFERENCE.md)
- [Example Code](https://github.com/etrid/bridge-examples)

**Support:**
- 💬 [Discord #bridge-support](https://discord.gg/etrid)
- 📧 Email: solana-bridge@etrid.org

---

**Ready to bridge?** [Start Bridging →](https://bridge.etrid.org)
