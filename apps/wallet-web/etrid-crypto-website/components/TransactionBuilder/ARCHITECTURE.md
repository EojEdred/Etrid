# Transaction Builder Architecture

## Component Hierarchy

```
TransactionBuilder (Main Orchestrator)
│
├─── Header
│    ├─── Title & Description
│    └─── Close Button (optional)
│
├─── Progress Indicator
│    ├─── Step 1: Build
│    └─── Step 2: Review & Submit
│
├─── Step 1: Build (Tab Interface)
│    │
│    ├─── TransferBuilder
│    │    ├─── Chain Selection (dropdown)
│    │    ├─── Recipient Address (input + validation)
│    │    ├─── Amount (input + "Use Max" + validation)
│    │    ├─── Memo (optional textarea)
│    │    └─── Fee Estimation Card
│    │
│    ├─── StakingBuilder
│    │    ├─── Staking Info Display (card)
│    │    ├─── Operation Selection (radio group)
│    │    │    ├─── Stake
│    │    │    ├─── Unstake
│    │    │    └─── Claim Rewards
│    │    ├─── Validator Address (stake only)
│    │    ├─── Amount (stake/unstake only)
│    │    ├─── Fee Estimation Card
│    │    └─── APY Information (stake only)
│    │
│    ├─── GovernanceBuilder
│    │    ├─── Action Tabs
│    │    │    ├─── Vote
│    │    │    │    ├─── Proposal List
│    │    │    │    ├─── Vote Type (Aye/Nay/Abstain)
│    │    │    │    ├─── Vote Amount
│    │    │    │    └─── Conviction Multiplier
│    │    │    ├─── Propose
│    │    │    │    ├─── Title
│    │    │    │    ├─── Description
│    │    │    │    └─── Deposit Amount
│    │    │    └─── Delegate
│    │    │         └─── Delegate Address
│    │    └─── Fee Estimation Card
│    │
│    └─── ChannelBuilder
│         ├─── Operation Selection (radio group)
│         │    ├─── Open Channel
│         │    ├─── Close Channel
│         │    └─── Update Channel
│         ├─── Counterparty Address (open only)
│         ├─── Deposit Amount (open only)
│         ├─── Duration (open only)
│         ├─── Channel Selection (close/update)
│         ├─── Update Amount (update only)
│         └─── Fee Estimation Card
│
├─── Step 2: Review (TransactionReview)
│    │
│    ├─── Status Header (if processing/complete)
│    │    ├─── Status Icon
│    │    ├─── Status Message
│    │    └─── Progress Bar
│    │
│    ├─── Transaction Details Card
│    │    ├─── Transaction Type Icon & Title
│    │    ├─── Chain Information
│    │    ├─── Type-Specific Details
│    │    ├─── Fee Information
│    │    └─── Total (if applicable)
│    │
│    ├─── Transaction Hash Card (if submitted)
│    │    ├─── Hash Display
│    │    ├─── Copy Button
│    │    └─── Block Number
│    │
│    ├─── Warnings/Alerts
│    │
│    └─── Action Buttons
│         ├─── Cancel/Back
│         └─── Sign & Submit / View Explorer
│
└─── Connected Account Info (footer)
     ├─── Account Address
     └─── Balance
```

## Data Flow

```
User Input
    ↓
Form Component (Builder)
    ↓
React Hook Form Validation
    ↓
onComplete Callback
    ↓
TransactionBuilder (Parent)
    ↓
setState(transactionData)
    ↓
TransactionReview Component
    ↓
Polkadot.js API
    ↓
Sign & Send
    ↓
Status Updates
    ↓
onSubmit Callback
    ↓
Transaction Hash
```

## State Management

### Component State (useState)

**TransactionBuilder:**
```typescript
- currentStep: 'build' | 'review'
- transactionType: TransactionType
- transactionData: TransactionData | null
- isSubmitting: boolean
- txHash: string | null
```

**Builder Components:**
```typescript
- estimatedFee: string
- isCalculating: boolean
- error: string | null
- [component-specific state]
```

**TransactionReview:**
```typescript
- status: TransactionStatus
- txHash: string | null
- error: string | null
- blockNumber: number | null
- progress: number
```

### Form State (React Hook Form)

Each builder component manages its own form state:

