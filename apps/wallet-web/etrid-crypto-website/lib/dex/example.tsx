'use client';

/**
 * Example DEX Swap Component
 *
 * This is a complete example showing how to use the DEX service
 * to build a token swap interface.
 */

import { useState } from 'react';
import { useSwapUI, useSupportedChains, useWETRAddresses } from './hooks';
import { useSigner } from 'wagmi';

export function DexSwapExample() {
  const [selectedChain, setSelectedChain] = useState('ethereum');
  const { data: signer } = useSigner();
  const supportedChains = useSupportedChains();
  const wETRAddresses = useWETRAddresses();

  const {
    // Token addresses
    fromToken,
    toToken,
    amount,
    slippage,
    // Setters
    setFromToken,
    setToToken,
    setAmount,
    setSlippage,
    // Quote data
    quote,
    isLoadingQuote,
    quoteError,
    refetchQuote,
    // Swap execution
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
  } = useSwapUI(selectedChain);

  const handleSwap = async () => {
    if (!signer) {
      alert('Please connect your wallet');
      return;
    }

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

      console.log('Swap successful!', result);
      alert(`Swap completed! Tx: ${result.txHash}`);

      // Reset form
      setAmount('');
    } catch (error) {
      console.error('Swap failed:', error);
      alert(`Swap failed: ${error instanceof Error ? error.message : 'Unknown error'}`);
    }
  };

  return (
    <div className="max-w-md mx-auto p-6 bg-white rounded-lg shadow-lg">
      <h2 className="text-2xl font-bold mb-4">Swap wETR</h2>

      {/* Chain Selector */}
      <div className="mb-4">
        <label className="block text-sm font-medium mb-2">Select Chain</label>
        <select
          value={selectedChain}
          onChange={(e) => setSelectedChain(e.target.value)}
          className="w-full p-2 border rounded"
        >
          {Object.entries(supportedChains).map(([key, config]) => (
            <option key={key} value={key}>
              {config.chainName}
            </option>
          ))}
        </select>
      </div>

      {/* wETR Address Display */}
      <div className="mb-4 p-3 bg-gray-100 rounded">
        <p className="text-xs text-gray-600">wETR Address on {supportedChains[selectedChain]?.chainName}:</p>
        <p className="text-xs font-mono break-all">{wETRAddress}</p>
      </div>

      {/* From Token */}
      <div className="mb-4">
        <label className="block text-sm font-medium mb-2">From</label>
        <input
          type="text"
          value={fromToken}
          onChange={(e) => setFromToken(e.target.value)}
          placeholder="Token address (or use wETR)"
          className="w-full p-2 border rounded mb-2"
        />
        <button
          onClick={() => setFromToken(wETRAddress || '')}
          className="text-sm text-blue-600 hover:underline"
        >
          Use wETR
        </button>
        {fromBalance && (
          <p className="text-sm text-gray-600 mt-1">
            Balance: {fromBalance}
          </p>
        )}
      </div>

      {/* Amount */}
      <div className="mb-4">
        <label className="block text-sm font-medium mb-2">Amount</label>
        <input
          type="text"
          value={amount}
          onChange={(e) => setAmount(e.target.value)}
          placeholder="0.0"
          className="w-full p-2 border rounded"
        />
        {hasInsufficientBalance && (
          <p className="text-sm text-red-600 mt-1">Insufficient balance</p>
        )}
      </div>

      {/* To Token */}
      <div className="mb-4">
        <label className="block text-sm font-medium mb-2">To</label>
        <input
          type="text"
          value={toToken}
          onChange={(e) => setToToken(e.target.value)}
          placeholder="Token address"
          className="w-full p-2 border rounded"
        />
        {toBalance && (
          <p className="text-sm text-gray-600 mt-1">
            Balance: {toBalance}
          </p>
        )}
      </div>

      {/* Slippage Settings */}
      <div className="mb-4">
        <label className="block text-sm font-medium mb-2">
          Slippage Tolerance: {slippage}%
        </label>
        <div className="flex gap-2">
          <button
            onClick={() => setSlippage(0.1)}
            className={`px-3 py-1 rounded ${slippage === 0.1 ? 'bg-blue-600 text-white' : 'bg-gray-200'}`}
          >
            0.1%
          </button>
          <button
            onClick={() => setSlippage(0.5)}
            className={`px-3 py-1 rounded ${slippage === 0.5 ? 'bg-blue-600 text-white' : 'bg-gray-200'}`}
          >
            0.5%
          </button>
          <button
            onClick={() => setSlippage(1.0)}
            className={`px-3 py-1 rounded ${slippage === 1.0 ? 'bg-blue-600 text-white' : 'bg-gray-200'}`}
          >
            1.0%
          </button>
        </div>
      </div>

      {/* Quote Display */}
      {isLoadingQuote && (
        <div className="mb-4 p-3 bg-blue-50 rounded">
          <p className="text-sm text-blue-600">Fetching quote...</p>
        </div>
      )}

      {quote && (
        <div className="mb-4 p-3 bg-gray-50 rounded">
          <h3 className="font-medium mb-2">Quote</h3>
          <div className="space-y-1 text-sm">
            <p>Amount Out: <span className="font-mono">{quote.amountOut}</span></p>
            <p>Price Impact: <span className={`font-medium ${quote.priceImpact > 5 ? 'text-red-600' : 'text-green-600'}`}>
              {quote.priceImpact.toFixed(2)}%
            </span></p>
            <p>Minimum Received: <span className="font-mono">{quote.minimumReceived}</span></p>
          </div>
        </div>
      )}

      {quoteError && (
        <div className="mb-4 p-3 bg-red-50 rounded">
          <p className="text-sm text-red-600">{quoteError.message}</p>
        </div>
      )}

      {/* Price Impact Warning */}
      {priceImpactWarning && (
        <div className={`mb-4 p-3 rounded ${
          priceImpactWarning.level === 'error' ? 'bg-red-50' :
          priceImpactWarning.level === 'warning' ? 'bg-yellow-50' :
          'bg-blue-50'
        }`}>
          <p className={`text-sm ${
            priceImpactWarning.level === 'error' ? 'text-red-600' :
            priceImpactWarning.level === 'warning' ? 'text-yellow-600' :
            'text-blue-600'
          }`}>
            {priceImpactWarning.message}
          </p>
        </div>
      )}

      {/* Swap Button */}
      <button
        onClick={handleSwap}
        disabled={
          !signer ||
          !fromToken ||
          !toToken ||
          !amount ||
          isSwapping ||
          isLoadingQuote ||
          hasInsufficientBalance ||
          !!quoteError
        }
        className={`w-full py-3 rounded font-medium ${
          !signer || !fromToken || !toToken || !amount || isSwapping || hasInsufficientBalance
            ? 'bg-gray-300 cursor-not-allowed'
            : 'bg-blue-600 hover:bg-blue-700 text-white'
        }`}
      >
        {!signer
          ? 'Connect Wallet'
          : isSwapping
          ? 'Swapping...'
          : hasInsufficientBalance
          ? 'Insufficient Balance'
          : 'Swap'}
      </button>

      {/* Transaction Status */}
      {txHash && (
        <div className="mt-4 p-3 bg-green-50 rounded">
          <p className="text-sm text-green-600 mb-1">Transaction Submitted!</p>
          <a
            href={`https://etherscan.io/tx/${txHash}`}
            target="_blank"
            rel="noopener noreferrer"
            className="text-xs text-blue-600 hover:underline break-all"
          >
            {txHash}
          </a>
        </div>
      )}

      {swapError && (
        <div className="mt-4 p-3 bg-red-50 rounded">
          <p className="text-sm text-red-600">{swapError.message}</p>
        </div>
      )}

      {/* All wETR Addresses */}
      <div className="mt-6 pt-6 border-t">
        <h3 className="text-sm font-medium mb-2">wETR Addresses on All Chains:</h3>
        <div className="space-y-2">
          {Object.entries(wETRAddresses).map(([chain, address]) => (
            <div key={chain} className="text-xs">
              <span className="font-medium capitalize">{chain}:</span>{' '}
              <span className="font-mono text-gray-600">{address}</span>
            </div>
          ))}
        </div>
      </div>
    </div>
  );
}

/**
 * Example: Using the service directly (without hooks)
 */
export async function directServiceExample() {
  const { DexService } = await import('./service');
  const { ethers } = await import('ethers');

  // Initialize service
  const dexService = new DexService('ethereum');

  // Get wETR address
  const wETRAddress = dexService.getWETRAddress();
  console.log('wETR Address:', wETRAddress);

  // Get quote
  const quote = await dexService.getSwapQuote({
    fromToken: wETRAddress,
    toToken: '0xC02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2', // WETH
    amount: '1000000000000000000', // 1 wETR
    slippage: 0.5,
  });

  console.log('Quote:', quote);

  // Execute swap (requires signer)
  // const provider = new ethers.BrowserProvider(window.ethereum);
  // const signer = await provider.getSigner();
  // const tx = await dexService.executeSwap(signer, {
  //   fromToken: wETRAddress,
  //   toToken: '0xC02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2',
  //   amount: '1000000000000000000',
  //   slippage: 0.5,
  // });
  // await tx.wait();
}
