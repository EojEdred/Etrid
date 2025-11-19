"use client"

import { ArrowLeft, Send, CheckCircle, Download } from "lucide-react"
import { Button } from "@/components/ui/button"
import { format } from "date-fns"
import type { Invoice } from "@/lib/types/business"

interface InvoiceDetailScreenProps {
  invoice: Invoice
  onBack: () => void
  onSend?: () => void
  onMarkPaid?: () => void
  onDownload?: () => void
}

export function InvoiceDetailScreen({
  invoice,
  onBack,
  onSend,
  onMarkPaid,
  onDownload,
}: InvoiceDetailScreenProps) {
  return (
    <div className="min-h-screen pb-24">
      <header className="glass-strong border-b border-border sticky top-0 z-10">
        <div className="flex items-center justify-between p-4">
          <div className="flex items-center gap-3">
            <Button variant="ghost" size="icon" onClick={onBack}>
              <ArrowLeft className="w-5 h-5" />
            </Button>
            <h1 className="text-xl font-bold text-foreground">
              Invoice #{invoice.invoice_number}
            </h1>
          </div>
        </div>
      </header>

      <main className="px-4 py-6 space-y-6">
        {/* Invoice Header */}
        <div className="glass-strong rounded-lg p-4 border border-border">
          <div className="flex items-center justify-between mb-4">
            <div>
              <p className="text-sm text-muted-foreground">Invoice Date</p>
              <p className="text-foreground font-medium">
                {format(new Date(invoice.issued_date), "MMMM d, yyyy")}
              </p>
            </div>
            <div className="text-right">
              <p className="text-sm text-muted-foreground">Due Date</p>
              <p className="text-foreground font-medium">
                {format(new Date(invoice.due_date), "MMMM d, yyyy")}
              </p>
            </div>
          </div>

          <div className="pt-4 border-t border-border">
            <p className="text-sm text-muted-foreground mb-1">Bill To</p>
            <p className="text-foreground font-semibold">{invoice.client_name}</p>
            <p className="text-sm text-muted-foreground">{invoice.client_email}</p>
            {invoice.client_address && (
              <p className="text-sm text-muted-foreground">{invoice.client_address}</p>
            )}
          </div>
        </div>

        {/* Line Items */}
        <div className="glass-strong rounded-lg border border-border overflow-hidden">
          <div className="p-4 border-b border-border">
            <h3 className="font-semibold text-foreground">Items</h3>
          </div>

          <div className="divide-y divide-border">
            {invoice.line_items.map((item, index) => (
              <div key={index} className="p-4">
                <div className="flex items-start justify-between mb-2">
                  <div className="flex-1">
                    <p className="text-foreground font-medium">{item.description}</p>
                    <p className="text-sm text-muted-foreground">
                      {item.quantity} × ${item.unit_price.toFixed(2)}
                    </p>
                  </div>
                  <p className="text-foreground font-semibold">
                    ${item.total.toFixed(2)}
                  </p>
                </div>
              </div>
            ))}
          </div>

          {/* Totals */}
          <div className="p-4 bg-accent/5 border-t border-border space-y-2">
            <div className="flex items-center justify-between text-sm">
              <span className="text-muted-foreground">Subtotal</span>
              <span className="text-foreground font-medium">
                ${invoice.subtotal.toFixed(2)}
              </span>
            </div>
            <div className="flex items-center justify-between text-sm">
              <span className="text-muted-foreground">
                Tax ({invoice.tax_rate}%)
              </span>
              <span className="text-foreground font-medium">
                ${invoice.tax_amount.toFixed(2)}
              </span>
            </div>
            <div className="flex items-center justify-between pt-2 border-t border-border">
              <span className="text-foreground font-semibold">Total</span>
              <span className="text-xl font-bold text-accent">
                ${invoice.total.toFixed(2)}
              </span>
            </div>
          </div>
        </div>

        {/* Notes */}
        {invoice.notes && (
          <div className="glass-strong rounded-lg p-4 border border-border">
            <p className="text-sm text-muted-foreground mb-2">Notes</p>
            <p className="text-foreground">{invoice.notes}</p>
          </div>
        )}

        {/* Actions */}
        <div className="flex items-center gap-3">
          {onSend && invoice.status === "draft" && (
            <Button className="flex-1" onClick={onSend}>
              <Send className="w-4 h-4" />
              Send Invoice
            </Button>
          )}
          {onMarkPaid && invoice.status !== "paid" && (
            <Button className="flex-1" onClick={onMarkPaid}>
              <CheckCircle className="w-4 h-4" />
              Mark Paid
            </Button>
          )}
          {onDownload && (
            <Button variant="outline" onClick={onDownload}>
              <Download className="w-4 h-4" />
              PDF
            </Button>
          )}
        </div>
      </main>
    </div>
  )
}
