"use client"

import { Home, Receipt, Vote, Lock, User } from "lucide-react"

interface BottomNavProps {
  activeTab: string
  onTabChange: (tab: string) => void
}

const tabs = [
  { id: "home", icon: Home, label: "Home" },
  { id: "transactions", icon: Receipt, label: "Transactions" },
  { id: "governance", icon: Vote, label: "Governance" },
  { id: "stake", icon: Lock, label: "Stake" },
  { id: "profile", icon: User, label: "Profile" },
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
