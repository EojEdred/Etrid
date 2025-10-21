/**
 * Swap API Functions
 * Handles ÉTR ↔ EDSC token swaps
 */

import { ApiPromise } from '@polkadot/api';
import { createApi, getBalance } from './api';
import type { ChainId } from './chains';

export interface SwapQuote {
  fromAmount: bigint;
  toAmount: bigint;
  exchangeRate: number;
  priceImpact: number;
  fee: bigint;
}

/**
 * Get swap quote for ÉTR → EDSC or EDSC → ÉTR
 */
export async function getSwapQuote(
  fromToken: 'ÉTR' | 'EDSC',
  toToken: 'ÉTR' | 'EDSC',
  fromAmount: string
): Promise<SwapQuote> {
  // Exchange rate: 1 ÉTR = 8 EDSC (pegged to $8 USD)
  const ETR_TO_EDSC_RATE = 8.0;
  const EDSC_TO_ETR_RATE = 1 / ETR_TO_EDSC_RATE;

  const fromAmountBigInt = BigInt(Math.floor(parseFloat(fromAmount) * 1e12)); // Convert to plancks
  const rate = fromToken === 'ÉTR' ? ETR_TO_EDSC_RATE : EDSC_TO_ETR_RATE;

  // Calculate to amount
  const toAmountBigInt = BigInt(Math.floor(Number(fromAmountBigInt) * rate));

  // Fee: 0.3% of from amount
  const feeBigInt = fromAmountBigInt * BigInt(3) / BigInt(1000);

  return {
    fromAmount: fromAmountBigInt,
    toAmount: toAmountBigInt,
    exchangeRate: rate,
    priceImpact: 0, // No price impact for pegged stablecoin
    fee: feeBigInt,
  };
}

/**
 * Get EDSC balance for an address
 */
export async function getEdscBalance(address: string): Promise<{
  balance: string;
  balanceRaw: bigint;
}> {
  try {
    const api = await createApi('edsc-pbc');
    const accountData = await api.query.system.account(address);
    const balance = accountData.data.free.toBigInt();

    return {
      balance: (Number(balance) / 1e12).toFixed(4),
      balanceRaw: balance,
    };
  } catch (error) {
    console.error('[EDSC] Failed to fetch balance:', error);
    return { balance: '0', balanceRaw: BigInt(0) };
  }
}

/**
 * Execute swap transaction
 * In production, this would call a DEX pallet or bridge pallet
 * For now, it uses a simple transfer mechanism
 */
export async function executeSwap(
  fromToken: 'ÉTR' | 'EDSC',
  toToken: 'ÉTR' | 'EDSC',
  fromAmount: string,
  fromAddress: string,
  signer: any
): Promise<string> {
  const quote = await getSwapQuote(fromToken, toToken, fromAmount);

  // In production: call swap pallet extrinsic
  // api.tx.swap.execute(fromToken, toToken, fromAmount, minToAmount)

  // For now, simulate swap with system.remark
  const chainId: ChainId = fromToken === 'ÉTR' ? 'flarechain' : 'edsc-pbc';
  const api = await createApi(chainId);

  return new Promise((resolve, reject) => {
    const swapData = {
      type: 'token_swap',
      fromToken,
      toToken,
      fromAmount: fromAmount,
      toAmount: (Number(quote.toAmount) / 1e12).toFixed(4),
      exchangeRate: quote.exchangeRate,
      fee: (Number(quote.fee) / 1e12).toFixed(4),
      timestamp: Date.now(),
    };

    const tx = api.tx.system.remark(JSON.stringify(swapData));

    tx.signAndSend(
      fromAddress,
      { signer },
      ({ status, txHash }) => {
        if (status.isInBlock) {
          console.log(`[Swap] Transaction in block: ${txHash.toHex()}`);
          resolve(txHash.toHex());
        } else if (status.isFinalized) {
          console.log(`[Swap] Transaction finalized`);
        } else if (status.isInvalid) {
          reject(new Error('Swap transaction invalid'));
        }
      }
    ).catch(reject);
  });
}

/**
 * Get multi-chain balances for ÉTR and EDSC
 */
export async function getSwapBalances(address: string): Promise<{
  etr: { balance: string; balanceRaw: bigint };
  edsc: { balance: string; balanceRaw: bigint };
}> {
  const [etrBalance, edscBalance] = await Promise.all([
    getBalance('flarechain', address),
    getEdscBalance(address),
  ]);

  return {
    etr: etrBalance,
    edsc: edscBalance,
  };
}
