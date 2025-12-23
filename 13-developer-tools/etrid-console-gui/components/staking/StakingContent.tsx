'use client'

import { useState, useEffect, useMemo } from 'react'
import { Card } from '@/components/ui/card'
import { Button } from '@/components/ui/button'
import { Input } from '@/components/ui/input'
import { Badge } from '@/components/ui/badge'
import { Tabs, TabsContent, TabsList, TabsTrigger } from '@/components/ui/tabs'
import {
  Wallet,
  TrendingUp,
  Users,
  Clock,
  ArrowUpCircle,
  ArrowDownCircle,
  CheckCircle2,
  AlertCircle,
  Info,
  ExternalLink,
  Download,
} from 'lucide-react'
import { initApi } from '@/lib/polkadot/api'
import { useWallet } from '@/contexts/WalletContext'
import {
  useStakingInfo,
  useValidators,
  useWaitingValidators,
  useEraInfo,
  useMinimumStake,
  useBondingDuration,
  useBond,
  useBondExtra,
  useUnbond,
  useWithdrawUnbonded,
  useNominate,
  useChill,
  formatETR,
  truncateAddress,
} from '@/hooks/useStaking'
import ValidatorCard from './ValidatorCard'
import { useToast } from '@/hooks/use-toast'
import Link from 'next/link'

