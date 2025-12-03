"use client"

import { Card } from "@/components/ui/card"
import { Calendar, BookOpen, History, Gift, TrendingUp, Users, Vote, Loader2 } from "lucide-react"
import Link from "next/link"
import { useGovernanceStats, useConsensusDayPhase, useReward, formatETR } from "@/lib/governance/hooks"

interface SidebarProps {
  userAddress?: string
}

export default function Sidebar({ userAddress }: SidebarProps) {
  const { stats, loading: loadingStats } = useGovernanceStats()
  const { phase, loading: loadingPhase } = useConsensusDayPhase()
  const { reward, loading: loadingReward } = useReward(userAddress || null)

  // Calculate next Consensus Day date (estimated)
  const getNextConsensusDay = () => {
    if (phase.isActive) {
      return "In Progress"
    }
    // Estimate based on blocks remaining (6 second block time)
    const daysUntilNext = Math.ceil((phase.blocksRemaining * 6) / 86400)
    const nextDate = new Date()
    nextDate.setDate(nextDate.getDate() + daysUntilNext)
    return nextDate.toLocaleDateString("en-US", { month: "long", day: "numeric", year: "numeric" })
  }

  return (
    <div className="space-y-6 sticky top-24">
      {/* Governance Stats */}
      <Card className="p-6 glass-card">
        <div className="flex items-center gap-2 mb-4">
          <TrendingUp className="w-5 h-5 text-accent" />
          <h3 className="font-bold">Governance Stats</h3>
        </div>
        {loadingStats ? (
          <div className="flex justify-center py-4">
            <Loader2 className="w-6 h-6 animate-spin text-accent" />
          </div>
        ) : (
          <div className="space-y-3 text-sm">
            <div className="flex justify-between items-center">
              <span className="text-muted-foreground">Active Proposals</span>
              <span className="font-bold text-accent">{stats.activeProposals}</span>
            </div>
            <div className="flex justify-between items-center">
              <span className="text-muted-foreground">Total Proposals</span>
              <span className="font-semibold">{stats.totalProposals}</span>
            </div>
            <div className="flex justify-between items-center">
              <span className="text-muted-foreground">Unique Voters</span>
              <span className="font-semibold">{stats.uniqueVoters.toLocaleString()}</span>
            </div>
            <div className="flex justify-between items-center">
              <span className="text-muted-foreground">Quorum Threshold</span>
              <span className="font-semibold">{stats.quorumThresholdPercent}%</span>
            </div>
          </div>
        )}
      </Card>

      {/* Consensus Day Phase */}
      <Card className="p-6 glass-card">
        <div className="flex items-center gap-2 mb-4">
          <Calendar className="w-5 h-5 text-accent" />
          <h3 className="font-bold">Consensus Day</h3>
        </div>
        {loadingPhase ? (
          <div className="flex justify-center py-4">
            <Loader2 className="w-6 h-6 animate-spin text-accent" />
          </div>
        ) : (
          <div className="space-y-3 text-sm">
            <div>
              <p className="text-muted-foreground">Current Phase</p>
              <p className="font-semibold text-lg">
                {phase.isActive ? phase.phase : "Inactive"}
              </p>
            </div>
            <div>
              <p className="text-muted-foreground">Next Consensus Day</p>
              <p className="font-semibold">{getNextConsensusDay()}</p>
            </div>
            {phase.isActive && (
              <div>
                <p className="text-muted-foreground">Blocks Remaining</p>
                <p className="font-semibold">{phase.blocksRemaining.toLocaleString()}</p>
              </div>
            )}
          </div>
        )}
      </Card>

      {/* Your Rewards */}
      {userAddress && (
        <Card className="p-6 bg-accent/5 border-accent/20 glass-card">
          <div className="flex items-center gap-2 mb-4">
            <Gift className="w-5 h-5 text-accent" />
            <h3 className="font-bold">Your Rewards</h3>
          </div>
          {loadingReward ? (
            <div className="flex justify-center py-4">
              <Loader2 className="w-6 h-6 animate-spin text-accent" />
            </div>
          ) : (
            <>
              <p className="text-sm text-muted-foreground mb-2">Pending participation reward</p>
              <p className="text-3xl font-bold text-accent mb-4">
                {formatETR(reward)} ETR
              </p>
              <p className="text-xs text-muted-foreground">
                Earned by participating in governance
              </p>
            </>
          )}
        </Card>
      )}

      {/* Resources */}
      <Card className="p-6 glass-card">
        <div className="flex items-center gap-2 mb-4">
          <BookOpen className="w-5 h-5 text-accent" />
          <h3 className="font-bold">Resources</h3>
        </div>
        <div className="space-y-2">
          <Link
            href="https://docs.etrid.org/governance/voting"
            target="_blank"
            rel="noopener noreferrer"
            className="flex items-center gap-2 text-sm text-muted-foreground hover:text-accent transition-colors"
          >
            <Vote className="w-4 h-4" />
            <span>How voting works</span>
          </Link>
          <Link
            href="https://docs.etrid.org/governance/proposals"
            target="_blank"
            rel="noopener noreferrer"
            className="flex items-center gap-2 text-sm text-muted-foreground hover:text-accent transition-colors"
          >
            <BookOpen className="w-4 h-4" />
            <span>Proposal guidelines</span>
          </Link>
          <Link
            href="https://docs.etrid.org/governance/history"
            target="_blank"
            rel="noopener noreferrer"
            className="flex items-center gap-2 text-sm text-muted-foreground hover:text-accent transition-colors"
          >
            <History className="w-4 h-4" />
            <span>Past Consensus Days</span>
          </Link>
        </div>
      </Card>
    </div>
  )
}
