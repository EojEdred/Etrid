"use client"

import { Plus, Shield, AlertCircle } from "lucide-react"
import { useMultiSig } from "@/hooks/useMultiSig"
import { MultiSigCard } from "@/components/multisig/MultiSigCard"
import { Button } from "@/components/ui/button"
import { Card, CardContent } from "@/components/ui/card"
import { Skeleton } from "@/components/ui/skeleton"
import { useRouter } from "next/navigation"

export default function MultiSigScreen() {
  const { wallets, stats, loading } = useMultiSig()
  const router = useRouter()

  if (loading && wallets.length === 0) {
    return (
      <div className="container mx-auto p-6 max-w-7xl">
        <div className="flex items-center justify-between mb-6">
          <Skeleton className="h-10 w-48" />
          <Skeleton className="h-10 w-40" />
        </div>
        <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
          {[1, 2, 3].map((i) => (
            <Skeleton key={i} className="h-64" />
          ))}
        </div>
      </div>
    )
  }

  return (
    <div className="container mx-auto p-6 max-w-7xl">
      {/* Header */}
      <div className="flex items-center justify-between mb-8">
        <div>
          <h1 className="text-3xl font-bold flex items-center gap-2">
            <Shield className="w-8 h-8" />
            Multi-Signature Wallets
          </h1>
          <p className="text-muted-foreground mt-1">
            Manage shared wallets with multiple signers
          </p>
        </div>
        <Button onClick={() => router.push("/multisig/create")} size="lg">
          <Plus className="w-4 h-4 mr-2" />
          Create Wallet
        </Button>
      </div>

      {/* Stats */}
      {stats && (
        <div className="grid grid-cols-1 md:grid-cols-4 gap-4 mb-8">
          <Card>
            <CardContent className="p-6">
              <p className="text-sm text-muted-foreground mb-1">Total Wallets</p>
              <p className="text-3xl font-bold">{stats.totalWallets}</p>
            </CardContent>
          </Card>
          <Card>
            <CardContent className="p-6">
              <p className="text-sm text-muted-foreground mb-1">Total Balance</p>
              <p className="text-3xl font-bold">
                {parseFloat(stats.totalBalance).toLocaleString()} ÉTR
              </p>
            </CardContent>
          </Card>
          <Card>
            <CardContent className="p-6">
              <p className="text-sm text-muted-foreground mb-1">Pending Approvals</p>
              <p className="text-3xl font-bold text-yellow-500">
                {stats.pendingApprovals}
              </p>
            </CardContent>
          </Card>
          <Card>
            <CardContent className="p-6">
              <p className="text-sm text-muted-foreground mb-1">Need Your Action</p>
              <p className="text-3xl font-bold text-red-500">
                {stats.walletsNeedingAction}
              </p>
            </CardContent>
          </Card>
        </div>
      )}

      {/* Wallets Grid */}
      {wallets.length > 0 ? (
        <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
          {wallets.map((wallet) => (
            <MultiSigCard
              key={wallet.id}
              wallet={wallet}
              onClick={() => router.push(`/multisig/${wallet.id}`)}
            />
          ))}
        </div>
      ) : (
        <Card className="border-dashed">
          <CardContent className="p-12 text-center">
            <Shield className="w-16 h-16 mx-auto mb-4 text-muted-foreground" />
            <h3 className="text-xl font-semibold mb-2">No Multi-Sig Wallets</h3>
            <p className="text-muted-foreground mb-6">
              Create your first multi-signature wallet for enhanced security and
              shared control
            </p>
            <Button onClick={() => router.push("/multisig/create")}>
              <Plus className="w-4 h-4 mr-2" />
              Create Your First Wallet
            </Button>
          </CardContent>
        </Card>
      )}

      {/* Info Card */}
      <Card className="mt-8 bg-accent/5 border-accent/20">
        <CardContent className="p-6">
          <div className="flex items-start gap-3">
            <AlertCircle className="w-5 h-5 text-accent mt-0.5" />
            <div>
              <h4 className="font-semibold mb-2">About Multi-Sig Wallets</h4>
              <p className="text-sm text-muted-foreground">
                Multi-signature wallets require multiple approvals before
                executing transactions. Perfect for couples (2-of-2), businesses
                (3-of-5), or DAOs (5-of-9). Enhance security and enable shared
                control over funds.
              </p>
            </div>
          </div>
        </CardContent>
      </Card>
    </div>
  )
}
