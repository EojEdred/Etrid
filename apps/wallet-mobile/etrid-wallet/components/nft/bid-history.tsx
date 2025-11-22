"use client"

import { formatDistanceToNow } from 'date-fns'
import { TrendingUp } from 'lucide-react'
import { cn } from '@/lib/utils'
import { Card } from '@/components/ui/card'
import { Badge } from '@/components/ui/badge'
import { ScrollArea } from '@/components/ui/scroll-area'
import type { Bid } from '@/lib/types/nft'

interface BidHistoryProps {
  bids: Bid[]
  currentUserAddress?: string
  currency?: string
  className?: string
}

export function BidHistory({
  bids,
  currentUserAddress,
  currency = 'ETRID',
  className,
}: BidHistoryProps) {
  const sortedBids = [...bids].sort(
    (a, b) => new Date(b.timestamp).getTime() - new Date(a.timestamp).getTime()
  )

  if (bids.length === 0) {
    return (
      <Card className={cn('p-6', className)}>
        <div className="text-center space-y-2">
          <div className="text-4xl">📊</div>
          <p className="text-muted-foreground">No bids yet</p>
          <p className="text-sm text-muted-foreground">
            Be the first to place a bid!
          </p>
        </div>
      </Card>
    )
  }

  return (
    <Card className={cn('p-4', className)}>
      <div className="space-y-4">
        {/* Header */}
        <div className="flex items-center justify-between">
          <h3 className="font-semibold flex items-center gap-2">
            <TrendingUp className="w-4 h-4" />
            Bid History
          </h3>
          <Badge variant="secondary">{bids.length} bids</Badge>
        </div>

        {/* Bid List */}
        <ScrollArea className="h-[300px] pr-4">
          <div className="space-y-3">
            {sortedBids.map((bid, index) => {
              const isCurrentUser = bid.bidder === currentUserAddress
              const isHighestBid = index === 0

              return (
                <div
                  key={bid.id}
                  className={cn(
                    'p-3 rounded-lg border transition-all',
                    isCurrentUser && 'bg-accent/10 border-accent',
                    isHighestBid && !isCurrentUser && 'border-primary/50 bg-primary/5'
                  )}
                >
                  <div className="flex items-start justify-between gap-3">
                    {/* Bidder Info */}
                    <div className="flex-1 min-w-0">
                      <div className="flex items-center gap-2">
                        <span className="text-sm font-medium truncate">
                          {isCurrentUser
                            ? 'You'
                            : `${bid.bidder.slice(0, 6)}...${bid.bidder.slice(-4)}`}
                        </span>
                        {isHighestBid && (
                          <Badge variant="default" className="text-xs">
                            Highest
                          </Badge>
                        )}
                      </div>
                      <p className="text-xs text-muted-foreground">
                        {formatDistanceToNow(new Date(bid.timestamp), {
                          addSuffix: true,
                        })}
                      </p>
                    </div>

                    {/* Bid Amount */}
                    <div className="text-right">
                      <p className="font-bold text-sm">
                        {bid.amount.toLocaleString()} {currency}
                      </p>
                      {bid.tx_hash && (
                        <a
                          href={`https://explorer.etrid.io/tx/${bid.tx_hash}`}
                          target="_blank"
                          rel="noopener noreferrer"
                          className="text-xs text-primary hover:underline"
                          onClick={(e) => e.stopPropagation()}
                        >
                          View TX
                        </a>
                      )}
                    </div>
                  </div>
                </div>
              )
            })}
          </div>
        </ScrollArea>

        {/* Stats */}
        <div className="pt-3 border-t grid grid-cols-2 gap-4">
          <div>
            <p className="text-xs text-muted-foreground">Total Bids</p>
            <p className="font-semibold">{bids.length}</p>
          </div>
          <div>
            <p className="text-xs text-muted-foreground">Average Bid</p>
            <p className="font-semibold">
              {(
                bids.reduce((sum, bid) => sum + bid.amount, 0) / bids.length
              ).toFixed(2)}{' '}
              {currency}
            </p>
          </div>
        </div>
      </div>
    </Card>
  )
}
