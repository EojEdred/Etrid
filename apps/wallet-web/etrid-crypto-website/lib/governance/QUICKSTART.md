# ETRID Governance - Quick Start Guide

Get started with ETRID governance in 5 minutes.

## 1. Import in Your Component

```typescript
import {
  useActiveProposals,
  useCastVote,
  useVotingPower,
  VoteType,
  ConvictionLevel
} from '@/lib/governance';
```

## 2. Fetch Active Proposals

```typescript
function MyGovernanceComponent() {
  const { proposals, loading } = useActiveProposals();

  if (loading) return <div>Loading...</div>;

  return (
    <div>
      {proposals.map(proposal => (
        <div key={proposal.id}>
          <h3>{proposal.title}</h3>
          <p>{proposal.category}</p>
        </div>
      ))}
    </div>
  );
}
```

## 3. Vote on a Proposal

```typescript
function VoteButton({ proposalId, account }) {
  const { castVote, voting } = useCastVote();

  const handleVote = async () => {
    // Get signer from Polkadot extension
    const { web3FromAddress } = await import('@polkadot/extension-dapp');
    const injector = await web3FromAddress(account);

    // Cast vote with 2x conviction (14 days lock)
    await castVote({
      proposalId,
      voteType: VoteType.Aye,
      conviction: ConvictionLevel.Locked2x,
    }, injector.signer);
  };

  return (
    <button onClick={handleVote} disabled={voting}>
      Vote Yes
    </button>
  );
}
```

## 4. Display Voting Power

```typescript
function VotingPowerCard({ account }) {
  const { votingPower } = useVotingPower(account);

  return (
    <div>
      <p>Staked: {votingPower?.stakedBalance} ETR</p>
      <p>Voting Power: {votingPower?.totalVotingPower}</p>
      <p>Can Vote: {votingPower?.canVote ? 'Yes' : 'No'}</p>
    </div>
  );
}
```

## 5. Enable Polkadot Extension

Before using transaction functions, enable the Polkadot extension:

```typescript
import { web3Enable } from '@polkadot/extension-dapp';

// Call this once when your app loads
await web3Enable('ETRID Wallet');
```

## Complete Example Page

```typescript
'use client';

import {
  useActiveProposals,
  useCastVote,
  useVotingPower,
  VoteType,
  ConvictionLevel
} from '@/lib/governance';
import { useEffect } from 'react';
import { web3Enable, web3FromAddress } from '@polkadot/extension-dapp';

export default function GovernancePage() {
  const account = 'YOUR_ACCOUNT_ADDRESS'; // Get from wallet context

  const { proposals, loading } = useActiveProposals();
  const { votingPower } = useVotingPower(account);
  const { castVote, voting } = useCastVote();

  // Enable extension on mount
  useEffect(() => {
    web3Enable('ETRID Wallet');
  }, []);

  const handleVote = async (proposalId: number) => {
    const injector = await web3FromAddress(account);

    await castVote({
      proposalId,
      voteType: VoteType.Aye,
      conviction: ConvictionLevel.Locked2x,
    }, injector.signer);
  };

  return (
    <div className="p-6">
      <h1 className="text-3xl font-bold mb-6">Governance</h1>

      {/* Voting Power */}
      <div className="mb-6 p-4 bg-white rounded shadow">
        <h2 className="text-xl font-semibold mb-2">Your Voting Power</h2>
        <p>Staked: {votingPower?.stakedBalance || '0'} ETR</p>
        <p>Total Power: {votingPower?.totalVotingPower || '0'}</p>
      </div>

      {/* Proposals */}
      <div>
        <h2 className="text-xl font-semibold mb-4">Active Proposals</h2>
        {loading ? (
          <p>Loading proposals...</p>
        ) : (
          proposals.map(proposal => (
            <div key={proposal.id} className="mb-4 p-4 bg-white rounded shadow">
              <h3 className="font-semibold">{proposal.title}</h3>
              <p className="text-sm text-gray-600 mb-2">{proposal.description}</p>
              <button
                onClick={() => handleVote(proposal.id)}
                disabled={voting}
                className="px-4 py-2 bg-blue-500 text-white rounded hover:bg-blue-600"
              >
                {voting ? 'Voting...' : 'Vote Yes'}
              </button>
            </div>
          ))
        )}
      </div>
    </div>
  );
}
```

## Conviction Levels

Choose your conviction level based on how long you want to lock tokens:

```typescript
ConvictionLevel.None      // 0.1x - No lock
ConvictionLevel.Locked1x  // 1x - 7 days
ConvictionLevel.Locked2x  // 2x - 14 days
ConvictionLevel.Locked3x  // 3x - 28 days
ConvictionLevel.Locked4x  // 4x - 56 days
ConvictionLevel.Locked5x  // 5x - 112 days
ConvictionLevel.Locked6x  // 6x - 224 days
```

Higher conviction = more voting power but longer lock period.

## Available Hooks

Quick reference:

```typescript
// Proposals
useProposals(filters?)          // All proposals
useProposal(id)                 // Single proposal
useActiveProposals()            // Active only
useProposalsByCategory(cat)     // By category

// Voting
useCastVote()                   // Cast vote
useVotes(proposalId)            // Get votes
useVotingStats(proposalId)      // Vote stats
useHasVoted(proposalId, acct)   // Check voted

// Power
useVotingPower(account)         // Get power
useVotingPowerBreakdown(bal, conv) // Calculate

// Delegation
useDelegate()                   // Delegate
useUndelegate()                 // Undelegate
useDelegation(account)          // Get delegation

// Stats
useGovernanceStats()            // Overall stats
useCategoryStats(category)      // Category stats
```

## Next Steps

1. ✅ Copy one of the examples above
2. ✅ Replace `YOUR_ACCOUNT_ADDRESS` with actual account
3. ✅ Add your UI styling
4. ✅ Enable Polkadot extension
5. ✅ Test voting on testnet first

## Need Help?

- See `README.md` for complete documentation
- Check `example.tsx` for full component example
- Run `npx tsx lib/governance/test.ts` to test connection

## Common Patterns

### Loading States
```typescript
if (loading) return <Spinner />;
if (error) return <ErrorMessage error={error} />;
```

### Error Handling
```typescript
const result = await castVote(...);
if (!result.success) {
  alert(result.error?.message);
}
```

### Real-time Updates
```typescript
const { proposals, refetch } = useActiveProposals();

// Refetch every 12 seconds (2 blocks)
useEffect(() => {
  const interval = setInterval(refetch, 12000);
  return () => clearInterval(interval);
}, [refetch]);
```

That's it! You're ready to integrate ETRID governance into your web wallet.
