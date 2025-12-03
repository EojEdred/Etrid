"use client"

import { Card } from "@/components/ui/card"
import { Button } from "@/components/ui/button"
import { Info, Loader2, TrendingUp } from "lucide-react"
import { Tooltip, TooltipContent, TooltipProvider, TooltipTrigger } from "@/components/ui/tooltip"
import { useVotingPower, formatETR } from "@/lib/governance/hooks"
import Link from "next/link"

interface UserStatsCardProps {
  address: string
}

export default function UserStatsCard({ address }: UserStatsCardProps) {
  const { votingPower, loading, error } = useVotingPower(address)

  if (loading) {
    return (
      <Card className="p-6 mb-6 glass-card">
        <div className="flex justify-center py-8">
          <Loader2 className="w-8 h-8 animate-spin text-accent" />
        </div>
      </Card>
    )
  }

  if (error) {
    return (
      <Card className="p-6 mb-6 glass-card border-red-500/20">
        <p className="text-red-400 text-sm">Error loading voting power: {error.message}</p>
      </Card>
    )
  }

  if (!votingPower) {
    return null
  }

  const stakedAmount = formatETR(votingPower.stakedAmount || "0")
  const totalVotingPower = votingPower.votingPower || "0"
  const participationCount = votingPower.participationHistory || 0
  const canVote = votingPower.canVote

  return (
    <Card className="p-6 mb-6 glass-card border-accent/20">
      <div className="flex flex-col md:flex-row items-start md:items-center justify-between gap-4">
        <div className="flex-1">
          <div className="flex items-center gap-2 mb-2">
            <h3 className="text-sm text-muted-foreground">Your Voting Power</h3>
            <TooltipProvider>
              <Tooltip>
                <TooltipTrigger>
                  <Info className="w-4 h-4 text-muted-foreground" />
                </TooltipTrigger>
                <TooltipContent className="max-w-xs">
                  <p>
                    Voting power is calculated based on your staked ETR and participation history.
                    Stake more tokens or participate regularly to increase your influence in governance.
                  </p>
                </TooltipContent>
              </Tooltip>
            </TooltipProvider>
          </div>

          <p className="text-4xl font-bold text-accent mb-3">
            {parseInt(totalVotingPower).toLocaleString()} votes
          </p>

          <div className="space-y-1.5 text-sm">
            <div className="flex items-center justify-between">
              <span className="text-muted-foreground">Staked Amount:</span>
              <span className="font-semibold">{stakedAmount} ETR</span>
            </div>
            <div className="flex items-center justify-between">
              <span className="text-muted-foreground">Participation Count:</span>
              <span className="font-semibold">{participationCount} times</span>
            </div>
            <div className="flex items-center justify-between">
              <span className="text-muted-foreground">Voting Status:</span>
              <span className={`font-semibold ${canVote ? "text-green-500" : "text-amber-500"}`}>
                {canVote ? "Eligible" : "Not Eligible"}
              </span>
            </div>
          </div>

          {!canVote && (
            <p className="text-xs text-amber-500 mt-2">
              You need to stake tokens to participate in governance
            </p>
          )}
        </div>

        <div className="flex flex-col gap-2">
          <Link href="/staking/eth-pbc">
            <Button className="whitespace-nowrap w-full">
              <TrendingUp className="w-4 h-4 mr-2" />
              Stake More ETR
            </Button>
          </Link>
          {participationCount > 0 && (
            <p className="text-xs text-center text-muted-foreground">
              Boost power by staking
            </p>
          )}
        </div>
      </div>
    </Card>
  )
}
