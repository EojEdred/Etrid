# Validator Onboarding - Quick Reference

## File Locations

```
/app/validator/
├── page.tsx                        # Main route (28 lines)
└── ValidatorOnboardingContent.tsx  # Content component (746 lines)

/lib/polkadot/
└── staking.ts                      # +5 validator functions (175 lines)

/hooks/
└── useStaking.ts                   # +5 React hooks (110 lines)
```

## New Backend Functions

### In `/lib/polkadot/staking.ts`

```typescript
// Query minimum validator bond from chain
getMinimumValidatorBond(): Promise<string>

// Check if address is active validator
isValidator(address: string): Promise<boolean>

// Get validator statistics
getValidatorCounts(): Promise<{current, max, slotsAvailable}>

// Set session keys (transaction)
setKeys(address, keys, proof?): Promise<{txHash, blockHash}>

// Declare intention to validate (transaction)
validate(address, commission): Promise<{txHash, blockHash}>
```

## New React Hooks

### In `/hooks/useStaking.ts`

```typescript
// Query hooks (automatic refetch)
useMinimumValidatorBond()  // Refetch: 5 min
useIsValidator(address)    // Refetch: 30 sec
useValidatorCounts()       // Refetch: 30 sec

// Mutation hooks (transactions)
useSetKeys()               // Set session keys on-chain
useValidate()              // Declare validation
```

## Component Structure

### ValidatorOnboardingContent.tsx

**State:**
- Wallet connection (address, isConnecting)
- Form inputs (bondAmount, sessionKeys, commission)
- Current step tracker (0-4)

**Data Fetching:**
- minValidatorBond (from chain)
- validatorStatus (is user a validator?)
- validatorCounts (network stats)
- eraInfo (current era/session)

**Mutations:**
- bondMutation (bond tokens)
- setKeysMutation (set session keys)
- validateMutation (declare validation)

**Sections:**
1. Header with wallet connection
2. Network status cards
3. Validator status badge
4. Tabs: Requirements / Setup Guide

## RPC Configuration

**Primary:** wss://rpc.etrid.org  
**Fallback:** ws://157.173.200.80:9944

Auto-failover with 2.5s timeout per endpoint.

## Transaction Flow

```
1. User clicks "Connect Wallet"
   └─> enableExtension()
       └─> setAddress()

2. User enters bond amount
   └─> bondMutation.mutate()
       └─> api.tx.staking.bond()
           └─> Wait for finalization
               └─> Show toast + advance step

3. User pastes session keys
   └─> setKeysMutation.mutate()
       └─> api.tx.session.setKeys()
           └─> Wait for finalization
               └─> Show toast + advance step

4. User sets commission
   └─> validateMutation.mutate()
       └─> api.tx.staking.validate()
           └─> Wait for finalization
               └─> Show success state
                   └─> Invalidate queries
```

## Query Invalidation

After transactions, these queries are invalidated:
- `stakingInfo` - User's staking position
- `isValidator` - Validator status check
- `validators` - Validator list

## Error Handling

All errors are decoded and shown as toast notifications:
- Wallet connection errors
- RPC failures
- Transaction failures (with module errors)
- Form validation errors

## Styling

Uses existing ETRID design system:
- `gradient-bg-animated` - Animated background
- `glass-card` - Glass morphism cards
- `gradient-text` - Cyan/purple text gradient
- Lucide icons throughout
- Shadcn/ui components

## Testing Checklist

- [ ] Page loads at /validator
- [ ] Wallet connection works
- [ ] Network stats display correctly
- [ ] Requirements show properly
- [ ] Bond transaction executes
- [ ] Session keys transaction executes
- [ ] Validate transaction executes
- [ ] Success state shows after completion
- [ ] Error handling works
- [ ] Copy-to-clipboard functions
- [ ] External links work
- [ ] Responsive on mobile

## Production Deployment

1. Verify RPC endpoints are accessible
2. Test full flow on testnet first
3. Check Polkadot.js extension compatibility
4. Monitor transaction success rates
5. Set up error tracking

## API Calls Summary

**Queries:**
- `api.consts.staking.minValidatorBond`
- `api.query.session.validators()`
- `api.consts.staking.maxValidatorCount`
- `api.query.staking.activeEra()`
- `api.query.staking.currentEra()`

**Transactions:**
- `api.tx.staking.bond(controller, value, payee)`
- `api.tx.session.setKeys(keys, proof)`
- `api.tx.staking.validate({commission, blocked})`

## Development

```bash
# Run dev server
npm run dev

# Build for production
npm run build

# Access page
http://localhost:3000/validator
```

## Support Links

- GitHub: https://github.com/ArsCodeAmatworker/etrid/releases
- Explorer: https://explorer.etrid.org
- Docs: (add your docs link)
