import * as React from "react"
import { cn } from "./utils"

interface BalanceDisplayProps extends React.HTMLAttributes<HTMLSpanElement> {
  balance: string | number
  symbol?: string
  decimals?: number
  showSymbol?: boolean
}

export function BalanceDisplay({
  balance,
  symbol = "ÉTR",
  decimals = 12,
  showSymbol = true,
  className,
  ...props
}: BalanceDisplayProps) {
  const formatBalance = () => {
    const num = typeof balance === 'string' ? parseFloat(balance) : balance
    const adjusted = num / Math.pow(10, decimals)
    return adjusted.toLocaleString(undefined, {
      maximumFractionDigits: 2,
      minimumFractionDigits: 0
    })
  }

  return (
    <span
      className={cn("font-mono font-medium", className)}
      {...props}
    >
      {formatBalance()}
      {showSymbol && ` ${symbol}`}
    </span>
  )
}
