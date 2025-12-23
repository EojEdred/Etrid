'use client';

import { useState, useEffect, useMemo } from 'react';
import Image from 'next/image';
import {
  Dialog,
  DialogContent,
  DialogHeader,
  DialogTitle,
  DialogDescription,
} from '@/components/ui/dialog';
import { Button } from '@/components/ui/button';
import { Input } from '@/components/ui/input';
import { Card } from '@/components/ui/card';
import { Tabs, TabsContent, TabsList, TabsTrigger } from '@/components/ui/tabs';
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuTrigger,
} from '@/components/ui/dropdown-menu';
import {
  ArrowDown,
  ArrowRight,
  ExternalLink,
  RefreshCw,
  AlertCircle,
  ChevronDown,
  TrendingUp,
  Coins,
  Zap,
  Info,
  Loader2,
} from 'lucide-react';
import {
  SWAP_TOKENS,
  ETR_PRICE_USD,
  calculateSwapOutput,
  isPrimeSwapAvailable,
  type SwapToken,
} from '@/lib/polkadot/primeswap';
import { useWallet } from '@/contexts/WalletContext';
import { useToast } from '@/hooks/use-toast';

// External DEX contract addresses
const CONTRACTS = {
  bsc: {
    wETR: '0xcc9b37fed77a01329502f8844620577742eb0dc6',
    pancakeRouter: '0x10ED43C718714eb63d5aA57B78B54704E256024E',
  },
};

interface BuyETRProps {
  open: boolean;
  onOpenChange: (open: boolean) => void;
}

