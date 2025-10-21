"use client"

import { useState } from "react"
import { Button } from "@/components/ui/button"
import { Badge } from "@/components/ui/badge"
import { X, ExternalLink, MessageSquare } from "lucide-react"
import { Card } from "@/components/ui/card"
import { useWallet } from "@/lib/polkadot/useWallet"
import { submitVote, getVotingPower } from "@/lib/polkadot/governance"

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

interface ProposalModalProps {
  proposal: Proposal
  onClose: () => void
}

export default function ProposalModal({ proposal, onClose }: ProposalModalProps) {
  const { selectedAccount, selectedChain } = useWallet()
  const [selectedVote, setSelectedVote] = useState<"yes" | "no" | "abstain" | null>(null)
  const [isVoting, setIsVoting] = useState(false)
  const [voteError, setVoteError] = useState<string | null>(null)
  const [txHash, setTxHash] = useState<string | null>(null)
  const [votingPower, setVotingPower] = useState<number>(0)

  // Calculate voting power based on user's balance
  useState(() => {
    if (selectedAccount?.balanceRaw) {
      getVotingPower(selectedChain, selectedAccount.address, selectedAccount.balanceRaw)
        .then(setVotingPower)
        .catch(console.error);
    }
  })

  const handleVote = async () => {
    if (!selectedVote || !selectedAccount) return

    setIsVoting(true)
    setVoteError(null)
    setTxHash(null)

    try {
      // Import web3FromAddress dynamically to avoid SSR issues
      const { web3FromAddress } = await import('@polkadot/extension-dapp')
      const injector = await web3FromAddress(selectedAccount.address)

      // Map UI vote options to on-chain format
      const voteType = selectedVote === 'yes' ? 'aye' : selectedVote === 'no' ? 'nay' : 'abstain'

      // Submit vote to blockchain
      const hash = await submitVote(
        selectedChain,
        selectedAccount.address,
        proposal.id,
        voteType,
        injector.signer
      )

      setTxHash(hash)

      // Close modal after successful vote
      setTimeout(() => {
        onClose()
      }, 2000)
    } catch (err) {
      const message = err instanceof Error ? err.message : 'Failed to submit vote'
      setVoteError(message)
      console.error('[ProposalModal] Vote error:', err)
    } finally {
      setIsVoting(false)
    }
  }

  return (
    <div className="fixed inset-0 z-50 bg-background/80 backdrop-blur-sm flex items-center justify-center p-4">
      <Card className="w-full max-w-4xl max-h-[90vh] overflow-y-auto animate-in slide-in-from-right">
        <div className="p-6 space-y-6">
          {/* Header */}
          <div className="flex items-start justify-between">
            <div className="space-y-2">
              <div className="flex items-center gap-2">
                <span className="text-sm font-mono text-muted-foreground">#{proposal.id}</span>
                <Badge>{proposal.status.charAt(0).toUpperCase() + proposal.status.slice(1)}</Badge>
                <Badge variant="outline">{proposal.category}</Badge>
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
              <p className="font-mono">{proposal.submittedBy}</p>
            </div>
            <span className="text-muted-foreground">•</span>
            <p className="text-sm text-muted-foreground">{proposal.submissionDate}</p>
          </div>

          {/* Full Description */}
          <div className="prose prose-invert max-w-none">
            <p>{proposal.description}</p>
          </div>

          {/* Economic Impact */}
          <Card className="p-6 bg-card/50">
            <h3 className="text-lg font-bold mb-4">Economic Impact</h3>
            <div className="space-y-2 text-sm">
              <p>
                <span className="text-muted-foreground">If this passes:</span> 30M new ÉTR will be minted in 2026
              </p>
              <p>
                <span className="text-muted-foreground">Inflation rate:</span> 2% → 3%
              </p>
            </div>
          </Card>

          {/* Discussion */}
          <Card className="p-6 bg-card/50">
            <h3 className="text-lg font-bold mb-4">Discussion</h3>
            <div className="flex items-center justify-between">
              <div className="flex items-center gap-2">
                <MessageSquare className="w-5 h-5 text-muted-foreground" />
                <span className="text-sm text-muted-foreground">142 comments</span>
              </div>
              <Button variant="outline" size="sm" className="gap-2 bg-transparent">
                View Discussion
                <ExternalLink className="w-4 h-4" />
              </Button>
            </div>
          </Card>

          {/* Voting Interface */}
          {!proposal.userVoted && (
            <Card className="p-6 bg-accent/5 border-accent/20">
              <h3 className="text-lg font-bold mb-4">Cast Your Vote</h3>

              {voteError && (
                <div className="mb-4 p-3 bg-destructive/10 border border-destructive/20 rounded-lg text-destructive text-sm">
                  {voteError}
                </div>
              )}

              {txHash && (
                <div className="mb-4 p-3 bg-green-500/10 border border-green-500/20 rounded-lg text-green-500 text-sm">
                  Vote submitted! Transaction: {txHash.slice(0, 10)}...{txHash.slice(-8)}
                </div>
              )}

              <div className="grid grid-cols-3 gap-4 mb-4">
                <Button
                  variant={selectedVote === "yes" ? "default" : "outline"}
                  className={selectedVote === "yes" ? "bg-green-500 hover:bg-green-600" : ""}
                  onClick={() => setSelectedVote("yes")}
                  disabled={isVoting}
                >
                  Yes
                </Button>
                <Button
                  variant={selectedVote === "no" ? "default" : "outline"}
                  className={selectedVote === "no" ? "bg-red-500 hover:bg-red-600" : ""}
                  onClick={() => setSelectedVote("no")}
                  disabled={isVoting}
                >
                  No
                </Button>
                <Button
                  variant={selectedVote === "abstain" ? "default" : "outline"}
                  className={selectedVote === "abstain" ? "bg-gray-500 hover:bg-gray-600" : ""}
                  onClick={() => setSelectedVote("abstain")}
                  disabled={isVoting}
                >
                  Abstain
                </Button>
              </div>

              {selectedVote && (
                <div className="space-y-4">
                  <p className="text-sm text-muted-foreground">
                    You're voting with <span className="font-bold text-accent">{votingPower.toLocaleString()} voting power</span>
                  </p>
                  <Button onClick={handleVote} disabled={isVoting} className="w-full">
                    {isVoting ? "Casting Vote..." : "Cast Vote"}
                  </Button>
                  <p className="text-xs text-muted-foreground text-center">
                    {isVoting ? "Sign transaction in Polkadot.js extension..." : "Web3 wallet signature required"}
                  </p>
                </div>
              )}
            </Card>
          )}

          {proposal.userVoted && (
            <Card className="p-6 bg-green-500/10 border-green-500/20">
              <p className="text-center text-green-500 font-semibold">
                You voted: {proposal.userVote?.toUpperCase()} ✅
              </p>
            </Card>
          )}
        </div>
      </Card>
    </div>
  )
}
