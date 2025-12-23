'use client'

import { useState, useEffect, useMemo } from 'react'
import Link from 'next/link'
import Image from 'next/image'
import {
  Wallet,
  ArrowUpRight,
  ArrowDownLeft,
  RefreshCw,
  Copy,
  Zap,
  Plus,
  Upload,
  LogOut,
  BarChart3,
  Shield,
  Check,
  ChevronDown,
  ExternalLink,
  Clock,
  Layers,
  Smartphone,
  Download,
  Lock,
} from 'lucide-react'
import { Button } from '@/components/ui/button'
import { Card, CardContent } from '@/components/ui/card'
import { Tabs, TabsContent, TabsList, TabsTrigger } from '@/components/ui/tabs'
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuSeparator,
  DropdownMenuTrigger,
} from '@/components/ui/dropdown-menu'
import SendReceive from './SendReceive'
import WalletOnboarding from './WalletOnboarding'
import BuyETR from './BuyETR'
import { ChainSelector } from './ChainSelector'
import { usePolkadotApi } from '@/hooks/usePolkadotApi'
import { useBalance } from '@/hooks/useBalance'
import { ChainSelectorProvider, useChainSelector, useChainBalance } from '@/hooks/useChainSelector'
import { useTransactions } from '@/hooks/useTransactions'
import { useToast } from '@/hooks/use-toast'
import { useWallet } from '@/contexts/WalletContext'
import { explorerAccount, explorerExtrinsic } from '@/lib/explorer'

// Generate a deterministic color from address for identicon
function generateIdenticonColors(address: string): string[] {
  if (!address) return ['#666', '#888', '#aaa', '#ccc']
  const colors: string[] = []
  for (let i = 0; i < 4; i++) {
    const hash = address.slice(2 + i * 8, 10 + i * 8)
    const hue = parseInt(hash, 16) % 360
    colors.push(`hsl(${hue}, 70%, 60%)`)
  }
  return colors
}

// Identicon component like MetaMask
function Identicon({ address, size = 40 }: { address: string; size?: number }) {
  const colors = useMemo(() => generateIdenticonColors(address), [address])
  return (
    <div className="rounded-full overflow-hidden" style={{ width: size, height: size }}>
      <svg viewBox="0 0 40 40" width={size} height={size}>
        <rect x="0" y="0" width="20" height="20" fill={colors[0]} />
        <rect x="20" y="0" width="20" height="20" fill={colors[1]} />
        <rect x="0" y="20" width="20" height="20" fill={colors[2]} />
        <rect x="20" y="20" width="20" height="20" fill={colors[3]} />
        <circle cx="20" cy="20" r="8" fill={colors[0]} opacity="0.5" />
      </svg>
    </div>
  )
}

