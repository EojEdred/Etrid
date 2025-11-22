"use client"

import { ArrowLeft, Plus, FileText } from "lucide-react"
import { Button } from "@/components/ui/button"
import { Tabs, TabsContent, TabsList, TabsTrigger } from "@/components/ui/tabs"
import { useInvoices } from "@/hooks/business/use-invoices"
import { InvoiceItem } from "@/components/business/InvoiceItem"
import { useState } from "react"
import type { InvoiceStatus } from "@/lib/types/business"

interface InvoicingScreenProps {
  onBack: () => void
  onCreate: () => void
  onView: (invoice: any) => void
}

export function InvoicingScreen({ onBack, onCreate, onView }: InvoicingScreenProps) {
  const [status, setStatus] = useState<InvoiceStatus | undefined>(undefined)
  const { invoices, loading, sendInvoice, markPaid, deleteInvoice } = useInvoices(
    status ? { status } : undefined
  )

  const tabs: { value: InvoiceStatus | "all"; label: string }[] = [
    { value: "all", label: "All" },
    { value: "draft", label: "Draft" },
    { value: "sent", label: "Sent" },
    { value: "paid", label: "Paid" },
    { value: "overdue", label: "Overdue" },
  ]

  const handleTabChange = (value: string) => {
    setStatus(value === "all" ? undefined : (value as InvoiceStatus))
  }

  const handleSend = async (invoice: any) => {
    try {
      await sendInvoice(invoice.id, invoice.client_email)
    } catch (error) {
      console.error("Failed to send invoice:", error)
    }
  }

  const handleMarkPaid = async (invoice: any) => {
    try {
      await markPaid(invoice.id)
    } catch (error) {
      console.error("Failed to mark invoice as paid:", error)
    }
  }

  const handleDelete = async (invoice: any) => {
    if (confirm("Delete this invoice?")) {
      try {
        await deleteInvoice(invoice.id)
      } catch (error) {
        console.error("Failed to delete invoice:", error)
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
            <h1 className="text-xl font-bold text-foreground">Invoices</h1>
          </div>
          <Button size="sm" onClick={onCreate}>
            <Plus className="w-4 h-4" />
            Create
          </Button>
        </div>
      </header>

      <main className="px-4 py-6">
        <Tabs value={status || "all"} onValueChange={handleTabChange}>
          <TabsList className="w-full grid grid-cols-5 mb-6">
            {tabs.map((tab) => (
              <TabsTrigger key={tab.value} value={tab.value} className="text-xs">
                {tab.label}
              </TabsTrigger>
            ))}
          </TabsList>

          <div className="space-y-3">
            {loading ? (
              <p className="text-center text-muted-foreground">Loading...</p>
            ) : invoices.length === 0 ? (
              <div className="glass-strong rounded-lg p-8 border border-border text-center">
                <FileText className="w-12 h-12 text-muted-foreground mx-auto mb-3" />
                <p className="text-foreground font-medium mb-1">No invoices found</p>
                <p className="text-sm text-muted-foreground mb-4">
                  Create your first invoice to get started
                </p>
                <Button onClick={onCreate}>
                  <Plus className="w-4 h-4" />
                  Create Invoice
                </Button>
              </div>
            ) : (
              invoices.map((invoice) => (
                <InvoiceItem
                  key={invoice.id}
                  invoice={invoice}
                  onView={onView}
                  onSend={handleSend}
                  onMarkPaid={handleMarkPaid}
                  onDelete={handleDelete}
                />
              ))
            )}
          </div>
        </Tabs>
      </main>
    </div>
  )
}
