"use client"

import { Dialog, DialogContent, DialogHeader, DialogTitle } from "@/components/ui/dialog"
import { Button } from "@/components/ui/button"
import { Check } from "lucide-react"

interface TokenSelectorProps {
  open: boolean
  onClose: () => void
  onSelect: (token: string) => void
  currentToken: string
}

const tokens = [
  { symbol: "ÉTR", name: "Ëtrid Token", balance: 1234.56 },
  { symbol: "EDSC", name: "Ëtrid Stablecoin", balance: 5678.9 },
]

export function TokenSelector({ open, onClose, onSelect, currentToken }: TokenSelectorProps) {
  return (
    <Dialog open={open} onOpenChange={onClose}>
      <DialogContent className="bg-card/95 backdrop-blur-xl border-border/50">
        <DialogHeader>
          <DialogTitle>Select a token</DialogTitle>
        </DialogHeader>

        <div className="space-y-2">
          {tokens.map((token) => (
            <Button
              key={token.symbol}
              variant="ghost"
              onClick={() => onSelect(token.symbol)}
              className="w-full justify-between h-auto p-4 hover:bg-accent/10"
            >
              <div className="flex items-center gap-3">
                <div className="w-10 h-10 rounded-full bg-gradient-to-br from-primary to-accent flex items-center justify-center">
                  <span className="font-bold">{token.symbol[0]}</span>
                </div>
                <div className="text-left">
                  <div className="font-semibold">{token.symbol}</div>
                  <div className="text-sm text-muted-foreground">{token.name}</div>
                </div>
              </div>

              <div className="flex items-center gap-3">
                <div className="text-right">
                  <div className="font-semibold">{token.balance.toLocaleString()}</div>
                </div>
                {currentToken === token.symbol && <Check className="w-5 h-5 text-accent" />}
              </div>
            </Button>
          ))}
        </div>
      </DialogContent>
    </Dialog>
  )
}
