# AI Assistant Prompts for ÉTRID Lightning Network

This document contains 4 ready-to-use AI prompts for rapidly expanding the ÉTRID Lightning Network ecosystem. Copy any prompt and paste it into Claude, ChatGPT, or another AI assistant to generate production-ready code.

---

## Prompt #1: Landing Page for etrid.org/lightning

```
I need you to create a complete, production-ready landing page for the ÉTRID Lightning Network. This will be deployed at https://etrid.org/lightning.

### Context

ÉTRID is a revolutionary blockchain platform with 14 Parachain-Based Chains (PBCs) including:
- Ethereum PBC
- Bitcoin PBC
- Solana PBC
- Cardano PBC
- Polkadot PBC
- Avalanche PBC
- Polygon PBC
- Algorand PBC
- Cosmos PBC
- Tezos PBC
- Flare PBC
- Hedera PBC
- NEAR PBC
- Aptos PBC

The Lightning Network enables instant, low-cost payments across all 14 chains with:
- Cross-chain routing
- Auto-discovery of new PBCs
- BOLT-11 compatible invoices
- QR code payments
- Multi-hop routing
- Watchtower services
- Fraud proofs
- Optimistic rollups

### Requirements

Create a Next.js 14 landing page with the following sections:

1. **Hero Section**
   - Compelling headline: "Lightning-Fast Payments Across 14 Blockchains"
   - Subheading about instant cross-chain transfers
   - CTA button: "Try Demo" and "Read Docs"
   - Animated background showing network connections between chains

2. **Features Grid**
   - 6-8 feature cards with icons
   - Features: Instant payments, Cross-chain, Low fees, Secure, Scalable, BOLT-11 compatible
   - Each card should have icon, title, description

3. **How It Works**
   - 4-step process with animations
   - Step 1: Open channel on any PBC
   - Step 2: Route payment through network
   - Step 3: Payment settles instantly
   - Step 4: Close channel and settle on-chain
   - Include visual diagram

4. **Supported Chains**
   - Grid of all 14 PBCs with logos
   - Chain name, symbol, status (Live/Coming Soon)
   - Use placeholder images for logos

5. **Live Statistics**
   - Total Value Locked (TVL)
   - Active Channels
   - Total Transactions
   - Average Fee
   - Use animated counters

6. **Use Cases**
   - E-commerce payments
   - Gaming microtransactions
   - Streaming payments
   - Cross-border remittances
   - Each with illustration

7. **Demo Section**
   - Interactive demo showing:
     - Create invoice
     - Generate QR code
     - Scan and pay
   - Real-time updates

8. **Developer Section**
   - Code examples in multiple languages
   - "npm install @etrid/lightning-sdk"
   - Quick start guide
   - API reference link

9. **Roadmap Timeline**
   - Q1 2025: Multi-path payments
   - Q2 2025: Submarine swaps
   - Q3 2025: Lightning DEX
   - Q4 2025: DAO governance

10. **Footer**
    - Links to docs, GitHub, Discord, Twitter
    - Newsletter signup
    - Copyright info

### Technical Stack

- Next.js 14 with App Router
- TypeScript
- Tailwind CSS
- Framer Motion for animations
- Recharts for statistics
- QRCode.react for QR codes
- React Icons

### Design Guidelines

- Modern, clean design
- Dark mode primary (with light mode toggle)
- Lightning bolt motif throughout
- Purple/blue gradient color scheme
- Responsive (mobile, tablet, desktop)
- Smooth animations and transitions
- Lightning-themed illustrations

### Deliverables

Provide the complete code structure:

```
/lightning
├── app/
│   ├── page.tsx (main page)
│   ├── layout.tsx
│   └── globals.css
├── components/
│   ├── Hero.tsx
│   ├── Features.tsx
│   ├── HowItWorks.tsx
│   ├── SupportedChains.tsx
│   ├── Statistics.tsx
│   ├── UseCases.tsx
│   ├── Demo.tsx
│   ├── Developer.tsx
│   ├── Roadmap.tsx
│   └── Footer.tsx
├── lib/
│   ├── types.ts
│   └── constants.ts
├── public/
│   └── (images)
├── package.json
└── README.md
```

Include:
1. All component files with full implementation
2. package.json with all dependencies
3. Tailwind config
4. TypeScript config
5. README with setup instructions
6. Deployment guide for Vercel/Netlify
7. SEO optimization (meta tags, OG images)

Make it visually stunning, highly performant, and conversion-optimized.
```

