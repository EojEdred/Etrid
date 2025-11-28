"use client"

import { Shield, Users, Clock } from "lucide-react"
import { MultiSigWallet } from "@/lib/types/multisig"
import { Card, CardContent } from "@/components/ui/card"
import { Badge } from "@/components/ui/badge"
import { Avatar, AvatarFallback } from "@/components/ui/avatar"

interface MultiSigCardProps {
  wallet: MultiSigWallet
  onClick?: () => void
}

export function MultiSigCard({ wallet, onClick }: MultiSigCardProps) {
  const purposeColors = {
    personal: "bg-blue-500/10 text-blue-500",
    couples: "bg-pink-500/10 text-pink-500",
    business: "bg-purple-500/10 text-purple-500",
    dao: "bg-green-500/10 text-green-500",
  }

  const purposeIcons = {
    personal: "👤",
    couples: "💑",
    business: "🏢",
    dao: "🏛️",
  }

  return (
    <Card
      className="cursor-pointer hover:shadow-lg transition-all duration-200 hover:scale-[1.02]"
      onClick={onClick}
    >
      <CardContent className="p-6">
        {/* Header */}
        <div className="flex items-start justify-between mb-4">
          <div className="flex items-center gap-3">
            <div className="w-12 h-12 rounded-full bg-gradient-to-br from-accent/20 to-accent/10 flex items-center justify-center text-2xl">
              {purposeIcons[wallet.purpose]}
            </div>
            <div>
              <h3 className="font-semibold text-lg">{wallet.name}</h3>
              <p className="text-xs text-muted-foreground capitalize">
                {wallet.purpose}
              </p>
            </div>
          </div>
          {wallet.pendingApprovals > 0 && (
            <Badge variant="destructive" className="animate-pulse">
              {wallet.pendingApprovals} pending
            </Badge>
          )}
        </div>

        {/* Threshold Badge */}
        <div className="flex items-center gap-2 mb-4">
          <Badge className={purposeColors[wallet.purpose]}>
            <Shield className="w-3 h-3 mr-1" />
            {wallet.threshold}-of-{wallet.totalSigners}
          </Badge>
          <Badge variant="outline">
            <Users className="w-3 h-3 mr-1" />
            {wallet.totalSigners} signers
          </Badge>
        </div>

        {/* Balance */}
        <div className="mb-4">
          <p className="text-sm text-muted-foreground mb-1">Total Balance</p>
          <p className="text-2xl font-bold">
            {parseFloat(wallet.balance).toLocaleString()} ÉTR
          </p>
        </div>

        {/* Signers Avatars */}
        <div className="flex items-center gap-2">
          <div className="flex -space-x-2">
            {wallet.signers.slice(0, 5).map((signer, i) => (
              <Avatar key={i} className="border-2 border-background w-8 h-8">
                <AvatarFallback className="text-xs">
                  {signer.slice(0, 2).toUpperCase()}
                </AvatarFallback>
              </Avatar>
            ))}
            {wallet.signers.length > 5 && (
              <Avatar className="border-2 border-background w-8 h-8">
                <AvatarFallback className="text-xs">
                  +{wallet.signers.length - 5}
                </AvatarFallback>
              </Avatar>
            )}
          </div>
          <p className="text-xs text-muted-foreground ml-2">
            {wallet.signers.length} signers
          </p>
        </div>

        {/* Created Date */}
        <div className="mt-4 pt-4 border-t flex items-center gap-2 text-xs text-muted-foreground">
          <Clock className="w-3 h-3" />
          Created {new Date(wallet.createdAt).toLocaleDateString()}
        </div>
      </CardContent>
    </Card>
  )
}
