"use client"

import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card"
import { Badge } from "@/components/ui/badge"
import { Zap, Shield, Clock, DollarSign } from "lucide-react"

export function CrossChainInfo() {
  return (
    <Card>
      <CardHeader>
        <CardTitle className="flex items-center gap-2">
          <Zap className="h-5 w-5 text-yellow-500" />
          Cross-Chain Lightning Network
        </CardTitle>
      </CardHeader>
      <CardContent className="space-y-4">
        <div className="grid grid-cols-2 gap-4">
          <div className="flex items-start gap-3">
            <div className="p-2 bg-blue-100 text-blue-600 rounded-lg">
              <Clock className="h-4 w-4" />
            </div>
            <div>
              <div className="font-medium text-sm">Instant</div>
              <div className="text-xs text-muted-foreground">
                Payments settle in &lt;60 seconds
              </div>
            </div>
          </div>

          <div className="flex items-start gap-3">
            <div className="p-2 bg-green-100 text-green-600 rounded-lg">
              <DollarSign className="h-4 w-4" />
            </div>
            <div>
              <div className="font-medium text-sm">Near-Zero Fees</div>
              <div className="text-xs text-muted-foreground">
                Minimal routing fees
              </div>
            </div>
          </div>

          <div className="flex items-start gap-3">
            <div className="p-2 bg-purple-100 text-purple-600 rounded-lg">
              <Shield className="h-4 w-4" />
            </div>
            <div>
              <div className="font-medium text-sm">Atomic Swaps</div>
              <div className="text-xs text-muted-foreground">
                HTLCs ensure safety
              </div>
            </div>
          </div>

          <div className="flex items-start gap-3">
            <div className="p-2 bg-orange-100 text-orange-600 rounded-lg">
              <Zap className="h-4 w-4" />
            </div>
            <div>
              <div className="font-medium text-sm">14 Chains</div>
              <div className="text-xs text-muted-foreground">
                91 payment routes
              </div>
            </div>
          </div>
        </div>

        <div className="pt-4 border-t">
          <div className="text-sm font-medium mb-2">Supported Networks</div>
          <div className="flex flex-wrap gap-2">
            {["ETH", "BTC", "BNB", "SOL", "ADA", "TRX", "XRP", "XLM", "MATIC", "LINK", "DOGE", "USDT", "EDSC"].map((symbol) => (
              <Badge key={symbol} variant="outline">
                {symbol}
              </Badge>
            ))}
          </div>
        </div>
      </CardContent>
    </Card>
  )
}
