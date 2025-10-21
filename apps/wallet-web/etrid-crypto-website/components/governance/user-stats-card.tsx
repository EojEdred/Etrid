"use client"

import { Card } from "@/components/ui/card"
import { Button } from "@/components/ui/button"
import { Info } from "lucide-react"
import { Tooltip, TooltipContent, TooltipProvider, TooltipTrigger } from "@/components/ui/tooltip"

interface UserStatsCardProps {
  address: string
  balance: string
}

export default function UserStatsCard({ address, balance }: UserStatsCardProps) {
  // Parse balance for calculations
  const stake = parseFloat(balance) || 0

  // Mock coinage for now (in production, this would come from on-chain staking data)
  // Assuming user has been staking for 180 days as an example
  const coinage = 180

  // Calculate voting power using Ascending Scale of Finality formula: √(Stake × Coinage)
  const votingPower = Math.sqrt(stake * coinage)

  return (
    <Card className="p-6 mb-6 bg-card/50 backdrop-blur-sm border-border">
      <div className="flex flex-col md:flex-row items-start md:items-center justify-between gap-4">
        <div>
          <div className="flex items-center gap-2 mb-2">
            <h3 className="text-sm text-muted-foreground">Your Voting Power</h3>
            <TooltipProvider>
              <Tooltip>
                <TooltipTrigger>
                  <Info className="w-4 h-4 text-muted-foreground" />
                </TooltipTrigger>
                <TooltipContent>
                  <p className="max-w-xs">
                    Voting power is calculated using the Ascending Scale of Finality formula: √(Stake × Coinage).
                    Coinage is the number of days your tokens have been staked.
                  </p>
                </TooltipContent>
              </Tooltip>
            </TooltipProvider>
          </div>
          <p className="text-4xl font-bold text-accent mb-2">
            {votingPower.toLocaleString(undefined, { maximumFractionDigits: 0 })} votes
          </p>
          <p className="text-sm text-muted-foreground mb-3">Formula: √(Stake × Coinage)</p>
          <div className="space-y-1 text-sm">
            <p>
              <span className="text-muted-foreground">Staked:</span>{" "}
              <span className="font-semibold">
                {stake.toLocaleString(undefined, { minimumFractionDigits: 2, maximumFractionDigits: 4 })} ÉTR
              </span>
            </p>
            <p>
              <span className="text-muted-foreground">Coinage:</span> <span className="font-semibold">{coinage} days</span>
            </p>
            <p>
              <span className="text-muted-foreground">Power:</span>{" "}
              <span className="font-semibold">
                √({stake.toFixed(0)} × {coinage}) = {votingPower.toLocaleString(undefined, { maximumFractionDigits: 0 })}
              </span>
            </p>
          </div>
        </div>
        <Button className="whitespace-nowrap">Increase stake to boost voting power</Button>
      </div>
    </Card>
  )
}
