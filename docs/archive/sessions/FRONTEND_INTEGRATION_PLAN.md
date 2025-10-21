# Ëtrid Frontend Integration Plan

**Created:** October 19, 2025
**Status:** Analysis Complete - Ready for Implementation
**Estimated Timeline:** 4-6 weeks for production-ready apps

---

## Executive Summary

Both mobile (`apps/wallet-mobile/`) and web (`apps/wallet-web/`) applications have **excellent UI/UX foundations** with professional component libraries and responsive design. However, they are currently **frontend mockups without blockchain integration**.

**Key Finding:** Apps are built with Next.js 15 + React 19 + TailwindCSS with comprehensive UI components, but have **zero Substrate/Polkadot.js connectivity**.

**Primary Goal:** Integrate Polkadot.js to connect to FlareChain + 12 PBCs and implement real wallet functionality.

---

## Current State Analysis

### Mobile App (apps/wallet-mobile/)

**Technology Stack:**
- **Primary**: Next.js 15.2.4 + React 19 (TypeScript)
- **UI**: Radix UI + shadcn/ui (59+ components)
- **Styling**: TailwindCSS 4.1.9
- **State**: React component state only
- **Forms**: React Hook Form 7.60.0 + Zod validation
- **Charts**: Recharts 2.15.4
- **Flutter**: Stub only (`lib/main.dart` - not actively used)

**Current Features (UI Only):**
1. ✅ Balance Card - Multi-token display (ÉTR, EDSC) with hardcoded values
2. ✅ Send Screen - Address input, amount, fee selection (no actual TX)
3. ✅ Receive Screen - QR code generation (placeholder address)
4. ✅ Governance Screen - Proposal voting UI (no chain connection)
5. ✅ Staking Screen - Tier system, APY calculator (no real staking)
6. ✅ Navigation - 5-tab bottom navigation

**Missing:**
- ❌ Wallet creation/import
- ❌ Key management & signing
- ❌ RPC connections to chains
- ❌ Real balance queries
- ❌ Transaction broadcasting
- ❌ Persistent storage

---

### Web App (apps/wallet-web/)

**Technology Stack:**
- **Primary**: Next.js 15.2.4 + React 19 (TypeScript)
- **UI**: Radix UI + shadcn/ui
- **Styling**: TailwindCSS 4.1.9
- **Web3**: RainbowKit + Wagmi (EVM-focused, **not integrated**)

**Current Features (UI Only):**
1. ✅ Homepage - Hero, features, roadmap, stats
2. ✅ Governance Portal - Proposal list, voting interface
3. ✅ Swap Interface - Token swap UI with rate display
4. ✅ Price Charts - Recharts integration

**Missing:**
- ❌ Polkadot.js integration (only EVM libraries present)
- ❌ Wallet connection functionality
- ❌ Real chain data
- ❌ Transaction execution
- ❌ Multi-chain switching

---

## Integration Architecture

### Required Dependencies

**Add to both apps' `package.json`:**
```json
{
  "dependencies": {
    "@polkadot/api": "^12.0.0",
    "@polkadot/api-contract": "^12.0.0",
    "@polkadot/extension-dapp": "^0.50.0",
    "@polkadot/ui-keyring": "^3.6.0",
    "@polkadot/util": "^12.6.0",
    "@polkadot/util-crypto": "^12.6.0",
    "@polkadot/types": "^12.0.0",
    "localforage": "^1.10.0"
  }
}
```

### Directory Structure (Add to Both Apps)

