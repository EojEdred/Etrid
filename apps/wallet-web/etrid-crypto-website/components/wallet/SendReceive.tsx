'use client'

import { useState, useEffect } from 'react'
import { ArrowUpRight, ArrowDownLeft, Copy, Check, QrCode, AlertCircle, Loader2 } from 'lucide-react'
import { transfer, isValidAddress } from '@/lib/polkadot'
import { Button } from '@/components/ui/button'
import {
  Dialog,
  DialogContent,
  DialogDescription,
  DialogFooter,
  DialogHeader,
  DialogTitle,
} from '@/components/ui/dialog'
import { Card } from '@/components/ui/card'
import { Input } from '@/components/ui/input'

interface SendReceiveProps {
  open: boolean
  onOpenChange: (open: boolean) => void
  mode: 'send' | 'receive'
  address: string
  balance: string
  onSendTransaction?: (to: string, amount: string) => Promise<void>
}

export default function SendReceive({
  open,
  onOpenChange,
  mode,
  address,
  balance,
  onSendTransaction
}: SendReceiveProps) {
  const [currentMode, setCurrentMode] = useState<'send' | 'receive'>(mode)
  const [copied, setCopied] = useState(false)

  // Send state
  const [recipientAddress, setRecipientAddress] = useState('')
  const [sendAmount, setSendAmount] = useState('')
  const [isSending, setIsSending] = useState(false)
  const [sendError, setSendError] = useState('')
  const [gasEstimate, setGasEstimate] = useState('0.0001')

  // QR code state
  const [qrCodeUrl, setQrCodeUrl] = useState('')

  useEffect(() => {
    setCurrentMode(mode)
  }, [mode])

  useEffect(() => {
    if (currentMode === 'receive' && address) {
      // Generate QR code URL using a QR code API
      const qrData = encodeURIComponent(address)
      setQrCodeUrl(`https://api.qrserver.com/v1/create-qr-code/?size=200x200&data=${qrData}&bgcolor=0A0E1A&color=FFFFFF`)
    }
  }, [currentMode, address])

  const copyAddress = () => {
    navigator.clipboard.writeText(address)
    setCopied(true)
    setTimeout(() => setCopied(false), 2000)
  }

  const validateAddress = (addr: string): boolean => {
    // SS58 address validation for Polkadot/Substrate
    if (!addr || addr.length < 45 || addr.length > 50) return false;
    try {
      // Use the API's validation
      return isValidAddress(addr);
    } catch {
      // Fallback basic check - SS58 addresses start with capital letter
      return /^[A-Z][a-zA-Z0-9]{45,48}$/.test(addr);
    }
  }

  const validateAmount = (amount: string): boolean => {
    const num = parseFloat(amount)
    const balanceNum = parseFloat(balance)
    return !isNaN(num) && num > 0 && num <= balanceNum
  }

  const handleMaxAmount = () => {
    // Set to balance minus estimated gas
    const maxAmount = Math.max(0, parseFloat(balance) - parseFloat(gasEstimate))
    setSendAmount(maxAmount.toString())
  }

  const handleSend = async () => {
    setSendError('')

    if (!validateAddress(recipientAddress)) {
      setSendError('Invalid recipient address. Please enter a valid SS58 address.')
      return
    }

    if (!validateAmount(sendAmount)) {
      setSendError('Invalid amount. Please enter a valid amount within your balance.')
      return
    }

    setIsSending(true)

    try {
      // Convert amount to chain units (12 decimals for ETR)
      const amountInUnits = BigInt(Math.floor(parseFloat(sendAmount) * 1e12))

      // Use Polkadot.js transfer (note: this requires wallet signing)
      // For now, call the parent's onSendTransaction which will handle the actual signing
      if (onSendTransaction) {
        await onSendTransaction(recipientAddress, sendAmount)
      }

      setRecipientAddress('')
      setSendAmount('')
      onOpenChange(false)
    } catch (error: any) {
      setSendError(error?.message || 'Failed to send transaction. Please try again.')
    } finally {
      setIsSending(false)
    }
  }

  return (
    <Dialog open={open} onOpenChange={onOpenChange}>
      <DialogContent className="glass-card border-white/20 max-w-md">
        <DialogHeader>
          <DialogTitle className="flex items-center gap-2 text-2xl gradient-text">
            {currentMode === 'send' ? (
              <>
                <ArrowUpRight className="w-6 h-6" />
                Send ETR
              </>
            ) : (
              <>
                <ArrowDownLeft className="w-6 h-6" />
                Receive ETR
              </>
            )}
          </DialogTitle>
          <DialogDescription className="text-white/60">
            {currentMode === 'send'
              ? 'Send ETR tokens to another address'
              : 'Share your address to receive ETR tokens'}
          </DialogDescription>
        </DialogHeader>

        {/* Mode Toggle */}
        <div className="flex gap-2 p-1 glass rounded-lg">
          <button
            onClick={() => setCurrentMode('send')}
            className={`flex-1 px-4 py-2 rounded-md transition-all font-medium ${
              currentMode === 'send'
                ? 'bg-gradient-to-r from-blue-500 to-purple-500 text-white'
                : 'text-white/60 hover:text-white'
            }`}
          >
            <ArrowUpRight className="w-4 h-4 inline mr-2" />
            Send
          </button>
          <button
            onClick={() => setCurrentMode('receive')}
            className={`flex-1 px-4 py-2 rounded-md transition-all font-medium ${
              currentMode === 'receive'
                ? 'bg-gradient-to-r from-blue-500 to-purple-500 text-white'
                : 'text-white/60 hover:text-white'
            }`}
          >
            <ArrowDownLeft className="w-4 h-4 inline mr-2" />
            Receive
          </button>
        </div>

        {currentMode === 'send' ? (
          <div className="space-y-6 py-6">
            {/* Balance Display */}
            <Card className="glass border-white/10 p-4">
              <div className="flex items-center justify-between">
                <span className="text-sm text-white/60">Available Balance</span>
                <span className="text-lg font-semibold gradient-text">{balance} ETR</span>
              </div>
            </Card>

            {/* Recipient Address */}
            <div className="space-y-2">
              <label className="text-sm font-medium">Recipient Address</label>
              <Input
                type="text"
                value={recipientAddress}
                onChange={(e) => setRecipientAddress(e.target.value)}
                placeholder="5..."
                className="font-mono glass border-white/20 bg-transparent text-white placeholder:text-white/40"
              />
            </div>

            {/* Amount */}
            <div className="space-y-2">
              <div className="flex items-center justify-between">
                <label className="text-sm font-medium">Amount</label>
                <Button
                  type="button"
                  variant="ghost"
                  size="sm"
                  onClick={handleMaxAmount}
                  className="text-blue-400 hover:text-blue-300 h-auto p-0"
                >
                  Max
                </Button>
              </div>
              <div className="relative">
                <Input
                  type="number"
                  step="0.000001"
                  value={sendAmount}
                  onChange={(e) => setSendAmount(e.target.value)}
                  placeholder="0.0"
                  className="glass border-white/20 bg-transparent text-white placeholder:text-white/40 pr-16"
                />
                <span className="absolute right-3 top-1/2 -translate-y-1/2 text-white/60 font-medium">
                  ETR
                </span>
              </div>
            </div>

            {/* Gas Estimate */}
            <Card className="glass border-white/10 p-4">
              <div className="flex items-center justify-between text-sm">
                <span className="text-white/60">Estimated Gas Fee</span>
                <span className="font-medium">{gasEstimate} ETR</span>
              </div>
              <div className="flex items-center justify-between text-sm mt-2 pt-2 border-t border-white/10">
                <span className="text-white/60 font-medium">Total</span>
                <span className="font-bold gradient-text">
                  {(parseFloat(sendAmount || '0') + parseFloat(gasEstimate)).toFixed(6)} ETR
                </span>
              </div>
            </Card>

            {/* Error Message */}
            {sendError && (
              <Card className="glass border-red-500/30 p-4">
                <div className="flex items-start gap-3">
                  <AlertCircle className="w-5 h-5 text-red-500 flex-shrink-0" />
                  <p className="text-sm text-red-400">{sendError}</p>
                </div>
              </Card>
            )}

            {/* Warning */}
            <Card className="glass border-yellow-500/30 p-4">
              <div className="flex items-start gap-3">
                <AlertCircle className="w-5 h-5 text-yellow-500 flex-shrink-0" />
                <div>
                  <p className="text-sm font-semibold text-yellow-500">Important</p>
                  <p className="text-sm text-white/80 mt-1">
                    Double-check the recipient address. Transactions cannot be reversed.
                  </p>
                </div>
              </div>
            </Card>
          </div>
        ) : (
          <div className="space-y-4 py-4">
            {/* QR Code */}
            <div className="flex flex-col items-center">
              <div className="p-3 glass-card rounded-xl mb-3">
                {qrCodeUrl ? (
                  <img
                    src={qrCodeUrl}
                    alt="Wallet QR Code"
                    className="w-40 h-40 rounded-lg"
                  />
                ) : (
                  <div className="w-40 h-40 glass rounded-lg flex items-center justify-center">
                    <QrCode className="w-12 h-12 text-white/40" />
                  </div>
                )}
              </div>
              <p className="text-xs text-white/60 text-center">
                Scan to get your wallet address
              </p>
            </div>

            {/* Address Display */}
            <div className="space-y-2">
              <label className="text-sm font-medium">Your Wallet Address</label>
              <div className="flex gap-2">
                <div className="flex-1 px-4 py-3 glass rounded-lg border border-white/20 font-mono text-sm break-all">
                  {address}
                </div>
                <Button
                  onClick={copyAddress}
                  variant="outline"
                  className="glass border-white/20 shrink-0"
                >
                  {copied ? (
                    <Check className="w-4 h-4 text-green-500" />
                  ) : (
                    <Copy className="w-4 h-4" />
                  )}
                </Button>
              </div>
            </div>

            {/* Info Card */}
            <Card className="glass border-blue-500/30 p-4">
              <div className="flex items-start gap-3">
                <AlertCircle className="w-5 h-5 text-blue-400 flex-shrink-0" />
                <div>
                  <p className="text-sm font-semibold text-blue-400">Receive ETR Tokens</p>
                  <p className="text-sm text-white/80 mt-1">
                    Share this address to receive ETR tokens. Only send ETR and ERC-20 tokens to this address.
                  </p>
                </div>
              </div>
            </Card>

            {/* Network Badge */}
            <div className="flex justify-center">
              <div className="px-3 py-1 glass rounded-full text-xs text-white/60">
                ETRID Mainnet • Primearc Core
              </div>
            </div>
          </div>
        )}

        <DialogFooter>
          {currentMode === 'send' ? (
            <>
              <Button
                variant="outline"
                onClick={() => onOpenChange(false)}
                disabled={isSending}
                className="glass border-white/20"
              >
                Cancel
              </Button>
              <Button
                onClick={handleSend}
                disabled={isSending || !recipientAddress || !sendAmount}
                className="btn-primary"
              >
                {isSending ? (
                  <>
                    <Loader2 className="w-4 h-4 mr-2 animate-spin" />
                    Sending...
                  </>
                ) : (
                  <>
                    <ArrowUpRight className="w-4 h-4 mr-2" />
                    Send Transaction
                  </>
                )}
              </Button>
            </>
          ) : (
            <Button
              onClick={() => onOpenChange(false)}
              className="btn-primary w-full"
            >
              Done
            </Button>
          )}
        </DialogFooter>
      </DialogContent>
    </Dialog>
  )
}
