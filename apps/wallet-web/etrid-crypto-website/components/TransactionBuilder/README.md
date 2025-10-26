# Transaction Builder Components

A comprehensive suite of React components for building, reviewing, and submitting transactions on the Etrid Protocol.

## Overview

The Transaction Builder provides a user-friendly, step-by-step wizard interface for creating various types of blockchain transactions including transfers, staking operations, governance actions, and payment channel management.

## Components

### 1. TransactionBuilder (Main Component)

The primary orchestrator component that manages the transaction building workflow.

**Location:** `components/TransactionBuilder/TransactionBuilder.tsx`

**Features:**
- Multi-step wizard interface (Build -> Review)
- Tab-based transaction type selection
- Progress indicator
- Connected wallet status display
- Responsive design

**Props:**
```typescript
interface TransactionBuilderProps {
  onClose?: () => void;
  initialType?: TransactionType; // 'transfer' | 'staking' | 'governance' | 'channel'
}
```

**Usage:**
```tsx
import { TransactionBuilder } from '@/components/TransactionBuilder/TransactionBuilder';

function WalletPage() {
  return (
    <TransactionBuilder
      initialType="transfer"
      onClose={() => console.log('Closed')}
    />
  );
}
```

---

### 2. TransferBuilder

Form component for creating token transfer transactions.

**Location:** `components/TransactionBuilder/TransferBuilder.tsx`

**Features:**
- Multi-chain support (FlareChain + 12 PBCs)
- Real-time balance checking
- Recipient address validation
- Fee estimation
- "Use Max" button for maximum amount
- Optional memo field
- Substrate address format validation

**Form Fields:**
- Chain selection (dropdown)
- Recipient address (validated input)
- Amount (with balance checking)
- Memo (optional, max 256 chars)

**Validation:**
- Address format (47-48 characters for Substrate)
- Amount > 0 and <= available balance
- Sufficient balance for amount + fees

**Usage:**
```tsx
import { TransferBuilder } from '@/components/TransactionBuilder/TransferBuilder';

<TransferBuilder
  onComplete={(data) => {
    console.log('Transfer data:', data);
    // Proceed to review step
  }}
/>
```

---

### 3. StakingBuilder

Form component for staking-related operations.

**Location:** `components/TransactionBuilder/StakingBuilder.tsx`

**Features:**
- Three operation types: Stake, Unstake, Claim Rewards
- Real-time staking info display
- Validator selection for staking
- APY estimation
- Unbonding period warnings
- Minimum stake validation

**Operations:**

1. **Stake**
   - Validator address selection
   - Amount input with minimum validation (1 ETR)
   - APY and yearly rewards estimation
   - Unbonding period notice (28 days)

2. **Unstake**
   - Amount validation against staked balance
   - Unbonding period warning
   - Maximum unstake helper

3. **Claim Rewards**
   - Display pending rewards
   - One-click claiming
   - Minimal transaction fee

**Form Fields:**
- Operation type (radio group)
- Validator address (for staking only)
- Amount (not required for claim)

**Usage:**
```tsx
import { StakingBuilder } from '@/components/TransactionBuilder/StakingBuilder';

<StakingBuilder
  onComplete={(data) => {
    console.log('Staking operation:', data);
  }}
/>
```

---

### 4. GovernanceBuilder

Form component for governance participation.

**Location:** `components/TransactionBuilder/GovernanceBuilder.tsx`

**Features:**
- Three action types: Vote, Propose, Delegate
- Live proposal listing
- Vote conviction system (1x to 6x)
- Proposal deposit management
- Vote tracking (Aye/Nay/Abstain)

**Actions:**

1. **Vote**
   - Browse active proposals
   - Vote type selection (Aye/Nay/Abstain)
   - Vote amount input
   - Conviction multiplier (locks tokens for longer, increases voting power)
   - Proposal details display

2. **Propose**
   - Proposal title and description
   - Minimum deposit requirement (10 ETR)
   - Rich proposal editor
   - Deposit refund information

3. **Delegate**
   - Delegate address input
   - Voting power delegation
   - Delegation management