---

## Prompt #2: MetaMask Browser Extension

```
Create a production-ready browser extension that integrates ÉTRID Lightning Network payments into MetaMask.

### Context

ÉTRID Lightning Network is a Layer 2 payment system spanning 14 blockchains:
- Ethereum, Bitcoin, Solana, Cardano, Polkadot, Avalanche, Polygon, Algorand, Cosmos, Tezos, Flare, Hedera, NEAR, Aptos

The extension should add Lightning functionality to MetaMask, allowing users to:
- Open/close Lightning channels
- Send instant Lightning payments
- Create/scan payment invoices
- View channel balances
- Route cross-chain payments
- Monitor payment history

### Requirements

#### Core Features

1. **Channel Management**
   - Open new Lightning channel (select PBC, amount, counterparty)
   - View active channels (balance, capacity, status)
   - Close channels (cooperative or force-close)
   - Channel rebalancing suggestions

2. **Payments**
   - Create payment invoice with QR code
   - Scan invoice from camera or paste
   - Send Lightning payment
   - Show routing path visualization
   - Real-time payment status updates

3. **Invoice System**
   - Generate BOLT-11 compatible invoices
   - Set amount, description, expiry
   - QR code display
   - Copy to clipboard
   - Share via various methods

4. **Transaction History**
   - List all Lightning payments
   - Filter by chain, status, date
   - Export to CSV
   - Payment details modal

5. **Multi-Chain Support**
   - Switch between 14 PBCs
   - Show balances for each chain
   - Cross-chain payment routing
   - Exchange rate display

6. **Settings**
   - RPC endpoint configuration
   - Watchtower service selection
   - Fee preferences
   - Security options (2FA)

#### Technical Implementation

**Architecture:**
```
/metamask-lightning-extension
├── manifest.json (v3)
├── background/
│   ├── service-worker.ts
│   ├── channel-manager.ts
│   ├── payment-handler.ts
│   └── api-client.ts
├── popup/
│   ├── App.tsx
│   ├── pages/
│   │   ├── Home.tsx
│   │   ├── Channels.tsx
│   │   ├── Send.tsx
│   │   ├── Receive.tsx
│   │   ├── History.tsx
│   │   └── Settings.tsx
│   └── components/
│       ├── ChannelCard.tsx
│       ├── InvoiceGenerator.tsx
│       ├── QRScanner.tsx
│       ├── PaymentForm.tsx
│       └── RouteVisualizer.tsx
├── content/
│   └── injector.ts (inject Lightning API into web pages)
├── lib/
│   ├── lightning-client.ts
│   ├── invoice.ts
│   ├── routing.ts
│   └── crypto.ts
└── styles/
    └── global.css
```

**Tech Stack:**
- TypeScript
- React 18
- Webpack 5
- ethers.js / web3.js
- QR code libraries
- IndexedDB for local storage
- WebSockets for real-time updates

**MetaMask Integration:**
- Use MetaMask API to access wallet
- Request signatures for Lightning operations
- Detect network/chain changes
- Share transaction history

**Security:**
- No private key storage (use MetaMask)
- Signature verification
- HTTPS-only API calls
- Content Security Policy
- XSS protection

#### Key Components

**1. Channel Manager**
```typescript
interface LightningChannel {
  id: string;
  chainId: string;
  localBalance: bigint;
  remoteBalance: bigint;
  status: 'opening' | 'active' | 'closing' | 'closed';
  counterparty: string;
}

