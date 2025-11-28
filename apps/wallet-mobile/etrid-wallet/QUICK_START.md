# Quick Start Guide - dApp Browser & DAO Features

## Installation

### 1. Install Dependencies

```bash
cd /home/user/Etrid/apps/wallet-mobile/etrid-wallet
npm install @walletconnect/sign-client date-fns recharts
```

### 2. Import Example Usage

#### Using dApp Browser

```typescript
import { DAppListScreen } from '@/screens/DAppListScreen';
import { useDAppBrowser } from '@/hooks/useDAppBrowser';
import { useWalletConnect } from '@/hooks/useWalletConnect';

function App() {
  const browser = useDAppBrowser();
  const walletConnect = useWalletConnect();

  const handleOpenDApp = (dApp) => {
    browser.navigateTo(dApp.url);
  };

  return (
    <DAppListScreen
      onOpenDApp={handleOpenDApp}
      onAddCustomDApp={() => {/* show custom dApp dialog */}}
    />
  );
}
```

#### Using DAO Management

```typescript
import { DAOListScreen } from '@/screens/DAOListScreen';
import { ProposalDetailScreen } from '@/screens/ProposalDetailScreen';
import { useDAOs } from '@/hooks/useDAOs';

function DAOApp() {
  const { daos, createDAO } = useDAOs();
  const [selectedDAO, setSelectedDAO] = useState(null);

  return (
    <>
      <DAOListScreen
        onViewDAO={setSelectedDAO}
        onCreateDAO={() => {/* show create DAO wizard */}}
      />

      {selectedDAO && (
        <DAODashboard dao={selectedDAO} />
      )}
    </>
  );
}
```

## Common Use Cases

### 1. Browse and Open dApp

```typescript
import { useDAppDirectory } from '@/hooks/useDAppDirectory';

function BrowseDApps() {
  const { featuredDApps, searchDApps, searchResults } = useDAppDirectory();

  const handleSearch = (query: string) => {
    searchDApps(query);
  };

  return (
    <div>
      <input onChange={(e) => handleSearch(e.target.value)} />
      {searchResults.map(dApp => (
        <DAppCard key={dApp.id} dApp={dApp} onOpen={openInBrowser} />
      ))}
    </div>
  );
}
```

### 2. Connect to dApp via WalletConnect

```typescript
import { useWalletConnect } from '@/hooks/useWalletConnect';

function WalletConnectExample() {
  const { pair, proposals, approveSession } = useWalletConnect();

  const handleScan = async (wcUri: string) => {
    await pair(wcUri);
    // Proposals will appear automatically
  };

  return (
    <>
      <QRScanner onScan={handleScan} />

      {proposals.map(proposal => (
        <WalletConnectModal
          key={proposal.id}
          proposal={proposal}
          onApprove={(remember) => approveSession(proposal.id)}
        />
      ))}
    </>
  );
}
```

### 3. Create DAO

```typescript
import { useDAOs } from '@/hooks/useDAOs';

function CreateDAO() {
  const { createDAO } = useDAOs();

  const handleCreate = async () => {
    const dao = await createDAO({
      name: 'My DAO',
      description: 'A community DAO',
      governance: {
        votingPeriod: 7, // days
        quorum: 20, // percentage
        proposalThreshold: 10, // tokens
        executionDelay: 2, // days
        votingStrategy: 'token-weighted',
        membershipType: 'open'
      },
      initialMembers: [],
      initialTreasuryAmount: '1000'
    });

    console.log('DAO created:', dao);
  };

  return <button onClick={handleCreate}>Create DAO</button>;
}
```

### 4. Create and Vote on Proposal

```typescript
import { useDAOProposals } from '@/hooks/useDAOProposals';

function ProposalFlow() {
  const { createProposal, vote } = useDAOProposals('dao-id');

  const handleCreateProposal = async () => {
    const proposal = await createProposal('dao-id', {
      title: 'Upgrade Treasury',
      description: 'Proposal to upgrade treasury contract',
      type: 'governance',
      executionData: { /* contract call data */ }
    });
  };

  const handleVote = async (proposalId: string) => {
    await vote(proposalId, 'for', 'I support this proposal');
  };

  return (
    <>
      <button onClick={handleCreateProposal}>Create Proposal</button>
      {/* proposals list with vote buttons */}
    </>
  );
}
```

### 5. Manage Treasury

```typescript
import { useDAOTreasury } from '@/hooks/useDAOTreasury';

function TreasuryManagement() {
  const { treasury, proposeSpend, analytics } = useDAOTreasury('dao-id');

  const handleProposeSpend = async () => {
    const proposal = await proposeSpend('dao-id', {
      recipient: '0x...',
      asset: 'ETH',
      amount: '10',
      reason: 'Grant for developer'
    });
  };

  return (
    <div>
      <h2>Treasury: ${treasury?.totalValue}</h2>
      <TreasuryChart
        assets={treasury?.assets || []}
        totalValue={treasury?.totalValue || '0'}
      />
      <button onClick={handleProposeSpend}>Propose Spend</button>
    </div>
  );
}
```

