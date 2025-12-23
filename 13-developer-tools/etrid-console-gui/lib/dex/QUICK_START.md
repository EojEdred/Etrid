# Quick Start Guide - ETRID DEX Service

## 1. Import the Service

```typescript
import { DexService, useSwapUI, useWETRAddresses } from '@/lib/dex';
```

## 2. Get All wETR Addresses

```typescript
import { useWETRAddresses } from '@/lib/dex';

function MyComponent() {
  const wETRAddresses = useWETRAddresses();

  return (
    <div>
      <p>Ethereum: {wETRAddresses.ethereum}</p>
      <p>BSC: {wETRAddresses.bsc}</p>
      <p>Polygon: {wETRAddresses.polygon}</p>
      <p>Arbitrum: {wETRAddresses.arbitrum}</p>
      <p>Solana: {wETRAddresses.solana}</p>
    </div>
  );
}
```

## 3. Simple Swap (Using Hooks)

```typescript
'use client';

import { useSwapUI } from '@/lib/dex';
import { useSigner } from 'wagmi';

export function SimpleSwap() {
  const { data: signer } = useSigner();
  const {
    amount,
    setAmount,
    fromToken,
    setFromToken,
    toToken,
    setToToken,
    quote,
    executeSwap,
    isSwapping,
    wETRAddress,
  } = useSwapUI('ethereum');

  const handleSwap = async () => {
    if (!signer) return;
    const result = await executeSwap(
      { fromToken, toToken, amount, slippage: 0.5 },
      signer
    );
    console.log('Success:', result.txHash);
  };

  return (
    <div>
      <button onClick={() => setFromToken(wETRAddress || '')}>
        Use wETR
      </button>
      <input
        value={amount}
        onChange={(e) => setAmount(e.target.value)}
        placeholder="Amount"
      />
      <input
        value={toToken}
        onChange={(e) => setToToken(e.target.value)}
        placeholder="To Token Address"
      />
      {quote && <p>You will receive: {quote.amountOut}</p>}
      <button onClick={handleSwap} disabled={isSwapping}>
        {isSwapping ? 'Swapping...' : 'Swap'}
      </button>
    </div>
  );
}
```

## 4. Simple Swap (Direct Service)

```typescript
import { DexService } from '@/lib/dex';
import { ethers } from 'ethers';

async function executeSimpleSwap() {
  // Initialize
  const dex = new DexService('ethereum');
  const wETR = dex.getWETRAddress();

  // Get quote
  const quote = await dex.getSwapQuote({
    fromToken: wETR,
    toToken: '0xC02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2', // WETH
    amount: '1000000000000000000', // 1 wETR
    slippage: 0.5,
  });

  console.log('Quote:', quote);

  // Execute (with signer)
  const provider = new ethers.BrowserProvider(window.ethereum);
  const signer = await provider.getSigner();

  const tx = await dex.executeSwap(signer, {
    fromToken: wETR,
    toToken: '0xC02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2',
    amount: '1000000000000000000',
    slippage: 0.5,
  });

  const receipt = await tx.wait();
  console.log('Success:', receipt.hash);
}
```

## 5. Get Token Balance

```typescript
import { useTokenBalance } from '@/lib/dex';
import { useAccount } from 'wagmi';

function TokenBalance() {
  const { address } = useAccount();
  const wETRAddress = '0x5566f6fb5cdb3aadf8662f9d1218ce2fc4bc72fb';

  const { balance, isLoading } = useTokenBalance(
    'ethereum',
    wETRAddress,
    address
  );

  return <p>Balance: {balance || '0'}</p>;
}
```

## 6. Get Pool Information

```typescript
import { usePoolInfo } from '@/lib/dex';

function PoolStats() {
  const { poolInfo } = usePoolInfo(
    'ethereum',
    '0x...', // Token A
    '0x...', // Token B
  );

  return (
    <div>
      <p>Reserve A: {poolInfo?.reserveA}</p>
      <p>Reserve B: {poolInfo?.reserveB}</p>
      <p>Your LP Balance: {poolInfo?.userBalance}</p>
    </div>
  );
}
```

