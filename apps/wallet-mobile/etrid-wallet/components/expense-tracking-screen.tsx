"use client"

import { ArrowLeft, Plus, Download, Filter } from "lucide-react"
import { Button } from "@/components/ui/button"
import { useExpenses } from "@/hooks/business/use-expenses"
import { ExpenseChart } from "@/components/business/ExpenseChart"
import { format } from "date-fns"
import type { ExpenseCategory } from "@/lib/types/business"

interface ExpenseTrackingScreenProps {
  onBack: () => void
  onAdd: () => void
}

export function ExpenseTrackingScreen({ onBack, onAdd }: ExpenseTrackingScreenProps) {
  const { expenses, categoryBreakdown, loading, exportToCSV } = useExpenses()

  const handleExport = async () => {
    try {
      await exportToCSV()
    } catch (error) {
      console.error("Failed to export expenses:", error)
    }
  }

  const categoryLabels: Record<ExpenseCategory, string> = {
    office: "Office",
    travel: "Travel",
    software: "Software",
    marketing: "Marketing",
    equipment: "Equipment",
    utilities: "Utilities",
    professional_services: "Professional Services",
    other: "Other",
  }

  return (
    <div className="min-h-screen pb-24">
      <header className="glass-strong border-b border-border sticky top-0 z-10">
        <div className="flex items-center justify-between p-4">
          <div className="flex items-center gap-3">
            <Button variant="ghost" size="icon" onClick={onBack}>
              <ArrowLeft className="w-5 h-5" />
            </Button>
            <h1 className="text-xl font-bold text-foreground">Expenses</h1>
          </div>
          <div className="flex items-center gap-2">
            <Button variant="outline" size="icon" onClick={handleExport}>
              <Download className="w-4 h-4" />
            </Button>
            <Button size="sm" onClick={onAdd}>
              <Plus className="w-4 h-4" />
              Add
            </Button>
          </div>
        </div>
      </header>

      <main className="px-4 py-6 space-y-6">
        {/* Category Chart */}
        <ExpenseChart data={categoryBreakdown} />

        {/* Recent Expenses */}
        <div>
          <div className="flex items-center justify-between mb-3">
            <h3 className="text-lg font-semibold text-foreground">Recent Expenses</h3>
            <Button variant="ghost" size="sm">
              <Filter className="w-4 h-4" />
              Filter
            </Button>
          </div>

          <div className="space-y-2">
            {loading ? (
              <p className="text-center text-muted-foreground">Loading...</p>
            ) : expenses.length === 0 ? (
              <div className="glass-strong rounded-lg p-8 border border-border text-center">
                <p className="text-foreground font-medium mb-1">No expenses yet</p>
                <p className="text-sm text-muted-foreground mb-4">
                  Add your first expense to start tracking
                </p>
                <Button onClick={onAdd}>
                  <Plus className="w-4 h-4" />
                  Add Expense
                </Button>
              </div>
            ) : (
              expenses.map((expense) => (
                <div
                  key={expense.id}
                  className="glass-strong rounded-lg p-4 border border-border"
                >
                  <div className="flex items-start justify-between mb-2">
                    <div className="flex-1">
                      <div className="flex items-center gap-2 mb-1">
                        <p className="font-medium text-foreground">
                          {expense.description}
                        </p>
                        <span className="text-xs px-2 py-0.5 rounded-full bg-accent/20 text-accent border border-accent/30">
                          {categoryLabels[expense.category]}
                        </span>
                      </div>
                      <p className="text-xs text-muted-foreground">
                        {format(new Date(expense.date), "MMM d, yyyy")}
                      </p>
                      {expense.team_member_name && (
                        <p className="text-xs text-muted-foreground">
                          By {expense.team_member_name}
                        </p>
                      )}
                    </div>
                    <div className="text-right">
                      <p className="text-lg font-bold text-foreground">
                        ${expense.amount.toFixed(2)}
                      </p>
                      {expense.reimbursable && !expense.reimbursed && (
                        <span className="text-xs text-yellow-400">
                          Pending reimbursement
                        </span>
                      )}
                    </div>
                  </div>
                </div>
              ))
            )}
          </div>
        </div>
      </main>
    </div>
  )
}
