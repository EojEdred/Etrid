# Ëtrid Mobile Wallet - Implementation Plan

**Project**: Ëtrid Mobile DeFi Wallet
**Timeline**: 22 weeks (5.5 months)
**Tech Stack**: React Native + Expo + @etrid/sdk
**Status**: 📐 Architecture Complete, Ready for Development

---

## 📊 Current Status

✅ **Completed**:
- Mobile wallet architecture document (MOBILE_WALLET_ARCHITECTURE.md)
- UI/UX design guide with ASCII mockups (UI_UX_DESIGN_GUIDE.md)
- Project folder structure created
- React Native/Expo project initialized
- Package.json with all dependencies
- TypeScript configuration
- Theme system (colors, typography, spacing)
- Navigation structure (tabs + stack)
- Placeholder screens (Home, Accounts, Governance, Portfolio, Settings)
- Ëtrid SDK integration setup

⏳ **Next Steps**:
- Implement core wallet features
- Build authentication flow
- Integrate blockchain functionality
- Add DeFi features (staking, governance)
- Implement ATM integration
- Add Ledger hardware wallet support

---

## 🎯 Phase 1: MVP (Weeks 1-8)

### Week 1-2: Foundation & Authentication

**Goal**: Set up authentication and secure key management

**Tasks**:
1. **Authentication Flow** (3 days)
   - [ ] Create `AuthContext` for state management
   - [ ] Implement biometric authentication (Face ID/Fingerprint)
   - [ ] Add PIN code entry screen
   - [ ] Build onboarding flow (welcome → create/import → backup → verify → biometric)
   - [ ] Test on iOS (Face ID) and Android (Fingerprint)

2. **Key Management** (2 days)
   - [ ] Create `KeychainService` using expo-secure-store
   - [ ] Generate keypair using @polkadot/util-crypto
   - [ ] Store encrypted keypair in SecureStore
   - [ ] Implement recovery phrase generation (12/24 words)
   - [ ] Add phrase verification screen