## 7. Multi-Chain Support

```typescript
const chains = ['ethereum', 'bsc', 'polygon', 'arbitrum', 'solana'];

chains.forEach(chain => {
  const dex = new DexService(chain);
  console.log(`${chain}: ${dex.getWETRAddress()}`);
});
```

## Common Use Cases

### Use Case 1: Display wETR Price

```typescript
import { DexService } from '@/lib/dex';

async function getWETRPrice() {
  const dex = new DexService('ethereum');
  const wETR = dex.getWETRAddress();
  const WETH = '0xC02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2';

  const price = await dex.getTokenPrice(
    wETR,
    WETH,
    '1000000000000000000' // 1 wETR
  );

  console.log(`1 wETR = ${price} WETH`);
}
```

### Use Case 2: Check Price Impact Before Swap

```typescript
import { usePriceImpactWarning } from '@/lib/dex';

function SwapWithWarning() {
  const { quote } = useSwapQuote('ethereum', params);
  const warning = usePriceImpactWarning(quote?.priceImpact);

  if (warning?.level === 'error') {
    return <div>⚠️ Price impact too high!</div>;
  }

  // Continue with swap...
}
```

### Use Case 3: Add Liquidity

```typescript
import { useAddLiquidity } from '@/lib/dex';

function AddLiquidityButton() {
  const { addLiquidity, isAdding } = useAddLiquidity('ethereum');
  const { data: signer } = useSigner();

  const handleAdd = async () => {
    const result = await addLiquidity(
      {
        tokenA: '0x...',
        tokenB: '0x...',
        amountA: '1000000000000000000',
        amountB: '1000000000000000000',
        slippage: 0.5,
      },
      signer!
    );
    console.log('LP tokens received');
  };

  return (
    <button onClick={handleAdd} disabled={isAdding}>
      Add Liquidity
    </button>
  );
}
```

## wETR Addresses Reference

```typescript
const WETR_ADDRESSES = {
  ethereum: '0x5566f6fb5cdb3aadf8662f9d1218ce2fc4bc72fb',
  bsc: '0xcc9b37fed77a01329502f8844620577742eb0dc6',
  polygon: '0x5566f6fb5cdb3aadf8662f9d1218ce2fc4bc72fb',
  arbitrum: '0x1A065196152C2A70e54AC06D3a3433e3D8606eF3',
  solana: 'CA4ALvCam7N3ya8d2axp3AakwNdCdQchQNNwYSYiMRR4',
};
```

## Slippage Settings

```typescript
const SLIPPAGE_OPTIONS = {
  low: 0.1,      // 0.1% - Stable pairs
  normal: 0.5,   // 0.5% - Most trades (default)
  high: 1.0,     // 1.0% - Volatile pairs
  veryHigh: 2.0, // 2.0% - Very volatile
};
```

## Error Handling

```typescript
try {
  const result = await executeSwap(params, signer);
  console.log('Success:', result.txHash);
} catch (error) {
  if (error.message.includes('insufficient')) {
    console.error('Insufficient balance');
  } else if (error.message.includes('slippage')) {
    console.error('Slippage exceeded');
  } else {
    console.error('Swap failed:', error.message);
  }
}
```

## Best Practices

1. **Always check balance** before swap
2. **Use appropriate slippage** for the token pair
3. **Show price impact** to users
4. **Handle errors gracefully**
5. **Refresh quotes** regularly (10-30 seconds)
6. **Test on testnet** first
7. **Verify addresses** before execution

## Next Steps

1. Check `example.tsx` for complete component
2. Read `README.md` for full documentation
3. Review `service.ts` for advanced features
4. Test on testnet before mainnet

## Support

- Documentation: `README.md`
- Example: `example.tsx`
- Summary: `IMPLEMENTATION_SUMMARY.md`
