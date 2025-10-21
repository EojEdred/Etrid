"use client"

import { useState } from "react"
import { X, Scale } from "lucide-react"
import { Button } from "@/components/ui/button"
import { ScrollArea } from "@/components/ui/scroll-area"

interface VoteModalProps {
  proposal: {
    id: number
    title: string
    description: string
    category: string
  }
  votingPower: number
  onClose: () => void
  onVote: (vote: "yes" | "no" | "abstain") => void
}

export function VoteModal({ proposal, votingPower, onClose, onVote }: VoteModalProps) {
  const [selectedVote, setSelectedVote] = useState<"yes" | "no" | "abstain" | null>(null)

  const handleSubmit = () => {
    if (selectedVote) {
      onVote(selectedVote)
    }
  }

  return (
    <div className="fixed inset-0 z-50 flex items-end sm:items-center justify-center">
      {/* Backdrop */}
      <div className="absolute inset-0 bg-black/80 backdrop-blur-sm" onClick={onClose} />

      {/* Modal */}
      <div className="relative w-full max-w-lg glass-strong rounded-t-3xl sm:rounded-3xl max-h-[90vh] flex flex-col">
        {/* Header */}
        <div className="flex items-center justify-between p-6 border-b border-border">
          <div className="flex items-center gap-3">
            <div className="w-10 h-10 rounded-full bg-accent/20 flex items-center justify-center">
              <Scale className="w-5 h-5 text-accent" />
            </div>
            <div>
              <h2 className="font-bold">Cast Your Vote</h2>
              <p className="text-xs text-muted-foreground">Proposal #{proposal.id}</p>
            </div>
          </div>
          <Button variant="ghost" size="icon" onClick={onClose}>
            <X className="w-5 h-5" />
          </Button>
        </div>

        {/* Content */}
        <ScrollArea className="flex-1 p-6">
          <div className="space-y-6">
            {/* Proposal Details */}
            <div>
              <span className="text-xs px-2 py-1 rounded-full bg-surface text-muted-foreground">
                {proposal.category}
              </span>
              <h3 className="font-bold text-lg mt-3 mb-2">{proposal.title}</h3>
              <p className="text-sm text-muted-foreground leading-relaxed">{proposal.description}</p>
            </div>

            {/* Economic Impact */}
            <div className="glass rounded-xl p-4">
              <h4 className="font-semibold text-sm mb-2">Economic Impact</h4>
              <p className="text-xs text-muted-foreground">
                If passed, this proposal will take effect at the next epoch and may affect staking rewards, transaction
                fees, or network parameters.
              </p>
            </div>

            {/* Vote Options */}
            <div className="space-y-3">
              <h4 className="font-semibold text-sm">Select Your Vote</h4>

              <button
                onClick={() => setSelectedVote("yes")}
                className={`w-full p-4 rounded-xl border-2 transition-all ${
                  selectedVote === "yes"
                    ? "border-success bg-success/10"
                    : "border-border glass hover:border-success/50"
                }`}
              >
                <div className="flex items-center justify-between">
                  <span className="font-semibold text-success">Yes</span>
                  <div
                    className={`w-5 h-5 rounded-full border-2 transition-all ${
                      selectedVote === "yes" ? "border-success bg-success" : "border-muted-foreground"
                    }`}
                  >
                    {selectedVote === "yes" && <div className="w-full h-full rounded-full bg-white scale-50" />}
                  </div>
                </div>
              </button>

              <button
                onClick={() => setSelectedVote("no")}
                className={`w-full p-4 rounded-xl border-2 transition-all ${
                  selectedVote === "no" ? "border-error bg-error/10" : "border-border glass hover:border-error/50"
                }`}
              >
                <div className="flex items-center justify-between">
                  <span className="font-semibold text-error">No</span>
                  <div
                    className={`w-5 h-5 rounded-full border-2 transition-all ${
                      selectedVote === "no" ? "border-error bg-error" : "border-muted-foreground"
                    }`}
                  >
                    {selectedVote === "no" && <div className="w-full h-full rounded-full bg-white scale-50" />}
                  </div>
                </div>
              </button>

              <button
                onClick={() => setSelectedVote("abstain")}
                className={`w-full p-4 rounded-xl border-2 transition-all ${
                  selectedVote === "abstain"
                    ? "border-muted-foreground bg-muted-foreground/10"
                    : "border-border glass hover:border-muted-foreground/50"
                }`}
              >
                <div className="flex items-center justify-between">
                  <span className="font-semibold text-muted-foreground">Abstain</span>
                  <div
                    className={`w-5 h-5 rounded-full border-2 transition-all ${
                      selectedVote === "abstain"
                        ? "border-muted-foreground bg-muted-foreground"
                        : "border-muted-foreground"
                    }`}
                  >
                    {selectedVote === "abstain" && <div className="w-full h-full rounded-full bg-white scale-50" />}
                  </div>
                </div>
              </button>
            </div>

            {/* Voting Power Confirmation */}
            <div className="glass rounded-xl p-4 text-center">
              <p className="text-sm text-muted-foreground mb-1">You&apos;re voting with</p>
              <p className="text-2xl font-bold text-accent">{votingPower.toLocaleString()}</p>
              <p className="text-xs text-muted-foreground">voting power</p>
            </div>
          </div>
        </ScrollArea>

        {/* Footer */}
        <div className="p-6 border-t border-border">
          <Button
            className="w-full h-12 text-base font-semibold"
            disabled={!selectedVote}
            onClick={handleSubmit}
            style={{
              background: selectedVote ? "#00d9ff" : undefined,
              color: selectedVote ? "#000" : undefined,
            }}
          >
            Cast Vote
          </Button>
          <p className="text-xs text-center text-muted-foreground mt-3">Biometric authentication required</p>
        </div>
      </div>
    </div>
  )
}