3. **SDK Integration** (2 days)
   - [ ] Create `EtridSDKService` singleton
   - [ ] Connect to FlareChain RPC (wss://rpc.flarechain.etrid.network)
   - [ ] Initialize all SDK wrappers
   - [ ] Test connection and basic queries

**Files to Create**:
```
src/
├── contexts/
│   └── AuthContext.tsx
├── services/
│   ├── EtridSDKService.ts
│   ├── KeychainService.ts
│   └── BiometricService.ts
├── screens/
│   ├── auth/
│   │   ├── WelcomeScreen.tsx
│   │   ├── CreateWalletScreen.tsx
│   │   ├── ImportWalletScreen.tsx
│   │   ├── BackupPhraseScreen.tsx
│   │   ├── VerifyPhraseScreen.tsx
│   │   └── BiometricSetupScreen.tsx
```

**Example Implementation**:

```typescript
// src/services/EtridSDKService.ts
import { ApiPromise, WsProvider } from '@polkadot/api';
import {
  AccountsWrapper,
  StakingWrapper,
  GovernanceWrapper,
  BridgeWrapper,
  OracleWrapper,
  ReserveVaultWrapper,
  LightningBlocWrapper,
  DistributionPayWrapper
} from '@etrid/sdk';

class EtridSDKService {
  private static instance: EtridSDKService;
  private api: ApiPromise | null = null;
  
  // SDK Wrappers
  public accounts: AccountsWrapper | null = null;
  public staking: StakingWrapper | null = null;
  public governance: GovernanceWrapper | null = null;
  public bridge: BridgeWrapper | null = null;
  public oracle: OracleWrapper | null = null;
  public reserveVault: ReserveVaultWrapper | null = null;
  public lightning: LightningBlocWrapper | null = null;
  public distribution: DistributionPayWrapper | null = null;

  private constructor() {}

  static getInstance(): EtridSDKService {
    if (!EtridSDKService.instance) {
      EtridSDKService.instance = new EtridSDKService();
    }
    return EtridSDKService.instance;
  }

  async connect(): Promise<void> {
    if (this.api) return; // Already connected

    const provider = new WsProvider('wss://rpc.flarechain.etrid.network');
    this.api = await ApiPromise.create({ provider });

    // Initialize wrappers
    this.accounts = new AccountsWrapper(this.api);
    this.staking = new StakingWrapper(this.api);
    this.governance = new GovernanceWrapper(this.api);
    this.bridge = new BridgeWrapper(this.api);
    this.oracle = new OracleWrapper(this.api);
    this.reserveVault = new ReserveVaultWrapper(this.api);
    this.lightning = new LightningBlocWrapper(this.api);
    this.distribution = new DistributionPayWrapper(this.api);

    console.log('✅ Connected to FlareChain');
  }

  async disconnect(): Promise<void> {
    if (this.api) {
      await this.api.disconnect();
      this.api = null;
      console.log('✅ Disconnected from FlareChain');
    }
  }

  getApi(): ApiPromise {
    if (!this.api) throw new Error('Not connected to FlareChain');
    return this.api;
  }
}

export default EtridSDKService;
```

```typescript
// src/services/KeychainService.ts
import * as SecureStore from 'expo-secure-store';
import { Keyring } from '@polkadot/keyring';
import { mnemonicGenerate } from '@polkadot/util-crypto';

class KeychainService {
  private static MNEMONIC_KEY = 'etrid_mnemonic';
  private static ADDRESS_KEY = 'etrid_address';

  // Generate new wallet
  static async generateWallet(): Promise<{ mnemonic: string; address: string }> {
    const mnemonic = mnemonicGenerate(12);
    const keyring = new Keyring({ type: 'sr25519' });
    const pair = keyring.addFromMnemonic(mnemonic);
    const address = pair.address;

    // Store encrypted
    await SecureStore.setItemAsync(this.MNEMONIC_KEY, mnemonic);
    await SecureStore.setItemAsync(this.ADDRESS_KEY, address);

    return { mnemonic, address };
  }

  // Import from mnemonic
  static async importWallet(mnemonic: string): Promise<string> {
    const keyring = new Keyring({ type: 'sr25519' });
    const pair = keyring.addFromMnemonic(mnemonic);
    const address = pair.address;

    await SecureStore.setItemAsync(this.MNEMONIC_KEY, mnemonic);
    await SecureStore.setItemAsync(this.ADDRESS_KEY, address);

    return address;
  }

  // Get keypair for signing
  static async getKeypair() {
    const mnemonic = await SecureStore.getItemAsync(this.MNEMONIC_KEY);
    if (!mnemonic) throw new Error('Wallet not found');

    const keyring = new Keyring({ type: 'sr25519' });
    return keyring.addFromMnemonic(mnemonic);
  }

  // Get address
  static async getAddress(): Promise<string | null> {
    return await SecureStore.getItemAsync(this.ADDRESS_KEY);
  }

  // Clear wallet (logout)
  static async clearWallet(): Promise<void> {
    await SecureStore.deleteItemAsync(this.MNEMONIC_KEY);
    await SecureStore.deleteItemAsync(this.ADDRESS_KEY);
  }
}

export default KeychainService;
```

---

### Week 3-4: Core Wallet Features

**Goal**: Implement send, receive, balance, and transaction history

**Tasks**:
1. **Home Dashboard** (2 days)
   - [ ] Enhance HomeScreen with real data from SDK
   - [ ] Display total balance from AccountsWrapper
   - [ ] Show asset breakdown (ÉTR, BTC, ETH, etc.)
   - [ ] Add pull-to-refresh
   - [ ] Implement balance count-up animation

2. **Send Money Flow** (3 days)
   - [ ] Create SendScreen with 4-step wizard
   - [ ] Step 1: Amount input with USD conversion
   - [ ] Step 2: Recipient (contacts, QR, address)
   - [ ] Step 3: Speed selection (Instant L3, Fast L2, Standard L1)
   - [ ] Step 4: Review and confirm
   - [ ] Biometric authentication before send
   - [ ] Success screen with transaction hash

3. **Receive Money** (1 day)
   - [ ] Create ReceiveScreen with QR code display
   - [ ] Generate QR code using react-native-qrcode-svg
   - [ ] Add copy address button
   - [ ] Share address via system share sheet

4. **Transaction History** (2 days)
   - [ ] Create TransactionHistoryScreen
   - [ ] Fetch transactions from blockchain
   - [ ] Display transaction cards (sent/received/swap/stake)
   - [ ] Add filtering (sent, received, all)
   - [ ] Implement pagination
   - [ ] Add transaction detail modal

**Custom Hooks**:

```typescript
// src/hooks/useBalance.ts
import { useState, useEffect } from 'react';
import EtridSDKService from '../services/EtridSDKService';
import KeychainService from '../services/KeychainService';

export function useBalance() {
  const [balance, setBalance] = useState<string>('0');
  const [balanceUSD, setBalanceUSD] = useState<string>('0');
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    loadBalance();
  }, []);

  const loadBalance = async () => {
    try {
      setLoading(true);
      const sdk = EtridSDKService.getInstance();
      await sdk.connect();

      const address = await KeychainService.getAddress();
      if (!address) return;

      // Get balance
      const bal = await sdk.accounts!.getBalance(address);
      setBalance((Number(bal) / 1e18).toFixed(4));

      // Get USD price
      const price = await sdk.oracle!.getPrice('ETR/USD');
      const usd = (Number(bal) / 1e18) * price;
      setBalanceUSD(usd.toFixed(2));
    } catch (error) {
      console.error('Error loading balance:', error);
    } finally {
      setLoading(false);
    }
  };

  return { balance, balanceUSD, loading, refresh: loadBalance };
}
```

---

### Week 5-6: Portfolio & Cross-Chain

**Goal**: Implement portfolio tracker and cross-chain swaps

**Tasks**:
1. **Portfolio Tracker** (3 days)
   - [ ] Create PortfolioScreen with chart
   - [ ] Display all assets (ÉTR, BTC, ETH, SOL, etc.)
   - [ ] Show 24h/7d/30d/1y performance
   - [ ] Add asset allocation pie chart
   - [ ] Calculate total net worth
   - [ ] Show profit/loss

2. **Cross-Chain Swaps** (3 days)
   - [ ] Create SwapScreen
   - [ ] Asset selection dropdowns (13 chains)
   - [ ] Real-time exchange rate from OracleWrapper
   - [ ] Slippage tolerance settings
   - [ ] Execute bridge via BridgeWrapper
   - [ ] Track bridge status (3 stages)

---

### Week 7-8: Testing & Polish

**Goal**: Bug fixes, testing, and UI polish

**Tasks**:
- [ ] Write unit tests for services
- [ ] Add integration tests for SDK
- [ ] Test on multiple devices (iPhone, Android)
- [ ] Performance optimization
- [ ] Error handling improvements
- [ ] Loading states and skeletons
- [ ] Accessibility improvements
- [ ] Beta testing with early users

---

## 🎯 Phase 2: DeFi Features (Weeks 9-14)

### Week 9-10: Staking

**Tasks**:
1. **Staking Dashboard** (2 days)
   - [ ] Create StakingScreen
   - [ ] Display total staked, APY, daily rewards
   - [ ] List active validators
   - [ ] Show staking history

2. **Stake Flow** (2 days)
   - [ ] Amount input
   - [ ] Duration selection (flexible/1mo/3mo/1yr)
   - [ ] Validator selection (auto or manual)
   - [ ] Auto-compound toggle
   - [ ] Execute stake via StakingWrapper

3. **Unstake Flow** (1 day)
   - [ ] Show unbonding period (28 days)
   - [ ] Countdown timer
   - [ ] Execute unstake

**Implementation**:

```typescript
// src/hooks/useStaking.ts
import { useState, useEffect } from 'react';
import EtridSDKService from '../services/EtridSDKService';
import KeychainService from '../services/KeychainService';

export function useStaking() {
  const [stakingInfo, setStakingInfo] = useState<any>(null);
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    loadStakingInfo();
  }, []);

  const loadStakingInfo = async () => {
    try {
      setLoading(true);
      const sdk = EtridSDKService.getInstance();
      const address = await KeychainService.getAddress();
      if (!address) return;

      const info = await sdk.staking!.getStakingInfo(address);
      setStakingInfo(info);
    } catch (error) {
      console.error('Error loading staking info:', error);
    } finally {
      setLoading(false);
    }
  };

  const stake = async (amount: string, validatorAddress: string) => {
    const sdk = EtridSDKService.getInstance();
    const keypair = await KeychainService.getKeypair();

    // Bond
    await sdk.staking!.bond(keypair, BigInt(Number(amount) * 1e18));

    // Nominate
    await sdk.staking!.nominate(keypair, [validatorAddress]);

    await loadStakingInfo(); // Refresh
  };

  return { stakingInfo, loading, stake, refresh: loadStakingInfo };
}
```

---

### Week 11-12: Governance

**Tasks**:
1. **Governance Screen** (2 days)
   - [ ] List active proposals
   - [ ] Show proposal details
   - [ ] Display current votes (YES/NO %)
   - [ ] Countdown to end date

2. **Voting Flow** (2 days)
   - [ ] Read full proposal text
   - [ ] Vote YES/NO/ABSTAIN
   - [ ] Conviction selection (0-6x)
   - [ ] Execute vote via GovernanceWrapper

3. **Delegation** (1 day)
   - [ ] Delegate voting power screen
   - [ ] Select validator/delegate
   - [ ] Set delegation amount

---

## 🎯 Phase 3: ATM & Hardware (Weeks 15-18)

### Week 15-16: ATM Integration

**Tasks**:
1. **ATM Locator** (2 days)
   - [ ] Create ATMScreen with map view
   - [ ] Integrate react-native-maps
   - [ ] Fetch ATM locations from backend API
   - [ ] Show nearest ATMs with fees/limits
   - [ ] Add list view with search

2. **Withdrawal Flow** (3 days)
   - [ ] Amount selection screen
   - [ ] Asset selection (ÉTR/BTC/ETH)
   - [ ] Fee calculation and display
   - [ ] Generate withdrawal code
   - [ ] QR code display
   - [ ] Expiration countdown (30 min)

**Backend API Needed**:
```typescript
// Backend endpoint: POST /api/v1/atm/withdraw
interface WithdrawalRequest {
  user: string;           // SS58 address
  amount: number;         // USD amount
  asset: string;          // 'ETR', 'BTC', 'ETH'
  atmPartner: string;     // 'Coinme', 'Bitcoin Depot', etc.
  atmLocationId: string;  // ATM location ID
}

interface WithdrawalResponse {
  withdrawalCode: string; // e.g. "8472-3951"
  expiresAt: string;      // ISO timestamp
  fee: number;            // Fee in USD
  total: number;          // Total cost in USD
  txHash: string;         // Blockchain TX hash
}
```

---

### Week 17-18: Ledger Integration

**Tasks**:
1. **Bluetooth Connection** (2 days)
   - [ ] Use react-native-ble-plx
   - [ ] Scan for Ledger Nano X
   - [ ] Pair and connect
   - [ ] Show battery level

2. **Transaction Signing** (2 days)
   - [ ] Detect high-value transactions (>$500)
   - [ ] Prompt for Ledger connection
   - [ ] Send TX to Ledger for approval
   - [ ] Wait for button press
   - [ ] Get signature and submit

3. **Settings Integration** (1 day)
   - [ ] Connected devices section
   - [ ] Manage Ledger
   - [ ] Set security levels

---

## 🎯 Phase 4: Launch (Weeks 19-22)

### Week 19-20: Enterprise Features

**Tasks**:
- [ ] GPU marketplace integration
- [ ] Hyperledger bridge
- [ ] ETH PBC precompiles
- [ ] KYC/AML flow (optional)

### Week 21-22: Launch Prep

**Tasks**:
- [ ] Security audit (third-party)
- [ ] Performance optimization
- [ ] Analytics integration (Mixpanel)
- [ ] Customer support (Intercom)
- [ ] Marketing materials
- [ ] App Store submission (iOS)
- [ ] Play Store submission (Android)

---

## 📦 Quick Commands

```bash
# Development
npm start                 # Start Expo dev server
npm run ios               # Run on iOS
npm run android           # Run on Android

# Testing
npm test                  # Run unit tests
npm run lint              # Run linter
npm run format            # Format code

# Build
eas build --platform ios  # Build for iOS
eas build --platform android # Build for Android

# Submit
eas submit --platform ios     # Submit to App Store
eas submit --platform android # Submit to Play Store
```

---

## 🎉 Next Actions

**Immediate (This Week)**:
1. Run `cd etrid-wallet && npm install` to install dependencies
2. Start Expo: `npm start`
3. Test on simulator: `npm run ios` or `npm run android`
4. Implement authentication flow (Week 1-2 tasks)
5. Integrate Ëtrid SDK and test connection

**Short Term (This Month)**:
1. Complete MVP features (Home, Send, Receive, Portfolio)
2. Test with real FlareChain testnet
3. Get early user feedback
4. Iterate on UX

**Medium Term (Next 3 Months)**:
1. Implement DeFi features (Staking, Governance)
2. Add ATM integration
3. Integrate Ledger hardware wallets
4. Prepare for beta launch

---

## 📊 Success Metrics

**MVP Success Criteria**:
- ✅ User can create wallet and backup phrase
- ✅ User can send/receive ÉTR
- ✅ Transaction history displays correctly
- ✅ Portfolio shows real-time balances
- ✅ App works on both iOS and Android
- ✅ <1% crash rate

**Launch Success Criteria**:
- ✅ 1,000+ downloads in first month
- ✅ 4.0+ rating on App Store/Play Store
- ✅ 30% 30-day retention
- ✅ $50K+ transaction volume

---

**This implementation plan provides a clear roadmap from architecture to production deployment. Follow the phases sequentially for best results!** 🚀

---

**Document Version**: 1.0
**Last Updated**: November 18, 2025
**Status**: Ready for Development