```typescript
TransferBuilder:
  - chainId, recipient, amount, memo

StakingBuilder:
  - operation, amount, validator, chainId

GovernanceBuilder:
  - action, proposalId, voteType, voteAmount,
    conviction, proposalTitle, proposalDescription,
    proposalDeposit, delegateAddress

ChannelBuilder:
  - operation, channelId, counterparty,
    depositAmount, updateAmount, duration
```

### Global State (useWallet Hook)

```typescript
- selectedAccount: WalletAccount | null
- isConnected: boolean
- selectedChain: ChainId
- setSelectedChain: (chainId: ChainId) => void
```

## Transaction Status State Machine

```
ready
  ↓ (user clicks "Sign & Submit")
signing
  ↓ (user signs in extension)
broadcasting
  ↓ (transaction sent to network)
in-block
  ↓ (included in block)
finalized
  ↓ (block finalized)
[END STATE]

       OR

ready → signing → failed
                     ↓
                  [ERROR STATE]
```

## API Integration Points

### Polkadot.js API

```typescript
// Connection Management
createApi(chainId) → ApiPromise

// Balance Queries
getBalance(chainId, address) → Promise<bigint>
formatBalance(balance, decimals) → string
parseBalance(amount, decimals) → bigint

// Balance Subscriptions
subscribeBalance(chainId, address, callback) → Promise<Unsubscribe>

// Transaction Building
api.tx.balances.transferKeepAlive(recipient, amount)
api.tx.staking.bond(validator, amount, payee)
api.tx.staking.unbond(amount)
api.tx.democracy.vote(proposalId, vote)

// Transaction Signing & Sending
tx.signAndSend(address, { signer }, callback)
```

### Wallet Extension

```typescript
// Extension Management
web3Enable(appName) → Promise<InjectedExtension[]>
web3Accounts() → Promise<InjectedAccountWithMeta[]>
web3FromAddress(address) → Promise<InjectedAccount>

// Signing
injector.signer → Signer
```

## Component Communication

```
Parent → Child:
  Props (initialType, onClose)
    ↓
Child → Parent:
  Callbacks (onComplete, onSubmit, onCancel)
    ↓
Sibling → Sibling:
  Via Parent State (transactionData)
```

## Validation Pipeline

```
User Input
    ↓
React Hook Form Register
    ↓
Field-Level Validators
    ├─── Required Check
    ├─── Format Validation (address, amount)
    ├─── Range Validation (min/max)
    └─── Custom Validators (balance check, etc.)
    ↓
Form State Updated
    ↓
Error Messages Displayed
    ↓
isValid Flag Updated
    ↓
Submit Button Enabled/Disabled
```

## Fee Estimation Flow

```
User Input Change
    ↓
Debounce Timer (500ms)
    ↓
setIsCalculating(true)
    ↓
Calculate Fee Based on:
    ├─── Transaction Type
    ├─── Chain
    ├─── Amount
    └─── Operation
    ↓
setEstimatedFee(fee)
    ↓
setIsCalculating(false)
    ↓
Display Updated Fee
```

## Error Handling Strategy

### Levels of Error Handling

1. **Form Validation Errors**
   - Caught at input level
   - Displayed inline
   - Prevent form submission

2. **API Connection Errors**
   - Caught in useEffect
   - Displayed in alerts
   - Allow retry

3. **Transaction Errors**
   - Caught in signAndSend callback
   - Displayed in status messages
   - Provide error details

4. **Extension Errors**
   - Caught at signing stage
   - User-friendly messages
   - Guide user to fix

### Error Flow

```
Error Occurs
    ↓
try/catch Block
    ↓
Extract Error Message
    ↓
Set Error State
    ↓
Display Error UI
    ├─── Alert Component
    ├─── Inline Message
    └─── Toast Notification
    ↓
Update Component State
    ├─── setStatus('failed')
    ├─── setError(message)
    └─── setIsLoading(false)
```

## Performance Optimizations

### 1. Debouncing
```typescript
useEffect(() => {
  const timer = setTimeout(calculateFee, 500);
  return () => clearTimeout(timer);
}, [dependencies]);
```

### 2. Memoization
```typescript
const expensiveValue = useMemo(() => {
  return calculateExpensiveValue(input);
}, [input]);
```

### 3. Lazy Loading
```typescript
// Load extension only when needed
if (typeof window !== 'undefined') {
  import('@polkadot/extension-dapp').then(module => {
    // Use module
  });
}
```