class ChannelManager {
  async openChannel(params: OpenChannelParams): Promise<string>
  async closeChannel(channelId: string): Promise<void>
  async getChannels(): Promise<LightningChannel[]>
  async rebalance(channelId: string): Promise<void>
}
```

**2. Payment Handler**
```typescript
interface PaymentRequest {
  invoice: string;
  amount: bigint;
  destinationChain: string;
}

class PaymentHandler {
  async sendPayment(request: PaymentRequest): Promise<PaymentResult>
  async createInvoice(params: InvoiceParams): Promise<string>
  async decodeInvoice(invoice: string): Promise<InvoiceData>
}
```

**3. Route Finder**
```typescript
class RouteFinder {
  async findRoute(from: string, to: string, amount: bigint): Promise<Route>
  async estimateFees(route: Route): Promise<bigint>
}
```

#### UI/UX Requirements

- Clean, modern interface matching MetaMask style
- Dark mode support
- Smooth animations
- Real-time updates (WebSocket)
- Error handling with user-friendly messages
- Loading states for all async operations
- Tooltips explaining Lightning concepts
- Onboarding tutorial for first-time users

#### API Integration

Connect to ÉTRID Lightning Network RPC:
```typescript
const API_ENDPOINTS = {
  mainnet: 'https://lightning-rpc.etrid.org',
  testnet: 'https://lightning-testnet.etrid.org'
};
```

API Methods:
- `lightning_openChannel`
- `lightning_closeChannel`
- `lightning_sendPayment`
- `lightning_createInvoice`
- `lightning_getChannels`
- `lightning_getRoute`
- `lightning_getHistory`

### Deliverables

1. Complete extension code (all files)
2. manifest.json (Chrome/Firefox compatible)
3. package.json with all dependencies
4. webpack.config.js
5. Build scripts
6. README.md with:
   - Installation instructions
   - Development setup
   - Testing guide
   - Publishing to Chrome/Firefox stores
7. Screenshots for store listings
8. Privacy policy
9. Store description

Make it production-ready with robust error handling, excellent UX, and comprehensive documentation.
```

---

## Prompt #3: React Native Mobile App

