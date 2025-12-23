'use client'

import { useMemo, useState } from 'react'
import { ConnectButton } from '@rainbow-me/rainbowkit'
import { useAccount, useBalance, useChainId, useDisconnect, useSendTransaction, useSignMessage, useSwitchChain, useWaitForTransactionReceipt } from 'wagmi'
import { formatUnits, parseEther } from 'viem'
import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card'
import { Tabs, TabsContent, TabsList, TabsTrigger } from '@/components/ui/tabs'
import { Button } from '@/components/ui/button'
import { Input } from '@/components/ui/input'
import { Label } from '@/components/ui/label'
import { useToast } from '@/hooks/use-toast'
import { Loader2 } from 'lucide-react'

export function EvmWallet() {
  const { toast } = useToast()
  const { address, isConnected } = useAccount()
  const chainId = useChainId()
  const { disconnect } = useDisconnect()
  const { chains, mutate: switchChain, isPending: isSwitching, error: switchError } = useSwitchChain()

  const { data: balance } = useBalance({
    address,
    query: { enabled: Boolean(address) },
  })

  const [to, setTo] = useState('')
  const [amount, setAmount] = useState('')
  const [message, setMessage] = useState('Hello from ËTRID')

  const {
    data: sendData,
    mutate: sendTx,
    isPending: isSending,
    error: sendError,
    reset: resetSend,
  } = useSendTransaction()

  const {
    data: receipt,
    isLoading: isConfirming,
    isSuccess: isConfirmed,
  } = useWaitForTransactionReceipt({
    hash: sendData,
    query: { enabled: Boolean(sendData) },
  })

  const {
    data: signature,
    mutate: sign,
    isPending: isSigning,
    error: signError,
    reset: resetSign,
  } = useSignMessage()

  const activeChain = useMemo(() => chains.find((c) => c.id === chainId) ?? null, [chains, chainId])

  const balanceText = useMemo(() => {
    if (!balance) return null
    const raw = formatUnits(balance.value, balance.decimals)
    const [whole, fraction = ''] = raw.split('.')
    const trimmedFraction = fraction.slice(0, 6).replace(/0+$/, '')
    const display = trimmedFraction ? `${whole}.${trimmedFraction}` : whole
    return `${display} ${balance.symbol}`
  }, [balance])

  const handleSend = () => {
    resetSend()
    const toAddress = to.trim()
    const value = Number(amount)
    if (!toAddress) {
      toast({ title: 'Missing recipient', description: 'Enter a recipient address.', variant: 'destructive' })
      return
    }
    if (!Number.isFinite(value) || value <= 0) {
      toast({ title: 'Invalid amount', description: 'Enter a positive amount.', variant: 'destructive' })
      return
    }

    try {
      sendTx({ to: toAddress as `0x${string}`, value: parseEther(amount as `${number}`) })
    } catch (e) {
      toast({ title: 'Send failed', description: e instanceof Error ? e.message : 'Unknown error', variant: 'destructive' })
    }
  }

  const handleSign = () => {
    resetSign()
    const msg = message.trim()
    if (!msg) {
      toast({ title: 'Missing message', description: 'Enter a message to sign.', variant: 'destructive' })
      return
    }
    sign({ message: msg })
  }

  return (
    <div className="space-y-6">
      <Card className="glass-card">
        <CardHeader className="flex flex-row items-center justify-between gap-4">
          <CardTitle>EVM Wallet</CardTitle>
          <ConnectButton />
        </CardHeader>
        <CardContent className="space-y-3 text-white/80">
          <div className="flex flex-wrap items-center gap-x-6 gap-y-2">
            <div>
              <div className="text-xs text-white/60">Account</div>
              <div className="font-mono text-sm">{address ?? '—'}</div>
            </div>
            <div>
              <div className="text-xs text-white/60">Network</div>
              <div className="font-mono text-sm">
                {activeChain ? `${activeChain.name} (${activeChain.id})` : chainId ? String(chainId) : '—'}
              </div>
            </div>
            <div>
              <div className="text-xs text-white/60">Balance</div>
              <div className="font-mono text-sm">
                {balanceText ?? '—'}
              </div>
            </div>
          </div>

          {isConnected && chains.length > 1 ? (
            <div className="pt-2">
              <div className="text-xs text-white/60 mb-2">Switch network</div>
              <div className="flex flex-wrap gap-2">
                {chains.map((c) => (
                  <Button
                    key={c.id}
                    variant={c.id === chainId ? 'default' : 'outline'}
                    onClick={() => switchChain({ chainId: c.id })}
                    disabled={isSwitching || c.id === chainId}
                  >
                    {isSwitching && c.id === chainId ? (
                      <>
                        <Loader2 className="mr-2 h-4 w-4 animate-spin" />
                        Switching...
                      </>
                    ) : (
                      c.name
                    )}
                  </Button>
                ))}
              </div>
              {switchError ? (
                <div className="text-sm text-red-400 mt-2">{switchError.message}</div>
              ) : null}
            </div>
          ) : null}

          {isConnected ? (
            <div className="pt-2">
              <Button variant="outline" onClick={() => disconnect()}>
                Disconnect
              </Button>
            </div>
          ) : null}
        </CardContent>
      </Card>

      <Tabs defaultValue="send" className="space-y-4">
        <TabsList className="grid w-full grid-cols-3 glass p-1">
          <TabsTrigger value="send" className="rounded-lg data-[state=active]:bg-white/10">Send</TabsTrigger>
          <TabsTrigger value="sign" className="rounded-lg data-[state=active]:bg-white/10">Sign</TabsTrigger>
          <TabsTrigger value="activity" className="rounded-lg data-[state=active]:bg-white/10">Activity</TabsTrigger>
        </TabsList>

        <TabsContent value="send">
          <Card className="glass-card">
            <CardHeader>
              <CardTitle>Send Native Token</CardTitle>
            </CardHeader>
            <CardContent className="space-y-4">
              <div className="space-y-2">
                <Label htmlFor="evm-to">To</Label>
                <Input
                  id="evm-to"
                  value={to}
                  onChange={(e) => setTo(e.target.value)}
                  placeholder="0x..."
                  className="font-mono"
                  disabled={!isConnected || isSending}
                />
              </div>
              <div className="space-y-2">
                <Label htmlFor="evm-amount">Amount</Label>
                <Input
                  id="evm-amount"
                  value={amount}
                  onChange={(e) => setAmount(e.target.value)}
                  placeholder="0.01"
                  className="font-mono"
                  disabled={!isConnected || isSending}
                />
              </div>
              <Button className="btn-primary" onClick={handleSend} disabled={!isConnected || isSending}>
                {isSending ? (
                  <>
                    <Loader2 className="mr-2 h-4 w-4 animate-spin" />
                    Sending...
                  </>
                ) : (
                  'Send'
                )}
              </Button>

              {sendError ? <div className="text-sm text-red-400">{sendError.message}</div> : null}
              {sendData ? (
                <div className="text-sm text-white/70">
                  Tx hash: <span className="font-mono break-all">{sendData}</span>
                </div>
              ) : null}
              {isConfirming ? (
                <div className="text-sm text-white/60">Confirming...</div>
              ) : null}
              {isConfirmed ? (
                <div className="text-sm text-green-400">
                  Confirmed in block{' '}
                  <span className="font-mono">{receipt?.blockNumber?.toString?.() ?? '—'}</span>
                </div>
              ) : null}
            </CardContent>
          </Card>
        </TabsContent>

        <TabsContent value="sign">
          <Card className="glass-card">
            <CardHeader>
              <CardTitle>Sign Message</CardTitle>
            </CardHeader>
            <CardContent className="space-y-4">
              <div className="space-y-2">
                <Label htmlFor="evm-message">Message</Label>
                <Input
                  id="evm-message"
                  value={message}
                  onChange={(e) => setMessage(e.target.value)}
                  placeholder="Message to sign"
                  disabled={!isConnected || isSigning}
                />
              </div>
              <Button className="btn-primary" onClick={handleSign} disabled={!isConnected || isSigning}>
                {isSigning ? (
                  <>
                    <Loader2 className="mr-2 h-4 w-4 animate-spin" />
                    Signing...
                  </>
                ) : (
                  'Sign'
                )}
              </Button>

              {signError ? <div className="text-sm text-red-400">{signError.message}</div> : null}
              {signature ? (
                <div className="text-sm text-white/70">
                  Signature: <span className="font-mono break-all">{signature}</span>
                </div>
              ) : null}
            </CardContent>
          </Card>
        </TabsContent>

        <TabsContent value="activity">
          <Card className="glass-card">
            <CardHeader>
              <CardTitle>Activity</CardTitle>
            </CardHeader>
            <CardContent className="text-white/70 text-sm space-y-2">
              {sendData ? (
                <>
                  <div>
                    Last tx hash: <span className="font-mono break-all">{sendData}</span>
                  </div>
                  <div className="text-white/60">
                    Status: {isConfirming ? 'confirming' : isConfirmed ? 'confirmed' : 'submitted'}
                  </div>
                </>
              ) : (
                <div className="text-white/60">No recent activity.</div>
              )}
            </CardContent>
          </Card>
        </TabsContent>
      </Tabs>
    </div>
  )
}
