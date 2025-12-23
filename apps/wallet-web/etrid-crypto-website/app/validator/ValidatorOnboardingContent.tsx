'use client'

import { useState, useEffect } from 'react'
import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card'
import { Button } from '@/components/ui/button'
import { Input } from '@/components/ui/input'
import { Badge } from '@/components/ui/badge'
import {
  Wallet,
  CheckCircle2,
  Circle,
  ArrowRight,
  Copy,
  ExternalLink,
  AlertCircle,
  Loader2,
  Shield,
  Coins,
  Key,
  Server,
} from 'lucide-react'
import { enableExtension, initApi } from '@/lib/polkadot/api'
import {
  useStakingInfo,
  useMinimumValidatorBond,
  useValidatorCounts,
  useIsValidator,
  useBond,
  useSetKeys,
  useValidate,
  formatETR,
  truncateAddress,
} from '@/hooks/useStaking'
import { useToast } from '@/hooks/use-toast'

type Step = 'connect' | 'bond' | 'keys' | 'validate' | 'complete'

export default function ValidatorOnboardingContent() {
  const { toast } = useToast()

  // Wallet state
  const [address, setAddress] = useState<string | null>(null)
  const [isConnecting, setIsConnecting] = useState(false)
  const [extensionError, setExtensionError] = useState<string | null>(null)
  const [apiReady, setApiReady] = useState(false)

  // Form state
  const [bondAmount, setBondAmount] = useState('')
  const [sessionKeys, setSessionKeys] = useState('')
  const [commission, setCommission] = useState('10')

  // Current step
  const [currentStep, setCurrentStep] = useState<Step>('connect')

  // Query hooks
  const { data: stakingInfo, isLoading: isLoadingStaking } = useStakingInfo(address || undefined)
  const { data: minValidatorBond } = useMinimumValidatorBond()
  const { data: validatorCounts } = useValidatorCounts()
  const { data: isAlreadyValidator } = useIsValidator(address || undefined)

  // Mutation hooks
  const bondMutation = useBond()
  const setKeysMutation = useSetKeys()
  const validateMutation = useValidate()

  // Initialize API
  useEffect(() => {
    const init = async () => {
      try {
        await initApi()
        setApiReady(true)
      } catch (error) {
        console.error('Failed to initialize API:', error)
        toast({
          title: 'Network Connection Failed',
          description: 'Unable to connect to ETRID network. Please try again later.',
          variant: 'destructive',
        })
      }
    }
    init()
  }, [])

  // Determine current step based on state
  useEffect(() => {
    if (!address) {
      setCurrentStep('connect')
    } else if (isAlreadyValidator) {
      setCurrentStep('complete')
    } else if (!stakingInfo || BigInt(stakingInfo.active) === BigInt(0)) {
      setCurrentStep('bond')
    } else if (currentStep === 'bond') {
      // Move to keys after bonding
      setCurrentStep('keys')
    }
  }, [address, stakingInfo, isAlreadyValidator])

  const handleConnect = async () => {
    setIsConnecting(true)
    setExtensionError(null)
    try {
      const accounts = await enableExtension()
      if (accounts.length > 0) {
        setAddress(accounts[0].address)
        toast({
          title: 'Wallet Connected',
          description: `Connected to ${truncateAddress(accounts[0].address)}`,
        })
      }
    } catch (error) {
      const errorMessage = error instanceof Error ? error.message : 'Failed to connect wallet'
      // Check if it's an extension-related error
      if (errorMessage.includes('extension') || errorMessage.includes('No Polkadot')) {
        setExtensionError('polkadot-extension')
      } else if (errorMessage.includes('No accounts')) {
        setExtensionError('no-accounts')
      } else {
        toast({
          title: 'Connection Failed',
          description: errorMessage,
          variant: 'destructive',
        })
      }
    } finally {
      setIsConnecting(false)
    }
  }

  const handleBond = async () => {
    if (!address || !bondAmount) return

    try {
      // Convert to planck (12 decimals)
      const amountInPlanck = BigInt(Math.floor(parseFloat(bondAmount) * 1e12)).toString()

      await bondMutation.mutateAsync({
        address,
        amount: amountInPlanck,
        payee: 'Staked',
      })

      toast({
        title: 'Tokens Bonded',
        description: `Successfully bonded ${bondAmount} ETR`,
      })

      setCurrentStep('keys')
    } catch (error) {
      toast({
        title: 'Bond Failed',
        description: error instanceof Error ? error.message : 'Failed to bond tokens',
        variant: 'destructive',
      })
    }
  }

  const handleSetKeys = async () => {
    if (!address || !sessionKeys) return

    // Validate session keys format (should be hex starting with 0x)
    if (!sessionKeys.startsWith('0x') || sessionKeys.length < 66) {
      toast({
        title: 'Invalid Session Keys',
        description: 'Session keys should be a hex string starting with 0x',
        variant: 'destructive',
      })
      return
    }

    try {
      await setKeysMutation.mutateAsync({
        address,
        keys: sessionKeys,
      })

      toast({
        title: 'Session Keys Set',
        description: 'Your session keys have been registered on-chain',
      })

      setCurrentStep('validate')
    } catch (error) {
      toast({
        title: 'Set Keys Failed',
        description: error instanceof Error ? error.message : 'Failed to set session keys',
        variant: 'destructive',
      })
    }
  }

  const handleValidate = async () => {
    if (!address) return

    const commissionValue = parseFloat(commission)
    if (isNaN(commissionValue) || commissionValue < 0 || commissionValue > 100) {
      toast({
        title: 'Invalid Commission',
        description: 'Commission must be between 0 and 100',
        variant: 'destructive',
      })
      return
    }

    try {
      await validateMutation.mutateAsync({
        address,
        commission: commissionValue,
      })

      toast({
        title: 'Validator Registered!',
        description: 'You are now a validator candidate',
      })

      setCurrentStep('complete')
    } catch (error) {
      toast({
        title: 'Validation Failed',
        description: error instanceof Error ? error.message : 'Failed to register as validator',
        variant: 'destructive',
      })
    }
  }

  const copyToClipboard = (text: string) => {
    navigator.clipboard.writeText(text)
    toast({ title: 'Copied to clipboard' })
  }

  const steps = [
    { id: 'connect', label: 'Connect Wallet', icon: Wallet },
    { id: 'bond', label: 'Bond Tokens', icon: Coins },
    { id: 'keys', label: 'Set Session Keys', icon: Key },
    { id: 'validate', label: 'Start Validating', icon: Server },
  ]

  const getStepStatus = (stepId: string) => {
    const stepOrder = ['connect', 'bond', 'keys', 'validate', 'complete']
    const currentIndex = stepOrder.indexOf(currentStep)
    const stepIndex = stepOrder.indexOf(stepId)

    if (stepIndex < currentIndex) return 'complete'
    if (stepIndex === currentIndex) return 'current'
    return 'pending'
  }

  return (
    <div className="min-h-screen py-8 px-4">
      <div className="max-w-4xl mx-auto space-y-8">
        {/* Header */}
        <div className="text-center">
          <h1 className="text-4xl font-bold gradient-text mb-4">Become a Validator</h1>
          <p className="text-white/60 max-w-2xl mx-auto">
            Secure the ETRID network and earn rewards by running a validator node.
            Follow these steps to get started.
          </p>
        </div>

        {/* Network Stats */}
        {validatorCounts && (
          <div className="grid grid-cols-3 gap-4">
            <Card className="glass-card">
              <CardContent className="p-4 text-center">
                <p className="text-2xl font-bold text-green-500">{validatorCounts.current}</p>
                <p className="text-sm text-white/60">Active Validators</p>
              </CardContent>
            </Card>
            <Card className="glass-card">
              <CardContent className="p-4 text-center">
                <p className="text-2xl font-bold">{validatorCounts.max}</p>
                <p className="text-sm text-white/60">Max Validators</p>
              </CardContent>
            </Card>
            <Card className="glass-card">
              <CardContent className="p-4 text-center">
                <p className="text-2xl font-bold text-cyan-500">{validatorCounts.slotsAvailable}</p>
                <p className="text-sm text-white/60">Slots Available</p>
              </CardContent>
            </Card>
          </div>
        )}

        {/* Progress Steps */}
        <Card className="glass-card">
          <CardContent className="p-6">
            <div className="flex items-center justify-between">
              {steps.map((step, index) => (
                <div key={step.id} className="flex items-center">
                  <div className="flex flex-col items-center">
                    <div className={`w-12 h-12 rounded-full flex items-center justify-center ${
                      getStepStatus(step.id) === 'complete'
                        ? 'bg-green-500'
                        : getStepStatus(step.id) === 'current'
                        ? 'bg-gradient-to-br from-cyan-500 to-purple-500'
                        : 'bg-white/10'
                    }`}>
                      {getStepStatus(step.id) === 'complete' ? (
                        <CheckCircle2 className="w-6 h-6 text-white" />
                      ) : (
                        <step.icon className="w-6 h-6 text-white" />
                      )}
                    </div>
                    <span className={`text-sm mt-2 ${
                      getStepStatus(step.id) === 'current' ? 'text-white font-medium' : 'text-white/60'
                    }`}>
                      {step.label}
                    </span>
                  </div>
                  {index < steps.length - 1 && (
                    <div className={`w-24 h-0.5 mx-4 ${
                      getStepStatus(steps[index + 1].id) === 'complete' || getStepStatus(steps[index + 1].id) === 'current'
                        ? 'bg-green-500'
                        : 'bg-white/10'
                    }`} />
                  )}
                </div>
              ))}
            </div>
          </CardContent>
        </Card>

        {/* Step Content */}
        {currentStep === 'connect' && (
          <Card className="glass-card">
            <CardHeader>
              <CardTitle className="flex items-center gap-2">
                <Wallet className="w-5 h-5 text-cyan-500" />
                Step 1: Connect Your Wallet
              </CardTitle>
            </CardHeader>
            <CardContent className="space-y-4">
              <p className="text-white/60">
                Connect your Polkadot.js wallet to begin the validator registration process.
                Make sure you have the Polkadot.js browser extension installed.
              </p>
              <Button
                onClick={handleConnect}
                disabled={isConnecting || !apiReady}
                className="btn-primary"
              >
                {!apiReady ? (
                  <>
                    <Loader2 className="w-4 h-4 mr-2 animate-spin" />
                    Connecting to network...
                  </>
                ) : isConnecting ? (
                  <>
                    <Loader2 className="w-4 h-4 mr-2 animate-spin" />
                    Connecting...
                  </>
                ) : (
                  <>
                    <Wallet className="w-4 h-4 mr-2" />
                    Connect Wallet
                  </>
                )}
              </Button>

              {/* Extension Error Messages */}
              {extensionError === 'polkadot-extension' && (
                <Card className="glass-card border-amber-500/30 p-4">
                  <div className="flex items-start gap-3">
                    <AlertCircle className="w-5 h-5 text-amber-500 mt-0.5 flex-shrink-0" />
                    <div>
                      <p className="font-medium text-amber-500">Polkadot.js Extension Required</p>
                      <p className="text-white/60 text-sm mt-1">
                        To connect your wallet and become a validator, you need to install the Polkadot.js browser extension.
                      </p>
                      <a
                        href="https://polkadot.js.org/extension/"
                        target="_blank"
                        rel="noopener noreferrer"
                        className="inline-flex items-center gap-2 mt-3 px-4 py-2 bg-cyan-500/20 text-cyan-400 rounded-lg hover:bg-cyan-500/30 transition-colors text-sm font-medium"
                      >
                        Download Extension <ExternalLink className="w-4 h-4" />
                      </a>
                    </div>
                  </div>
                </Card>
              )}
              {extensionError === 'no-accounts' && (
                <Card className="glass-card border-amber-500/30 p-4">
                  <div className="flex items-start gap-3">
                    <AlertCircle className="w-5 h-5 text-amber-500 mt-0.5 flex-shrink-0" />
                    <div>
                      <p className="font-medium text-amber-500">No Accounts Found</p>
                      <p className="text-white/60 text-sm mt-1">
                        Please create or import an account in your Polkadot.js extension, then try connecting again.
                      </p>
                    </div>
                  </div>
                </Card>
              )}

              {/* Install Guide - only show if no error yet */}
              {!extensionError && (
                <div className="p-4 glass rounded-lg">
                  <p className="text-sm text-white/60">
                    Don't have the extension?{' '}
                    <a
                      href="https://polkadot.js.org/extension/"
                      target="_blank"
                      rel="noopener noreferrer"
                      className="text-cyan-400 hover:underline inline-flex items-center gap-1"
                    >
                      Download Polkadot.js <ExternalLink className="w-3 h-3" />
                    </a>
                  </p>
                </div>
              )}
            </CardContent>
          </Card>
        )}

        {currentStep === 'bond' && (
          <Card className="glass-card">
            <CardHeader>
              <CardTitle className="flex items-center gap-2">
                <Coins className="w-5 h-5 text-yellow-500" />
                Step 2: Bond Tokens
              </CardTitle>
            </CardHeader>
            <CardContent className="space-y-6">
              <p className="text-white/60">
                Bond ETR tokens to become eligible for validation. The minimum validator bond is required.
              </p>

              {minValidatorBond && (
                <div className="p-4 glass rounded-lg">
                  <div className="flex items-center justify-between">
                    <span className="text-white/60">Minimum Validator Bond</span>
                    <span className="font-bold text-lg">{formatETR(minValidatorBond)} ETR</span>
                  </div>
                </div>
              )}

              <div>
                <label className="text-sm text-white/60 mb-2 block">Amount to Bond (ETR)</label>
                <Input
                  type="number"
                  placeholder="Enter amount"
                  value={bondAmount}
                  onChange={(e) => setBondAmount(e.target.value)}
                  className="glass"
                />
              </div>

              <Button
                onClick={handleBond}
                disabled={!bondAmount || bondMutation.isPending}
                className="btn-primary w-full"
              >
                {bondMutation.isPending ? (
                  <>
                    <Loader2 className="w-4 h-4 mr-2 animate-spin" />
                    Bonding...
                  </>
                ) : (
                  <>
                    Bond Tokens
                    <ArrowRight className="w-4 h-4 ml-2" />
                  </>
                )}
              </Button>
            </CardContent>
          </Card>
        )}

        {currentStep === 'keys' && (
          <Card className="glass-card">
            <CardHeader>
              <CardTitle className="flex items-center gap-2">
                <Key className="w-5 h-5 text-purple-500" />
                Step 3: Set Session Keys
              </CardTitle>
            </CardHeader>
            <CardContent className="space-y-6">
              <p className="text-white/60">
                Generate session keys on your validator node and submit them here.
                Run this command on your node:
              </p>

              <div className="p-4 glass rounded-lg">
                <code className="text-sm text-cyan-400 break-all">
                  curl -H "Content-Type: application/json" -d '{"{\"id\":1, \"jsonrpc\":\"2.0\", \"method\": \"author_rotateKeys\", \"params\":[]}"}' http://localhost:9944
                </code>
                <Button
                  variant="ghost"
                  size="sm"
                  className="mt-2"
                  onClick={() => copyToClipboard('curl -H "Content-Type: application/json" -d \'{"id":1, "jsonrpc":"2.0", "method": "author_rotateKeys", "params":[]}\' http://localhost:9944')}
                >
                  <Copy className="w-4 h-4 mr-2" />
                  Copy Command
                </Button>
              </div>

              <div>
                <label className="text-sm text-white/60 mb-2 block">Session Keys (from rotateKeys result)</label>
                <Input
                  placeholder="0x..."
                  value={sessionKeys}
                  onChange={(e) => setSessionKeys(e.target.value)}
                  className="glass font-mono text-sm"
                />
              </div>

              <Button
                onClick={handleSetKeys}
                disabled={!sessionKeys || setKeysMutation.isPending}
                className="btn-primary w-full"
              >
                {setKeysMutation.isPending ? (
                  <>
                    <Loader2 className="w-4 h-4 mr-2 animate-spin" />
                    Setting Keys...
                  </>
                ) : (
                  <>
                    Set Session Keys
                    <ArrowRight className="w-4 h-4 ml-2" />
                  </>
                )}
              </Button>
            </CardContent>
          </Card>
        )}

        {currentStep === 'validate' && (
          <Card className="glass-card">
            <CardHeader>
              <CardTitle className="flex items-center gap-2">
                <Server className="w-5 h-5 text-green-500" />
                Step 4: Start Validating
              </CardTitle>
            </CardHeader>
            <CardContent className="space-y-6">
              <p className="text-white/60">
                Set your commission rate and declare your intention to validate.
                Commission is the percentage of rewards you keep from nominations.
              </p>

              <div>
                <label className="text-sm text-white/60 mb-2 block">Commission Rate (%)</label>
                <Input
                  type="number"
                  placeholder="10"
                  value={commission}
                  onChange={(e) => setCommission(e.target.value)}
                  className="glass"
                  min="0"
                  max="100"
                />
                <p className="text-xs text-white/40 mt-1">
                  Recommended: 5-15% for competitive nominations
                </p>
              </div>

              <Button
                onClick={handleValidate}
                disabled={validateMutation.isPending}
                className="btn-primary w-full"
              >
                {validateMutation.isPending ? (
                  <>
                    <Loader2 className="w-4 h-4 mr-2 animate-spin" />
                    Registering...
                  </>
                ) : (
                  <>
                    <Shield className="w-4 h-4 mr-2" />
                    Start Validating
                  </>
                )}
              </Button>
            </CardContent>
          </Card>
        )}

        {currentStep === 'complete' && (
          <Card className="glass-card border-green-500/30">
            <CardContent className="p-8 text-center">
              <div className="w-20 h-20 mx-auto mb-6 rounded-full bg-green-500 flex items-center justify-center">
                <CheckCircle2 className="w-10 h-10 text-white" />
              </div>
              <h2 className="text-2xl font-bold gradient-text mb-4">
                You're a Validator!
              </h2>
              <p className="text-white/60 mb-6">
                Congratulations! You've successfully registered as a validator on the ETRID network.
                Make sure your node is running and synced to start validating blocks.
              </p>
              <div className="p-4 glass rounded-lg text-left space-y-2">
                <div className="flex justify-between">
                  <span className="text-white/60">Address</span>
                  <span className="font-mono">{address && truncateAddress(address, 8, 6)}</span>
                </div>
                <div className="flex justify-between">
                  <span className="text-white/60">Status</span>
                  <Badge className="bg-green-500/20 text-green-400">Active Candidate</Badge>
                </div>
              </div>
            </CardContent>
          </Card>
        )}

        {/* Connected Address Display */}
        {address && currentStep !== 'connect' && (
          <Card className="glass-card">
            <CardContent className="p-4">
              <div className="flex items-center justify-between">
                <div className="flex items-center gap-3">
                  <div className="w-2 h-2 rounded-full bg-green-500 animate-pulse" />
                  <span className="text-white/60">Connected:</span>
                  <span className="font-mono">{truncateAddress(address)}</span>
                </div>
                <Button variant="ghost" size="sm" onClick={() => copyToClipboard(address)}>
                  <Copy className="w-4 h-4" />
                </Button>
              </div>
              {stakingInfo && BigInt(stakingInfo.active) > BigInt(0) && (
                <div className="mt-2 pt-2 border-t border-white/10">
                  <span className="text-white/60">Bonded: </span>
                  <span className="font-bold">{formatETR(stakingInfo.active)} ETR</span>
                </div>
              )}
            </CardContent>
          </Card>
        )}
      </div>
    </div>
  )
}
