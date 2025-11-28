"use client"

import { ArrowRight, Check, X, Clock, User } from "lucide-react"
import { Proposal } from "@/lib/types/multisig"
import { Card, CardContent } from "@/components/ui/card"
import { Badge } from "@/components/ui/badge"
import { Button } from "@/components/ui/button"
import { Progress } from "@/components/ui/progress"
import { useState } from "react"

interface ApprovalItemProps {
  proposal: Proposal
  onSign?: () => Promise<void>
  onReject?: () => Promise<void>
}

export function ApprovalItem({ proposal, onSign, onReject }: ApprovalItemProps) {
  const [isSigningLoading, setSigningLoading] = useState(false)
  const [isRejectingLoading, setRejectingLoading] = useState(false)

  const progress = (proposal.signaturesCollected / proposal.threshold) * 100

  const typeConfig = {
    transfer: { icon: "💸", label: "Transfer", color: "text-blue-500" },
    contract_call: { icon: "📜", label: "Contract Call", color: "text-purple-500" },
    add_signer: { icon: "➕", label: "Add Signer", color: "text-green-500" },
    remove_signer: { icon: "➖", label: "Remove Signer", color: "text-red-500" },
    change_threshold: { icon: "⚙️", label: "Change Threshold", color: "text-yellow-500" },
  }

  const config = typeConfig[proposal.type]

  const timeRemaining = new Date(proposal.expiresAt).getTime() - Date.now()
  const hoursRemaining = Math.floor(timeRemaining / (1000 * 60 * 60))
  const isExpiringSoon = hoursRemaining < 24

  const handleSign = async () => {
    if (!onSign) return
    setSigningLoading(true)
    try {
      await onSign()
    } finally {
      setSigningLoading(false)
    }
  }

  const handleReject = async () => {
    if (!onReject) return
    setRejectingLoading(true)
    try {
      await onReject()
    } finally {
      setRejectingLoading(false)
    }
  }

  return (
    <Card className="hover:shadow-md transition-shadow">
      <CardContent className="p-6">
        {/* Header */}
        <div className="flex items-start justify-between mb-4">
          <div className="flex items-center gap-3">
            <div className="text-3xl">{config.icon}</div>
            <div>
              <h4 className="font-semibold">{config.label}</h4>
              <p className="text-xs text-muted-foreground">
                Proposed by {proposal.proposerUsername || `${proposal.proposer.slice(0, 6)}...${proposal.proposer.slice(-4)}`}
              </p>
            </div>
          </div>
          <Badge
            variant={isExpiringSoon ? "destructive" : "secondary"}
            className={isExpiringSoon ? "animate-pulse" : ""}
          >
            <Clock className="w-3 h-3 mr-1" />
            {hoursRemaining}h left
          </Badge>
        </div>

        {/* Transaction Details */}
        {proposal.type === "transfer" && proposal.transactionData.to && (
          <div className="mb-4 p-3 rounded-lg bg-muted/50">
            <div className="flex items-center justify-between mb-2">
              <p className="text-sm font-medium">Amount</p>
              <p className="text-lg font-bold">
                {parseFloat(proposal.transactionData.amount || "0").toLocaleString()} ÉTR
              </p>
            </div>
            <div className="flex items-center gap-2 text-sm text-muted-foreground">
              <span>To:</span>
              <code className="px-2 py-1 rounded bg-background">
                {proposal.transactionData.to.slice(0, 8)}...{proposal.transactionData.to.slice(-6)}
              </code>
            </div>
            {proposal.transactionData.description && (
              <p className="mt-2 text-sm text-muted-foreground">
                "{proposal.transactionData.description}"
              </p>
            )}
          </div>
        )}

        {/* Signature Progress */}
        <div className="mb-4">
          <div className="flex items-center justify-between mb-2">
            <p className="text-sm font-medium">Signatures</p>
            <p className="text-sm font-semibold">
              {proposal.signaturesCollected} of {proposal.threshold}
            </p>
          </div>
          <Progress value={progress} className="h-2" />
        </div>

        {/* Signers Status */}
        <div className="mb-4 flex flex-wrap gap-2">
          {proposal.signatures.map((sig, i) => (
            <Badge
              key={i}
              variant={sig.status === "approved" ? "default" : "destructive"}
              className="text-xs"
            >
              {sig.status === "approved" ? (
                <Check className="w-3 h-3 mr-1" />
              ) : (
                <X className="w-3 h-3 mr-1" />
              )}
              {sig.signer.slice(0, 6)}
            </Badge>
          ))}
        </div>

        {/* Action Buttons */}
        {proposal.status === "pending" && (
          <div className="flex gap-3">
            <Button
              className="flex-1"
              onClick={handleSign}
              disabled={isSigningLoading || isRejectingLoading}
            >
              {isSigningLoading ? (
                "Signing..."
              ) : (
                <>
                  <Check className="w-4 h-4 mr-2" />
                  Sign Transaction
                </>
              )}
            </Button>
            <Button
              variant="outline"
              className="flex-1"
              onClick={handleReject}
              disabled={isSigningLoading || isRejectingLoading}
            >
              {isRejectingLoading ? (
                "Rejecting..."
              ) : (
                <>
                  <X className="w-4 h-4 mr-2" />
                  Reject
                </>
              )}
            </Button>
          </div>
        )}

        {/* Status Messages */}
        {proposal.status === "approved" && (
          <div className="flex items-center gap-2 text-green-500 bg-green-500/10 p-3 rounded-lg">
            <Check className="w-4 h-4" />
            <span className="text-sm font-medium">Ready to execute</span>
          </div>
        )}
        {proposal.status === "executed" && (
          <div className="flex items-center gap-2 text-blue-500 bg-blue-500/10 p-3 rounded-lg">
            <Check className="w-4 h-4" />
            <span className="text-sm font-medium">Executed</span>
          </div>
        )}

        {/* Timestamp */}
        <p className="mt-4 text-xs text-muted-foreground">
          Submitted {new Date(proposal.createdAt).toLocaleString()}
        </p>
      </CardContent>
    </Card>
  )
}