```
Create a production-ready iOS and Android mobile app for ÉTRID Lightning Network payments.

### Context

ÉTRID Lightning Network enables instant, low-cost payments across 14 blockchains:
- Ethereum, Bitcoin, Solana, Cardano, Polkadot, Avalanche, Polygon, Algorand, Cosmos, Tezos, Flare, Hedera, NEAR, Aptos

The app should be a complete Lightning wallet with:
- Send/receive Lightning payments
- QR code scanning
- Channel management
- Multi-chain support
- Transaction history
- NFC payments (tap-to-pay)
- Biometric authentication

### Requirements

#### Core Features

1. **Wallet Management**
   - Create new wallet (seed phrase generation)
   - Import existing wallet (12/24 word phrase)
   - Multiple wallet accounts
   - Backup and restore
   - Biometric unlock (Face ID/Touch ID/Fingerprint)

2. **Lightning Channels**
   - Open channel (select chain, amount, node)
   - View active channels (list and details)
   - Close channels (cooperative/force)
   - Auto-pilot for channel management
   - Incoming channel requests

3. **Payments**
   - Send payment (scan QR, paste invoice, NFC)
   - Receive payment (generate invoice/QR)
   - Payment confirmation with haptic feedback
   - Multi-hop routing visualization
   - Fee estimation

4. **Invoice System**
   - Create invoice (amount, description, expiry)
   - QR code generation
   - Share invoice (message, email, social)
   - Invoice templates for recurring payments
   - Payment links

5. **Transaction History**
   - Chronological list of all payments
   - Filter by chain, type, status, date range
   - Search functionality
   - Export to CSV/PDF
   - Transaction details view

6. **Multi-Chain Support**
   - Switch between 14 PBCs
   - Show balances for each chain
   - Cross-chain payment routing
   - Real-time exchange rates
   - Chain-specific settings

7. **Contacts**
   - Save frequent recipients
   - Nickname contacts
   - Recent transactions per contact
   - Contact groups

8. **Settings**
   - Network selection (mainnet/testnet)
   - RPC endpoints
   - Watchtower services
   - Notification preferences
   - Security settings
   - Theme (light/dark/auto)
   - Language selection

9. **NFC Payments**
   - Tap phone to pay/receive
   - NFC detection and handling
   - Visual/haptic feedback

10. **Push Notifications**
    - Payment received
    - Channel opened/closed
    - Low balance alerts
    - Security alerts

#### Technical Implementation

**Architecture:**
```
/etrid-lightning-mobile
├── src/
│   ├── screens/
│   │   ├── Auth/
│   │   │   ├── Welcome.tsx
│   │   │   ├── CreateWallet.tsx
│   │   │   ├── ImportWallet.tsx
│   │   │   └── Unlock.tsx
│   │   ├── Home/
│   │   │   ├── Dashboard.tsx
│   │   │   ├── Channels.tsx
│   │   │   └── Settings.tsx
│   │   ├── Payment/
│   │   │   ├── Send.tsx
│   │   │   ├── Receive.tsx
│   │   │   ├── ScanQR.tsx
│   │   │   └── Confirm.tsx
│   │   └── History/
│   │       ├── Transactions.tsx
│   │       └── Details.tsx
│   ├── components/
│   │   ├── ChannelCard.tsx
│   │   ├── InvoiceDisplay.tsx
│   │   ├── QRScanner.tsx
│   │   ├── PaymentForm.tsx
│   │   ├── BalanceCard.tsx
│   │   └── RouteVisualizer.tsx
│   ├── navigation/
│   │   ├── AppNavigator.tsx
│   │   ├── AuthNavigator.tsx
│   │   └── TabNavigator.tsx
│   ├── services/
│   │   ├── LightningClient.ts
│   │   ├── WalletService.ts
│   │   ├── ChannelManager.ts
│   │   ├── PaymentService.ts
│   │   ├── InvoiceService.ts
│   │   └── NotificationService.ts
│   ├── store/
│   │   ├── wallet.ts (Redux slice)
│   │   ├── channels.ts
│   │   ├── payments.ts
│   │   └── settings.ts
│   ├── utils/
│   │   ├── crypto.ts
│   │   ├── storage.ts
│   │   ├── validation.ts
│   │   └── formatters.ts
│   └── types/
│       └── index.ts
├── android/
├── ios/
├── package.json
└── README.md
```

**Tech Stack:**
- React Native 0.73+
- TypeScript
- React Navigation 6
- Redux Toolkit + RTK Query
- React Native Reanimated 3
- React Native Gesture Handler
- React Native Vision Camera (QR scanning)
- React Native Biometrics
- React Native NFC Manager
- React Native Push Notifications
- React Native Keychain (secure storage)
- React Native QRCode SVG

**Native Modules:**
- Camera (QR scanning)
- Biometrics (Face ID, Touch ID, Fingerprint)
- NFC (tap-to-pay)
- Secure storage (Keychain/Keystore)
- Push notifications

#### Key Services

**1. Lightning Client**
```typescript
class LightningClient {
  async connect(endpoint: string): Promise<void>
  async openChannel(params: OpenChannelParams): Promise<string>
  async closeChannel(channelId: string): Promise<void>
  async sendPayment(invoice: string): Promise<PaymentResult>
  async createInvoice(params: InvoiceParams): Promise<string>
  async getChannels(): Promise<Channel[]>
  async getBalance(): Promise<Balance>
}
```

**2. Wallet Service**
```typescript
class WalletService {
  async createWallet(): Promise<{ address: string; mnemonic: string }>
  async importWallet(mnemonic: string): Promise<string>
  async signMessage(message: string): Promise<string>
  async encrypt(data: string, password: string): Promise<string>
  async decrypt(encrypted: string, password: string): Promise<string>
}
```

**3. NFC Service**
```typescript
class NFCService {
  async readNFC(): Promise<Invoice>
  async writeNFC(invoice: string): Promise<void>
  async enableNFC(): Promise<void>
  onNFCDetected(callback: (data: string) => void): void
}
```

#### UI/UX Requirements

- **Design System:**
  - Modern, clean interface
  - Lightning-themed color palette (purple, blue, electric)
  - Smooth animations (60fps)
  - Haptic feedback for key actions
  - Dark mode by default (light mode available)

- **Accessibility:**
  - Large touch targets (min 44x44)
  - High contrast text
  - Screen reader support
  - Font scaling support

- **Onboarding:**
  - Welcome tutorial (3-5 slides)
  - Interactive demo mode
  - Security best practices education
  - Backup reminder

- **Error Handling:**
  - User-friendly error messages
  - Retry mechanisms
  - Offline mode support
  - Network error recovery

#### Security

- Seed phrase encrypted with device keychain
- Biometric authentication
- Auto-lock after inactivity
- Transaction confirmation prompts
- No logging of sensitive data
- Certificate pinning for API calls
- Jailbreak/root detection

#### API Integration

ÉTRID Lightning RPC:
```
Mainnet: https://lightning-rpc.etrid.org
Testnet: https://lightning-testnet.etrid.org
```

WebSocket for real-time updates:
```
wss://lightning-ws.etrid.org
```

#### Testing

Include:
- Unit tests (Jest)
- Component tests (React Native Testing Library)
- E2E tests (Detox)
- Test coverage > 80%

### Deliverables

1. Complete React Native project
2. iOS and Android native code
3. package.json with dependencies
4. README.md with:
   - Setup instructions
   - Development guide
   - Build/deployment guide
   - Testing guide
5. Screenshots for App Store/Play Store
6. App Store Connect metadata
7. Google Play metadata
8. Privacy policy
9. Terms of service
10. CI/CD pipeline (GitHub Actions)

Make it production-ready with excellent performance, beautiful UI, and comprehensive documentation for both users and developers.
```

