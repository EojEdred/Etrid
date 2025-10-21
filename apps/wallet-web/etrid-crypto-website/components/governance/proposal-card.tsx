"use client"

import { Card } from "@/components/ui/card"
import { Button } from "@/components/ui/button"
import { Badge } from "@/components/ui/badge"
import { CheckCircle2, AlertCircle, Clock } from "lucide-react"

interface Proposal {
  id: number
  title: string
  status: "active" | "passed" | "rejected"
  category: string
  submittedBy: string
  submissionDate: string
  description: string
  votesYes: number
  votesNo: number
  votesAbstain: number
  totalVotes: number
  quorumReached: boolean
  quorumPercentage: number
  timeLeft: string
  userVoted: boolean
  userVote?: "yes" | "no" | "abstain"
}

interface ProposalCardProps {
  proposal: Proposal
  onClick: () => void
}

export default function ProposalCard({ proposal, onClick }: ProposalCardProps) {
  const yesPercentage = (proposal.votesYes / proposal.totalVotes) * 100
  const noPercentage = (proposal.votesNo / proposal.totalVotes) * 100
  const abstainPercentage = (proposal.votesAbstain / proposal.totalVotes) * 100

  const statusColors = {
    active: "bg-green-500/10 text-green-500 border-green-500/20",
    passed: "bg-blue-500/10 text-blue-500 border-blue-500/20",
    rejected: "bg-red-500/10 text-red-500 border-red-500/20",
  }

  const categoryColors: Record<string, string> = {
    "Fiscal Policy": "bg-purple-500/10 text-purple-500",
    "Protocol Upgrades": "bg-blue-500/10 text-blue-500",
    Treasury: "bg-amber-500/10 text-amber-500",
  }

  return (
    <Card className="p-6 hover:border-accent/50 transition-all cursor-pointer group">
      <div className="space-y-4">
        {/* Header */}
        <div className="flex items-start justify-between gap-4">
          <div className="flex items-center gap-2">
            <span className="text-sm font-mono text-muted-foreground">#{proposal.id}</span>
            <Badge className={statusColors[proposal.status]}>
              {proposal.status.charAt(0).toUpperCase() + proposal.status.slice(1)}
            </Badge>
            <Badge className={categoryColors[proposal.category]}>{proposal.category}</Badge>
          </div>
        </div>

        {/* Title */}
        <h3 className="text-xl font-bold group-hover:text-accent transition-colors">{proposal.title}</h3>

        {/* Metadata */}
        <div className="flex items-center gap-4 text-sm text-muted-foreground">
          <div className="flex items-center gap-2">
            <div className="w-6 h-6 rounded-full bg-gradient-to-br from-primary to-accent" />
            <span>{proposal.submittedBy}</span>
          </div>
          <span>•</span>
          <span>{proposal.submissionDate}</span>
        </div>

        {/* Description */}
        <p className="text-sm text-muted-foreground line-clamp-3">{proposal.description}</p>
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
            <span className="text-green-500">{(proposal.votesYes / 1000000).toFixed(1)}M Yes</span>
            <span className="text-red-500">{(proposal.votesNo / 1000000).toFixed(1)}M No</span>
            <span className="text-gray-500">{(proposal.votesAbstain / 1000000).toFixed(1)}M Abstain</span>
          </div>

          <div className="flex items-center gap-2 text-sm">
            {proposal.quorumReached ? (
              <>
                <CheckCircle2 className="w-4 h-4 text-green-500" />
                <span className="text-green-500">{proposal.quorumPercentage}% quorum reached</span>
              </>
            ) : (
              <>
                <AlertCircle className="w-4 h-4 text-amber-500" />
                <span className="text-amber-500">{proposal.quorumPercentage}% needed</span>
              </>
            )}
          </div>
        </div>

        {/* Time Left */}
        <div className="flex items-center gap-2 text-sm text-muted-foreground">
          <Clock className="w-4 h-4" />
          <span>{proposal.timeLeft} remaining</span>
        </div>

        {/* Actions */}
        <div className="flex gap-2">
          {proposal.userVoted ? (
            <Badge className="bg-green-500/10 text-green-500 border-green-500/20">
              You voted: {proposal.userVote?.toUpperCase()} ✅
            </Badge>
          ) : (
            <Button onClick={onClick} className="flex-1">
              Vote
            </Button>
          )}
          <Button onClick={onClick} variant="outline">
            Details
          </Button>
        </div>
      </div>
    </Card>
  )
}
