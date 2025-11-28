"use client"

import { ArrowLeft, Plus, FileText, Users, DollarSign, CreditCard, TrendingUp, TrendingDown } from "lucide-react"
import { Button } from "@/components/ui/button"
import { useBusiness } from "@/hooks/business/use-business"
import { useInvoices } from "@/hooks/business/use-invoices"
import { LineChart, Line, XAxis, YAxis, CartesianGrid, Tooltip, ResponsiveContainer } from "recharts"

interface BusinessDashboardScreenProps {
  onBack: () => void
  onCreateInvoice: () => void
  onRunPayroll: () => void
  onAddTeam: () => void
  onNavigate: (screen: string) => void
}

export function BusinessDashboardScreen({
  onBack,
  onCreateInvoice,
  onRunPayroll,
  onAddTeam,
  onNavigate,
}: BusinessDashboardScreenProps) {
  const { account, stats, loading } = useBusiness()
  const { invoices } = useInvoices()

  // Mock revenue data for chart
  const revenueData = [
    { month: "Jan", revenue: 4000 },
    { month: "Feb", revenue: 3000 },
    { month: "Mar", revenue: 5000 },
    { month: "Apr", revenue: 4500 },
    { month: "May", revenue: 6000 },
    { month: "Jun", revenue: 5500 },
  ]

  if (loading) {
    return (
      <div className="min-h-screen flex items-center justify-center">
        <p className="text-muted-foreground">Loading...</p>
      </div>
    )
  }

  return (
    <div className="min-h-screen pb-24">
      {/* Header */}
      <header className="glass-strong border-b border-border sticky top-0 z-10">
        <div className="flex items-center justify-between p-4">
          <div className="flex items-center gap-3">
            <Button variant="ghost" size="icon" onClick={onBack}>
              <ArrowLeft className="w-5 h-5" />
            </Button>
            <div>
              <h1 className="text-xl font-bold text-foreground">Business Dashboard</h1>
              {account && (
                <p className="text-sm text-muted-foreground">{account.name}</p>
              )}
            </div>
          </div>
        </div>
      </header>

      <main className="px-4 py-6 space-y-6">
        {/* Stats Overview */}
        <div className="grid grid-cols-2 gap-4">
          <div className="glass-strong rounded-lg p-4 border border-border">
            <div className="flex items-center justify-between mb-2">
              <p className="text-sm text-muted-foreground">Revenue</p>
              <DollarSign className="w-4 h-4 text-accent" />
            </div>
            <p className="text-2xl font-bold text-foreground">
              ${stats?.revenue_this_month.toFixed(2) || "0.00"}
            </p>
            <div className="flex items-center gap-1 mt-1">
              {(stats?.revenue_change || 0) >= 0 ? (
                <>
                  <TrendingUp className="w-3 h-3 text-green-400" />
                  <span className="text-xs text-green-400">
                    +{stats?.revenue_change.toFixed(1)}%
                  </span>
                </>
              ) : (
                <>
                  <TrendingDown className="w-3 h-3 text-red-400" />
                  <span className="text-xs text-red-400">
                    {stats?.revenue_change.toFixed(1)}%
                  </span>
                </>
              )}
            </div>
          </div>

          <div className="glass-strong rounded-lg p-4 border border-border">
            <div className="flex items-center justify-between mb-2">
              <p className="text-sm text-muted-foreground">Pending Invoices</p>
              <FileText className="w-4 h-4 text-accent" />
            </div>
            <p className="text-2xl font-bold text-foreground">
              {stats?.pending_invoices || 0}
            </p>
            <p className="text-xs text-muted-foreground mt-1">
              {stats?.invoice_count.overdue || 0} overdue
            </p>
          </div>

          <div className="glass-strong rounded-lg p-4 border border-border">
            <div className="flex items-center justify-between mb-2">
              <p className="text-sm text-muted-foreground">Team Members</p>
              <Users className="w-4 h-4 text-accent" />
            </div>
            <p className="text-2xl font-bold text-foreground">
              {stats?.team_member_count || 0}
            </p>
          </div>

          <div className="glass-strong rounded-lg p-4 border border-border">
            <div className="flex items-center justify-between mb-2">
              <p className="text-sm text-muted-foreground">Expenses</p>
              <CreditCard className="w-4 h-4 text-accent" />
            </div>
            <p className="text-2xl font-bold text-foreground">
              ${stats?.total_expenses.toFixed(2) || "0.00"}
            </p>
          </div>
        </div>

        {/* Revenue Chart */}
        <div className="glass-strong rounded-lg p-4 border border-border">
          <h3 className="text-lg font-semibold text-foreground mb-4">
            Revenue This Month
          </h3>
          <ResponsiveContainer width="100%" height={200}>
            <LineChart data={revenueData}>
              <CartesianGrid strokeDasharray="3 3" stroke="#333" />
              <XAxis dataKey="month" stroke="#888" fontSize={12} />
              <YAxis stroke="#888" fontSize={12} />
              <Tooltip
                contentStyle={{
                  backgroundColor: "rgba(0,0,0,0.8)",
                  border: "1px solid #333",
                  borderRadius: "8px",
                }}
              />
              <Line
                type="monotone"
                dataKey="revenue"
                stroke="#8b5cf6"
                strokeWidth={2}
              />
            </LineChart>
          </ResponsiveContainer>
        </div>

        {/* Quick Actions */}
        <div>
          <h3 className="text-lg font-semibold text-foreground mb-3">
            Quick Actions
          </h3>
          <div className="grid grid-cols-3 gap-3">
            <Button
              variant="outline"
              className="flex-col h-20 gap-2"
              onClick={onCreateInvoice}
            >
              <FileText className="w-5 h-5" />
              <span className="text-xs">Create Invoice</span>
            </Button>

            <Button
              variant="outline"
              className="flex-col h-20 gap-2"
              onClick={onRunPayroll}
            >
              <DollarSign className="w-5 h-5" />
              <span className="text-xs">Run Payroll</span>
            </Button>

            <Button
              variant="outline"
              className="flex-col h-20 gap-2"
              onClick={onAddTeam}
            >
              <Users className="w-5 h-5" />
              <span className="text-xs">Add Team</span>
            </Button>
          </div>
        </div>

        {/* Recent Transactions */}
        <div>
          <div className="flex items-center justify-between mb-3">
            <h3 className="text-lg font-semibold text-foreground">
              Recent Activity
            </h3>
            <Button
              variant="ghost"
              size="sm"
              onClick={() => onNavigate("invoices")}
            >
              View All
            </Button>
          </div>

          <div className="space-y-2">
            {invoices.slice(0, 5).map((invoice) => (
              <div
                key={invoice.id}
                className="glass-strong rounded-lg p-3 border border-border flex items-center justify-between"
              >
                <div>
                  <p className="text-sm font-medium text-foreground">
                    Invoice #{invoice.invoice_number}
                  </p>
                  <p className="text-xs text-muted-foreground">
                    {invoice.client_name}
                  </p>
                </div>
                <div className="text-right">
                  <p className="text-sm font-semibold text-foreground">
                    ${invoice.total.toFixed(2)}
                  </p>
                  <p className="text-xs text-muted-foreground">
                    {invoice.status}
                  </p>
                </div>
              </div>
            ))}
          </div>
        </div>
      </main>
    </div>
  )
}