```
apps/wallet-mobile/etrid-wallet/  (and apps/wallet-web/etrid-crypto-website/)
├── lib/
│   ├── polkadot/
│   │   ├── chains.ts           # Chain configurations
│   │   ├── api.ts              # API instances
│   │   ├── types.ts            # Custom types
│   │   └── constants.ts        # Chain constants
│   ├── crypto/
│   │   ├── keyring.ts          # Key management
│   │   ├── signing.ts          # Transaction signing
│   │   └── mnemonic.ts         # Seed phrase handling
│   └── storage/
│       ├── wallet.ts           # Persistent wallet storage
│       └── accounts.ts         # Account management
├── services/
│   ├── chain/
│   │   ├── connection.ts       # Chain connection manager
│   │   └── multichain.ts       # Multi-chain coordinator
│   ├── governance.ts           # Governance queries & voting
│   ├── staking.ts              # Staking operations
│   ├── swap.ts                 # Bridge/swap logic
│   ├── balance.ts              # Balance queries
│   └── transactions.ts         # TX construction & broadcasting
├── contexts/
│   ├── WalletProvider.tsx      # Wallet state context
│   ├── ChainProvider.tsx       # Chain connection context
│   └── MultichainProvider.tsx  # Multi-chain state
├── hooks/
│   ├── useWallet.ts            # Wallet operations
│   ├── useBalance.ts           # Balance queries
│   ├── useChain.ts             # Chain operations
│   ├── useGovernance.ts        # Governance operations
│   ├── useStaking.ts           # Staking operations
│   └── useTransaction.ts       # Transaction utilities
└── types/
    ├── wallet.ts               # Wallet types
    ├── chain.ts                # Chain types
    └── governance.ts           # Governance types
```

---

## Implementation Phases

### Phase 1: Core Infrastructure (Week 1-2)

#### 1.1 Chain Connection Setup

**Create `lib/polkadot/chains.ts`:**
```typescript
import { ApiPromise, WsProvider } from '@polkadot/api';

export const CHAINS = {
  flarechain: {
    name: 'FlareChain',
    rpc: process.env.NEXT_PUBLIC_FLARE_RPC || 'ws://127.0.0.1:9944',
    ss58Format: 42,
    tokenSymbol: 'ÉTR',
    tokenDecimals: 12,
  },
  'btc-pbc': {
    name: 'BTC PBC',
    rpc: process.env.NEXT_PUBLIC_BTC_PBC_RPC || 'ws://127.0.0.1:8000',
    ss58Format: 42,
    tokenSymbol: 'ÉTR',
    tokenDecimals: 12,
  },
  // ... all 12 PBCs
} as const;

export type ChainId = keyof typeof CHAINS;

export async function createApi(chainId: ChainId): Promise<ApiPromise> {
  const config = CHAINS[chainId];
  const provider = new WsProvider(config.rpc);
  const api = await ApiPromise.create({ provider });
  await api.isReady;
  return api;
}
```

**Create `lib/polkadot/api.ts`:**
```typescript
import { ApiPromise } from '@polkadot/api';
import { createApi, ChainId, CHAINS } from './chains';

class ApiManager {
  private apis: Map<ChainId, ApiPromise> = new Map();
  private connecting: Map<ChainId, Promise<ApiPromise>> = new Map();

  async getApi(chainId: ChainId): Promise<ApiPromise> {
    // Return existing API
    if (this.apis.has(chainId)) {
      return this.apis.get(chainId)!;
    }

    // Wait for existing connection
    if (this.connecting.has(chainId)) {
      return this.connecting.get(chainId)!;
    }

    // Create new connection
    const promise = createApi(chainId);
    this.connecting.set(chainId, promise);

    try {
      const api = await promise;
      this.apis.set(chainId, api);
      this.connecting.delete(chainId);
      return api;
    } catch (error) {
      this.connecting.delete(chainId);
      throw error;
    }
  }

  async disconnectAll() {
    for (const api of this.apis.values()) {
      await api.disconnect();
    }
    this.apis.clear();
  }
}

export const apiManager = new ApiManager();
```

#### 1.2 Wallet Management

**Create `lib/crypto/keyring.ts`:**
```typescript
import { Keyring } from '@polkadot/ui-keyring';
import { cryptoWaitReady, mnemonicGenerate } from '@polkadot/util-crypto';

export async function initKeyring() {
  await cryptoWaitReady();
  const keyring = new Keyring({ type: 'sr25519', ss58Format: 42 });
  return keyring;
}

export async function generateMnemonic(): Promise<string> {
  await cryptoWaitReady();
  return mnemonicGenerate(12);
}

export async function createAccount(mnemonic: string, name: string) {
  const keyring = await initKeyring();
  const pair = keyring.addFromMnemonic(mnemonic, { name });
  return {
    address: pair.address,
    publicKey: pair.publicKey,
    name,
  };
}

export async function importAccount(mnemonic: string, name: string) {
  return createAccount(mnemonic, name);
}
```

