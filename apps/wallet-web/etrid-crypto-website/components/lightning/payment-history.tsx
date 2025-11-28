"use client"

import { Card, CardContent, CardDescription, CardHeader, CardTitle } from "@/components/ui/card"
import { Badge } from "@/components/ui/badge"
import { ScrollArea } from "@/components/ui/scroll-area"
import { ArrowDownLeft, ArrowUpRight } from "lucide-react"
import type { Payment } from "@/lib/lightning/types"
import { SUPPORTED_CHAINS } from "@/lib/lightning/types"

interface PaymentHistoryProps {
  payments: Payment[]
}

export function PaymentHistory({ payments }: PaymentHistoryProps) {
  if (payments.length === 0) {
    return (
      <Card>
        <CardHeader>
          <CardTitle>Payment History</CardTitle>
          <CardDescription>No payments yet</CardDescription>
        </CardHeader>
      </Card>
    )
  }

  return (
    <Card>
      <CardHeader>
        <CardTitle>Payment History</CardTitle>
        <CardDescription>Recent Lightning transactions</CardDescription>
      </CardHeader>
      <CardContent>
        <ScrollArea className="h-[500px]">
          <div className="space-y-3">
            {payments.map((payment) => {
              const sourceChain = SUPPORTED_CHAINS.find(c => c.id === payment.sourceChain)
              const destChain = SUPPORTED_CHAINS.find(c => c.id === payment.destChain)
              const isSend = payment.type === "send"

              return (
                <div
                  key={payment.id}
                  className="p-4 border rounded-lg space-y-3"
                >
                  <div className="flex items-center justify-between">
                    <div className="flex items-center gap-3">
                      <div className={`p-2 rounded-full ${
                        isSend ? "bg-red-100 text-red-600" : "bg-green-100 text-green-600"
                      }`}>
                        {isSend ? (
                          <ArrowUpRight className="h-4 w-4" />
                        ) : (
                          <ArrowDownLeft className="h-4 w-4" />
                        )}
                      </div>
                      <div>
                        <div className="font-medium">
                          {isSend ? "Sent" : "Received"}
                        </div>
                        <div className="text-sm text-muted-foreground">
                          {sourceChain?.name} → {destChain?.name}
                        </div>
                      </div>
                    </div>
                    <Badge variant={
                      payment.status === "completed" ? "default" :
                      payment.status === "pending" ? "secondary" :
                      "destructive"
                    }>
                      {payment.status}
                    </Badge>
                  </div>

                  <div className="grid grid-cols-2 gap-4 text-sm">
                    <div>
                      <div className="text-muted-foreground">Amount</div>
                      <div className="font-medium">
                        {isSend ? "-" : "+"}{payment.sourceAmount} {sourceChain?.symbol}
                      </div>
                    </div>
                    <div>
                      <div className="text-muted-foreground">Received</div>
                      <div className="font-medium">
                        {payment.destAmount} {destChain?.symbol}
                      </div>
                    </div>
                  </div>

                  <div className="text-xs text-muted-foreground">
                    {new Date(payment.timestamp).toLocaleString()}
                  </div>

                  {payment.error && (
                    <div className="text-xs text-destructive">
                      {payment.error}
                    </div>
                  )}
                </div>
              )
            })}
          </div>
        </ScrollArea>
      </CardContent>
    </Card>
  )
}
