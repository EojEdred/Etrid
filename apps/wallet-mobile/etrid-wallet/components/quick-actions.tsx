"use client"

import { ArrowUpRight, ArrowDownLeft, ArrowLeftRight, Lock } from "lucide-react"

interface QuickActionsProps {
  onSend?: () => void
  onReceive?: () => void
}

const actions = [
  { icon: ArrowUpRight, label: "Send", color: "text-accent", action: "send" },
  { icon: ArrowDownLeft, label: "Receive", color: "text-success", action: "receive" },
  { icon: ArrowLeftRight, label: "Swap", color: "text-warning", action: "swap" },
  { icon: Lock, label: "Stake", color: "text-primary", action: "stake" },
]

export function QuickActions({ onSend, onReceive }: QuickActionsProps) {
  const handleClick = (action: string) => {
    if (action === "send" && onSend) onSend()
    if (action === "receive" && onReceive) onReceive()
  }

  return (
    <div className="grid grid-cols-4 gap-4">
      {actions.map((action) => (
        <button
          key={action.label}
          onClick={() => handleClick(action.action)}
          className="flex flex-col items-center gap-2 p-4 rounded-2xl glass hover:glass-strong transition-all"
        >
          <div className="w-12 h-12 rounded-full bg-surface flex items-center justify-center">
            <action.icon className={`w-5 h-5 ${action.color}`} />
          </div>
          <span className="text-xs font-medium">{action.label}</span>
        </button>
      ))}
    </div>
  )
}
