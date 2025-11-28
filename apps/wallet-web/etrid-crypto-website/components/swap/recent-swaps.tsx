"use client"

import { Card } from "@/components/ui/card"
import { Button } from "@/components/ui/button"
import { ExternalLink, ArrowRight } from "lucide-react"

const recentSwaps = [
  { from: "100 ÉTR", to: "800 EDSC", time: "2 hours ago", tx: "0x1234...5678" },
  { from: "50 EDSC", to: "6.25 ÉTR", time: "5 hours ago", tx: "0xabcd...efgh" },
  { from: "250 ÉTR", to: "2000 EDSC", time: "1 day ago", tx: "0x9876...5432" },
]

export function RecentSwaps() {
  return (
    <Card className="p-6 bg-card/30 backdrop-blur-xl border-border/50">
      <h3 className="text-xl font-bold mb-4">Your Recent Swaps</h3>

      <div className="space-y-3">
        {recentSwaps.map((swap, index) => (
          <div
            key={index}
            className="flex items-center justify-between p-3 rounded-lg bg-background/30 hover:bg-background/50 transition-colors"
          >
            <div className="flex items-center gap-3">
              <div className="flex items-center gap-2">
                <span className="font-medium">{swap.from}</span>
                <ArrowRight className="w-4 h-4 text-muted-foreground" />
                <span className="font-medium">{swap.to}</span>
              </div>
            </div>

            <div className="flex items-center gap-4">
              <span className="text-sm text-muted-foreground">{swap.time}</span>
              <Button variant="ghost" size="sm" className="gap-2">
                View TX
                <ExternalLink className="w-3 h-3" />
              </Button>
            </div>
          </div>
        ))}
      </div>
    </Card>
  )
}