**Form Fields:**
- Action type (tabs)
- Proposal selection (for voting)
- Vote type and amount (for voting)
- Conviction multiplier (for voting)
- Proposal details (for proposing)
- Delegate address (for delegation)

**Usage:**
```tsx
import { GovernanceBuilder } from '@/components/TransactionBuilder/GovernanceBuilder';

<GovernanceBuilder
  onComplete={(data) => {
    console.log('Governance action:', data);
  }}
/>
```

---

### 5. ChannelBuilder

Form component for payment channel operations.

**Location:** `components/TransactionBuilder/ChannelBuilder.tsx`

**Features:**
- Three operations: Open, Close, Update
- Active channel listing
- Instant, low-fee transactions
- Channel duration management
- Balance tracking

**Operations:**

1. **Open Channel**
   - Counterparty address input
   - Initial deposit amount
   - Channel duration (in blocks)
   - Block time conversion helper

2. **Close Channel**
   - Select from active channels
   - Balance return confirmation
   - Irreversible action warning

3. **Update Channel**
   - Select from active channels
   - Update amount input
   - Minimal fee notification

**Form Fields:**
- Operation type (radio group)
- Counterparty address (for opening)
- Deposit/update amount
- Duration in blocks (for opening)
- Channel selection (for close/update)

**Usage:**
```tsx
import { ChannelBuilder } from '@/components/TransactionBuilder/ChannelBuilder';

<ChannelBuilder
  onComplete={(data) => {
    console.log('Channel operation:', data);
  }}
/>
```

---

### 6. TransactionReview

Component for reviewing and submitting transactions.

**Location:** `components/TransactionBuilder/TransactionReview.tsx`

**Features:**
- Comprehensive transaction summary
- Real-time status tracking
- Progress indicator
- Transaction signing via Polkadot.js extension
- Block explorer integration
- Transaction hash display and copy
- Error handling with detailed messages

**Transaction Statuses:**
- `ready` - Ready to submit
- `signing` - Awaiting signature from wallet
- `broadcasting` - Broadcasting to network
- `in-block` - Included in block
- `finalized` - Transaction finalized
- `failed` - Transaction failed

**Features by Status:**
- **Ready:** Display all transaction details, warnings for high-value transactions
- **Signing:** Show signing prompt, progress bar at 20%
- **Broadcasting:** Network broadcast indicator, progress bar at 40%
- **In-block:** Block number display, progress bar at 70%
- **Finalized:** Success message, transaction hash, explorer link, progress bar at 100%
- **Failed:** Error message with details, retry option

**Props:**
```typescript
interface TransactionReviewProps {
  transaction: TransactionData;
  onSubmit: (txHash: { hash: string }) => void;
  onCancel: () => void;
  isSubmitting?: boolean;
  txHash?: string | null;
}
```

**Usage:**
```tsx
import { TransactionReview } from '@/components/TransactionBuilder/TransactionReview';

<TransactionReview
  transaction={transactionData}
  onSubmit={(txHash) => {
    console.log('Transaction submitted:', txHash);
  }}
  onCancel={() => {
    console.log('Review cancelled');
  }}
/>
```

---

## Integration Guide

### Prerequisites

The Transaction Builder requires:
- React 18+
- TypeScript
- TailwindCSS
- Polkadot.js API (`@polkadot/api`, `@polkadot/extension-dapp`)
- React Hook Form
- Existing wallet connection (via `useWallet` hook)

### Installation

All components are located in `components/TransactionBuilder/`. Ensure the following dependencies are installed:

```bash
npm install @polkadot/api @polkadot/extension-dapp @polkadot/util @polkadot/util-crypto
npm install react-hook-form @hookform/resolvers zod
```

### Basic Setup

1. **Import the main component:**

```tsx
import { TransactionBuilder } from '@/components/TransactionBuilder/TransactionBuilder';
```

2. **Use in your app:**

```tsx
function App() {
  return (
    <div className="container mx-auto py-8">
      <TransactionBuilder />
    </div>
  );
}
```

