/**
 * TokenSelector - Token Selection Component
 * Supports native tokens and ERC-20/custom tokens for multi-chain transactions
 */

'use client';

import React, { useState, useEffect } from 'react';
import { Card, CardContent } from '@/components/ui/card';
import { Label } from '@/components/ui/label';
import { Input } from '@/components/ui/input';
import { Button } from '@/components/ui/button';
import { Badge } from '@/components/ui/badge';
import { Alert, AlertDescription } from '@/components/ui/alert';
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from '@/components/ui/select';
import { ChainId, CHAINS } from '@/lib/polkadot/chains';
import { Coins, Search, Plus, AlertCircle, TrendingUp } from 'lucide-react';

export interface Token {
  address?: string; // Contract address for ERC-20, undefined for native
  symbol: string;
  name: string;
  decimals: number;
  balance?: string;
  isNative: boolean;
  logoUrl?: string;
  priceUsd?: number;
}

export interface TokenSelectorProps {
  chainId: ChainId;
  value: Token | null;
  onChange: (token: Token) => void;
  label?: string;
  showBalance?: boolean;
  allowCustomTokens?: boolean;
}

export function TokenSelector({
  chainId,
  value,
  onChange,
  label = 'Select Token',
  showBalance = true,
  allowCustomTokens = true,
}: TokenSelectorProps) {
  const [tokens, setTokens] = useState<Token[]>([]);
  const [searchQuery, setSearchQuery] = useState('');
  const [isAddingCustom, setIsAddingCustom] = useState(false);
  const [customTokenAddress, setCustomTokenAddress] = useState('');
  const [isLoadingCustom, setIsLoadingCustom] = useState(false);
  const [error, setError] = useState<string | null>(null);

  const chainConfig = CHAINS[chainId];

  // Load default tokens for the chain
  useEffect(() => {
    const loadDefaultTokens = () => {
      // Native token
      const nativeToken: Token = {
        symbol: chainConfig.symbol,
        name: chainConfig.name,
        decimals: chainConfig.decimals,
        isNative: true,
        balance: '0.00', // Would be fetched from wallet
      };

      // Mock ERC-20 tokens based on chain
      const erc20Tokens: Token[] = [];

      if (chainId === 'eth-pbc') {
        erc20Tokens.push(
          {
            address: '0xA0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48',
            symbol: 'USDC',
            name: 'USD Coin',
            decimals: 6,
            isNative: false,
            balance: '1000.00',
            priceUsd: 1.0,
          },
          {
            address: '0xdAC17F958D2ee523a2206206994597C13D831ec7',
            symbol: 'USDT',
            name: 'Tether USD',
            decimals: 6,
            isNative: false,
            balance: '500.00',
            priceUsd: 1.0,
          },
          {
            address: '0x514910771AF9Ca656af840dff83E8264EcF986CA',
            symbol: 'LINK',
            name: 'Chainlink',
            decimals: 18,
            isNative: false,
            balance: '25.50',
            priceUsd: 15.25,
          }
        );
      } else if (chainId === 'bnb-pbc') {
        erc20Tokens.push(
          {
            address: '0xe9e7CEA3DedcA5984780Bafc599bD69ADd087D56',
            symbol: 'BUSD',
            name: 'Binance USD',
            decimals: 18,
            isNative: false,
            balance: '750.00',
            priceUsd: 1.0,
          }
        );
      }

      setTokens([nativeToken, ...erc20Tokens]);

      // Auto-select native token if no value
      if (!value) {
        onChange(nativeToken);
      }
    };

    loadDefaultTokens();
  }, [chainId]);

  // Filter tokens based on search
  const filteredTokens = tokens.filter(
    (token) =>
      token.symbol.toLowerCase().includes(searchQuery.toLowerCase()) ||
      token.name.toLowerCase().includes(searchQuery.toLowerCase()) ||
      token.address?.toLowerCase().includes(searchQuery.toLowerCase())
  );

  // Add custom token
  const handleAddCustomToken = async () => {
    setIsLoadingCustom(true);
    setError(null);

    try {
      // Validate address format
      if (!customTokenAddress.startsWith('0x') || customTokenAddress.length !== 42) {
        throw new Error('Invalid token contract address');
      }

      // In production, this would fetch token details from the blockchain
      // For now, we'll simulate it
      await new Promise((resolve) => setTimeout(resolve, 1500));

      const customToken: Token = {
        address: customTokenAddress,
        symbol: 'CUSTOM',
        name: 'Custom Token',
        decimals: 18,
        isNative: false,
        balance: '0.00',
      };

      setTokens([...tokens, customToken]);
      setIsAddingCustom(false);
      setCustomTokenAddress('');
      onChange(customToken);
    } catch (err) {
      setError(err instanceof Error ? err.message : 'Failed to add custom token');
    } finally {
      setIsLoadingCustom(false);
    }
  };

  const selectedToken = value || tokens[0];

  return (
    <div className="space-y-3">
      <Label className="flex items-center gap-2">
        <Coins className="w-4 h-4" />
        {label}
      </Label>

      {/* Token Selection */}
      <Select
        value={selectedToken?.symbol}
        onValueChange={(symbol) => {
          const token = tokens.find((t) => t.symbol === symbol);
          if (token) onChange(token);
        }}
      >
        <SelectTrigger>
          <SelectValue>
            <div className="flex items-center gap-2">
              <div className="w-6 h-6 rounded-full bg-gradient-to-br from-primary/20 to-primary/40 flex items-center justify-center text-xs font-bold">
                {selectedToken?.symbol.charAt(0)}
              </div>
              <div className="flex-1 text-left">
                <div className="font-medium">{selectedToken?.symbol}</div>
                <div className="text-xs text-muted-foreground">{selectedToken?.name}</div>
              </div>
              {showBalance && selectedToken?.balance && (
                <Badge variant="secondary" className="font-mono">
                  {selectedToken.balance}
                </Badge>
              )}
            </div>
          </SelectValue>
        </SelectTrigger>
        <SelectContent>
          {/* Search */}
          <div className="p-2 border-b">
            <div className="relative">
              <Search className="absolute left-2 top-1/2 -translate-y-1/2 w-4 h-4 text-muted-foreground" />
              <Input
                placeholder="Search tokens..."
                value={searchQuery}
                onChange={(e) => setSearchQuery(e.target.value)}
                className="pl-8 h-8"
              />
            </div>
          </div>

          {/* Token List */}
          <div className="max-h-[300px] overflow-y-auto">
            {filteredTokens.map((token) => (
              <SelectItem key={token.address || token.symbol} value={token.symbol}>
                <div className="flex items-center gap-3 w-full py-1">
                  <div className="w-8 h-8 rounded-full bg-gradient-to-br from-primary/20 to-primary/40 flex items-center justify-center text-sm font-bold">
                    {token.symbol.charAt(0)}
                  </div>
                  <div className="flex-1">
                    <div className="flex items-center gap-2">
                      <span className="font-medium">{token.symbol}</span>
                      {token.isNative && (
                        <Badge variant="outline" className="text-xs">
                          Native
                        </Badge>
                      )}
                    </div>
                    <div className="text-xs text-muted-foreground">{token.name}</div>
                  </div>
                  <div className="text-right">
                    {showBalance && token.balance && (
                      <div className="font-mono text-sm">{token.balance}</div>
                    )}
                    {token.priceUsd && (
                      <div className="text-xs text-muted-foreground">
                        ${token.priceUsd.toFixed(2)}
                      </div>
                    )}
                  </div>
                </div>
              </SelectItem>
            ))}

            {filteredTokens.length === 0 && (
              <div className="p-4 text-center text-sm text-muted-foreground">
                No tokens found
              </div>
            )}
          </div>

          {/* Add Custom Token Button */}
          {allowCustomTokens && (
            <div className="p-2 border-t">
              <Button
                variant="ghost"
                size="sm"
                className="w-full justify-start gap-2"
                onClick={() => setIsAddingCustom(true)}
              >
                <Plus className="w-4 h-4" />
                Add Custom Token
              </Button>
            </div>
          )}
        </SelectContent>
      </Select>

      {/* Token Details Card */}
      {selectedToken && (
        <Card className="bg-muted/30">
          <CardContent className="pt-4 pb-4">
            <div className="space-y-2 text-sm">
              <div className="flex justify-between">
                <span className="text-muted-foreground">Token Type</span>
                <Badge variant="secondary">
                  {selectedToken.isNative ? 'Native' : 'ERC-20'}
                </Badge>
              </div>
              {!selectedToken.isNative && selectedToken.address && (
                <div className="flex justify-between">
                  <span className="text-muted-foreground">Contract</span>
                  <code className="text-xs font-mono">
                    {selectedToken.address.slice(0, 6)}...{selectedToken.address.slice(-4)}
                  </code>
                </div>
              )}
              <div className="flex justify-between">
                <span className="text-muted-foreground">Decimals</span>
                <span className="font-medium">{selectedToken.decimals}</span>
              </div>
              {selectedToken.priceUsd && (
                <div className="flex justify-between items-center pt-2 border-t">
                  <span className="text-muted-foreground">Price (USD)</span>
                  <div className="flex items-center gap-2">
                    <TrendingUp className="w-3 h-3 text-green-600" />
                    <span className="font-medium">${selectedToken.priceUsd.toFixed(2)}</span>
                  </div>
                </div>
              )}
            </div>
          </CardContent>
        </Card>
      )}

      {/* Add Custom Token Modal/Dialog */}
      {isAddingCustom && (
        <Card className="border-primary">
          <CardContent className="pt-4 space-y-3">
            <div className="flex items-center justify-between">
              <h4 className="font-medium">Add Custom Token</h4>
              <Button
                variant="ghost"
                size="sm"
                onClick={() => {
                  setIsAddingCustom(false);
                  setCustomTokenAddress('');
                  setError(null);
                }}
              >
                Cancel
              </Button>
            </div>

            <div className="space-y-2">
              <Label htmlFor="token-address">Token Contract Address</Label>
              <Input
                id="token-address"
                placeholder="0x..."
                value={customTokenAddress}
                onChange={(e) => setCustomTokenAddress(e.target.value)}
                disabled={isLoadingCustom}
              />
              <p className="text-xs text-muted-foreground">
                Enter the ERC-20 token contract address on {chainConfig.name}
              </p>
            </div>

            {error && (
              <Alert variant="destructive">
                <AlertCircle className="h-4 w-4" />
                <AlertDescription className="text-xs">{error}</AlertDescription>
              </Alert>
            )}

            <Button
              className="w-full"
              onClick={handleAddCustomToken}
              disabled={!customTokenAddress || isLoadingCustom}
            >
              {isLoadingCustom ? 'Loading...' : 'Add Token'}
            </Button>
          </CardContent>
        </Card>
      )}
    </div>
  );
}
