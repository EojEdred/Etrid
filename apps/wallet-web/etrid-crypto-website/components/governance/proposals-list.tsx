"use client"

import { useState, useMemo } from "react"
import ProposalCard from "./proposal-card"
import ProposalModal from "./proposal-modal"
import { useProposals } from "@/lib/governance/hooks"
import { Proposal as ChainProposal } from "@/lib/polkadot/governance"
import { Loader2 } from "lucide-react"

interface ProposalsListProps {
  filter: string
  sortBy: string
  searchQuery: string
  isWalletConnected: boolean
  userAddress?: string
}

// Map category to display name
const categoryDisplayNames: Record<string, string> = {
  InflationRate: "Inflation Rate",
  ParameterChange: "Parameter Change",
  BudgetAllocation: "Budget Allocation",
  ProtocolUpgrade: "Protocol Upgrade",
  DirectorElection: "Director Election",
  EmergencyAction: "Emergency Action",
}

export default function ProposalsList({ filter, sortBy, searchQuery, isWalletConnected, userAddress }: ProposalsListProps) {
  const [selectedProposal, setSelectedProposal] = useState<ChainProposal | null>(null)
  const { proposals, loading, error } = useProposals()

  // Filter, search, and sort proposals
  const filteredProposals = useMemo(() => {
    let filtered = proposals.filter((proposal) => {
      // Filter by status/category
      if (filter === "all") return true
      if (filter === "active") return proposal.status === "Active"
      if (filter === "passed") return proposal.status === "Approved"
      if (filter === "rejected") return proposal.status === "Rejected"
      if (filter === "inflation") return proposal.category === "InflationRate"
      if (filter === "parameter") return proposal.category === "ParameterChange"
      if (filter === "budget") return proposal.category === "BudgetAllocation"
      if (filter === "protocol") return proposal.category === "ProtocolUpgrade"
      if (filter === "director") return proposal.category === "DirectorElection"
      if (filter === "emergency") return proposal.category === "EmergencyAction"
      return true
    })

    // Search filter
    if (searchQuery) {
      filtered = filtered.filter((proposal) =>
        proposal.title.toLowerCase().includes(searchQuery.toLowerCase()) ||
        proposal.description.toLowerCase().includes(searchQuery.toLowerCase())
      )
    }

    // Sort proposals
    filtered = [...filtered].sort((a, b) => {
      if (sortBy === "most-votes") {
        const totalA = BigInt(a.votesFor) + BigInt(a.votesAgainst) + BigInt(a.votesAbstain)
        const totalB = BigInt(b.votesFor) + BigInt(b.votesAgainst) + BigInt(b.votesAbstain)
        return totalB > totalA ? 1 : totalB < totalA ? -1 : 0
      }
      if (sortBy === "ending-soon") return a.id - b.id
      if (sortBy === "recently-added") return b.id - a.id
      return 0
    })

    return filtered
  }, [proposals, filter, sortBy, searchQuery])

  if (!isWalletConnected) {
    return (
      <div className="text-center py-16">
        <p className="text-xl text-muted-foreground">Connect your wallet to participate in governance</p>
      </div>
    )
  }

  if (loading) {
    return (
      <div className="text-center py-16">
        <Loader2 className="w-12 h-12 animate-spin text-accent mx-auto mb-4" />
        <p className="text-xl text-muted-foreground">Loading proposals from chain...</p>
      </div>
    )
  }

  if (error) {
    return (
      <div className="text-center py-16">
        <p className="text-xl text-red-400 mb-2">Error loading proposals</p>
        <p className="text-sm text-muted-foreground">{error}</p>
      </div>
    )
  }

  if (filteredProposals.length === 0) {
    return (
      <div className="text-center py-16">
        <p className="text-xl text-muted-foreground">
          {searchQuery
            ? "No proposals match your search"
            : "No proposals found. Next Consensus Day voting period will open soon."}
        </p>
      </div>
    )
  }

  return (
    <>
      <div className="grid grid-cols-1 lg:grid-cols-2 gap-6">
        {filteredProposals.map((proposal) => (
          <ProposalCard
            key={proposal.id}
            proposal={proposal}
            onClick={() => setSelectedProposal(proposal)}
            userAddress={userAddress}
          />
        ))}
      </div>

      {selectedProposal && (
        <ProposalModal
          proposal={selectedProposal}
          onClose={() => setSelectedProposal(null)}
          userAddress={userAddress}
        />
      )}
    </>
  )
}