---

## Prompt #4: Solana Wallet Adapter

```
Create a Lightning Network wallet adapter that integrates with popular Solana wallets (Phantom, Solflare, Backpack, Sollet, etc.) to enable Lightning payments from Solana dApps.

### Context

ÉTRID Lightning Network includes Solana as one of 14 supported PBCs. This adapter will allow Solana dApps to accept Lightning payments without implementing the full Lightning protocol.

Users should be able to:
- Pay Lightning invoices from their Solana wallet
- Receive Lightning payments to their Solana address
- Open/close Lightning channels from Solana dApps
- Route payments across all 14 chains
- View Lightning transaction history

### Requirements

#### Core Features

1. **Wallet Connection**
   - Connect to any Solana Wallet Adapter-compatible wallet
   - Detect installed wallets
   - Handle wallet switching
   - Auto-reconnect on page load
   - Display connection status

2. **Lightning Payments**
   - Send Lightning payment (decode invoice, show details, confirm)
   - Receive payment (generate invoice, display QR, monitor status)
   - Multi-hop routing
   - Fee estimation
   - Payment proof

3. **Channel Management**
   - Open Lightning channel from Solana
   - View active channels
   - Close channels
   - Channel recommendations

4. **Invoice Handling**
   - Create BOLT-11 invoices
   - Decode and validate invoices
   - QR code generation
   - Payment link creation
   - Invoice expiration tracking

5. **Cross-Chain Bridge**
   - Bridge SOL to other PBCs
   - Route payments via most efficient path
   - Exchange rate display
   - Slippage protection

#### Technical Implementation

**Architecture:**
```
/@etrid/solana-lightning-adapter
├── src/
│   ├── adapter.ts (main adapter class)
│   ├── provider.tsx (React context provider)
│   ├── hooks/
│   │   ├── useConnection.ts
│   │   ├── useSendPayment.ts
│   │   ├── useReceivePayment.ts
│   │   ├── useChannels.ts
│   │   └── useInvoice.ts
│   ├── components/
│   │   ├── ConnectButton.tsx
│   │   ├── PaymentModal.tsx
│   │   ├── InvoiceDisplay.tsx
│   │   ├── ChannelList.tsx
│   │   └── QRCode.tsx
│   ├── utils/
│   │   ├── lightning-client.ts
│   │   ├── invoice.ts
│   │   ├── routing.ts
│   │   └── formatters.ts
│   ├── types/
│   │   └── index.ts
│   └── constants.ts
├── examples/
│   ├── nextjs/
│   ├── react/
│   └── vanilla/
├── package.json
├── tsconfig.json
└── README.md
```

**Tech Stack:**
- TypeScript
- React 18
- @solana/wallet-adapter-react
- @solana/web3.js
- ethers.js (for multi-chain)
- QR code library
- WebSocket client

**Adapter Interface:**
```typescript
interface LightningAdapter extends BaseWalletAdapter {
  // Connection
  connect(): Promise<void>;
  disconnect(): Promise<void>;

