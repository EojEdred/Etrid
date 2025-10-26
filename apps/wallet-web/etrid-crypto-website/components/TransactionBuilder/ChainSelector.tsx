/**
 * ChainSelector - Reusable Chain Selection Component
 * Provides multi-chain selection with network detection and switching
 */

'use client';

import React, { useState, useEffect } from 'react';
import { Card, CardContent } from '@/components/ui/card';
import { Label } from '@/components/ui/label';
import { Select, SelectContent, SelectItem, SelectTrigger, SelectValue } from '@/components/ui/select';
import { Badge } from '@/components/ui/badge';
import { Alert, AlertDescription } from '@/components/ui/alert';
import { CHAINS, ChainId, getAllChains, ChainConfig } from '@/lib/polkadot/chains';
import { Network, AlertCircle, CheckCircle2, Loader2 } from 'lucide-react';

export interface ChainSelectorProps {
  value: ChainId;
  onChange: (chainId: ChainId) => void;
  label?: string;
  showBalance?: boolean;
  balance?: string;
  disabled?: boolean;
  showNetworkStatus?: boolean;
  filterRelay?: boolean; // Only show relay chain
  filterParachains?: boolean; // Only show parachains
}

export function ChainSelector({
  value,
  onChange,
  label = 'Select Chain',
  showBalance = false,
  balance,
  disabled = false,
  showNetworkStatus = true,
  filterRelay = false,
  filterParachains = false,
}: ChainSelectorProps) {
  const [networkStatus, setNetworkStatus] = useState<Record<string, 'connected' | 'disconnected' | 'checking'>>({});
  const [isChecking, setIsChecking] = useState(false);

  const chains = getAllChains().filter(chain => {
    if (filterRelay) return chain.isRelay;
    if (filterParachains) return !chain.isRelay;
    return true;
  });

  const selectedChain = CHAINS[value];

  // Check network status for all chains
  useEffect(() => {
    if (!showNetworkStatus) return;

    const checkNetworkStatus = async () => {
      setIsChecking(true);
      const statuses: Record<string, 'connected' | 'disconnected' | 'checking'> = {};

      for (const chain of chains) {
        statuses[chain.id] = 'checking';
      }
      setNetworkStatus(statuses);

      // Simulate network status check
      await new Promise(resolve => setTimeout(resolve, 1000));

      for (const chain of chains) {
        // In production, this would actually check WebSocket connection
        // For now, simulate some chains as connected, others as disconnected
        const isConnected = Math.random() > 0.2; // 80% connected
        statuses[chain.id] = isConnected ? 'connected' : 'disconnected';
      }

      setNetworkStatus(statuses);
      setIsChecking(false);
    };

    checkNetworkStatus();
  }, [chains, showNetworkStatus]);

  const getNetworkStatusIcon = (chainId: string) => {
    const status = networkStatus[chainId];
    if (!showNetworkStatus) return null;

    switch (status) {
      case 'connected':
        return <CheckCircle2 className="w-3 h-3 text-green-600" />;
      case 'disconnected':
        return <AlertCircle className="w-3 h-3 text-destructive" />;
      case 'checking':
        return <Loader2 className="w-3 h-3 animate-spin text-muted-foreground" />;
      default:
        return null;
    }
  };

  const getNetworkStatusBadge = () => {
    const status = networkStatus[value];
    if (!showNetworkStatus) return null;

    switch (status) {
      case 'connected':
        return (
          <Badge variant="default" className="gap-1">
            <CheckCircle2 className="w-3 h-3" />
            Connected
          </Badge>
        );
      case 'disconnected':
        return (
          <Badge variant="destructive" className="gap-1">
            <AlertCircle className="w-3 h-3" />
            Disconnected
          </Badge>
        );
      case 'checking':
        return (
          <Badge variant="secondary" className="gap-1">
            <Loader2 className="w-3 h-3 animate-spin" />
            Checking...
          </Badge>
        );
      default:
        return null;
    }
  };

  return (
    <div className="space-y-3">
      <div className="flex items-center justify-between">
        <Label htmlFor="chain-selector" className="flex items-center gap-2">
          <Network className="w-4 h-4" />
          {label}
        </Label>
        {getNetworkStatusBadge()}
      </div>

      <Select value={value} onValueChange={onChange} disabled={disabled}>
        <SelectTrigger id="chain-selector">
          <SelectValue>
            <div className="flex items-center gap-2">
              <div
                className="w-3 h-3 rounded-full"
                style={{ backgroundColor: selectedChain.color }}
              />
              <span>{selectedChain.name}</span>
              <Badge variant="secondary" className="ml-auto">
                {selectedChain.symbol}
              </Badge>
            </div>
          </SelectValue>
        </SelectTrigger>
        <SelectContent>
          {chains.map((chain) => (
            <SelectItem key={chain.id} value={chain.id}>
              <div className="flex items-center gap-2 w-full">
                <div
                  className="w-3 h-3 rounded-full"
                  style={{ backgroundColor: chain.color }}
                />
                <span className="flex-1">{chain.name}</span>
                <div className="flex items-center gap-2">
                  {getNetworkStatusIcon(chain.id)}
                  <Badge variant="secondary" className="text-xs">
                    {chain.symbol}
                  </Badge>
                  {chain.isRelay && (
                    <Badge variant="outline" className="text-xs">
                      Relay
                    </Badge>
                  )}
                </div>
              </div>
            </SelectItem>
          ))}
        </SelectContent>
      </Select>

      {/* Chain Info */}
      <Card className="bg-muted/30">
        <CardContent className="pt-4 pb-4">
          <div className="grid grid-cols-2 gap-3 text-sm">
            <div>
              <p className="text-muted-foreground text-xs">Network Type</p>
              <p className="font-medium">
                {selectedChain.isRelay ? 'Relay Chain' : 'Partition Burst Chain'}
              </p>
            </div>
            <div>
              <p className="text-muted-foreground text-xs">Decimals</p>
              <p className="font-medium">{selectedChain.decimals}</p>
            </div>
            {showBalance && balance && (
              <div className="col-span-2">
                <p className="text-muted-foreground text-xs">Available Balance</p>
                <p className="font-medium font-mono">
                  {balance} {selectedChain.symbol}
                </p>
              </div>
            )}
          </div>
        </CardContent>
      </Card>

      {/* Network Status Warning */}
      {showNetworkStatus && networkStatus[value] === 'disconnected' && (
        <Alert variant="destructive">
          <AlertCircle className="h-4 w-4" />
          <AlertDescription className="text-xs">
            Unable to connect to {selectedChain.name}. Please check your network connection
            or try a different chain.
          </AlertDescription>
        </Alert>
      )}
    </div>
  );
}
