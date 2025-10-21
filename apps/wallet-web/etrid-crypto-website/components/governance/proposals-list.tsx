"use client"

import { useState } from "react"
import ProposalCard from "./proposal-card"
import ProposalModal from "./proposal-modal"

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

const mockProposals: Proposal[] = [
  {
    id: 3,
    title: "Increase Annual Inflation Rate to 3%",
    status: "active",
    category: "Fiscal Policy",
    submittedBy: "0x7a8b...9c0d",
    submissionDate: "March 15, 2026",
    description:
      "This proposal suggests increasing the annual inflation rate from 2% to 3% to fund additional development and ecosystem growth. The additional 1% would be allocated to the treasury for grants and partnerships. This would result in approximately 30M new ÉTR tokens being minted in 2026.",
    votesYes: 6500000,
    votesNo: 3000000,
    votesAbstain: 500000,
    totalVotes: 10000000,
    quorumReached: true,
    quorumPercentage: 67,
    timeLeft: "4 days 12 hours",
    userVoted: true,
    userVote: "yes",
  },
  {
    id: 2,
    title: "Implement EIP-4844 Proto-Danksharding",
    status: "active",
    category: "Protocol Upgrades",
    submittedBy: "0x3c4d...5e6f",
    submissionDate: "March 10, 2026",
    description:
      "Adopt EIP-4844 to reduce transaction costs by introducing blob-carrying transactions. This upgrade will significantly improve scalability and reduce gas fees for Layer 2 solutions built on Ëtrid.",
    votesYes: 8200000,
    votesNo: 1500000,
    votesAbstain: 300000,
    totalVotes: 10000000,
    quorumReached: true,
    quorumPercentage: 82,
    timeLeft: "3 days 8 hours",
    userVoted: false,
  },
  {
    id: 1,
    title: "Allocate 5M ÉTR to Developer Grants Program",
    status: "active",
    category: "Treasury",
    submittedBy: "0x9e0f...1a2b",
    submissionDate: "March 5, 2026",
    description:
      "Establish a developer grants program funded with 5M ÉTR from the treasury to incentivize ecosystem development. Grants would range from 10K to 500K ÉTR per project.",
    votesYes: 4500000,
    votesNo: 4000000,
    votesAbstain: 1500000,
    totalVotes: 10000000,
    quorumReached: false,
    quorumPercentage: 45,
    timeLeft: "2 days 6 hours",
    userVoted: false,
  },
]

interface ProposalsListProps {
  filter: string
  sortBy: string
  searchQuery: string
  isWalletConnected: boolean
}

export default function ProposalsList({ filter, sortBy, searchQuery, isWalletConnected }: ProposalsListProps) {
  const [selectedProposal, setSelectedProposal] = useState<Proposal | null>(null)

  // Filter proposals
  let filteredProposals = mockProposals.filter((proposal) => {
    if (filter === "all") return true
    if (filter === "fiscal") return proposal.category === "Fiscal Policy"
    if (filter === "protocol") return proposal.category === "Protocol Upgrades"
    if (filter === "treasury") return proposal.category === "Treasury"
    if (filter === "voted") return proposal.userVoted
    if (filter === "not-voted") return !proposal.userVoted
    return true
  })

  // Search filter
  if (searchQuery) {
    filteredProposals = filteredProposals.filter((proposal) =>
      proposal.title.toLowerCase().includes(searchQuery.toLowerCase()),
    )
  }

  // Sort proposals
  filteredProposals = [...filteredProposals].sort((a, b) => {
    if (sortBy === "most-votes") return b.totalVotes - a.totalVotes
    if (sortBy === "ending-soon") return a.id - b.id // Simplified
    if (sortBy === "recently-added") return b.id - a.id
    return 0
  })

  if (!isWalletConnected) {
    return (
      <div className="text-center py-16">
        <p className="text-xl text-muted-foreground">Connect your wallet to participate in governance</p>
      </div>
    )
  }

  if (filteredProposals.length === 0) {
    return (
      <div className="text-center py-16">
        <p className="text-xl text-muted-foreground">No active proposals. Next Consensus Day: June 1, 2026</p>
      </div>
    )
  }

  return (
    <>
      <div className="grid grid-cols-1 lg:grid-cols-2 gap-6">
        {filteredProposals.map((proposal) => (
          <ProposalCard key={proposal.id} proposal={proposal} onClick={() => setSelectedProposal(proposal)} />
        ))}
      </div>

      {selectedProposal && <ProposalModal proposal={selectedProposal} onClose={() => setSelectedProposal(null)} />}
    </>
  )
}