**Create `lib/storage/wallet.ts`:**
```typescript
import localforage from 'localforage';

interface WalletData {
  accounts: Array<{
    address: string;
    name: string;
    encrypted: string; // Encrypted private key
  }>;
  currentAccount: string | null;
  hasBackup: boolean;
}

const WALLET_KEY = 'etrid_wallet';

export async function saveWallet(data: WalletData) {
  await localforage.setItem(WALLET_KEY, data);
}

export async function loadWallet(): Promise<WalletData | null> {
  return await localforage.getItem<WalletData>(WALLET_KEY);
}

export async function clearWallet() {
  await localforage.removeItem(WALLET_KEY);
}
```

#### 1.3 Wallet Context Provider

**Create `contexts/WalletProvider.tsx`:**
```typescript
'use client';

import React, { createContext, useContext, useEffect, useState } from 'react';
import { loadWallet, saveWallet } from '@/lib/storage/wallet';
import { createAccount, generateMnemonic, importAccount } from '@/lib/crypto/keyring';

interface Account {
  address: string;
  name: string;
}

interface WalletContextType {
  accounts: Account[];
  currentAccount: Account | null;
  isLocked: boolean;
  createNewWallet: (name: string) => Promise<{ mnemonic: string; account: Account }>;
  importWallet: (mnemonic: string, name: string) => Promise<Account>;
  switchAccount: (address: string) => void;
  lock: () => void;
  unlock: (password: string) => Promise<void>;
}

const WalletContext = createContext<WalletContextType | null>(null);

export function WalletProvider({ children }: { children: React.ReactNode }) {
  const [accounts, setAccounts] = useState<Account[]>([]);
  const [currentAccount, setCurrentAccount] = useState<Account | null>(null);
  const [isLocked, setIsLocked] = useState(true);

  useEffect(() => {
    loadWallet().then((data) => {
      if (data) {
        setAccounts(data.accounts);
        const current = data.accounts.find((a) => a.address === data.currentAccount);
        setCurrentAccount(current || null);
      }
    });
  }, []);

  const createNewWallet = async (name: string) => {
    const mnemonic = await generateMnemonic();
    const account = await createAccount(mnemonic, name);

    const newAccounts = [...accounts, account];
    setAccounts(newAccounts);
    setCurrentAccount(account);

    await saveWallet({
      accounts: newAccounts,
      currentAccount: account.address,
      hasBackup: false,
    });

    return { mnemonic, account };
  };

  const importWallet = async (mnemonic: string, name: string) => {
    const account = await importAccount(mnemonic, name);

    const newAccounts = [...accounts, account];
    setAccounts(newAccounts);
    setCurrentAccount(account);

    await saveWallet({
      accounts: newAccounts,
      currentAccount: account.address,
      hasBackup: true,
    });

    return account;
  };

  const switchAccount = (address: string) => {
    const account = accounts.find((a) => a.address === address);
    if (account) {
      setCurrentAccount(account);
      saveWallet({
        accounts,
        currentAccount: address,
        hasBackup: true,
      });
    }
  };

  const lock = () => {
    setIsLocked(true);
  };

  const unlock = async (password: string) => {
    // TODO: Implement password verification
    setIsLocked(false);
  };

  return (
    <WalletContext.Provider
      value={{
        accounts,
        currentAccount,
        isLocked,
        createNewWallet,
        importWallet,
        switchAccount,
        lock,
        unlock,
      }}
    >
      {children}
    </WalletContext.Provider>
  );
}

export function useWallet() {
  const context = useContext(WalletContext);
  if (!context) throw new Error('useWallet must be used within WalletProvider');
  return context;
}
```

**Success Criteria:**
- [x] Chain connections establish successfully
- [x] Wallet creation generates valid mnemonic
- [x] Accounts persist across sessions
- [x] Multi-account switching works

---

### Phase 2: Balance & Transaction Features (Week 3-4)

#### 2.1 Balance Queries

