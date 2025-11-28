"use client"

import { Check, X, Clock, XCircle } from "lucide-react"
import { SignerWithStatus } from "@/lib/types/multisig"
import { Avatar, AvatarFallback } from "@/components/ui/avatar"
import { Badge } from "@/components/ui/badge"
import { Button } from "@/components/ui/button"
import { Tooltip, TooltipContent, TooltipProvider, TooltipTrigger } from "@/components/ui/tooltip"

interface SignerAvatarProps {
  signer: SignerWithStatus
  onRemove?: () => void
  showRemove?: boolean
}

export function SignerAvatar({ signer, onRemove, showRemove = false }: SignerAvatarProps) {
  const statusConfig = {
    pending: {
      icon: Clock,
      color: "text-yellow-500",
      bg: "bg-yellow-500/10",
      label: "Pending",
    },
    approved: {
      icon: Check,
      color: "text-green-500",
      bg: "bg-green-500/10",
      label: "Approved",
    },
    rejected: {
      icon: X,
      color: "text-red-500",
      bg: "bg-red-500/10",
      label: "Rejected",
    },
  }

  const config = statusConfig[signer.status]
  const StatusIcon = config.icon

  return (
    <div className="flex items-center justify-between p-4 rounded-lg border bg-card hover:bg-accent/5 transition-colors">
      <div className="flex items-center gap-3">
        {/* Avatar */}
        <div className="relative">
          <Avatar className="w-12 h-12">
            <AvatarFallback className="text-sm font-semibold">
              {signer.username
                ? signer.username.slice(0, 2).toUpperCase()
                : signer.address.slice(0, 2).toUpperCase()}
            </AvatarFallback>
          </Avatar>
          {/* Status Badge */}
          <div
            className={`absolute -bottom-1 -right-1 w-6 h-6 rounded-full ${config.bg} ${config.color} flex items-center justify-center border-2 border-background`}
          >
            <StatusIcon className="w-3 h-3" />
          </div>
        </div>

        {/* Info */}
        <div>
          <div className="flex items-center gap-2">
            <p className="font-semibold">
              {signer.username || `${signer.address.slice(0, 6)}...${signer.address.slice(-4)}`}
            </p>
            {signer.isOwner && (
              <Badge variant="outline" className="text-xs">
                Owner
              </Badge>
            )}
          </div>
          <p className="text-xs text-muted-foreground">
            {signer.username && (
              <>
                {signer.address.slice(0, 6)}...{signer.address.slice(-4)} •{" "}
              </>
            )}
            Added {new Date(signer.addedAt).toLocaleDateString()}
          </p>
        </div>
      </div>

      {/* Actions */}
      <div className="flex items-center gap-2">
        <Badge className={`${config.bg} ${config.color} border-0`}>
          {config.label}
        </Badge>
        {showRemove && !signer.isOwner && (
          <TooltipProvider>
            <Tooltip>
              <TooltipTrigger asChild>
                <Button
                  variant="ghost"
                  size="icon"
                  className="h-8 w-8 text-destructive hover:text-destructive hover:bg-destructive/10"
                  onClick={onRemove}
                >
                  <XCircle className="w-4 h-4" />
                </Button>
              </TooltipTrigger>
              <TooltipContent>
                <p>Remove signer</p>
              </TooltipContent>
            </Tooltip>
          </TooltipProvider>
        )}
      </div>
    </div>
  )
}
