"use client"

import Link from "next/link"
import { Zap, Home } from "lucide-react"
import { Button } from "@/components/ui/button"

interface LightningHeaderProps {
  wallet: any
  lightning: any
}

export function LightningHeader({ wallet, lightning }: LightningHeaderProps) {
  return (
    <header className="border-b">
      <div className="container mx-auto px-4 py-4">
        <div className="flex items-center justify-between">
          <div className="flex items-center gap-6">
            <Link href="/" className="flex items-center gap-2">
              <Zap className="h-6 w-6 text-yellow-500" />
              <span className="text-xl font-bold">Ëtrid Lightning</span>
            </Link>
            <nav className="hidden md:flex gap-4">
              <Link href="/lightning" className="text-sm font-medium hover:underline">
                Payments
              </Link>
              <Link href="/lightning/channels" className="text-sm font-medium hover:underline">
                Channels
              </Link>
              <Link href="/lightning/network" className="text-sm font-medium hover:underline">
                Network
              </Link>
            </nav>
          </div>

          <div className="flex items-center gap-3">
            {wallet.account ? (
              <div className="flex items-center gap-3">
                <div className="text-sm">
                  <div className="text-muted-foreground">Connected</div>
                  <div className="font-mono">
                    {wallet.account.address.slice(0, 6)}...{wallet.account.address.slice(-4)}
                  </div>
                </div>
                <Button
                  variant="outline"
                  size="sm"
                  onClick={wallet.disconnect}
                >
                  Disconnect
                </Button>
              </div>
            ) : (
              <Button onClick={wallet.connect}>
                Connect Wallet
              </Button>
            )}

            <Link href="/">
              <Button variant="ghost" size="icon">
                <Home className="h-5 w-5" />
              </Button>
            </Link>
          </div>
        </div>
      </div>
    </header>
  )
}
