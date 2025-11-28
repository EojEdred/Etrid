"use client"

import { ArrowUpRight, ArrowDownLeft, Check, Clock } from "lucide-react"

const transactions = [
  {
    id: 1,
    type: "sent",
    description: "Sent to 0x1a2b...4f5g",
    amount: "-50.00 ÉTR",
    timestamp: "2 hours ago",
    status: "confirmed",
  },
  {
    id: 2,
    type: "received",
    description: "Received from 0x9c8d...2e1f",
    amount: "+125.50 ÉTR",
    timestamp: "5 hours ago",
    status: "confirmed",
  },
  {
    id: 3,
    type: "sent",
    description: "Sent to 0x3f4e...8a9b",
    amount: "-30.00 EDSC",
    timestamp: "1 day ago",
    status: "pending",
  },
  {
    id: 4,
    type: "received",
    description: "Staking reward",
    amount: "+12.45 ÉTR",
    timestamp: "2 days ago",
    status: "confirmed",
  },
]

export function RecentTransactions() {
  return (
    <div className="space-y-4">
      <div className="flex items-center justify-between">
        <h2 className="text-xl font-bold">Recent Activity</h2>
        <button className="text-sm text-accent hover:underline">See All</button>
      </div>

      <div className="space-y-3">
        {transactions.map((tx) => (
          <div key={tx.id} className="glass rounded-2xl p-4 hover:glass-strong transition-all">
            <div className="flex items-center gap-4">
              {/* Icon */}
              <div
                className={`w-10 h-10 rounded-full flex items-center justify-center ${
                  tx.type === "sent" ? "bg-error/20" : "bg-success/20"
                }`}
              >
                {tx.type === "sent" ? (
                  <ArrowUpRight className={`w-5 h-5 ${tx.type === "sent" ? "text-error" : "text-success"}`} />
                ) : (
                  <ArrowDownLeft className="w-5 h-5 text-success" />
                )}
              </div>

              {/* Details */}
              <div className="flex-1 min-w-0">
                <p className="font-medium truncate">{tx.description}</p>
                <p className="text-xs text-muted-foreground">{tx.timestamp}</p>
              </div>

              {/* Amount & Status */}
              <div className="text-right">
                <p className={`font-semibold ${tx.amount.startsWith("+") ? "text-success" : "text-foreground"}`}>
                  {tx.amount}
                </p>
                <div className="flex items-center justify-end gap-1 mt-1">
                  {tx.status === "confirmed" ? (
                    <>
                      <Check className="w-3 h-3 text-success" />
                      <span className="text-xs text-success">Confirmed</span>
                    </>
                  ) : (
                    <>
                      <Clock className="w-3 h-3 text-warning" />
                      <span className="text-xs text-warning">Pending</span>
                    </>
                  )}
                </div>
              </div>
            </div>
          </div>
        ))}
      </div>
    </div>
  )
}
