# ETRID Governance Service

Complete governance functionality for the ETRID web wallet, matching the iOS app implementation.

## Features

- **Proposal Management**: Submit, query, and track governance proposals
- **Conviction Voting**: Vote with conviction multipliers (0.1x to 6x)
- **Vote Delegation**: Delegate voting power to trusted accounts
- **Real-time Stats**: Track governance statistics and participation
- **Six Proposal Categories**:
  - InflationRate - Modify network inflation parameters
  - ParameterChange - Update runtime parameters
  - BudgetAllocation - Treasury fund allocation
  - ProtocolUpgrade - Runtime and protocol upgrades
  - DirectorElection - Elect network directors
  - EmergencyAction - Emergency protocol actions

## Installation

The governance service is already included in the project. Import it in your components:

```typescript
import {
  useProposals,
  useCastVote,
  useVotingPower,
  ProposalCategory,
  ConvictionLevel
} from '@/lib/governance';
```

## Usage Examples

### 1. Fetch Active Proposals

```typescript
import { useActiveProposals } from '@/lib/governance';

function ProposalsList() {
  const { proposals, loading, error } = useActiveProposals();

  if (loading) return <div>Loading proposals...</div>;
  if (error) return <div>Error: {error.message}</div>;

  return (
    <div>
      {proposals.map(proposal => (
        <div key={proposal.id}>
          <h3>{proposal.title}</h3>
          <p>{proposal.description}</p>
          <span>{proposal.category}</span>
        </div>
      ))}
    </div>
  );
}
```

### 2. Cast a Vote with Conviction

```typescript
import { useCastVote, ConvictionLevel, VoteType } from '@/lib/governance';

function VoteButton({ proposalId }: { proposalId: number }) {
  const { castVote, voting, result } = useCastVote();

  const handleVote = async () => {
    // Get signer from Polkadot extension
    const { web3FromAddress } = await import('@polkadot/extension-dapp');
    const injector = await web3FromAddress(userAddress);

    await castVote({
      proposalId,
      voteType: VoteType.Aye,
      conviction: ConvictionLevel.Locked3x, // 3x multiplier, 28 days lock
    }, injector.signer);
  };

  return (
    <button onClick={handleVote} disabled={voting}>
      {voting ? 'Voting...' : 'Vote Yes (3x)'}
    </button>
  );
}
```

### 3. Check Voting Power

```typescript
import { useVotingPower } from '@/lib/governance';

function VotingPowerDisplay({ account }: { account: string }) {
  const { votingPower, loading } = useVotingPower(account);

  if (loading) return <div>Loading...</div>;

  return (
    <div>
      <p>Staked: {votingPower?.stakedBalance} ETR</p>
      <p>Voting Power: {votingPower?.totalVotingPower}</p>
      <p>Active Votes: {votingPower?.activeVotes}</p>
    </div>
  );
}
```

### 4. Submit a Proposal

```typescript
import { useSubmitProposal, ProposalCategory } from '@/lib/governance';

function CreateProposal() {
  const { submitProposal, submitting } = useSubmitProposal();

  const handleSubmit = async (formData: any) => {
    const { web3FromAddress } = await import('@polkadot/extension-dapp');
    const injector = await web3FromAddress(userAddress);

    const result = await submitProposal({
      title: formData.title,
      description: formData.description,
      category: ProposalCategory.ParameterChange,
      metadata: {
        parameter: formData.parameter,
        newValue: formData.value,
      },
    }, injector.signer);

    if (result.success) {
      console.log('Proposal submitted:', result.txHash);
    }
  };

  return (
    <form onSubmit={handleSubmit}>
      {/* Form fields */}
    </form>
  );
}
```

### 5. Delegate Voting Power

```typescript
import { useDelegate, ConvictionLevel } from '@/lib/governance';

function DelegateForm() {
  const { delegate, delegating } = useDelegate();

  const handleDelegate = async (targetAddress: string) => {
    const { web3FromAddress } = await import('@polkadot/extension-dapp');
    const injector = await web3FromAddress(userAddress);

    await delegate({
      target: targetAddress,
      conviction: ConvictionLevel.Locked2x, // 2x multiplier, 14 days
      balance: '1000000000000000000', // 1 ETR in wei
    }, injector.signer);
  };

  return (
    <button onClick={() => handleDelegate(targetAddress)} disabled={delegating}>
      Delegate Voting Power
    </button>
  );
}
```

### 6. Display Proposal Statistics

```typescript
import { useVotingStats } from '@/lib/governance';

function ProposalStats({ proposalId }: { proposalId: number }) {
  const { stats, loading } = useVotingStats(proposalId);

  if (loading) return <div>Loading stats...</div>;

  return (
    <div>
      <p>Total Votes: {stats?.totalVotes}</p>
      <p>Aye Power: {stats?.ayePower}</p>
      <p>Nay Power: {stats?.nayPower}</p>
      <p>Passing: {stats?.passingPercent.toFixed(2)}%</p>
    </div>
  );
}
```

### 7. Filter Proposals by Category