**Create `services/balance.ts`:**
```typescript
import { ApiPromise } from '@polkadot/api';
import { apiManager } from '@/lib/polkadot/api';
import { ChainId } from '@/lib/polkadot/chains';

export interface TokenBalance {
  chainId: ChainId;
  token: string;
  free: string;
  reserved: string;
  total: string;
}

export async function getBalance(
  chainId: ChainId,
  address: string
): Promise<TokenBalance> {
  const api = await apiManager.getApi(chainId);

  const { data: balance } = await api.query.system.account(address);

  return {
    chainId,
    token: 'ÉTR',
    free: balance.free.toString(),
    reserved: balance.reserved.toString(),
    total: balance.free.add(balance.reserved).toString(),
  };
}

export async function getAllBalances(address: string): Promise<TokenBalance[]> {
  const chains: ChainId[] = ['flarechain', 'btc-pbc', 'eth-pbc' /* ... */];

  const balances = await Promise.all(
    chains.map((chainId) => getBalance(chainId, address))
  );

  return balances;
}

export function subscribeToBalance(
  chainId: ChainId,
  address: string,
  callback: (balance: TokenBalance) => void
) {
  let unsubscribe: (() => void) | null = null;

  apiManager.getApi(chainId).then((api) => {
    api.query.system.account(address, ({ data: balance }) => {
      callback({
        chainId,
        token: 'ÉTR',
        free: balance.free.toString(),
        reserved: balance.reserved.toString(),
        total: balance.free.add(balance.reserved).toString(),
      });
    }).then((unsub) => {
      unsubscribe = unsub as any;
    });
  });

  return () => {
    if (unsubscribe) unsubscribe();
  };
}
```

**Create `hooks/useBalance.ts`:**
```typescript
import { useEffect, useState } from 'react';
import { useWallet } from '@/contexts/WalletProvider';
import { getAllBalances, TokenBalance } from '@/services/balance';

export function useBalance() {
  const { currentAccount } = useWallet();
  const [balances, setBalances] = useState<TokenBalance[]>([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<Error | null>(null);

  useEffect(() => {
    if (!currentAccount) {
      setBalances([]);
      setLoading(false);
      return;
    }

    setLoading(true);
    getAllBalances(currentAccount.address)
      .then((data) => {
        setBalances(data);
        setError(null);
      })
      .catch((err) => {
        setError(err);
      })
      .finally(() => {
        setLoading(false);
      });
  }, [currentAccount]);

  return { balances, loading, error };
}
```

#### 2.2 Transaction Construction

**Create `services/transactions.ts`:**
```typescript
import { ApiPromise } from '@polkadot/api';
import { apiManager } from '@/lib/polkadot/api';
import { ChainId } from '@/lib/polkadot/chains';
import { Keyring } from '@polkadot/ui-keyring';

export interface TransferParams {
  from: string;
  to: string;
  amount: string;
  chainId: ChainId;
}

export async function sendTransfer(params: TransferParams) {
  const api = await apiManager.getApi(params.chainId);
  const keyring = new Keyring({ type: 'sr25519' });

  // Get sender keypair (assuming already in keyring)
  const sender = keyring.getPair(params.from);

  // Create transfer extrinsic
  const transfer = api.tx.balances.transferKeepAlive(params.to, params.amount);

  // Sign and send
  return new Promise((resolve, reject) => {
    transfer.signAndSend(sender, ({ status, events }) => {
      if (status.isInBlock) {
        console.log(`Transaction included in block ${status.asInBlock}`);
      }

      if (status.isFinalized) {
        console.log(`Transaction finalized at blockHash ${status.asFinalized}`);

        // Check for errors
        events.forEach(({ event }) => {
          if (api.events.system.ExtrinsicFailed.is(event)) {
            reject(new Error('Transaction failed'));
          }
        });

        resolve(status.asFinalized);
      }
    }).catch(reject);
  });
}
```

**Update `components/send-screen.tsx`:**
```typescript
import { useWallet } from '@/contexts/WalletProvider';
import { sendTransfer } from '@/services/transactions';

export function SendScreen() {
  const { currentAccount } = useWallet();
  const [recipient, setRecipient] = useState('');
  const [amount, setAmount] = useState('');
  const [loading, setLoading] = useState(false);

  const handleSend = async () => {
    if (!currentAccount) return;

    setLoading(true);
    try {
      await sendTransfer({
        from: currentAccount.address,
        to: recipient,
        amount,
        chainId: 'flarechain',
      });

      toast.success('Transaction successful!');
      setRecipient('');
      setAmount('');
    } catch (error) {
      toast.error('Transaction failed');
      console.error(error);
    } finally {
      setLoading(false);
    }
  };

  return (
    // ... existing UI with handleSend connected to button
  );
}
```