3. **With custom configuration:**

```tsx
function WalletDashboard() {
  const [isOpen, setIsOpen] = useState(false);

  return (
    <>
      <Button onClick={() => setIsOpen(true)}>
        New Transaction
      </Button>

      {isOpen && (
        <TransactionBuilder
          initialType="transfer"
          onClose={() => setIsOpen(false)}
        />
      )}
    </>
  );
}
```

---

## State Management

The Transaction Builder uses React Hook Form for form state management and the existing `useWallet` hook for blockchain interactions.

### useWallet Hook

Required hook that provides:
```typescript
interface UseWalletReturn {
  selectedAccount: WalletAccount | null;
  isConnected: boolean;
  selectedChain: ChainId;
  setSelectedChain: (chainId: ChainId) => void;
  sendTransaction: (to: string, amount: string) => Promise<string>;
}
```

### Form Data Flow

1. User fills form in builder component
2. Form validation occurs in real-time
3. On submit, data passed to parent via `onComplete` callback
4. Parent transitions to review step
5. Review component signs and submits transaction
6. Transaction status updates in real-time
7. Final hash returned to parent via `onSubmit` callback

---

## Styling

All components use TailwindCSS with the following design principles:

- **Colors:** Primary brand color with semantic variants (success, warning, destructive)
- **Typography:** Clear hierarchy with proper font weights
- **Spacing:** Consistent spacing scale (4px base unit)
- **Responsiveness:** Mobile-first design, responsive breakpoints
- **Dark Mode:** Full dark mode support via TailwindCSS dark variant
- **Accessibility:** ARIA labels, keyboard navigation, screen reader support

### Customization

To customize styles, modify the TailwindCSS theme in `tailwind.config.js`:

```js
module.exports = {
  theme: {
    extend: {
      colors: {
        primary: {
          DEFAULT: '#FF6B35',
          foreground: '#FFFFFF',
        },
      },
    },
  },
};
```

---

## Error Handling

The Transaction Builder implements comprehensive error handling:

### Form Validation Errors
- Display inline below input fields
- Real-time validation feedback
- Clear error messages

### Transaction Errors
- Network connectivity issues
- Insufficient balance
- Invalid addresses
- Gas estimation failures
- Signature rejection
- Transaction reversion

### Error Display
```tsx
// Inline form error
{errors.amount && (
  <p className="text-sm text-destructive">{errors.amount.message}</p>
)}

// Alert-style error
<Alert variant="destructive">
  <AlertCircle className="h-4 w-4" />
  <AlertDescription>{error}</AlertDescription>
</Alert>
```

---

## Testing

### Manual Testing Checklist

**TransferBuilder:**
- [ ] Valid address accepts input
- [ ] Invalid address shows error
- [ ] Amount validation (positive, within balance)
- [ ] Fee estimation updates
- [ ] "Use Max" button works correctly
- [ ] Chain switching updates balance
- [ ] Memo field accepts input

**StakingBuilder:**
- [ ] All three operations display correctly
- [ ] Validator address validation (stake only)
- [ ] Minimum stake amount enforced
- [ ] Unstake maximum is current stake
- [ ] Claim shows pending rewards
- [ ] APY calculation displays

**GovernanceBuilder:**
- [ ] Proposals load and display
- [ ] Vote types selectable (Aye/Nay/Abstain)
- [ ] Conviction multiplier works
- [ ] Proposal creation validates inputs
- [ ] Minimum deposit enforced
- [ ] Delegate address validation

**ChannelBuilder:**
- [ ] Operations switch correctly
- [ ] Active channels display
- [ ] Open channel validates all inputs
- [ ] Duration converts to hours correctly
- [ ] Close shows warning message
- [ ] Update amount validation

**TransactionReview:**
- [ ] All transaction types display correctly
- [ ] Status updates in real-time
- [ ] Progress bar advances
- [ ] Transaction hash displays
- [ ] Copy function works
- [ ] Explorer link is correct
- [ ] Error states display properly

---

## Performance Considerations

### Optimization Techniques

