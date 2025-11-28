# Transaction Builder - Component Architecture

## Visual Component Hierarchy

```
TransactionBuilder (Main Orchestrator)
│
├── Step 1: Build Transaction
│   │
│   ├── ChainSelector ⭐ NEW
│   │   ├── Network Detection
│   │   ├── Status Monitoring
│   │   └── Chain Information Display
│   │
│   ├── TokenSelector ⭐ NEW
│   │   ├── Native Tokens
│   │   ├── ERC-20 Tokens
│   │   ├── Custom Token Addition
│   │   └── Token Search & Filtering
│   │
│   ├── Transaction Type Tabs
│   │   ├── TransferBuilder
│   │   │   ├── Recipient Input
│   │   │   ├── Amount Input
│   │   │   └── Memo Input
│   │   │
│   │   ├── BatchBuilder
│   │   │   ├── Multiple Recipients
│   │   │   ├── Individual Amounts
│   │   │   └── Total Calculation
│   │   │
│   │   ├── StakingBuilder
│   │   │   ├── Operation Selection
│   │   │   ├── Validator Selection
│   │   │   └── Amount Input
│   │   │
│   │   ├── GovernanceBuilder
│   │   │   ├── Action Selection
│   │   │   ├── Proposal Selection
│   │   │   └── Vote Configuration
│   │   │
│   │   └── ChannelBuilder
│   │       ├── Operation Selection
│   │       ├── Counterparty Input
│   │       └── Deposit/Duration
│   │
│   └── FeeEstimator
│       ├── Priority Selection
│       ├── Network Congestion
│       ├── Fee Breakdown
│       └── Optimization Tips
│
├── Step 2: Preview Transaction ⭐ NEW
│   │
│   └── TransactionPreview
│       ├── Transaction Details
│       ├── Warning System
│       ├── Fee & Time Estimation
│       ├── Total Cost Display
│       └── Safety Checklist
│
├── Step 3: Simulate (Optional)
│   │
│   └── TransactionSimulator
│       ├── Balance Changes
│       ├── State Changes
│       ├── Event Preview
│       └── Warning Detection
│
├── Step 4: Review & Sign
│   │
│   └── TransactionReview
│       ├── Transaction Signing
│       ├── Status Tracking
│       ├── Progress Indicator
│       └── Block Explorer Link
│
└── Auxiliary Features
    │
    ├── TransactionHistory
    │   ├── Transaction List
    │   ├── Filtering & Search
    │   ├── Status Tracking
    │   └── Transaction Reuse
    │
    └── TransactionExport ⭐ NEW
        ├── Format Selection (JSON/CSV)
        ├── Preview Display
        ├── Download Functionality
        └── Export Statistics
```

---

## Component Relationships

### Data Flow

```
User Input
    ↓
ChainSelector → Selected Chain ID
    ↓
TokenSelector → Selected Token
    ↓
Transaction Builder → Transaction Data
    ↓
FeeEstimator → Fee Calculation
    ↓
TransactionPreview → User Confirmation
    ↓
TransactionSimulator → Simulation Results
    ↓
TransactionReview → Signed Transaction
    ↓
Blockchain Submission
    ↓
TransactionHistory → Historical Record
    ↓
TransactionExport → Data Export
```

---

## Component Integration Map

### Core Flow (Required Components)

```
┌─────────────────────────────────────────────────────────────┐
│                    TransactionBuilder                        │
│  ┌──────────────────────────────────────────────────────┐  │
│  │  STEP 1: BUILD                                        │  │
│  │  ┌────────────────┐  ┌─────────────────┐             │  │
│  │  │ ChainSelector  │→ │ TokenSelector   │             │  │
│  │  │     ⭐ NEW     │  │     ⭐ NEW      │             │  │
│  │  └────────────────┘  └─────────────────┘             │  │
│  │           ↓                   ↓                        │  │
│  │  ┌──────────────────────────────────────┐             │  │
│  │  │    Transaction Type Builder          │             │  │
│  │  │  (Transfer/Batch/Staking/Gov/Channel)│             │  │
│  │  └──────────────────────────────────────┘             │  │
│  │           ↓                                            │  │
│  │  ┌──────────────────┐                                 │  │
│  │  │  FeeEstimator    │ (priority-based fees)           │  │
│  │  └──────────────────┘                                 │  │
│  └──────────────────────────────────────────────────────┘  │
│                          ↓                                  │
│  ┌──────────────────────────────────────────────────────┐  │
│  │  STEP 2: PREVIEW ⭐ NEW                              │  │
│  │  ┌────────────────────────────────────┐               │  │
│  │  │   TransactionPreview               │               │  │
│  │  │   - Details Display                │               │  │
│  │  │   - Warning System                 │               │  │
│  │  │   - Safety Checklist               │               │  │
│  │  └────────────────────────────────────┘               │  │
│  └──────────────────────────────────────────────────────┘  │
│                          ↓                                  │
│  ┌──────────────────────────────────────────────────────┐  │
│  │  STEP 3: SIMULATE (Optional)                          │  │
│  │  ┌────────────────────────────────────┐               │  │
│  │  │   TransactionSimulator             │               │  │
│  │  └────────────────────────────────────┘               │  │
│  └──────────────────────────────────────────────────────┘  │
│                          ↓                                  │
│  ┌──────────────────────────────────────────────────────┐  │
│  │  STEP 4: SIGN                                         │  │
│  │  ┌────────────────────────────────────┐               │  │
│  │  │   TransactionReview                │               │  │
│  │  └────────────────────────────────────┘               │  │
│  └──────────────────────────────────────────────────────┘  │
└─────────────────────────────────────────────────────────────┘
```

