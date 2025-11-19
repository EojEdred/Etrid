"use client"

import { Link, Copy, Share2, QrCode, DollarSign, Calendar, MoreVertical } from "lucide-react"
import { Button } from "@/components/ui/button"
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuTrigger,
} from "@/components/ui/dropdown-menu"
import QRCode from "qrcode.react"
import type { PaymentLink } from "@/lib/types/merchant"
import { format } from "date-fns"
import { useState } from "react"

interface PaymentLinkCardProps {
  link: PaymentLink
  onCopy?: (link: PaymentLink) => void
  onShare?: (link: PaymentLink) => void
  onDeactivate?: (link: PaymentLink) => void
  onDelete?: (link: PaymentLink) => void
  getUrl: (linkCode: string) => string
}

const statusColors = {
  active: "bg-green-500/20 text-green-400 border-green-500/30",
  paid: "bg-blue-500/20 text-blue-400 border-blue-500/30",
  expired: "bg-gray-500/20 text-gray-400 border-gray-500/30",
  cancelled: "bg-red-500/20 text-red-400 border-red-500/30",
}

export function PaymentLinkCard({
  link,
  onCopy,
  onShare,
  onDeactivate,
  onDelete,
  getUrl,
}: PaymentLinkCardProps) {
  const [showQR, setShowQR] = useState(false)
  const linkUrl = getUrl(link.link_code)

  const truncateUrl = (url: string, maxLength: number = 40) => {
    if (url.length <= maxLength) return url
    return url.substring(0, maxLength - 3) + "..."
  }

  return (
    <div className="glass-strong rounded-lg p-4 border border-border">
      <div className="flex items-start justify-between mb-4">
        <div className="flex-1">
          <div className="flex items-center gap-2 mb-2">
            <div className="w-8 h-8 rounded-lg bg-accent/20 flex items-center justify-center">
              <Link className="w-4 h-4 text-accent" />
            </div>
            <h3 className="font-semibold text-foreground truncate">
              {link.description}
            </h3>
          </div>

          <div className="flex items-center gap-2">
            <span className={`text-xs px-2 py-0.5 rounded-full border ${statusColors[link.status]}`}>
              {link.status}
            </span>
            {link.reusable && (
              <span className="text-xs px-2 py-0.5 rounded-full bg-purple-500/20 text-purple-400 border border-purple-500/30">
                Reusable
              </span>
            )}
          </div>
        </div>

        <DropdownMenu>
          <DropdownMenuTrigger asChild>
            <Button variant="ghost" size="icon-sm">
              <MoreVertical className="w-4 h-4" />
            </Button>
          </DropdownMenuTrigger>
          <DropdownMenuContent align="end">
            <DropdownMenuItem onClick={() => setShowQR(!showQR)}>
              <QrCode className="w-4 h-4 mr-2" />
              {showQR ? "Hide" : "Show"} QR Code
            </DropdownMenuItem>
            {onCopy && (
              <DropdownMenuItem onClick={() => onCopy(link)}>
                <Copy className="w-4 h-4 mr-2" />
                Copy Link
              </DropdownMenuItem>
            )}
            {onShare && (
              <DropdownMenuItem onClick={() => onShare(link)}>
                <Share2 className="w-4 h-4 mr-2" />
                Share
              </DropdownMenuItem>
            )}
            {onDeactivate && link.status === "active" && (
              <DropdownMenuItem onClick={() => onDeactivate(link)}>
                Deactivate
              </DropdownMenuItem>
            )}
            {onDelete && (
              <DropdownMenuItem
                onClick={() => onDelete(link)}
                className="text-destructive"
              >
                Delete
              </DropdownMenuItem>
            )}
          </DropdownMenuContent>
        </DropdownMenu>
      </div>

      {showQR && (
        <div className="mb-4 p-4 bg-white rounded-lg flex justify-center">
          <QRCode value={linkUrl} size={200} />
        </div>
      )}

      <div className="space-y-2 mb-4">
        <div className="flex items-center gap-2 text-sm">
          <Link className="w-3 h-3 text-muted-foreground" />
          <span className="text-muted-foreground truncate">
            {truncateUrl(linkUrl)}
          </span>
        </div>

        {link.amount && (
          <div className="flex items-center gap-2 text-sm">
            <DollarSign className="w-3 h-3 text-muted-foreground" />
            <span className="text-foreground font-medium">${link.amount.toFixed(2)}</span>
          </div>
        )}

        {link.expires_at && (
          <div className="flex items-center gap-2 text-sm">
            <Calendar className="w-3 h-3 text-muted-foreground" />
            <span className="text-muted-foreground">
              Expires {format(new Date(link.expires_at), "MMM d, yyyy")}
            </span>
          </div>
        )}
      </div>

      <div className="flex items-center gap-2">
        {onCopy && (
          <Button
            variant="outline"
            size="sm"
            className="flex-1"
            onClick={() => onCopy(link)}
          >
            <Copy className="w-3 h-3" />
            Copy
          </Button>
        )}
        {onShare && (
          <Button
            variant="outline"
            size="sm"
            className="flex-1"
            onClick={() => onShare(link)}
          >
            <Share2 className="w-3 h-3" />
            Share
          </Button>
        )}
      </div>

      {link.max_uses && (
        <div className="mt-3 pt-3 border-t border-border">
          <p className="text-xs text-muted-foreground">
            Used {link.use_count} of {link.max_uses} times
          </p>
        </div>
      )}
    </div>
  )
}
