"use client"

import { useState, useEffect, useRef } from 'react'
import { Loader2 } from 'lucide-react'
import { cn } from '@/lib/utils'
import { NFTCard } from './nft-card'
import type { NFT } from '@/lib/types/nft'

interface NFTGridProps {
  nfts: NFT[]
  loading?: boolean
  onNFTClick?: (nft: NFT) => void
  onLoadMore?: () => void
  hasMore?: boolean
  emptyMessage?: string
  columns?: 2 | 3
  className?: string
}

export function NFTGrid({
  nfts,
  loading = false,
  onNFTClick,
  onLoadMore,
  hasMore = false,
  emptyMessage = 'No NFTs found',
  columns = 2,
  className,
}: NFTGridProps) {
  const [isRefreshing, setIsRefreshing] = useState(false)
  const observerRef = useRef<HTMLDivElement>(null)

  // Infinite scroll observer
  useEffect(() => {
    if (!onLoadMore || !hasMore || loading) return

    const observer = new IntersectionObserver(
      (entries) => {
        if (entries[0].isIntersecting) {
          onLoadMore()
        }
      },
      { threshold: 0.1 }
    )

    const currentRef = observerRef.current
    if (currentRef) {
      observer.observe(currentRef)
    }

    return () => {
      if (currentRef) {
        observer.unobserve(currentRef)
      }
    }
  }, [onLoadMore, hasMore, loading])

  // Pull to refresh handler
  const handleRefresh = async () => {
    setIsRefreshing(true)
    // Simulate refresh - in real implementation, this would call a refresh function
    await new Promise((resolve) => setTimeout(resolve, 1000))
    setIsRefreshing(false)
  }

  if (loading && nfts.length === 0) {
    return (
      <div className="flex items-center justify-center py-12">
        <Loader2 className="w-8 h-8 animate-spin text-muted-foreground" />
      </div>
    )
  }

  if (nfts.length === 0) {
    return (
      <div className="flex flex-col items-center justify-center py-12 px-6">
        <div className="text-6xl mb-4">🎨</div>
        <p className="text-muted-foreground text-center">{emptyMessage}</p>
      </div>
    )
  }

  return (
    <div className={cn('space-y-4', className)}>
      {/* Grid */}
      <div
        className={cn(
          'grid gap-4',
          columns === 2 ? 'grid-cols-2' : 'grid-cols-3'
        )}
      >
        {nfts.map((nft) => (
          <NFTCard
            key={nft.id}
            nft={nft}
            onClick={() => onNFTClick?.(nft)}
          />
        ))}
      </div>

      {/* Loading More */}
      {loading && nfts.length > 0 && (
        <div className="flex items-center justify-center py-4">
          <Loader2 className="w-6 h-6 animate-spin text-muted-foreground" />
        </div>
      )}

      {/* Infinite Scroll Trigger */}
      {hasMore && <div ref={observerRef} className="h-4" />}
    </div>
  )
}
