"use client"

import { useState } from "react"
import { Button } from "@/components/ui/button"
import { Card } from "@/components/ui/card"
import { Input } from "@/components/ui/input"
import { Textarea } from "@/components/ui/textarea"
import { Label } from "@/components/ui/label"
import { X, Loader2, Plus, FileText, DollarSign } from "lucide-react"
import { ProposalCategory } from "@/lib/polkadot/governance"
import { governanceApi } from "@/lib/polkadot/governance"
import { Badge } from "@/components/ui/badge"
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "@/components/ui/select"

interface CreateProposalProps {
  onClose: () => void
  userAddress: string
  onSuccess?: () => void
}

const PROPOSAL_CATEGORIES: { value: ProposalCategory; label: string; description: string }[] = [
  {
    value: "InflationRate",
    label: "Inflation Rate",
    description: "Propose changes to the annual ETR inflation rate",
  },
  {
    value: "ParameterChange",
    label: "Parameter Change",
    description: "Modify protocol parameters like transaction fees, block time, etc.",
  },
  {
    value: "BudgetAllocation",
    label: "Budget Allocation",
    description: "Request treasury funds for development, grants, or partnerships",
  },
  {
    value: "ProtocolUpgrade",
    label: "Protocol Upgrade",
    description: "Propose runtime upgrades or protocol improvements",
  },
  {
    value: "DirectorElection",
    label: "Director Election",
    description: "Nominate or elect new Foundation Directors",
  },
  {
    value: "EmergencyAction",
    label: "Emergency Action",
    description: "Critical proposals requiring immediate attention",
  },
]

const BUDGET_CATEGORIES = [
  "Development",
  "Marketing",
  "Grants",
  "Partnerships",
  "Infrastructure",
  "Research",
  "Community",
  "Other",
]