function WalletDashboardContent() {
  const { toast } = useToast()
  const { status, account, lock, deleteCurrentWallet, storedWallet } = useWallet()

  const [copied, setCopied] = useState(false)
  const [activeTab, setActiveTab] = useState('assets')

  // Modal states
  const [showWalletOnboarding, setShowWalletOnboarding] = useState(false)
  const [showSendReceive, setShowSendReceive] = useState(false)
  const [showBuyETR, setShowBuyETR] = useState(false)
  const [sendReceiveMode, setSendReceiveMode] = useState<'send' | 'receive'>('send')

  // Derived state from wallet context
  const isConnected = status === 'unlocked' && !!account
  const address = account?.address || ''

  // Hooks
  const { balance, isLoading: isLoadingBalance, refetch: refreshBalance } = useBalance(address)
  const { isConnected: isChainConnected, chainInfo, currentBlock, error: chainError } = usePolkadotApi()
  const { selectedChain, isConnected: isPBCConnected } = useChainSelector()
  const { balance: pbcBalance, isLoading: isPBCBalanceLoading } = useChainBalance(address)
  const { transactions, isLoading: isLoadingTx, refetch: refetchTx } = useTransactions(address)

  const displayBalance = pbcBalance?.formatted || balance?.formatted || '0'
  const displayToken = selectedChain.token
  const etrBalance = balance?.formatted || '0 ETR'

  // Show onboarding if wallet exists but is locked
  useEffect(() => {
    if (status === 'locked' && storedWallet) {
      setShowWalletOnboarding(true)
    }
  }, [status, storedWallet])

  const handleLockWallet = () => {
    lock()
    toast({ title: 'Wallet Locked', description: 'Your wallet has been locked' })
  }

  const handleDisconnectWallet = () => {
    if (window.confirm('Are you sure you want to remove this wallet? Make sure you have your recovery phrase saved.')) {
      deleteCurrentWallet()
      toast({ title: 'Wallet Removed', description: 'Your wallet has been removed from this device' })
    }
  }

  const formatAddress = (addr: string) => {
    if (!addr) return ''
    return `${addr.slice(0, 6)}...${addr.slice(-4)}`
  }

  const copyAddress = () => {
    navigator.clipboard.writeText(address)
    setCopied(true)
    toast({ title: 'Address Copied', description: 'Wallet address copied to clipboard' })
    setTimeout(() => setCopied(false), 2000)
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
    await new Promise(resolve => setTimeout(resolve, 2000))
    refreshBalance()
  }

  const refreshData = () => {
    if (address) {
      refreshBalance()
      refetchTx()
    }
  }

  return (
    <div className="min-h-screen gradient-bg-animated">
      {/* Header with ETRID Branding */}
      <header className="sticky top-0 z-50 glass border-b border-white/10">
        <div className="container mx-auto px-4 py-3">
          <div className="flex items-center justify-between">
            {/* Logo + Navigation */}
            <div className="flex items-center gap-6">
              <Link href="/" className="flex items-center gap-2">
                <Image src="/etrid-logo.png" alt="ETRID" width={36} height={36} className="rounded-xl" />
                <span className="font-bold text-lg hidden sm:inline gradient-text">ETRID</span>
              </Link>

              {/* Navigation Tabs */}
              <nav className="hidden md:flex items-center gap-1">
                <Link href="/staking" className="flex items-center gap-1.5 px-3 py-2 rounded-lg text-white/70 hover:text-white hover:bg-white/10 transition-all text-sm">
                  <BarChart3 className="w-4 h-4" />
                  <span>Staking</span>
                </Link>
                <Link href="/lightning" className="flex items-center gap-1.5 px-3 py-2 rounded-lg text-white/70 hover:text-white hover:bg-white/10 transition-all text-sm">
                  <Zap className="w-4 h-4" />
                  <span>Lightning</span>
                </Link>
                <Link href="/validator" className="flex items-center gap-1.5 px-3 py-2 rounded-lg text-white/70 hover:text-white hover:bg-white/10 transition-all text-sm">
                  <Shield className="w-4 h-4" />
                  <span>Validator</span>
                </Link>
              </nav>
            </div>

            {/* Right Side: Chain Selector + Account */}
            <div className="flex items-center gap-3">
              {isConnected && (
                <ChainSelector address={address} showBalance={false} compact={true} />
              )}

              {isConnected ? (
                <DropdownMenu>
                  <DropdownMenuTrigger asChild>
                    <button className="flex items-center gap-2 px-3 py-1.5 rounded-full glass border border-white/20 hover:bg-white/10 transition-all">
                      <Identicon address={address} size={24} />
                      <span className="text-sm font-mono hidden sm:inline">{formatAddress(address)}</span>
                      <ChevronDown className="w-3 h-3 text-white/60" />
                    </button>
                  </DropdownMenuTrigger>
                  <DropdownMenuContent align="end" className="glass border-white/20 w-56">
                    <div className="p-3 border-b border-white/10">
                      <div className="flex items-center gap-3">
                        <Identicon address={address} size={40} />
                        <div>
                          <p className="font-medium">Account 1</p>
                          <p className="text-xs text-white/60 font-mono">{formatAddress(address)}</p>
                        </div>
                      </div>
                    </div>
                    <DropdownMenuItem onClick={copyAddress} className="cursor-pointer">
                      {copied ? <Check className="w-4 h-4 mr-2 text-green-500" /> : <Copy className="w-4 h-4 mr-2" />}
                      {copied ? 'Copied!' : 'Copy Address'}
                    </DropdownMenuItem>
                    <DropdownMenuItem asChild>
                      <a href={explorerAccount(address)} target="_blank" rel="noopener noreferrer" className="cursor-pointer">
                        <ExternalLink className="w-4 h-4 mr-2" />
                        View on Explorer
                      </a>
                    </DropdownMenuItem>
                    <DropdownMenuSeparator className="bg-white/10" />
                    <DropdownMenuItem onClick={handleLockWallet} className="cursor-pointer">
                      <Lock className="w-4 h-4 mr-2" />
                      Lock Wallet
                    </DropdownMenuItem>
                    <DropdownMenuSeparator className="bg-white/10" />
                    <DropdownMenuItem onClick={handleDisconnectWallet} className="cursor-pointer text-red-400 focus:text-red-400">
                      <LogOut className="w-4 h-4 mr-2" />
                      Remove Wallet
                    </DropdownMenuItem>
                  </DropdownMenuContent>
                </DropdownMenu>
              ) : (
                <Button onClick={() => setShowWalletOnboarding(true)} className="btn-primary px-4 py-2 rounded-lg font-medium">
                  <Wallet className="w-4 h-4 mr-2" />
                  <span className="hidden sm:inline">Connect Wallet</span>
                  <span className="sm:hidden">Connect</span>
                </Button>
              )}
            </div>
          </div>
        </div>
      </header>

      {/* Main Content */}
      <main className="container mx-auto px-4 py-8">
        {!isConnected ? (
          /* Welcome Screen */
          <div className="flex flex-col items-center justify-center min-h-[60vh] text-center">
            <div className="glass-card p-12 rounded-2xl max-w-lg">
              <div className="w-24 h-24 mx-auto mb-6 rounded-full bg-gradient-to-br from-[#66D9E6] to-[#4DB3CC] flex items-center justify-center shadow-lg shadow-cyan-500/20">
                <Image src="/etrid-logo.png" alt="ETRID" width={56} height={56} className="rounded-full" />
              </div>
              <h1 className="text-3xl font-bold mb-4 gradient-text">Welcome to ETRID Wallet</h1>
              <p className="text-white/60 mb-8">
                Create a new wallet or import an existing one to manage your ETR tokens and interact with the ETRID network.
              </p>
              <div className="space-y-3">
                <Button onClick={() => setShowWalletOnboarding(true)} className="btn-primary w-full py-6 rounded-xl text-lg font-medium">
                  <Plus className="w-5 h-5 mr-2" />
                  Get Started
                </Button>
              </div>
              <p className="mt-6 text-white/40 text-sm">Your keys, your crypto. Non-custodial and secure.</p>
            </div>
          </div>
        ) : (
          /* Wallet Dashboard */
          <div className="max-w-2xl mx-auto space-y-6">
            {/* Balance Card with Identicon */}
            <Card className="glass-card border-0 overflow-hidden">
              <CardContent className="p-6">
                {/* Account Row */}
                <div className="flex items-center justify-center gap-2 mb-4">
                  <Identicon address={address} size={32} />
                  <button onClick={copyAddress} className="flex items-center gap-1 text-white/60 hover:text-white transition-colors">
                    <span className="font-mono text-sm">{formatAddress(address)}</span>
                    {copied ? <Check className="w-3 h-3 text-green-500" /> : <Copy className="w-3 h-3" />}
                  </button>
                </div>

                {/* Balance */}
                <div className="text-center mb-6">
                  {isPBCBalanceLoading || isLoadingBalance ? (
                    <div className="h-16 flex items-center justify-center">
                      <RefreshCw className="w-6 h-6 animate-spin text-white/40" />
                    </div>
                  ) : (
                    <>
                      <h2 className="text-5xl font-bold tracking-tight gradient-text">{displayBalance}</h2>
                      <p className="text-xl text-white/60 mt-1">{displayToken}</p>
                    </>
                  )}
                </div>

                {/* Action Buttons */}
                <div className="flex justify-center gap-6">
                  <button onClick={handleReceiveClick} className="flex flex-col items-center gap-2 group">
                    <div className="w-14 h-14 rounded-full bg-gradient-to-br from-cyan-500 to-blue-600 flex items-center justify-center shadow-lg shadow-cyan-500/20 group-hover:scale-105 transition-transform">
                      <ArrowDownLeft className="w-6 h-6 text-white" />
                    </div>
                    <span className="text-sm font-medium">Receive</span>
                  </button>

                  <button onClick={handleSendClick} className="flex flex-col items-center gap-2 group">
                    <div className="w-14 h-14 rounded-full bg-gradient-to-br from-purple-500 to-pink-600 flex items-center justify-center shadow-lg shadow-purple-500/20 group-hover:scale-105 transition-transform">
                      <ArrowUpRight className="w-6 h-6 text-white" />
                    </div>
                    <span className="text-sm font-medium">Send</span>
                  </button>

                  <button onClick={() => setShowBuyETR(true)} className="flex flex-col items-center gap-2 group">
                    <div className="w-14 h-14 rounded-full bg-gradient-to-br from-amber-500 to-orange-600 flex items-center justify-center shadow-lg shadow-amber-500/20 group-hover:scale-105 transition-transform">
                      <Plus className="w-6 h-6 text-white" />
                    </div>
                    <span className="text-sm font-medium">Buy</span>
                  </button>

                  <Link href="/staking" className="flex flex-col items-center gap-2 group">
                    <div className="w-14 h-14 rounded-full bg-gradient-to-br from-green-500 to-emerald-600 flex items-center justify-center shadow-lg shadow-green-500/20 group-hover:scale-105 transition-transform">
                      <BarChart3 className="w-6 h-6 text-white" />
                    </div>
                    <span className="text-sm font-medium">Stake</span>
                  </Link>

                  <button onClick={refreshData} className="flex flex-col items-center gap-2 group">
                    <div className="w-14 h-14 rounded-full glass border border-white/20 flex items-center justify-center group-hover:scale-105 transition-transform group-hover:bg-white/10">
                      <RefreshCw className={`w-6 h-6 ${isLoadingBalance ? 'animate-spin' : ''}`} />
                    </div>
                    <span className="text-sm font-medium">Refresh</span>
                  </button>
                </div>
              </CardContent>
            </Card>

            {/* Chain Info Banner */}
            <div className="glass-card p-4 rounded-xl border border-white/10">
              <div className="flex items-center gap-3">
                <Image src="/etrid-logo.png" alt={selectedChain.token} width={40} height={40} className="rounded-full" />
                <div className="flex-1">
                  <h3 className="font-semibold">{selectedChain.name}</h3>
                  <p className="text-white/60 text-sm">{selectedChain.description}</p>
                </div>
                <div className={`flex items-center gap-2 px-3 py-1 rounded-lg ${isPBCConnected ? 'glass border border-green-500/30' : 'glass border border-yellow-500/30'}`}>
                  <div className={`w-2 h-2 rounded-full ${isPBCConnected ? 'bg-green-500 animate-pulse' : 'bg-yellow-500 animate-pulse'}`} />
                  <span className="text-xs font-medium">{isPBCConnected ? 'Connected' : 'Connecting'}</span>
                </div>
              </div>
            </div>

            {/* Tabs - Assets / Activity */}
            <Tabs value={activeTab} onValueChange={setActiveTab}>
              <TabsList className="w-full glass rounded-xl p-1">
                <TabsTrigger value="assets" className="flex-1 rounded-lg data-[state=active]:bg-white/10">Assets</TabsTrigger>
                <TabsTrigger value="activity" className="flex-1 rounded-lg data-[state=active]:bg-white/10">Activity</TabsTrigger>
              </TabsList>

              <TabsContent value="assets" className="mt-4 space-y-2">
                <div className="flex items-center justify-between p-4 glass rounded-xl hover:bg-white/5 transition-colors cursor-pointer">
                  <div className="flex items-center gap-3">
                    <Image src="/etrid-logo.png" alt="ETR" width={40} height={40} className="rounded-full" />
                    <div>
                      <h4 className="font-semibold">ETR</h4>
                      <p className="text-white/60 text-sm">Etrid</p>
                    </div>
                  </div>
                  <div className="text-right">
                    <p className="font-semibold">{displayBalance}</p>
                    <p className="text-white/60 text-sm">$0.00</p>
                  </div>
                </div>
                <button className="w-full p-4 glass rounded-xl hover:bg-white/5 transition-colors flex items-center justify-center gap-2 text-white/60 hover:text-white">
                  <Plus className="w-4 h-4" />
                  <span>Import Token</span>
                </button>
              </TabsContent>

              <TabsContent value="activity" className="mt-4 space-y-2">
                {isLoadingTx ? (
                  <div className="flex items-center justify-center py-8">
                    <RefreshCw className="w-6 h-6 animate-spin text-white/40" />
                  </div>
                ) : transactions.length === 0 ? (
                  <div className="text-center py-8 text-white/40">
                    <Clock className="w-8 h-8 mx-auto mb-2 opacity-40" />
                    <p>No transactions yet</p>
                    <p className="text-sm">Your transaction history will appear here</p>
                  </div>
                ) : (
                  transactions.map((tx) => (
                    <a
                      key={tx.id}
                      href={explorerExtrinsic(tx.hash)}
                      target="_blank"
                      rel="noopener noreferrer"
                      className="flex items-center justify-between p-4 glass rounded-xl hover:bg-white/5 transition-colors cursor-pointer"
                    >
                      <div className="flex items-center gap-3">
                        <div className={`w-10 h-10 rounded-full flex items-center justify-center ${tx.type === 'receive' ? 'bg-green-500/20' : 'bg-purple-500/20'}`}>
                          {tx.type === 'receive' ? <ArrowDownLeft className="w-5 h-5 text-green-500" /> : <ArrowUpRight className="w-5 h-5 text-purple-500" />}
                        </div>
                        <div>
                          <h4 className="font-medium capitalize">{tx.type}</h4>
                          <p className="text-white/60 text-sm flex items-center gap-1">
                            <Clock className="w-3 h-3" />
                            {tx.time}
                          </p>
                        </div>
                      </div>
                      <div className="text-right">
                        <p className={`font-semibold ${tx.type === 'receive' ? 'text-green-500' : ''}`}>
                          {tx.type === 'receive' ? '+' : '-'}{tx.amountFormatted}
                        </p>
                        <p className="text-white/60 text-xs font-mono">{tx.hashDisplay}</p>
                      </div>
                    </a>
                  ))
                )}
              </TabsContent>
            </Tabs>

            {/* Quick Links */}
            <div className="grid grid-cols-2 gap-3">
              <Link href="/lightning" className="p-4 glass rounded-xl hover:bg-white/5 transition-colors flex items-center gap-3">
                <div className="w-10 h-10 rounded-full bg-yellow-500/20 flex items-center justify-center">
                  <Zap className="w-5 h-5 text-yellow-500" />
                </div>
                <div>
                  <p className="font-medium">Lightning</p>
                  <p className="text-xs text-white/60">Fast payments</p>
                </div>
              </Link>
              <Link href="/validator" className="p-4 glass rounded-xl hover:bg-white/5 transition-colors flex items-center gap-3">
                <div className="w-10 h-10 rounded-full bg-cyan-500/20 flex items-center justify-center">
                  <Shield className="w-5 h-5 text-cyan-500" />
                </div>
                <div>
                  <p className="font-medium">Validator</p>
                  <p className="text-xs text-white/60">Run a node</p>
                </div>
              </Link>
            </div>

            {/* Mobile App Promotion */}
            <Card className="glass-card border-0">
              <CardContent className="p-6">
                <div className="flex items-center gap-4">
                  <div className="w-12 h-12 rounded-xl bg-gradient-to-br from-[#66D9E6] to-[#4DB3CC] flex items-center justify-center flex-shrink-0">
                    <Smartphone className="w-6 h-6 text-[#0a0014]" />
                  </div>
                  <div className="flex-1">
                    <h3 className="font-bold gradient-text">Get the Mobile App</h3>
                    <p className="text-white/60 text-sm">Full DeFi experience on the go</p>
                  </div>
                  <div className="flex gap-2">
                    <a href="https://apps.apple.com/app/etrid-wallet" target="_blank" rel="noopener noreferrer" className="p-2 glass rounded-lg hover:bg-white/10 transition-colors">
                      <Download className="w-5 h-5" />
                    </a>
                  </div>
                </div>
              </CardContent>
            </Card>

            {/* Network Stats */}
            <div className="grid grid-cols-3 gap-3">
              <Card className="glass-card border-0">
                <CardContent className="p-4 text-center">
                  <div className={`w-2 h-2 rounded-full mx-auto mb-2 ${chainError ? 'bg-red-500' : isChainConnected ? 'bg-green-500 animate-pulse' : 'bg-yellow-500 animate-pulse'}`} />
                  <p className="text-xs text-white/60">Network</p>
                  <p className={`font-bold ${chainError ? 'text-red-500' : isChainConnected ? 'text-green-500' : 'text-yellow-500'}`}>
                    {chainError ? 'Error' : isChainConnected ? 'Connected' : 'Connecting'}
                  </p>
                </CardContent>
              </Card>
              <Card className="glass-card border-0">
                <CardContent className="p-4 text-center">
                  <Zap className="w-4 h-4 text-yellow-500 mx-auto mb-2" />
                  <p className="text-xs text-white/60">TPS</p>
                  <p className="font-bold">171K+</p>
                </CardContent>
              </Card>
              <Card className="glass-card border-0">
                <CardContent className="p-4 text-center">
                  <Layers className="w-4 h-4 text-purple-500 mx-auto mb-2" />
                  <p className="text-xs text-white/60">Block</p>
                  <p className="font-bold font-mono text-sm">{currentBlock ? currentBlock.toLocaleString() : '...'}</p>
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
              <Image src="/etrid-logo.png" alt="ETRID" width={32} height={32} className="rounded-lg" />
              <span className="font-semibold">ETRID Protocol</span>
            </div>
            <div className="flex items-center gap-6 text-white/60 text-sm">
              <a href="https://etrid.org" target="_blank" rel="noopener noreferrer" className="hover:text-white transition-colors">Website</a>
              <a href="https://docs.etrid.org" target="_blank" rel="noopener noreferrer" className="hover:text-white transition-colors">Docs</a>
              <a href="https://github.com/EojEdred/Etrid" target="_blank" rel="noopener noreferrer" className="hover:text-white transition-colors">GitHub</a>
              <a href="https://twitter.com/etridprotocol" target="_blank" rel="noopener noreferrer" className="hover:text-white transition-colors">Twitter</a>
            </div>
          </div>
        </div>
      </footer>

      {/* Modals */}
      <WalletOnboarding open={showWalletOnboarding} onOpenChange={setShowWalletOnboarding} />
      <SendReceive open={showSendReceive} onOpenChange={setShowSendReceive} mode={sendReceiveMode} address={address} balance={displayBalance + ' ' + displayToken} onSendTransaction={handleSendTransaction} />
      <BuyETR open={showBuyETR} onOpenChange={setShowBuyETR} />
    </div>
  )
}

export default function WalletDashboard() {
  return (
    <ChainSelectorProvider initialChainId="primearc-core">
      <WalletDashboardContent />
    </ChainSelectorProvider>
  )
}
