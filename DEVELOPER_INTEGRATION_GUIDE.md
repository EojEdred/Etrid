# ÈTRID Developer Integration Guide

**Version:** 1.0.0
**Date:** December 8, 2025
**Purpose:** Enable developers to build interfaces for ÈTRID's native omnichain currency conversion

---

## 1. EXECUTIVE SUMMARY

ÈTRID is a native omnichain execution layer where users seamlessly convert external currencies (BTC, ETH, SOL, etc.) to ÉTR **without ever seeing wrapped tokens**. This guide provides everything developers need to build conversion interfaces.

**Key Principle:** Wrapped tokens (wBTC, wETH, etc.) are internal abstractions that exist for ~2 seconds during conversion. Users only see external currencies and ÉTR.

---

## 2. HIGH-LEVEL ARCHITECTURE

### 2.1 What Users See

```
[User Sends BTC] → [Conversion Process (Hidden)] → [User Receives ÉTR]
```

### 2.2 What Actually Happens

```
1. User sends BTC to ÈTRID bridge address
2. BTC locked in multisig vault (6 confirmations)
3. [INTERNAL ONLY]:
   - wBTC minted on BTC-PBC
   - IntentRouter contract detects BTC arrival
   - Automatic swap in PrimeSwap: wBTC → ÉTR
   - wBTC consumed (never reaches user)
4. User receives ÉTR in wallet
```

---

## 3. NETWORK ENDPOINTS

### 3.1 RPC Endpoints

```
HTTP RPC:
- Primearc Core: https://rpc.etrid.org/primearc (Chain ID: 1337)
- BTC-PBC: https://rpc.etrid.org/btc-pbc (Chain ID: 1339)
- ETH-PBC: https://rpc.etrid.org/eth-pbc (Chain ID: 1338)
- SOL-PBC: https://rpc.etrid.org/sol-pbc (Chain ID: 1340)
- [13 total PBCs available]

WebSocket:
- Primearc Core: wss://ws.etrid.org/primearc
- PBC chains: wss://ws.etrid.org/{pbc-name}
```

### 3.2 Key Services

```
Web Wallet: https://wallet.etrid.org
Block Explorer: https://explorer.etrid.org
REST API: https://api.etrid.org/v1
```

---

## 4. INTEGRATION PATTERNS

### 4.1 Deposit Flow (BTC → ÉTR)

**Step 1: Display Conversion Quote**
```javascript
// Pseudo-code for fetching conversion quote
const quote = await getConversionQuote({
  sourceChain: 'bitcoin',
  sourceCurrency: 'BTC',
  targetChain: 'primearc',
  targetCurrency: 'ETR',
  amount: 0.5
});

// Returns:
// {
//   youSend: 0.5,
//   youReceive: 12487,
//   exchangeRate: 24974,
//   networkFee: 0.0001,
//   etrGasFee: 0.0002,
//   estimatedTime: 3600
// }
```

**Step 2: Get Bridge Address**
```javascript
// Fetch unique deposit address for user
const depositAddress = await getBridgeDepositAddress({
  userWallet: '0x...',
  sourceChain: 'bitcoin',
  targetChain: 'primearc'
});

// Returns user-specific BTC address to send funds to
// Returns: bc1q... (Bech32 address)
```

**Step 3: Monitor Transaction Status**
```javascript
// Subscribe to tx status updates
const unsubscribe = await subscribeToDepositStatus({
  depositAddress: 'bc1q...',
  callbacks: {
    onBtcReceived: (tx) => { /* Show "BTC received" */ },
    onBtcConfirmed: (tx) => { /* Show "Processing..." */ },
    onSwapComplete: (tx) => { /* Show "ETR received!" */ }
  }
});

// Transaction lifecycle:
// 1. BTC sent by user
// 2. 6 confirmations on Bitcoin (60 min)
// 3. wBTC minted on BTC-PBC
// 4. IntentRouter swaps wBTC → ÉTR
// 5. ÉTR delivered to user wallet
```

