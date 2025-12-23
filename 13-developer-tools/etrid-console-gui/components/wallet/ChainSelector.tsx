'use client';

import { useState } from 'react';
import Image from 'next/image';
import { Check, ChevronDown, Loader2, RefreshCw, AlertCircle } from 'lucide-react';
import { Button } from '@/components/ui/button';
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuLabel,
  DropdownMenuSeparator,
  DropdownMenuTrigger,
} from '@/components/ui/dropdown-menu';
import { useChainSelector, useChainBalance } from '@/hooks/useChainSelector';
import { ChainConfig } from '@/lib/chains/config';
import { cn } from '@/lib/utils';

interface ChainSelectorProps {
  /** Wallet address to show balance for */
  address?: string | null;
  /** Show balance in selector */
  showBalance?: boolean;
  /** Compact mode for mobile */
  compact?: boolean;
}

/**
 * Chain Selector Component
 *
 * Allows users to switch between ËTRID Relay Chain and PBC chains
 * Shows current chain, connection status, and balance
 */
export function ChainSelector({
  address,
  showBalance = true,
  compact = false,
}: ChainSelectorProps) {
  const {
    selectedChain,
    availableChains,
    switchChain,
    isConnected,
    isSwitching,
    error: connectionError,
    reconnect,
  } = useChainSelector();

  const { balance, isLoading: isLoadingBalance } = useChainBalance(address || null);
  const [isOpen, setIsOpen] = useState(false);

  const handleChainSwitch = async (chainId: string) => {
    if (chainId === selectedChain.id) {
      setIsOpen(false);
      return;
    }

    try {
      await switchChain(chainId);
      setIsOpen(false);
    } catch (err) {
      console.error('Failed to switch chain:', err);
    }
  };

  const handleReconnect = async () => {
    try {
      await reconnect();
    } catch (err) {
      console.error('Failed to reconnect:', err);
    }
  };

  /**
   * Get status indicator
   */
  const getStatusIndicator = () => {
    if (isSwitching) {
      return <Loader2 className="w-2 h-2 animate-spin text-yellow-500" />;
    }
    if (connectionError) {
      return <div className="w-2 h-2 rounded-full bg-red-500" />;
    }
    if (isConnected) {
      return <div className="w-2 h-2 rounded-full bg-green-500 animate-pulse" />;
    }
    return <div className="w-2 h-2 rounded-full bg-yellow-500 animate-pulse" />;
  };

  /**
   * Render chain icon using ETR logo
   */
  const renderChainIcon = (chain: ChainConfig, size: 'sm' | 'md' | 'lg' = 'md') => {
    const sizeMap = {
      sm: 24,
      md: 32,
      lg: 40,
    };

    return (
      <Image
        src="/etrid-logo.png"
        alt={chain.token}
        width={sizeMap[size]}
        height={sizeMap[size]}
        className="rounded-full"
      />
    );
  };

  /**
   * Group chains by type
   */
  const relayChain = availableChains.find((c) => c.isRelay);
  const pbcChains = availableChains.filter((c) => !c.isRelay);

  return (
    <div className="flex items-center gap-2">
      {/* Chain Selector Dropdown */}
      <DropdownMenu open={isOpen} onOpenChange={setIsOpen}>
        <DropdownMenuTrigger asChild>
          <Button
            variant="outline"
            className={cn(
              'glass border-white/20 hover:bg-white/10 transition-all',
              compact ? 'px-3' : 'px-4'
            )}
            disabled={isSwitching}
          >
            <div className="flex items-center gap-2">
              {getStatusIndicator()}
              {renderChainIcon(selectedChain, compact ? 'sm' : 'md')}
              {!compact && (
                <div className="flex flex-col items-start">
                  <span className="font-semibold text-sm">{selectedChain.token}</span>
                  {showBalance && balance && !isLoadingBalance && (
                    <span className="text-xs text-white/60">{balance.formatted}</span>
                  )}
                  {showBalance && isLoadingBalance && (
                    <span className="text-xs text-white/60">Loading...</span>
                  )}
                </div>
              )}
              {isSwitching ? (
                <Loader2 className="w-4 h-4 ml-1 animate-spin" />
              ) : (
                <ChevronDown className="w-4 h-4 ml-1" />
              )}
            </div>
          </Button>
        </DropdownMenuTrigger>

        <DropdownMenuContent
          align="end"
          className="glass border-white/20 w-[320px] max-h-[500px] overflow-y-auto"
        >
          {/* Connection Error */}
          {connectionError && (
            <div className="p-3 mb-2 glass-card border border-red-500/30 rounded-lg">
              <div className="flex items-start gap-2">
                <AlertCircle className="w-4 h-4 text-red-500 mt-0.5 flex-shrink-0" />
                <div className="flex-1 min-w-0">
                  <p className="text-xs text-red-400 font-medium">Connection Error</p>
                  <p className="text-xs text-white/60 mt-1 break-words">{connectionError}</p>
                  <Button
                    variant="outline"
                    size="sm"
                    onClick={handleReconnect}
                    className="mt-2 text-xs border-red-500/50 text-red-400 hover:bg-red-500/10"
                  >
                    <RefreshCw className="w-3 h-3 mr-1" />
                    Reconnect
                  </Button>
                </div>
              </div>
            </div>
          )}

          {/* Relay Chain */}
          {relayChain && (
            <>
              <DropdownMenuLabel className="text-white/60 text-xs font-semibold uppercase tracking-wider">
                Relay Chain
              </DropdownMenuLabel>
              <DropdownMenuItem
                onClick={() => handleChainSwitch(relayChain.id)}
                className="cursor-pointer hover:bg-white/10 transition-colors p-3"
              >
                <div className="flex items-center gap-3 flex-1">
                  {renderChainIcon(relayChain, 'md')}
                  <div className="flex-1 min-w-0">
                    <div className="flex items-center gap-2">
                      <span className="font-semibold text-sm">{relayChain.name}</span>
                      {selectedChain.id === relayChain.id && (
                        <Check className="w-4 h-4 text-green-500" />
                      )}
                    </div>
                    <p className="text-xs text-white/60 truncate">{relayChain.description}</p>
                    <p className="text-xs text-white/40 mt-1">Token: {relayChain.token}</p>
                  </div>
                </div>
              </DropdownMenuItem>
              <DropdownMenuSeparator className="bg-white/10" />
            </>
          )}

          {/* PBC Chains */}
          <DropdownMenuLabel className="text-white/60 text-xs font-semibold uppercase tracking-wider">
            Partition Burst Chains (PBCs)
          </DropdownMenuLabel>
          {pbcChains.map((chain) => (
            <DropdownMenuItem
              key={chain.id}
              onClick={() => handleChainSwitch(chain.id)}
              className="cursor-pointer hover:bg-white/10 transition-colors p-3"
            >
              <div className="flex items-center gap-3 flex-1">
                {renderChainIcon(chain, 'md')}
                <div className="flex-1 min-w-0">
                  <div className="flex items-center gap-2">
                    <span className="font-semibold text-sm">{chain.name}</span>
                    {selectedChain.id === chain.id && (
                      <Check className="w-4 h-4 text-green-500" />
                    )}
                  </div>
                  <p className="text-xs text-white/60 truncate">{chain.description}</p>
                  <p className="text-xs text-white/40 mt-1">
                    Token: {chain.token} | Port: {chain.port}
                  </p>
                </div>
              </div>
            </DropdownMenuItem>
          ))}
        </DropdownMenuContent>
      </DropdownMenu>

      {/* Connection Status (Mobile) */}
      {compact && (
        <div className="flex flex-col items-start">
          <span className="text-xs font-semibold">{selectedChain.token}</span>
          {showBalance && balance && !isLoadingBalance && (
            <span className="text-xs text-white/60">{balance.formatted}</span>
          )}
        </div>
      )}
    </div>
  );
}

/**
 * Compact Chain Badge
 * Shows just the chain icon and token symbol
 */
export function ChainBadge({ className }: { className?: string }) {
  const { selectedChain, isConnected } = useChainSelector();

  return (
    <div className={cn('flex items-center gap-2', className)}>
      <Image
        src="/etrid-logo.png"
        alt={selectedChain.token}
        width={24}
        height={24}
        className="rounded-full"
      />
      <span className="text-sm font-semibold">{selectedChain.token}</span>
      {isConnected && <div className="w-1.5 h-1.5 rounded-full bg-green-500 animate-pulse" />}
    </div>
  );
}
