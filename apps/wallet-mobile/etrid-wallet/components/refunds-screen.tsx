"use client"

import { ArrowLeft, AlertCircle } from "lucide-react"
import { Button } from "@/components/ui/button"
import { Tabs, TabsContent, TabsList, TabsTrigger } from "@/components/ui/tabs"
import { useRefunds } from "@/hooks/merchant/use-refunds"
import { RefundItem } from "@/components/merchant/RefundItem"
import type { RefundStatus } from "@/lib/types/merchant"
import { useState } from "react"

interface RefundsScreenProps {
  onBack: () => void
}

export function RefundsScreen({ onBack }: RefundsScreenProps) {
  const [filter, setFilter] = useState<RefundStatus | undefined>(undefined)
  const { refunds, loading, processRefund, approveRefund, rejectRefund } =
    useRefunds(filter)

  const handleApprove = async (refund: any) => {
    try {
      await approveRefund(refund.id)
    } catch (error) {
      console.error("Failed to approve refund:", error)
    }
  }

  const handleReject = async (refund: any) => {
    const reason = prompt("Enter rejection reason:")
    if (reason) {
      try {
        await rejectRefund(refund.id, reason)
      } catch (error) {
        console.error("Failed to reject refund:", error)
      }
    }
  }

  const handleProcess = async (refund: any) => {
    if (
      confirm(
        `Process refund of $${refund.refund_amount.toFixed(2)}?`
      )
    ) {
      try {
        await processRefund(refund.id, refund.refund_amount)
        alert("Refund processed successfully!")
      } catch (error) {
        console.error("Failed to process refund:", error)
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
            <h1 className="text-xl font-bold text-foreground">Refunds</h1>
          </div>
        </div>
      </header>

      <main className="px-4 py-6">
        <Tabs
          value={filter || "all"}
          onValueChange={(val) =>
            setFilter(val === "all" ? undefined : (val as RefundStatus))
          }
        >
          <TabsList className="w-full grid grid-cols-3 mb-6">
            <TabsTrigger value="all">All</TabsTrigger>
            <TabsTrigger value="pending">Pending</TabsTrigger>
            <TabsTrigger value="completed">Completed</TabsTrigger>
          </TabsList>

          <div className="space-y-4">
            {loading ? (
              <p className="text-center text-muted-foreground">Loading...</p>
            ) : refunds.length === 0 ? (
              <div className="glass-strong rounded-lg p-8 border border-border text-center">
                <AlertCircle className="w-12 h-12 text-muted-foreground mx-auto mb-3" />
                <p className="text-foreground font-medium mb-1">No refunds found</p>
                <p className="text-sm text-muted-foreground">
                  Refund requests will appear here
                </p>
              </div>
            ) : (
              refunds.map((refund) => (
                <RefundItem
                  key={refund.id}
                  refund={refund}
                  onApprove={handleApprove}
                  onReject={handleReject}
                  onProcess={handleProcess}
                />
              ))
            )}
          </div>
        </Tabs>
      </main>
    </div>
  )
}
