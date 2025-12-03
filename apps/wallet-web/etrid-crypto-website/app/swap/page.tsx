"use client"

import Link from "next/link"
import dynamic from "next/dynamic"
import { ArrowLeft, ExternalLink, Menu, X, Wallet, Loader2 } from "lucide-react"
import { useState, Suspense } from "react"
import { Button } from "@/components/ui/button"
import { useWallet } from "@/lib/polkadot/useWallet"

// Dynamic imports to prevent SSR issues with wagmi/rainbowkit
const SwapCard = dynamic(() => import("@/components/swap/swap-card").then(mod => ({ default: mod.SwapCard })), {
  ssr: false,
  loading: () => <SwapLoadingCard />,
})
const ExchangeRate = dynamic(() => import("@/components/swap/exchange-rate").then(mod => ({ default: mod.ExchangeRate })), {
  ssr: false,
  loading: () => <div className="h-20 glass-card rounded-xl animate-pulse" />,
})
const RecentSwaps = dynamic(() => import("@/components/swap/recent-swaps").then(mod => ({ default: mod.RecentSwaps })), {
  ssr: false,
  loading: () => <div className="h-40 glass-card rounded-xl animate-pulse" />,
})
const PriceChart = dynamic(() => import("@/components/swap/price-chart").then(mod => ({ default: mod.PriceChart })), {
  ssr: false,
  loading: () => <div className="h-80 glass-card rounded-xl animate-pulse" />,
})
const InfoCards = dynamic(() => import("@/components/swap/info-cards").then(mod => ({ default: mod.InfoCards })), {
  ssr: false,
  loading: () => <div className="h-24 glass-card rounded-xl animate-pulse" />,
})

function SwapLoadingCard() {
  return (
    <div className="h-96 flex items-center justify-center glass-card rounded-xl">
      <div className="flex flex-col items-center gap-4">
        <Loader2 className="w-8 h-8 animate-spin text-primary" />
        <span className="text-white/60">Loading swap interface...</span>
      </div>
    </div>
  )
}

export default function SwapPage() {
  const wallet = useWallet()
  const [isMenuOpen, setIsMenuOpen] = useState(false)

  return (
    <div className="min-h-screen gradient-bg-animated">
      {/* Header */}
      <header className="sticky top-0 z-50 glass border-b border-white/10">
        <div className="container mx-auto px-4 py-4">
          <div className="flex items-center justify-between">
            {/* Logo */}
            <Link href="/" className="flex items-center gap-3">
              <div className="w-10 h-10 rounded-xl bg-gradient-to-br from-blue-500 via-purple-500 to-cyan-500 flex items-center justify-center">
                <span className="text-white font-bold text-lg">E</span>
              </div>
              <span className="text-xl font-bold gradient-text hidden sm:block">ETRID</span>
            </Link>

            {/* Desktop Navigation */}
            <nav className="hidden md:flex items-center gap-6">
              <Link href="/" className="text-white/80 hover:text-white transition-colors font-medium">
                Wallet
              </Link>
              <Link href="/swap" className="text-white font-medium border-b-2 border-blue-500 pb-1">
                Swap
              </Link>
              <Link href="/staking/eth-pbc" className="text-white/80 hover:text-white transition-colors font-medium">
                Staking
              </Link>
              <Link href="/governance" className="text-white/80 hover:text-white transition-colors font-medium">
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

            {/* Wallet Connection */}
            <div className="flex items-center gap-4">
              {wallet.isConnected ? (
                <button className="glass px-4 py-2 rounded-lg flex items-center gap-2 hover:bg-white/10 transition-colors">
                  <div className="w-2 h-2 rounded-full bg-green-500 animate-pulse" />
                  <span className="text-sm font-mono">{wallet.address?.slice(0, 6)}...{wallet.address?.slice(-4)}</span>
                </button>
              ) : (
                <Button
                  onClick={wallet.connect}
                  className="btn-primary px-6 py-2 rounded-lg font-medium"
                >
                  <Wallet className="w-4 h-4 mr-2" />
                  Connect
                </Button>
              )}

              {/* Mobile Menu Button */}
              <button
                onClick={() => setIsMenuOpen(!isMenuOpen)}
                className="md:hidden p-2 hover:bg-white/10 rounded-lg transition-colors"
              >
                {isMenuOpen ? <X className="w-6 h-6" /> : <Menu className="w-6 h-6" />}
              </button>
            </div>
          </div>

          {/* Mobile Navigation */}
          {isMenuOpen && (
            <nav className="md:hidden mt-4 pb-4 border-t border-white/10 pt-4 flex flex-col gap-3">
              <Link href="/" className="text-white/80 hover:text-white transition-colors py-2">
                Wallet
              </Link>
              <Link href="/swap" className="text-white py-2 font-medium">
                Swap
              </Link>
              <Link href="/staking/eth-pbc" className="text-white/80 hover:text-white transition-colors py-2">
                Staking
              </Link>
              <Link href="/governance" className="text-white/80 hover:text-white transition-colors py-2">
                Consënsus
              </Link>
              <Link href="/lightning" className="text-white/80 hover:text-white transition-colors py-2">
                Lightning
              </Link>
            </nav>
          )}
        </div>
      </header>

      <main className="container mx-auto px-4 py-8">
        {/* Page Title */}
        <div className="mb-8">
          <h1 className="text-3xl font-bold gradient-text mb-2">Swap Tokens</h1>
          <p className="text-white/60">Exchange tokens instantly with the best rates</p>
        </div>

        {wallet.error && (
          <div className="mb-6 p-4 glass-card border-red-500/20 rounded-lg text-red-400">
            {wallet.error}
          </div>
        )}

        <div className="grid grid-cols-1 lg:grid-cols-3 gap-8">
          {/* Main swap area */}
          <div className="lg:col-span-2 space-y-6">
            <div className="glass-card rounded-2xl p-6">
              <SwapCard wallet={wallet} />
            </div>
            <div className="glass-card rounded-2xl p-6">
              <ExchangeRate />
            </div>
            <div className="glass-card rounded-2xl p-6">
              <RecentSwaps />
            </div>
          </div>

          {/* Sidebar */}
          <div className="space-y-6">
            <div className="glass-card rounded-2xl p-6">
              <PriceChart />
            </div>
          </div>
        </div>

        {/* Info cards at bottom */}
        <div className="mt-8">
          <InfoCards />
        </div>
      </main>

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
              <a href="https://etrid.org" target="_blank" rel="noopener noreferrer" className="hover:text-white transition-colors">
                Website
              </a>
              <a href="https://docs.etrid.org" target="_blank" rel="noopener noreferrer" className="hover:text-white transition-colors">
                Docs
              </a>
              <a href="https://github.com/etrid" target="_blank" rel="noopener noreferrer" className="hover:text-white transition-colors">
                GitHub
              </a>
            </div>
          </div>
        </div>
      </footer>
    </div>
  )
}
