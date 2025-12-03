# ETRID DEX Service

A comprehensive, multi-chain DEX integration service for the ETRID web wallet, supporting wrapped ETR (wETR) trading across Ethereum, BSC, Polygon, Arbitrum, and Solana.

## Features

- **Multi-chain Support**: Trade wETR on 5 different chains
- **DEX Integration**: Uniswap V2, PancakeSwap V2, QuickSwap, Camelot, and Raydium
- **Complete Trading Features**: Swap, add/remove liquidity, price quotes
- **React Hooks**: Easy-to-use hooks for React applications
- **Price Impact Calculation**: Real-time price impact warnings
- **Slippage Protection**: Configurable slippage tolerance
- **TypeScript**: Full type safety

## Supported Chains & DEXs

| Chain | DEX | wETR Address |
|-------|-----|-------------|
| Ethereum | Uniswap V2 | `0x5566f6fb5cdb3aadf8662f9d1218ce2fc4bc72fb` |
| BSC | PancakeSwap V2 | `0xcc9b37fed77a01329502f8844620577742eb0dc6` |
| Polygon | QuickSwap | `0x5566f6fb5cdb3aadf8662f9d1218ce2fc4bc72fb` |
| Arbitrum | Camelot | `0x1A065196152C2A70e54AC06D3a3433e3D8606eF3` |
| Solana | Raydium | `CA4ALvCam7N3ya8d2axp3AakwNdCdQchQNNwYSYiMRR4` |

## Installation

The service uses existing dependencies in your project:
- `ethers` for EVM chains
- `@solana/web3.js` for Solana
- `wagmi` for React hooks

## Quick Start

### 1. Initialize DEX Service

```typescript
import { DexService } from '@/lib/dex';

// Initialize for a specific chain
const dexService = new DexService('ethereum'); // or 'bsc', 'polygon', 'arbitrum', 'solana'

// Get wETR address for the chain
const wETRAddress = dexService.getWETRAddress();
console.log('wETR Address:', wETRAddress);

// Get DEX name
const dexName = dexService.getDexName();
console.log('DEX:', dexName); // "Uniswap V2"
```

### 2. Get Token Price

```typescript
// Get price quote
const amountIn = '1000000000000000000'; // 1 wETR (18 decimals)
const tokenOut = '0xC02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2'; // WETH

const price = await dexService.getTokenPrice(wETRAddress, tokenOut, amountIn);
console.log('Price:', price);
```

### 3. Get Swap Quote with Price Impact

```typescript
const quote = await dexService.getSwapQuote({
  fromToken: wETRAddress,
  toToken: tokenOut,
  amount: amountIn,
  slippage: 0.5, // 0.5% slippage
});

console.log('Amount Out:', quote.amountOut);
console.log('Price Impact:', quote.priceImpact, '%');
console.log('Minimum Received:', quote.minimumReceived);
```

### 4. Execute Swap

```typescript
import { ethers } from 'ethers';

// Get signer from wallet
const provider = new ethers.BrowserProvider(window.ethereum);
const signer = await provider.getSigner();

// Execute swap
const tx = await dexService.executeSwap(signer, {
  fromToken: wETRAddress,
  toToken: tokenOut,
  amount: amountIn,
  slippage: 0.5,
});

// Wait for confirmation
const receipt = await tx.wait();
console.log('Swap completed:', receipt.hash);
```

## React Hooks Usage

### useSwapUI - Complete Swap Interface

The easiest way to build a swap UI:

