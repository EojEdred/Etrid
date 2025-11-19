"use client"

import { TrendingUp } from "lucide-react"
import type { CryptoAsset } from "@/lib/types/features"

interface CollateralAssetItemProps {
  asset: CryptoAsset
  amount: number
  valueUsd: number
  percentage: number
  onClick?: () => void
  className?: string
}

export function CollateralAssetItem({
  asset,
  amount,
  valueUsd,
  percentage,
  onClick,
  className = "",
}: CollateralAssetItemProps) {
  const getAssetColor = (asset: CryptoAsset) => {
    const colors: Record<CryptoAsset, string> = {
      ETR: "bg-purple-500",
      BTC: "bg-orange-500",
      ETH: "bg-blue-500",
      USDT: "bg-green-500",
      USDC: "bg-blue-600",
    }
    return colors[asset] || "bg-accent"
  }

  const getAssetName = (asset: CryptoAsset) => {
    const names: Record<CryptoAsset, string> = {
      ETR: "Ëtrid",
      BTC: "Bitcoin",
      ETH: "Ethereum",
      USDT: "Tether",
      USDC: "USD Coin",
    }
    return names[asset] || asset
  }

  return (
    <div
      onClick={onClick}
      className={`glass p-4 rounded-2xl ${onClick ? "cursor-pointer hover:glass-strong" : ""} ${className}`}
    >
      <div className="flex items-center gap-4">
        {/* Asset Icon */}
        <div className={`w-12 h-12 rounded-full ${getAssetColor(asset)} flex items-center justify-center shrink-0`}>
          <span className="text-white font-bold text-sm">{asset}</span>
        </div>

        {/* Asset Info */}
        <div className="flex-1 min-w-0">
          <div className="flex items-baseline gap-2 mb-1">
            <p className="font-semibold">{getAssetName(asset)}</p>
            <span className="text-xs text-muted-foreground">{percentage.toFixed(1)}%</span>
          </div>
          <p className="text-sm text-muted-foreground">
            {amount.toLocaleString(undefined, { maximumFractionDigits: 8 })} {asset}
          </p>
        </div>

        {/* Value */}
        <div className="text-right">
          <p className="font-semibold">${valueUsd.toLocaleString()}</p>
          <div className="flex items-center gap-1 text-xs text-green-500">
            <TrendingUp className="w-3 h-3" />
            <span>+2.5%</span>
          </div>
        </div>
      </div>

      {/* Progress Bar */}
      <div className="mt-3 h-1.5 bg-muted rounded-full overflow-hidden">
        <div
          className={`h-full ${getAssetColor(asset)} transition-all duration-300`}
          style={{ width: `${percentage}%` }}
        />
      </div>
    </div>
  )
}