1. **Form Debouncing:** Fee estimation debounced by 500ms to reduce API calls
2. **Memoization:** Expensive calculations memoized
3. **Lazy Loading:** Polkadot.js extension loaded on-demand
4. **API Connection Pooling:** Reuses existing API connections
5. **Balance Subscription:** Efficient WebSocket subscriptions for real-time updates

### Best Practices

- Use `useMemo` for expensive calculations
- Implement proper cleanup in `useEffect`
- Avoid unnecessary re-renders with `React.memo`
- Use skeleton loaders for async data
- Implement proper error boundaries

---

## Accessibility

All components follow WCAG 2.1 Level AA guidelines:

- **Keyboard Navigation:** Full keyboard support
- **Screen Readers:** Proper ARIA labels and descriptions
- **Focus Management:** Clear focus indicators
- **Color Contrast:** Meets minimum contrast ratios
- **Form Labels:** All inputs have associated labels
- **Error Announcements:** Error messages announced to screen readers

### ARIA Attributes Used

```tsx
// Form inputs
<Input
  id="amount"
  aria-label="Transaction amount"
  aria-invalid={!!errors.amount}
  aria-describedby={errors.amount ? "amount-error" : undefined}
/>

// Error messages
<p id="amount-error" role="alert" className="text-destructive">
  {errors.amount.message}
</p>
```

---

## API Reference

### Component Props

#### TransactionBuilder
```typescript
interface TransactionBuilderProps {
  onClose?: () => void;
  initialType?: 'transfer' | 'staking' | 'governance' | 'channel';
}
```

#### Builder Components (Transfer, Staking, Governance, Channel)
```typescript
interface BuilderProps {
  onComplete: (data: FormData & { estimatedFee: string }) => void;
}
```

#### TransactionReview
```typescript
interface TransactionReviewProps {
  transaction: TransactionData;
  onSubmit: (txHash: { hash: string }) => void;
  onCancel: () => void;
  isSubmitting?: boolean;
  txHash?: string | null;
}
```

### Types

```typescript
type TransactionType = 'transfer' | 'staking' | 'governance' | 'channel';
type ChainId = 'flarechain' | 'btc-pbc' | 'eth-pbc' | /* ... other PBCs */;
type TransactionStatus = 'ready' | 'signing' | 'broadcasting' | 'in-block' | 'finalized' | 'failed';

interface TransactionData {
  type: TransactionType;
  chainId: ChainId;
  data: any; // Type-specific data
}
```

---

## Troubleshooting

### Common Issues

**Issue:** "No Polkadot.js extension found"
- **Solution:** Install Polkadot.js browser extension and refresh page

**Issue:** Fee estimation shows 0
- **Solution:** Ensure API connection is established, check network connectivity

**Issue:** Transaction stuck at "signing"
- **Solution:** Check wallet extension popup, may be blocked by browser

**Issue:** Transaction fails with "Insufficient balance"
- **Solution:** Verify balance includes amount + fees, reduce amount

**Issue:** Invalid address error
- **Solution:** Ensure using Substrate address format (47-48 chars)

### Debug Mode

Enable debug logging:
```typescript
// In TransactionReview.tsx
console.log('[TransactionReview] Status:', status);
console.log('[TransactionReview] Transaction:', transaction);
```

---

## Changelog

### Version 1.0.0 (2025-10-22)
- Initial release
- Transfer, Staking, Governance, and Channel builders
- Real-time fee estimation
- Multi-chain support (FlareChain + 12 PBCs)
- Transaction status tracking
- Full TypeScript support
- Comprehensive error handling
- Dark mode support
- Accessibility compliance (WCAG 2.1 AA)

---

## Contributing

When adding new features:

1. Follow existing component patterns
2. Maintain TypeScript type safety
3. Add proper error handling
4. Include accessibility attributes
5. Update this README
6. Test across different chains
7. Verify dark mode compatibility

---

## License

Part of the Etrid Protocol wallet application.

---

## Support

For issues and questions:
- GitHub Issues: [project repository]
- Documentation: [docs link]
- Discord: [community link]
