"use client"

import { useState } from "react"
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card"
import { Button } from "@/components/ui/button"
import { Badge } from "@/components/ui/badge"
import { Vote, Users, Calendar, Shield, ExternalLink, TrendingUp } from "lucide-react"
import Link from "next/link"

export default function GovernancePage() {
  const [stats] = useState({
    activeProposals: 12,
    yourVotes: 8,
    votingPower: "1,250",
    totalVoters: 2456,
    participation: 67,
    nextConsensusDay: "June 1, 2026"
  })

  const recentProposals = [
    {
      id: 3,
      title: "Increase Annual Inflation Rate to 3%",
      status: "active" as const,
      votesFor: 6500000,
      votesAgainst: 3000000,
      endsIn: "4 days"
    },
    {
      id: 2,
      title: "Implement EIP-4844 Proto-Danksharding",
      status: "active" as const,
      votesFor: 8200000,
      votesAgainst: 1500000,
      endsIn: "3 days"
    },
    {
      id: 1,
      title: "Allocate 5M ÉTR to Developer Grants",
      status: "active" as const,
      votesFor: 4500000,
      votesAgainst: 4000000,
      endsIn: "2 days"
    }
  ]

  return (
    <div className="container mx-auto p-8">
      <div className="mb-8">
        <div className="flex items-center gap-3 mb-2">
          <Vote className="h-8 w-8 text-indigo-500" />
          <h1 className="text-4xl font-bold bg-gradient-to-r from-indigo-600 to-purple-600 bg-clip-text text-transparent">
            Governance Overview
          </h1>
        </div>
        <p className="text-zinc-600 dark:text-zinc-400">
          Participate in protocol governance and shape the future of Etrid
        </p>
      </div>

      {/* Quick Stats */}
      <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-4 mb-8">
        <Card>
          <CardHeader className="flex flex-row items-center justify-between space-y-0 pb-2">
            <CardTitle className="text-sm font-medium">Active Proposals</CardTitle>
            <Vote className="h-4 w-4 text-muted-foreground" />
          </CardHeader>
          <CardContent>
            <div className="text-2xl font-bold">{stats.activeProposals}</div>
            <p className="text-xs text-muted-foreground">Awaiting votes</p>
          </CardContent>
        </Card>

        <Card>
          <CardHeader className="flex flex-row items-center justify-between space-y-0 pb-2">
            <CardTitle className="text-sm font-medium">Your Votes</CardTitle>
            <TrendingUp className="h-4 w-4 text-muted-foreground" />
          </CardHeader>
          <CardContent>
            <div className="text-2xl font-bold">{stats.yourVotes}</div>
            <p className="text-xs text-muted-foreground">Participation history</p>
          </CardContent>
        </Card>

        <Card>
          <CardHeader className="flex flex-row items-center justify-between space-y-0 pb-2">
            <CardTitle className="text-sm font-medium">Voting Power</CardTitle>
            <Shield className="h-4 w-4 text-muted-foreground" />
          </CardHeader>
          <CardContent>
            <div className="text-2xl font-bold">{stats.votingPower} ÉTR</div>
            <p className="text-xs text-muted-foreground">Based on stake</p>
          </CardContent>
        </Card>

        <Card>
          <CardHeader className="flex flex-row items-center justify-between space-y-0 pb-2">
            <CardTitle className="text-sm font-medium">Participation</CardTitle>
            <Users className="h-4 w-4 text-muted-foreground" />
          </CardHeader>
          <CardContent>
            <div className="text-2xl font-bold">{stats.participation}%</div>
            <p className="text-xs text-muted-foreground">{stats.totalVoters} voters</p>
          </CardContent>
        </Card>
      </div>

      {/* Navigation Cards */}
      <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6 mb-8">
        <Link href="/governance/snapshot">
          <Card className="hover:border-accent/50 transition-all cursor-pointer h-full">
            <CardHeader>
              <CardTitle className="flex items-center gap-2">
                <Vote className="h-5 w-5 text-indigo-500" />
                Active Proposals
              </CardTitle>
            </CardHeader>
            <CardContent>
              <p className="text-sm text-muted-foreground mb-4">
                View and vote on active governance proposals using Snapshot
              </p>
              <div className="flex items-center gap-2 text-sm text-accent">
                View All Proposals
                <ExternalLink className="h-4 w-4" />
              </div>
            </CardContent>
          </Card>
        </Link>

        <Link href="/governance/directors">
          <Card className="hover:border-accent/50 transition-all cursor-pointer h-full">
            <CardHeader>
              <CardTitle className="flex items-center gap-2">
                <Shield className="h-5 w-5 text-purple-500" />
                Director Dashboard
              </CardTitle>
            </CardHeader>
            <CardContent>
              <p className="text-sm text-muted-foreground mb-4">
                Emergency proposals and multi-sig approvals (9 directors)
              </p>
              <Badge variant="outline">Requires Director Role</Badge>
            </CardContent>
          </Card>
        </Link>

        <Link href="/governance/consensus-day">
          <Card className="hover:border-accent/50 transition-all cursor-pointer h-full">
            <CardHeader>
              <CardTitle className="flex items-center gap-2">
                <Calendar className="h-5 w-5 text-amber-500" />
                Consensus Day
              </CardTitle>
            </CardHeader>
            <CardContent>
              <p className="text-sm text-muted-foreground mb-4">
                Annual governance event for constitution changes
              </p>
              <p className="text-sm font-medium">Next: {stats.nextConsensusDay}</p>
            </CardContent>
          </Card>
        </Link>
      </div>

      {/* Recent Proposals */}
      <Card>
        <CardHeader>
          <div className="flex items-center justify-between">
            <CardTitle>Recent Proposals</CardTitle>
            <Link href="/governance/snapshot">
              <Button variant="outline" size="sm">
                View All
              </Button>
            </Link>
          </div>
        </CardHeader>
        <CardContent>
          <div className="space-y-4">
            {recentProposals.map((proposal) => {
              const total = proposal.votesFor + proposal.votesAgainst
              const forPercentage = (proposal.votesFor / total) * 100
              const againstPercentage = (proposal.votesAgainst / total) * 100

              return (
                <div key={proposal.id} className="border-b last:border-0 pb-4 last:pb-0">
                  <div className="flex items-start justify-between mb-2">
                    <div className="flex items-center gap-2">
                      <span className="text-sm font-mono text-muted-foreground">#{proposal.id}</span>
                      <h4 className="font-semibold">{proposal.title}</h4>
                    </div>
                    <Badge variant="outline" className="bg-green-500/10 text-green-500">
                      {proposal.status}
                    </Badge>
                  </div>

                  <div className="space-y-2">
                    <div className="flex h-2 rounded-full overflow-hidden bg-muted">
                      <div
                        className="bg-green-500 transition-all duration-1000"
                        style={{ width: `${forPercentage}%` }}
                      />
                      <div
                        className="bg-red-500 transition-all duration-1000"
                        style={{ width: `${againstPercentage}%` }}
                      />
                    </div>

                    <div className="flex justify-between text-xs">
                      <span className="text-green-500">
                        {(proposal.votesFor / 1000000).toFixed(1)}M For ({forPercentage.toFixed(1)}%)
                      </span>
                      <span className="text-muted-foreground">Ends in {proposal.endsIn}</span>
                      <span className="text-red-500">
                        {(proposal.votesAgainst / 1000000).toFixed(1)}M Against ({againstPercentage.toFixed(1)}%)
                      </span>
                    </div>
                  </div>
                </div>
              )
            })}
          </div>
        </CardContent>
      </Card>

      {/* Info Banner */}
      <Card className="mt-8 bg-indigo-500/10 border-indigo-500/20">
        <CardContent className="pt-6">
          <div className="flex items-start gap-4">
            <Vote className="h-6 w-6 text-indigo-500 flex-shrink-0 mt-1" />
            <div>
              <h3 className="font-semibold mb-2">How Governance Works</h3>
              <p className="text-sm text-muted-foreground mb-4">
                Etrid uses a hybrid governance model combining Snapshot voting with a 9-member Director Board. Regular
                proposals use quadratic voting based on your staked tokens. Constitution changes and emergency
                proposals require Director approval with a 6/9 quorum.
              </p>
              <Link href="/governance/snapshot">
                <Button variant="outline" size="sm">
                  Learn More
                </Button>
              </Link>
            </div>
          </div>
        </CardContent>
      </Card>
    </div>
  )
}
