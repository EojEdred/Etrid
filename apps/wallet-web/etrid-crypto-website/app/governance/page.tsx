"use client"

import { useState } from "react"
import GovHeader from "@/components/governance/gov-header"
import HeroBanner from "@/components/governance/hero-banner"
import UserStatsCard from "@/components/governance/user-stats-card"
import FilterBar from "@/components/governance/filter-bar"
import ProposalsList from "@/components/governance/proposals-list"
import Sidebar from "@/components/governance/sidebar"
import { useWallet } from "@/lib/polkadot/useWallet"

export default function GovernancePage() {
  const {
    isConnected,
    selectedAccount,
    connect,
    disconnect,
    error,
    isLoading
  } = useWallet()

  const [selectedFilter, setSelectedFilter] = useState("all")
  const [sortBy, setSortBy] = useState("most-votes")
  const [searchQuery, setSearchQuery] = useState("")

  // Format wallet address for display (e.g., "5GrwE...Kw3t")
  const formatAddress = (address: string) => {
    if (!address) return ""
    return `${address.slice(0, 5)}...${address.slice(-4)}`
  }

  const handleConnectWallet = async () => {
    await connect()
  }

  const handleDisconnect = () => {
    disconnect()
  }

  return (
    <div className="min-h-screen bg-background">
      <GovHeader
        isConnected={isConnected}
        walletAddress={selectedAccount ? formatAddress(selectedAccount.address) : ""}
        onConnect={handleConnectWallet}
        onDisconnect={handleDisconnect}
      />

      <HeroBanner />

      <div className="container mx-auto px-4 py-8">
        {error && (
          <div className="mb-6 p-4 bg-destructive/10 border border-destructive/20 rounded-lg text-destructive">
            {error}
          </div>
        )}

        <div className="flex flex-col lg:flex-row gap-8">
          <div className="flex-1">
            {isConnected && selectedAccount && (
              <UserStatsCard
                address={selectedAccount.address}
                balance={selectedAccount.balance || "0"}
              />
            )}

            <FilterBar
              selectedFilter={selectedFilter}
              onFilterChange={setSelectedFilter}
              sortBy={sortBy}
              onSortChange={setSortBy}
              searchQuery={searchQuery}
              onSearchChange={setSearchQuery}
            />

            <ProposalsList
              filter={selectedFilter}
              sortBy={sortBy}
              searchQuery={searchQuery}
              isWalletConnected={isConnected}
            />
          </div>

          <div className="hidden lg:block w-80">
            <Sidebar />
          </div>
        </div>
      </div>
    </div>
  )
}