  // Payments
  sendPayment(invoice: string): Promise<PaymentResult>;
  createInvoice(params: InvoiceParams): Promise<string>;

  // Channels
  openChannel(params: ChannelParams): Promise<string>;
  closeChannel(channelId: string): Promise<void>;
  getChannels(): Promise<Channel[]>;

  // Routing
  findRoute(destination: string, amount: bigint): Promise<Route>;
  estimateFees(route: Route): Promise<bigint>;

  // Events
  on(event: 'paymentReceived' | 'paymentSent' | 'channelOpened', handler: Function): void;
}
```

#### Key Components

**1. Lightning Provider**
```tsx
import { LightningProvider } from '@etrid/solana-lightning-adapter';

function App() {
  return (
    <LightningProvider network="mainnet">
      <YourApp />
    </LightningProvider>
  );
}
```

**2. Connect Button**
```tsx
import { useConnection } from '@etrid/solana-lightning-adapter';

function ConnectButton() {
  const { connected, connect, disconnect } = useConnection();

  return (
    <button onClick={connected ? disconnect : connect}>
      {connected ? 'Disconnect' : 'Connect Lightning'}
    </button>
  );
}
```

**3. Send Payment Hook**
```tsx
import { useSendPayment } from '@etrid/solana-lightning-adapter';

function PayInvoice() {
  const { sendPayment, loading, error } = useSendPayment();

  const handlePay = async (invoice: string) => {
    const result = await sendPayment(invoice);
    if (result.success) {
      console.log('Payment sent!', result.preimage);
    }
  };

  return (/* UI */);
}
```

**4. Receive Payment Hook**
```tsx
import { useReceivePayment } from '@etrid/solana-lightning-adapter';

function ReceivePayment() {
  const { createInvoice, invoice, qrCode } = useReceivePayment();

  const handleCreate = async () => {
    await createInvoice({
      amount: 10000, // lamports
      description: 'Payment for services',
      expiresIn: 3600 // 1 hour
    });
  };

  return (/* Display QR code and invoice */);
}
```

**5. Channel Management Hook**
```tsx
import { useChannels } from '@etrid/solana-lightning-adapter';

function Channels() {
  const { channels, openChannel, closeChannel } = useChannels();

  return (
    <div>
      {channels.map(channel => (
        <ChannelCard
          key={channel.id}
          channel={channel}
          onClose={() => closeChannel(channel.id)}
        />
      ))}
    </div>
  );
}
```

#### Integration Examples

**E-commerce Checkout:**
```tsx
import { useSendPayment } from '@etrid/solana-lightning-adapter';

function Checkout({ invoice, amount }) {
  const { sendPayment } = useSendPayment();

  const handlePayWithLightning = async () => {
    const result = await sendPayment(invoice);
    if (result.success) {
      // Order confirmed!
      completeOrder(result.paymentHash);
    }
  };

  return (
    <button onClick={handlePayWithLightning}>
      Pay {amount} SOL via Lightning ⚡
    </button>
  );
}
```

**Tipping/Donations:**
```tsx
import { useReceivePayment } from '@etrid/solana-lightning-adapter';

