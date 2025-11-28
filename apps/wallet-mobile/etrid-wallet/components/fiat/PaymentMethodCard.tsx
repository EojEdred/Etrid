"use client"

import { CreditCard, Building2, Check, Trash2 } from "lucide-react"
import { Button } from "@/components/ui/button"
import { Badge } from "@/components/ui/badge"
import type { PaymentMethod } from "@/lib/types/features"

interface PaymentMethodCardProps {
  method: PaymentMethod
  onDelete?: (id: string) => void
  onSetDefault?: (id: string) => void
  isSelectable?: boolean
  isSelected?: boolean
  onSelect?: (id: string) => void
}

export function PaymentMethodCard({
  method,
  onDelete,
  onSetDefault,
  isSelectable = false,
  isSelected = false,
  onSelect,
}: PaymentMethodCardProps) {
  const Icon = method.type === "card" ? CreditCard : Building2

  const getProviderName = () => {
    const providers: Record<string, string> = {
      visa: "Visa",
      mastercard: "Mastercard",
      amex: "American Express",
      discover: "Discover",
      chase: "Chase Bank",
      bofa: "Bank of America",
      wells: "Wells Fargo",
    }
    return providers[method.provider.toLowerCase()] || method.provider
  }

  const handleClick = () => {
    if (isSelectable && onSelect) {
      onSelect(method.id)
    }
  }

  return (
    <div
      onClick={handleClick}
      className={`glass p-4 rounded-2xl transition-all ${
        isSelectable ? "cursor-pointer hover:glass-strong" : ""
      } ${isSelected ? "ring-2 ring-accent" : ""}`}
    >
      <div className="flex items-start gap-4">
        <div className="w-12 h-12 rounded-xl bg-accent/10 flex items-center justify-center shrink-0">
          <Icon className="w-6 h-6 text-accent" />
        </div>

        <div className="flex-1 min-w-0">
          <div className="flex items-start justify-between gap-2 mb-1">
            <div>
              <p className="font-semibold">{getProviderName()}</p>
              <p className="text-sm text-muted-foreground">
                {method.type === "card" ? `•••• ${method.last4}` : `••••${method.last4}`}
              </p>
            </div>

            {isSelected && (
              <div className="w-6 h-6 rounded-full bg-accent flex items-center justify-center">
                <Check className="w-4 h-4 text-white" />
              </div>
            )}
          </div>

          {method.expiryMonth && method.expiryYear && (
            <p className="text-xs text-muted-foreground mb-2">
              Expires {method.expiryMonth.toString().padStart(2, "0")}/{method.expiryYear.toString().slice(-2)}
            </p>
          )}

          <div className="flex items-center gap-2 flex-wrap">
            {method.isDefault && (
              <Badge variant="secondary" className="text-xs">
                Default
              </Badge>
            )}

            {method.verificationStatus === "verified" && (
              <Badge variant="outline" className="text-xs text-green-500 border-green-500">
                Verified
              </Badge>
            )}

            {method.verificationStatus === "pending" && (
              <Badge variant="outline" className="text-xs text-yellow-500 border-yellow-500">
                Pending
              </Badge>
            )}
          </div>
        </div>
      </div>

      {/* Actions (shown when not selectable) */}
      {!isSelectable && (
        <div className="flex items-center gap-2 mt-4 pt-4 border-t border-border">
          {!method.isDefault && onSetDefault && (
            <Button variant="outline" size="sm" onClick={() => onSetDefault(method.id)} className="flex-1">
              Set as Default
            </Button>
          )}

          {onDelete && (
            <Button variant="ghost" size="sm" onClick={() => onDelete(method.id)} className="text-destructive">
              <Trash2 className="w-4 h-4" />
            </Button>
          )}
        </div>
      )}
    </div>
  )
}