```typescript
'use client';

import { useSwapUI } from '@/lib/dex';
import { useSigner } from 'wagmi';

export function SwapComponent() {
  const { data: signer } = useSigner();
  const {
    // State
    fromToken,
    toToken,
    amount,
    slippage,
    // Setters
    setFromToken,
    setToToken,
    setAmount,
    setSlippage,
    // Quote
    quote,
    isLoadingQuote,
    quoteError,
    // Swap
    executeSwap,
    isSwapping,
    swapError,
    txHash,
    // Validation
    priceImpactWarning,
    hasInsufficientBalance,
    // Balances
    fromBalance,
    toBalance,
    // Config
    wETRAddress,
  } = useSwapUI('ethereum');

  const handleSwap = async () => {
    if (!signer) return;

    try {
      const result = await executeSwap(
        {
          fromToken,
          toToken,
          amount,
          slippage,
        },
        signer
      );
      console.log('Swap successful:', result.txHash);
    } catch (error) {
      console.error('Swap failed:', error);
    }
  };

  return (
    <div>
      <input
        type="text"
        value={amount}
        onChange={(e) => setAmount(e.target.value)}
        placeholder="Amount"
      />

      {quote && (
        <div>
          <p>You will receive: {quote.amountOut}</p>
          <p>Price Impact: {quote.priceImpact.toFixed(2)}%</p>
        </div>
      )}

      {priceImpactWarning && (
        <div className={`alert-${priceImpactWarning.level}`}>
          {priceImpactWarning.message}
        </div>
      )}

      <button
        onClick={handleSwap}
        disabled={isSwapping || isLoadingQuote || hasInsufficientBalance}
      >
        {isSwapping ? 'Swapping...' : 'Swap'}
      </button>

      {txHash && <p>Transaction: {txHash}</p>}
    </div>
  );
}
```

### Individual Hooks

#### useSwapQuote - Get Real-time Quotes

```typescript
import { useSwapQuote } from '@/lib/dex';

const { quote, isLoading, error, refetch } = useSwapQuote(
  'ethereum',
  {
    fromToken: '0x...', // wETR address
    toToken: '0x...', // Other token
    amount: '1000000000000000000',
    slippage: 0.5,
  },
  {
    enabled: true,
    refetchInterval: 10000, // Refresh every 10 seconds
  }
);
```

#### useSwap - Execute Swaps

```typescript
import { useSwap } from '@/lib/dex';
import { useSigner } from 'wagmi';

const { executeSwap, isSwapping, error, txHash } = useSwap('ethereum');
const { data: signer } = useSigner();

const handleSwap = async () => {
  if (!signer) return;

  const result = await executeSwap(
    {
      fromToken: '0x...',
      toToken: '0x...',
      amount: '1000000000000000000',
      slippage: 0.5,
    },
    signer
  );
};
```

#### usePoolInfo - Get Liquidity Pool Data

```typescript
import { usePoolInfo } from '@/lib/dex';

const { poolInfo, isLoading, error } = usePoolInfo(
  'ethereum',
  '0x...', // Token A
  '0x...', // Token B
  {
    refetchInterval: 30000, // Refresh every 30 seconds
  }
);

// Access pool data
console.log('Reserve A:', poolInfo?.reserveA);
console.log('Reserve B:', poolInfo?.reserveB);
console.log('Your LP Balance:', poolInfo?.userBalance);
```

#### useTokenBalance - Get Token Balance

```typescript
import { useTokenBalance } from '@/lib/dex';
import { useAccount } from 'wagmi';

const { address } = useAccount();
const { balance, isLoading, refetch } = useTokenBalance(
  'ethereum',
  '0x...', // Token address
  address
);
```

#### useAddLiquidity / useRemoveLiquidity

```typescript
import { useAddLiquidity } from '@/lib/dex';
import { useSigner } from 'wagmi';

const { addLiquidity, isAdding, error } = useAddLiquidity('ethereum');
const { data: signer } = useSigner();

const handleAddLiquidity = async () => {
  if (!signer) return;

  const result = await addLiquidity(
    {
      tokenA: '0x...',
      tokenB: '0x...',
      amountA: '1000000000000000000',
      amountB: '1000000000000000000',
      slippage: 0.5,
    },
    signer
  );
};
```

### Utility Hooks

```typescript
// Get all wETR addresses
const wETRAddresses = useWETRAddresses();
console.log(wETRAddresses); // { ethereum: '0x...', bsc: '0x...', ... }

// Get supported chains
const chains = useSupportedChains();

// Format token amounts
const format = useFormatTokenAmount();
const formatted = format('1000000000000000000', 18); // "1.0"

// Parse token amounts
const parse = useParseTokenAmount();
const parsed = parse('1.0', 18); // "1000000000000000000"

// Price impact warning
const warning = usePriceImpactWarning(5.2);
// { level: 'warning', message: '...' }
```

