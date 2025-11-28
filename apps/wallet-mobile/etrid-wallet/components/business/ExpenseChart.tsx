"use client"

import { PieChart, Pie, Cell, ResponsiveContainer, Legend, Tooltip } from "recharts"
import type { CategoryBreakdown } from "@/lib/types/business"

interface ExpenseChartProps {
  data: CategoryBreakdown[]
}

const COLORS = [
  "#8b5cf6", // purple
  "#3b82f6", // blue
  "#10b981", // green
  "#f59e0b", // amber
  "#ef4444", // red
  "#ec4899", // pink
  "#06b6d4", // cyan
  "#6366f1", // indigo
]

const categoryLabels: Record<string, string> = {
  office: "Office",
  travel: "Travel",
  software: "Software",
  marketing: "Marketing",
  equipment: "Equipment",
  utilities: "Utilities",
  professional_services: "Professional Services",
  other: "Other",
}

export function ExpenseChart({ data }: ExpenseChartProps) {
  const chartData = data.map((item) => ({
    name: categoryLabels[item.category] || item.category,
    value: item.total,
    percentage: item.percentage,
  }))

  const CustomTooltip = ({ active, payload }: any) => {
    if (active && payload && payload.length) {
      return (
        <div className="glass-strong border border-border rounded-lg p-3">
          <p className="text-sm font-semibold text-foreground">
            {payload[0].name}
          </p>
          <p className="text-sm text-accent">
            ${payload[0].value.toFixed(2)}
          </p>
          <p className="text-xs text-muted-foreground">
            {payload[0].payload.percentage.toFixed(1)}%
          </p>
        </div>
      )
    }
    return null
  }

  if (data.length === 0) {
    return (
      <div className="glass-strong rounded-lg p-6 border border-border">
        <p className="text-center text-muted-foreground">
          No expense data available
        </p>
      </div>
    )
  }

  return (
    <div className="glass-strong rounded-lg p-4 border border-border">
      <h3 className="text-lg font-semibold text-foreground mb-4">
        Expenses by Category
      </h3>

      <ResponsiveContainer width="100%" height={300}>
        <PieChart>
          <Pie
            data={chartData}
            cx="50%"
            cy="50%"
            labelLine={false}
            outerRadius={100}
            fill="#8884d8"
            dataKey="value"
            label={({ percentage }) => `${percentage.toFixed(0)}%`}
          >
            {chartData.map((entry, index) => (
              <Cell
                key={`cell-${index}`}
                fill={COLORS[index % COLORS.length]}
              />
            ))}
          </Pie>
          <Tooltip content={<CustomTooltip />} />
          <Legend />
        </PieChart>
      </ResponsiveContainer>

      <div className="mt-4 space-y-2">
        {data.map((item, index) => (
          <div
            key={item.category}
            className="flex items-center justify-between text-sm"
          >
            <div className="flex items-center gap-2">
              <div
                className="w-3 h-3 rounded-full"
                style={{ backgroundColor: COLORS[index % COLORS.length] }}
              />
              <span className="text-muted-foreground">
                {categoryLabels[item.category] || item.category}
              </span>
            </div>
            <div className="flex items-center gap-3">
              <span className="text-foreground font-medium">
                ${item.total.toFixed(2)}
              </span>
              <span className="text-muted-foreground text-xs">
                {item.count} items
              </span>
            </div>
          </div>
        ))}
      </div>
    </div>
  )
}