### Auxiliary Components

```
┌─────────────────────────────────┐  ┌─────────────────────────────────┐
│    TransactionHistory           │  │   TransactionExport ⭐ NEW     │
│  ┌───────────────────────────┐  │  │  ┌───────────────────────────┐ │
│  │ - List Display            │  │  │  │ - JSON Format             │ │
│  │ - Filtering & Search      │  │  │  │ - CSV Format              │ │
│  │ - Status Tracking         │  │  │  │ - Preview Display         │ │
│  │ - Reuse Functionality     │◄─┼──┼─►│ - Download/Copy           │ │
│  └───────────────────────────┘  │  │  └───────────────────────────┘ │
└─────────────────────────────────┘  └─────────────────────────────────┘
```

---

## New Components Deep Dive

### 1. ChainSelector

**Purpose**: Multi-chain network selection with status monitoring

**Key Features**:
- 13 chain support (1 relay + 12 PBCs)
- Network status detection
- Real-time monitoring
- Visual indicators
- Balance display

**Integration Points**:
- Used in all transaction builders
- Updates TokenSelector on change
- Updates FeeEstimator on change
- Triggers balance refresh

**Props**:
```typescript
{
  value: ChainId;
  onChange: (chainId: ChainId) => void;
  showNetworkStatus?: boolean;
  showBalance?: boolean;
  filterRelay?: boolean;
  filterParachains?: boolean;
}
```

---

### 2. TokenSelector

**Purpose**: Token selection for native and ERC-20 tokens

**Key Features**:
- Native token support
- ERC-20 token support
- Custom token addition
- Token search
- Balance tracking
- Price display

**Integration Points**:
- Depends on ChainSelector
- Updates amount calculation
- Updates fee estimation
- Provides token metadata

**Props**:
```typescript
{
  chainId: ChainId;
  value: Token | null;
  onChange: (token: Token) => void;
  showBalance?: boolean;
  allowCustomTokens?: boolean;
}
```

---

### 3. TransactionPreview

**Purpose**: Enhanced preview before transaction signing

**Key Features**:
- Type-specific rendering
- Warning system
- Fee/time estimation
- Total cost display
- Safety checklist

**Integration Points**:
- Receives transaction data
- Shows before signing
- Triggers simulation (optional)
- Proceeds to signing

**Props**:
```typescript
{
  transaction: TransactionData;
  estimatedFee?: string;
  estimatedTime?: string;
  onConfirm: () => void;
  onCancel: () => void;
  isLoading?: boolean;
}
```

---

### 4. TransactionExport

**Purpose**: Export transaction data to file formats

**Key Features**:
- JSON/CSV formats
- Preview display
- Download/copy
- Statistics

**Integration Points**:
- Used with TransactionHistory
- Receives transaction array
- Standalone or modal usage

**Props**:
```typescript
{
  transactions: ExportTransaction[];
  onClose?: () => void;
}
```

---

## State Management

### Global State (via useWallet hook)

```typescript
{
  selectedAccount: WalletAccount | null;
  isConnected: boolean;
  selectedChain: ChainId;
  setSelectedChain: (chainId: ChainId) => void;
  balance: string;
}
```

### Component-Level State

**TransactionBuilder**:
- currentStep: 'build' | 'review'
- transactionType: TransactionType
- transactionData: TransactionData | null
- isSubmitting: boolean
- txHash: string | null

**ChainSelector**:
- networkStatus: Record<string, 'connected' | 'disconnected' | 'checking'>

**TokenSelector**:
- tokens: Token[]
- searchQuery: string
- isAddingCustom: boolean

**TransactionPreview**:
- warnings: string[]

**TransactionExport**:
- exportFormat: 'json' | 'csv'
- copied: boolean

---

## Event Flow

### Transaction Creation Flow