### 4.2 Withdrawal Flow (ÉTR → BTC)

**Step 1: Initiate Conversion**
```javascript
const txHash = await initiateConversion({
  sourceChain: 'primearc',
  targetChain: 'bitcoin',
  amount: 25000,
  targetAddress: 'bc1q...'  // User's BTC address
});
```

**Step 2: Monitor Status**
```javascript
// Subscribe to withdrawal events
const unsubscribe = await subscribeToWithdrawalStatus({
  txHash: '0x...',
  callbacks: {
    onSwapStart: () => { /* Show "Converting to BTC..." */ },
    onBurnComplete: () => { /* Show "Releasing BTC..." */ },
    onBtcSent: () => { /* Show "Complete!" */ }
  }
});

// Withdrawal lifecycle:
// 1. ÉTR locked in user wallet
// 2. Swap ÉTR → wBTC in PrimeSwap pool
// 3. wBTC sent to IntentRouter
// 4. wBTC burned, release event triggered
// 5. Bitcoin multisig unlocks BTC
// 6. BTC sent to user's Bitcoin address
```

### 4.3 Query IntentRouter for Swap Info

```javascript
// Get swap details from IntentRouter
const swapInfo = await queryIntentRouter({
  contract: '0x...',  // IntentRouter address on source PBC
  method: 'getPendingSwaps',
  params: {
    userAddress: '0x...'
  }
});

// Returns:
// {
//   pendingSwaps: [
//     {
//       id: 1,
//       sourceToken: 'wBTC',
//       targetToken: 'ETR',
//       sourceAmount: 0.5,
//       targetAmount: 12487,
//       status: 'completed',
//       completedAt: 1702000000
//     }
//   ]
// }
```

---

## 5. DEVELOPER PROMPTS & SPECIFICATIONS

### 5.1 Frontend Integration (React Example)

```javascript
// REACT COMPONENT: Currency Converter
import React, { useState, useEffect } from 'react';

export function CurrencyConverter() {
  const [sourceAmount, setSourceAmount] = useState('');
  const [quote, setQuote] = useState(null);
  const [status, setStatus] = useState('idle'); // idle, loading, converting, complete

  // Fetch quote on amount change
  useEffect(() => {
    if (!sourceAmount) return;

    const fetchQuote = async () => {
      const data = await fetch('https://api.etrid.org/v1/quote', {
        method: 'POST',
        body: JSON.stringify({
          sourceChain: 'bitcoin',
          targetChain: 'primearc',
          amount: parseFloat(sourceAmount)
        })
      }).then(r => r.json());

      setQuote(data);
    };

    const timer = setTimeout(fetchQuote, 500);
    return () => clearTimeout(timer);
  }, [sourceAmount]);

  const handleConvert = async () => {
    setStatus('loading');

    try {
      // Get bridge address
      const { depositAddress } = await fetch('https://api.etrid.org/v1/bridge-address', {
        method: 'POST',
        body: JSON.stringify({ userWallet: currentWallet })
      }).then(r => r.json());

      // Show address to user for manual transfer or auto-bridge
      displayDepositAddress(depositAddress);

      // Start monitoring
      monitorDeposit(depositAddress, () => {
        setStatus('complete');
      });

      setStatus('converting');
    } catch (error) {
      setStatus('idle');
      showError(error.message);
    }
  };

  return (
    <div className="converter">
      <h2>Convert to ÉTR</h2>

      <div className="input-group">
        <label>You Send</label>
        <div className="currency-input">
          <input
            type="number"
            value={sourceAmount}
            onChange={(e) => setSourceAmount(e.target.value)}
            placeholder="0.5"
          />
          <span className="currency">BTC</span>
        </div>
      </div>

      {quote && (
        <div className="quote-box">
          <div className="receive">
            <label>You Receive</label>
            <div className="amount">{quote.youReceive} ÉTR</div>
            <div className="usd">(${(quote.youReceive * 0.002).toFixed(2)})</div>
          </div>

          <div className="fees">
            <div className="fee-row">
              <span>Exchange Rate</span>
              <span>1 BTC = {quote.exchangeRate} ÉTR</span>
            </div>
            <div className="fee-row">
              <span>Network Fee</span>
              <span>{quote.networkFee} BTC (~${(quote.networkFee * 25000).toFixed(2)}})</span>
            </div>
            <div className="fee-row">
              <span>Gas Fee</span>
              <span>~{quote.etrGasFee} ÉTR</span>
            </div>
          </div>
        </div>
      )}

      <button
        onClick={handleConvert}
        disabled={status !== 'idle' || !sourceAmount}
        className={`convert-btn ${status}`}
      >
        {status === 'idle' && 'Convert Now'}
        {status === 'loading' && 'Loading...'}
        {status === 'converting' && 'Converting...'}
        {status === 'complete' && 'Complete!'}
      </button>

      {status === 'converting' && <ProgressIndicator />}
    </div>
  );
}
```