### 4. Connection Pooling
```typescript
// Cache API instances
const apiInstances: Map<ChainId, ApiPromise> = new Map();
```

### 5. Cleanup
```typescript
useEffect(() => {
  const subscription = subscribe();
  return () => subscription.unsubscribe();
}, []);
```

## Accessibility Architecture

### Semantic Structure
```html
<form role="form">
  <label for="input">Label</label>
  <input
    id="input"
    aria-label="Descriptive label"
    aria-invalid={hasError}
    aria-describedby="error-message"
  />
  <p id="error-message" role="alert">Error</p>
</form>
```

### Focus Management
- Tab order follows visual order
- Focus visible on all interactive elements
- Modal traps focus
- Escape key closes dialogs

### Screen Reader Support
- ARIA labels on all inputs
- Error messages announced with role="alert"
- Status updates announced
- Progress communicated

## Security Architecture

### Input Sanitization
```typescript
// Address validation
const isValidAddress = (addr: string) => {
  return addr.length >= 47 && addr.length <= 48;
};

// Amount validation
const isValidAmount = (amount: string) => {
  const num = parseFloat(amount);
  return !isNaN(num) && num > 0;
};
```

### Private Key Safety
- Never request private keys
- All signing via extension
- No private data stored
- Extension provides isolation

### Transaction Safety
- Review before signing
- Clear transaction details
- Warnings for large amounts
- Cancellable before signing

## Styling Architecture

### TailwindCSS Structure
```
Base Styles (global)
    ↓
Component Styles (components/ui)
    ↓
Utility Classes (inline)
    ↓
Dark Mode Variants (dark:)
    ↓
Responsive Variants (md:, lg:)
```

### Design Tokens
```typescript
colors: {
  primary: '#FF6B35',
  success: '#10b981',
  error: '#ef4444',
  warning: '#f59e0b',
}

spacing: {
  base: '4px',  // 1 unit
}

borderRadius: {
  default: '0.5rem',
}
```

## File Size Analysis

```
TransactionReview.tsx     20 KB  (largest - handles all TX types)
ChannelBuilder.tsx        18 KB  (complex channel operations)
GovernanceBuilder.tsx     18 KB  (proposal listing + voting)
StakingBuilder.tsx        14 KB  (3 operations)
TransferBuilder.tsx       10 KB  (simplest form)
TransactionBuilder.tsx     8 KB  (orchestrator)
examples.tsx               7 KB  (8 examples)
index.ts                   0.5 KB (exports)
```

## Extension Points

Areas designed for future extension:

1. **New Transaction Types**
   - Add new builder component
   - Add to tab list
   - Implement form logic
   - Add to review component

2. **Custom Validators**
   - Add to validation pipeline
   - Extend form schema
   - Add error messages

3. **Additional Chains**
   - Update CHAINS config
   - No component changes needed

4. **Custom Fee Logic**
   - Update fee calculation
   - Can be chain-specific

5. **Transaction Templates**
   - Save form state
   - Restore from saved state

## Dependencies Graph

```
TransactionBuilder
├── TransferBuilder
│   ├── useWallet
│   ├── react-hook-form
│   └── UI components
├── StakingBuilder
│   ├── useWallet
│   ├── react-hook-form
│   └── UI components
├── GovernanceBuilder
│   ├── useWallet
│   ├── react-hook-form
│   └── UI components
├── ChannelBuilder
│   ├── useWallet
│   ├── react-hook-form
│   └── UI components
└── TransactionReview
    ├── useWallet
    ├── useToast
    ├── @polkadot/api
    ├── @polkadot/extension-dapp
    └── UI components
```

## Build & Bundle Optimization

### Code Splitting
- Dynamic imports for extension
- Lazy load heavy components
- Route-based splitting (Next.js automatic)

### Tree Shaking
- Named exports (not default)
- ES6 modules
- No side effects in imports

### Bundle Analysis
```bash
# Analyze bundle
npm run build && npm run analyze
```

---

This architecture supports:
- ✅ Scalability (easy to add features)
- ✅ Maintainability (clear structure)
- ✅ Testability (separated concerns)
- ✅ Performance (optimized rendering)
- ✅ Accessibility (WCAG compliant)
- ✅ Security (safe by default)
