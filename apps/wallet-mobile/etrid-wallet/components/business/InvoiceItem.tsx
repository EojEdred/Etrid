"use client"

import { FileText, Calendar, DollarSign, MoreVertical } from "lucide-react"
import { Button } from "@/components/ui/button"
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuTrigger,
} from "@/components/ui/dropdown-menu"
import type { Invoice } from "@/lib/types/business"
import { format } from "date-fns"

interface InvoiceItemProps {
  invoice: Invoice
  onView?: (invoice: Invoice) => void
  onSend?: (invoice: Invoice) => void
  onMarkPaid?: (invoice: Invoice) => void
  onDelete?: (invoice: Invoice) => void
}

const statusColors = {
  draft: "bg-gray-500/20 text-gray-400 border-gray-500/30",
  sent: "bg-blue-500/20 text-blue-400 border-blue-500/30",
  paid: "bg-green-500/20 text-green-400 border-green-500/30",
  overdue: "bg-red-500/20 text-red-400 border-red-500/30",
  cancelled: "bg-gray-500/20 text-gray-400 border-gray-500/30",
}

export function InvoiceItem({
  invoice,
  onView,
  onSend,
  onMarkPaid,
  onDelete,
}: InvoiceItemProps) {
  const isOverdue = invoice.status === "sent" && new Date(invoice.due_date) < new Date()

  return (
    <div
      className="glass-strong rounded-lg p-4 border border-border hover:border-accent/50 transition-colors cursor-pointer"
      onClick={() => onView?.(invoice)}
    >
      <div className="flex items-start justify-between mb-3">
        <div className="flex items-start gap-3">
          <div className="w-10 h-10 rounded-lg bg-accent/20 flex items-center justify-center">
            <FileText className="w-5 h-5 text-accent" />
          </div>

          <div>
            <div className="flex items-center gap-2 mb-1">
              <h3 className="font-semibold text-foreground">
                #{invoice.invoice_number}
              </h3>
              <span
                className={`text-xs px-2 py-0.5 rounded-full border ${
                  statusColors[isOverdue ? "overdue" : invoice.status]
                }`}
              >
                {isOverdue ? "Overdue" : invoice.status}
              </span>
            </div>
            <p className="text-sm text-muted-foreground">
              {invoice.client_name}
            </p>
          </div>
        </div>

        <DropdownMenu>
          <DropdownMenuTrigger asChild onClick={(e) => e.stopPropagation()}>
            <Button variant="ghost" size="icon-sm">
              <MoreVertical className="w-4 h-4" />
            </Button>
          </DropdownMenuTrigger>
          <DropdownMenuContent align="end">
            {onView && (
              <DropdownMenuItem onClick={() => onView(invoice)}>
                View Details
              </DropdownMenuItem>
            )}
            {onSend && invoice.status === "draft" && (
              <DropdownMenuItem onClick={() => onSend(invoice)}>
                Send Invoice
              </DropdownMenuItem>
            )}
            {onMarkPaid && invoice.status !== "paid" && (
              <DropdownMenuItem onClick={() => onMarkPaid(invoice)}>
                Mark as Paid
              </DropdownMenuItem>
            )}
            {onDelete && (
              <DropdownMenuItem
                onClick={() => onDelete(invoice)}
                className="text-destructive"
              >
                Delete Invoice
              </DropdownMenuItem>
            )}
          </DropdownMenuContent>
        </DropdownMenu>
      </div>

      <div className="flex items-center justify-between text-sm">
        <div className="flex items-center gap-4 text-muted-foreground">
          <div className="flex items-center gap-1">
            <Calendar className="w-3 h-3" />
            <span>Due {format(new Date(invoice.due_date), "MMM d, yyyy")}</span>
          </div>
        </div>

        <div className="flex items-center gap-1 font-semibold text-foreground">
          <DollarSign className="w-4 h-4" />
          <span>{invoice.total.toFixed(2)}</span>
        </div>
      </div>
    </div>
  )
}