### 5.2 Wallet Connection (MetaMask + Polkadot.js)

```javascript
// ETHEREUM WALLETS (MetaMask, RainbowKit)
import { useAccount, useConnect } from 'wagmi';

async function connectMetaMask() {
  try {
    // Request account access
    const accounts = await window.ethereum.request({
      method: 'eth_requestAccounts'
    });

    // Add ÈTRID Primearc network to MetaMask
    await window.ethereum.request({
      method: 'wallet_addEthereumChain',
      params: [{
        chainId: '0x539',  // 1337 in hex
        chainName: 'ÈTRID Primearc',
        rpcUrls: ['https://rpc.etrid.org/primearc'],
        nativeCurrency: { name: 'ÉTR', symbol: 'ETR', decimals: 18 },
        blockExplorerUrls: ['https://explorer.etrid.org']
      }]
    });

    return accounts[0];
  } catch (error) {
    console.error('Failed to connect:', error);
  }
}

// SUBSTRATE WALLETS (Polkadot.js)
import { web3FromAddress } from '@polkadot/extension-dapp';

async function connectPolkadotWallet() {
  // Request wallet extension
  const { web3Enable, web3Accounts } = await import('@polkadot/extension-dapp');

  await web3Enable('ÈTRID Converter');
  const accounts = await web3Accounts();

  return accounts[0]?.address;
}

// UNIFIED WALLET CONNECTION
async function initializeWallet() {
  let connectedAddress;

  try {
    // Try Ethereum first
    connectedAddress = await connectMetaMask();
  } catch {
    // Fall back to Substrate
    connectedAddress = await connectPolkadotWallet();
  }

  return {
    address: connectedAddress,
    type: connectedAddress?.startsWith('0x') ? 'evm' : 'substrate'
  };
}
```

### 5.3 RPC Calls to ÈTRID Nodes

```javascript
// FETCH BALANCE
async function getBalance(address, chain = 'primearc') {
  const response = await fetch(`https://rpc.etrid.org/${chain}`, {
    method: 'POST',
    body: JSON.stringify({
      id: 1,
      jsonrpc: '2.0',
      method: 'eth_getBalance',  // For EVM chains
      params: [address, 'latest']
    })
  });

  const { result } = await response.json();
  return parseInt(result, 16) / 1e18;  // Convert wei to ETR
}

// QUERY TOKEN INFO
async function getTokenInfo(contractAddress, chain = 'primearc') {
  const response = await fetch(`https://rpc.etrid.org/${chain}`, {
    method: 'POST',
    body: JSON.stringify({
      id: 1,
      jsonrpc: '2.0',
      method: 'eth_call',
      params: [{
        to: contractAddress,
        data: '0x70a08231' + address.slice(2).padStart(64, '0')
      }, 'latest']
    })
  });

  const { result } = await response.json();
  return parseInt(result, 16);
}

