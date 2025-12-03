"use client"

import { useState } from "react"
import { Button } from "@/components/ui/button"
import { Badge } from "@/components/ui/badge"
import { X, ExternalLink, MessageSquare, TrendingUp, Users, Clock } from "lucide-react"
import { Card } from "@/components/ui/card"
import { Proposal } from "@/lib/polkadot/governance"
import { formatETR } from "@/lib/governance/hooks"
import VoteModal from "./VoteModal"

interface ProposalModalProps {
  proposal: Proposal
  onClose: () => void
  userAddress?: string
}

export default function ProposalModal({ proposal, onClose, userAddress }: ProposalModalProps) {
  const [showVoteModal, setShowVoteModal] = useState(false)

  // Calculate vote percentages
  const totalVotesBigInt = BigInt(proposal.votesFor) + BigInt(proposal.votesAgainst) + BigInt(proposal.votesAbstain)
  const totalVotes = Number(totalVotesBigInt)

  const yesPercentage = totalVotes > 0 ? (Number(BigInt(proposal.votesFor) * BigInt(10000) / totalVotesBigInt) / 100) : 0
  const noPercentage = totalVotes > 0 ? (Number(BigInt(proposal.votesAgainst) * BigInt(10000) / totalVotesBigInt) / 100) : 0
  const abstainPercentage = totalVotes > 0 ? (Number(BigInt(proposal.votesAbstain) * BigInt(10000) / totalVotesBigInt) / 100) : 0

  // Format vote counts
  const formatVoteCount = (votes: string) => {
    return formatETR(votes)
  }

  // Map category to display name
  const categoryNames: Record<string, string> = {
    InflationRate: "Inflation Rate",
    ParameterChange: "Parameter Change",
    BudgetAllocation: "Budget Allocation",
    ProtocolUpgrade: "Protocol Upgrade",
    DirectorElection: "Director Election",
    EmergencyAction: "Emergency Action",
  }

  const displayCategory = categoryNames[proposal.category] || proposal.category

  // Format submission date
  const submissionDate = proposal.createdAt
    ? new Date(proposal.createdAt * 1000).toLocaleDateString("en-US", {
        month: "long",
        day: "numeric",
        year: "numeric",
      })
    : "Unknown"

  return (
    <>
      <div className="fixed inset-0 z-40 bg-background/80 backdrop-blur-sm flex items-center justify-center p-4">
        <Card className="w-full max-w-4xl max-h-[90vh] overflow-y-auto animate-in slide-in-from-right glass-card">
          <div className="p-6 space-y-6">
            {/* Header */}
            <div className="flex items-start justify-between">
              <div className="space-y-2">
                <div className="flex items-center gap-2 flex-wrap">
                  <span className="text-sm font-mono text-muted-foreground">#{proposal.id}</span>
                  <Badge>{proposal.status}</Badge>
                  <Badge variant="outline">{displayCategory}</Badge>
                </div>
                <h2 className="text-3xl font-bold">{proposal.title}</h2>
              </div>
              <Button variant="ghost" size="icon" onClick={onClose}>
                <X className="w-5 h-5" />
              </Button>
            </div>

            {/* Submitted By */}
            <div className="flex items-center gap-3">
              <div className="w-10 h-10 rounded-full bg-gradient-to-br from-primary to-accent" />
              <div>
                <p className="text-sm text-muted-foreground">Submitted by</p>
                <p className="font-mono text-sm">{proposal.proposer.slice(0, 10)}...{proposal.proposer.slice(-8)}</p>
              </div>
              <span className="text-muted-foreground">•</span>
              <p className="text-sm text-muted-foreground">{submissionDate}</p>
            </div>

            {/* Full Description */}
            <Card className="p-4 glass-card">
              <h3 className="text-lg font-bold mb-3">Proposal Description</h3>
              <div className="prose prose-invert max-w-none text-sm text-muted-foreground">
                <p>{proposal.description || "No description provided"}</p>
              </div>
            </Card>

            {/* Voting Statistics */}
            <Card className="p-6 glass-card">
              <h3 className="text-lg font-bold mb-4">Voting Statistics</h3>

              <div className="space-y-4">
                {/* Vote Progress Bar */}
                <div className="space-y-2">
                  <div className="flex h-4 rounded-full overflow-hidden bg-muted">
                    <div className="bg-green-500 transition-all" style={{ width: `${yesPercentage}%` }} />
                    <div className="bg-red-500 transition-all" style={{ width: `${noPercentage}%` }} />
                    <div className="bg-gray-500 transition-all" style={{ width: `${abstainPercentage}%` }} />
                  </div>

                  <div className="grid grid-cols-3 gap-4 text-sm">
                    <div className="text-center">
                      <p className="text-green-500 font-semibold">{yesPercentage.toFixed(1)}%</p>
                      <p className="text-muted-foreground">Yes</p>
                      <p className="font-mono text-xs">{formatVoteCount(proposal.votesFor)} ETR</p>
                    </div>
                    <div className="text-center">
                      <p className="text-red-500 font-semibold">{noPercentage.toFixed(1)}%</p>
                      <p className="text-muted-foreground">No</p>
                      <p className="font-mono text-xs">{formatVoteCount(proposal.votesAgainst)} ETR</p>
                    </div>
                    <div className="text-center">
                      <p className="text-gray-500 font-semibold">{abstainPercentage.toFixed(1)}%</p>
                      <p className="text-muted-foreground">Abstain</p>
                      <p className="font-mono text-xs">{formatVoteCount(proposal.votesAbstain)} ETR</p>
                    </div>
                  </div>
                </div>

                {/* Additional Stats */}
                <div className="grid grid-cols-2 gap-4 pt-4 border-t border-white/10">
                  <div className="flex items-center gap-3">
                    <Users className="w-5 h-5 text-blue-400" />
                    <div>
                      <p className="text-sm text-muted-foreground">Total Votes</p>
                      <p className="font-semibold">{formatVoteCount(totalVotesBigInt.toString())} ETR</p>
                    </div>
                  </div>
                  <div className="flex items-center gap-3">
                    <TrendingUp className="w-5 h-5 text-purple-400" />
                    <div>
                      <p className="text-sm text-muted-foreground">Status</p>
                      <p className={`font-semibold ${proposal.approved ? "text-green-500" : "text-amber-500"}`}>
                        {proposal.approved ? "Approved" : "Pending"}
                      </p>
                    </div>
                  </div>
                </div>
              </div>
            </Card>

            {/* Discussion Link */}
            <Card className="p-4 glass-card">
              <div className="flex items-center justify-between">
                <div className="flex items-center gap-2">
                  <MessageSquare className="w-5 h-5 text-muted-foreground" />
                  <span className="text-sm text-muted-foreground">Join the discussion on the forum</span>
                </div>
                <Button variant="outline" size="sm" className="gap-2" asChild>
                  <a href={`https://forum.etrid.org/proposal/${proposal.id}`} target="_blank" rel="noopener noreferrer">
                    View Discussion
                    <ExternalLink className="w-4 h-4" />
                  </a>
                </Button>
              </div>
            </Card>

            {/* Vote Button */}
            {proposal.status === "Active" && userAddress && (
              <Button onClick={() => setShowVoteModal(true)} className="w-full" size="lg">
                Cast Your Vote
              </Button>
            )}

            {!userAddress && proposal.status === "Active" && (
              <Card className="p-4 bg-amber-500/10 border-amber-500/20">
                <p className="text-center text-amber-500 text-sm">
                  Connect your wallet to vote on this proposal
                </p>
              </Card>
            )}

            {proposal.executed && (
              <Card className="p-4 bg-green-500/10 border-green-500/20">
                <p className="text-center text-green-500 font-semibold">
                  This proposal has been executed
                </p>
              </Card>
            )}
          </div>
        </Card>
      </div>

      {/* Vote Modal */}
      {showVoteModal && userAddress && (
        <VoteModal
          proposal={proposal}
          onClose={() => setShowVoteModal(false)}
          userAddress={userAddress}
          onVoteSuccess={() => {
            setShowVoteModal(false)
            onClose()
          }}
        />
      )}
    </>
  )
}
