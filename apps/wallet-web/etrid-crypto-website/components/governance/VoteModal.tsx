"use client"

import { useState } from "react"
import { Button } from "@/components/ui/button"
import { Badge } from "@/components/ui/badge"
import { Card } from "@/components/ui/card"
import { X, Info, Loader2 } from "lucide-react"
import { Proposal } from "@/lib/polkadot/governance"
import { useVotingPower } from "@/lib/governance/hooks"
import { governanceApi } from "@/lib/polkadot/governance"
import { Slider } from "@/components/ui/slider"
import { Tooltip, TooltipContent, TooltipProvider, TooltipTrigger } from "@/components/ui/tooltip"

interface VoteModalProps {
  proposal: Proposal
  onClose: () => void
  userAddress: string
  onVoteSuccess?: () => void
}

// Conviction levels for conviction voting
const CONVICTION_LEVELS = [
  { multiplier: 0.1, lockPeriods: 0, label: "0.1x (No lock)" },
  { multiplier: 1, lockPeriods: 1, label: "1x (1 period)" },
  { multiplier: 2, lockPeriods: 2, label: "2x (2 periods)" },
  { multiplier: 3, lockPeriods: 4, label: "3x (4 periods)" },
  { multiplier: 4, lockPeriods: 8, label: "4x (8 periods)" },
  { multiplier: 5, lockPeriods: 16, label: "5x (16 periods)" },
  { multiplier: 6, lockPeriods: 32, label: "6x (32 periods)" },
]