// LISTEN FOR EVENTS (WebSocket)
async function subscribeToSwapEvents() {
  const ws = new WebSocket('wss://ws.etrid.org/primearc');

  ws.onopen = () => {
    ws.send(JSON.stringify({
      id: 1,
      jsonrpc: '2.0',
      method: 'eth_subscribe',
      params: ['logs', {
        address: '0x...',  // IntentRouter address
        topics: ['0x...']  // Swap event signature
      }]
    }));
  };

  ws.onmessage = (event) => {
    const { params } = JSON.parse(event.data);
    if (params.result) {
      handleSwapEvent(params.result);
    }
  };
}
```

### 5.4 Event Listening for Transaction Status

```javascript
// REAL-TIME TX STATUS TRACKING
class TransactionMonitor {
  constructor(txHash, chain = 'primearc') {
    this.txHash = txHash;
    this.chain = chain;
    this.status = 'pending';
    this.callbacks = {};
  }

  // Register callbacks for different statuses
  on(event, callback) {
    if (!this.callbacks[event]) {
      this.callbacks[event] = [];
    }
    this.callbacks[event].push(callback);
    return this;
  }

  async start() {
    // Poll until confirmed
    let confirmations = 0;
    let maxRetries = 120; // 1 hour (30 sec intervals)

    while (confirmations < 6 && maxRetries > 0) {
      try {
        const receipt = await getTransactionReceipt(this.txHash);

        if (receipt) {
          confirmations++;
          this.emit('confirmation', { confirmations, receipt });

          if (confirmations === 1) {
            this.emit('included', receipt);
            this.status = 'included';
          }

          if (confirmations === 6) {
            this.emit('finalized', receipt);
            this.status = 'finalized';
            break;
          }
        }
      } catch (error) {
        this.emit('error', error);
      }

      await delay(30000); // 30 second intervals
      maxRetries--;
    }

    if (maxRetries === 0) {
      this.emit('timeout');
    }
  }

  emit(event, data) {
    if (this.callbacks[event]) {
      this.callbacks[event].forEach(cb => cb(data));
    }
  }
}

// USAGE
const monitor = new TransactionMonitor('0x...');
monitor
  .on('included', (receipt) => {
    updateUI('Transaction confirmed, processing...');
  })
  .on('finalized', (receipt) => {
    updateUI('Transaction complete!');
  })
  .on('error', (error) => {
    showError(`Transaction failed: ${error.message}`);
  })
  .start();
```

### 5.5 Error Handling & Recovery

```javascript
// COMPREHENSIVE ERROR HANDLING
async function handleConversionWithRecovery(params) {
  const maxRetries = 3;
  let lastError;

  for (let attempt = 1; attempt <= maxRetries; attempt++) {
    try {
      // Attempt conversion
      const result = await initiateConversion(params);
      return result;

    } catch (error) {
      lastError = error;

      // Categorize error
      if (error.message.includes('insufficient balance')) {
        throw new Error('Insufficient funds. Please check your balance.');
      }

      if (error.message.includes('network timeout')) {
        if (attempt < maxRetries) {
          console.log(`Retry ${attempt}/${maxRetries}...`);
          await delay(5000 * attempt); // Exponential backoff
          continue;
        }
      }

      if (error.message.includes('bridge unavailable')) {
        throw new Error('Bridge temporarily offline. Please try again in a few minutes.');
      }

      if (error.code === 'LOW_BALANCE_AFTER_FEES') {
        throw new Error(`Insufficient funds for fees. Need ${error.required} ÉTR, have ${error.available} ÉTR`);
      }

      // Unrecoverable error
      throw error;
    }
  }

  throw new Error(`Conversion failed after ${maxRetries} attempts: ${lastError.message}`);
}

