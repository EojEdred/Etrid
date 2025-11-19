"use client"

import {
  ShoppingBag,
  Utensils,
  Car,
  Plane,
  Music,
  Zap,
  MoreHorizontal,
  CheckCircle2,
  Clock,
  XCircle
} from "lucide-react"
import { Badge } from "@/components/ui/badge"
import type { BloccardTransaction, TransactionCategory } from "@/lib/types/features"

interface BloccardTransactionItemProps {
  transaction: BloccardTransaction
  onClick?: (id: string) => void
  className?: string
}

export function BloccardTransactionItem({ transaction, onClick, className = "" }: BloccardTransactionItemProps) {
  const getCategoryIcon = (category: TransactionCategory) => {
    const icons = {
      groceries: ShoppingBag,
      dining: Utensils,
      shopping: ShoppingBag,
      travel: Plane,
      entertainment: Music,
      gas: Car,
      utilities: Zap,
      other: MoreHorizontal,
    }
    return icons[category] || MoreHorizontal
  }

  const getCategoryColor = (category: TransactionCategory) => {
    const colors = {
      groceries: "bg-green-500/10 text-green-500",
      dining: "bg-orange-500/10 text-orange-500",
      shopping: "bg-purple-500/10 text-purple-500",
      travel: "bg-blue-500/10 text-blue-500",
      entertainment: "bg-pink-500/10 text-pink-500",
      gas: "bg-yellow-500/10 text-yellow-500",
      utilities: "bg-cyan-500/10 text-cyan-500",
      other: "bg-gray-500/10 text-gray-500",
    }
    return colors[category] || colors.other
  }

  const getStatusIcon = () => {
    switch (transaction.status) {
      case "completed":
        return <CheckCircle2 className="w-4 h-4 text-green-500" />
      case "pending":
        return <Clock className="w-4 h-4 text-yellow-500" />
      case "declined":
        return <XCircle className="w-4 h-4 text-red-500" />
    }
  }

  const getStatusBadge = () => {
    switch (transaction.status) {
      case "completed":
        return <Badge variant="outline" className="text-green-500 border-green-500">Completed</Badge>
      case "pending":
        return <Badge variant="outline" className="text-yellow-500 border-yellow-500">Pending</Badge>
      case "declined":
        return <Badge variant="outline" className="text-red-500 border-red-500">Declined</Badge>
    }
  }

  const Icon = getCategoryIcon(transaction.merchantCategory)

  const formatDate = (date: Date) => {
    const d = new Date(date)
    const today = new Date()
    const yesterday = new Date(today)
    yesterday.setDate(yesterday.getDate() - 1)

    if (d.toDateString() === today.toDateString()) {
      return `Today, ${d.toLocaleTimeString("en-US", { hour: "numeric", minute: "2-digit" })}`
    } else if (d.toDateString() === yesterday.toDateString()) {
      return `Yesterday, ${d.toLocaleTimeString("en-US", { hour: "numeric", minute: "2-digit" })}`
    } else {
      return d.toLocaleDateString("en-US", { month: "short", day: "numeric", hour: "numeric", minute: "2-digit" })
    }
  }

  return (
    <div
      onClick={() => onClick?.(transaction.id)}
      className={`glass p-4 rounded-2xl ${onClick ? "cursor-pointer hover:glass-strong" : ""} ${className}`}
    >
      <div className="flex items-start gap-4">
        {/* Category Icon */}
        <div className={`w-12 h-12 rounded-xl ${getCategoryColor(transaction.merchantCategory)} flex items-center justify-center shrink-0`}>
          <Icon className="w-6 h-6" />
        </div>

        {/* Transaction Info */}
        <div className="flex-1 min-w-0">
          <div className="flex items-start justify-between gap-2 mb-1">
            <div className="flex-1 min-w-0">
              <p className="font-semibold truncate">{transaction.merchant}</p>
              {transaction.description && (
                <p className="text-xs text-muted-foreground truncate">{transaction.description}</p>
              )}
            </div>
            <div className="text-right shrink-0">
              <p className="font-semibold">-${transaction.amountUsd.toFixed(2)}</p>
            </div>
          </div>

          {/* Date and Status */}
          <div className="flex items-center justify-between gap-2 mt-2">
            <div className="flex items-center gap-2">
              {getStatusIcon()}
              <p className="text-xs text-muted-foreground">{formatDate(transaction.timestamp)}</p>
            </div>
            {transaction.status !== "completed" && getStatusBadge()}
          </div>

          {/* Location */}
          {transaction.location && (
            <p className="text-xs text-muted-foreground mt-1">{transaction.location}</p>
          )}

          {/* Decline Reason */}
          {transaction.status === "declined" && transaction.declineReason && (
            <div className="mt-2 p-2 rounded-lg bg-red-500/10 border border-red-500/20">
              <p className="text-xs text-red-500">{transaction.declineReason}</p>
            </div>
          )}
        </div>
      </div>
    </div>
  )
}