export default function CreateProposal({ onClose, userAddress, onSuccess }: CreateProposalProps) {
  const [title, setTitle] = useState("")
  const [description, setDescription] = useState("")
  const [category, setCategory] = useState<ProposalCategory>("ParameterChange")
  const [budgetRequest, setBudgetRequest] = useState("")
  const [budgetCategory, setBudgetCategory] = useState<string | null>(null)
  const [isSubmitting, setIsSubmitting] = useState(false)
  const [error, setError] = useState<string | null>(null)
  const [txHash, setTxHash] = useState<string | null>(null)

  const selectedCategory = PROPOSAL_CATEGORIES.find((c) => c.value === category)
  const isBudgetProposal = category === "BudgetAllocation"

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault()

    if (!title.trim() || !description.trim()) {
      setError("Title and description are required")
      return
    }

    if (isBudgetProposal && !budgetRequest) {
      setError("Budget request amount is required for budget allocation proposals")
      return
    }

    setIsSubmitting(true)
    setError(null)
    setTxHash(null)

    try {
      // Import web3FromAddress dynamically to avoid SSR issues
      const { web3FromAddress } = await import("@polkadot/extension-dapp")
      const injector = await web3FromAddress(userAddress)

      // Convert budget request to wei (18 decimals)
      const budgetInWei = isBudgetProposal && budgetRequest
        ? (parseFloat(budgetRequest) * 1e18).toString()
        : "0"

      // Submit proposal to blockchain
      const hash = await governanceApi.submitProposal(
        title,
        category,
        budgetInWei,
        isBudgetProposal ? budgetCategory : null,
        injector.signer
      )

      setTxHash(hash)

      // Call success callback
      if (onSuccess) {
        onSuccess()
      }

      // Close modal after successful submission
      setTimeout(() => {
        onClose()
      }, 3000)
    } catch (err) {
      const message = err instanceof Error ? err.message : "Failed to submit proposal"
      setError(message)
      console.error("[CreateProposal] Submission error:", err)
    } finally {
      setIsSubmitting(false)
    }
  }

  return (
    <div className="fixed inset-0 z-50 bg-background/80 backdrop-blur-sm flex items-center justify-center p-4">
      <Card className="w-full max-w-3xl max-h-[90vh] overflow-y-auto animate-in slide-in-from-bottom glass-card">
        <div className="p-6 space-y-6">
          {/* Header */}
          <div className="flex items-start justify-between">
            <div className="space-y-2">
              <div className="flex items-center gap-2">
                <Plus className="w-6 h-6 text-accent" />
                <h2 className="text-2xl font-bold">Create New Proposal</h2>
              </div>
              <p className="text-sm text-muted-foreground">
                Submit a proposal for the ETRID community to vote on during the next Consensus Day
              </p>
            </div>
            <Button variant="ghost" size="icon" onClick={onClose} disabled={isSubmitting}>
              <X className="w-5 h-5" />
            </Button>
          </div>

          {/* Form */}
          <form onSubmit={handleSubmit} className="space-y-6">
            {/* Category Selection */}
            <div className="space-y-3">
              <Label htmlFor="category">Proposal Category *</Label>
              <Select value={category} onValueChange={(value) => setCategory(value as ProposalCategory)}>
                <SelectTrigger id="category" disabled={isSubmitting}>
                  <SelectValue placeholder="Select a category" />
                </SelectTrigger>
                <SelectContent>
                  {PROPOSAL_CATEGORIES.map((cat) => (
                    <SelectItem key={cat.value} value={cat.value}>
                      {cat.label}
                    </SelectItem>
                  ))}
                </SelectContent>
              </Select>
              {selectedCategory && (
                <p className="text-sm text-muted-foreground">{selectedCategory.description}</p>
              )}
            </div>

            {/* Title */}
            <div className="space-y-3">
              <Label htmlFor="title">Proposal Title *</Label>
              <Input
                id="title"
                value={title}
                onChange={(e) => setTitle(e.target.value)}
                placeholder="e.g., Increase block gas limit to 30M"
                disabled={isSubmitting}
                maxLength={200}
                required
              />
              <p className="text-xs text-muted-foreground">{title.length}/200 characters</p>
            </div>

            {/* Description */}
            <div className="space-y-3">
              <Label htmlFor="description">Detailed Description *</Label>
              <Textarea
                id="description"
                value={description}
                onChange={(e) => setDescription(e.target.value)}
                placeholder="Provide a detailed explanation of your proposal, including rationale, implementation details, and expected impact..."
                disabled={isSubmitting}
                rows={8}
                maxLength={5000}
                required
                className="resize-none"
              />
              <p className="text-xs text-muted-foreground">{description.length}/5000 characters</p>
            </div>

            {/* Budget Request (only for BudgetAllocation) */}
            {isBudgetProposal && (
              <Card className="p-4 bg-accent/5 border-accent/20 space-y-4">
                <div className="flex items-center gap-2">
                  <DollarSign className="w-5 h-5 text-accent" />
                  <h3 className="font-semibold">Budget Request Details</h3>
                </div>

                <div className="space-y-3">
                  <Label htmlFor="budgetRequest">Requested Amount (ETR) *</Label>
                  <Input
                    id="budgetRequest"
                    type="number"
                    value={budgetRequest}
                    onChange={(e) => setBudgetRequest(e.target.value)}
                    placeholder="e.g., 50000"
                    disabled={isSubmitting}
                    min="0"
                    step="0.01"
                    required
                  />
                </div>

                <div className="space-y-3">
                  <Label htmlFor="budgetCategory">Budget Category *</Label>
                  <Select value={budgetCategory || ""} onValueChange={setBudgetCategory}>
                    <SelectTrigger id="budgetCategory" disabled={isSubmitting}>
                      <SelectValue placeholder="Select a budget category" />
                    </SelectTrigger>
                    <SelectContent>
                      {BUDGET_CATEGORIES.map((cat) => (
                        <SelectItem key={cat} value={cat}>
                          {cat}
                        </SelectItem>
                      ))}
                    </SelectContent>
                  </Select>
                </div>
              </Card>
            )}

            {/* Error Display */}
            {error && (
              <div className="p-3 bg-destructive/10 border border-destructive/20 rounded-lg text-destructive text-sm">
                {error}
              </div>
            )}

            {/* Success Display */}
            {txHash && (
              <div className="p-3 bg-green-500/10 border border-green-500/20 rounded-lg text-green-500 text-sm">
                Proposal submitted successfully! Transaction: {txHash.slice(0, 10)}...{txHash.slice(-8)}
              </div>
            )}

            {/* Requirements Notice */}
            <Card className="p-4 bg-muted/50">
              <div className="flex items-start gap-3">
                <FileText className="w-5 h-5 text-muted-foreground mt-0.5" />
                <div className="space-y-2 text-sm">
                  <h4 className="font-semibold">Proposal Requirements</h4>
                  <ul className="list-disc list-inside space-y-1 text-muted-foreground">
                    <li>Proposals can only be submitted during the Registration phase of Consensus Day</li>
                    <li>A minimum stake may be required to submit a proposal</li>
                    <li>Your proposal will be reviewed by the community before voting begins</li>
                    <li>Approved proposals will be voted on during the Voting phase</li>
                  </ul>
                </div>
              </div>
            </Card>

            {/* Submit Button */}
            <div className="flex gap-3">
              <Button
                type="button"
                variant="outline"
                onClick={onClose}
                disabled={isSubmitting}
                className="flex-1"
              >
                Cancel
              </Button>
              <Button type="submit" disabled={isSubmitting} className="flex-1" size="lg">
                {isSubmitting ? (
                  <>
                    <Loader2 className="w-4 h-4 mr-2 animate-spin" />
                    Submitting...
                  </>
                ) : (
                  <>
                    <Plus className="w-4 h-4 mr-2" />
                    Submit Proposal
                  </>
                )}
              </Button>
            </div>

            {isSubmitting && (
              <p className="text-xs text-muted-foreground text-center">
                Sign transaction in your wallet to submit proposal...
              </p>
            )}
          </form>
        </div>
      </Card>
    </div>
  )
}