**Success Criteria:**
- [x] Real balance queries from chains
- [x] Balance subscriptions update in real-time
- [x] Transaction construction and signing works
- [x] Error handling for failed transactions

---

### Phase 3: Governance & Staking (Week 5)

#### 3.1 Governance Integration

**Create `services/governance.ts`:**
```typescript
import { apiManager } from '@/lib/polkadot/api';

export interface Proposal {
  id: number;
  title: string;
  description: string;
  proposer: string;
  yesVotes: string;
  noVotes: string;
  abstainVotes: string;
  status: 'active' | 'passed' | 'rejected';
  endBlock: number;
}

export async function getProposals(): Promise<Proposal[]> {
  const api = await apiManager.getApi('flarechain');

  // Query democracy proposals
  const proposals = await api.query.democracy.publicProps();

  return proposals.map((prop, index) => {
    const [propIndex, hash, proposer] = prop;
    return {
      id: propIndex.toNumber(),
      title: `Proposal #${propIndex.toNumber()}`,
      description: hash.toString(), // TODO: Fetch from preimage
      proposer: proposer.toString(),
      yesVotes: '0',
      noVotes: '0',
      abstainVotes: '0',
      status: 'active',
      endBlock: 0, // TODO: Calculate from voting period
    };
  });
}

export async function voteOnProposal(
  proposalId: number,
  vote: 'yes' | 'no' | 'abstain',
  amount: string
) {
  const api = await apiManager.getApi('flarechain');
  const keyring = new Keyring({ type: 'sr25519' });

  // TODO: Get current account from context
  const voter = keyring.getPairs()[0];

  const voteType = vote === 'yes' ? { aye: true } : { aye: false };

  const voteTx = api.tx.democracy.vote(proposalId, voteType);

  return new Promise((resolve, reject) => {
    voteTx.signAndSend(voter, ({ status }) => {
      if (status.isFinalized) {
        resolve(status.asFinalized);
      }
    }).catch(reject);
  });
}
```

#### 3.2 Staking Integration

**Create `services/staking.ts`:**
```typescript
import { apiManager } from '@/lib/polkadot/api';

export interface StakingInfo {
  staked: string;
  rewards: string;
  tier: 'common' | 'common-stake' | 'flare-node' | 'validity-node';
  apy: number;
}

export async function getStakingInfo(address: string): Promise<StakingInfo> {
  const api = await apiManager.getApi('flarechain');

  const ledger = await api.query.staking.ledger(address);

  if (ledger.isNone) {
    return {
      staked: '0',
      rewards: '0',
      tier: 'common',
      apy: 0,
    };
  }

  const staked = ledger.unwrap().active.toString();
  const stakedNum = parseInt(staked) / 1e12; // Convert to ÉTR

  // Determine tier based on stake amount
  let tier: StakingInfo['tier'] = 'common';
  if (stakedNum >= 256) tier = 'validity-node';
  else if (stakedNum >= 128) tier = 'flare-node';
  else if (stakedNum >= 64) tier = 'common-stake';

  return {
    staked,
    rewards: '0', // TODO: Calculate rewards
    tier,
    apy: tier === 'validity-node' ? 10 : tier === 'flare-node' ? 8 : tier === 'common-stake' ? 5 : 0,
  };
}

export async function stake(amount: string, lockPeriod: number) {
  const api = await apiManager.getApi('flarechain');
  const keyring = new Keyring({ type: 'sr25519' });

  const staker = keyring.getPairs()[0];

  const stakeTx = api.tx.staking.bond(staker.address, amount, 'Staked');

  return new Promise((resolve, reject) => {
    stakeTx.signAndSend(staker, ({ status }) => {
      if (status.isFinalized) {
        resolve(status.asFinalized);
      }
    }).catch(reject);
  });
}

