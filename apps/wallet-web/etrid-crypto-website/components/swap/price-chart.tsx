"use client"

import { useState } from "react"
import { Card } from "@/components/ui/card"
import { Button } from "@/components/ui/button"
import { LineChart, Line, XAxis, YAxis, Tooltip, ResponsiveContainer } from "recharts"

const timeRanges = ["1H", "24H", "7D", "30D", "All"]

const generateData = (range: string) => {
  const points = range === "1H" ? 12 : range === "24H" ? 24 : range === "7D" ? 7 : 30
  return Array.from({ length: points }, (_, i) => ({
    time: i,
    rate: 8 + Math.random() * 0.5 - 0.25,
  }))
}

export function PriceChart() {
  const [selectedRange, setSelectedRange] = useState("24H")
  const [data] = useState(generateData(selectedRange))

  return (
    <Card className="p-6 bg-card/30 backdrop-blur-xl border-border/50">
      <div className="flex items-center justify-between mb-4">
        <h3 className="text-xl font-bold">ÉTR/EDSC Rate</h3>
        <div className="flex gap-1">
          {timeRanges.map((range) => (
            <Button
              key={range}
              variant={selectedRange === range ? "default" : "ghost"}
              size="sm"
              onClick={() => setSelectedRange(range)}
              className="h-8 px-3"
            >
              {range}
            </Button>
          ))}
        </div>
      </div>

      <div className="h-64">
        <ResponsiveContainer width="100%" height="100%">
          <LineChart data={data}>
            <XAxis
              dataKey="time"
              stroke="hsl(var(--muted-foreground))"
              fontSize={12}
              tickLine={false}
              axisLine={false}
            />
            <YAxis
              stroke="hsl(var(--muted-foreground))"
              fontSize={12}
              tickLine={false}
              axisLine={false}
              domain={[7.5, 8.5]}
            />
            <Tooltip
              contentStyle={{
                backgroundColor: "hsl(var(--card))",
                border: "1px solid hsl(var(--border))",
                borderRadius: "8px",
              }}
            />
            <Line type="monotone" dataKey="rate" stroke="hsl(var(--accent))" strokeWidth={2} dot={false} />
          </LineChart>
        </ResponsiveContainer>
      </div>

      <div className="mt-4 text-center">
        <div className="text-3xl font-bold">8.00</div>
        <div className="text-sm text-muted-foreground">Current Rate</div>
      </div>
    </Card>
  )
}
