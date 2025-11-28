"use client"

import { useState } from "react"
import { ArrowLeft, Copy, Share2, ImageIcon, Check } from "lucide-react"
import { Button } from "@/components/ui/button"
import { Input } from "@/components/ui/input"
import { Label } from "@/components/ui/label"
import { Select, SelectContent, SelectItem, SelectTrigger, SelectValue } from "@/components/ui/select"
import { QRCodeSVG } from "qrcode.react"

interface ReceiveScreenProps {
  onClose: () => void
}

export function ReceiveScreen({ onClose }: ReceiveScreenProps) {
  const [token, setToken] = useState("ETR")
  const [copied, setCopied] = useState(false)
  const [showAmountRequest, setShowAmountRequest] = useState(false)
  const [requestAmount, setRequestAmount] = useState("")

  const address = "0x1a2b3c4d5e6f7g8h9i0j1k2l3m4n5o6p7q8r9s0t"

  const handleCopy = () => {
    navigator.clipboard.writeText(address)
    setCopied(true)
    setTimeout(() => setCopied(false), 2000)
  }

  return (
    <div className="min-h-screen">
      {/* Header */}
      <header className="flex items-center justify-between p-6 glass-strong">
        <Button variant="ghost" size="icon" onClick={onClose}>
          <ArrowLeft className="w-5 h-5" />
        </Button>
        <h1 className="text-xl font-bold">Receive</h1>
        <Select value={token} onValueChange={setToken}>
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
        {/* QR Code */}
        <div className="flex justify-center">
          <div className="glass-strong rounded-3xl p-8 inline-block">
            <div className="bg-white p-4 rounded-2xl">
              <QRCodeSVG value={address} size={240} level="H" fgColor="#1a0033" bgColor="#ffffff" />
            </div>
          </div>
        </div>

        {/* Address Display */}
        <div className="space-y-3">
          <Label className="text-muted-foreground">Your Ëtrid Address</Label>
          <div className="glass rounded-2xl p-4">
            <div className="flex items-center justify-between gap-3">
              <p className="text-sm font-mono truncate flex-1">
                {address.slice(0, 10)}...{address.slice(-10)}
              </p>
              <Button variant="ghost" size="icon" onClick={handleCopy} className="shrink-0">
                {copied ? <Check className="w-5 h-5 text-success" /> : <Copy className="w-5 h-5 text-accent" />}
              </Button>
            </div>
          </div>
          {copied && <p className="text-success text-sm text-center">Address copied!</p>}
        </div>

        {/* Share Options */}
        <div className="grid grid-cols-3 gap-3">
          <Button variant="outline" className="glass border-border h-auto py-4 flex-col gap-2 bg-transparent">
            <Share2 className="w-5 h-5 text-accent" />
            <span className="text-xs">Share</span>
          </Button>
          <Button variant="outline" className="glass border-border h-auto py-4 flex-col gap-2 bg-transparent">
            <ImageIcon className="w-5 h-5 text-accent" />
            <span className="text-xs">Save QR</span>
          </Button>
          <Button variant="outline" className="glass border-border h-auto py-4 flex-col gap-2 bg-transparent">
            <Copy className="w-5 h-5 text-accent" />
            <span className="text-xs">Copy</span>
          </Button>
        </div>

        {/* Amount Request (Optional) */}
        <div className="space-y-3">
          <button
            onClick={() => setShowAmountRequest(!showAmountRequest)}
            className="flex items-center justify-between w-full text-left"
          >
            <Label className="text-muted-foreground cursor-pointer">Request Specific Amount</Label>
            <div
              className={`w-12 h-6 rounded-full transition-colors ${showAmountRequest ? "bg-accent" : "bg-surface"} relative`}
            >
              <div
                className={`w-5 h-5 rounded-full bg-white absolute top-0.5 transition-transform ${showAmountRequest ? "translate-x-6" : "translate-x-0.5"}`}
              />
            </div>
          </button>

          {showAmountRequest && (
            <div className="space-y-3 animate-in fade-in slide-in-from-top-2">
              <Input
                type="number"
                placeholder="Amount"
                value={requestAmount}
                onChange={(e) => setRequestAmount(e.target.value)}
                className="glass border-border h-12"
              />
              <Input placeholder="What's this for? (optional)" className="glass border-border h-12" />
              <Button className="w-full" style={{ background: "#00d9ff", color: "#000" }}>
                Generate Payment Request
              </Button>
            </div>
          )}
        </div>

        {/* Info Card */}
        <div className="glass rounded-2xl p-4 bg-warning/10 border-warning/20">
          <div className="flex gap-3">
            <div className="w-5 h-5 rounded-full bg-warning/20 flex items-center justify-center shrink-0 mt-0.5">
              <span className="text-warning text-xs">!</span>
            </div>
            <p className="text-sm text-muted-foreground">
              Only send <span className="text-foreground font-semibold">{token}</span> to this address. Sending other
              tokens may result in permanent loss.
            </p>
          </div>
        </div>
      </main>
    </div>
  )
}
