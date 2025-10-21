"use client"

import Link from "next/link"
import { Wallet, LogOut, Copy } from "lucide-react"
import { Button } from "@/components/ui/button"
import { DropdownMenu, DropdownMenuContent, DropdownMenuItem, DropdownMenuTrigger } from "@/components/ui/dropdown-menu"
import type { UseWalletReturn } from "@/lib/polkadot/useWallet"

interface SwapHeaderProps {
  wallet: UseWalletReturn
}

export function SwapHeader({ wallet }: SwapHeaderProps) {
  const { isConnected, selectedAccount, connect, disconnect } = wallet

  const formatAddress = (address: string) => {
    if (!address) return ""
    return `${address.slice(0, 5)}...${address.slice(-4)}`
  }

  return (
    <header className="border-b border-border/50 bg-card/30 backdrop-blur-lg sticky top-0 z-50">
      <div className="container mx-auto px-4 py-4">
        <div className="flex items-center justify-between">
          <div className="flex items-center gap-8">
            <Link href="/" className="flex items-center gap-2">
              <div className="w-8 h-8 rounded-lg bg-gradient-to-br from-primary to-accent flex items-center justify-center">
                <span className="text-white font-bold text-lg">É</span>
              </div>
              <span className="text-xl font-bold">Ëtrid</span>
            </Link>

            <nav className="hidden md:flex items-center gap-6">
              <Link href="/" className="text-muted-foreground hover:text-foreground transition-colors">
                Home
              </Link>
              <Link href="/governance" className="text-muted-foreground hover:text-foreground transition-colors">
                Governance
              </Link>
              <Link href="/swap" className="text-foreground font-medium">
                Swap
              </Link>
            </nav>
          </div>

          {isConnected && selectedAccount ? (
            <DropdownMenu>
              <DropdownMenuTrigger asChild>
                <Button className="bg-accent hover:bg-accent/90 gap-2">
                  <Wallet className="w-4 h-4" />
                  {formatAddress(selectedAccount.address)}
                </Button>
              </DropdownMenuTrigger>
              <DropdownMenuContent align="end">
                <DropdownMenuItem onClick={() => navigator.clipboard.writeText(selectedAccount.address)}>
                  <Copy className="w-4 h-4 mr-2" />
                  Copy Address
                </DropdownMenuItem>
                <DropdownMenuItem onClick={disconnect}>
                  <LogOut className="w-4 h-4 mr-2" />
                  Disconnect
                </DropdownMenuItem>
              </DropdownMenuContent>
            </DropdownMenu>
          ) : (
            <Button onClick={connect} className="bg-accent hover:bg-accent/90 gap-2">
              <Wallet className="w-4 h-4" />
              Connect Wallet
            </Button>
          )}
        </div>
      </div>
    </header>
  )
}