function TipButton({ creator }) {
  const { createInvoice } = useReceivePayment();

  const handleTip = async (amount: number) => {
    const invoice = await createInvoice({
      amount,
      description: `Tip for ${creator}`,
      recipient: creator.address
    });

    // Show QR code modal
    showInvoiceModal(invoice);
  };

  return (/* Tip UI */);
}
```

**Gaming Microtransactions:**
```tsx
import { useSendPayment } from '@etrid/solana-lightning-adapter';

function BuyItem({ item }) {
  const { sendPayment } = useSendPayment();

  const purchase = async () => {
    // Generate invoice from game server
    const invoice = await gameAPI.createInvoice(item.id, item.price);

    // Pay instantly
    const result = await sendPayment(invoice);

    if (result.success) {
      // Grant item immediately
      grantItem(item);
    }
  };

  return (/* Purchase UI */);
}
```

#### Configuration

**Network Configuration:**
```typescript
const config = {
  network: 'mainnet' | 'testnet' | 'devnet',
  rpcEndpoint: 'https://lightning-rpc.etrid.org',
  wsEndpoint: 'wss://lightning-ws.etrid.org',
  solanaRPC: 'https://api.mainnet-beta.solana.com',
  autoConnect: true,
  onChainFallback: true // Fall back to on-chain if Lightning fails
};
```

**Wallet Configuration:**
```typescript
const wallets = [
  new PhantomWalletAdapter(),
  new SolflareWalletAdapter(),
  new BackpackWalletAdapter(),
  new LightningAdapter(config)
];
```

#### Security

- No private key handling (uses existing wallet)
- Invoice validation before payment
- Amount verification
- Routing verification
- Payment proof generation
- HTTPS-only API calls

#### Testing

Include:
- Unit tests (Jest)
- Integration tests
- Example apps for testing
- Mock Lightning Network for testing
- Test coverage > 85%

### Deliverables

1. Complete npm package source code
2. TypeScript declarations (.d.ts)
3. package.json with dependencies
4. README.md with:
   - Installation guide
   - Quick start
   - API reference
   - Integration examples
   - Troubleshooting
5. 3 example apps:
   - Next.js e-commerce
   - React gaming app
   - Vanilla JS integration
6. CHANGELOG.md
7. Contributing guidelines
8. License (MIT)
9. GitHub Actions CI/CD
10. npm publish scripts

Make it easy to integrate (< 5 minutes), well-documented, and production-ready with excellent developer experience.
```

---

## Usage Instructions

### How to Use These Prompts

1. **Choose a prompt** based on what you want to build
2. **Copy the entire prompt** (including context and requirements)
3. **Open a new chat** with Claude, ChatGPT, or another AI assistant
4. **Paste the prompt** and send
5. **Review the generated code**
6. **Copy the code** into your project
7. **Test and iterate** with follow-up prompts if needed

### Tips for Best Results

- Use Claude Sonnet 4 or GPT-4 for best code quality
- Ask for specific files one at a time if output is truncated
- Request tests, documentation, or examples in follow-ups
- Iterate on the design with specific feedback
- Ask the AI to explain complex parts

### Example Follow-Up Prompts

- "Now add dark mode support to the landing page"
- "Add unit tests for the invoice generation"
- "Create a deployment guide for AWS"
- "Add TypeScript strict mode and fix all errors"
- "Optimize the mobile app for low-end Android devices"

### Customization

Feel free to modify these prompts:
- Change the tech stack (Vue instead of React, etc.)
- Add/remove features
- Adjust the design style
- Target different platforms
- Add integration with your existing systems

---

## Next Steps

After generating code with these prompts:

1. **Test thoroughly** in development environment
2. **Review security** implications
3. **Optimize performance** for production
4. **Add monitoring** and analytics
5. **Deploy to staging** first
6. **Gather user feedback**
7. **Iterate and improve**

## Support

If you have issues with the generated code:
- Check the ÉTRID Lightning documentation: https://etrid.org/docs
- Join Discord: https://discord.gg/etrid
- GitHub Issues: https://github.com/etrid/lightning-network

---

**Note:** These prompts generate starting points. Always review, test, and audit code before production deployment.
