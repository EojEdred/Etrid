"use client"

import Link from "next/link"
import { Button } from "@/components/ui/button"
import { DropdownMenu, DropdownMenuContent, DropdownMenuItem, DropdownMenuTrigger } from "@/components/ui/dropdown-menu"
import { Wallet, LogOut, Copy } from "lucide-react"

interface GovHeaderProps {
  isConnected: boolean
  walletAddress: string
  onConnect: () => void
  onDisconnect: () => void
}

export default function GovHeader({ isConnected, walletAddress, onConnect, onDisconnect }: GovHeaderProps) {
  return (
    <header className="border-b border-border bg-card/50 backdrop-blur-sm sticky top-0 z-50">
      <div className="container mx-auto px-4 py-4 flex items-center justify-between">
        <Link href="/" className="flex items-center gap-2">
          <div className="w-8 h-8 rounded-full bg-gradient-to-br from-primary to-accent" />
          <span className="text-xl font-bold">Ëtrid</span>
        </Link>

        <h1 className="text-2xl font-bold hidden md:block">Consensus Day 2026</h1>

        {isConnected ? (
          <DropdownMenu>
            <DropdownMenuTrigger asChild>
              <Button variant="outline" className="gap-2 bg-transparent">
                <Wallet className="w-4 h-4" />
                {walletAddress}
              </Button>
            </DropdownMenuTrigger>
            <DropdownMenuContent align="end">
              <DropdownMenuItem onClick={() => navigator.clipboard.writeText(walletAddress)}>
                <Copy className="w-4 h-4 mr-2" />
                Copy Address
              </DropdownMenuItem>
              <DropdownMenuItem onClick={onDisconnect}>
                <LogOut className="w-4 h-4 mr-2" />
                Disconnect
              </DropdownMenuItem>
            </DropdownMenuContent>
          </DropdownMenu>
        ) : (
          <Button onClick={onConnect} className="gap-2">
            <Wallet className="w-4 h-4" />
            Connect Wallet
          </Button>
        )}
      </div>
    </header>
  )
}
