'use client'

import { useState, useEffect } from 'react'
import Link from 'next/link'
import { useRouter } from 'next/navigation'
import Image from 'next/image'
import { Wallet, BarChart3, Shield, Home, RefreshCw, Layers, Download, Globe, Activity, HelpCircle } from 'lucide-react'
import { Button } from '@/components/ui/button'
import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card'
import { Tabs, TabsContent, TabsList, TabsTrigger } from '@/components/ui/tabs'
import { useWallet } from '@/contexts/WalletContext'
import { usePolkadotApi } from '@/hooks/usePolkadotApi'
import { useBalance } from '@/hooks/useBalance'
import { ChainSelector } from '@/components/wallet/ChainSelector'
import { ChainSelectorProvider } from '@/hooks/useChainSelector'
import { useToast } from '@/hooks/use-toast'
import { OnboardingModal } from '@/components/onboarding/OnboardingModal'
import { NetworkMap } from '@/components/console/NetworkMap'
import { Terminal } from '@/components/console/Terminal'
import { useNetworkStats } from '@/hooks/useNetworkStats'
import { NodePeers } from '@/components/console/NodePeers'
import { explorerAccount, explorerBlock, explorerHome } from '@/lib/explorer'

export default function ConsoleDashboard() {
  const { toast } = useToast()
  const { status, account } = useWallet()
  const { isConnected, chainInfo, currentBlock, reconnect: reconnectPolkadot } = usePolkadotApi()
  const { nodes, stats: netStats } = useNetworkStats()
  const address = account?.address || ''
  const { balance, isLoading: isLoadingBalance, refetch: refreshBalance } = useBalance(address)
  
  const [activeTab, setActiveTab] = useState('dashboard')
  const [showOnboarding, setShowOnboarding] = useState(false)
  const router = useRouter()

  // Debugging log for wallet connection state
  useEffect(() => {
    console.log('[Dashboard] Wallet Status:', status, 'Account:', account?.address);
  }, [status, account]);

  useEffect(() => {
    try {
      // Show onboarding unless it's explicitly completed.
      const completed = localStorage.getItem('etrid_onboarding_completed') === 'true'
      setShowOnboarding(!completed)
    } catch {
      // If storage is unavailable, default to showing onboarding.
      setShowOnboarding(true)
    }
  }, [])

  const handleOnboardingComplete = () => {
    try {
      localStorage.setItem('etrid_onboarding_completed', 'true')
    } catch {
      // Ignore storage write failures (e.g., restricted environments).
    }
    setShowOnboarding(false)
    toast({
      title: "Onboarding Complete",
      description: "Welcome to Etrid Console!",
    })
    void reconnectPolkadot()
  }

  const isConnectedToWallet = status === 'unlocked' && !!address

  const handleConnect = () => {
    router.push('/wallet')
  }

  const quickActions = [
    { 
      title: 'Stake Tokens', 
      icon: BarChart3, 
      color: 'from-green-500 to-emerald-600',
      href: '/staking',
      description: 'Minimum 64 ETR'
    },
    {
      title: 'EVM Wallet',
      icon: Layers,
      color: 'from-indigo-500 to-purple-600',
      href: '/evm',
      description: 'MetaMask-style'
    },
    { 
      title: 'Create Wallet', 
      icon: Wallet, 
      color: 'from-blue-500 to-cyan-600', 
      href: '/wallet',
      description: 'Secure storage'
    },
    { 
      title: 'Run Validator', 
      icon: Shield, 
      color: 'from-purple-500 to-pink-600', 
      href: '/validator',
      description: 'Earn rewards'
    },
    { 
      title: 'Network Map', 
      icon: Globe, 
      color: 'from-amber-500 to-orange-600', 
      href: '#network',
      onClick: () => setActiveTab('network'),
      description: 'View topology'
    },
  ]

  return (
    <ChainSelectorProvider initialChainId="primearc-core">
      <div className="min-h-screen gradient-bg-animated">
        <OnboardingModal open={showOnboarding} onComplete={handleOnboardingComplete} />
        <Terminal balance={balance} />
        {/* Header */}
        <header className="sticky top-0 z-50 glass border-b border-white/10">
        <div className="container mx-auto px-4 py-3">
          <div className="flex items-center justify-between">
            {/* Logo + Navigation */}
            <div className="flex items-center gap-6">
              <Link href="/" className="flex items-center gap-2">
                <Image src="/etrid-logo.png" alt="ETRID" width={36} height={36} className="rounded-xl" />
                <span className="font-bold text-lg hidden sm:inline gradient-text">ETRID CONSOLE</span>
              </Link>

              {/* Navigation Tabs */}
              <nav className="hidden md:flex items-center gap-1">
                <Link href="/" onClick={() => setActiveTab('dashboard')} className={`flex items-center gap-1.5 px-3 py-2 rounded-lg text-sm transition-all ${
                  activeTab === 'dashboard' 
                    ? 'text-white bg-white/10' 
                    : 'text-white/70 hover:text-white hover:bg-white/10'
                }`}>
                  <Home className="w-4 h-4 mr-1" />
                  <span>Dashboard</span>
                </Link>
                <Link href="/staking" className="flex items-center gap-1.5 px-3 py-2 rounded-lg text-white/70 hover:text-white hover:bg-white/10 transition-all text-sm">
                  <BarChart3 className="w-4 h-4 mr-1" />
                  <span>Staking</span>
                </Link>
                <Link href="/validator" className="flex items-center gap-1.5 px-3 py-2 rounded-lg text-white/70 hover:text-white hover:bg-white/10 transition-all text-sm">
                  <Shield className="w-4 h-4 mr-1" />
                  <span>Validator</span>
                </Link>
                <button onClick={() => setActiveTab('network')} className={`flex items-center gap-1.5 px-3 py-2 rounded-lg text-sm transition-all ${
                  activeTab === 'network' 
                    ? 'text-white bg-white/10' 
                    : 'text-white/70 hover:text-white hover:bg-white/10'
                }`}>
                  <Globe className="w-4 h-4 mr-1" />
                  <span>Network</span>
                </button>
              </nav>
            </div>

            {/* Right Side: Chain Selector + Account */}
            <div className="flex items-center gap-3">
              <Button
                variant="outline"
                size="icon"
                onClick={() => setShowOnboarding(true)}
                className="glass border-white/20 text-white/80 hover:text-white hover:bg-white/10"
                title="Setup / Onboarding"
              >
                <HelpCircle className="w-4 h-4" />
              </Button>
              {isConnectedToWallet && (
                <ChainSelector address={address} showBalance={true} compact={false} />
              )}

              {isConnectedToWallet ? (
                <div className="flex items-center gap-2 px-3 py-1.5 rounded-full glass border border-white/20">
                  <div className="w-2 h-2 rounded-full bg-green-500 animate-pulse" />
                  <span className="text-sm font-mono text-white/70">
                    {address.slice(0, 6)}...{address.slice(-4)}
                  </span>
                </div>
              ) : (
                <Button onClick={handleConnect} className="btn-primary px-4 py-2 rounded-lg font-medium">
                  <Wallet className="w-4 h-4 mr-2" />
                  Connect Wallet
                </Button>
              )}
            </div>
          </div>
        </div>
      </header>

      {/* Main Content */}
      <main className="container mx-auto px-4 py-8">
        <div className="max-w-6xl mx-auto">
          {/* Welcome Section */}
          <div className="text-center mb-12">
            <div className="inline-flex items-center gap-2 mb-4">
              <h1 className="text-4xl md:text-5xl font-bold gradient-text bg-gradient-to-r from-cyan-400 via-purple-500 to-pink-500 bg-clip-text text-transparent">
                Etrid Multichain Console
              </h1>
            </div>
            <p className="text-white/70 text-lg max-w-2xl mx-auto">
              Your complete interface for Etrid network participation. Manage wallets, stake tokens, run validators, and access documentation.
            </p>
          </div>

          {!isConnectedToWallet ? (
            /* Not Connected - Welcome Screen */
            <div className="flex flex-col items-center justify-center min-h-[60vh] text-center">
              <div className="glass-card p-12 rounded-2xl max-w-lg">
                <div className="w-24 h-24 mx-auto mb-6 rounded-full bg-gradient-to-br from-[#66D9E6] to-[#4DB3CC] flex items-center justify-center shadow-lg shadow-cyan-500/20">
                  <Image src="/etrid-logo.png" alt="ETRID" width={56} height={56} className="rounded-full" />
                </div>
                <h1 className="text-3xl font-bold mb-4 gradient-text">Welcome to ETRID Console</h1>
                <p className="text-white/60 mb-8">
                  Connect your wallet to manage your ETR tokens, stake, validate, and interact with the ETRID network.
                </p>
                <div className="space-y-3">
                  <Button onClick={handleConnect} className="btn-primary w-full py-6 rounded-xl text-lg font-medium">
                    <Wallet className="w-5 h-5 mr-2" />
                    Connect Wallet
                  </Button>
                </div>
                <p className="mt-6 text-white/40 text-sm">Secure, decentralized, and non-custodial.</p>
              </div>
            </div>
          ) : (
            /* Connected - Console Dashboard */
            <Tabs value={activeTab} onValueChange={setActiveTab} className="space-y-6">
              <TabsList className="w-full grid grid-cols-2 md:grid-cols-4 glass rounded-xl p-1 max-w-2xl mx-auto">
                <TabsTrigger value="dashboard" className="rounded-lg data-[state=active]:bg-white/10">
                  <Home className="w-4 h-4 mr-2" />
                  Dashboard
                </TabsTrigger>
                <TabsTrigger value="staking" className="rounded-lg data-[state=active]:bg-white/10">
                  <BarChart3 className="w-4 h-4 mr-2" />
                  Staking
                </TabsTrigger>
                <TabsTrigger value="validator" className="rounded-lg data-[state=active]:bg-white/10">
                  <Shield className="w-4 h-4 mr-2" />
                  Validator
                </TabsTrigger>
                <TabsTrigger value="network" className="rounded-lg data-[state=active]:bg-white/10">
                  <Globe className="w-4 h-4 mr-2" />
                  Network
                </TabsTrigger>
              </TabsList>

              {/* Dashboard Tab */}
              <TabsContent value="dashboard" className="space-y-6">
                {/* Quick Actions */}
                <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-4">
                  {quickActions.map((action, index) => {
                    const Icon = action.icon
                    return (
                      <div 
                        key={index} 
                        onClick={action.onClick ? action.onClick : undefined}
                        className="block cursor-pointer"
                      >
                        {action.onClick ? (
                          <Card className="glass-card hover:bg-white/10 transition-all border-0 h-full group">
                            <CardContent className="p-6 flex flex-col items-center text-center">
                              <div className={`w-14 h-14 rounded-full bg-gradient-to-br ${action.color} flex items-center justify-center mb-4 group-hover:scale-105 transition-transform`}>
                                <Icon className="w-6 h-6 text-white" />
                              </div>
                              <h3 className="font-semibold mb-1">{action.title}</h3>
                              <p className="text-white/60 text-sm mb-2">{action.description}</p>
                              <span className="text-xs text-cyan-400 font-medium">View →</span>
                            </CardContent>
                          </Card>
                        ) : (
                          <Link href={action.href}>
                            <Card className="glass-card hover:bg-white/10 transition-all border-0 h-full group">
                              <CardContent className="p-6 flex flex-col items-center text-center">
                                <div className={`w-14 h-14 rounded-full bg-gradient-to-br ${action.color} flex items-center justify-center mb-4 group-hover:scale-105 transition-transform`}>
                                  <Icon className="w-6 h-6 text-white" />
                                </div>
                                <h3 className="font-semibold mb-1">{action.title}</h3>
                                <p className="text-white/60 text-sm mb-2">{action.description}</p>
                                <span className="text-xs text-cyan-400 font-medium">Continue →</span>
                              </CardContent>
                            </Card>
                          </Link>
                        )}
                      </div>
                    )
                  })}
                </div>

                {/* Wallet Overview */}
                <div className="grid grid-cols-1 lg:grid-cols-3 gap-6">
                  {/* Balance Card */}
                  <Card className="glass-card border-0 lg:col-span-2">
                    <CardHeader>
                      <CardTitle className="flex items-center justify-between">
                        <span>Wallet Overview</span>
                        <button onClick={refreshBalance} className="p-2 rounded-lg glass border border-white/20 hover:bg-white/10 transition-colors">
                          <RefreshCw className={`w-4 h-4 ${isLoadingBalance ? 'animate-spin' : ''}`} />
                        </button>
                      </CardTitle>
                    </CardHeader>
                    <CardContent>
                      <div className="flex items-center justify-between mb-6">
                        <div>
                          <h2 className="text-3xl font-bold gradient-text">
                            {balance?.formatted || '0 ETR'}
                          </h2>
                          <p className="text-white/60">{address && `${address.slice(0, 6)}...${address.slice(-4)}`}</p>
                        </div>
                        <div className="text-right">
                          <div className={`inline-flex items-center gap-2 px-3 py-1 rounded-full text-xs font-medium ${
                            isConnected ? 'text-green-400 bg-green-500/20' : 'text-yellow-400 bg-yellow-500/20'
                          }`}>
                            <div className={`w-2 h-2 rounded-full ${isConnected ? 'bg-green-500 animate-pulse' : 'bg-yellow-500 animate-pulse'}`} />
                            {isConnected ? 'Connected' : 'Connecting'}
                          </div>
                          <p className="text-white/60 text-xs mt-1">to ETRID network</p>
                        </div>
                      </div>
                      
                      {/* Quick Actions */}
                      <div className="flex gap-3">
                        <Button className="flex-1 bg-gradient-to-r from-green-500 to-emerald-600 hover:from-green-600 hover:to-emerald-700">
                          <BarChart3 className="w-4 h-4 mr-2" />
                          Stake
                        </Button>
                        <Button variant="outline" className="flex-1 border-white/20 hover:bg-white/10">
                          <Download className="w-4 h-4 mr-2" />
                          Send
                        </Button>
                      </div>
                    </CardContent>
                  </Card>

                  {/* Network Stats */}
                  <Card className="glass-card border-0">
                    <CardHeader>
                      <CardTitle>Network Stats</CardTitle>
                    </CardHeader>
                    <CardContent>
                      <div className="space-y-4">
                        <div className="flex justify-between">
                          <span className="text-white/60">Current Block</span>
                          <span className="font-mono font-medium">
                            {currentBlock ? currentBlock.toLocaleString() : '...'}
                          </span>
                        </div>
                        <div className="flex justify-between">
                          <span className="text-white/60">Chain</span>
                          <span>{netStats.chainName || chainInfo?.chain || '—'}</span>
                        </div>
                        <div className="flex justify-between">
                          <span className="text-white/60">Block Time</span>
                          <span className="font-mono">
                            {netStats.blockTime ? `${netStats.blockTime.toFixed(2)}s` : '—'}
                          </span>
                        </div>
                        <div className="flex justify-between">
                          <span className="text-white/60">TPS</span>
                          <span className="font-mono">{netStats.tps ? netStats.tps.toFixed(2) : '—'}</span>
                        </div>
                      </div>
                    </CardContent>
                  </Card>
                </div>

                {/* Recent Activity */}
                <Card className="glass-card border-0">
                  <CardHeader>
                    <CardTitle>Recent Activity</CardTitle>
                  </CardHeader>
                  <CardContent>
                    <div className="text-center py-8 text-white/40">
                      <p className="text-lg mb-2">No recent activity</p>
                      <p className="text-sm">Your transactions will appear here</p>
                    </div>
                  </CardContent>
                </Card>
              </TabsContent>

              {/* Staking Tab - Will be populated with staking content */}
              <TabsContent value="staking">
                <div className="text-center py-12">
                  <BarChart3 className="w-16 h-16 mx-auto text-cyan-500 mb-4" />
                  <h3 className="text-2xl font-bold mb-2">Staking Dashboard</h3>
                  <p className="text-white/70 mb-6">Manage your staking activities and rewards</p>
                  <Link href="/staking">
                    <Button className="btn-primary">
                      <BarChart3 className="w-4 h-4 mr-2" />
                      Go to Staking
                    </Button>
                  </Link>
                </div>
              </TabsContent>

              {/* Validator Tab - Will be populated with validator content */}
              <TabsContent value="validator">
                <div className="text-center py-12">
                  <Shield className="w-16 h-16 mx-auto text-purple-500 mb-4" />
                  <h3 className="text-2xl font-bold mb-2">Validator Dashboard</h3>
                  <p className="text-white/70 mb-6">Manage your validator node and earnings</p>
                  <Link href="/validator">
                    <Button className="btn-primary">
                      <Shield className="w-4 h-4 mr-2" />
                      Go to Validator
                    </Button>
                  </Link>
                </div>
              </TabsContent>

              {/* Network Tab */}
              <TabsContent value="network">
                <div className="space-y-6">
                  <div className="flex items-center justify-between">
                    <div>
                      <h3 className="text-2xl font-bold">Network Topology</h3>
                      <p className="text-white/60">Live visualization of network nodes and peers</p>
                    </div>
                    <div className="flex items-center gap-4 flex-wrap justify-end">
                      <div className="flex items-center gap-2 flex-wrap justify-end">
                        <a href={explorerHome()} target="_blank" rel="noopener noreferrer">
                          <Button variant="outline" size="sm" className="border-white/20 hover:bg-white/10">
                            Open Explorer
                          </Button>
                        </a>
                        {address ? (
                          <a href={explorerAccount(address)} target="_blank" rel="noopener noreferrer">
                            <Button variant="outline" size="sm" className="border-white/20 hover:bg-white/10">
                              My Account
                            </Button>
                          </a>
                        ) : null}
                        {netStats.blockHeight ? (
                          <a href={explorerBlock(netStats.blockHeight)} target="_blank" rel="noopener noreferrer">
                            <Button variant="outline" size="sm" className="border-white/20 hover:bg-white/10">
                              Block #{netStats.blockHeight}
                            </Button>
                          </a>
                        ) : null}
                      </div>
                      <div className="px-4 py-2 rounded-lg glass border border-white/10">
                        <span className="text-white/60 text-sm block">Validator Nodes</span>
                        <span className="text-xl font-bold">{netStats.validatorCount}</span>
                      </div>
                      <div className="px-4 py-2 rounded-lg glass border border-white/10">
                        <span className="text-white/60 text-sm block">Total Nodes</span>
                        <span className="text-xl font-bold">{netStats.nodeCount}</span>
                      </div>
                    </div>
                  </div>
                  
                  <NetworkMap nodes={nodes} />
                  
                  <div className="grid grid-cols-1 md:grid-cols-2 gap-6">
                    <Card className="glass-card">
                      <CardHeader>
                        <CardTitle className="flex items-center gap-2">
                          <Activity className="w-5 h-5 text-green-400" />
                          Network Health
                        </CardTitle>
                      </CardHeader>
                      <CardContent>
                        <div className="space-y-4">
                          <div className="flex justify-between items-center">
                            <span className="text-white/60">Current Block</span>
                            {netStats.blockHeight ? (
                              <a
                                href={explorerBlock(netStats.blockHeight)}
                                target="_blank"
                                rel="noopener noreferrer"
                                className="font-mono text-cyan-300 hover:underline"
                              >
                                {netStats.blockHeight}
                              </a>
                            ) : (
                              <span className="font-mono">—</span>
                            )}
                          </div>
                          <div className="flex justify-between items-center">
                            <span className="text-white/60">Finalized Height</span>
                            <span className="font-mono">{netStats.finalizedHeight}</span>
                          </div>
                          <div className="flex justify-between items-center">
                            <span className="text-white/60">Target Block Time</span>
                            <span className="font-mono">
                              {netStats.blockTime ? `${netStats.blockTime.toFixed(2)}s` : '—'}
                            </span>
                          </div>
                          <div className="flex justify-between items-center">
                            <span className="text-white/60">Active Peers</span>
                            <span className="font-mono text-green-400">{netStats.peerCount}</span>
                          </div>
                        </div>
                      </CardContent>
                    </Card>
                    
                    <Card className="glass-card">
                      <CardHeader>
                        <CardTitle>Node List</CardTitle>
                      </CardHeader>
                      <CardContent>
                        <div className="space-y-2 max-h-[200px] overflow-y-auto pr-2">
                          {nodes.map((node, i) => (
                            <div key={i} className="flex items-center justify-between p-2 rounded bg-white/5 border border-white/5">
                              <div className="flex items-center gap-3">
                                <div className={`w-2 h-2 rounded-full ${node.status === 'online' ? 'bg-green-500' : 'bg-red-500'}`} />
                                <div>
                                  <p className="text-sm font-medium">{node.name}</p>
                                  <p className="text-xs text-white/40">{node.location}</p>
                                </div>
                              </div>
                              <span className="text-xs font-mono text-white/60">{node.version}</span>
                            </div>
                          ))}
                        </div>
                      </CardContent>
                    </Card>
                  </div>

                  <NodePeers enabled={isConnected} />
                </div>
              </TabsContent>
            </Tabs>
          )}
        </div>
      </main>

      {/* Footer */}
      <footer className="border-t border-white/10 mt-16">
        <div className="container mx-auto px-4 py-8">
          <div className="flex flex-col md:flex-row items-center justify-between gap-4">
            <div className="flex items-center gap-2">
              <Image src="/etrid-logo.png" alt="ETRID" width={32} height={32} className="rounded-lg" />
              <span className="font-semibold">ETRID Console</span>
            </div>
            <div className="flex items-center gap-6 text-white/60 text-sm">
              <span>Minimum Stake: 64 ETR</span>
              <span>•</span>
              <span>Unbonding: 21 days</span>
              <span>•</span>
              <span>
                Block Time:{' '}
                {netStats.blockTime ? `${netStats.blockTime.toFixed(2)}s` : '—'}
              </span>
            </div>
          </div>
        </div>
      </footer>
    </div>
      </ChainSelectorProvider>
  )
}
