"use client"

import { Building2, Check, Shield } from "lucide-react"
import { Badge } from "@/components/ui/badge"
import type { PaymentMethod } from "@/lib/types/features"

interface BankAccountCardProps {
  account: PaymentMethod
  isSelected?: boolean
  onSelect?: (id: string) => void
}

export function BankAccountCard({ account, isSelected = false, onSelect }: BankAccountCardProps) {
  const getBankLogo = (provider: string) => {
    // In a real app, you'd return actual bank logos
    const colors: Record<string, string> = {
      chase: "bg-blue-500",
      bofa: "bg-red-500",
      wells: "bg-yellow-500",
      citi: "bg-blue-600",
      usbank: "bg-red-600",
    }
    return colors[provider.toLowerCase()] || "bg-accent"
  }

  const getBankName = (provider: string) => {
    const banks: Record<string, string> = {
      chase: "Chase Bank",
      bofa: "Bank of America",
      wells: "Wells Fargo",
      citi: "Citibank",
      usbank: "U.S. Bank",
    }
    return banks[provider.toLowerCase()] || provider
  }

  return (
    <div
      onClick={() => onSelect?.(account.id)}
      className={`glass p-4 rounded-2xl cursor-pointer transition-all hover:glass-strong ${
        isSelected ? "ring-2 ring-accent" : ""
      }`}
    >
      <div className="flex items-start gap-4">
        {/* Bank Logo */}
        <div className={`w-12 h-12 rounded-xl ${getBankLogo(account.provider)} flex items-center justify-center shrink-0`}>
          <Building2 className="w-6 h-6 text-white" />
        </div>

        <div className="flex-1 min-w-0">
          <div className="flex items-start justify-between gap-2 mb-1">
            <div>
              <p className="font-semibold">{getBankName(account.provider)}</p>
              <p className="text-sm text-muted-foreground">
                {account.type === "ach" ? "ACH" : "Wire"} ••••{account.last4}
              </p>
            </div>

            {isSelected && (
              <div className="w-6 h-6 rounded-full bg-accent flex items-center justify-center">
                <Check className="w-4 h-4 text-white" />
              </div>
            )}
          </div>

          <div className="flex items-center gap-2 flex-wrap mt-2">
            {account.isDefault && (
              <Badge variant="secondary" className="text-xs">
                Default
              </Badge>
            )}

            {account.verificationStatus === "verified" && (
              <Badge variant="outline" className="text-xs text-green-500 border-green-500">
                <Shield className="w-3 h-3 mr-1" />
                Verified
              </Badge>
            )}

            {account.verificationStatus === "pending" && (
              <Badge variant="outline" className="text-xs text-yellow-500 border-yellow-500">
                Pending Verification
              </Badge>
            )}

            {account.verificationStatus === "failed" && (
              <Badge variant="outline" className="text-xs text-red-500 border-red-500">
                Verification Failed
              </Badge>
            )}
          </div>
        </div>
      </div>
    </div>
  )
}