export default function VoteModal({ proposal, onClose, userAddress, onVoteSuccess }: VoteModalProps) {
  const [selectedVote, setSelectedVote] = useState<"Yes" | "No" | "Abstain" | null>(null)
  const [convictionLevel, setConvictionLevel] = useState(1) // Index in CONVICTION_LEVELS
  const [isVoting, setIsVoting] = useState(false)
  const [voteError, setVoteError] = useState<string | null>(null)
  const [txHash, setTxHash] = useState<string | null>(null)

  const { votingPower, loading: loadingPower } = useVotingPower(userAddress)

  const currentConviction = CONVICTION_LEVELS[convictionLevel]
  const effectiveVotingPower = votingPower.votingPower
    ? Math.floor(parseFloat(votingPower.votingPower) * currentConviction.multiplier)
    : 0

  const handleVote = async () => {
    if (!selectedVote) return

    setIsVoting(true)
    setVoteError(null)
    setTxHash(null)

    try {
      // Import web3FromAddress dynamically to avoid SSR issues
      const { web3FromAddress } = await import("@polkadot/extension-dapp")
      const injector = await web3FromAddress(userAddress)

      // Submit vote to blockchain with conviction
      // Note: For now we're using the basic vote function
      // In production, you'd want to use a conviction-aware vote extrinsic
      const hash = await governanceApi.submitVote(proposal.id, selectedVote, injector.signer)

      setTxHash(hash)

      // Call success callback
      if (onVoteSuccess) {
        onVoteSuccess()
      }

      // Close modal after successful vote
      setTimeout(() => {
        onClose()
      }, 3000)
    } catch (err) {
      const message = err instanceof Error ? err.message : "Failed to submit vote"
      setVoteError(message)
      console.error("[VoteModal] Vote error:", err)
    } finally {
      setIsVoting(false)
    }
  }

  return (
    <div className="fixed inset-0 z-50 bg-background/80 backdrop-blur-sm flex items-center justify-center p-4">
      <Card className="w-full max-w-2xl max-h-[90vh] overflow-y-auto animate-in slide-in-from-bottom glass-card">
        <div className="p-6 space-y-6">
          {/* Header */}
          <div className="flex items-start justify-between">
            <div className="space-y-2">
              <div className="flex items-center gap-2">
                <span className="text-sm font-mono text-muted-foreground">#{proposal.id}</span>
                <Badge>{proposal.status}</Badge>
                <Badge variant="outline">{proposal.category}</Badge>
              </div>
              <h2 className="text-2xl font-bold">{proposal.title}</h2>
            </div>
            <Button variant="ghost" size="icon" onClick={onClose} disabled={isVoting}>
              <X className="w-5 h-5" />
            </Button>
          </div>

          {/* Voting Power Display */}
          <Card className="p-4 bg-accent/10 border-accent/20">
            <div className="space-y-2">
              <div className="flex items-center justify-between">
                <span className="text-sm text-muted-foreground">Your Voting Power:</span>
                {loadingPower ? (
                  <Loader2 className="w-4 h-4 animate-spin" />
                ) : (
                  <span className="text-xl font-bold text-accent">{votingPower.votingPower} votes</span>
                )}
              </div>
              <div className="flex items-center justify-between">
                <span className="text-sm text-muted-foreground">Staked Amount:</span>
                <span className="text-sm font-semibold">
                  {(parseFloat(votingPower.stakedAmount) / 1e18).toLocaleString(undefined, {
                    maximumFractionDigits: 2,
                  })}{" "}
                  ETR
                </span>
              </div>
            </div>
          </Card>

          {/* Vote Selection */}
          <div className="space-y-4">
            <h3 className="text-lg font-bold">Cast Your Vote</h3>

            {voteError && (
              <div className="p-3 bg-destructive/10 border border-destructive/20 rounded-lg text-destructive text-sm">
                {voteError}
              </div>
            )}

            {txHash && (
              <div className="p-3 bg-green-500/10 border border-green-500/20 rounded-lg text-green-500 text-sm">
                Vote submitted successfully! Transaction: {txHash.slice(0, 10)}...{txHash.slice(-8)}
              </div>
            )}

            <div className="grid grid-cols-3 gap-4">
              <Button
                variant={selectedVote === "Yes" ? "default" : "outline"}
                className={selectedVote === "Yes" ? "bg-green-500 hover:bg-green-600" : ""}
                onClick={() => setSelectedVote("Yes")}
                disabled={isVoting}
              >
                Yes
              </Button>
              <Button
                variant={selectedVote === "No" ? "default" : "outline"}
                className={selectedVote === "No" ? "bg-red-500 hover:bg-red-600" : ""}
                onClick={() => setSelectedVote("No")}
                disabled={isVoting}
              >
                No
              </Button>
              <Button
                variant={selectedVote === "Abstain" ? "default" : "outline"}
                className={selectedVote === "Abstain" ? "bg-gray-500 hover:bg-gray-600" : ""}
                onClick={() => setSelectedVote("Abstain")}
                disabled={isVoting}
              >
                Abstain
              </Button>
            </div>
          </div>

          {/* Conviction Voting */}
          {selectedVote && selectedVote !== "Abstain" && (
            <div className="space-y-4">
              <div className="flex items-center gap-2">
                <h3 className="text-lg font-bold">Conviction Level</h3>
                <TooltipProvider>
                  <Tooltip>
                    <TooltipTrigger>
                      <Info className="w-4 h-4 text-muted-foreground" />
                    </TooltipTrigger>
                    <TooltipContent className="max-w-xs">
                      <p>
                        Lock your tokens for longer periods to multiply your voting power. Higher conviction means
                        your vote counts more, but your tokens will be locked for longer.
                      </p>
                    </TooltipContent>
                  </Tooltip>
                </TooltipProvider>
              </div>

              <div className="space-y-3">
                <div className="flex items-center justify-between">
                  <span className="text-sm text-muted-foreground">{currentConviction.label}</span>
                  <span className="text-sm font-semibold">
                    Effective Power: {effectiveVotingPower.toLocaleString()} votes
                  </span>
                </div>

                <Slider
                  value={[convictionLevel]}
                  onValueChange={(value) => setConvictionLevel(value[0])}
                  min={0}
                  max={CONVICTION_LEVELS.length - 1}
                  step={1}
                  disabled={isVoting}
                  className="w-full"
                />

                <div className="grid grid-cols-7 gap-1 text-xs text-muted-foreground">
                  {CONVICTION_LEVELS.map((level, idx) => (
                    <button
                      key={idx}
                      onClick={() => setConvictionLevel(idx)}
                      className={`text-center py-1 rounded transition-colors ${
                        convictionLevel === idx ? "bg-accent/20 text-accent font-semibold" : "hover:bg-accent/10"
                      }`}
                      disabled={isVoting}
                    >
                      {level.multiplier}x
                    </button>
                  ))}
                </div>
              </div>

              <Card className="p-4 bg-muted/50">
                <div className="space-y-2 text-sm">
                  <div className="flex justify-between">
                    <span className="text-muted-foreground">Lock Period:</span>
                    <span className="font-semibold">
                      {currentConviction.lockPeriods === 0
                        ? "No lock"
                        : `${currentConviction.lockPeriods} periods (~${currentConviction.lockPeriods * 28} days)`}
                    </span>
                  </div>
                  <div className="flex justify-between">
                    <span className="text-muted-foreground">Vote Multiplier:</span>
                    <span className="font-semibold">{currentConviction.multiplier}x</span>
                  </div>
                  <div className="flex justify-between">
                    <span className="text-muted-foreground">Your Effective Vote:</span>
                    <span className="font-bold text-accent">{effectiveVotingPower.toLocaleString()} votes</span>
                  </div>
                </div>
              </Card>
            </div>
          )}

          {/* Submit Button */}
          {selectedVote && (
            <div className="space-y-3">
              <Button onClick={handleVote} disabled={isVoting || loadingPower} className="w-full" size="lg">
                {isVoting ? (
                  <>
                    <Loader2 className="w-4 h-4 mr-2 animate-spin" />
                    Casting Vote...
                  </>
                ) : (
                  `Cast ${selectedVote} Vote`
                )}
              </Button>
              <p className="text-xs text-muted-foreground text-center">
                {isVoting ? "Sign transaction in your wallet..." : "You will be prompted to sign this transaction"}
              </p>
            </div>
          )}

          {/* Proposal Details */}
          <Card className="p-4 bg-muted/30">
            <h4 className="font-semibold mb-2">Proposal Description</h4>
            <p className="text-sm text-muted-foreground">{proposal.description}</p>
          </Card>
        </div>
      </Card>
    </div>
  )
}