```typescript
import { useProposalsByCategory, ProposalCategory } from '@/lib/governance';

function DirectorElections() {
  const { proposals } = useProposalsByCategory(ProposalCategory.DirectorElection);

  return (
    <div>
      <h2>Director Elections</h2>
      {proposals.map(proposal => (
        <ProposalCard key={proposal.id} proposal={proposal} />
      ))}
    </div>
  );
}
```

## Conviction Voting Levels

The governance system supports conviction voting with the following multipliers:

| Level | Multiplier | Lock Period | Description |
|-------|-----------|-------------|-------------|
| None | 0.1x | 0 days | No lock period |
| Locked1x | 1x | 7 days | Standard voting power |
| Locked2x | 2x | 14 days | Double voting power |
| Locked3x | 3x | 28 days | Triple voting power |
| Locked4x | 4x | 56 days | 4x voting power |
| Locked5x | 5x | 112 days | 5x voting power |
| Locked6x | 6x | 224 days | 6x voting power |

Higher conviction levels give you more voting power but lock your tokens for longer periods.

## Available Hooks

### Proposal Hooks
- `useProposals(filters?)` - Fetch all proposals with optional filters
- `useProposal(proposalId)` - Fetch a single proposal
- `useActiveProposals()` - Fetch only active proposals
- `useProposalsByCategory(category)` - Filter proposals by category
- `usePaginatedProposals(params, filters?)` - Fetch proposals with pagination
- `useSubmitProposal()` - Submit a new proposal

### Voting Hooks
- `useCastVote()` - Cast a vote on a proposal
- `useVotes(proposalId)` - Fetch all votes for a proposal
- `useVotingStats(proposalId)` - Get voting statistics
- `useHasVoted(proposalId, account)` - Check if account has voted

### Voting Power Hooks
- `useVotingPower(account)` - Get voting power for an account
- `useVotingPowerBreakdown(balance, conviction)` - Calculate voting power with conviction

### Delegation Hooks
- `useDelegate()` - Delegate voting power
- `useUndelegate()` - Remove delegation
- `useDelegation(account)` - Get active delegation
- `useDelegationStats(account)` - Get delegation statistics

### Statistics Hooks
- `useGovernanceStats()` - Overall governance statistics
- `useCategoryStats(category)` - Statistics for a specific category

### Utility Hooks
- `useGovernanceConnection()` - Manage connection to the chain
- `useGovernancePolling(fetchFn, interval)` - Poll data at regular intervals
- `useGovernance(account)` - Comprehensive hook combining multiple data sources
- `useTimeRemaining(blocksRemaining)` - Calculate time from block count

## Direct Service Usage

If you need more control, you can use the service directly:

```typescript
import { governanceService } from '@/lib/governance';

// Fetch proposals
const proposals = await governanceService.getProposals();

// Get voting power
const votingPower = await governanceService.getVotingPower(account);

// Cast vote
const result = await governanceService.castVote({
  proposalId: 1,
  voteType: VoteType.Aye,
  conviction: ConvictionLevel.Locked2x,
}, signer);
```

## Connection

The service automatically connects to the ETRID chain at `wss://rpc.etrid.org` with automatic failover to backup endpoints. Connection is established lazily when first needed.

## Error Handling

All hooks return error states that you should handle:

```typescript
const { proposals, loading, error, refetch } = useProposals();

if (error) {
  return (
    <div>
      <p>Error: {error.message}</p>
      <button onClick={refetch}>Retry</button>
    </div>
  );
}
```

Transaction hooks return `TransactionResult` with success status and error details:

```typescript
const { castVote, result } = useCastVote();

if (result && !result.success) {
  console.error('Vote failed:', result.error?.message);
}
```

## TypeScript Support

All types are fully typed. Import types as needed:

```typescript
import type {
  Proposal,
  ProposalCategory,
  VoteType,
  ConvictionLevel,
  VotingPower,
  TransactionResult
} from '@/lib/governance';
```

## Polkadot.js Integration

The service uses Polkadot.js API and extension integration. Make sure users have a Polkadot wallet extension installed (Polkadot.js, Talisman, SubWallet, etc.).

Enable web3 before using transaction hooks:

```typescript
import { web3Enable, web3FromAddress } from '@polkadot/extension-dapp';

// Enable extension
await web3Enable('ETRID Wallet');

// Get signer
const injector = await web3FromAddress(account);
const signer = injector.signer;
```

## Best Practices

1. **Connection Management**: The service manages its own connection. Don't disconnect unless unmounting.

2. **Polling**: Use `useGovernancePolling` for real-time data that needs frequent updates.

3. **Pagination**: For large lists, use `usePaginatedProposals` instead of `useProposals`.

4. **Error Boundaries**: Wrap governance components in error boundaries for better UX.

5. **Loading States**: Always show loading indicators while fetching data.

6. **Transaction Feedback**: Provide clear feedback for transaction states (signing, broadcasting, finalized).

## Chain Configuration

The service connects to:
- **Primary RPC**: `wss://rpc.etrid.org`
- **Fallback 1**: `ws://157.173.200.80:9944`
- **Fallback 2**: `ws://100.96.84.69:9944` (Tailscale)

Automatic failover ensures high availability.

## Support

For issues or questions about the governance service, check the ETRID documentation or contact the development team.
