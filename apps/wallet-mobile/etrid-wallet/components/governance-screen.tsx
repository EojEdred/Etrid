"use client"

import { useState } from "react"
import { Scale, Info, Clock, TrendingUp } from "lucide-react"
import { Button } from "@/components/ui/button"
import { BottomNav } from "@/components/bottom-nav"
import { VoteModal } from "@/components/vote-modal"

interface GovernanceScreenProps {
  onBack: () => void
  activeTab: string
  onTabChange: (tab: string) => void
}

interface Proposal {
  id: number
  title: string
  category: string
  description: string
  status: "active" | "passed" | "rejected"
  yesVotes: number
  noVotes: number
  abstainVotes: number
  totalVotes: number
  timeLeft: string
  userVoted?: "yes" | "no" | "abstain"
}

const proposals: Proposal[] = [
  {
    id: 3,
    title: "Increase Annual Inflation to 3%",
    category: "Fiscal Policy",
    description:
      "This proposal suggests increasing the annual inflation rate from 2% to 3% to provide more rewards for stakers and validators, encouraging network participation and security.",
    status: "active",
    yesVotes: 6500000,
    noVotes: 3000000,
    abstainVotes: 500000,
    totalVotes: 10000000,
    timeLeft: "4 days",
  },
  {
    id: 4,
    title: "Implement New Governance Framework",
    category: "Governance",
    description:
      "Proposal to update the governance framework to allow for more frequent consensus days and lower voting thresholds for certain proposal types.",
    status: "active",
    yesVotes: 4200000,
    noVotes: 2800000,
    abstainVotes: 1000000,
    totalVotes: 8000000,
    timeLeft: "6 days",
  },
  {
    id: 2,
    title: "Reduce Transaction Fees by 50%",
    category: "Network",
    description:
      "Lower base transaction fees to make the network more accessible and competitive with other blockchains.",
    status: "passed",
    yesVotes: 8500000,
    noVotes: 1500000,
    abstainVotes: 0,
    totalVotes: 10000000,
    timeLeft: "Ended",
    userVoted: "yes",
  },
]