## Component Examples

### Using Pre-built Components

```typescript
import { DAppCard } from '@/components/dapp/DAppCard';
import { ProposalCard } from '@/components/dao/ProposalCard';
import { DAOCard } from '@/components/dao/DAOCard';

function ComponentExamples() {
  return (
    <>
      {/* dApp Card */}
      <DAppCard
        dApp={{
          id: '1',
          name: 'Uniswap',
          url: 'https://app.uniswap.org',
          category: 'DeFi',
          description: 'Decentralized exchange',
          iconUrl: 'https://...',
          rating: 4.8,
          userCount: 1000000
        }}
        onOpen={(dApp) => console.log('Open:', dApp)}
      />

      {/* DAO Card */}
      <DAOCard
        dao={{
          id: '1',
          name: 'My DAO',
          description: 'Community DAO',
          memberCount: 100,
          treasuryValue: '50000',
          activeProposalCount: 3,
          userRole: 'member'
        }}
        onClick={(dao) => console.log('View:', dao)}
      />

      {/* Proposal Card */}
      <ProposalCard
        proposal={/* proposal object */}
        onVote={(id, vote) => console.log('Vote:', id, vote)}
        onViewDetails={(id) => console.log('View:', id)}
      />
    </>
  );
}
```

## Service Direct Usage

### dApp Browser Service

```typescript
import { dAppBrowserService } from '@/services/DAppBrowserService';

// Inject Web3 provider
await dAppBrowserService.injectWeb3Provider(webViewRef);

// Handle dApp request
const result = await dAppBrowserService.handleDAppRequest({
  id: '1',
  dAppUrl: 'https://uniswap.org',
  method: 'eth_requestAccounts',
  params: [],
  timestamp: new Date()
});
```

### WalletConnect Service

```typescript
import { walletConnectService } from '@/services/WalletConnectService';

// Initialize
await walletConnectService.initialize();

// Pair
await walletConnectService.pair('wc:...');

// Get sessions
const sessions = await walletConnectService.getSessions();

// Disconnect
await walletConnectService.disconnect('session-id');
```

### DAO Service

```typescript
import { daoService } from '@/services/DAOService';

// Get user's DAOs
const daos = await daoService.getDAOs();

// Get DAO details
const details = await daoService.getDAODetails('dao-id');

// Join DAO
await daoService.joinDAO('dao-id');
```

## Testing Examples

```typescript
import { renderHook, act } from '@testing-library/react-hooks';
import { useDAOProposals } from '@/hooks/useDAOProposals';

describe('useDAOProposals', () => {
  it('should create proposal', async () => {
    const { result } = renderHook(() => useDAOProposals('dao-1'));

    await act(async () => {
      await result.current.createProposal('dao-1', {
        title: 'Test',
        description: 'Test proposal',
        type: 'governance'
      });
    });

    expect(result.current.proposals).toHaveLength(1);
  });

  it('should vote on proposal', async () => {
    const { result } = renderHook(() => useDAOProposals('dao-1'));

    await act(async () => {
      await result.current.vote('proposal-1', 'for');
    });

    expect(result.current.error).toBeNull();
  });
});
```

## Troubleshooting

### Issue: WalletConnect not connecting

**Solution**: Make sure to initialize the service:
```typescript
await walletConnectService.initialize();
```

### Issue: Transactions not signing

**Solution**: Check that Web3 provider is injected:
```typescript
await dAppBrowserService.injectWeb3Provider(webView);
```

### Issue: Proposals not loading

**Solution**: Ensure daoId is passed to the hook:
```typescript
const { proposals } = useDAOProposals(daoId);
```

### Issue: Type errors

**Solution**: Import types from the correct location:
```typescript
import { DApp, DAO, Proposal } from '@/types/dapp';
import { DAO, Proposal, Vote } from '@/types/dao';
```

## Best Practices

1. **Always handle errors**
   ```typescript
   try {
     await createDAO(params);
   } catch (error) {
     console.error('Failed to create DAO:', error);
     showErrorToast(error.message);
   }
   ```

2. **Use loading states**
   ```typescript
   const { isLoading } = useDAOs();
   if (isLoading) return <Spinner />;
   ```

3. **Validate user input**
   ```typescript
   if (!daoName || !description) {
     throw new Error('Name and description required');
   }
   ```

4. **Clean up on unmount**
   ```typescript
   useEffect(() => {
     return () => {
       // Cleanup subscriptions
     };
   }, []);
   ```

5. **Optimize re-renders**
   ```typescript
   const memoizedCallback = useCallback(() => {
     // callback logic
   }, [dependencies]);
   ```

## Additional Resources

- **Documentation**: See `DAPP_DAO_FEATURES.md`
- **Implementation Details**: See `IMPLEMENTATION_SUMMARY.md`
- **Type Definitions**: Check `types/dapp.ts` and `types/dao.ts`
- **Example Screens**: Review `screens/` directory

## Support

For questions or issues:
1. Check type definitions for API details
2. Review service implementations
3. Consult example screens for usage patterns
4. Test with mock data first

Happy building!
