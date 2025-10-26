/**
 * TransactionHistory - Transaction History Component
 * Displays past transactions with filtering and search
 */

'use client';

import React, { useState, useEffect } from 'react';
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from '@/components/ui/card';
import { Button } from '@/components/ui/button';
import { Input } from '@/components/ui/input';
import { Badge } from '@/components/ui/badge';
import { Select, SelectContent, SelectItem, SelectTrigger, SelectValue } from '@/components/ui/select';
import { Tabs, TabsContent, TabsList, TabsTrigger } from '@/components/ui/tabs';
import { useWallet } from '@/lib/polkadot/useWallet';
import { CHAINS } from '@/lib/polkadot/chains';
import {
  Send,
  Lock,
  Vote,
  Layers,
  Search,
  ExternalLink,
  Copy,
  CheckCircle2,
  Clock,
  XCircle,
  Filter,
} from 'lucide-react';

interface Transaction {
  hash: string;
  type: 'transfer' | 'staking' | 'governance' | 'channel';
  chainId: string;
  timestamp: number;
  status: 'pending' | 'confirmed' | 'failed';
  blockNumber?: number;
  from: string;
  to?: string;
  amount?: string;
  fee: string;
  details: any;
}

interface TransactionHistoryProps {
  onReuse?: (tx: Transaction) => void;
}

