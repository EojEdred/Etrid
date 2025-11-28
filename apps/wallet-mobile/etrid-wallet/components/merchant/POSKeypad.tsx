"use client"

import { Delete } from "lucide-react"
import { Button } from "@/components/ui/button"

interface POSKeypadProps {
  value: string
  onChange: (value: string) => void
}

export function POSKeypad({ value, onChange }: POSKeypadProps) {
  const handleNumberClick = (num: string) => {
    // Prevent multiple decimal points
    if (num === "." && value.includes(".")) return

    // Limit to 2 decimal places
    if (value.includes(".")) {
      const decimalPart = value.split(".")[1]
      if (decimalPart && decimalPart.length >= 2) return
    }

    onChange(value + num)
  }

  const handleBackspace = () => {
    onChange(value.slice(0, -1))
  }

  const handleClear = () => {
    onChange("")
  }

  const buttons = [
    "1", "2", "3",
    "4", "5", "6",
    "7", "8", "9",
    ".", "0", "⌫",
  ]

  return (
    <div className="space-y-4">
      <div className="glass-strong rounded-lg p-6 border border-border">
        <div className="text-right">
          <div className="text-sm text-muted-foreground mb-1">Amount</div>
          <div className="text-4xl font-bold text-foreground font-mono">
            ${value || "0.00"}
          </div>
        </div>
      </div>

      <div className="grid grid-cols-3 gap-3">
        {buttons.map((btn) => (
          <Button
            key={btn}
            size="lg"
            variant={btn === "⌫" ? "outline" : "default"}
            className="h-16 text-xl font-semibold"
            onClick={() => {
              if (btn === "⌫") {
                handleBackspace()
              } else {
                handleNumberClick(btn)
              }
            }}
          >
            {btn === "⌫" ? <Delete className="w-5 h-5" /> : btn}
          </Button>
        ))}
      </div>

      <Button
        variant="destructive"
        className="w-full h-12"
        onClick={handleClear}
      >
        Clear
      </Button>
    </div>
  )
}
