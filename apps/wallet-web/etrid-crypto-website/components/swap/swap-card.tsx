"use client"

import { useState, useEffect } from "react"
import { ChevronDown, ArrowDownUp, Settings } from "lucide-react"
import { Button } from "@/components/ui/button"
import { Input } from "@/components/ui/input"
import { Card } from "@/components/ui/card"
import { TokenSelector } from "./token-selector"
import { SwapDetails } from "./swap-details"
import { ConfirmSwapModal } from "./confirm-swap-modal"
import type { UseWalletReturn } from "@/lib/polkadot/useWallet"
import { getSwapBalances } from "@/lib/polkadot/swap"

interface SwapCardProps {
  wallet: UseWalletReturn
}

export function SwapCard({ wallet }: SwapCardProps) {
  const { isConnected, selectedAccount } = wallet

  const [fromToken, setFromToken] = useState("ÉTR")
  const [toToken, setToToken] = useState("EDSC")
  const [fromAmount, setFromAmount] = useState("")
  const [toAmount, setToAmount] = useState("")
  const [showFromSelector, setShowFromSelector] = useState(false)
  const [showToSelector, setShowToSelector] = useState(false)
  const [showDetails, setShowDetails] = useState(false)
  const [showConfirmModal, setShowConfirmModal] = useState(false)
  const [isSwapping, setIsSwapping] = useState(false)
  const [etrBalance, setEtrBalance] = useState(0)
  const [edscBalance, setEdscBalance] = useState(0)

  // Calculate exchange rate based on direction
  const exchangeRate = fromToken === 'ÉTR' ? 8.0 : 0.125 // 1 ÉTR = 8 EDSC, 1 EDSC = 0.125 ÉTR

  // Fetch balances for both ÉTR and EDSC when wallet connects
  useEffect(() => {
    if (isConnected && selectedAccount?.address) {
      getSwapBalances(selectedAccount.address)
        .then(({ etr, edsc }) => {
          setEtrBalance(parseFloat(etr.balance))
          setEdscBalance(parseFloat(edsc.balance))
        })
        .catch(console.error)
    }
  }, [isConnected, selectedAccount?.address])

  // Get balance for current token
  const fromBalance = isConnected
    ? (fromToken === 'ÉTR' ? etrBalance : edscBalance)
    : 0
  const toBalance = isConnected
    ? (toToken === 'ÉTR' ? etrBalance : edscBalance)
    : 0

  const handleFromAmountChange = (value: string) => {
    setFromAmount(value)
    if (value && !isNaN(Number(value))) {
      const rate = fromToken === 'ÉTR' ? 8.0 : 0.125
      const calculated = (Number(value) * rate).toFixed(4)
      setToAmount(calculated)
    } else {
      setToAmount("")
    }
  }

  const handleSwapDirection = () => {
    setFromToken(toToken)
    setToToken(fromToken)
    setFromAmount(toAmount)
    setToAmount(fromAmount)
  }

  const handleMaxClick = () => {
    handleFromAmountChange(fromBalance.toString())
  }

  const getButtonState = () => {
    if (!isConnected) {
      return { text: "Connect Wallet to Swap", disabled: true, variant: "secondary" as const }
    }
    if (!fromAmount || Number(fromAmount) === 0) {
      return { text: "Enter amount", disabled: true, variant: "secondary" as const }
    }
    if (Number(fromAmount) > fromBalance) {
      return { text: "Insufficient balance", disabled: true, variant: "destructive" as const }
    }
    if (isSwapping) {
      return { text: "Swapping...", disabled: true, variant: "default" as const }
    }
    return { text: "Swap", disabled: false, variant: "default" as const }
  }

  const buttonState = getButtonState()

  const handleSwap = () => {
    if (!buttonState.disabled) {
      setShowConfirmModal(true)
    }
  }

  return (
    <>
      <Card className="p-6 bg-card/50 backdrop-blur-xl border-border/50">
        <div className="flex items-center justify-between mb-6">
          <h2 className="text-2xl font-bold">Swap</h2>
          <Button variant="ghost" size="icon" onClick={() => setShowDetails(!showDetails)}>
            <Settings className="w-5 h-5" />
          </Button>
        </div>

        {/* From Section */}
        <div className="space-y-2 mb-2">
          <div className="flex items-center justify-between text-sm">
            <span className="text-muted-foreground">You pay</span>
            <span className="text-muted-foreground">
              Balance: {fromBalance.toLocaleString()} {fromToken}
            </span>
          </div>

          <div className="flex items-center gap-2 p-4 rounded-xl bg-background/50 border border-border/50">
            <Input
              type="number"
              placeholder="0.0"
              value={fromAmount}
              onChange={(e) => handleFromAmountChange(e.target.value)}
              className="text-3xl font-bold border-0 bg-transparent p-0 h-auto focus-visible:ring-0"
            />

            <div className="flex items-center gap-2 shrink-0">
              <Button
                variant="outline"
                size="sm"
                onClick={handleMaxClick}
                className="text-accent border-accent/50 hover:bg-accent/10 bg-transparent"
              >
                MAX
              </Button>
              <Button variant="ghost" onClick={() => setShowFromSelector(true)} className="gap-2 font-semibold">
                {fromToken}
                <ChevronDown className="w-4 h-4" />
              </Button>
            </div>
          </div>

          {fromAmount && (
            <div className="text-sm text-muted-foreground text-right">
              ≈ $
              {(Number(fromAmount) * 8).toLocaleString(undefined, {
                minimumFractionDigits: 2,
                maximumFractionDigits: 2,
              })}
            </div>
          )}
        </div>

        {/* Swap Direction Button */}
        <div className="flex justify-center -my-2 relative z-10">
          <Button
            variant="outline"
            size="icon"
            onClick={handleSwapDirection}
            className="rounded-full bg-card border-2 border-border hover:bg-accent/10 hover:border-accent transition-all hover:rotate-180 duration-300"
          >
            <ArrowDownUp className="w-5 h-5" />
          </Button>
        </div>

        {/* To Section */}
        <div className="space-y-2 mt-2">
          <div className="flex items-center justify-between text-sm">
            <span className="text-muted-foreground">You receive</span>
            <span className="text-muted-foreground">
              Balance: {toBalance.toLocaleString()} {toToken}
            </span>
          </div>

          <div className="flex items-center gap-2 p-4 rounded-xl bg-background/50 border border-border/50">
            <div className="text-3xl font-bold flex-1">{toAmount || "0.0"}</div>

            <Button variant="ghost" onClick={() => setShowToSelector(true)} className="gap-2 font-semibold shrink-0">
              {toToken}
              <ChevronDown className="w-4 h-4" />
            </Button>
          </div>

          {toAmount && (
            <div className="text-sm text-muted-foreground text-right">
              ≈ $
              {(Number(toAmount) * 1).toLocaleString(undefined, { minimumFractionDigits: 2, maximumFractionDigits: 2 })}
            </div>
          )}
        </div>

        {/* Details Dropdown */}
        {showDetails && <SwapDetails fromAmount={fromAmount} toAmount={toAmount} />}

        {/* Swap Button */}
        <Button
          onClick={handleSwap}
          disabled={buttonState.disabled}
          variant={buttonState.variant}
          className="w-full mt-6 h-14 text-lg font-semibold"
        >
          {buttonState.text}
        </Button>
      </Card>

      <TokenSelector
        open={showFromSelector}
        onClose={() => setShowFromSelector(false)}
        onSelect={(token) => {
          setFromToken(token)
          setShowFromSelector(false)
        }}
        currentToken={fromToken}
      />

      <TokenSelector
        open={showToSelector}
        onClose={() => setShowToSelector(false)}
        onSelect={(token) => {
          setToToken(token)
          setShowToSelector(false)
        }}
        currentToken={toToken}
      />

      <ConfirmSwapModal
        open={showConfirmModal}
        onClose={() => setShowConfirmModal(false)}
        fromToken={fromToken}
        toToken={toToken}
        fromAmount={fromAmount}
        toAmount={toAmount}
        wallet={wallet}
        onSwapComplete={() => {
          setShowConfirmModal(false)
          setFromAmount("")
          setToAmount("")
          // Refresh balances
          if (selectedAccount?.address) {
            getSwapBalances(selectedAccount.address)
              .then(({ etr, edsc }) => {
                setEtrBalance(parseFloat(etr.balance))
                setEdscBalance(parseFloat(edsc.balance))
              })
              .catch(console.error)
          }
        }}
      />
    </>
  )
}