export function GovernanceScreen({ onBack, activeTab, onTabChange }: GovernanceScreenProps) {
  const [selectedProposal, setSelectedProposal] = useState<Proposal | null>(null)
  const [showPastVotes, setShowPastVotes] = useState(false)

  const userStake = 1000
  const userCoinage = 365
  const votingPower = Math.floor(Math.sqrt(userStake * userCoinage))

  const activeProposals = proposals.filter((p) => p.status === "active")
  const pastProposals = proposals.filter((p) => p.status !== "active")

  return (
    <div className="min-h-screen pb-24">
      {/* Header */}
      <header className="p-6 space-y-4">
        <div className="text-center">
          <h1 className="text-2xl font-bold mb-1">Consensus Day 2026</h1>
          <div className="flex items-center justify-center gap-2 text-warning">
            <Clock className="w-4 h-4" />
            <span className="text-sm">Voting ends in: 5d 12h 34m</span>
          </div>
          <p className="text-sm text-muted-foreground mt-2">Your vote shapes Ëtrid&apos;s future</p>
        </div>

        {/* Voting Power Card */}
        <div className="glass-strong rounded-2xl p-5">
          <div className="flex items-center gap-3 mb-3">
            <div className="w-10 h-10 rounded-full bg-accent/20 flex items-center justify-center">
              <Scale className="w-5 h-5 text-accent" />
            </div>
            <div className="flex-1">
              <p className="text-sm text-muted-foreground">Your Voting Power</p>
              <p className="text-2xl font-bold">{votingPower.toLocaleString()}</p>
            </div>
            <Button variant="ghost" size="icon" className="text-muted-foreground">
              <Info className="w-5 h-5" />
            </Button>
          </div>
          <p className="text-xs text-muted-foreground">
            √({userStake.toLocaleString()} ÉTR × {userCoinage} days coinage)
          </p>
        </div>
      </header>

      {/* Tabs */}
      <div className="px-6 mb-4">
        <div className="flex gap-2 glass rounded-xl p-1">
          <button
            onClick={() => setShowPastVotes(false)}
            className={`flex-1 py-2 px-4 rounded-lg text-sm font-medium transition-colors ${
              !showPastVotes ? "bg-accent text-black" : "text-muted-foreground"
            }`}
          >
            Active Proposals
          </button>
          <button
            onClick={() => setShowPastVotes(true)}
            className={`flex-1 py-2 px-4 rounded-lg text-sm font-medium transition-colors ${
              showPastVotes ? "bg-accent text-black" : "text-muted-foreground"
            }`}
          >
            Past Votes
          </button>
        </div>
      </div>

      {/* Proposals List */}
      <main className="px-6 space-y-4">
        {(showPastVotes ? pastProposals : activeProposals).map((proposal) => {
          const yesPercent = (proposal.yesVotes / proposal.totalVotes) * 100
          const noPercent = (proposal.noVotes / proposal.totalVotes) * 100
          const abstainPercent = (proposal.abstainVotes / proposal.totalVotes) * 100

          return (
            <div key={proposal.id} className="glass rounded-2xl p-5 space-y-4">
              {/* Header */}
              <div className="flex items-start justify-between gap-3">
                <div className="flex-1">
                  <div className="flex items-center gap-2 mb-2">
                    <span
                      className={`text-xs px-2 py-1 rounded-full ${
                        proposal.status === "active"
                          ? "bg-success/20 text-success"
                          : proposal.status === "passed"
                            ? "bg-accent/20 text-accent"
                            : "bg-error/20 text-error"
                      }`}
                    >
                      {proposal.status.charAt(0).toUpperCase() + proposal.status.slice(1)}
                    </span>
                    <span className="text-xs px-2 py-1 rounded-full bg-surface text-muted-foreground">
                      {proposal.category}
                    </span>
                  </div>
                  <h3 className="font-bold text-lg mb-1">Proposal #{proposal.id}</h3>
                  <p className="text-sm font-semibold mb-2">{proposal.title}</p>
                  <p className="text-sm text-muted-foreground line-clamp-2">{proposal.description}</p>
                </div>
              </div>

              {/* Voting Stats */}
              <div className="space-y-2">
                <div className="flex items-center gap-2">
                  <div className="flex-1 h-2 bg-surface rounded-full overflow-hidden flex">
                    <div className="bg-success h-full" style={{ width: `${yesPercent}%` }} />
                    <div className="bg-error h-full" style={{ width: `${noPercent}%` }} />
                    <div className="bg-muted-foreground/30 h-full" style={{ width: `${abstainPercent}%` }} />
                  </div>
                </div>
                <div className="flex items-center justify-between text-xs">
                  <div className="flex items-center gap-4">
                    <span className="text-success">
                      Yes {yesPercent.toFixed(1)}% ({(proposal.yesVotes / 1000000).toFixed(1)}M)
                    </span>
                    <span className="text-error">
                      No {noPercent.toFixed(1)}% ({(proposal.noVotes / 1000000).toFixed(1)}M)
                    </span>
                    <span className="text-muted-foreground">Abstain {abstainPercent.toFixed(1)}%</span>
                  </div>
                </div>
              </div>

              {/* Time & Action */}
              <div className="flex items-center justify-between pt-2">
                <span className="text-sm text-muted-foreground">{proposal.timeLeft} remaining</span>
                {proposal.status === "active" ? (
                  <Button
                    onClick={() => setSelectedProposal(proposal)}
                    style={{ background: "#00d9ff", color: "#000" }}
                    className="font-semibold"
                  >
                    Vote
                  </Button>
                ) : proposal.userVoted ? (
                  <span className="text-sm text-accent">You voted: {proposal.userVoted}</span>
                ) : null}
              </div>
            </div>
          )
        })}

        {/* Info Banner */}
        <div className="glass rounded-2xl p-4 bg-accent/10 border-accent/20">
          <div className="flex items-center gap-3">
            <TrendingUp className="w-5 h-5 text-accent shrink-0" />
            <div>
              <p className="text-sm font-semibold">Earn distribution pay by voting!</p>
              <button className="text-xs text-accent hover:underline">Learn about rewards</button>
            </div>
          </div>
        </div>
      </main>

      {/* Vote Modal */}
      {selectedProposal && (
        <VoteModal
          proposal={selectedProposal}
          votingPower={votingPower}
          onClose={() => setSelectedProposal(null)}
          onVote={(vote) => {
            console.log(`Voted ${vote} on proposal ${selectedProposal.id}`)
            setSelectedProposal(null)
          }}
        />
      )}

      {/* Bottom Navigation */}
      <BottomNav activeTab={activeTab} onTabChange={onTabChange} />
    </div>
  )
}