## Advanced Usage

### Custom RPC Endpoints

```typescript
import { EVMDexService } from '@/lib/dex';
import { ethers } from 'ethers';

// Use custom RPC
const provider = new ethers.JsonRpcProvider('https://your-rpc-url.com');
const dexService = new EVMDexService(provider, 1); // Ethereum mainnet
```

### Direct DEX Service Usage

```typescript
import { EVMDexService, SolanaDexService } from '@/lib/dex';

// EVM chains
const evmDex = new EVMDexService(provider, 56); // BSC

// Solana
const solanaDex = new SolanaDexService('https://api.mainnet-beta.solana.com');
```

### Add Liquidity

```typescript
const tx = await dexService.addLiquidity(signer, {
  tokenA: wETRAddress,
  tokenB: '0x...', // Other token
  amountA: '1000000000000000000',
  amountB: '1000000000000000000',
  slippage: 0.5,
});
```

### Remove Liquidity

```typescript
const poolInfo = await dexService.getLiquidityPoolInfo(tokenA, tokenB);
const lpBalance = poolInfo.userBalance;

const tx = await dexService.removeLiquidity(
  signer,
  tokenA,
  tokenB,
  lpBalance, // Amount of LP tokens to burn
  0.5 // Slippage
);
```

## Error Handling

```typescript
try {
  const quote = await dexService.getSwapQuote({
    fromToken: wETRAddress,
    toToken: tokenOut,
    amount: amountIn,
    slippage: 0.5,
  });
} catch (error) {
  if (error instanceof Error) {
    if (error.message.includes('Liquidity pool does not exist')) {
      console.error('No pool found for this token pair');
    } else if (error.message.includes('insufficient')) {
      console.error('Insufficient balance');
    } else {
      console.error('Error:', error.message);
    }
  }
}
```

## Price Impact Warnings

The service automatically calculates price impact and provides warnings:

- **< 2%**: Low impact (no warning)
- **2-5%**: Moderate impact (info)
- **5-15%**: High impact (warning)
- **> 15%**: Very high impact (error)

```typescript
const { priceImpactWarning } = usePriceImpactWarning(quote?.priceImpact);

if (priceImpactWarning?.level === 'error') {
  // Show error message to user
}
```

## Gas Estimation

For EVM chains, gas is estimated automatically. You can customize gas settings:

```typescript
// Gas will be estimated automatically, but you can override
const tx = await dexService.executeSwap(signer, swapParams);

// Or customize before sending
const populatedTx = await tx.populateTransaction();
populatedTx.gasLimit = BigInt(300000);
const result = await signer.sendTransaction(populatedTx);
```

## Slippage Configuration

Default slippage is 0.5%. Adjust based on market conditions:

```typescript
// Low slippage for stable pairs
const quote = await dexService.getSwapQuote({
  ...params,
  slippage: 0.1, // 0.1%
});

// Higher slippage for volatile pairs
const quote = await dexService.getSwapQuote({
  ...params,
  slippage: 2.0, // 2%
});
```

## Testing

### Mock Data

```typescript
// In your tests
jest.mock('@/lib/dex', () => ({
  useDexService: () => ({
    service: mockService,
    isInitialized: true,
    wETRAddress: '0x...',
  }),
  useSwapQuote: () => ({
    quote: {
      amountOut: '1000000000000000000',
      priceImpact: 0.5,
      minimumReceived: '995000000000000000',
    },
    isLoading: false,
    error: null,
  }),
}));
```

## Architecture

```
lib/dex/
   service.ts       # Main unified service
   evm.ts          # EVM chain implementation
   solana.ts       # Solana implementation
   hooks.ts        # React hooks
   index.ts        # Exports
   README.md       # Documentation
```

## Contributing

When adding new chains:

1. Add chain config to `SUPPORTED_CHAINS` in `service.ts`
2. Add DEX config to `EVM_DEX_CONFIGS` in `evm.ts` (for EVM chains)
3. Update wETR address mapping
4. Add tests for new chain

## License

MIT

## Support

For issues or questions, please open an issue in the ETRID repository.
