"use client"

import { Briefcase, Store, Home, Receipt, Vote, Lock, User } from "lucide-react"

interface BusinessMerchantNavProps {
  activeTab: string
  onTabChange: (tab: string) => void
  mode: "personal" | "business" | "merchant"
}

const personalTabs = [
  { id: "home", icon: Home, label: "Home" },
  { id: "transactions", icon: Receipt, label: "Transactions" },
  { id: "governance", icon: Vote, label: "Governance" },
  { id: "stake", icon: Lock, label: "Stake" },
  { id: "profile", icon: User, label: "Profile" },
]

const businessTabs = [
  { id: "business", icon: Briefcase, label: "Business" },
  { id: "invoices", icon: Receipt, label: "Invoices" },
  { id: "team", icon: User, label: "Team" },
  { id: "expenses", icon: Receipt, label: "Expenses" },
  { id: "home", icon: Home, label: "Home" },
]

const merchantTabs = [
  { id: "merchant", icon: Store, label: "Merchant" },
  { id: "pos", icon: Receipt, label: "POS" },
  { id: "products", icon: Receipt, label: "Products" },
  { id: "links", icon: Receipt, label: "Links" },
  { id: "home", icon: Home, label: "Home" },
]

export function BusinessMerchantNav({ activeTab, onTabChange, mode }: BusinessMerchantNavProps) {
  const tabs = mode === "business" ? businessTabs : mode === "merchant" ? merchantTabs : personalTabs

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