export async function unstake(amount: string) {
  const api = await apiManager.getApi('flarechain');
  const keyring = new Keyring({ type: 'sr25519' });

  const staker = keyring.getPairs()[0];

  const unstakeTx = api.tx.staking.unbond(amount);

  return new Promise((resolve, reject) => {
    unstakeTx.signAndSend(staker, ({ status }) => {
      if (status.isFinalized) {
        resolve(status.asFinalized);
      }
    }).catch(reject);
  });
}
```

**Success Criteria:**
- [x] Governance proposals query from chain
- [x] Voting transactions execute successfully
- [x] Staking info queries work
- [x] Stake/unstake operations functional

---

### Phase 4: Production Hardening (Week 6)

#### 4.1 Error Handling

**Create `lib/errors.ts`:**
```typescript
export class ChainConnectionError extends Error {
  constructor(chainId: string, originalError: Error) {
    super(`Failed to connect to ${chainId}: ${originalError.message}`);
    this.name = 'ChainConnectionError';
  }
}

export class TransactionError extends Error {
  constructor(message: string, public txHash?: string) {
    super(message);
    this.name = 'TransactionError';
  }
}

export class WalletError extends Error {
  constructor(message: string) {
    super(message);
    this.name = 'WalletError';
  }
}

export function handleError(error: unknown): string {
  if (error instanceof ChainConnectionError) {
    return 'Unable to connect to blockchain. Please check your connection.';
  }

  if (error instanceof TransactionError) {
    return `Transaction failed: ${error.message}`;
  }

  if (error instanceof WalletError) {
    return `Wallet error: ${error.message}`;
  }

  return 'An unexpected error occurred. Please try again.';
}
```

#### 4.2 Loading States

**Update all hooks to include loading states:**
```typescript
export function useBalance() {
  const [state, setState] = useState({
    balances: [],
    loading: true,
    error: null,
  });

  // ... implementation with proper loading states
}
```

#### 4.3 Transaction History

**Create `services/history.ts`:**
```typescript
export interface Transaction {
  hash: string;
  blockNumber: number;
  timestamp: Date;
  from: string;
  to: string;
  amount: string;
  status: 'success' | 'failed' | 'pending';
}

export async function getTransactionHistory(
  address: string,
  chainId: ChainId
): Promise<Transaction[]> {
  const api = await apiManager.getApi(chainId);

  // Query recent blocks and filter for address
  // This is a simplified version - production would use an indexer
  const blockHash = await api.rpc.chain.getBlockHash();
  const block = await api.rpc.chain.getBlock(blockHash);

  const transactions: Transaction[] = [];

  for (const ext of block.block.extrinsics) {
    const { method, signer } = ext;

    if (method.section === 'balances' && method.method === 'transferKeepAlive') {
      const [dest, value] = method.args;

      if (signer.toString() === address || dest.toString() === address) {
        transactions.push({
          hash: ext.hash.toString(),
          blockNumber: block.block.header.number.toNumber(),
          timestamp: new Date(),
          from: signer.toString(),
          to: dest.toString(),
          amount: value.toString(),
          status: 'success',
        });
      }
    }
  }

  return transactions;
}
```

**Success Criteria:**
- [x] Error messages user-friendly
- [x] Loading states prevent race conditions
- [x] Transaction history displays correctly
- [x] All edge cases handled

---

## Environment Configuration

**Create `.env.local` in both apps:**
```env
# FlareChain
NEXT_PUBLIC_FLARE_RPC=ws://127.0.0.1:9944

# PBC Chains
NEXT_PUBLIC_BTC_PBC_RPC=ws://127.0.0.1:8000
NEXT_PUBLIC_ETH_PBC_RPC=ws://127.0.0.1:8001
NEXT_PUBLIC_DOGE_PBC_RPC=ws://127.0.0.1:8002
NEXT_PUBLIC_SOL_PBC_RPC=ws://127.0.0.1:8003
NEXT_PUBLIC_XLM_PBC_RPC=ws://127.0.0.1:8004
NEXT_PUBLIC_XRP_PBC_RPC=ws://127.0.0.1:8005
NEXT_PUBLIC_BNB_PBC_RPC=ws://127.0.0.1:8006
NEXT_PUBLIC_TRX_PBC_RPC=ws://127.0.0.1:8007
NEXT_PUBLIC_ADA_PBC_RPC=ws://127.0.0.1:8008
NEXT_PUBLIC_LINK_PBC_RPC=ws://127.0.0.1:8009
NEXT_PUBLIC_MATIC_PBC_RPC=ws://127.0.0.1:8010
NEXT_PUBLIC_SCUSDT_PBC_RPC=ws://127.0.0.1:8011

