"use client"

import { Home, Receipt, Vote, Lock, User, CreditCard, Image, TrendingUp, Users } from "lucide-react"

interface BottomNavProps {
  activeTab: string
  onTabChange: (tab: string) => void
}

const tabs = [
  { id: "home", icon: Home, label: "Home" },
  { id: "savings", icon: CreditCard, label: "Savings" },
  { id: "lending", icon: TrendingUp, label: "DeFi" },
  { id: "governance", icon: Vote, label: "Govern" },
  { id: "stake", icon: Lock, label: "Stake" },
]

export function BottomNav({ activeTab, onTabChange }: BottomNavProps) {
  return (
    <nav className="fixed bottom-0 left-0 right-0 glass-strong border-t border-border">
      <div className="flex items-center justify-around px-4 py-3">
        {tabs.map((tab) => {
          const isActive = activeTab === tab.id
          return (
            <button
              key={tab.id}
              onClick={() => onTabChange(tab.id)}
              className="flex flex-col items-center gap-1 min-w-0 flex-1"
            >
              <tab.icon className={`w-5 h-5 transition-colors ${isActive ? "text-accent" : "text-muted-foreground"}`} />
              <span
                className={`text-xs transition-colors ${
                  isActive ? "text-accent font-medium" : "text-muted-foreground"
                }`}
              >
                {tab.label}
              </span>
            </button>
          )
        })}
      </div>
    </nav>
  )
}
