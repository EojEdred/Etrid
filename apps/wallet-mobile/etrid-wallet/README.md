# Ëtrid Mobile Wallet

**The world's first comprehensive crypto bank account** - Mobile DeFi wallet with ATM access, cold storage integration, and full DeFi features.

## 🎯 Overview

Ëtrid Wallet is a mobile application that combines:
- Bank-like UX (checking/savings accounts)
- Crypto ATM access (cash withdrawals at 50K+ locations)
- Ledger/DEGN cold storage wallets as "debit cards"
- Direct staking and governance voting
- Cross-chain support (13 blockchains)
- Lightning-fast payments (500K TPS)

## 🚀 Quick Start

### Prerequisites

- Node.js 18+ and npm/yarn
- iOS: Xcode 14+ and CocoaPods
- Android: Android Studio and JDK 11+
- Expo CLI: `npm install -g expo-cli`

### Installation

```bash
# Navigate to project directory
cd apps/wallet-mobile/etrid-wallet

# Install dependencies
npm install

# Start development server
npm start

# Run on iOS simulator
npm run ios

# Run on Android emulator
npm run android

# Run on web browser
npm run web
```

## 📱 Features

### Core Features (MVP)
- ✅ Home Dashboard with total balance
- ✅ Send/Receive with QR codes
- ✅ Transaction history
- ✅ Portfolio tracker
- ⏳ Cross-chain swaps (13 chains)
- ⏳ Price feeds (real-time)

### Banking Features
- ⏳ Checking Account (daily spending)
- ⏳ Savings Account (15% APY via DeFi)
- ⏳ Asset breakdown charts
- ⏳ Bill payments

### DeFi Features
- ⏳ Staking (10-15% APY)
- ⏳ Governance voting
- ⏳ Conviction levels (1x-6x)
- ⏳ Auto-compound rewards

### Advanced Features
- ⏳ ATM cash withdrawals (Coinme, Bitcoin Depot)
- ⏳ Ledger Nano X integration (Bluetooth)
- ⏳ Lightning-Bloc channels (instant payments)
- ⏳ GPU marketplace
- ⏳ Hyperledger bridge

## 🏗️ Project Structure

```
etrid-wallet/
├── src/
│   ├── components/          # Reusable UI components
│   │   ├── BalanceCard.tsx
│   │   ├── AccountCard.tsx
│   │   ├── TransactionItem.tsx
│   │   └── ...
│   ├── screens/             # Screen components
│   │   ├── HomeScreen.tsx
│   │   ├── SendScreen.tsx
│   │   ├── ReceiveScreen.tsx
│   │   ├── StakingScreen.tsx
│   │   ├── GovernanceScreen.tsx
│   │   ├── ATMScreen.tsx
│   │   └── ...
│   ├── navigation/          # Navigation configuration
│   │   ├── RootNavigator.tsx
│   │   ├── types.ts
│   │   └── ...
│   ├── services/            # SDK integrations
│   │   ├── EtridSDKService.ts
│   │   ├── AccountService.ts
│   │   ├── StakingService.ts
│   │   ├── GovernanceService.ts
│   │   └── ...
│   ├── hooks/               # Custom React hooks
│   │   ├── useBalance.ts
│   │   ├── useStaking.ts
│   │   ├── useGovernance.ts
│   │   └── ...
│   ├── utils/               # Helper functions
│   │   ├── formatters.ts
│   │   ├── validators.ts
│   │   └── ...
│   ├── theme/               # Design system
│   │   ├── theme.ts
│   │   ├── colors.ts
│   │   └── typography.ts
│   └── types/               # TypeScript types
│       └── index.ts
├── assets/                  # Images, fonts, etc.
├── App.tsx                  # Root component
├── app.json                 # Expo configuration
├── package.json
├── tsconfig.json
└── README.md
```

## 🎨 Design System

### Colors
- **Primary**: `#6C5CE7` (Purple - Ëtrid brand)
- **Secondary**: `#00B894` (Green - success)
- **Accent**: `#FD79A8` (Pink - highlights)
- **Background**: `#FFFFFF` (Light), `#1E1E1E` (Dark)
- **Text**: `#2D3436` (Dark), `#DFE6E9` (Light)

### Typography
- **Headings**: Inter Bold (24-32px)
- **Body**: Inter Regular (14-16px)
- **Monospace**: JetBrains Mono (addresses, amounts)

