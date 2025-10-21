"use client"

import { useState } from "react"
import { ArrowLeft, QrCode, CheckCircle, Loader2 } from "lucide-react"
import { Button } from "@/components/ui/button"
import { Input } from "@/components/ui/input"
import { Label } from "@/components/ui/label"
import { Select, SelectContent, SelectItem, SelectTrigger, SelectValue } from "@/components/ui/select"
import { useWallet } from "@/lib/polkadot/useWallet"

interface SendScreenProps {
  onClose: () => void
}

export function SendScreen({ onClose }: SendScreenProps) {
  const { sendTransaction, selectedAccount, selectedChain, setSelectedChain } = useWallet()
  const [token, setToken] = useState<"ETR" | "EDSC">("ETR")
  const [amount, setAmount] = useState("")
  const [address, setAddress] = useState("")
  const [fee, setFee] = useState("standard")
  const [isSending, setIsSending] = useState(false)
  const [txHash, setTxHash] = useState<string | null>(null)
  const [error, setError] = useState<string | null>(null)

  // Get balance from selected account (you'd update this to fetch from chain)
  const availableBalance = selectedAccount?.balance
    ? parseFloat(selectedAccount.balance)
    : token === "ETR" ? 1234.56 : 2469.13

  const feeAmount = fee === "standard" ? 0.001 : fee === "fast" ? 0.002 : 0.001
  const totalAmount = Number.parseFloat(amount || "0") + feeAmount

  const isValid = address.length > 0 && Number.parseFloat(amount || "0") > 0 && totalAmount <= availableBalance

  // Handle token change - switch chain accordingly
  const handleTokenChange = (value: "ETR" | "EDSC") => {
    setToken(value)
    // Switch to appropriate chain
    if (value === "ETR") {
      setSelectedChain('flarechain')
    } else {
      setSelectedChain('edsc-pbc')
    }
  }

  // Handle transaction submission
  const handleSendTransaction = async () => {
    if (!isValid || !selectedAccount) return

    setIsSending(true)
    setError(null)
    setTxHash(null)

    try {
      const hash = await sendTransaction(address, amount)
      setTxHash(hash)

      // Reset form after successful send
      setTimeout(() => {
        setAmount("")
        setAddress("")
        onClose()
      }, 3000)
    } catch (err) {
      const message = err instanceof Error ? err.message : 'Transaction failed'
      setError(message)
      console.error('[SendScreen] Transaction error:', err)
    } finally {
      setIsSending(false)
    }
  }

  return (
    <div className="min-h-screen">
      {/* Header */}
      <header className="flex items-center justify-between p-6 glass-strong">
        <Button variant="ghost" size="icon" onClick={onClose}>
          <ArrowLeft className="w-5 h-5" />
        </Button>
        <h1 className="text-xl font-bold">Send</h1>
        <Select value={token} onValueChange={handleTokenChange}>
          <SelectTrigger className="w-24 glass border-border">
            <SelectValue />
          </SelectTrigger>
          <SelectContent>
            <SelectItem value="ETR">ÉTR</SelectItem>
            <SelectItem value="EDSC">EDSC</SelectItem>
          </SelectContent>
        </Select>
      </header>

      {/* Content */}
      <main className="p-6 space-y-6">
        {/* Recipient Section */}
        <div className="space-y-3">
          <Label className="text-muted-foreground">To</Label>
          <div className="relative">
            <Input
              placeholder="Enter address or ENS name"
              value={address}
              onChange={(e) => setAddress(e.target.value)}
              className="glass border-border pr-12 h-14 text-base"
            />
            <Button variant="ghost" size="icon" className="absolute right-2 top-1/2 -translate-y-1/2">
              <QrCode className="w-5 h-5 text-accent" />
            </Button>
          </div>

          {/* Recent Contacts */}
          <div className="flex gap-3 pt-2">
            {[1, 2, 3].map((i) => (
              <button
                key={i}
                className="flex flex-col items-center gap-1"
                onClick={() => setAddress(`0x${i}a2b...4f5g`)}
              >
                <div className="w-12 h-12 rounded-full bg-accent/20 flex items-center justify-center">
                  <span className="text-accent font-bold text-sm">{i}</span>
                </div>
                <span className="text-xs text-muted-foreground">Contact {i}</span>
              </button>
            ))}
          </div>
        </div>

        {/* Amount Section */}
        <div className="space-y-3">
          <div className="flex items-center justify-between">
            <Label className="text-muted-foreground">Amount</Label>
            <span className="text-sm text-muted-foreground">
              Available: {availableBalance.toFixed(2)} {token}
            </span>
          </div>
          <div className="glass-strong rounded-3xl p-6">
            <div className="flex items-center gap-2">
              <Input
                type="number"
                placeholder="0.00"
                value={amount}
                onChange={(e) => setAmount(e.target.value)}
                className="text-4xl font-bold bg-transparent border-0 p-0 h-auto focus-visible:ring-0"
              />
              <span className="text-2xl font-semibold text-muted-foreground">{token}</span>
            </div>
            <p className="text-muted-foreground mt-2">
              ≈ ${(Number.parseFloat(amount || "0") * (token === "ETR" ? 8 : 1)).toFixed(2)}
            </p>
            <Button
              variant="ghost"
              size="sm"
              className="mt-3 text-accent"
              onClick={() => setAmount(availableBalance.toString())}
            >
              Max
            </Button>
          </div>
        </div>

        {/* Fee Section */}
        <div className="space-y-3">
          <Label className="text-muted-foreground">Network Fee</Label>
          <Select value={fee} onValueChange={setFee}>
            <SelectTrigger className="glass border-border h-14">
              <SelectValue />
            </SelectTrigger>
            <SelectContent>
              <SelectItem value="standard">Standard (~5 sec) - 0.001 {token}</SelectItem>
              <SelectItem value="fast">Fast (~3 sec) - 0.002 {token}</SelectItem>
              <SelectItem value="custom">Custom</SelectItem>
            </SelectContent>
          </Select>
          <div className="flex items-center justify-between text-sm">
            <span className="text-muted-foreground">Fee amount</span>
            <span>
              {feeAmount} {token} ≈ ${(feeAmount * (token === "ETR" ? 8 : 1)).toFixed(3)}
            </span>
          </div>
        </div>

        {/* Total Section */}
        <div className="glass-strong rounded-2xl p-4">
          <div className="flex items-center justify-between">
            <span className="text-lg font-semibold">Total</span>
            <div className="text-right">
              <p className="text-xl font-bold">
                {totalAmount.toFixed(3)} {token}
              </p>
              <p className="text-sm text-muted-foreground">≈ ${(totalAmount * (token === "ETR" ? 8 : 1)).toFixed(2)}</p>
            </div>
          </div>
        </div>

        {/* Success/Error Messages */}
        {txHash && (
          <div className="glass-strong rounded-2xl p-4 border border-success/30">
            <div className="flex items-center gap-2 text-success mb-2">
              <CheckCircle className="w-5 h-5" />
              <span className="font-semibold">Transaction Sent!</span>
            </div>
            <p className="text-sm text-muted-foreground">
              Hash: {txHash.slice(0, 10)}...{txHash.slice(-8)}
            </p>
          </div>
        )}

        {error && (
          <div className="glass-strong rounded-2xl p-4 border border-error/30">
            <p className="text-error text-sm">{error}</p>
          </div>
        )}

        {/* Validation Messages */}
        {!error && !txHash && (
          <>
            {address.length > 0 && address.length < 10 && <p className="text-error text-sm">Invalid address format</p>}
            {Number.parseFloat(amount || "0") > availableBalance && (
              <p className="text-error text-sm">Insufficient balance</p>
            )}
          </>
        )}

        {/* Send Button */}
        <Button
          className="w-full h-14 text-lg font-semibold"
          disabled={!isValid || isSending}
          onClick={handleSendTransaction}
          style={{
            background: isValid && !isSending ? "#00d9ff" : undefined,
            color: isValid && !isSending ? "#000" : undefined,
          }}
        >
          {isSending ? (
            <>
              <Loader2 className="w-5 h-5 mr-2 animate-spin" />
              Sending...
            </>
          ) : txHash ? (
            <>
              <CheckCircle className="w-5 h-5 mr-2" />
              Sent!
            </>
          ) : (
            "Send Transaction"
          )}
        </Button>
      </main>
    </div>
  )
}
