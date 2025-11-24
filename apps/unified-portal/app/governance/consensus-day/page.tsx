"use client"

import { useState } from "react"
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card"
import { Button } from "@/components/ui/button"
import { Badge } from "@/components/ui/badge"
import { Alert, AlertDescription } from "@/components/ui/alert"
import { Progress } from "@/components/ui/progress"
import {
  Calendar,
  Users,
  Shield,
  AlertCircle,
  TrendingUp,
  Clock,
  CheckCircle2,
  FileText,
  Info
} from "lucide-react"

interface ConsensusDayProposal {
  id: number
  type: "constitution" | "director-election" | "protocol-change"
  title: string
  description: string
  requiredSupermajority: number
  currentApproval: number
  totalVotes: number
  votesFor: number
  votesAgainst: number
  votesAbstain: number
  status: "active" | "passed" | "failed"
  userVoted: boolean
}

export default function ConsensusDayPage() {
  const [consensusDayInfo] = useState({
    nextDate: "June 1, 2026",
    daysUntil: 190,
    isActive: false, // True only on June 1st
    totalParticipants: 0,
    requiredParticipation: 51,
    currentYear: 2026
  })

  const [proposals] = useState<ConsensusDayProposal[]>([
    {
      id: 1,
      type: "constitution",
      title: "Amendment: Add Environmental Sustainability Clause",
      description:
        "Add Article 12 to the Protocol Constitution requiring all validators to use renewable energy sources by 2028, with phased implementation milestones.",
      requiredSupermajority: 75,
      currentApproval: 68,
      totalVotes: 15000000,
      votesFor: 10200000,
      votesAgainst: 4500000,
      votesAbstain: 300000,
      status: "active",
      userVoted: false
    },
    {
      id: 2,
      type: "director-election",
      title: "Director Election - Seat #5 (Epsilon Position)",
      description:
        "Annual election for Director Seat #5. Current director eligible for re-election. Three candidates nominated by the community.",
      requiredSupermajority: 50,
      currentApproval: 52,
      totalVotes: 15000000,
      votesFor: 7800000,
      votesAgainst: 6900000,
      votesAbstain: 300000,
      status: "active",
      userVoted: false
    },
    {
      id: 3,
      type: "protocol-change",
      title: "Implement Liquid Democracy for Regular Governance",
      description:
        "Allow token holders to delegate their voting power to trusted community members, creating a liquid democracy system for non-Consensus Day proposals.",
      requiredSupermajority: 67,
      currentApproval: 71,
      totalVotes: 15000000,
      votesFor: 10650000,
      votesAgainst: 4200000,
      votesAbstain: 150000,
      status: "active",
      userVoted: true
    }
  ])

  const [candidates] = useState([
    {
      id: 1,
      name: "Alice Nakamoto",
      address: "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY",
      votes: 4200000,
      percentage: 54,
      bio: "Core developer with 8 years blockchain experience. Current Director seeking re-election.",
      isIncumbent: true
    },
    {
      id: 2,
      name: "Bob Finney",
      address: "5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty",
      votes: 2100000,
      percentage: 27,
      bio: "DeFi protocol founder and security researcher. Nominated by community.",
      isIncumbent: false
    },
    {
      id: 3,
      name: "Carol Szabo",
      address: "5FLSigC9HGRKVhB9FiEo4Y3koPsNmBmLJbpXg2mp1hXcS59Y",
      votes: 1500000,
      percentage: 19,
      bio: "Academic researcher in distributed consensus. New candidate.",
      isIncumbent: false
    }
  ])

  return (
    <div className="container mx-auto p-8">
      <div className="mb-8">
        <div className="flex items-center gap-3 mb-2">
          <Calendar className="h-8 w-8 text-amber-500" />
          <h1 className="text-4xl font-bold bg-gradient-to-r from-amber-600 to-orange-600 bg-clip-text text-transparent">
            Consensus Day {consensusDayInfo.currentYear}
          </h1>
        </div>
        <p className="text-zinc-600 dark:text-zinc-400">
          Annual governance event for constitution amendments and director elections
        </p>
      </div>

      {/* Consensus Day Status */}
      {!consensusDayInfo.isActive ? (
        <Alert className="mb-8 bg-amber-500/10 border-amber-500/20">
          <Calendar className="h-4 w-4 text-amber-500" />
          <AlertDescription className="text-amber-500">
            <strong>Upcoming Event:</strong> The next Consensus Day is scheduled for{" "}
            <strong>{consensusDayInfo.nextDate}</strong> ({consensusDayInfo.daysUntil} days remaining). Voting will be
            enabled on that date only.
          </AlertDescription>
        </Alert>
      ) : (
        <Alert className="mb-8 bg-green-500/10 border-green-500/20">
          <CheckCircle2 className="h-4 w-4 text-green-500" />
          <AlertDescription className="text-green-500">
            <strong>Consensus Day is ACTIVE!</strong> Voting is now open for all proposals. Voting closes at 23:59:59
            UTC tonight.
          </AlertDescription>
        </Alert>
      )}

      {/* Key Stats */}
      <div className="grid grid-cols-1 md:grid-cols-4 gap-4 mb-8">
        <Card>
          <CardHeader className="flex flex-row items-center justify-between space-y-0 pb-2">
            <CardTitle className="text-sm font-medium">Next Event</CardTitle>
            <Calendar className="h-4 w-4 text-muted-foreground" />
          </CardHeader>
          <CardContent>
            <div className="text-2xl font-bold">{consensusDayInfo.daysUntil}</div>
            <p className="text-xs text-muted-foreground">days until {consensusDayInfo.nextDate}</p>
          </CardContent>
        </Card>

        <Card>
          <CardHeader className="flex flex-row items-center justify-between space-y-0 pb-2">
            <CardTitle className="text-sm font-medium">Active Proposals</CardTitle>
            <FileText className="h-4 w-4 text-muted-foreground" />
          </CardHeader>
          <CardContent>
            <div className="text-2xl font-bold">{proposals.length}</div>
            <p className="text-xs text-muted-foreground">
              {proposals.filter((p) => p.type === "constitution").length} constitution
            </p>
          </CardContent>
        </Card>

        <Card>
          <CardHeader className="flex flex-row items-center justify-between space-y-0 pb-2">
            <CardTitle className="text-sm font-medium">Participation</CardTitle>
            <Users className="h-4 w-4 text-muted-foreground" />
          </CardHeader>
          <CardContent>
            <div className="text-2xl font-bold">{consensusDayInfo.currentYear === 2026 ? "0%" : "72%"}</div>
            <p className="text-xs text-muted-foreground">
              {consensusDayInfo.requiredParticipation}% required minimum
            </p>
          </CardContent>
        </Card>

        <Card>
          <CardHeader className="flex flex-row items-center justify-between space-y-0 pb-2">
            <CardTitle className="text-sm font-medium">Your Votes</CardTitle>
            <CheckCircle2 className="h-4 w-4 text-muted-foreground" />
          </CardHeader>
          <CardContent>
            <div className="text-2xl font-bold">{proposals.filter((p) => p.userVoted).length}/{proposals.length}</div>
            <p className="text-xs text-muted-foreground">proposals voted</p>
          </CardContent>
        </Card>
      </div>

      {/* How Consensus Day Works */}
      <Card className="mb-8 bg-blue-500/10 border-blue-500/20">
        <CardContent className="pt-6">
          <div className="flex items-start gap-4">
            <Info className="h-6 w-6 text-blue-500 flex-shrink-0 mt-1" />
            <div className="space-y-3">
              <h3 className="font-semibold text-lg">How Consensus Day Works</h3>
              <p className="text-sm text-muted-foreground">
                Consensus Day occurs annually on <strong>June 1st</strong>. It is the only day when the following
                critical governance actions can occur:
              </p>
              <ul className="text-sm text-muted-foreground space-y-2 ml-4">
                <li className="flex items-start gap-2">
                  <Shield className="w-4 h-4 mt-0.5 text-blue-500 flex-shrink-0" />
                  <span>
                    <strong>Constitution Amendments:</strong> Require 75% supermajority approval
                  </span>
                </li>
                <li className="flex items-start gap-2">
                  <Users className="w-4 h-4 mt-0.5 text-blue-500 flex-shrink-0" />
                  <span>
                    <strong>Director Elections:</strong> Elect or re-elect the 9-member Director Board
                  </span>
                </li>
                <li className="flex items-start gap-2">
                  <TrendingUp className="w-4 h-4 mt-0.5 text-blue-500 flex-shrink-0" />
                  <span>
                    <strong>Protocol Changes:</strong> Major protocol changes requiring 67% supermajority
                  </span>
                </li>
                <li className="flex items-start gap-2">
                  <AlertCircle className="w-4 h-4 mt-0.5 text-blue-500 flex-shrink-0" />
                  <span>
                    <strong>Participation Threshold:</strong> Minimum 51% of all staked tokens must participate for
                    results to be valid
                  </span>
                </li>
              </ul>
            </div>
          </div>
        </CardContent>
      </Card>

      {/* Active Proposals */}
      <div className="space-y-6 mb-8">
        <h2 className="text-2xl font-bold">Active Proposals</h2>

        {proposals.map((proposal) => {
          const forPercentage = (proposal.votesFor / proposal.totalVotes) * 100
          const againstPercentage = (proposal.votesAgainst / proposal.totalVotes) * 100
          const abstainPercentage = (proposal.votesAbstain / proposal.totalVotes) * 100
          const isPassing = proposal.currentApproval >= proposal.requiredSupermajority

          return (
            <Card key={proposal.id}>
              <CardHeader>
                <div className="flex items-start justify-between">
                  <div className="space-y-2">
                    <div className="flex items-center gap-2">
                      <span className="text-sm font-mono text-muted-foreground">#{proposal.id}</span>
                      <Badge
                        variant="outline"
                        className={
                          proposal.type === "constitution"
                            ? "bg-purple-500/10 text-purple-500"
                            : proposal.type === "director-election"
                              ? "bg-blue-500/10 text-blue-500"
                              : "bg-amber-500/10 text-amber-500"
                        }
                      >
                        {proposal.type === "constitution"
                          ? "Constitution"
                          : proposal.type === "director-election"
                            ? "Director Election"
                            : "Protocol Change"}
                      </Badge>
                      <Badge
                        variant="outline"
                        className={isPassing ? "bg-green-500/10 text-green-500" : "bg-red-500/10 text-red-500"}
                      >
                        {isPassing ? "Passing" : "Failing"} ({proposal.currentApproval}%)
                      </Badge>
                    </div>
                    <h3 className="text-xl font-bold">{proposal.title}</h3>
                  </div>
                </div>
              </CardHeader>
              <CardContent className="space-y-4">
                <p className="text-sm text-muted-foreground">{proposal.description}</p>

                {/* Voting Progress */}
                <div className="space-y-3">
                  <div className="flex items-center justify-between text-sm">
                    <span className="text-muted-foreground">
                      Supermajority Required: {proposal.requiredSupermajority}%
                    </span>
                    <span className="font-semibold">
                      Current: {proposal.currentApproval}% ({isPassing ? "✓ Passing" : "✗ Failing"})
                    </span>
                  </div>

                  <div className="flex h-3 rounded-full overflow-hidden bg-muted">
                    <div
                      className="bg-green-500 transition-all duration-1000"
                      style={{ width: `${forPercentage}%` }}
                    />
                    <div
                      className="bg-red-500 transition-all duration-1000"
                      style={{ width: `${againstPercentage}%` }}
                    />
                    <div
                      className="bg-gray-500 transition-all duration-1000"
                      style={{ width: `${abstainPercentage}%` }}
                    />
                  </div>

                  <div className="grid grid-cols-3 gap-2 text-xs">
                    <div className="text-center">
                      <div className="font-semibold text-green-500">
                        {(proposal.votesFor / 1000000).toFixed(1)}M
                      </div>
                      <div className="text-muted-foreground">For ({forPercentage.toFixed(1)}%)</div>
                    </div>
                    <div className="text-center">
                      <div className="font-semibold text-red-500">
                        {(proposal.votesAgainst / 1000000).toFixed(1)}M
                      </div>
                      <div className="text-muted-foreground">Against ({againstPercentage.toFixed(1)}%)</div>
                    </div>
                    <div className="text-center">
                      <div className="font-semibold text-gray-500">
                        {(proposal.votesAbstain / 1000000).toFixed(1)}M
                      </div>
                      <div className="text-muted-foreground">Abstain ({abstainPercentage.toFixed(1)}%)</div>
                    </div>
                  </div>
                </div>

                {/* Voting Actions */}
                {consensusDayInfo.isActive && !proposal.userVoted && (
                  <div className="grid grid-cols-3 gap-2">
                    <Button className="w-full" variant="default">
                      Vote For
                    </Button>
                    <Button className="w-full" variant="destructive">
                      Vote Against
                    </Button>
                    <Button className="w-full" variant="outline">
                      Abstain
                    </Button>
                  </div>
                )}

                {proposal.userVoted && (
                  <Alert className="bg-green-500/10 border-green-500/20">
                    <CheckCircle2 className="h-4 w-4 text-green-500" />
                    <AlertDescription className="text-green-500">
                      You have voted on this proposal
                    </AlertDescription>
                  </Alert>
                )}

                {!consensusDayInfo.isActive && (
                  <Alert>
                    <Clock className="h-4 w-4" />
                    <AlertDescription>
                      Voting opens on {consensusDayInfo.nextDate} at 00:00:00 UTC
                    </AlertDescription>
                  </Alert>
                )}
              </CardContent>
            </Card>
          )
        })}
      </div>

      {/* Director Election Details */}
      <Card>
        <CardHeader>
          <CardTitle>Director Seat #5 - Candidate Information</CardTitle>
        </CardHeader>
        <CardContent>
          <div className="space-y-4">
            {candidates.map((candidate) => (
              <div key={candidate.id} className="border rounded-lg p-4">
                <div className="flex items-start justify-between mb-3">
                  <div className="flex items-center gap-3">
                    <div className="w-12 h-12 rounded-full bg-gradient-to-br from-purple-500 to-blue-500 flex items-center justify-center text-white font-bold text-lg">
                      {candidate.name.charAt(0)}
                    </div>
                    <div>
                      <div className="flex items-center gap-2">
                        <h4 className="font-bold text-lg">{candidate.name}</h4>
                        {candidate.isIncumbent && (
                          <Badge variant="outline" className="bg-blue-500/10 text-blue-500">
                            Incumbent
                          </Badge>
                        )}
                      </div>
                      <p className="text-xs text-muted-foreground font-mono">{candidate.address}</p>
                    </div>
                  </div>
                  <div className="text-right">
                    <div className="text-2xl font-bold">{candidate.percentage}%</div>
                    <p className="text-xs text-muted-foreground">
                      {(candidate.votes / 1000000).toFixed(1)}M votes
                    </p>
                  </div>
                </div>

                <p className="text-sm text-muted-foreground mb-3">{candidate.bio}</p>

                <Progress value={candidate.percentage} className="h-2" />
              </div>
            ))}
          </div>
        </CardContent>
      </Card>
    </div>
  )
}