### Spacing
- **xs**: 4px
- **sm**: 8px
- **md**: 16px
- **lg**: 24px
- **xl**: 32px
- **xxl**: 48px

## 🔧 SDK Integration

The app uses the Ëtrid JavaScript SDK for all blockchain interactions:

```typescript
import { AccountsWrapper, StakingWrapper, GovernanceWrapper } from '@etrid/sdk';
import { ApiPromise, WsProvider } from '@polkadot/api';

// Connect to FlareChain
const provider = new WsProvider('wss://rpc.flarechain.etrid.network');
const api = await ApiPromise.create({ provider });

// Use SDK wrappers
const accounts = new AccountsWrapper(api);
const staking = new StakingWrapper(api);
const governance = new GovernanceWrapper(api);

// Example: Get balance
const balance = await accounts.getBalance(address);
console.log(`Balance: ${balance / 1e18} ÉTR`);
```

### Available Wrappers
1. **AccountsWrapper** - Send, receive, balances
2. **StakingWrapper** - Stake, unstake, rewards
3. **GovernanceWrapper** - Vote, delegate, proposals
4. **BridgeWrapper** - Cross-chain transfers (13 chains)
5. **OracleWrapper** - Price feeds
6. **ReserveVaultWrapper** - DeFi lending/borrowing
7. **LightningBlocWrapper** - Instant payments (L3)
8. **DistributionPayWrapper** - Daily rewards
9. **EtwasmVMWrapper** - Smart contracts
10. **AIDidWrapper** - AI identities
11. **GPURegistryWrapper** - GPU marketplace
12. **LedgerHardwareWrapper** - Hardware wallets
13. **ETHPBCPrecompileWrapper** - Ethereum L2 integration

## 📋 Development Roadmap

### Phase 1: MVP (Weeks 1-8)
- [ ] Authentication (biometric + PIN)
- [ ] Home dashboard
- [ ] Send/receive
- [ ] Transaction history
- [ ] Portfolio tracker
- [ ] Basic tests

### Phase 2: DeFi (Weeks 9-14)
- [ ] Staking dashboard
- [ ] Governance voting
- [ ] Lightning-Bloc channels
- [ ] Distribution rewards
- [ ] Advanced charts

### Phase 3: ATM & Hardware (Weeks 15-18)
- [ ] ATM location map
- [ ] Cash withdrawal flow
- [ ] Ledger Nano X Bluetooth
- [ ] Multi-account support

### Phase 4: Launch (Weeks 19-22)
- [ ] Security audit
- [ ] Performance optimization
- [ ] App Store submission
- [ ] Marketing materials

## 🧪 Testing

```bash
# Run unit tests
npm test

# Run with coverage
npm test -- --coverage

# Run linter
npm run lint

# Format code
npm run format
```

## 🔐 Security

### Key Storage
- **iOS**: Keychain with Secure Enclave
- **Android**: Keystore with TEE
- **Encryption**: AES-256-GCM

### Transaction Security
- <$100: Biometric only
- $100-$500: Biometric + PIN
- >$500: Ledger/DEGN required
- >$5,000: Ledger + 2FA

### Best Practices
- Never store private keys in AsyncStorage
- Use SecureStore for sensitive data
- Implement certificate pinning
- Enable biometric authentication
- Support hardware wallets

## 📚 Documentation

- [Architecture](../MOBILE_WALLET_ARCHITECTURE.md) - Complete technical architecture
- [UI/UX Design](../UI_UX_DESIGN_GUIDE.md) - Screen mockups and design system
- [SDK Documentation](../../../13-developer-tools/sdk/README.md) - SDK usage guide
- [Contributing](../../../CONTRIBUTING.md) - Contribution guidelines

## 🤝 Contributing

We welcome contributions! Please see [CONTRIBUTING.md](../../../CONTRIBUTING.md) for details.

## 📄 License

MIT License - see [LICENSE](../../../LICENSE) for details.

## 🆘 Support

- GitHub Issues: https://github.com/etrid/etrid-protocol/issues
- Discord: https://discord.gg/etrid
- Email: support@etrid.network

## 🎉 Acknowledgments

Built with:
- React Native + Expo
- Ëtrid JavaScript SDK
- Polkadot.js API
- React Navigation
- React Native Paper

---

**Status**: 🚧 In Development
**Version**: 1.0.0-alpha
**Last Updated**: November 18, 2025
