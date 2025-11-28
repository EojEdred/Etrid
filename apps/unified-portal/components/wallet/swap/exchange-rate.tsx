"use client"

import { useState, useEffect } from "react"
import { RefreshCw } from "lucide-react"
import { Button } from "@/components/ui/button"
import { Card } from "@/components/ui/card"

export function ExchangeRate() {
  const [lastUpdated, setLastUpdated] = useState(2)
  const [isRefreshing, setIsRefreshing] = useState(false)

  useEffect(() => {
    const interval = setInterval(() => {
      setLastUpdated((prev) => prev + 1)
    }, 1000)
    return () => clearInterval(interval)
  }, [])

  const handleRefresh = () => {
    setIsRefreshing(true)
    setLastUpdated(0)
    setTimeout(() => setIsRefreshing(false), 500)
  }

  return (
    <Card className="p-4 bg-card/30 backdrop-blur-xl border-border/50">
      <div className="flex items-center justify-between">
        <div className="space-y-1">
          <div className="flex items-center gap-2">
            <span className="text-lg font-semibold">1 ÉTR = 8.00 EDSC</span>
            <Button variant="ghost" size="icon" onClick={handleRefresh} className="h-8 w-8">
              <RefreshCw className={`w-4 h-4 ${isRefreshing ? "animate-spin" : ""}`} />
            </Button>
          </div>
          <div className="text-sm text-muted-foreground">1 EDSC = 0.125 ÉTR</div>
        </div>

        <div className="text-sm text-muted-foreground">Updated {lastUpdated}s ago</div>
      </div>
    </Card>
  )
}