export default function StakingContent() {
  const { toast } = useToast()
  const { account, status } = useWallet()
  const address = account?.address || null

  const [apiReady, setApiReady] = useState(false)

  // Form states
  const [bondAmount, setBondAmount] = useState('')
  const [unbondAmount, setUnbondAmount] = useState('')
  const [selectedValidators, setSelectedValidators] = useState<string[]>([])

  // Filter and search
  const [searchQuery, setSearchQuery] = useState('')
  const [validatorFilter, setValidatorFilter] = useState<'all' | 'active' | 'waiting'>('active')

  // Fetch staking data
  const { data: stakingInfo, isLoading: isLoadingStaking } = useStakingInfo(address || undefined)
  const { data: validators = [], isLoading: isLoadingValidators } = useValidators()
  const { data: waitingValidators = [], isLoading: isLoadingWaiting } = useWaitingValidators()
  const { data: eraInfo } = useEraInfo()
  const { data: minimumStake } = useMinimumStake()
  const { data: bondingDuration } = useBondingDuration()

  // Mutations
  const bondMutation = useBond()
  const bondExtraMutation = useBondExtra()
  const unbondMutation = useUnbond()
  const withdrawMutation = useWithdrawUnbonded()
  const nominateMutation = useNominate()
  const chillMutation = useChill()

  // Initialize API
  useEffect(() => {
    const init = async () => {
      try {
        await initApi()
        setApiReady(true)
      } catch (error) {
        console.error('Failed to initialize API:', error)
      }
    }
    init()
  }, [])

  // Handle bond/bond extra
  const handleBond = async () => {
    if (!address || !bondAmount) return

    try {
      if (stakingInfo && BigInt(stakingInfo.active) > 0) {
        // Already bonded, use bondExtra
        await bondExtraMutation.mutateAsync({
          address,
          amount: bondAmount,
        })
        toast({
          title: 'Bond Successful',
          description: `Successfully bonded ${bondAmount} ETR`,
        })
      } else {
        // First time bonding
        await bondMutation.mutateAsync({
          address,
          amount: bondAmount,
          payee: 'Staked',
        })
        toast({
          title: 'Bond Successful',
          description: `Successfully bonded ${bondAmount} ETR`,
        })
      }
      setBondAmount('')
    } catch (error) {
      toast({
        title: 'Bond Failed',
        description: error instanceof Error ? error.message : 'Failed to bond tokens',
        variant: 'destructive',
      })
    }
  }

  // Handle unbond
  const handleUnbond = async () => {
    if (!address || !unbondAmount) return

    try {
      await unbondMutation.mutateAsync({
        address,
        amount: unbondAmount,
      })
      toast({
        title: 'Unbond Successful',
        description: `Successfully unbonded ${unbondAmount} ETR`,
      })
      setUnbondAmount('')
    } catch (error) {
      toast({
        title: 'Unbond Failed',
        description: error instanceof Error ? error.message : 'Failed to unbond tokens',
        variant: 'destructive',
      })
    }
  }

  // Handle withdraw unbonded
  const handleWithdraw = async () => {
    if (!address) return

    try {
      await withdrawMutation.mutateAsync({ address })
      toast({
        title: 'Withdrawal Successful',
        description: 'Successfully withdrew unbonded tokens',
      })
    } catch (error) {
      toast({
        title: 'Withdrawal Failed',
        description: error instanceof Error ? error.message : 'Failed to withdraw tokens',
        variant: 'destructive',
      })
    }
  }

  // Handle nominate
  const handleNominate = async () => {
    if (!address || selectedValidators.length === 0) return

    try {
      await nominateMutation.mutateAsync({
        address,
        targets: selectedValidators,
      })
      toast({
        title: 'Nomination Successful',
        description: `Successfully nominated ${selectedValidators.length} validators`,
      })
      setSelectedValidators([])
    } catch (error) {
      toast({
        title: 'Nomination Failed',
        description: error instanceof Error ? error.message : 'Failed to nominate validators',
        variant: 'destructive',
      })
    }
  }

  // Handle stop nominating
  const handleChill = async () => {
    if (!address) return

    try {
      await chillMutation.mutateAsync({ address })
      toast({
        title: 'Stopped Nominating',
        description: 'Successfully stopped all nominations',
      })
    } catch (error) {
      toast({
        title: 'Failed to Chill',
        description: error instanceof Error ? error.message : 'Failed to stop nominations',
        variant: 'destructive',
      })
    }
  }

  // Toggle validator selection
  const toggleValidatorSelection = (validatorAddress: string) => {
    setSelectedValidators((prev) => {
      if (prev.includes(validatorAddress)) {
        return prev.filter((addr) => addr !== validatorAddress)
      } else {
        // Maximum 16 validators can be nominated
        if (prev.length >= 16) {
          toast({
            title: 'Maximum Validators Reached',
            description: 'You can nominate a maximum of 16 validators',
            variant: 'destructive',
          })
          return prev
        }
        return [...prev, validatorAddress]
      }
    })
  }

  // Filter and search validators
  const filteredValidators = useMemo(() => {
    let validatorList: typeof validators = []

    if (validatorFilter === 'active') {
      validatorList = validators
    } else if (validatorFilter === 'waiting') {
      validatorList = waitingValidators
    } else {
      validatorList = [...validators, ...waitingValidators]
    }

    if (searchQuery) {
      const query = searchQuery.toLowerCase()
      validatorList = validatorList.filter(
        (v) =>
          v.address.toLowerCase().includes(query) ||
          v.identity?.display?.toLowerCase().includes(query)
      )
    }

    // Sort by total stake (descending)
    return validatorList.sort((a, b) => {
      const aStake = BigInt(a.totalStake || 0)
      const bStake = BigInt(b.totalStake || 0)
      if (aStake > bStake) return -1
      if (aStake < bStake) return 1
      return 0
    })
  }, [validators, waitingValidators, validatorFilter, searchQuery])

  // Calculate claimable unbonded tokens
  const claimableAmount = useMemo(() => {
    if (!stakingInfo || !eraInfo) return '0'

    let claimable = BigInt(0)
    stakingInfo.unlocking.forEach((unlock) => {
      if (unlock.era <= eraInfo.activeEra) {
        claimable += BigInt(unlock.value)
      }
    })

    return claimable.toString()
  }, [stakingInfo, eraInfo])

  // Calculate unlocking amount
  const unlockingAmount = useMemo(() => {
    if (!stakingInfo || !eraInfo) return '0'

    let unlocking = BigInt(0)
    stakingInfo.unlocking.forEach((unlock) => {
      if (unlock.era > eraInfo.activeEra) {
        unlocking += BigInt(unlock.value)
      }
    })

    return unlocking.toString()
  }, [stakingInfo, eraInfo])

  return (
    <div className="min-h-screen gradient-bg-animated py-8 px-4">
      <div className="max-w-7xl mx-auto space-y-6">
        {/* Header */}
        <div className="flex flex-col md:flex-row md:items-center justify-between gap-4">
          <div>
            <h1 className="text-4xl font-bold gradient-text mb-2">Native Staking</h1>
            <p className="text-muted-foreground">
              Stake ETR tokens on Primearc Core Chain to secure the network and earn rewards
            </p>
          </div>

          {!address ? (
            <div className="flex flex-col items-end gap-2">
              <Link href="/wallet">
                <Button className="bg-gradient-to-r from-primary to-accent hover:opacity-90">
                  <Wallet className="w-4 h-4 mr-2" />
                  Connect Wallet
                </Button>
              </Link>
            </div>
          ) : (
            <Card className="glass-card p-3">
              <div className="flex items-center gap-2">
                <div className="w-8 h-8 rounded-full bg-gradient-to-br from-primary to-accent" />
                <div>
                  <p className="text-xs text-muted-foreground">Connected</p>
                  <p className="text-sm font-mono">{truncateAddress(address)}</p>
                </div>
              </div>
            </Card>
          )}
        </div>

        {/* Era Info */}
        {eraInfo && (
          <Card className="glass-card p-4">
            <div className="grid grid-cols-2 md:grid-cols-4 gap-4">
              <div className="flex items-center gap-3">
                <Clock className="w-5 h-5 text-primary" />
                <div>
                  <p className="text-xs text-muted-foreground">Current Era</p>
                  <p className="text-lg font-bold">{eraInfo.activeEra}</p>
                </div>
              </div>
              <div className="flex items-center gap-3">
                <TrendingUp className="w-5 h-5 text-accent" />
                <div>
                  <p className="text-xs text-muted-foreground">Session Length</p>
                  <p className="text-lg font-bold">{eraInfo.sessionLength} blocks</p>
                </div>
              </div>
              {minimumStake && (
                <div className="flex items-center gap-3">
                  <ArrowUpCircle className="w-5 h-5 text-green-500" />
                  <div>
                    <p className="text-xs text-muted-foreground">Min. Stake</p>
                    <p className="text-lg font-bold">{formatETR(minimumStake)} ETR</p>
                  </div>
                </div>
              )}
              {bondingDuration && (
                <div className="flex items-center gap-3">
                  <Info className="w-5 h-5 text-blue-500" />
                  <div>
                    <p className="text-xs text-muted-foreground">Unbonding Period</p>
                    <p className="text-lg font-bold">{bondingDuration} eras</p>
                  </div>
                </div>
              )}
            </div>
          </Card>
        )}

        {address && (
          <>
            {/* User Staking Info */}
            <Card className="glass-card p-6">
              <h2 className="text-2xl font-bold mb-6 flex items-center gap-2">
                <Wallet className="w-6 h-6 text-primary" />
                Your Staking Position
              </h2>

              {isLoadingStaking ? (
                <div className="flex items-center justify-center py-12">
                  <div className="animate-pulse flex flex-col items-center">
                    <div className="w-12 h-12 rounded-full bg-gradient-to-br from-primary to-accent mb-3" />
                    <div className="h-3 w-32 bg-white/10 rounded" />
                  </div>
                </div>
              ) : stakingInfo ? (
                <div className="space-y-6">
                  {/* Stats Grid */}
                  <div className="grid grid-cols-1 md:grid-cols-4 gap-4">
                    <div className="space-y-2">
                      <p className="text-sm text-muted-foreground">Active Stake</p>
                      <p className="text-2xl font-bold text-green-500">
                        {formatETR(stakingInfo.active)} ETR
                      </p>
                    </div>
                    <div className="space-y-2">
                      <p className="text-sm text-muted-foreground">Total Bonded</p>
                      <p className="text-2xl font-bold">{formatETR(stakingInfo.total)} ETR</p>
                    </div>
                    <div className="space-y-2">
                      <p className="text-sm text-muted-foreground">Unlocking</p>
                      <p className="text-2xl font-bold text-amber-500">
                        {formatETR(unlockingAmount)} ETR
                      </p>
                    </div>
                    <div className="space-y-2">
                      <p className="text-sm text-muted-foreground">Claimable</p>
                      <p className="text-2xl font-bold text-blue-500">
                        {formatETR(claimableAmount)} ETR
                      </p>
                    </div>
                  </div>

                  {/* Current Nominations */}
                  {stakingInfo.nominations && stakingInfo.nominations.length > 0 && (
                    <div className="space-y-3">
                      <div className="flex items-center justify-between">
                        <h3 className="text-lg font-semibold">
                          Current Nominations ({stakingInfo.nominations.length})
                        </h3>
                        <Button
                          variant="outline"
                          size="sm"
                          onClick={handleChill}
                          disabled={chillMutation.isPending}
                        >
                          Stop Nominating
                        </Button>
                      </div>
                      <div className="space-y-2">
                        {stakingInfo.nominations.map((nom) => (
                          <div
                            key={nom}
                            className="flex items-center gap-2 p-2 rounded bg-white/5"
                          >
                            <CheckCircle2 className="w-4 h-4 text-green-500" />
                            <span className="font-mono text-sm">{truncateAddress(nom, 10, 6)}</span>
                          </div>
                        ))}
                      </div>
                    </div>
                  )}

                  {/* Unbonding Schedule */}
                  {stakingInfo.unlocking.length > 0 && (
                    <div className="space-y-3">
                      <h3 className="text-lg font-semibold">Unbonding Schedule</h3>
                      <div className="space-y-2">
                        {stakingInfo.unlocking.map((unlock, idx) => (
                          <div
                            key={idx}
                            className="flex items-center justify-between p-3 rounded bg-white/5"
                          >
                            <div>
                              <p className="font-semibold">{formatETR(unlock.value)} ETR</p>
                              <p className="text-xs text-muted-foreground">
                                {unlock.era <= (eraInfo?.activeEra || 0)
                                  ? 'Ready to claim'
                                  : `Unlocks at era ${unlock.era}`}
                              </p>
                            </div>
                            {unlock.era <= (eraInfo?.activeEra || 0) && (
                              <Badge className="bg-green-500/10 text-green-500">Claimable</Badge>
                            )}
                          </div>
                        ))}
                      </div>
                      {BigInt(claimableAmount) > 0 && (
                        <Button
                          onClick={handleWithdraw}
                          disabled={withdrawMutation.isPending}
                          className="w-full"
                        >
                          {withdrawMutation.isPending
                            ? 'Withdrawing...'
                            : `Withdraw ${formatETR(claimableAmount)} ETR`}
                        </Button>
                      )}
                    </div>
                  )}
                </div>
              ) : (
                <div className="text-center py-12">
                  <AlertCircle className="w-12 h-12 text-muted-foreground mx-auto mb-3" />
                  <p className="text-muted-foreground mb-6">
                    You don't have any staked tokens yet
                  </p>
                  <p className="text-sm text-muted-foreground">
                    Bond some ETR tokens below to start staking
                  </p>
                </div>
              )}
            </Card>

            {/* Bond/Unbond Forms */}
            <div className="grid grid-cols-1 md:grid-cols-2 gap-6">
              {/* Bond Form */}
              <Card className="glass-card p-6">
                <h3 className="text-xl font-bold mb-4 flex items-center gap-2">
                  <ArrowUpCircle className="w-5 h-5 text-green-500" />
                  Bond Tokens
                </h3>
                <div className="space-y-4">
                  <div>
                    <label className="text-sm text-muted-foreground mb-2 block">
                      Amount (ETR)
                    </label>
                    <Input
                      type="number"
                      placeholder="0.00"
                      value={bondAmount}
                      onChange={(e) => setBondAmount(e.target.value)}
                      className="glass"
                    />
                    {minimumStake && (
                      <p className="text-xs text-muted-foreground mt-1">
                        Minimum: {formatETR(minimumStake)} ETR
                      </p>
                    )}
                  </div>
                  <Button
                    onClick={handleBond}
                    disabled={
                      !bondAmount ||
                      bondMutation.isPending ||
                      bondExtraMutation.isPending ||
                      parseFloat(bondAmount) <= 0
                    }
                    className="w-full bg-gradient-to-r from-green-500 to-emerald-500"
                  >
                    {bondMutation.isPending || bondExtraMutation.isPending
                      ? 'Bonding...'
                      : stakingInfo && BigInt(stakingInfo.active) > 0
                      ? 'Bond More'
                      : 'Bond Tokens'}
                  </Button>
                </div>
              </Card>

              {/* Unbond Form */}
              <Card className="glass-card p-6">
                <h3 className="text-xl font-bold mb-4 flex items-center gap-2">
                  <ArrowDownCircle className="w-5 h-5 text-amber-500" />
                  Unbond Tokens
                </h3>
                <div className="space-y-4">
                  <div>
                    <label className="text-sm text-muted-foreground mb-2 block">
                      Amount (ETR)
                    </label>
                    <Input
                      type="number"
                      placeholder="0.00"
                      value={unbondAmount}
                      onChange={(e) => setUnbondAmount(e.target.value)}
                      className="glass"
                      disabled={!stakingInfo || BigInt(stakingInfo.active) === BigInt(0)}
                    />
                    {stakingInfo && BigInt(stakingInfo.active) > 0 && (
                      <p className="text-xs text-muted-foreground mt-1">
                        Available: {formatETR(stakingInfo.active)} ETR
                      </p>
                    )}
                  </div>
                  <Button
                    onClick={handleUnbond}
                    disabled={
                      !unbondAmount ||
                      unbondMutation.isPending ||
                      !stakingInfo ||
                      BigInt(stakingInfo.active) === BigInt(0) ||
                      parseFloat(unbondAmount) <= 0
                    }
                    className="w-full bg-gradient-to-r from-amber-500 to-orange-500"
                  >
                    {unbondMutation.isPending ? 'Unbonding...' : 'Unbond Tokens'}
                  </Button>
                  {bondingDuration && (
                    <p className="text-xs text-muted-foreground">
                      Unbonding takes {bondingDuration} eras (~{bondingDuration * 24} hours)
                    </p>
                  )}
                </div>
              </Card>
            </div>
          </>
        )}

        {/* Validators Section */}
        <Card className="glass-card p-6">
          <div className="space-y-6">
            <div className="flex flex-col md:flex-row md:items-center justify-between gap-4">
              <div>
                <h2 className="text-2xl font-bold flex items-center gap-2 mb-2">
                  <Users className="w-6 h-6 text-primary" />
                  Validators
                </h2>
                <p className="text-sm text-muted-foreground">
                  Select up to 16 validators to nominate
                  {selectedValidators.length > 0 && ` (${selectedValidators.length} selected)`}
                </p>
              </div>

              {selectedValidators.length > 0 && address && (
                <Button
                  onClick={handleNominate}
                  disabled={nominateMutation.isPending}
                  className="bg-gradient-to-r from-primary to-accent"
                >
                  {nominateMutation.isPending
                    ? 'Nominating...'
                    : `Nominate ${selectedValidators.length} Validator${
                        selectedValidators.length > 1 ? 's' : ''
                      }`}
                </Button>
              )}
            </div>

            {/* Filter and Search */}
            <div className="flex flex-col md:flex-row gap-4">
              <Input
                placeholder="Search validators by address or identity..."
                value={searchQuery}
                onChange={(e) => setSearchQuery(e.target.value)}
                className="glass flex-1"
              />
              <Tabs
                value={validatorFilter}
                onValueChange={(v) => setValidatorFilter(v as typeof validatorFilter)}
              >
                <TabsList className="glass">
                  <TabsTrigger value="active">Active ({validators.length})</TabsTrigger>
                  <TabsTrigger value="waiting">Waiting ({waitingValidators.length})</TabsTrigger>
                  <TabsTrigger value="all">
                    All ({validators.length + waitingValidators.length})
                  </TabsTrigger>
                </TabsList>
              </Tabs>
            </div>

            {/* Validators List */}
            {isLoadingValidators || isLoadingWaiting ? (
              <div className="flex items-center justify-center py-12">
                <div className="animate-pulse flex flex-col items-center">
                  <div className="w-12 h-12 rounded-full bg-gradient-to-br from-primary to-accent mb-3" />
                  <div className="h-3 w-32 bg-white/10 rounded" />
                </div>
              </div>
            ) : filteredValidators.length > 0 ? (
              <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4 max-h-[600px] overflow-y-auto pr-2">
                {filteredValidators.map((validator) => (
                  <ValidatorCard
                    key={validator.address}
                    validator={validator}
                    isSelected={selectedValidators.includes(validator.address)}
                    onToggleSelect={toggleValidatorSelection}
                    selectable={!!address && !validator.blocked}
                  />
                ))}
              </div>
            ) : (
              <div className="text-center py-12">
                <AlertCircle className="w-12 h-12 text-muted-foreground mx-auto mb-3" />
                <p className="text-muted-foreground">No validators found</p>
              </div>
            )}
          </div>
        </Card>
      </div>
    </div>
  )
}
