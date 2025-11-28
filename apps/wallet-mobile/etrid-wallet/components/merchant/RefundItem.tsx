"use client"

import { AlertCircle, CheckCircle2, Clock, XCircle, DollarSign } from "lucide-react"
import { Button } from "@/components/ui/button"
import type { Refund } from "@/lib/types/merchant"
import { format } from "date-fns"

interface RefundItemProps {
  refund: Refund
  onApprove?: (refund: Refund) => void
  onReject?: (refund: Refund) => void
  onProcess?: (refund: Refund) => void
  onView?: (refund: Refund) => void
}

const statusConfig = {
  pending: {
    color: "bg-yellow-500/20 text-yellow-400 border-yellow-500/30",
    icon: Clock,
    label: "Pending Review",
  },
  approved: {
    color: "bg-blue-500/20 text-blue-400 border-blue-500/30",
    icon: CheckCircle2,
    label: "Approved",
  },
  rejected: {
    color: "bg-red-500/20 text-red-400 border-red-500/30",
    icon: XCircle,
    label: "Rejected",
  },
  processing: {
    color: "bg-purple-500/20 text-purple-400 border-purple-500/30",
    icon: Clock,
    label: "Processing",
  },
  completed: {
    color: "bg-green-500/20 text-green-400 border-green-500/30",
    icon: CheckCircle2,
    label: "Completed",
  },
  failed: {
    color: "bg-red-500/20 text-red-400 border-red-500/30",
    icon: AlertCircle,
    label: "Failed",
  },
}

export function RefundItem({
  refund,
  onApprove,
  onReject,
  onProcess,
  onView,
}: RefundItemProps) {
  const config = statusConfig[refund.status]
  const StatusIcon = config.icon

  return (
    <div
      className="glass-strong rounded-lg p-4 border border-border hover:border-accent/50 transition-colors"
      onClick={() => onView?.(refund)}
    >
      <div className="flex items-start justify-between mb-3">
        <div>
          <div className="flex items-center gap-2 mb-2">
            <h3 className="font-semibold text-foreground">
              Refund #{refund.id.substring(0, 8)}
            </h3>
            <span
              className={`text-xs px-2 py-0.5 rounded-full border flex items-center gap-1 ${config.color}`}
            >
              <StatusIcon className="w-3 h-3" />
              {config.label}
            </span>
          </div>

          <p className="text-sm text-muted-foreground">
            Sale #{refund.sale_id.substring(0, 8)}
          </p>
        </div>

        <div className="text-right">
          <div className="flex items-center gap-1 text-lg font-bold text-foreground">
            <DollarSign className="w-4 h-4" />
            <span>{refund.refund_amount.toFixed(2)}</span>
          </div>
          <p className="text-xs text-muted-foreground">
            of ${refund.original_amount.toFixed(2)}
          </p>
        </div>
      </div>

      <div className="space-y-2 mb-4">
        <div>
          <p className="text-xs text-muted-foreground">Reason</p>
          <p className="text-sm text-foreground">{refund.reason}</p>
        </div>

        {refund.customer_notes && (
          <div>
            <p className="text-xs text-muted-foreground">Customer Notes</p>
            <p className="text-sm text-foreground italic">
              "{refund.customer_notes}"
            </p>
          </div>
        )}

        {refund.merchant_notes && (
          <div>
            <p className="text-xs text-muted-foreground">Merchant Notes</p>
            <p className="text-sm text-foreground">{refund.merchant_notes}</p>
          </div>
        )}

        <div className="flex items-center justify-between text-xs text-muted-foreground">
          <span>
            Requested {format(new Date(refund.requested_at), "MMM d, yyyy 'at' h:mm a")}
          </span>
          {refund.processed_at && (
            <span>
              Processed {format(new Date(refund.processed_at), "MMM d, yyyy")}
            </span>
          )}
        </div>
      </div>

      {refund.status === "pending" && (onApprove || onReject) && (
        <div className="flex items-center gap-2">
          {onApprove && (
            <Button
              size="sm"
              className="flex-1"
              onClick={(e) => {
                e.stopPropagation()
                onApprove(refund)
              }}
            >
              <CheckCircle2 className="w-3 h-3" />
              Approve
            </Button>
          )}
          {onReject && (
            <Button
              variant="outline"
              size="sm"
              className="flex-1"
              onClick={(e) => {
                e.stopPropagation()
                onReject(refund)
              }}
            >
              <XCircle className="w-3 h-3" />
              Reject
            </Button>
          )}
        </div>
      )}

      {refund.status === "approved" && onProcess && (
        <Button
          size="sm"
          className="w-full"
          onClick={(e) => {
            e.stopPropagation()
            onProcess(refund)
          }}
        >
          Process Refund
        </Button>
      )}
    </div>
  )
}
