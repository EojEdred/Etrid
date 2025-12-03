"use client"

import { useState, useEffect } from "react"
import { ChevronDown, ArrowDownUp, Settings, AlertTriangle } from "lucide-react"
import { Button } from "@/components/ui/button"
import { Input } from "@/components/ui/input"
import { Card } from "@/components/ui/card"
import { TokenSelector } from "./token-selector"
import { SwapDetails } from "./swap-details"
import { ConfirmSwapModal } from "./confirm-swap-modal"
import type { UseWalletReturn } from "@/lib/polkadot/useWallet"
import { getSwapBalances } from "@/lib/polkadot/swap"
import {
  Network,
  Token,
  useQuote,
  useNetworkSelection,
  useAvailableTokens,
  useTokenBalance,
  useSlippage,
  WETR_TOKEN,
  formatTokenAmount,
  formatUsdValue,
  getPriceImpactLevel,
} from "@/lib/dex"

interface SwapCardProps {
  wallet: UseWalletReturn
}

export function SwapCard({ wallet }: SwapCardProps) {
  const { isConnected, selectedAccount } = wallet

  // Network and DEX selection
  const { network, dex, updateNetwork } = useNetworkSelection(Network.ETHEREUM)

  // Available tokens for selected network
  const availableTokens = useAvailableTokens(network)

  const [fromToken, setFromToken] = useState<Token | null>(null)
  const [toToken, setToToken] = useState<Token | null>(null)
  const [fromAmount, setFromAmount] = useState("")
  const [showFromSelector, setShowFromSelector] = useState(false)
  const [showToSelector, setShowToSelector] = useState(false)
  const [showDetails, setShowDetails] = useState(false)
  const [showConfirmModal, setShowConfirmModal] = useState(false)
  const [isSwapping, setIsSwapping] = useState(false)

  // Initialize tokens when available tokens change
  useEffect(() => {
    if (availableTokens.length > 0 && !fromToken) {
      setFromToken(availableTokens[0]) // wETR
      setToToken(availableTokens[1]) // First base token
    }
  }, [availableTokens, fromToken])

  // Slippage management
  const { slippage, updateSlippage, setCustom: setCustomSlippage } = useSlippage()

  // Get quote from DEX
  const { quote, loading: quoteLoading, error: quoteError, refetch: refetchQuote } = useQuote(
    network,
    fromToken,
    toToken,
    fromAmount,
    slippage
  )

  // Get token balances
  const { balance: fromBalance, refetch: refetchFromBalance } = useTokenBalance(
    network,
    fromToken?.address || null,
    selectedAccount?.address || null
  )

  const { balance: toBalance, refetch: refetchToBalance } = useTokenBalance(
    network,
    toToken?.address || null,
    selectedAccount?.address || null
  )

  // Calculate output amount from quote
  const toAmount = quote?.outputAmount || ""

  const handleFromAmountChange = (value: string) => {
    setFromAmount(value)
  }

  const handleSwapDirection = () => {
    const tempToken = fromToken
    setFromToken(toToken)
    setToToken(tempToken)
    setFromAmount("")
  }

  const handleMaxClick = () => {
    handleFromAmountChange(fromBalance)
  }

  const getButtonState = () => {
    if (!isConnected) {
      return { text: "Connect Wallet to Swap", disabled: true, variant: "secondary" as const }
    }
    if (!fromAmount || Number(fromAmount) === 0) {
      return { text: "Enter amount", disabled: true, variant: "secondary" as const }
    }
    if (quoteLoading) {
      return { text: "Fetching quote...", disabled: true, variant: "secondary" as const }
    }
    if (quoteError) {
      return { text: "Error fetching quote", disabled: true, variant: "destructive" as const }
    }
    if (!quote) {
      return { text: "No quote available", disabled: true, variant: "secondary" as const }
    }
    if (Number(fromAmount) > Number(fromBalance)) {
      return { text: "Insufficient balance", disabled: true, variant: "destructive" as const }
    }
    if (quote.priceImpact > 10) {
      return { text: "Price impact too high", disabled: true, variant: "destructive" as const }
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

  const priceImpactInfo = quote ? getPriceImpactLevel(quote.priceImpact) : null

  return (
    <>
      <Card className="p-6 bg-card/50 backdrop-blur-xl border-border/50">
        <div className="flex items-center justify-between mb-6">
          <div>
            <h2 className="text-2xl font-bold">Swap</h2>
            <p className="text-sm text-muted-foreground mt-1">
              {network.toUpperCase()} • {dex.charAt(0).toUpperCase() + dex.slice(1)}
            </p>
          </div>
          <Button variant="ghost" size="icon" onClick={() => setShowDetails(!showDetails)}>
            <Settings className="w-5 h-5" />
          </Button>
        </div>

        {/* Network Warning for High Price Impact */}
        {priceImpactInfo && priceImpactInfo.level === 'high' && (
          <div className="mb-4 p-3 rounded-lg bg-orange-500/10 border border-orange-500/20 flex items-start gap-2">
            <AlertTriangle className="w-4 h-4 text-orange-500 mt-0.5" />
            <div className="text-sm">
              <p className="font-medium text-orange-500">High Price Impact</p>
              <p className="text-muted-foreground">
                This swap will move the market price by {quote?.priceImpact.toFixed(2)}%
              </p>
            </div>
          </div>
        )}

        {/* From Section */}
        <div className="space-y-2 mb-2">
          <div className="flex items-center justify-between text-sm">
            <span className="text-muted-foreground">You pay</span>
            <span className="text-muted-foreground">
              Balance: {formatTokenAmount(fromBalance)} {fromToken?.symbol || ''}
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
                disabled={!isConnected}
              >
                MAX
              </Button>
              <Button variant="ghost" onClick={() => setShowFromSelector(true)} className="gap-2 font-semibold">
                {fromToken?.symbol || 'Select'}
                <ChevronDown className="w-4 h-4" />
              </Button>
            </div>
          </div>

          {fromAmount && quote && (
            <div className="text-sm text-muted-foreground text-right">
              ≈ {formatUsdValue(Number(fromAmount) * 8)}
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
              Balance: {formatTokenAmount(toBalance)} {toToken?.symbol || ''}
            </span>
          </div>

          <div className="flex items-center gap-2 p-4 rounded-xl bg-background/50 border border-border/50">
            <div className="text-3xl font-bold flex-1">
              {quoteLoading ? (
                <span className="text-muted-foreground animate-pulse">Loading...</span>
              ) : (
                formatTokenAmount(toAmount)
              )}
            </div>

            <Button variant="ghost" onClick={() => setShowToSelector(true)} className="gap-2 font-semibold shrink-0">
              {toToken?.symbol || 'Select'}
              <ChevronDown className="w-4 h-4" />
            </Button>
          </div>

          {toAmount && quote && (
            <div className="text-sm text-muted-foreground text-right">
              ≈ {formatUsdValue(Number(toAmount) * 1)}
            </div>
          )}
        </div>

        {/* Details Dropdown */}
        {showDetails && (
          <SwapDetails
            fromAmount={fromAmount}
            toAmount={toAmount}
            quote={quote}
            slippage={slippage}
            onSlippageChange={updateSlippage}
            onCustomSlippage={setCustomSlippage}
            network={network}
          />
        )}

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
        currentToken={fromToken?.symbol || ''}
        tokens={availableTokens}
        network={network}
      />

      <TokenSelector
        open={showToSelector}
        onClose={() => setShowToSelector(false)}
        onSelect={(token) => {
          setToToken(token)
          setShowToSelector(false)
        }}
        currentToken={toToken?.symbol || ''}
        tokens={availableTokens}
        network={network}
      />

      <ConfirmSwapModal
        open={showConfirmModal}
        onClose={() => setShowConfirmModal(false)}
        fromToken={fromToken?.symbol || ''}
        toToken={toToken?.symbol || ''}
        fromAmount={fromAmount}
        toAmount={toAmount}
        quote={quote}
        network={network}
        dex={dex}
        wallet={wallet}
        onSwapComplete={() => {
          setShowConfirmModal(false)
          setFromAmount("")
          // Refresh balances
          refetchFromBalance()
          refetchToBalance()
        }}
      />
    </>
  )
}