export function TransactionHistory({ onReuse }: TransactionHistoryProps) {
  const [transactions, setTransactions] = useState<Transaction[]>([]);
  const [filteredTransactions, setFilteredTransactions] = useState<Transaction[]>([]);
  const [searchQuery, setSearchQuery] = useState('');
  const [typeFilter, setTypeFilter] = useState<string>('all');
  const [statusFilter, setStatusFilter] = useState<string>('all');
  const [isLoading, setIsLoading] = useState(true);

  const { selectedAccount } = useWallet();

  // Fetch transaction history
  useEffect(() => {
    const fetchHistory = async () => {
      setIsLoading(true);
      try {
        // Mock transaction history data
        const mockTransactions: Transaction[] = [
          {
            hash: '0x1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef',
            type: 'transfer',
            chainId: 'flarechain',
            timestamp: Date.now() - 3600000,
            status: 'confirmed',
            blockNumber: 1234567,
            from: selectedAccount?.address || '',
            to: '5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY',
            amount: '10.5',
            fee: '0.01',
            details: { memo: 'Payment for services' },
          },
          {
            hash: '0xabcdef1234567890abcdef1234567890abcdef1234567890abcdef1234567890',
            type: 'staking',
            chainId: 'flarechain',
            timestamp: Date.now() - 7200000,
            status: 'confirmed',
            blockNumber: 1234550,
            from: selectedAccount?.address || '',
            amount: '100',
            fee: '0.02',
            details: { operation: 'stake', validator: '5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty' },
          },
          {
            hash: '0x9876543210fedcba9876543210fedcba9876543210fedcba9876543210fedcba',
            type: 'governance',
            chainId: 'flarechain',
            timestamp: Date.now() - 14400000,
            status: 'confirmed',
            blockNumber: 1234540,
            from: selectedAccount?.address || '',
            fee: '0.005',
            details: { action: 'vote', proposalId: 1, voteType: 'aye' },
          },
          {
            hash: '0xfedcba9876543210fedcba9876543210fedcba9876543210fedcba9876543210',
            type: 'transfer',
            chainId: 'eth-pbc',
            timestamp: Date.now() - 21600000,
            status: 'pending',
            from: selectedAccount?.address || '',
            to: '5DAAnrj7VHTznn2AWBemMuyBwZWs6FNFjdyVXUeYum3PTXFy',
            amount: '5.25',
            fee: '0.001',
            details: {},
          },
          {
            hash: '0x5555555555555555555555555555555555555555555555555555555555555555',
            type: 'channel',
            chainId: 'flarechain',
            timestamp: Date.now() - 28800000,
            status: 'failed',
            from: selectedAccount?.address || '',
            fee: '0.05',
            details: { operation: 'open', error: 'Insufficient balance' },
          },
        ];

        setTransactions(mockTransactions);
        setFilteredTransactions(mockTransactions);
      } catch (err) {
        console.error('Failed to fetch transaction history:', err);
      } finally {
        setIsLoading(false);
      }
    };

    if (selectedAccount) {
      fetchHistory();
    }
  }, [selectedAccount]);

  // Apply filters
  useEffect(() => {
    let filtered = transactions;

    // Type filter
    if (typeFilter !== 'all') {
      filtered = filtered.filter((tx) => tx.type === typeFilter);
    }

    // Status filter
    if (statusFilter !== 'all') {
      filtered = filtered.filter((tx) => tx.status === statusFilter);
    }

    // Search filter
    if (searchQuery) {
      filtered = filtered.filter(
        (tx) =>
          tx.hash.toLowerCase().includes(searchQuery.toLowerCase()) ||
          tx.to?.toLowerCase().includes(searchQuery.toLowerCase())
      );
    }

    setFilteredTransactions(filtered);
  }, [transactions, typeFilter, statusFilter, searchQuery]);

  // Get transaction type icon
  const getTypeIcon = (type: string) => {
    switch (type) {
      case 'transfer':
        return <Send className="w-4 h-4" />;
      case 'staking':
        return <Lock className="w-4 h-4" />;
      case 'governance':
        return <Vote className="w-4 h-4" />;
      case 'channel':
        return <Layers className="w-4 h-4" />;
      default:
        return null;
    }
  };

  // Get status icon
  const getStatusIcon = (status: string) => {
    switch (status) {
      case 'confirmed':
        return <CheckCircle2 className="w-4 h-4 text-green-600" />;
      case 'pending':
        return <Clock className="w-4 h-4 text-yellow-600" />;
      case 'failed':
        return <XCircle className="w-4 h-4 text-destructive" />;
      default:
        return null;
    }
  };

  // Format timestamp
  const formatTimestamp = (timestamp: number): string => {
    const diff = Date.now() - timestamp;
    const minutes = Math.floor(diff / 60000);
    const hours = Math.floor(diff / 3600000);
    const days = Math.floor(diff / 86400000);

    if (minutes < 60) return `${minutes}m ago`;
    if (hours < 24) return `${hours}h ago`;
    return `${days}d ago`;
  };

  // Copy hash to clipboard
  const copyHash = (hash: string) => {
    navigator.clipboard.writeText(hash);
  };

  return (
    <div className="space-y-6">
      {/* Header */}
      <Card>
        <CardHeader>
          <CardTitle>Transaction History</CardTitle>
          <CardDescription>View and manage your past transactions</CardDescription>
        </CardHeader>
      </Card>

      {/* Filters */}
      <Card>
        <CardContent className="pt-6">
          <div className="grid grid-cols-1 md:grid-cols-3 gap-4">
            {/* Search */}
            <div className="relative">
              <Search className="absolute left-3 top-1/2 -translate-y-1/2 w-4 h-4 text-muted-foreground" />
              <Input
                placeholder="Search by hash or address..."
                value={searchQuery}
                onChange={(e) => setSearchQuery(e.target.value)}
                className="pl-9"
              />
            </div>

            {/* Type Filter */}
            <Select value={typeFilter} onValueChange={setTypeFilter}>
              <SelectTrigger>
                <SelectValue placeholder="All types" />
              </SelectTrigger>
              <SelectContent>
                <SelectItem value="all">All Types</SelectItem>
                <SelectItem value="transfer">Transfer</SelectItem>
                <SelectItem value="staking">Staking</SelectItem>
                <SelectItem value="governance">Governance</SelectItem>
                <SelectItem value="channel">Channel</SelectItem>
              </SelectContent>
            </Select>

            {/* Status Filter */}
            <Select value={statusFilter} onValueChange={setStatusFilter}>
              <SelectTrigger>
                <SelectValue placeholder="All statuses" />
              </SelectTrigger>
              <SelectContent>
                <SelectItem value="all">All Statuses</SelectItem>
                <SelectItem value="confirmed">Confirmed</SelectItem>
                <SelectItem value="pending">Pending</SelectItem>
                <SelectItem value="failed">Failed</SelectItem>
              </SelectContent>
            </Select>
          </div>
        </CardContent>
      </Card>

      {/* Transaction List */}
      <div className="space-y-3">
        {isLoading ? (
          <Card>
            <CardContent className="pt-6">
              <p className="text-center text-muted-foreground">Loading transactions...</p>
            </CardContent>
          </Card>
        ) : filteredTransactions.length === 0 ? (
          <Card>
            <CardContent className="pt-6">
              <p className="text-center text-muted-foreground">No transactions found</p>
            </CardContent>
          </Card>
        ) : (
          filteredTransactions.map((tx) => {
            const chainConfig = CHAINS[tx.chainId as keyof typeof CHAINS];
            return (
              <Card key={tx.hash} className="hover:border-primary/50 transition-colors">
                <CardContent className="pt-6">
                  <div className="flex items-start justify-between gap-4">
                    {/* Left: Icon and Details */}
                    <div className="flex items-start gap-3 flex-1">
                      <div className="p-2 rounded-lg bg-primary/10 text-primary">
                        {getTypeIcon(tx.type)}
                      </div>
                      <div className="flex-1 space-y-2">
                        {/* Type and Chain */}
                        <div className="flex items-center gap-2 flex-wrap">
                          <h4 className="font-medium capitalize">{tx.type}</h4>
                          <Badge variant="secondary" className="text-xs">
                            {chainConfig?.name || tx.chainId}
                          </Badge>
                          <div className="flex items-center gap-1 text-xs text-muted-foreground">
                            {getStatusIcon(tx.status)}
                            <span className="capitalize">{tx.status}</span>
                          </div>
                        </div>

                        {/* Transaction Details */}
                        <div className="space-y-1 text-sm">
                          {tx.to && (
                            <div className="flex items-center gap-2">
                              <span className="text-muted-foreground">To:</span>
                              <span className="font-mono text-xs">
                                {tx.to.slice(0, 10)}...{tx.to.slice(-8)}
                              </span>
                            </div>
                          )}
                          {tx.amount && (
                            <div className="flex items-center gap-2">
                              <span className="text-muted-foreground">Amount:</span>
                              <span className="font-medium">
                                {tx.amount} {chainConfig?.symbol}
                              </span>
                            </div>
                          )}
                          <div className="flex items-center gap-2">
                            <span className="text-muted-foreground">Fee:</span>
                            <span className="font-mono text-xs">
                              {tx.fee} {chainConfig?.symbol}
                            </span>
                          </div>
                          {tx.blockNumber && (
                            <div className="flex items-center gap-2">
                              <span className="text-muted-foreground">Block:</span>
                              <span className="font-mono text-xs">#{tx.blockNumber}</span>
                            </div>
                          )}
                        </div>

                        {/* Hash */}
                        <div className="flex items-center gap-2">
                          <code className="text-xs text-muted-foreground font-mono">
                            {tx.hash.slice(0, 16)}...{tx.hash.slice(-8)}
                          </code>
                          <Button
                            variant="ghost"
                            size="sm"
                            className="h-6 px-2"
                            onClick={() => copyHash(tx.hash)}
                          >
                            <Copy className="w-3 h-3" />
                          </Button>
                          <Button variant="ghost" size="sm" className="h-6 px-2" asChild>
                            <a
                              href={`https://polkadot.js.org/apps/#/explorer/query/${tx.hash}`}
                              target="_blank"
                              rel="noopener noreferrer"
                            >
                              <ExternalLink className="w-3 h-3" />
                            </a>
                          </Button>
                        </div>
                      </div>
                    </div>

                    {/* Right: Timestamp and Actions */}
                    <div className="flex flex-col items-end gap-2">
                      <span className="text-xs text-muted-foreground">
                        {formatTimestamp(tx.timestamp)}
                      </span>
                      {onReuse && tx.status === 'confirmed' && tx.type === 'transfer' && (
                        <Button
                          variant="outline"
                          size="sm"
                          onClick={() => onReuse(tx)}
                        >
                          Reuse
                        </Button>
                      )}
                    </div>
                  </div>
                </CardContent>
              </Card>
            );
          })
        )}
      </div>

      {/* Summary Stats */}
      <Card className="bg-muted/50">
        <CardContent className="pt-6">
          <div className="grid grid-cols-3 gap-4 text-center">
            <div>
              <p className="text-2xl font-bold">
                {transactions.filter((tx) => tx.status === 'confirmed').length}
              </p>
              <p className="text-xs text-muted-foreground">Confirmed</p>
            </div>
            <div>
              <p className="text-2xl font-bold">
                {transactions.filter((tx) => tx.status === 'pending').length}
              </p>
              <p className="text-xs text-muted-foreground">Pending</p>
            </div>
            <div>
              <p className="text-2xl font-bold">
                {transactions.filter((tx) => tx.status === 'failed').length}
              </p>
              <p className="text-xs text-muted-foreground">Failed</p>
            </div>
          </div>
        </CardContent>
      </Card>
    </div>
  );
}
