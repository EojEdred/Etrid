"use client"

import { ArrowLeft, Plus, Settings } from "lucide-react"
import { Button } from "@/components/ui/button"
import { Tabs, TabsContent, TabsList, TabsTrigger } from "@/components/ui/tabs"
import { usePayroll } from "@/hooks/business/use-payroll"
import { PayrollBatch } from "@/components/business/PayrollBatch"
import { useState } from "react"

interface PayrollScreenProps {
  onBack: () => void
  onCreate: () => void
  onSettings: () => void
}

export function PayrollScreen({ onBack, onCreate, onSettings }: PayrollScreenProps) {
  const { payrolls, schedule, loading, executePayroll } = usePayroll()
  const [activeTab, setActiveTab] = useState("pending")

  const pendingPayrolls = payrolls.filter((p) => p.status === "pending")
  const completedPayrolls = payrolls.filter((p) => p.status === "completed")

  const handleExecute = async (payroll: any) => {
    if (confirm(`Execute payroll for ${payroll.employees.length} employees?`)) {
      try {
        await executePayroll(payroll.id)
      } catch (error) {
        console.error("Failed to execute payroll:", error)
      }
    }
  }

  return (
    <div className="min-h-screen pb-24">
      <header className="glass-strong border-b border-border sticky top-0 z-10">
        <div className="flex items-center justify-between p-4">
          <div className="flex items-center gap-3">
            <Button variant="ghost" size="icon" onClick={onBack}>
              <ArrowLeft className="w-5 h-5" />
            </Button>
            <h1 className="text-xl font-bold text-foreground">Payroll</h1>
          </div>
          <div className="flex items-center gap-2">
            <Button variant="outline" size="icon" onClick={onSettings}>
              <Settings className="w-4 h-4" />
            </Button>
            <Button size="sm" onClick={onCreate}>
              <Plus className="w-4 h-4" />
              New
            </Button>
          </div>
        </div>
      </header>

      <main className="px-4 py-6">
        {/* Schedule Info */}
        {schedule && (
          <div className="glass-strong rounded-lg p-4 border border-border mb-6">
            <div className="flex items-center justify-between">
              <div>
                <p className="text-sm text-muted-foreground">Payroll Schedule</p>
                <p className="text-foreground font-semibold capitalize">
                  {schedule.frequency}
                </p>
              </div>
              {schedule.auto_execute && (
                <span className="text-xs px-2 py-1 rounded-full bg-green-500/20 text-green-400 border border-green-500/30">
                  Auto-execute enabled
                </span>
              )}
            </div>
          </div>
        )}

        <Tabs value={activeTab} onValueChange={setActiveTab}>
          <TabsList className="w-full grid grid-cols-2 mb-6">
            <TabsTrigger value="pending">
              Pending ({pendingPayrolls.length})
            </TabsTrigger>
            <TabsTrigger value="history">
              History ({completedPayrolls.length})
            </TabsTrigger>
          </TabsList>

          <TabsContent value="pending" className="space-y-4">
            {loading ? (
              <p className="text-center text-muted-foreground">Loading...</p>
            ) : pendingPayrolls.length === 0 ? (
              <div className="glass-strong rounded-lg p-8 border border-border text-center">
                <p className="text-foreground font-medium mb-1">No pending payrolls</p>
                <p className="text-sm text-muted-foreground mb-4">
                  Create a new payroll batch to get started
                </p>
                <Button onClick={onCreate}>
                  <Plus className="w-4 h-4" />
                  Create Payroll
                </Button>
              </div>
            ) : (
              pendingPayrolls.map((payroll) => (
                <PayrollBatch
                  key={payroll.id}
                  payroll={payroll}
                  onExecute={handleExecute}
                />
              ))
            )}
          </TabsContent>

          <TabsContent value="history" className="space-y-4">
            {completedPayrolls.map((payroll) => (
              <PayrollBatch key={payroll.id} payroll={payroll} />
            ))}
          </TabsContent>
        </Tabs>
      </main>
    </div>
  )
}
