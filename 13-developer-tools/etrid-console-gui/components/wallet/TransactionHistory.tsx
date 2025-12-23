'use client';

import { useState } from 'react';
import {
  ArrowUpRight,
  ArrowDownLeft,
  CheckCircle,
  XCircle,
  ExternalLink,
  RefreshCw,
  Clock,
  Hash,
} from 'lucide-react';
import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card';
import { Button } from '@/components/ui/button';
import { useTransactionHistory } from '@/hooks/useTransactionHistory';
import {
  formatTransactionValue,
  formatTimeAgo,
  truncateHash,
} from '@/lib/polkadot/transactions';
import { explorerAccount, explorerHome, explorerTx } from '@/lib/explorer';

interface TransactionHistoryProps {
  address: string | null;
  limit?: number;
}

export default function TransactionHistory({ address, limit = 50 }: TransactionHistoryProps) {
  const { transactions, isLoading, error, refetch } = useTransactionHistory(address, {
    limit,
    subscribe: true,
  });

  const [isRefreshing, setIsRefreshing] = useState(false);

  const handleRefresh = async () => {
    setIsRefreshing(true);
    await refetch();
    setTimeout(() => setIsRefreshing(false), 500);
  };

  const getExplorerUrl = (hash: string) => explorerTx(hash);

  const formatAddress = (addr: string | null) => {
    if (!addr) return 'N/A';
    return `${addr.slice(0, 6)}...${addr.slice(-4)}`;
  };

  return (
    <Card className="glass-card border-0">
      <CardHeader>
        <CardTitle className="flex items-center justify-between">
          <div className="flex items-center gap-2">
            <Clock className="w-5 h-5 text-[#66D9E6]" />
            <span>Transaction History</span>
          </div>
          <Button
            variant="ghost"
            size="sm"
            onClick={handleRefresh}
            disabled={isLoading || isRefreshing}
            className="text-white/60 hover:text-white"
          >
            <RefreshCw className={`w-4 h-4 mr-2 ${isLoading || isRefreshing ? 'animate-spin' : ''}`} />
            Refresh
          </Button>
        </CardTitle>
      </CardHeader>
      <CardContent>
        {/* Error State */}
        {error && (
          <div className="flex items-center justify-center py-12 text-red-400">
            <XCircle className="w-5 h-5 mr-2" />
            <span>{error}</span>
          </div>
        )}

        {/* Loading State */}
        {isLoading && transactions.length === 0 && !error && (
          <div className="space-y-4">
            {[...Array(5)].map((_, i) => (
              <div key={i} className="flex items-center gap-4 p-4 glass rounded-xl animate-pulse">
                <div className="w-10 h-10 rounded-full bg-white/10" />
                <div className="flex-1 space-y-2">
                  <div className="w-32 h-4 bg-white/10 rounded" />
                  <div className="w-48 h-3 bg-white/10 rounded" />
                </div>
                <div className="text-right space-y-2">
                  <div className="w-24 h-4 bg-white/10 rounded ml-auto" />
                  <div className="w-16 h-3 bg-white/10 rounded ml-auto" />
                </div>
              </div>
            ))}
          </div>
        )}

        {/* Empty State */}
        {!isLoading && !error && transactions.length === 0 && (
          <div className="flex flex-col items-center justify-center py-12 text-white/60">
            <div className="w-16 h-16 rounded-full bg-white/5 flex items-center justify-center mb-4">
              <Clock className="w-8 h-8 text-white/40" />
            </div>
            <p className="text-lg font-medium mb-2">No transactions yet</p>
            <p className="text-sm text-white/40">Your transaction history will appear here</p>
          </div>
        )}

        {/* Transaction List */}
        {!error && transactions.length > 0 && (
          <div className="space-y-3">
            {transactions.map((tx) => {
              const isSent = tx.from.toLowerCase() === address?.toLowerCase();
              const isReceived = tx.to?.toLowerCase() === address?.toLowerCase();
              const isSuccess = tx.status === 'success';

              return (
                <div
                  key={tx.hash}
                  className="flex items-center gap-4 p-4 glass rounded-xl hover:bg-white/10 transition-colors cursor-pointer"
                  onClick={() => window.open(getExplorerUrl(tx.hash), '_blank')}
                >
                  {/* Icon */}
                  <div
                    className={`w-10 h-10 rounded-full flex items-center justify-center ${
                      isSuccess
                        ? isSent
                          ? 'bg-orange-500/20'
                          : 'bg-green-500/20'
                        : 'bg-red-500/20'
                    }`}
                  >
                    {isSuccess ? (
                      isSent ? (
                        <ArrowUpRight className="w-5 h-5 text-orange-400" />
                      ) : (
                        <ArrowDownLeft className="w-5 h-5 text-green-400" />
                      )
                    ) : (
                      <XCircle className="w-5 h-5 text-red-400" />
                    )}
                  </div>

                  {/* Transaction Details */}
                  <div className="flex-1 min-w-0">
                    <div className="flex items-center gap-2 mb-1">
                      <span className="font-semibold">
                        {isSent ? 'Sent' : 'Received'}
                        {' '}
                        {tx.section === 'balances' && (tx.method === 'transfer' || tx.method === 'transferKeepAlive')
                          ? 'ETR'
                          : `${tx.section}.${tx.method}`}
                      </span>
                      {isSuccess ? (
                        <CheckCircle className="w-4 h-4 text-green-400" />
                      ) : (
                        <XCircle className="w-4 h-4 text-red-400" />
                      )}
                    </div>
                    <div className="flex items-center gap-2 text-sm text-white/60">
                      <Hash className="w-3 h-3" />
                      <span className="font-mono">{truncateHash(tx.hash, 8, 6)}</span>
                      <span>•</span>
                      {isSent && tx.to && (
                        <>
                          <span>To: {formatAddress(tx.to)}</span>
                          <span>•</span>
                        </>
                      )}
                      {isReceived && (
                        <>
                          <span>From: {formatAddress(tx.from)}</span>
                          <span>•</span>
                        </>
                      )}
                      <span>Block #{tx.blockNumber.toLocaleString()}</span>
                    </div>
                  </div>

                  {/* Amount and Time */}
                  <div className="text-right">
                    <div className={`font-semibold mb-1 ${
                      isSent ? 'text-orange-400' : 'text-green-400'
                    }`}>
                      {isSent ? '- ' : '+ '}
                      {tx.value !== '0' ? formatTransactionValue(tx.value) : '—'}
                    </div>
                    <div className="text-sm text-white/60">
                      {formatTimeAgo(tx.timestamp)}
                    </div>
                    {tx.fee !== '0' && (
                      <div className="text-xs text-white/40 mt-1">
                        Fee: {formatTransactionValue(tx.fee)}
                      </div>
                    )}
                  </div>

                  {/* External Link Icon */}
                  <ExternalLink className="w-4 h-4 text-white/40 hover:text-white/80 transition-colors flex-shrink-0" />
                </div>
              );
            })}
          </div>
        )}

        {/* View More Link */}
        {!isLoading && !error && transactions.length > 0 && (
          <div className="mt-6 text-center">
            <a
              href={address ? explorerAccount(address) : explorerHome()}
              target="_blank"
              rel="noopener noreferrer"
              className="text-[#66D9E6] hover:text-[#4DB3CC] transition-colors inline-flex items-center gap-2"
            >
              View all transactions on Explorer
              <ExternalLink className="w-4 h-4" />
            </a>
          </div>
        )}
      </CardContent>
    </Card>
  );
}