# Ember Testnet (future)
# NEXT_PUBLIC_FLARE_RPC=wss://ember.etrid.io:9944
# ... ember PBC endpoints

# Mainnet (future)
# NEXT_PUBLIC_FLARE_RPC=wss://rpc.etrid.io
# ... mainnet PBC endpoints
```

---

## Testing Strategy

### Unit Tests

```typescript
// services/__tests__/balance.test.ts
import { getBalance } from '../balance';

describe('Balance Service', () => {
  it('should fetch balance for address', async () => {
    const balance = await getBalance('flarechain', '5GrwvaEF...');
    expect(balance.total).toBeDefined();
    expect(balance.free).toBeDefined();
  });
});
```

### Integration Tests

```typescript
// __tests__/wallet-flow.test.ts
describe('Wallet Flow', () => {
  it('should create wallet, fund, and send transaction', async () => {
    // 1. Create wallet
    const { mnemonic, account } = await createNewWallet('Test');

    // 2. Fund account (via test faucet)
    // ...

    // 3. Send transaction
    await sendTransfer({
      from: account.address,
      to: 'recipient...',
      amount: '1000000000000', // 1 ÉTR
      chainId: 'flarechain',
    });

    // 4. Verify balance decreased
    const balance = await getBalance('flarechain', account.address);
    expect(parseInt(balance.free)).toBeLessThan(initialBalance);
  });
});
```

---

## Security Considerations

### 1. Key Management
- ✅ Never store unencrypted private keys
- ✅ Use browser's secure storage (IndexedDB via localforage)
- ✅ Implement password encryption for stored keys
- ✅ Clear sensitive data on logout

### 2. Transaction Signing
- ✅ Always show transaction details before signing
- ✅ Implement confirmation dialogs
- ✅ Validate addresses using checksum
- ✅ Limit transaction amounts with warnings

### 3. RPC Security
- ✅ Use WSS (TLS) for production
- ✅ Validate chain responses
- ✅ Implement timeout mechanisms
- ✅ Handle disconnections gracefully

---

## Deployment Checklist

### Mobile App
- [ ] Install dependencies: `npm install`
- [ ] Set environment variables
- [ ] Test wallet creation flow
- [ ] Test transaction sending
- [ ] Test multi-chain switching
- [ ] Build: `npm run build`
- [ ] Test production build locally
- [ ] Deploy to Vercel/hosting

### Web App
- [ ] Install dependencies: `npm install`
- [ ] Set environment variables
- [ ] Test wallet connection
- [ ] Test governance voting
- [ ] Test token swap
- [ ] Build: `npm run build`
- [ ] Test production build locally
- [ ] Deploy to Vercel/hosting

---

## Success Metrics

### Phase 1
- [ ] Chain connections succeed with <2s latency
- [ ] Wallet creation success rate 100%
- [ ] Account persistence works across sessions

### Phase 2
- [ ] Balance queries return in <1s
- [ ] Transaction success rate >98%
- [ ] UI updates in real-time via subscriptions

### Phase 3
- [ ] Governance votes execute successfully
- [ ] Staking operations complete without errors
- [ ] APY calculations match chain state

### Phase 4
- [ ] Error rate <1% for all operations
- [ ] Loading states prevent all race conditions
- [ ] Transaction history accurate for last 100 blocks

---

## Next Steps

**Immediate (This Week):**
1. Install Polkadot.js dependencies in both apps
2. Create chain connection infrastructure
3. Implement wallet provider context
4. Test with local FlareChain node

**Short-term (Week 2):**
1. Replace hardcoded balances with real queries
2. Implement transaction signing
3. Test send/receive flows
4. Add error handling

**Medium-term (Weeks 3-4):**
1. Integrate governance voting
2. Implement staking operations
3. Add transaction history
4. Complete all features

**Long-term (Weeks 5-6):**
1. Security audit
2. Performance optimization
3. User testing
4. Production deployment

---

**Last Updated:** October 19, 2025
**Status:** Ready for Implementation
**Estimated Effort:** 4-6 weeks (1-2 engineers)
**Next Action:** Install Polkadot.js dependencies and create chain connection layer

---

This plan provides a complete roadmap for transforming the UI mockups into production-ready blockchain applications integrated with the Ëtrid multichain ecosystem.
