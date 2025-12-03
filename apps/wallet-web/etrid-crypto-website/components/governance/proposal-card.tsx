"use client"

import { Card } from "@/components/ui/card"
import { Button } from "@/components/ui/button"
import { Badge } from "@/components/ui/badge"
import { CheckCircle2, AlertCircle, Clock } from "lucide-react"
import { Proposal } from "@/lib/polkadot/governance"
import { formatETR } from "@/lib/governance/hooks"
import { useEffect, useState } from "react"

interface ProposalCardProps {
  proposal: Proposal
  onClick: () => void
  userAddress?: string
}

// Map categories to display names
const categoryDisplayNames: Record<string, string> = {
  InflationRate: "Inflation Rate",
  ParameterChange: "Parameter Change",
  BudgetAllocation: "Budget Allocation",
  ProtocolUpgrade: "Protocol Upgrade",
  DirectorElection: "Director Election",
  EmergencyAction: "Emergency Action",
}

// Map status to UI-friendly status
const statusMap: Record<string, "active" | "passed" | "rejected"> = {
  Active: "active",
  Pending: "active",
  Approved: "passed",
  Executed: "passed",
  Rejected: "rejected",
  Cancelled: "rejected",
}

export default function ProposalCard({ proposal, onClick, userAddress }: ProposalCardProps) {
  // Calculate vote percentages
  const totalVotesBigInt = BigInt(proposal.votesFor) + BigInt(proposal.votesAgainst) + BigInt(proposal.votesAbstain)
  const totalVotes = Number(totalVotesBigInt)

  const yesPercentage = totalVotes > 0 ? (Number(BigInt(proposal.votesFor) * BigInt(10000) / totalVotesBigInt) / 100) : 0
  const noPercentage = totalVotes > 0 ? (Number(BigInt(proposal.votesAgainst) * BigInt(10000) / totalVotesBigInt) / 100) : 0
  const abstainPercentage = totalVotes > 0 ? (Number(BigInt(proposal.votesAbstain) * BigInt(10000) / totalVotesBigInt) / 100) : 0

  // Countdown timer state
  const [timeRemaining, setTimeRemaining] = useState({ days: 0, hours: 0, minutes: 0 })

  useEffect(() => {
    // Calculate time remaining based on block time (6 seconds per block)
    // For active proposals, estimate based on typical voting period
    if (proposal.status === "Active") {
      const BLOCKS_PER_DAY = 14400 // 86400 seconds / 6 seconds per block
      const VOTING_PERIOD_DAYS = 7 // Example: 7 day voting period
      const TOTAL_VOTING_BLOCKS = BLOCKS_PER_DAY * VOTING_PERIOD_DAYS

      // Calculate blocks since proposal creation (simplified)
      const currentBlock = Date.now() / 6000 // Very rough estimate
      const proposalBlock = proposal.createdAt || 0
      const blocksPassed = Math.max(0, currentBlock - proposalBlock)
      const blocksRemaining = Math.max(0, TOTAL_VOTING_BLOCKS - blocksPassed)

      const totalSeconds = blocksRemaining * 6
      const days = Math.floor(totalSeconds / 86400)
      const hours = Math.floor((totalSeconds % 86400) / 3600)
      const minutes = Math.floor((totalSeconds % 3600) / 60)

      setTimeRemaining({ days, hours, minutes })
    }
  }, [proposal])

  const statusColors = {
    active: "bg-green-500/10 text-green-500 border-green-500/20",
    passed: "bg-blue-500/10 text-blue-500 border-blue-500/20",
    rejected: "bg-red-500/10 text-red-500 border-red-500/20",
  }

  const categoryColors: Record<string, string> = {
    "InflationRate": "bg-purple-500/10 text-purple-500",
    "ParameterChange": "bg-blue-500/10 text-blue-500",
    "BudgetAllocation": "bg-amber-500/10 text-amber-500",
    "ProtocolUpgrade": "bg-cyan-500/10 text-cyan-500",
    "DirectorElection": "bg-pink-500/10 text-pink-500",
    "EmergencyAction": "bg-red-500/10 text-red-500",
  }

  const uiStatus = statusMap[proposal.status] || "active"
  const displayCategory = categoryDisplayNames[proposal.category] || proposal.category

  // Format vote counts
  const formatVoteCount = (votes: string) => {
    const count = Number(BigInt(votes) / BigInt(1e18))
    if (count >= 1000000) return `${(count / 1000000).toFixed(1)}M`
    if (count >= 1000) return `${(count / 1000).toFixed(1)}K`
    return count.toFixed(0)
  }

  // Format submission date
  const submissionDate = proposal.createdAt
    ? new Date(proposal.createdAt * 1000).toLocaleDateString("en-US", {
        month: "short",
        day: "numeric",
        year: "numeric",
      })
    : "Unknown"

  return (
    <Card className="p-6 hover:border-accent/50 transition-all cursor-pointer group glass-card">
      <div className="space-y-4">
        {/* Header */}
        <div className="flex items-start justify-between gap-4">
          <div className="flex items-center gap-2 flex-wrap">
            <span className="text-sm font-mono text-muted-foreground">#{proposal.id}</span>
            <Badge className={statusColors[uiStatus]}>
              {uiStatus.charAt(0).toUpperCase() + uiStatus.slice(1)}
            </Badge>
            <Badge className={categoryColors[proposal.category]}>{displayCategory}</Badge>
          </div>
        </div>

        {/* Title */}
        <h3 className="text-xl font-bold group-hover:text-accent transition-colors">{proposal.title}</h3>

        {/* Metadata */}
        <div className="flex items-center gap-4 text-sm text-muted-foreground">
          <div className="flex items-center gap-2">
            <div className="w-6 h-6 rounded-full bg-gradient-to-br from-primary to-accent" />
            <span className="font-mono text-xs">{proposal.proposer.slice(0, 6)}...{proposal.proposer.slice(-4)}</span>
          </div>
          <span>•</span>
          <span>{submissionDate}</span>
        </div>

        {/* Description */}
        <p className="text-sm text-muted-foreground line-clamp-3">{proposal.description || "No description provided"}</p>
        <button onClick={onClick} className="text-sm text-accent hover:underline">
          Read more
        </button>

        {/* Voting Progress */}
        <div className="space-y-2">
          <div className="flex h-3 rounded-full overflow-hidden bg-muted">
            <div className="bg-green-500 transition-all duration-1000" style={{ width: `${yesPercentage}%` }} />
            <div className="bg-red-500 transition-all duration-1000" style={{ width: `${noPercentage}%` }} />
            <div className="bg-gray-500 transition-all duration-1000" style={{ width: `${abstainPercentage}%` }} />
          </div>

          <div className="flex justify-between text-xs">
            <span className="text-green-500">{formatVoteCount(proposal.votesFor)} Yes ({yesPercentage.toFixed(1)}%)</span>
            <span className="text-red-500">{formatVoteCount(proposal.votesAgainst)} No ({noPercentage.toFixed(1)}%)</span>
            <span className="text-gray-500">{formatVoteCount(proposal.votesAbstain)} Abstain</span>
          </div>

          <div className="flex items-center gap-2 text-sm">
            {proposal.approved ? (
              <>
                <CheckCircle2 className="w-4 h-4 text-green-500" />
                <span className="text-green-500">Quorum reached</span>
              </>
            ) : (
              <>
                <AlertCircle className="w-4 h-4 text-amber-500" />
                <span className="text-amber-500">Pending quorum</span>
              </>
            )}
          </div>
        </div>

        {/* Time Left (only for active proposals) */}
        {proposal.status === "Active" && (
          <div className="flex items-center gap-2 text-sm text-muted-foreground">
            <Clock className="w-4 h-4" />
            <span>
              {timeRemaining.days > 0 && `${timeRemaining.days}d `}
              {timeRemaining.hours}h {timeRemaining.minutes}m remaining
            </span>
          </div>
        )}

        {/* Actions */}
        <div className="flex gap-2">
          {proposal.status === "Active" ? (
            <Button onClick={onClick} className="flex-1">
              Vote Now
            </Button>
          ) : (
            <Button onClick={onClick} variant="outline" className="flex-1">
              View Details
            </Button>
          )}
        </div>
      </div>
    </Card>
  )
}
