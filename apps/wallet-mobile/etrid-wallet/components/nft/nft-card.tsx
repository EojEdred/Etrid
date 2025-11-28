"use client"

import { useState } from 'react'
import Image from 'next/image'
import { Heart } from 'lucide-react'
import { cn } from '@/lib/utils'
import { Card } from '@/components/ui/card'
import { Badge } from '@/components/ui/badge'
import type { NFT } from '@/lib/types/nft'

interface NFTCardProps {
  nft: NFT
  price?: number
  currency?: string
  showChain?: boolean
  onLike?: () => void
  onClick?: () => void
  isLiked?: boolean
  className?: string
}

export function NFTCard({
  nft,
  price,
  currency = 'ETRID',
  showChain = true,
  onLike,
  onClick,
  isLiked = false,
  className,
}: NFTCardProps) {
  const [imageLoaded, setImageLoaded] = useState(false)
  const [imageError, setImageError] = useState(false)

  const handleImageError = () => {
    setImageError(true)
    setImageLoaded(true)
  }

  return (
    <Card
      className={cn(
        'overflow-hidden border-border/50 bg-card hover:bg-accent/5 transition-all cursor-pointer group',
        className
      )}
      onClick={onClick}
    >
      {/* Image Container */}
      <div className="relative aspect-square overflow-hidden bg-muted">
        {!imageError ? (
          <Image
            src={nft.image_url || '/placeholder-nft.png'}
            alt={nft.name}
            fill
            className={cn(
              'object-cover transition-all duration-300 group-hover:scale-105',
              imageLoaded ? 'opacity-100' : 'opacity-0'
            )}
            onLoad={() => setImageLoaded(true)}
            onError={handleImageError}
          />
        ) : (
          <div className="w-full h-full flex items-center justify-center bg-muted">
            <span className="text-4xl">🖼️</span>
          </div>
        )}

        {/* Chain Badge */}
        {showChain && (
          <Badge
            variant="secondary"
            className="absolute top-2 left-2 text-xs bg-background/80 backdrop-blur-sm"
          >
            {nft.chain}
          </Badge>
        )}

        {/* Like Button */}
        {onLike && (
          <button
            onClick={(e) => {
              e.stopPropagation()
              onLike()
            }}
            className="absolute top-2 right-2 p-2 rounded-full bg-background/80 backdrop-blur-sm hover:bg-background transition-all"
          >
            <Heart
              className={cn(
                'w-4 h-4 transition-colors',
                isLiked ? 'fill-red-500 text-red-500' : 'text-foreground'
              )}
            />
          </button>
        )}
      </div>

      {/* Content */}
      <div className="p-3 space-y-2">
        {/* Collection Name */}
        {nft.collection && (
          <p className="text-xs text-muted-foreground truncate">
            {nft.collection.name}
          </p>
        )}

        {/* NFT Name */}
        <h3 className="font-semibold text-sm truncate">{nft.name}</h3>

        {/* Price */}
        {price !== undefined && (
          <div className="flex items-center justify-between">
            <span className="text-xs text-muted-foreground">Price</span>
            <span className="font-bold text-sm">
              {price} {currency}
            </span>
          </div>
        )}
      </div>
    </Card>
  )
}
