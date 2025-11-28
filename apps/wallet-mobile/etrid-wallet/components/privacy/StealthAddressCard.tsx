"use client"

import { QrCode, Copy, RefreshCw, CheckCircle2, Clock, XCircle } from "lucide-react"
import { StealthAddress } from "@/lib/types/privacy"
import { Card, CardContent } from "@/components/ui/card"
import { Button } from "@/components/ui/button"
import { Badge } from "@/components/ui/badge"
import { useState } from "react"
import { toast } from "sonner"
import QRCode from "qrcode.react"

interface StealthAddressCardProps {
  address: StealthAddress
  onGenerate?: () => void
  showQR?: boolean
}

export function StealthAddressCard({
  address,
  onGenerate,
  showQR = false,
}: StealthAddressCardProps) {
  const [showQRCode, setShowQRCode] = useState(showQR)
  const [copied, setCopied] = useState(false)

  const statusConfig = {
    unused: {
      icon: Clock,
      color: "text-green-500",
      bg: "bg-green-500/10 border-green-500",
      label: "Ready to use",
    },
    used: {
      icon: CheckCircle2,
      color: "text-blue-500",
      bg: "bg-blue-500/10 border-blue-500",
      label: "Used",
    },
    expired: {
      icon: XCircle,
      color: "text-red-500",
      bg: "bg-red-500/10 border-red-500",
      label: "Expired",
    },
  }

  const config = statusConfig[address.status]
  const StatusIcon = config.icon

  const handleCopy = async () => {
    try {
      await navigator.clipboard.writeText(address.address)
      setCopied(true)
      toast.success("Address copied to clipboard")
      setTimeout(() => setCopied(false), 2000)
    } catch (error) {
      toast.error("Failed to copy address")
    }
  }

  return (
    <Card className={`border-2 ${config.bg}`}>
      <CardContent className="p-6">
        {/* Header */}
        <div className="flex items-center justify-between mb-4">
          <div className="flex items-center gap-2">
            <StatusIcon className={`w-5 h-5 ${config.color}`} />
            <div>
              <h4 className="font-semibold">
                {address.label || "Stealth Address"}
              </h4>
              <p className="text-xs text-muted-foreground">
                One-time use address
              </p>
            </div>
          </div>
          <Badge className={config.bg + " border-0"}>
            {config.label}
          </Badge>
        </div>

        {/* QR Code */}
        {showQRCode && address.status === "unused" && (
          <div className="flex justify-center mb-4 p-4 bg-white rounded-lg">
            <QRCode value={address.address} size={200} />
          </div>
        )}

        {/* Address */}
        <div className="mb-4">
          <p className="text-xs text-muted-foreground mb-2">Address</p>
          <div className="flex items-center gap-2">
            <code className="flex-1 px-3 py-2 rounded-lg bg-muted text-xs font-mono break-all">
              {address.address}
            </code>
            <Button
              variant="ghost"
              size="icon"
              className="shrink-0"
              onClick={handleCopy}
            >
              {copied ? (
                <CheckCircle2 className="w-4 h-4 text-green-500" />
              ) : (
                <Copy className="w-4 h-4" />
              )}
            </Button>
          </div>
        </div>

        {/* Metadata */}
        <div className="space-y-2 text-xs text-muted-foreground">
          <div className="flex justify-between">
            <span>Generated:</span>
            <span>{new Date(address.generatedAt).toLocaleString()}</span>
          </div>
          {address.usedAt && (
            <div className="flex justify-between">
              <span>Used:</span>
              <span>{new Date(address.usedAt).toLocaleString()}</span>
            </div>
          )}
          {address.expiresAt && address.status === "unused" && (
            <div className="flex justify-between">
              <span>Expires:</span>
              <span>{new Date(address.expiresAt).toLocaleString()}</span>
            </div>
          )}
          {address.linkedTransaction && (
            <div className="flex justify-between">
              <span>Transaction:</span>
              <code className="text-xs">
                {address.linkedTransaction.slice(0, 8)}...
              </code>
            </div>
          )}
        </div>

        {/* Actions */}
        <div className="flex gap-2 mt-4">
          {address.status === "unused" && (
            <Button
              variant="outline"
              size="sm"
              className="flex-1"
              onClick={() => setShowQRCode(!showQRCode)}
            >
              <QrCode className="w-4 h-4 mr-2" />
              {showQRCode ? "Hide" : "Show"} QR Code
            </Button>
          )}
          {onGenerate && address.status !== "unused" && (
            <Button
              variant="outline"
              size="sm"
              className="flex-1"
              onClick={onGenerate}
            >
              <RefreshCw className="w-4 h-4 mr-2" />
              Generate New
            </Button>
          )}
        </div>

        {/* Privacy Notice */}
        {address.status === "unused" && (
          <div className="mt-4 p-3 rounded-lg bg-accent/10">
            <p className="text-xs text-muted-foreground">
              <strong className="text-foreground">Privacy Tip:</strong> This
              address can only be used once. A new address will be generated
              automatically after use.
            </p>
          </div>
        )}
      </CardContent>
    </Card>
  )
}