export default function BuyETR({ open, onOpenChange }: BuyETRProps) {
  const { account } = useWallet();
  const { toast } = useToast();

  const [activeTab, setActiveTab] = useState('primeswap');
  const [selectedToken, setSelectedToken] = useState<SwapToken>(SWAP_TOKENS[0]);
  const [inputAmount, setInputAmount] = useState('');
  const [isSwapping, setIsSwapping] = useState(false);
  const [slippage, setSlippage] = useState(0.5);

  const primeSwapAvailable = isPrimeSwapAvailable();

  // Calculate swap output
  const swapOutput = useMemo(() => {
    const amount = parseFloat(inputAmount) || 0;
    return calculateSwapOutput(amount, selectedToken);
  }, [inputAmount, selectedToken]);

  // Handle swap execution
  const handleSwap = async () => {
    if (!account) {
      toast({
        title: 'Wallet Required',
        description: 'Please connect your wallet first',
        variant: 'destructive',
      });
      return;
    }

    if (!primeSwapAvailable) {
      toast({
        title: 'Coming Soon',
        description: 'PrimeSwap contracts are being deployed. Use PancakeSwap for now.',
      });
      return;
    }

    setIsSwapping(true);
    try {
      // Execute swap when contracts are ready
      toast({
        title: 'Swap Submitted',
        description: `Swapping ${inputAmount} ${selectedToken.symbol} for ~${swapOutput.etrAmount.toLocaleString()} ETR`,
      });
    } catch (error) {
      toast({
        title: 'Swap Failed',
        description: error instanceof Error ? error.message : 'Unknown error',
        variant: 'destructive',
      });
    } finally {
      setIsSwapping(false);
    }
  };

  // Quick amount buttons
  const quickAmounts = selectedToken.symbol === 'wBTC'
    ? ['0.001', '0.01', '0.1', '1']
    : selectedToken.symbol === 'wETH'
      ? ['0.1', '0.5', '1', '5']
      : ['10', '100', '1000', '10000'];

  return (
    <Dialog open={open} onOpenChange={onOpenChange}>
      <DialogContent className="glass-card border-white/20 max-w-lg max-h-[90vh] overflow-y-auto">
        <DialogHeader>
          <DialogTitle className="flex items-center gap-2 text-xl gradient-text">
            <Coins className="w-5 h-5" />
            Buy ETR
          </DialogTitle>
          <DialogDescription className="text-white/60">
            Swap tokens for ETR on Primearc Core
          </DialogDescription>
        </DialogHeader>

        <Tabs value={activeTab} onValueChange={setActiveTab} className="mt-4">
          <TabsList className="w-full glass rounded-xl p-1">
            <TabsTrigger
              value="primeswap"
              className="flex-1 rounded-lg data-[state=active]:bg-gradient-to-r data-[state=active]:from-cyan-500/20 data-[state=active]:to-blue-500/20"
            >
              <Zap className="w-4 h-4 mr-2" />
              PrimeSwap
            </TabsTrigger>
            <TabsTrigger
              value="external"
              className="flex-1 rounded-lg data-[state=active]:bg-white/10"
            >
              <ExternalLink className="w-4 h-4 mr-2" />
              External DEX
            </TabsTrigger>
          </TabsList>

          {/* PrimeSwap Native Tab */}
          <TabsContent value="primeswap" className="mt-4 space-y-4">
            {/* Status Banner */}
            <Card className={`p-3 ${primeSwapAvailable ? 'glass border-green-500/30' : 'glass border-amber-500/30'}`}>
              <div className="flex items-center gap-2">
                <div className={`w-2 h-2 rounded-full ${primeSwapAvailable ? 'bg-green-500 animate-pulse' : 'bg-amber-500 animate-pulse'}`} />
                <span className="text-sm">
                  {primeSwapAvailable ? (
                    <span className="text-green-400">PrimeSwap Live on Primearc Core</span>
                  ) : (
                    <span className="text-amber-400">Contracts deploying... UI ready for testing</span>
                  )}
                </span>
              </div>
            </Card>

            {/* Swap Interface */}
            <div className="space-y-3">
              {/* Input Token */}
              <div className="glass rounded-xl p-4 border border-white/10">
                <div className="flex items-center justify-between mb-2">
                  <span className="text-sm text-white/60">You Pay</span>
                  <span className="text-xs text-white/40">
                    Balance: 0.00 {selectedToken.symbol}
                  </span>
                </div>
                <div className="flex items-center gap-3">
                  <Input
                    type="number"
                    value={inputAmount}
                    onChange={(e) => setInputAmount(e.target.value)}
                    placeholder="0.0"
                    className="flex-1 bg-transparent border-none text-2xl font-semibold p-0 focus-visible:ring-0"
                  />
                  <DropdownMenu>
                    <DropdownMenuTrigger asChild>
                      <Button variant="ghost" className="glass px-3 py-2 h-auto">
                        <div className={`w-6 h-6 rounded-full bg-gradient-to-br ${selectedToken.color} flex items-center justify-center mr-2 text-sm`}>
                          {selectedToken.icon}
                        </div>
                        <span className="font-semibold">{selectedToken.symbol}</span>
                        <ChevronDown className="w-4 h-4 ml-1 text-white/60" />
                      </Button>
                    </DropdownMenuTrigger>
                    <DropdownMenuContent className="glass border-white/20 max-h-64 overflow-y-auto">
                      {SWAP_TOKENS.map((token) => (
                        <DropdownMenuItem
                          key={token.symbol}
                          onClick={() => setSelectedToken(token)}
                          className="cursor-pointer"
                        >
                          <div className={`w-6 h-6 rounded-full bg-gradient-to-br ${token.color} flex items-center justify-center mr-2 text-sm`}>
                            {token.icon}
                          </div>
                          <div className="flex-1">
                            <span className="font-medium">{token.symbol}</span>
                            <span className="text-white/40 ml-2 text-xs">{token.name}</span>
                          </div>
                          <span className="text-white/40 text-xs">${token.priceUSD.toLocaleString()}</span>
                        </DropdownMenuItem>
                      ))}
                    </DropdownMenuContent>
                  </DropdownMenu>
                </div>
                {/* Quick amounts */}
                <div className="flex gap-2 mt-3">
                  {quickAmounts.map((amount) => (
                    <button
                      key={amount}
                      onClick={() => setInputAmount(amount)}
                      className="px-2 py-1 text-xs glass rounded-md hover:bg-white/10 transition-colors"
                    >
                      {amount}
                    </button>
                  ))}
                </div>
              </div>

              {/* Arrow */}
              <div className="flex justify-center -my-1">
                <div className="w-10 h-10 rounded-full glass border border-white/20 flex items-center justify-center">
                  <ArrowDown className="w-5 h-5 text-cyan-400" />
                </div>
              </div>

              {/* Output Token (ETR) */}
              <div className="glass rounded-xl p-4 border border-cyan-500/30 bg-cyan-500/5">
                <div className="flex items-center justify-between mb-2">
                  <span className="text-sm text-white/60">You Receive</span>
                  <span className="text-xs text-cyan-400">
                    ~${swapOutput.usdValue.toLocaleString(undefined, { maximumFractionDigits: 2 })}
                  </span>
                </div>
                <div className="flex items-center gap-3">
                  <span className="flex-1 text-2xl font-semibold text-cyan-400">
                    {swapOutput.etrAmount > 0
                      ? swapOutput.etrAmount.toLocaleString(undefined, { maximumFractionDigits: 0 })
                      : '0'}
                  </span>
                  <div className="flex items-center gap-2 px-3 py-2 glass rounded-lg">
                    <Image
                      src="/etrid-logo.png"
                      alt="ETR"
                      width={24}
                      height={24}
                      className="rounded-full"
                    />
                    <span className="font-semibold">ETR</span>
                  </div>
                </div>
              </div>
            </div>

            {/* Swap Details */}
            {parseFloat(inputAmount) > 0 && (
              <Card className="glass border-white/10 p-3 space-y-2 text-sm">
                <div className="flex justify-between">
                  <span className="text-white/60">Rate</span>
                  <span>1 {selectedToken.symbol} = {(selectedToken.priceUSD / ETR_PRICE_USD).toLocaleString()} ETR</span>
                </div>
                <div className="flex justify-between">
                  <span className="text-white/60">Price Impact</span>
                  <span className={swapOutput.priceImpact > 1 ? 'text-amber-400' : 'text-green-400'}>
                    {swapOutput.priceImpact.toFixed(2)}%
                  </span>
                </div>
                <div className="flex justify-between">
                  <span className="text-white/60">Fee (0.3%)</span>
                  <span>{(swapOutput.etrAmount * 0.003).toLocaleString(undefined, { maximumFractionDigits: 0 })} ETR</span>
                </div>
                <div className="flex justify-between">
                  <span className="text-white/60">Slippage</span>
                  <span>{slippage}%</span>
                </div>
              </Card>
            )}

            {/* Swap Button */}
            <Button
              onClick={handleSwap}
              disabled={!parseFloat(inputAmount) || isSwapping || !primeSwapAvailable}
              className="w-full py-6 text-lg font-semibold bg-gradient-to-r from-cyan-500 to-blue-600 hover:from-cyan-600 hover:to-blue-700 disabled:opacity-50"
            >
              {isSwapping ? (
                <>
                  <Loader2 className="w-5 h-5 mr-2 animate-spin" />
                  Swapping...
                </>
              ) : !primeSwapAvailable ? (
                <>
                  <RefreshCw className="w-5 h-5 mr-2" />
                  Awaiting Contract Deployment
                </>
              ) : !parseFloat(inputAmount) ? (
                'Enter Amount'
              ) : (
                <>
                  <Zap className="w-5 h-5 mr-2" />
                  Swap for {swapOutput.etrAmount.toLocaleString(undefined, { maximumFractionDigits: 0 })} ETR
                </>
              )}
            </Button>

            {/* Pool Info */}
            <Card className="glass border-white/10 p-3">
              <div className="flex items-center gap-2 mb-2">
                <Info className="w-4 h-4 text-white/40" />
                <span className="text-sm font-medium">Pool Liquidity</span>
              </div>
              <div className="text-xs text-white/60">
                <p>ETR/{selectedToken.symbol} Pool: {selectedToken.allocation} ETR seeded</p>
                <p className="mt-1">TVL: ~${(parseFloat(selectedToken.allocation.replace(/,/g, '')) * ETR_PRICE_USD).toLocaleString()}</p>
              </div>
            </Card>
          </TabsContent>

          {/* External DEX Tab */}
          <TabsContent value="external" className="mt-4 space-y-4">
            {/* Price Display */}
            <Card className="glass border-white/10 p-4">
              <div className="flex items-center justify-between">
                <div className="flex items-center gap-3">
                  <Image
                    src="/etrid-logo.png"
                    alt="ETR"
                    width={40}
                    height={40}
                    className="rounded-full"
                  />
                  <div>
                    <p className="font-semibold">ETR Token</p>
                    <p className="text-sm text-white/60">Primearc Core</p>
                  </div>
                </div>
                <div className="text-right">
                  <p className="text-xl font-bold text-green-400">${ETR_PRICE_USD.toFixed(4)}</p>
                  <p className="text-xs text-white/40 flex items-center gap-1 justify-end">
                    <TrendingUp className="w-3 h-3 text-green-500" />
                    +2.4% 24h
                  </p>
                </div>
              </div>
            </Card>

            {/* PancakeSwap Option */}
            <a
              href={`https://pancakeswap.finance/swap?outputCurrency=${CONTRACTS.bsc.wETR}`}
              target="_blank"
              rel="noopener noreferrer"
              className="block w-full p-4 glass-card border border-white/10 hover:border-amber-500/50 rounded-xl transition-all"
            >
              <div className="flex items-center justify-between">
                <div className="flex items-center gap-3">
                  <div className="w-10 h-10 rounded-full bg-gradient-to-br from-amber-500 to-yellow-500 flex items-center justify-center">
                    <span className="text-lg">🥞</span>
                  </div>
                  <div>
                    <div className="flex items-center gap-2">
                      <span className="font-semibold">PancakeSwap</span>
                      <span className="px-2 py-0.5 text-xs bg-green-500/20 text-green-400 rounded-full">
                        Live
                      </span>
                    </div>
                    <p className="text-sm text-white/60">BNB Smart Chain</p>
                  </div>
                </div>
                <ExternalLink className="w-4 h-4 text-white/40" />
              </div>
            </a>

            {/* Info Card */}
            <Card className="glass border-blue-500/30 p-4">
              <div className="flex items-start gap-3">
                <AlertCircle className="w-5 h-5 text-blue-400 flex-shrink-0 mt-0.5" />
                <div className="text-sm">
                  <p className="font-medium text-blue-400">How to Buy wETR on BSC</p>
                  <ol className="text-white/60 mt-2 space-y-1 list-decimal list-inside">
                    <li>Connect MetaMask to BNB Smart Chain</li>
                    <li>Swap BNB or USDT for wETR on PancakeSwap</li>
                    <li>Bridge wETR to Primearc Core (coming soon)</li>
                  </ol>
                </div>
              </div>
            </Card>

            {/* Contract Info */}
            <div className="text-center text-xs text-white/40 space-y-1">
              <p>wETR Contract (BSC):</p>
              <code className="font-mono text-white/60 break-all">{CONTRACTS.bsc.wETR}</code>
            </div>
          </TabsContent>
        </Tabs>
      </DialogContent>
    </Dialog>
  );
}
