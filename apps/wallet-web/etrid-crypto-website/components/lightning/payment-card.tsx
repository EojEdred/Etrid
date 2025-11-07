"use client"

import { useState } from "react"
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from "@/components/ui/card"
import { Button } from "@/components/ui/button"
import { Input } from "@/components/ui/input"
import { Label } from "@/components/ui/label"
import { Select, SelectContent, SelectItem, SelectTrigger, SelectValue } from "@/components/ui/select"
import { Zap, ArrowRight, Loader2 } from "lucide-react"
import { SUPPORTED_CHAINS } from "@/lib/lightning/types"
import type { CrossPBCRoute } from "@/lib/lightning/types"

interface PaymentCardProps {
  wallet: any
  lightning: any
}

export function PaymentCard({ wallet, lightning }: PaymentCardProps) {
  const [sourceChain, setSourceChain] = useState("eth-pbc")
  const [destChain, setDestChain] = useState("btc-pbc")
  const [amount, setAmount] = useState("")
  const [destAddress, setDestAddress] = useState("")
  const [route, setRoute] = useState<CrossPBCRoute | null>(null)
  const [step, setStep] = useState<"input" | "confirm">("input")

  const sourceChainInfo = SUPPORTED_CHAINS.find(c => c.id === sourceChain)
  const destChainInfo = SUPPORTED_CHAINS.find(c => c.id === destChain)

  async function handleFindRoute() {
    if (!wallet.account || !amount || !destAddress) return

    const foundRoute = await lightning.findRoute({
      sourceChain,
      destChain,
      sourceAddress: wallet.account.address,
      destAddress,
      amount,
    })

    if (foundRoute) {
      setRoute(foundRoute)
      setStep("confirm")
    }
  }

  async function handleSendPayment() {
    if (!route || !wallet.account) return

    try {
      await lightning.sendPayment({
        route,
        sourceAddress: wallet.account.address,
        destAddress,
      })

      // Reset form
      setAmount("")
      setDestAddress("")
      setRoute(null)
      setStep("input")
    } catch (err) {
      console.error("Payment failed:", err)
    }
  }

  if (step === "confirm" && route) {
    return (
      <Card>
        <CardHeader>
          <CardTitle className="flex items-center gap-2">
            <Zap className="h-5 w-5 text-yellow-500" />
            Confirm Cross-Chain Payment
          </CardTitle>
          <CardDescription>
            Review your Lightning payment details
          </CardDescription>
        </CardHeader>
        <CardContent className="space-y-6">
          {/* Route visualization */}
          <div className="bg-muted/50 p-4 rounded-lg space-y-3">
            <div className="flex items-center justify-between">
              <div>
                <div className="text-sm text-muted-foreground">You send</div>
                <div className="text-2xl font-bold">
                  {amount} {sourceChainInfo?.symbol}
                </div>
                <div className="text-sm text-muted-foreground">
                  on {sourceChainInfo?.name}
                </div>
              </div>
              <ArrowRight className="h-6 w-6 text-muted-foreground" />
              <div className="text-right">
                <div className="text-sm text-muted-foreground">They receive</div>
                <div className="text-2xl font-bold">
                  {(parseFloat(amount) * route.exchangeRate.rate / 10000).toFixed(6)} {destChainInfo?.symbol}
                </div>
                <div className="text-sm text-muted-foreground">
                  on {destChainInfo?.name}
                </div>
              </div>
            </div>
          </div>

          {/* Route details */}
          <div className="space-y-2">
            <div className="flex justify-between text-sm">
              <span className="text-muted-foreground">Exchange rate</span>
              <span className="font-medium">
                1 {sourceChainInfo?.symbol} = {(route.exchangeRate.rate / 10000).toFixed(6)} {destChainInfo?.symbol}
              </span>
            </div>
            <div className="flex justify-between text-sm">
              <span className="text-muted-foreground">Route segments</span>
              <span className="font-medium">{route.segments.length}</span>
            </div>
            <div className="flex justify-between text-sm">
              <span className="text-muted-foreground">Total fees</span>
              <span className="font-medium">{route.totalFees} {sourceChainInfo?.symbol}</span>
            </div>
            <div className="flex justify-between text-sm">
              <span className="text-muted-foreground">Estimated time</span>
              <span className="font-medium">&lt;{route.estimatedTime}s</span>
            </div>
          </div>

          {/* Destination address */}
          <div className="bg-muted/30 p-3 rounded">
            <div className="text-xs text-muted-foreground mb-1">To address</div>
            <div className="text-sm font-mono break-all">{destAddress}</div>
          </div>

          {/* Actions */}
          <div className="flex gap-3">
            <Button
              variant="outline"
              className="flex-1"
              onClick={() => {
                setRoute(null)
                setStep("input")
              }}
            >
              Back
            </Button>
            <Button
              className="flex-1"
              onClick={handleSendPayment}
              disabled={lightning.loading}
            >
              {lightning.loading ? (
                <>
                  <Loader2 className="mr-2 h-4 w-4 animate-spin" />
                  Sending...
                </>
              ) : (
                <>
                  <Zap className="mr-2 h-4 w-4" />
                  Send Payment
                </>
              )}
            </Button>
          </div>
        </CardContent>
      </Card>
    )
  }

  return (
    <Card>
      <CardHeader>
        <CardTitle className="flex items-center gap-2">
          <Zap className="h-5 w-5 text-yellow-500" />
          Lightning Cross-Chain Payment
        </CardTitle>
        <CardDescription>
          Instant payments across 14 blockchain networks
        </CardDescription>
      </CardHeader>
      <CardContent className="space-y-6">
        {/* Source chain & amount */}
        <div className="space-y-4">
          <div>
            <Label>From Chain</Label>
            <Select value={sourceChain} onValueChange={setSourceChain}>
              <SelectTrigger>
                <SelectValue />
              </SelectTrigger>
              <SelectContent>
                {SUPPORTED_CHAINS.map((chain) => (
                  <SelectItem key={chain.id} value={chain.id}>
                    {chain.name} ({chain.symbol})
                  </SelectItem>
                ))}
              </SelectContent>
            </Select>
          </div>

          <div>
            <Label>Amount</Label>
            <Input
              type="number"
              placeholder="0.0"
              value={amount}
              onChange={(e) => setAmount(e.target.value)}
              disabled={!wallet.account}
            />
          </div>
        </div>

        {/* Destination */}
        <div className="space-y-4">
          <div>
            <Label>To Chain</Label>
            <Select value={destChain} onValueChange={setDestChain}>
              <SelectTrigger>
                <SelectValue />
              </SelectTrigger>
              <SelectContent>
                {SUPPORTED_CHAINS.filter(c => c.id !== sourceChain).map((chain) => (
                  <SelectItem key={chain.id} value={chain.id}>
                    {chain.name} ({chain.symbol})
                  </SelectItem>
                ))}
              </SelectContent>
            </Select>
          </div>

          <div>
            <Label>Recipient Address</Label>
            <Input
              placeholder={`${destChainInfo?.name} address`}
              value={destAddress}
              onChange={(e) => setDestAddress(e.target.value)}
              disabled={!wallet.account}
            />
          </div>
        </div>

        {/* Find route button */}
        <Button
          className="w-full"
          onClick={handleFindRoute}
          disabled={!wallet.account || !amount || !destAddress || lightning.loading}
        >
          {lightning.loading ? (
            <>
              <Loader2 className="mr-2 h-4 w-4 animate-spin" />
              Finding Route...
            </>
          ) : (
            <>
              <Zap className="mr-2 h-4 w-4" />
              Find Route
            </>
          )}
        </Button>

        {!wallet.account && (
          <p className="text-sm text-muted-foreground text-center">
            Connect your wallet to send payments
          </p>
        )}

        {lightning.error && (
          <div className="p-3 bg-destructive/10 border border-destructive/20 rounded text-sm text-destructive">
            {lightning.error}
          </div>
        )}
      </CardContent>
    </Card>
  )
}
