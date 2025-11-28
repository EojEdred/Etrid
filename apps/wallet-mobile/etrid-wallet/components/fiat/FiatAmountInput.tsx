"use client"

import { useState, useEffect } from "react"
import { ArrowDownUp } from "lucide-react"
import { Input } from "@/components/ui/input"
import { Button } from "@/components/ui/button"
import type { CryptoAsset, FiatCurrency } from "@/lib/types/features"

interface FiatAmountInputProps {
  asset: CryptoAsset
  fiatCurrency?: FiatCurrency
  exchangeRate: number
  onAmountChange: (amount: number, isFiat: boolean) => void
  minAmount?: number
  maxAmount?: number
  className?: string
}

export function FiatAmountInput({
  asset,
  fiatCurrency = "USD",
  exchangeRate,
  onAmountChange,
  minAmount = 0,
  maxAmount = Infinity,
  className = "",
}: FiatAmountInputProps) {
  const [isFiatInput, setIsFiatInput] = useState(true)
  const [fiatAmount, setFiatAmount] = useState<string>("")
  const [cryptoAmount, setCryptoAmount] = useState<string>("")

  // Update converted amount when rate or input changes
  useEffect(() => {
    if (isFiatInput && fiatAmount) {
      const fiat = parseFloat(fiatAmount)
      if (!isNaN(fiat) && exchangeRate > 0) {
        const crypto = fiat / exchangeRate
        setCryptoAmount(crypto.toFixed(8))
      }
    } else if (!isFiatInput && cryptoAmount) {
      const crypto = parseFloat(cryptoAmount)
      if (!isNaN(crypto) && exchangeRate > 0) {
        const fiat = crypto * exchangeRate
        setFiatAmount(fiat.toFixed(2))
      }
    }
  }, [fiatAmount, cryptoAmount, exchangeRate, isFiatInput])

  const handleFiatChange = (value: string) => {
    setFiatAmount(value)
    const amount = parseFloat(value)
    if (!isNaN(amount)) {
      onAmountChange(amount, true)
    }
  }

  const handleCryptoChange = (value: string) => {
    setCryptoAmount(value)
    const amount = parseFloat(value)
    if (!isNaN(amount)) {
      onAmountChange(amount, false)
    }
  }

  const toggleCurrency = () => {
    setIsFiatInput(!isFiatInput)
  }

  const isValid = () => {
    const amount = parseFloat(isFiatInput ? fiatAmount : cryptoAmount)
    if (isNaN(amount)) return true // Don't show error for empty input
    if (isFiatInput) {
      return amount >= minAmount && amount <= maxAmount
    }
    return true
  }

  return (
    <div className={`space-y-4 ${className}`}>
      {/* Main Input */}
      <div className="glass p-6 rounded-2xl space-y-4">
        <div className="flex items-center justify-between">
          <span className="text-sm text-muted-foreground">You Pay</span>
          <span className="text-xs text-muted-foreground">
            1 {asset} = ${exchangeRate.toFixed(2)} {fiatCurrency}
          </span>
        </div>

        <div className="flex items-center gap-4">
          <Input
            type="number"
            value={isFiatInput ? fiatAmount : cryptoAmount}
            onChange={(e) => (isFiatInput ? handleFiatChange(e.target.value) : handleCryptoChange(e.target.value))}
            placeholder="0.00"
            className="text-2xl font-bold bg-transparent border-none focus-visible:ring-0 h-auto p-0"
          />
          <Button variant="outline" size="sm" onClick={toggleCurrency} className="gap-2 shrink-0">
            <span className="font-semibold">{isFiatInput ? fiatCurrency : asset}</span>
            <ArrowDownUp className="w-4 h-4" />
          </Button>
        </div>

        {!isValid() && (
          <p className="text-sm text-destructive">
            Amount must be between ${minAmount} and ${maxAmount}
          </p>
        )}
      </div>

      {/* Converted Amount Display */}
      <div className="glass p-4 rounded-2xl">
        <div className="flex items-center justify-between">
          <span className="text-sm text-muted-foreground">You Receive</span>
          <div className="text-right">
            <p className="text-lg font-semibold">
              {isFiatInput
                ? `${cryptoAmount || "0"} ${asset}`
                : `$${fiatAmount || "0"} ${fiatCurrency}`}
            </p>
            <p className="text-xs text-muted-foreground">
              {isFiatInput
                ? `${fiatAmount} ${fiatCurrency}`
                : `${cryptoAmount} ${asset}`}
            </p>
          </div>
        </div>
      </div>

      {/* Quick Amount Buttons */}
      <div className="grid grid-cols-4 gap-2">
        {[50, 100, 500, 1000].map((amount) => (
          <Button
            key={amount}
            variant="outline"
            size="sm"
            onClick={() => {
              setIsFiatInput(true)
              handleFiatChange(amount.toString())
            }}
            className="glass"
          >
            ${amount}
          </Button>
        ))}
      </div>
    </div>
  )
}
