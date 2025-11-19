"use client"

import { Users, Calendar, DollarSign, PlayCircle, CheckCircle2, Clock } from "lucide-react"
import { Button } from "@/components/ui/button"
import type { Payroll } from "@/lib/types/business"
import { format } from "date-fns"

interface PayrollBatchProps {
  payroll: Payroll
  onExecute?: (payroll: Payroll) => void
  onView?: (payroll: Payroll) => void
}

const statusConfig = {
  pending: {
    color: "bg-yellow-500/20 text-yellow-400 border-yellow-500/30",
    icon: Clock,
    label: "Pending",
  },
  processing: {
    color: "bg-blue-500/20 text-blue-400 border-blue-500/30",
    icon: PlayCircle,
    label: "Processing",
  },
  completed: {
    color: "bg-green-500/20 text-green-400 border-green-500/30",
    icon: CheckCircle2,
    label: "Completed",
  },
  failed: {
    color: "bg-red-500/20 text-red-400 border-red-500/30",
    icon: Clock,
    label: "Failed",
  },
}

export function PayrollBatch({ payroll, onExecute, onView }: PayrollBatchProps) {
  const config = statusConfig[payroll.status]
  const StatusIcon = config.icon

  return (
    <div
      className="glass-strong rounded-lg p-4 border border-border hover:border-accent/50 transition-colors"
      onClick={() => onView?.(payroll)}
    >
      <div className="flex items-start justify-between mb-4">
        <div>
          <div className="flex items-center gap-2 mb-2">
            <h3 className="font-semibold text-foreground">
              Payroll: {format(new Date(payroll.pay_period_start), "MMM d")} -{" "}
              {format(new Date(payroll.pay_period_end), "MMM d, yyyy")}
            </h3>
          </div>

          <div className="flex items-center gap-2">
            <span className={`text-xs px-2 py-1 rounded-full border flex items-center gap-1 ${config.color}`}>
              <StatusIcon className="w-3 h-3" />
              {config.label}
            </span>
          </div>
        </div>

        {payroll.status === "pending" && onExecute && (
          <Button size="sm" onClick={(e) => {
            e.stopPropagation()
            onExecute(payroll)
          }}>
            <PlayCircle className="w-4 h-4" />
            Execute
          </Button>
        )}
      </div>

      <div className="grid grid-cols-3 gap-4">
        <div className="space-y-1">
          <div className="flex items-center gap-1 text-muted-foreground text-xs">
            <Users className="w-3 h-3" />
            <span>Employees</span>
          </div>
          <p className="text-lg font-semibold text-foreground">
            {payroll.employees.length}
          </p>
        </div>

        <div className="space-y-1">
          <div className="flex items-center gap-1 text-muted-foreground text-xs">
            <DollarSign className="w-3 h-3" />
            <span>Total Amount</span>
          </div>
          <p className="text-lg font-semibold text-foreground">
            ${payroll.total_net.toFixed(2)}
          </p>
        </div>

        {payroll.executed_at && (
          <div className="space-y-1">
            <div className="flex items-center gap-1 text-muted-foreground text-xs">
              <Calendar className="w-3 h-3" />
              <span>Executed</span>
            </div>
            <p className="text-sm font-medium text-foreground">
              {format(new Date(payroll.executed_at), "MMM d, yyyy")}
            </p>
          </div>
        )}
      </div>

      {payroll.notes && (
        <p className="mt-3 text-sm text-muted-foreground italic">
          {payroll.notes}
        </p>
      )}
    </div>
  )
}
