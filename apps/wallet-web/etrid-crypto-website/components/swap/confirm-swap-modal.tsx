"use client"

import { useState } from "react"
import { Dialog, DialogContent, DialogHeader, DialogTitle } from "@/components/ui/dialog"
import { Button } from "@/components/ui/button"
import { ArrowRight, AlertCircle } from "lucide-react"
import type { UseWalletReturn } from "@/lib/polkadot/useWallet"
import { executeSwap } from "@/lib/polkadot/swap"

interface ConfirmSwapModalProps {
  open: boolean
  onClose: () => void
  fromToken: string
  toToken: string
  fromAmount: string
  toAmount: string
  wallet: UseWalletReturn
  onSwapComplete: () => void
}

export function ConfirmSwapModal({
  open,
  onClose,
  fromToken,
  toToken,
  fromAmount,
  toAmount,
  wallet,
  onSwapComplete,
}: ConfirmSwapModalProps) {
  const { selectedAccount } = wallet
  const [isSwapping, setIsSwapping] = useState(false)
  const [swapError, setSwapError] = useState<string | null>(null)
  const [txHash, setTxHash] = useState<string | null>(null)

  const exchangeRate = fromToken === 'ÉTR' ? 8.0 : 0.125

  const handleConfirmSwap = async () => {
    if (!selectedAccount) return

    setIsSwapping(true)
    setSwapError(null)
    setTxHash(null)

    try {
      // Import web3FromAddress dynamically to avoid SSR issues
      const { web3FromAddress } = await import('@polkadot/extension-dapp')
      const injector = await web3FromAddress(selectedAccount.address)

      // Execute swap transaction
      const hash = await executeSwap(
        fromToken as 'ÉTR' | 'EDSC',
        toToken as 'ÉTR' | 'EDSC',
        fromAmount,
        selectedAccount.address,
        injector.signer
      )

      setTxHash(hash)

      // Close modal after successful swap
      setTimeout(() => {
        onSwapComplete()
      }, 2000)
    } catch (err) {
      const message = err instanceof Error ? err.message : 'Failed to execute swap'
      setSwapError(message)
      console.error('[ConfirmSwapModal] Swap error:', err)
    } finally {
      setIsSwapping(false)
    }
  }
  return (
    <Dialog open={open} onOpenChange={onClose}>
      <DialogContent className="bg-card/95 backdrop-blur-xl border-border/50">
        <DialogHeader>
          <DialogTitle>Confirm Swap</DialogTitle>
        </DialogHeader>

        <div className="space-y-6">
          {/* Swap summary */}
          <div className="flex items-center justify-between p-4 rounded-xl bg-background/50">
            <div className="text-center">
              <div className="text-2xl font-bold">{fromAmount}</div>
              <div className="text-sm text-muted-foreground">{fromToken}</div>
            </div>

            <ArrowRight className="w-6 h-6 text-muted-foreground" />

            <div className="text-center">
              <div className="text-2xl font-bold">{toAmount}</div>
              <div className="text-sm text-muted-foreground">{toToken}</div>
            </div>
          </div>

          {/* Details */}
          <div className="space-y-2 text-sm">
            <div className="flex justify-between">
              <span className="text-muted-foreground">Exchange rate</span>
              <span className="font-medium">
                1 {fromToken} = {exchangeRate.toFixed(4)} {toToken}
              </span>
            </div>
            <div className="flex justify-between">
              <span className="text-muted-foreground">Price impact</span>
              <span className="font-medium text-success">0%</span>
            </div>
            <div className="flex justify-between">
              <span className="text-muted-foreground">Minimum received</span>
              <span className="font-medium">
                {(Number(toAmount) * 0.997).toFixed(4)} {toToken}
              </span>
            </div>
            <div className="flex justify-between">
              <span className="text-muted-foreground">Swap fee</span>
              <span className="font-medium">0.3%</span>
            </div>
          </div>

          {/* Error message */}
          {swapError && (
            <div className="flex gap-3 p-3 rounded-lg bg-destructive/10 border border-destructive/20">
              <AlertCircle className="w-5 h-5 text-destructive shrink-0 mt-0.5" />
              <p className="text-sm text-destructive">{swapError}</p>
            </div>
          )}

          {/* Success message */}
          {txHash && (
            <div className="flex gap-3 p-3 rounded-lg bg-green-500/10 border border-green-500/20">
              <p className="text-sm text-green-500">
                Swap successful! Transaction: {txHash.slice(0, 10)}...{txHash.slice(-8)}
              </p>
            </div>
          )}

          {/* Warning */}
          {!txHash && !swapError && (
            <div className="flex gap-3 p-3 rounded-lg bg-warning/10 border border-warning/20">
              <AlertCircle className="w-5 h-5 text-warning shrink-0 mt-0.5" />
              <p className="text-sm text-muted-foreground">
                You will receive at least {(Number(toAmount) * 0.997).toFixed(4)} {toToken} after swap fees.
              </p>
            </div>
          )}

          {/* Confirm button */}
          <Button
            onClick={handleConfirmSwap}
            disabled={isSwapping || !!txHash}
            className="w-full h-12 text-lg font-semibold bg-accent hover:bg-accent/90"
          >
            {isSwapping ? 'Swapping...' : txHash ? 'Swap Complete' : 'Confirm Swap'}
          </Button>

          {isSwapping && (
            <p className="text-xs text-muted-foreground text-center">
              Sign transaction in Polkadot.js extension...
            </p>
          )}
        </div>
      </DialogContent>
    </Dialog>
  )
}