```
1. User opens TransactionBuilder
   ↓
2. ChainSelector mounted → Network status check starts
   ↓
3. User selects chain → TokenSelector loads tokens
   ↓
4. User selects token → Transaction builder updates
   ↓
5. User fills transaction details
   ↓
6. FeeEstimator calculates fees in background
   ↓
7. User clicks "Review" → Move to preview step
   ↓
8. TransactionPreview displays → Warnings generated
   ↓
9. User clicks "Confirm" → Optional simulation
   ↓
10. TransactionSimulator runs (if enabled)
   ↓
11. User proceeds → TransactionReview for signing
   ↓
12. Transaction signed and submitted
   ↓
13. Status tracked until finalization
   ↓
14. Transaction appears in TransactionHistory
   ↓
15. User can export via TransactionExport
```

---

## Error Handling

### Error Boundaries

```
TransactionBuilder (Top Level)
│
├── ChainSelector
│   ├── Network connection error
│   └── Status check timeout
│
├── TokenSelector
│   ├── Token loading error
│   ├── Custom token validation error
│   └── Price fetch error
│
├── Transaction Builders
│   ├── Validation errors
│   ├── Balance check errors
│   └── Fee calculation errors
│
├── TransactionPreview
│   ├── Data parsing errors
│   └── Warning generation errors
│
└── TransactionExport
    ├── Format conversion errors
    ├── Download errors
    └── Clipboard errors
```

---

## Performance Optimization

### Lazy Loading

```typescript
// Heavy components loaded on-demand
const TransactionSimulator = lazy(() => import('./TransactionSimulator'));
const TransactionExport = lazy(() => import('./TransactionExport'));
```

### Debouncing

```typescript
// Fee calculation debounced (500ms)
useEffect(() => {
  const timer = setTimeout(calculateFee, 500);
  return () => clearTimeout(timer);
}, [amount, recipient]);
```

### Memoization

```typescript
// Expensive calculations memoized
const totalCost = useMemo(() => {
  return parseFloat(amount) + parseFloat(fee);
}, [amount, fee]);
```

---

## Accessibility Features

### Keyboard Navigation
- Full tab navigation support
- Enter to submit forms
- Escape to cancel modals
- Arrow keys for selections

### Screen Reader Support
- ARIA labels on all inputs
- Role attributes on interactive elements
- Live region announcements for status changes
- Descriptive button labels

### Focus Management
- Clear focus indicators
- Focus trap in modals
- Auto-focus on important fields
- Focus restoration after modals

---

## Component File Structure

```
components/TransactionBuilder/
│
├── Core Components
│   ├── TransactionBuilder.tsx       (Main orchestrator)
│   ├── TransferBuilder.tsx          (Transfer transactions)
│   ├── StakingBuilder.tsx           (Staking operations)
│   ├── GovernanceBuilder.tsx        (Governance actions)
│   ├── ChannelBuilder.tsx           (Payment channels)
│   ├── BatchBuilder.tsx             (Batch transfers)
│   └── TransactionReview.tsx        (Review & sign)
│
├── Enhanced Features ⭐ NEW
│   ├── ChainSelector.tsx            (Multi-chain selection)
│   ├── TokenSelector.tsx            (Token selection)
│   ├── TransactionPreview.tsx       (Enhanced preview)
│   └── TransactionExport.tsx        (Export functionality)
│
├── Existing Enhanced Features
│   ├── FeeEstimator.tsx             (Fee estimation)
│   ├── TransactionHistory.tsx       (History display)
│   └── TransactionSimulator.tsx     (Transaction simulation)
│
├── Configuration & Types
│   └── index.ts                     (Exports & types)
│
├── Documentation
│   ├── README.md                    (Comprehensive guide)
│   ├── ENHANCEMENTS.md              (Enhancement details)
│   ├── ARCHITECTURE.md              (Architecture docs)
│   ├── SUMMARY.md                   (Quick summary)
│   ├── QUICK_START.md               (Getting started)
│   ├── INDEX.md                     (Documentation index)
│   ├── COMPONENT_ARCHITECTURE.md    (This file)
│   └── TRANSACTION_BUILDER_ENHANCEMENTS_REPORT.md
│
└── Examples
    ├── examples.tsx                 (Usage examples)
    └── INTEGRATION_EXAMPLE.tsx      (Full integration)
```

---

## Summary

### Total Components: 17

**New Components (4)**:
- ⭐ ChainSelector
- ⭐ TokenSelector
- ⭐ TransactionPreview
- ⭐ TransactionExport

**Existing Core (6)**:
- TransactionBuilder
- TransferBuilder
- StakingBuilder
- GovernanceBuilder
- ChannelBuilder
- TransactionReview

**Existing Enhanced (4)**:
- BatchBuilder
- TransactionSimulator
- TransactionHistory
- FeeEstimator

**Utilities (1)**:
- index.ts

**Examples (2)**:
- examples.tsx
- INTEGRATION_EXAMPLE.tsx

---

**Architecture Status**: ✅ COMPLETE
**Integration Status**: ✅ READY
**Documentation Status**: ✅ COMPREHENSIVE
