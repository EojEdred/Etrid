/**
 * LoanOffer - P2P loan listing card
 */

'use client'

import { Star, Clock, Shield } from 'lucide-react'
import { Button } from '@/components/ui/button'
import type { LoanOffer as LoanOfferType } from '@/lib/services/p2p-lending-service'

interface LoanOfferProps {
  offer: LoanOfferType
  onAccept?: (offerId: string) => void
}

export function LoanOffer({ offer, onAccept }: LoanOfferProps) {
  const renderStars = (rating: number) => {
    return Array.from({ length: 5 }).map((_, i) => (
      <Star
        key={i}
        className={`w-3 h-3 ${
          i < rating ? 'fill-warning text-warning' : 'text-muted-foreground'
        }`}
      />
    ))
  }

  return (
    <div className="glass rounded-2xl p-5 space-y-4 hover:bg-accent/5 transition-colors">
      {/* Lender info */}
      <div className="flex items-center justify-between">
        <div className="flex items-center gap-3">
          <div className="w-10 h-10 rounded-full bg-accent/20 flex items-center justify-center">
            <span className="text-lg">👤</span>
          </div>
          <div>
            <p className="font-semibold">{offer.lenderUsername}</p>
            <div className="flex items-center gap-1">
              {renderStars(Math.floor(offer.lenderRating))}
              <span className="text-xs text-muted-foreground ml-1">
                ({offer.lenderRating.toFixed(1)})
              </span>
            </div>
          </div>
        </div>

        <div className="text-right">
          <p className="text-2xl font-bold">{offer.amount}</p>
          <p className="text-xs text-muted-foreground">{offer.asset}</p>
        </div>
      </div>

      {/* Loan terms */}
      <div className="grid grid-cols-3 gap-3 pt-3 border-t border-border">
        <div>
          <div className="flex items-center gap-1 mb-1">
            <Shield className="w-3 h-3 text-muted-foreground" />
            <p className="text-xs text-muted-foreground">APY</p>
          </div>
          <p className="font-semibold text-success">{offer.apy}%</p>
        </div>

        <div>
          <div className="flex items-center gap-1 mb-1">
            <Clock className="w-3 h-3 text-muted-foreground" />
            <p className="text-xs text-muted-foreground">Duration</p>
          </div>
          <p className="font-semibold">{offer.duration} days</p>
        </div>

        <div>
          <p className="text-xs text-muted-foreground mb-1">Collateral</p>
          <p className="font-semibold">{offer.minCollateralRatio}%</p>
        </div>
      </div>

      {/* Interest calculation */}
      <div className="glass-strong rounded-xl p-3">
        <div className="flex items-center justify-between">
          <span className="text-xs text-muted-foreground">Total Interest</span>
          <span className="font-semibold">
            {(offer.amount * (offer.apy / 100) * (offer.duration / 365)).toFixed(2)} {offer.asset}
          </span>
        </div>
      </div>

      {onAccept && (
        <Button
          onClick={() => onAccept(offer.id)}
          className="w-full"
          style={{ background: '#00d9ff', color: '#000' }}
        >
          Accept Offer
        </Button>
      )}
    </div>
  )
}
