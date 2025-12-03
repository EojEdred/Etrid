"use client"

import { useState, useEffect } from "react"
import { Card } from "@/components/ui/card"
import { Button } from "@/components/ui/button"
import { LineChart, Line, XAxis, YAxis, Tooltip, ResponsiveContainer } from "recharts"
import { Network, usePriceHistory, useNetworkSelection, WETR_TOKEN, formatTokenAmount } from "@/lib/dex"
import { Loader2 } from "lucide-react"

const timeRanges = ["1H", "24H", "7D", "30D", "All"] as const
type TimeRange = (typeof timeRanges)[number]

interface PriceChartProps {
  network?: Network
}

export function PriceChart({ network: propNetwork }: PriceChartProps = {}) {
  const { network: defaultNetwork } = useNetworkSelection()
  const network = propNetwork || defaultNetwork

  const [selectedRange, setSelectedRange] = useState<TimeRange>("24H")

  const wetrToken = WETR_TOKEN[network]
  const { priceData, loading, error, refetch } = usePriceHistory(network, wetrToken.address, selectedRange)

  // Refetch when range changes
  useEffect(() => {
    refetch()
  }, [selectedRange, refetch])

  // Format chart data
  const chartData = priceData.map((item) => ({
    timestamp: new Date(item.timestamp).toLocaleTimeString('en-US', {
      hour: '2-digit',
      minute: selectedRange === '1H' ? '2-digit' : undefined,
      month: selectedRange === '7D' || selectedRange === '30D' || selectedRange === 'All' ? 'short' : undefined,
      day: selectedRange === '7D' || selectedRange === '30D' || selectedRange === 'All' ? 'numeric' : undefined,
    }),
    price: item.price,
  }))

  const currentPrice = priceData.length > 0 ? priceData[priceData.length - 1].price : 8.0
  const firstPrice = priceData.length > 0 ? priceData[0].price : 8.0
  const priceChange = ((currentPrice - firstPrice) / firstPrice) * 100

  // Calculate price domain with padding
  const prices = priceData.map((d) => d.price)
  const minPrice = Math.min(...prices)
  const maxPrice = Math.max(...prices)
  const padding = (maxPrice - minPrice) * 0.1
  const domain = [minPrice - padding, maxPrice + padding]

  return (
    <Card className="p-6 bg-card/30 backdrop-blur-xl border-border/50">
      <div className="flex items-center justify-between mb-4">
        <div>
          <h3 className="text-xl font-bold">wETR Price Chart</h3>
          <p className="text-sm text-muted-foreground">
            {network.charAt(0).toUpperCase() + network.slice(1)}
          </p>
        </div>
        <div className="flex gap-1">
          {timeRanges.map((range) => (
            <Button
              key={range}
              variant={selectedRange === range ? "default" : "ghost"}
              size="sm"
              onClick={() => setSelectedRange(range)}
              className="h-8 px-3"
              disabled={loading}
            >
              {range}
            </Button>
          ))}
        </div>
      </div>

      {loading && (
        <div className="h-64 flex items-center justify-center">
          <Loader2 className="w-8 h-8 animate-spin text-muted-foreground" />
        </div>
      )}

      {error && (
        <div className="h-64 flex items-center justify-center">
          <p className="text-sm text-destructive">Failed to load price data</p>
        </div>
      )}

      {!loading && !error && chartData.length > 0 && (
        <>
          <div className="h-64">
            <ResponsiveContainer width="100%" height="100%">
              <LineChart data={chartData}>
                <XAxis
                  dataKey="timestamp"
                  stroke="hsl(var(--muted-foreground))"
                  fontSize={12}
                  tickLine={false}
                  axisLine={false}
                  interval="preserveStartEnd"
                />
                <YAxis
                  stroke="hsl(var(--muted-foreground))"
                  fontSize={12}
                  tickLine={false}
                  axisLine={false}
                  domain={domain}
                  tickFormatter={(value) => `$${value.toFixed(2)}`}
                />
                <Tooltip
                  contentStyle={{
                    backgroundColor: "hsl(var(--card))",
                    border: "1px solid hsl(var(--border))",
                    borderRadius: "8px",
                  }}
                  formatter={(value: number) => [`$${value.toFixed(4)}`, 'Price']}
                  labelFormatter={(label) => `Time: ${label}`}
                />
                <Line
                  type="monotone"
                  dataKey="price"
                  stroke={priceChange >= 0 ? "hsl(var(--success))" : "hsl(var(--destructive))"}
                  strokeWidth={2}
                  dot={false}
                  activeDot={{ r: 6 }}
                />
              </LineChart>
            </ResponsiveContainer>
          </div>

          <div className="mt-4 text-center">
            <div className="text-3xl font-bold">${formatTokenAmount(currentPrice.toString())}</div>
            <div className="flex items-center justify-center gap-2 text-sm mt-1">
              <span className="text-muted-foreground">Current Price</span>
              <span className={priceChange >= 0 ? "text-green-500" : "text-red-500"}>
                {priceChange >= 0 ? "+" : ""}
                {priceChange.toFixed(2)}%
              </span>
            </div>
          </div>
        </>
      )}
    </Card>
  )
}
