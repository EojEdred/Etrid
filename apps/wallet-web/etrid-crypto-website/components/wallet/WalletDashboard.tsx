'use client'

import { useState, useEffect } from 'react'
import Link from 'next/link'
import {
  Wallet,
  ArrowUpRight,
  ArrowDownLeft,
  RefreshCw,
  Copy,
  ExternalLink,
  TrendingUp,
  TrendingDown,
  Zap,
  Vote,
  Layers,
  ChevronDown,
  Menu,
  X,
  Plus,
  Upload,
  LogOut,
  Clock
} from 'lucide-react'
import { Button } from '@/components/ui/button'
import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card'
import CreateWallet from './CreateWallet'
import ImportWallet from './ImportWallet'
import SendReceive from './SendReceive'

interface Token {
  symbol: string
  name: string
  balance: string
  usdValue: string
  change24h: number
  icon: string
}

interface Transaction {
  hash: string
  type: 'send' | 'receive'
  amount: string
  address: string
  timestamp: string
  status: 'confirmed' | 'pending' | 'failed'
}

export default function WalletDashboard() {
  const [isConnected, setIsConnected] = useState(false)
  const [address, setAddress] = useState('')
  const [privateKey, setPrivateKey] = useState('')
  const [mnemonic, setMnemonic] = useState('')
  const [isMenuOpen, setIsMenuOpen] = useState(false)
  const [etrBalance, setEtrBalance] = useState('0')
  const [totalBalance, setTotalBalance] = useState('$0.00')

  // Modal states
  const [showCreateWallet, setShowCreateWallet] = useState(false)
  const [showImportWallet, setShowImportWallet] = useState(false)
  const [showSendReceive, setShowSendReceive] = useState(false)
  const [sendReceiveMode, setSendReceiveMode] = useState<'send' | 'receive'>('send')

  // Transaction history
  const [transactions, setTransactions] = useState<Transaction[]>([])
  const [isLoadingBalance, setIsLoadingBalance] = useState(false)

  // Load wallet from localStorage on mount
  useEffect(() => {
    const savedWallet = localStorage.getItem('etrid_wallet')
    if (savedWallet) {
      const wallet = JSON.parse(savedWallet)
      setAddress(wallet.address)
      setPrivateKey(wallet.privateKey)
      setMnemonic(wallet.mnemonic || '')
      setIsConnected(true)
      loadBalance(wallet.address)
      loadTransactions(wallet.address)
    }
  }, [])

  const loadBalance = async (addr: string) => {
    setIsLoadingBalance(true)
    try {
      // This will be replaced with actual chain balance fetching from lib/wallet/service.ts
      // For now, using placeholder
      await new Promise(resolve => setTimeout(resolve, 1000))
      const balance = '1245.50'
      setEtrBalance(balance)
      setTotalBalance(`$${(parseFloat(balance) * 2).toFixed(2)}`)
    } catch (error) {
      console.error('Failed to load balance:', error)
    } finally {
      setIsLoadingBalance(false)
    }
  }

  const loadTransactions = async (addr: string) => {
    try {
      // This will be replaced with actual transaction history fetching
      // For now, using placeholder data
      const mockTransactions: Transaction[] = [
        {
          hash: '0xabc123...',
          type: 'receive',
          amount: '100.50',
          address: '0x1234...5678',
          timestamp: new Date(Date.now() - 3600000).toISOString(),
          status: 'confirmed'
        },
        {
          hash: '0xdef456...',
          type: 'send',
          amount: '50.25',
          address: '0x8765...4321',
          timestamp: new Date(Date.now() - 7200000).toISOString(),
          status: 'confirmed'
        }
      ]
      setTransactions(mockTransactions)
    } catch (error) {
      console.error('Failed to load transactions:', error)
    }
  }

  const handleWalletCreated = (addr: string, pk: string, mnem: string) => {
    setAddress(addr)
    setPrivateKey(pk)
    setMnemonic(mnem)
    setIsConnected(true)

    // Save to localStorage
    localStorage.setItem('etrid_wallet', JSON.stringify({
      address: addr,
      privateKey: pk,
      mnemonic: mnem
    }))

    loadBalance(addr)
    loadTransactions(addr)
  }

  const handleWalletImported = (addr: string, pk: string, mnem?: string) => {
    setAddress(addr)
    setPrivateKey(pk)
    setMnemonic(mnem || '')
    setIsConnected(true)

    // Save to localStorage
    localStorage.setItem('etrid_wallet', JSON.stringify({
      address: addr,
      privateKey: pk,
      mnemonic: mnem || ''
    }))

    loadBalance(addr)
    loadTransactions(addr)
  }

  const disconnectWallet = () => {
    setAddress('')
    setPrivateKey('')
    setMnemonic('')
    setEtrBalance('0')
    setTotalBalance('$0.00')
    setTransactions([])
    setIsConnected(false)
    localStorage.removeItem('etrid_wallet')
  }

  const formatAddress = (addr: string) => {
    if (!addr) return ''
    return `${addr.slice(0, 6)}...${addr.slice(-4)}`
  }

  const copyAddress = () => {
    navigator.clipboard.writeText(address)
  }

  const handleSendClick = () => {
    setSendReceiveMode('send')
    setShowSendReceive(true)
  }

  const handleReceiveClick = () => {
    setSendReceiveMode('receive')
    setShowSendReceive(true)
  }

  const handleSendTransaction = async (to: string, amount: string) => {
    // This will be replaced with actual transaction sending from lib/wallet/service.ts
    await new Promise(resolve => setTimeout(resolve, 2000))

    // Refresh balance and transactions after sending
    await loadBalance(address)
    await loadTransactions(address)
  }

  const refreshData = () => {
    if (address) {
      loadBalance(address)
      loadTransactions(address)
    }
  }

  const formatTimestamp = (timestamp: string) => {
    const date = new Date(timestamp)
    const now = new Date()
    const diff = now.getTime() - date.getTime()
    const hours = Math.floor(diff / 3600000)

    if (hours < 1) return 'Just now'
    if (hours < 24) return `${hours}h ago`
    return date.toLocaleDateString()
  }

  return (
    <div className="min-h-screen gradient-bg-animated">
      {/* Header */}
      <header className="sticky top-0 z-50 glass border-b border-white/10">
        <div className="container mx-auto px-4 py-4">
          <div className="flex items-center justify-between">
            {/* Logo - iOS Match */}
            <Link href="/" className="flex items-center gap-3">
              <div className="w-10 h-10 rounded-xl bg-gradient-to-br from-[#66D9E6] to-[#4DB3CC] flex items-center justify-center">
                <span className="text-[#0a0014] font-bold text-lg">E</span>
              </div>
              <span className="text-xl font-bold gradient-text hidden sm:block">ETRID</span>
            </Link>

            {/* Desktop Navigation */}
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
              {isConnected ? (
                <div className="flex items-center gap-2">
                  <button
                    onClick={copyAddress}
                    className="glass px-4 py-2 rounded-lg flex items-center gap-2 hover:bg-white/10 transition-colors"
                  >
                    <div className="w-2 h-2 rounded-full bg-green-500 animate-pulse" />
                    <span className="text-sm font-mono">{formatAddress(address)}</span>
                    <Copy className="w-4 h-4 text-white/60" />
                  </button>
                  <Button
                    onClick={disconnectWallet}
                    variant="outline"
                    size="sm"
                    className="border-red-500/50 text-red-400 hover:bg-red-500/10"
                  >
                    <LogOut className="w-4 h-4 mr-2" />
                    <span className="hidden sm:inline">Disconnect</span>
                  </Button>
                </div>
              ) : (
                <div className="flex items-center gap-2">
                  <Button
                    onClick={() => setShowCreateWallet(true)}
                    className="btn-primary px-4 py-2 rounded-lg font-medium"
                  >
                    <Plus className="w-4 h-4 mr-2" />
                    <span className="hidden sm:inline">Create</span>
                  </Button>
                  <Button
                    onClick={() => setShowImportWallet(true)}
                    variant="outline"
                    className="glass border-white/20 px-4 py-2 rounded-lg font-medium hover:bg-white/10"
                  >
                    <Upload className="w-4 h-4 mr-2" />
                    <span className="hidden sm:inline">Import</span>
                  </Button>
                </div>
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
              <Link href="/swap" className="text-white/80 hover:text-white transition-colors py-2">
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

      {/* Main Content */}
      <main className="container mx-auto px-4 py-8">
        {!isConnected ? (
          /* Connect Wallet Prompt */
          <div className="flex flex-col items-center justify-center min-h-[60vh] text-center">
            <div className="glass-card p-12 rounded-2xl max-w-lg">
              <div className="w-20 h-20 mx-auto mb-6 rounded-2xl bg-gradient-to-br from-[#66D9E6] to-[#4DB3CC] flex items-center justify-center pulse-glow">
                <Wallet className="w-10 h-10 text-[#0a0014]" />
              </div>
              <h1 className="text-3xl font-bold mb-4 gradient-text">Welcome to ETRID Wallet</h1>
              <p className="text-white/60 mb-8">
                Create a new wallet or import an existing one to access DeFi features, swap tokens, stake assets, and participate in Consënsus governance.
              </p>
              <div className="space-y-3">
                <Button
                  onClick={() => setShowCreateWallet(true)}
                  className="btn-primary px-8 py-3 rounded-xl text-lg font-medium w-full"
                >
                  <Plus className="w-5 h-5 mr-2" />
                  Create New Wallet
                </Button>
                <Button
                  onClick={() => setShowImportWallet(true)}
                  variant="outline"
                  className="glass border-white/20 px-8 py-3 rounded-xl text-lg font-medium w-full hover:bg-white/10"
                >
                  <Upload className="w-5 h-5 mr-2" />
                  Import Wallet
                </Button>
              </div>
              <div className="mt-6 text-white/40 text-sm">
                <p>Your keys, your crypto. Non-custodial and secure.</p>
              </div>
            </div>
          </div>
        ) : (
          /* Wallet Dashboard */
          <div className="space-y-8">
            {/* Portfolio Overview */}
            <Card className="glass-card border-0">
              <CardContent className="p-6">
                <div className="flex flex-col md:flex-row md:items-center md:justify-between gap-6">
                  <div>
                    <p className="text-white/60 text-sm mb-1">Total Balance</p>
                    <div className="flex items-center gap-3">
                      <h2 className="text-4xl font-bold gradient-text">
                        {isLoadingBalance ? (
                          <div className="w-32 h-10 glass rounded animate-pulse" />
                        ) : (
                          etrBalance + ' ETR'
                        )}
                      </h2>
                      <Button
                        variant="ghost"
                        size="icon-sm"
                        onClick={refreshData}
                        className="hover:bg-white/10"
                        disabled={isLoadingBalance}
                      >
                        <RefreshCw className={`w-4 h-4 ${isLoadingBalance ? 'animate-spin' : ''}`} />
                      </Button>
                    </div>
                    <div className="flex items-center gap-2 mt-2">
                      <span className="text-white/60 text-sm">{totalBalance} USD</span>
                    </div>
                  </div>
                  <div className="flex flex-wrap gap-3">
                    <Button
                      onClick={handleSendClick}
                      className="btn-primary rounded-xl px-6"
                    >
                      <ArrowUpRight className="w-4 h-4 mr-2" />
                      Send
                    </Button>
                    <Button
                      onClick={handleReceiveClick}
                      variant="outline"
                      className="glass border-white/20 rounded-xl px-6 hover:bg-white/10"
                    >
                      <ArrowDownLeft className="w-4 h-4 mr-2" />
                      Receive
                    </Button>
                    <Link href="/swap">
                      <Button variant="outline" className="glass border-white/20 rounded-xl px-6 hover:bg-white/10">
                        <RefreshCw className="w-4 h-4 mr-2" />
                        Swap
                      </Button>
                    </Link>
                  </div>
                </div>
              </CardContent>
            </Card>

            {/* Quick Actions */}
            <div className="grid grid-cols-2 md:grid-cols-4 gap-4">
              <Link href="/swap">
                <Card className="glass-card border-0 card-hover cursor-pointer">
                  <CardContent className="p-6 flex flex-col items-center text-center">
                    <div className="w-12 h-12 rounded-xl bg-blue-500/20 flex items-center justify-center mb-3">
                      <RefreshCw className="w-6 h-6 text-blue-400" />
                    </div>
                    <h3 className="font-semibold">Swap</h3>
                    <p className="text-white/60 text-sm">Exchange tokens</p>
                  </CardContent>
                </Card>
              </Link>

              <Link href="/staking/eth-pbc">
                <Card className="glass-card border-0 card-hover cursor-pointer">
                  <CardContent className="p-6 flex flex-col items-center text-center">
                    <div className="w-12 h-12 rounded-xl bg-purple-500/20 flex items-center justify-center mb-3">
                      <Layers className="w-6 h-6 text-purple-400" />
                    </div>
                    <h3 className="font-semibold">Stake</h3>
                    <p className="text-white/60 text-sm">Earn rewards</p>
                  </CardContent>
                </Card>
              </Link>

              <Link href="/governance">
                <Card className="glass-card border-0 card-hover cursor-pointer">
                  <CardContent className="p-6 flex flex-col items-center text-center">
                    <div className="w-12 h-12 rounded-xl bg-cyan-500/20 flex items-center justify-center mb-3">
                      <Vote className="w-6 h-6 text-cyan-400" />
                    </div>
                    <h3 className="font-semibold">Consënsus</h3>
                    <p className="text-white/60 text-sm">Vote on proposals</p>
                  </CardContent>
                </Card>
              </Link>

              <Link href="/lightning">
                <Card className="glass-card border-0 card-hover cursor-pointer">
                  <CardContent className="p-6 flex flex-col items-center text-center">
                    <div className="w-12 h-12 rounded-xl bg-yellow-500/20 flex items-center justify-center mb-3">
                      <Zap className="w-6 h-6 text-yellow-400" />
                    </div>
                    <h3 className="font-semibold">Lightning</h3>
                    <p className="text-white/60 text-sm">Fast transfers</p>
                  </CardContent>
                </Card>
              </Link>
            </div>

            {/* Assets */}
            <Card className="glass-card border-0">
              <CardHeader>
                <CardTitle className="flex items-center justify-between">
                  <span>Assets</span>
                  <Button
                    variant="ghost"
                    size="sm"
                    onClick={refreshData}
                    disabled={isLoadingBalance}
                    className="text-white/60 hover:text-white"
                  >
                    <RefreshCw className={`w-4 h-4 mr-2 ${isLoadingBalance ? 'animate-spin' : ''}`} />
                    Refresh
                  </Button>
                </CardTitle>
              </CardHeader>
              <CardContent className="p-0">
                <div className="divide-y divide-white/10">
                  <div className="flex items-center justify-between p-4 hover:bg-white/5 transition-colors">
                    <div className="flex items-center gap-4">
                      <div className="w-10 h-10 rounded-full bg-gradient-to-br from-[#66D9E6] to-[#4DB3CC] flex items-center justify-center">
                        <span className="text-[#0a0014] font-bold text-sm">E</span>
                      </div>
                      <div>
                        <h4 className="font-semibold">ETR</h4>
                        <p className="text-white/60 text-sm">Etrid</p>
                      </div>
                    </div>
                    <div className="text-right">
                      {isLoadingBalance ? (
                        <div className="w-24 h-8 glass rounded animate-pulse" />
                      ) : (
                        <>
                          <p className="font-semibold">{etrBalance}</p>
                          <span className="text-white/60 text-sm">{totalBalance}</span>
                        </>
                      )}
                    </div>
                  </div>
                </div>
              </CardContent>
            </Card>

            {/* Transaction History */}
            <Card className="glass-card border-0">
              <CardHeader>
                <CardTitle className="flex items-center justify-between">
                  <span>Recent Transactions</span>
                  <Button
                    variant="ghost"
                    size="sm"
                    onClick={refreshData}
                    className="text-white/60 hover:text-white"
                  >
                    <RefreshCw className="w-4 h-4" />
                  </Button>
                </CardTitle>
              </CardHeader>
              <CardContent className="p-0">
                {transactions.length === 0 ? (
                  <div className="p-12 text-center">
                    <Clock className="w-12 h-12 mx-auto mb-4 text-white/20" />
                    <h3 className="font-semibold text-white/60 mb-2">No transactions yet</h3>
                    <p className="text-sm text-white/40">Your transaction history will appear here</p>
                  </div>
                ) : (
                  <div className="divide-y divide-white/10">
                    {transactions.map((tx) => (
                      <div
                        key={tx.hash}
                        className="flex items-center justify-between p-4 hover:bg-white/5 transition-colors cursor-pointer"
                      >
                        <div className="flex items-center gap-4">
                          <div
                            className={`w-10 h-10 rounded-full flex items-center justify-center ${
                              tx.type === 'receive'
                                ? 'bg-green-500/20'
                                : 'bg-blue-500/20'
                            }`}
                          >
                            {tx.type === 'receive' ? (
                              <ArrowDownLeft className="w-5 h-5 text-green-400" />
                            ) : (
                              <ArrowUpRight className="w-5 h-5 text-blue-400" />
                            )}
                          </div>
                          <div>
                            <h4 className="font-semibold capitalize">{tx.type}</h4>
                            <p className="text-white/60 text-sm font-mono">
                              {formatAddress(tx.address)}
                            </p>
                          </div>
                        </div>
                        <div className="text-right">
                          <p className={`font-semibold ${
                            tx.type === 'receive' ? 'text-green-400' : 'text-white'
                          }`}>
                            {tx.type === 'receive' ? '+' : '-'}{tx.amount} ETR
                          </p>
                          <div className="flex items-center gap-2 justify-end">
                            <span className="text-white/60 text-sm">
                              {formatTimestamp(tx.timestamp)}
                            </span>
                            <span className={`text-xs px-2 py-0.5 rounded ${
                              tx.status === 'confirmed'
                                ? 'bg-green-500/20 text-green-400'
                                : tx.status === 'pending'
                                ? 'bg-yellow-500/20 text-yellow-400'
                                : 'bg-red-500/20 text-red-400'
                            }`}>
                              {tx.status}
                            </span>
                          </div>
                        </div>
                      </div>
                    ))}
                  </div>
                )}
              </CardContent>
            </Card>

            {/* Network Stats */}
            <div className="grid grid-cols-1 md:grid-cols-3 gap-4">
              <Card className="glass-card border-0">
                <CardContent className="p-6">
                  <div className="flex items-center gap-3 mb-2">
                    <div className="w-2 h-2 rounded-full bg-green-500 animate-pulse" />
                    <span className="text-white/60 text-sm">Network Status</span>
                  </div>
                  <p className="text-2xl font-bold text-green-500">Excellent</p>
                  <p className="text-white/40 text-sm mt-1">All systems operational</p>
                </CardContent>
              </Card>

              <Card className="glass-card border-0">
                <CardContent className="p-6">
                  <div className="flex items-center gap-3 mb-2">
                    <Zap className="w-4 h-4 text-yellow-500" />
                    <span className="text-white/60 text-sm">TPS</span>
                  </div>
                  <p className="text-2xl font-bold">171,000+</p>
                  <p className="text-white/40 text-sm mt-1">Transactions per second</p>
                </CardContent>
              </Card>

              <Card className="glass-card border-0">
                <CardContent className="p-6">
                  <div className="flex items-center gap-3 mb-2">
                    <Layers className="w-4 h-4 text-purple-500" />
                    <span className="text-white/60 text-sm">Block Height</span>
                  </div>
                  <p className="text-2xl font-bold">12,847,293</p>
                  <p className="text-white/40 text-sm mt-1">15s finality</p>
                </CardContent>
              </Card>
            </div>
          </div>
        )}
      </main>

      {/* Footer */}
      <footer className="border-t border-white/10 mt-16">
        <div className="container mx-auto px-4 py-8">
          <div className="flex flex-col md:flex-row items-center justify-between gap-4">
            <div className="flex items-center gap-2">
              <div className="w-8 h-8 rounded-lg bg-gradient-to-br from-[#66D9E6] to-[#4DB3CC] flex items-center justify-center">
                <span className="text-[#0a0014] font-bold text-sm">E</span>
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
              <a href="https://twitter.com/etridprotocol" target="_blank" rel="noopener noreferrer" className="hover:text-white transition-colors">
                Twitter
              </a>
            </div>
          </div>
        </div>
      </footer>

      {/* Modals */}
      <CreateWallet
        open={showCreateWallet}
        onOpenChange={setShowCreateWallet}
        onWalletCreated={handleWalletCreated}
      />
      <ImportWallet
        open={showImportWallet}
        onOpenChange={setShowImportWallet}
        onWalletImported={handleWalletImported}
      />
      <SendReceive
        open={showSendReceive}
        onOpenChange={setShowSendReceive}
        mode={sendReceiveMode}
        address={address}
        balance={etrBalance}
        onSendTransaction={handleSendTransaction}
      />
    </div>
  )
}
