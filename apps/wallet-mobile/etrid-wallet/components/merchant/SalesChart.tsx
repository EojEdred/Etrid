"use client"

import {
  LineChart,
  Line,
  BarChart,
  Bar,
  XAxis,
  YAxis,
  CartesianGrid,
  Tooltip,
  ResponsiveContainer,
  Legend,
} from "recharts"
import { useState } from "react"
import { Button } from "@/components/ui/button"
import type { SalesStats } from "@/lib/types/merchant"

interface SalesChartProps {
  stats: SalesStats
}

type ChartType = "line" | "bar"
type TimePeriod = "day" | "week" | "month" | "year"

export function SalesChart({ stats }: SalesChartProps) {
  const [chartType, setChartType] = useState<ChartType>("line")

  const chartData = stats.sales_by_day.map((item) => ({
    date: new Date(item.date).toLocaleDateString("en-US", {
      month: "short",
      day: "numeric",
    }),
    sales: item.sales,
    transactions: item.transactions,
  }))

  const CustomTooltip = ({ active, payload }: any) => {
    if (active && payload && payload.length) {
      return (
        <div className="glass-strong border border-border rounded-lg p-3">
          <p className="text-sm font-semibold text-foreground mb-2">
            {payload[0].payload.date}
          </p>
          <div className="space-y-1">
            <div className="flex items-center gap-2">
              <div className="w-3 h-3 rounded-full bg-accent" />
              <span className="text-sm text-muted-foreground">Sales:</span>
              <span className="text-sm font-medium text-foreground">
                ${payload[0].value.toFixed(2)}
              </span>
            </div>
            <div className="flex items-center gap-2">
              <div className="w-3 h-3 rounded-full bg-blue-500" />
              <span className="text-sm text-muted-foreground">
                Transactions:
              </span>
              <span className="text-sm font-medium text-foreground">
                {payload[1]?.value || 0}
              </span>
            </div>
          </div>
        </div>
      )
    }
    return null
  }

  if (chartData.length === 0) {
    return (
      <div className="glass-strong rounded-lg p-6 border border-border">
        <p className="text-center text-muted-foreground">
          No sales data available
        </p>
      </div>
    )
  }

  return (
    <div className="glass-strong rounded-lg p-4 border border-border">
      <div className="flex items-center justify-between mb-4">
        <h3 className="text-lg font-semibold text-foreground">Sales Trend</h3>

        <div className="flex items-center gap-2">
          <Button
            variant={chartType === "line" ? "default" : "outline"}
            size="sm"
            onClick={() => setChartType("line")}
          >
            Line
          </Button>
          <Button
            variant={chartType === "bar" ? "default" : "outline"}
            size="sm"
            onClick={() => setChartType("bar")}
          >
            Bar
          </Button>
        </div>
      </div>

      <ResponsiveContainer width="100%" height={300}>
        {chartType === "line" ? (
          <LineChart data={chartData}>
            <CartesianGrid strokeDasharray="3 3" stroke="#333" />
            <XAxis
              dataKey="date"
              stroke="#888"
              fontSize={12}
              tickLine={false}
            />
            <YAxis stroke="#888" fontSize={12} tickLine={false} />
            <Tooltip content={<CustomTooltip />} />
            <Legend />
            <Line
              type="monotone"
              dataKey="sales"
              stroke="#8b5cf6"
              strokeWidth={2}
              dot={{ fill: "#8b5cf6", r: 4 }}
              activeDot={{ r: 6 }}
              name="Sales ($)"
            />
            <Line
              type="monotone"
              dataKey="transactions"
              stroke="#3b82f6"
              strokeWidth={2}
              dot={{ fill: "#3b82f6", r: 4 }}
              activeDot={{ r: 6 }}
              name="Transactions"
            />
          </LineChart>
        ) : (
          <BarChart data={chartData}>
            <CartesianGrid strokeDasharray="3 3" stroke="#333" />
            <XAxis
              dataKey="date"
              stroke="#888"
              fontSize={12}
              tickLine={false}
            />
            <YAxis stroke="#888" fontSize={12} tickLine={false} />
            <Tooltip content={<CustomTooltip />} />
            <Legend />
            <Bar dataKey="sales" fill="#8b5cf6" name="Sales ($)" />
            <Bar dataKey="transactions" fill="#3b82f6" name="Transactions" />
          </BarChart>
        )}
      </ResponsiveContainer>

      <div className="mt-4 grid grid-cols-3 gap-4">
        <div className="text-center">
          <p className="text-xs text-muted-foreground mb-1">Total Sales</p>
          <p className="text-lg font-bold text-foreground">
            ${stats.total_sales.toFixed(2)}
          </p>
          <p
            className={`text-xs ${
              stats.sales_change >= 0 ? "text-green-400" : "text-red-400"
            }`}
          >
            {stats.sales_change >= 0 ? "+" : ""}
            {stats.sales_change.toFixed(1)}%
          </p>
        </div>

        <div className="text-center">
          <p className="text-xs text-muted-foreground mb-1">Transactions</p>
          <p className="text-lg font-bold text-foreground">
            {stats.transaction_count}
          </p>
          <p
            className={`text-xs ${
              stats.transaction_change >= 0 ? "text-green-400" : "text-red-400"
            }`}
          >
            {stats.transaction_change >= 0 ? "+" : ""}
            {stats.transaction_change.toFixed(1)}%
          </p>
        </div>

        <div className="text-center">
          <p className="text-xs text-muted-foreground mb-1">Average Sale</p>
          <p className="text-lg font-bold text-foreground">
            ${stats.average_sale.toFixed(2)}
          </p>
        </div>
      </div>
    </div>
  )
}
