"use client"

import { useState } from "react"
import Link from "next/link"
import UserStatsCard from "@/components/governance/user-stats-card"
import FilterBar from "@/components/governance/filter-bar"
import ProposalsList from "@/components/governance/proposals-list"
import Sidebar from "@/components/governance/sidebar"
import CreateProposal from "@/components/governance/CreateProposal"
import { useWallet } from "@/lib/polkadot/useWallet"
import { useGovernanceStats, useConsensusDayPhase } from "@/lib/governance/hooks"
import { ExternalLink, Menu, X, Wallet, Vote, Clock, Users, TrendingUp, Plus, Loader2 } from "lucide-react"
import { Button } from "@/components/ui/button"
import { Card, CardContent } from "@/components/ui/card"

// Consënsus - The governance module for ETRID
export default function GovernanceContent() {
  const {
    isConnected,
    address,
    balance,
    connect,
    disconnect,
  } = useWallet()

  const [selectedFilter, setSelectedFilter] = useState("all")
  const [sortBy, setSortBy] = useState("most-votes")
  const [searchQuery, setSearchQuery] = useState("")
  const [error, setError] = useState<string | null>(null)
  const [isMenuOpen, setIsMenuOpen] = useState(false)
  const [showCreateProposal, setShowCreateProposal] = useState(false)

  // Fetch real governance stats
  const { stats, loading: statsLoading } = useGovernanceStats()
  const { phase, loading: phaseLoading } = useConsensusDayPhase()

  const formatAddress = (addr: string) => {
    if (!addr) return ""
    return `${addr.slice(0, 5)}...${addr.slice(-4)}`
  }

  const handleConnectWallet = async () => {
    try {
      await connect()
    } catch (e) {
      setError('Failed to connect wallet')
    }
  }

  return (
    <div className="min-h-screen gradient-bg-animated">
      {/* Header */}
      <header className="sticky top-0 z-50 glass border-b border-white/10">
        <div className="container mx-auto px-4 py-4">
          <div className="flex items-center justify-between">
            <Link href="/" className="flex items-center gap-3">
              <div className="w-10 h-10 rounded-xl bg-gradient-to-br from-blue-500 via-purple-500 to-cyan-500 flex items-center justify-center">
                <span className="text-white font-bold text-lg">E</span>
              </div>
              <span className="text-xl font-bold gradient-text hidden sm:block">ETRID</span>
            </Link>

            <nav className="hidden md:flex items-center gap-6">
              <Link href="/" className="text-white/80 hover:text-white transition-colors font-medium">
                Wallet
              </Link>
              <Link href="/swap" className="text-white/80 hover:text-white transition-colors font-medium">
                Swap
              </Link>
              <Link href="/staking/eth-pbc" className="text-white/80 hover:text-white transition-colors font-medium">
                Staking
              </Link>
              <Link href="/governance" className="text-white font-medium border-b-2 border-cyan-500 pb-1">
                Consënsus
              </Link>
              <Link href="/lightning" className="text-white/80 hover:text-white transition-colors font-medium">
                Lightning
              </Link>
              <a
                href="https://etrid.org"
                target="_blank"
                rel="noopener noreferrer"
                className="text-white/80 hover:text-white transition-colors font-medium flex items-center gap-1"
              >
                etrid.org <ExternalLink className="w-3 h-3" />
              </a>
            </nav>

            <div className="flex items-center gap-4">
              {isConnected ? (
                <button className="glass px-4 py-2 rounded-lg flex items-center gap-2 hover:bg-white/10 transition-colors">
                  <div className="w-2 h-2 rounded-full bg-green-500 animate-pulse" />
                  <span className="text-sm font-mono">{address?.slice(0, 6)}...{address?.slice(-4)}</span>
                </button>
              ) : (
                <Button
                  onClick={handleConnectWallet}
                  className="btn-primary px-6 py-2 rounded-lg font-medium"
                >
                  <Wallet className="w-4 h-4 mr-2" />
                  Connect
                </Button>
              )}

              <button
                onClick={() => setIsMenuOpen(!isMenuOpen)}
                className="md:hidden p-2 hover:bg-white/10 rounded-lg transition-colors"
              >
                {isMenuOpen ? <X className="w-6 h-6" /> : <Menu className="w-6 h-6" />}
              </button>
            </div>
          </div>

          {isMenuOpen && (
            <nav className="md:hidden mt-4 pb-4 border-t border-white/10 pt-4 flex flex-col gap-3">
              <Link href="/" className="text-white/80 hover:text-white transition-colors py-2">Wallet</Link>
              <Link href="/swap" className="text-white/80 hover:text-white transition-colors py-2">Swap</Link>
              <Link href="/staking/eth-pbc" className="text-white/80 hover:text-white transition-colors py-2">Staking</Link>
              <Link href="/governance" className="text-white py-2 font-medium">Consënsus</Link>
              <Link href="/lightning" className="text-white/80 hover:text-white transition-colors py-2">Lightning</Link>
            </nav>
          )}
        </div>
      </header>

      {/* Hero Banner */}
      <div className="relative overflow-hidden border-b border-white/10">
        <div className="absolute inset-0 bg-gradient-to-r from-cyan-500/10 via-purple-500/10 to-blue-500/10" />
        <div className="container mx-auto px-4 py-16 text-center relative">
          <div className="flex items-center justify-center gap-3 mb-4">
            <Vote className="w-10 h-10 text-cyan-400" />
            <h1 className="text-5xl md:text-6xl font-bold gradient-text">Consënsus</h1>
          </div>
          <p className="text-xl text-white/60 mb-6">Shape the future of ETRID Protocol</p>

          {/* Consensus Day Phase */}
          {phaseLoading ? (
            <div className="mb-8">
              <Loader2 className="w-8 h-8 animate-spin text-accent mx-auto" />
            </div>
          ) : phase.isActive ? (
            <div className="mb-8">
              <p className="text-white/60 mb-3">Current Phase: <span className="text-accent font-semibold">{phase.phase}</span></p>
              <div className="flex justify-center gap-4 text-3xl md:text-4xl font-bold">
                <div className="glass-card px-4 py-2 rounded-xl">
                  <span className="gradient-text">{Math.floor(phase.blocksRemaining / 14400)}</span>
                  <span className="text-xs text-white/60 block">days</span>
                </div>
                <span className="text-white/40 self-center">:</span>
                <div className="glass-card px-4 py-2 rounded-xl">
                  <span className="gradient-text">{Math.floor((phase.blocksRemaining % 14400) / 600)}</span>
                  <span className="text-xs text-white/60 block">hours</span>
                </div>
                <span className="text-white/40 self-center">:</span>
                <div className="glass-card px-4 py-2 rounded-xl">
                  <span className="gradient-text">{Math.floor((phase.blocksRemaining % 600) / 10)}</span>
                  <span className="text-xs text-white/60 block">min</span>
                </div>
              </div>
            </div>
          ) : (
            <div className="mb-8">
              <p className="text-white/60">Next Consensus Day coming soon</p>
            </div>
          )}

          {/* Stats Cards */}
          <div className="flex flex-wrap justify-center gap-4">
            <Card className="glass-card border-0">
              <CardContent className="p-4 flex items-center gap-3">
                <Users className="w-5 h-5 text-blue-400" />
                <div className="text-left">
                  <p className="text-sm text-white/60">Unique Voters</p>
                  <p className="font-bold">
                    {statsLoading ? (
                      <Loader2 className="w-4 h-4 animate-spin inline" />
                    ) : (
                      stats?.uniqueVoters.toLocaleString() || "0"
                    )}
                  </p>
                </div>
              </CardContent>
            </Card>
            <Card className="glass-card border-0">
              <CardContent className="p-4 flex items-center gap-3">
                <TrendingUp className="w-5 h-5 text-green-400" />
                <div className="text-left">
                  <p className="text-sm text-white/60">Total Proposals</p>
                  <p className="font-bold">
                    {statsLoading ? (
                      <Loader2 className="w-4 h-4 animate-spin inline" />
                    ) : (
                      stats?.totalProposals || "0"
                    )}
                  </p>
                </div>
              </CardContent>
            </Card>
            <Card className="glass-card border-0">
              <CardContent className="p-4 flex items-center gap-3">
                <Clock className="w-5 h-5 text-purple-400" />
                <div className="text-left">
                  <p className="text-sm text-white/60">Active Proposals</p>
                  <p className="font-bold">
                    {statsLoading ? (
                      <Loader2 className="w-4 h-4 animate-spin inline" />
                    ) : (
                      stats?.activeProposals || "0"
                    )}
                  </p>
                </div>
              </CardContent>
            </Card>
          </div>

          {/* Create Proposal Button */}
          {isConnected && phase.phase === "Registration" && (
            <div className="mt-8">
              <Button onClick={() => setShowCreateProposal(true)} size="lg" className="btn-primary">
                <Plus className="w-5 h-5 mr-2" />
                Submit New Proposal
              </Button>
            </div>
          )}
        </div>
      </div>

      {/* Main Content */}
      <div className="container mx-auto px-4 py-8">
        {error && (
          <div className="mb-6 p-4 glass-card border-red-500/20 rounded-lg text-red-400">
            {error}
          </div>
        )}

        <div className="flex flex-col lg:flex-row gap-8">
          <div className="flex-1">
            {isConnected && address && (
              <UserStatsCard address={address} />
            )}

            <div className="glass-card rounded-2xl p-6 mb-6">
              <FilterBar
                selectedFilter={selectedFilter}
                onFilterChange={setSelectedFilter}
                sortBy={sortBy}
                onSortChange={setSortBy}
                searchQuery={searchQuery}
                onSearchChange={setSearchQuery}
              />
            </div>

            <div className="glass-card rounded-2xl p-6">
              <ProposalsList
                filter={selectedFilter}
                sortBy={sortBy}
                searchQuery={searchQuery}
                isWalletConnected={isConnected}
                userAddress={address || undefined}
              />
            </div>
          </div>

          <div className="hidden lg:block w-80">
            <Sidebar userAddress={address || undefined} />
          </div>
        </div>
      </div>

      {/* Create Proposal Modal */}
      {showCreateProposal && address && (
        <CreateProposal
          onClose={() => setShowCreateProposal(false)}
          userAddress={address}
          onSuccess={() => {
            setShowCreateProposal(false)
            // Optionally show success message
          }}
        />
      )}

      {/* Footer */}
      <footer className="border-t border-white/10 mt-16">
        <div className="container mx-auto px-4 py-8">
          <div className="flex flex-col md:flex-row items-center justify-between gap-4">
            <div className="flex items-center gap-2">
              <div className="w-8 h-8 rounded-lg bg-gradient-to-br from-blue-500 to-purple-500 flex items-center justify-center">
                <span className="text-white font-bold text-sm">E</span>
              </div>
              <span className="font-semibold">ETRID Protocol</span>
            </div>
            <div className="flex items-center gap-6 text-white/60 text-sm">
              <a href="https://etrid.org" target="_blank" rel="noopener noreferrer" className="hover:text-white transition-colors">Website</a>
              <a href="https://docs.etrid.org" target="_blank" rel="noopener noreferrer" className="hover:text-white transition-colors">Docs</a>
              <a href="https://github.com/etrid" target="_blank" rel="noopener noreferrer" className="hover:text-white transition-colors">GitHub</a>
            </div>
          </div>
        </div>
      </footer>
    </div>
  )
}