// ERROR DISPLAY HELPER
function displayError(error) {
  const userFriendlyMessages = {
    'Insufficient funds': 'Your wallet doesn\'t have enough funds',
    'Network timeout': 'Network took too long. Please try again',
    'Invalid address': 'The address you entered is invalid',
    'Bridge offline': 'The bridge is temporarily unavailable'
  };

  const message = Object.entries(userFriendlyMessages).find(
    ([key]) => error.message.includes(key)
  )?.[1] || 'Something went wrong. Please try again.';

  showNotification({
    type: 'error',
    title: 'Conversion Failed',
    message,
    action: { label: 'Retry', onClick: retry }
  });
}
```

---

## 6. HIDING WRAPPED TOKENS FROM UI

### 6.1 Filter Balance Queries

```javascript
// DO NOT DISPLAY wrapped token balances to users
async function getDisplayableBalances(address) {
  const allBalances = await queryAllTokens(address);

  // Filter OUT wrapped tokens
  const displayableTokens = allBalances.filter(token => {
    const wrappedTokens = ['wBTC', 'wETH', 'wSOL', 'wADA'];
    return !wrappedTokens.includes(token.symbol);
  });

  return displayableTokens;
}

// SHOW ONLY: BTC, ETH, SOL, ADA, DOGE, etc. + ÉTR
// HIDE FROM UI: wBTC, wETH, wSOL, wADA, etc.
```

### 6.2 Transaction Display

```javascript
// When showing transaction history, map wrapped token transfers
function formatTransactionForDisplay(tx) {
  // If tx is wBTC → ÉTR swap, display as "BTC → ÉTR conversion"
  if (tx.from === 'wBTC' && tx.to === 'ÉTR') {
    return {
      type: 'conversion',
      from: 'BTC',
      to: 'ÉTR',
      amount: tx.amount,
      description: 'Currency conversion',
      // Hide the fact it went through wrapped token
    };
  }

  return tx;
}
```

---

## 7. UX PRINCIPLES

### 7.1 Simple Interface

```
Goal: One-click currency conversion

┌─────────────────────────────────────────┐
│ Convert to ÉTR                          │
├─────────────────────────────────────────┤
│                                         │
│  You Send:     [0.5        ]  BTC       │
│  You Receive:  [12,487     ]  ÉTR       │
│                                         │
│  Fee: 0.0002 ÉTR   Rate: 24,974 ÉTR/BTC │
│                                         │
│  [Convert Now]                          │
│                                         │
└─────────────────────────────────────────┘
```

### 7.2 Progress Indicators

```
Step 1: "Waiting for your BTC..."
  ◯ ○ ○ [Confirming on Bitcoin blockchain...]

Step 2: "Processing conversion..."
  ◉ ◯ ○ [Swapping to ÉTR...]

Step 3: "Finalizing..."
  ◉ ◉ ◯ [Transferring to your wallet...]

Step 4: "Complete!"
  ◉ ◉ ◉ [You received 12,487 ÉTR ✓]
```

### 7.3 No Technical Jargon

**Avoid saying:**
- "wBTC minted"
- "Wrapped token swap"
- "IntentRouter executed"

**Instead say:**
- "Converting BTC..."
- "Processing..."
- "Finalizing transaction..."

### 7.4 Transparent Fees

```
Always show:
✓ Network fee (BTC fee, not technical details)
✓ Exchange rate (clear, no hidden spreads)
✓ ÈTRID gas fee (minimal, transparent)
✓ Estimated time (60 min for BTC 6 confirmations)
```

---

## 8. QUICK REFERENCE: API METHODS

| Method | Purpose | Returns |
|--------|---------|---------|
| `getConversionQuote()` | Get swap price & fees | Quote object |
| `getBridgeDepositAddress()` | Get user's bridge address | Address string |
| `initiateConversion()` | Start ÉTR withdrawal | TX hash |
| `getTransactionStatus()` | Check TX status | Status enum |
| `subscribeToEvents()` | Listen for changes | Unsubscribe fn |

---

## 9. RESOURCES

- **Architecture Docs:** `ETRID_NATIVE_OMNICHAIN_FINAL.md`
- **RPC Endpoints:** `https://api.etrid.org/v1/endpoints`
- **GitHub:** `https://github.com/etrid`
- **Support:** `gizzi_io@proton.me`

---

**Last Updated:** December 8, 2025
**Maintained by:** ÈTRID Development Team
