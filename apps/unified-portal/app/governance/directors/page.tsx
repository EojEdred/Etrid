"use client"

import { useState } from "react"
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card"
import { Button } from "@/components/ui/button"
import { Badge } from "@/components/ui/badge"
import { Alert, AlertDescription } from "@/components/ui/alert"
import {
  Shield,
  AlertTriangle,
  Clock,
  CheckCircle2,
  XCircle,
  Users,
  FileText,
  Timer
} from "lucide-react"

interface EmergencyProposal {
  id: number
  title: string
  description: string
  proposedBy: string
  createdAt: string
  expiresAt: string
  requiredApprovals: number
  currentApprovals: number
  approvedBy: string[]
  status: "pending" | "approved" | "rejected" | "expired"
  timelockHours: number
}

interface Director {
  id: number
  name: string
  address: string
  joinedDate: string
  proposalsApproved: number
  proposalsRejected: number
  isActive: boolean
}

export default function DirectorDashboardPage() {
  // Mock current user (in production, check wallet address against director list)
  const [currentUser] = useState({
    address: "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY",
    isDirector: true, // Set to false to test non-director view
    directorId: 1
  })

  const [directors] = useState<Director[]>([
    {
      id: 1,
      name: "Director Alpha",
      address: "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY",
      joinedDate: "2024-01-01",
      proposalsApproved: 45,
      proposalsRejected: 3,
      isActive: true
    },
    {
      id: 2,
      name: "Director Beta",
      address: "5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty",
      joinedDate: "2024-01-01",
      proposalsApproved: 42,
      proposalsRejected: 6,
      isActive: true
    },
    {
      id: 3,
      name: "Director Gamma",
      address: "5FLSigC9HGRKVhB9FiEo4Y3koPsNmBmLJbpXg2mp1hXcS59Y",
      joinedDate: "2024-01-15",
      proposalsApproved: 38,
      proposalsRejected: 10,
      isActive: true
    },
    {
      id: 4,
      name: "Director Delta",
      address: "5DAAnrj7VHTznn2AWBemMuyBwZWs6FNFjdyVXUeYum3PTXFy",
      joinedDate: "2024-02-01",
      proposalsApproved: 35,
      proposalsRejected: 5,
      isActive: true
    },
    {
      id: 5,
      name: "Director Epsilon",
      address: "5HGjWAeFDfFCWPsjFQdVV2Msvz2XtMktvgocEZcCj68kUMaw",
      joinedDate: "2024-02-15",
      proposalsApproved: 33,
      proposalsRejected: 7,
      isActive: true
    },
    {
      id: 6,
      name: "Director Zeta",
      address: "5CiPPseXPECbkjWCa6MnjNokrgYjMqmKndv2rSnekmSK2DjL",
      joinedDate: "2024-03-01",
      proposalsApproved: 30,
      proposalsRejected: 8,
      isActive: true
    },
    {
      id: 7,
      name: "Director Eta",
      address: "5GNJqTPyNqANBkUVMN1LPPrxXnFouWXoe2wNSmmEoLctxiZY",
      joinedDate: "2024-03-15",
      proposalsApproved: 28,
      proposalsRejected: 4,
      isActive: true
    },
    {
      id: 8,
      name: "Director Theta",
      address: "5Ck5SLSHYac6WFt5UZRSsdJjwmpSZq85fd5TRNAdZQVzEAPT",
      joinedDate: "2024-04-01",
      proposalsApproved: 25,
      proposalsRejected: 9,
      isActive: true
    },
    {
      id: 9,
      name: "Director Iota",
      address: "5HpG9w8EBLe5XCrbczpwq5TSXvedjrBGCwqxK1iQ7qUsSWFc",
      joinedDate: "2024-04-15",
      proposalsApproved: 22,
      proposalsRejected: 6,
      isActive: true
    }
  ])

  const [emergencyProposals] = useState<EmergencyProposal[]>([
    {
      id: 1,
      title: "Emergency Network Halt - Critical Bug Detected",
      description:
        "A critical vulnerability has been discovered in the staking pallet that could allow unauthorized token minting. This proposal halts network operations immediately for 24 hours to deploy a patch.",
      proposedBy: "5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty",
      createdAt: "2025-11-22T10:30:00Z",
      expiresAt: "2025-11-23T10:30:00Z",
      requiredApprovals: 6,
      currentApprovals: 4,
      approvedBy: [
        "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY",
        "5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty",
        "5FLSigC9HGRKVhB9FiEo4Y3koPsNmBmLJbpXg2mp1hXcS59Y",
        "5DAAnrj7VHTznn2AWBemMuyBwZWs6FNFjdyVXUeYum3PTXFy"
      ],
      status: "pending",
      timelockHours: 0 // No timelock for critical emergencies
    },
    {
      id: 2,
      title: "Accelerated Runtime Upgrade - v2.4.1 Security Patch",
      description:
        "Deploy runtime upgrade v2.4.1 which includes critical security patches for the XCM module. Standard governance process bypassed due to active exploit attempts.",
      proposedBy: "5DAAnrj7VHTznn2AWBemMuyBwZWs6FNFjdyVXUeYum3PTXFy",
      createdAt: "2025-11-21T14:00:00Z",
      expiresAt: "2025-11-24T14:00:00Z",
      requiredApprovals: 6,
      currentApprovals: 7,
      approvedBy: [
        "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY",
        "5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty",
        "5FLSigC9HGRKVhB9FiEo4Y3koPsNmBmLJbpXg2mp1hXcS59Y",
        "5DAAnrj7VHTznn2AWBemMuyBwZWs6FNFjdyVXUeYum3PTXFy",
        "5HGjWAeFDfFCWPsjFQdVV2Msvz2XtMktvgocEZcCj68kUMaw",
        "5CiPPseXPECbkjWCa6MnjNokrgYjMqmKndv2rSnekmSK2DjL",
        "5GNJqTPyNqANBkUVMN1LPPrxXnFouWXoe2wNSmmEoLctxiZY"
      ],
      status: "approved",
      timelockHours: 24
    }
  ])

  const [actions] = useState([
    {
      id: 1,
      type: "approval",
      proposalId: 2,
      proposalTitle: "Accelerated Runtime Upgrade - v2.4.1",
      director: "Director Alpha",
      timestamp: "2025-11-21T15:30:00Z"
    },
    {
      id: 2,
      type: "rejection",
      proposalId: 3,
      proposalTitle: "Emergency Treasury Disbursement",
      director: "Director Beta",
      timestamp: "2025-11-20T09:15:00Z"
    },
    {
      id: 3,
      type: "approval",
      proposalId: 1,
      proposalTitle: "Emergency Network Halt - Critical Bug",
      director: "Director Gamma",
      timestamp: "2025-11-22T11:00:00Z"
    }
  ])

  const handleApprove = (proposalId: number) => {
    console.log("Approving proposal:", proposalId)
    // In production, this would call a Polkadot.js extrinsic
  }

  const handleReject = (proposalId: number) => {
    console.log("Rejecting proposal:", proposalId)
    // In production, this would call a Polkadot.js extrinsic
  }

  if (!currentUser.isDirector) {
    return (
      <div className="container mx-auto p-8">
        <Alert variant="destructive">
          <Shield className="h-4 w-4" />
          <AlertDescription>
            Access Denied: This dashboard is only accessible to the 9 protocol directors. Your wallet address is not
            registered as a director.
          </AlertDescription>
        </Alert>
      </div>
    )
  }

  return (
    <div className="container mx-auto p-8">
      <div className="mb-8">
        <div className="flex items-center gap-3 mb-2">
          <Shield className="h-8 w-8 text-purple-500" />
          <h1 className="text-4xl font-bold bg-gradient-to-r from-purple-600 to-pink-600 bg-clip-text text-transparent">
            Director Dashboard
          </h1>
        </div>
        <p className="text-zinc-600 dark:text-zinc-400">Emergency proposals and multi-signature approvals</p>
      </div>

      {/* Director Stats */}
      <div className="grid grid-cols-1 md:grid-cols-4 gap-4 mb-8">
        <Card>
          <CardHeader className="flex flex-row items-center justify-between space-y-0 pb-2">
            <CardTitle className="text-sm font-medium">Active Directors</CardTitle>
            <Users className="h-4 w-4 text-muted-foreground" />
          </CardHeader>
          <CardContent>
            <div className="text-2xl font-bold">9/9</div>
            <p className="text-xs text-muted-foreground">All seats filled</p>
          </CardContent>
        </Card>

        <Card>
          <CardHeader className="flex flex-row items-center justify-between space-y-0 pb-2">
            <CardTitle className="text-sm font-medium">Pending Approvals</CardTitle>
            <AlertTriangle className="h-4 w-4 text-amber-500" />
          </CardHeader>
          <CardContent>
            <div className="text-2xl font-bold">1</div>
            <p className="text-xs text-muted-foreground">Requires action</p>
          </CardContent>
        </Card>

        <Card>
          <CardHeader className="flex flex-row items-center justify-between space-y-0 pb-2">
            <CardTitle className="text-sm font-medium">Quorum Required</CardTitle>
            <CheckCircle2 className="h-4 w-4 text-muted-foreground" />
          </CardHeader>
          <CardContent>
            <div className="text-2xl font-bold">6/9</div>
            <p className="text-xs text-muted-foreground">66.7% approval</p>
          </CardContent>
        </Card>

        <Card>
          <CardHeader className="flex flex-row items-center justify-between space-y-0 pb-2">
            <CardTitle className="text-sm font-medium">Your Approvals</CardTitle>
            <FileText className="h-4 w-4 text-muted-foreground" />
          </CardHeader>
          <CardContent>
            <div className="text-2xl font-bold">45</div>
            <p className="text-xs text-muted-foreground">3 rejected</p>
          </CardContent>
        </Card>
      </div>

      {/* Emergency Proposals */}
      <Card className="mb-8">
        <CardHeader>
          <div className="flex items-center justify-between">
            <CardTitle className="flex items-center gap-2">
              <AlertTriangle className="h-5 w-5 text-amber-500" />
              Emergency Proposals
            </CardTitle>
            <Badge variant="outline" className="bg-amber-500/10 text-amber-500">
              {emergencyProposals.filter((p) => p.status === "pending").length} Pending
            </Badge>
          </div>
        </CardHeader>
        <CardContent>
          <div className="space-y-6">
            {emergencyProposals.map((proposal) => {
              const hasUserApproved = proposal.approvedBy.includes(currentUser.address)
              const progressPercentage = (proposal.currentApprovals / proposal.requiredApprovals) * 100
              const isQuorumReached = proposal.currentApprovals >= proposal.requiredApprovals

              return (
                <div key={proposal.id} className="border rounded-lg p-6 space-y-4">
                  <div className="flex items-start justify-between">
                    <div>
                      <div className="flex items-center gap-2 mb-2">
                        <span className="text-sm font-mono text-muted-foreground">#{proposal.id}</span>
                        <Badge
                          variant="outline"
                          className={
                            proposal.status === "approved"
                              ? "bg-green-500/10 text-green-500"
                              : proposal.status === "rejected"
                                ? "bg-red-500/10 text-red-500"
                                : "bg-amber-500/10 text-amber-500"
                          }
                        >
                          {proposal.status}
                        </Badge>
                        {proposal.timelockHours > 0 && (
                          <Badge variant="outline" className="bg-blue-500/10 text-blue-500">
                            <Timer className="w-3 h-3 mr-1" />
                            {proposal.timelockHours}h timelock
                          </Badge>
                        )}
                      </div>
                      <h3 className="text-xl font-bold mb-2">{proposal.title}</h3>
                      <p className="text-sm text-muted-foreground mb-3">{proposal.description}</p>

                      <div className="flex items-center gap-4 text-sm text-muted-foreground">
                        <span>Proposed by {proposal.proposedBy.slice(0, 8)}...</span>
                        <span>•</span>
                        <span>Created {new Date(proposal.createdAt).toLocaleDateString()}</span>
                        <span>•</span>
                        <span className="flex items-center gap-1">
                          <Clock className="w-3 h-3" />
                          Expires {new Date(proposal.expiresAt).toLocaleDateString()}
                        </span>
                      </div>
                    </div>
                  </div>

                  {/* Approval Progress */}
                  <div className="space-y-2">
                    <div className="flex items-center justify-between text-sm">
                      <span className="text-muted-foreground">Approval Progress</span>
                      <span className="font-semibold">
                        {proposal.currentApprovals}/{proposal.requiredApprovals} ({progressPercentage.toFixed(0)}%)
                      </span>
                    </div>
                    <div className="h-2 bg-muted rounded-full overflow-hidden">
                      <div
                        className={`h-full transition-all duration-500 ${isQuorumReached ? "bg-green-500" : "bg-amber-500"}`}
                        style={{ width: `${Math.min(progressPercentage, 100)}%` }}
                      />
                    </div>
                    {isQuorumReached && (
                      <p className="text-sm text-green-500 flex items-center gap-2">
                        <CheckCircle2 className="w-4 h-4" />
                        Quorum reached - Proposal will execute after timelock
                      </p>
                    )}
                  </div>

                  {/* Approved Directors */}
                  <div>
                    <p className="text-sm text-muted-foreground mb-2">Approved by:</p>
                    <div className="flex flex-wrap gap-2">
                      {proposal.approvedBy.map((address, idx) => (
                        <Badge key={idx} variant="outline" className="bg-green-500/10 text-green-500">
                          {directors.find((d) => d.address === address)?.name || `${address.slice(0, 8)}...`}
                        </Badge>
                      ))}
                    </div>
                  </div>

                  {/* Actions */}
                  {proposal.status === "pending" && (
                    <div className="flex gap-3">
                      {hasUserApproved ? (
                        <Badge variant="outline" className="bg-green-500/10 text-green-500">
                          <CheckCircle2 className="w-4 h-4 mr-1" />
                          You approved this proposal
                        </Badge>
                      ) : (
                        <>
                          <Button onClick={() => handleApprove(proposal.id)} className="flex-1">
                            <CheckCircle2 className="w-4 h-4 mr-2" />
                            Approve
                          </Button>
                          <Button onClick={() => handleReject(proposal.id)} variant="destructive" className="flex-1">
                            <XCircle className="w-4 h-4 mr-2" />
                            Reject
                          </Button>
                        </>
                      )}
                    </div>
                  )}
                </div>
              )
            })}
          </div>
        </CardContent>
      </Card>

      {/* Director List */}
      <Card className="mb-8">
        <CardHeader>
          <CardTitle>Director Board (9 Members)</CardTitle>
        </CardHeader>
        <CardContent>
          <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
            {directors.map((director) => (
              <div key={director.id} className="border rounded-lg p-4">
                <div className="flex items-center justify-between mb-3">
                  <div className="flex items-center gap-2">
                    <div className="w-10 h-10 rounded-full bg-gradient-to-br from-purple-500 to-pink-500 flex items-center justify-center text-white font-bold">
                      {director.id}
                    </div>
                    <div>
                      <h4 className="font-semibold">{director.name}</h4>
                      <p className="text-xs text-muted-foreground">{director.address.slice(0, 12)}...</p>
                    </div>
                  </div>
                  {director.isActive && (
                    <Badge variant="outline" className="bg-green-500/10 text-green-500">
                      Active
                    </Badge>
                  )}
                </div>

                <div className="space-y-1 text-sm">
                  <div className="flex justify-between">
                    <span className="text-muted-foreground">Joined:</span>
                    <span>{new Date(director.joinedDate).toLocaleDateString()}</span>
                  </div>
                  <div className="flex justify-between">
                    <span className="text-muted-foreground">Approved:</span>
                    <span className="text-green-500">{director.proposalsApproved}</span>
                  </div>
                  <div className="flex justify-between">
                    <span className="text-muted-foreground">Rejected:</span>
                    <span className="text-red-500">{director.proposalsRejected}</span>
                  </div>
                </div>
              </div>
            ))}
          </div>
        </CardContent>
      </Card>

      {/* Recent Actions */}
      <Card>
        <CardHeader>
          <CardTitle>Recent Director Actions</CardTitle>
        </CardHeader>
        <CardContent>
          <div className="space-y-3">
            {actions.map((action) => (
              <div key={action.id} className="flex items-center justify-between border-b last:border-0 pb-3 last:pb-0">
                <div className="flex items-center gap-3">
                  {action.type === "approval" ? (
                    <CheckCircle2 className="w-5 h-5 text-green-500" />
                  ) : (
                    <XCircle className="w-5 h-5 text-red-500" />
                  )}
                  <div>
                    <p className="font-medium">{action.proposalTitle}</p>
                    <p className="text-sm text-muted-foreground">
                      {action.type === "approval" ? "Approved" : "Rejected"} by {action.director}
                    </p>
                  </div>
                </div>
                <span className="text-sm text-muted-foreground">
                  {new Date(action.timestamp).toLocaleDateString()}
                </span>
              </div>
            ))}
          </div>
        </CardContent>
      </Card>
    </div>
  )
}
