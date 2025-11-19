"use client"

import { Check, X, Clock } from "lucide-react"
import { SignerWithStatus } from "@/lib/types/multisig"
import { Progress } from "@/components/ui/progress"
import { Avatar, AvatarFallback } from "@/components/ui/avatar"
import { Tooltip, TooltipContent, TooltipProvider, TooltipTrigger } from "@/components/ui/tooltip"

interface SignatureProgressProps {
  signers: SignerWithStatus[]
  threshold: number
  collected: number
}

export function SignatureProgress({ signers, threshold, collected }: SignatureProgressProps) {
  const progress = (collected / threshold) * 100
  const isComplete = collected >= threshold

  return (
    <div className="space-y-4">
      {/* Progress Bar */}
      <div>
        <div className="flex items-center justify-between mb-2">
          <h4 className="text-sm font-semibold">Signature Progress</h4>
          <p className="text-sm font-bold">
            {collected} / {threshold}
          </p>
        </div>
        <Progress
          value={progress}
          className={`h-3 ${isComplete ? "bg-green-500/20" : ""}`}
        />
        <p className="text-xs text-muted-foreground mt-1">
          {isComplete
            ? "Threshold reached! Ready to execute"
            : `${threshold - collected} more signature${threshold - collected > 1 ? "s" : ""} needed`}
        </p>
      </div>

      {/* Threshold Indicator */}
      <div className="flex items-center gap-2 p-3 rounded-lg bg-muted/50">
        <div className="w-8 h-8 rounded-full bg-accent/20 flex items-center justify-center">
          <span className="text-sm font-bold">{threshold}</span>
        </div>
        <div>
          <p className="text-sm font-semibold">Threshold Required</p>
          <p className="text-xs text-muted-foreground">
            {threshold} out of {signers.length} signers must approve
          </p>
        </div>
      </div>

      {/* Signer Status Grid */}
      <div>
        <h5 className="text-sm font-semibold mb-3">Signer Status</h5>
        <div className="grid grid-cols-2 sm:grid-cols-3 md:grid-cols-4 gap-3">
          {signers.map((signer, i) => {
            const statusConfig = {
              pending: {
                icon: Clock,
                color: "text-muted-foreground",
                bg: "bg-muted",
              },
              approved: {
                icon: Check,
                color: "text-green-500",
                bg: "bg-green-500/20 border-green-500",
              },
              rejected: {
                icon: X,
                color: "text-red-500",
                bg: "bg-red-500/20 border-red-500",
              },
            }

            const config = statusConfig[signer.status]
            const StatusIcon = config.icon

            return (
              <TooltipProvider key={i}>
                <Tooltip>
                  <TooltipTrigger asChild>
                    <div
                      className={`relative p-3 rounded-lg border-2 ${config.bg} transition-all hover:scale-105 cursor-pointer`}
                    >
                      <Avatar className="w-12 h-12 mx-auto mb-2">
                        <AvatarFallback className="text-xs">
                          {signer.username
                            ? signer.username.slice(0, 2).toUpperCase()
                            : signer.address.slice(0, 2).toUpperCase()}
                        </AvatarFallback>
                      </Avatar>
                      <p className="text-xs text-center font-medium truncate">
                        {signer.username || `${signer.address.slice(0, 4)}...`}
                      </p>
                      <div
                        className={`absolute top-1 right-1 w-6 h-6 rounded-full bg-background flex items-center justify-center ${config.color}`}
                      >
                        <StatusIcon className="w-3 h-3" />
                      </div>
                    </div>
                  </TooltipTrigger>
                  <TooltipContent>
                    <div>
                      <p className="font-semibold">
                        {signer.username || signer.address}
                      </p>
                      <p className="text-xs text-muted-foreground">
                        Status: {signer.status}
                      </p>
                    </div>
                  </TooltipContent>
                </Tooltip>
              </TooltipProvider>
            )
          })}
        </div>
      </div>

      {/* Completion Message */}
      {isComplete && (
        <div className="flex items-center gap-2 p-4 rounded-lg bg-green-500/10 border border-green-500/50">
          <Check className="w-5 h-5 text-green-500" />
          <div>
            <p className="text-sm font-semibold text-green-500">
              Threshold Reached
            </p>
            <p className="text-xs text-muted-foreground">
              This transaction can now be executed
            </p>
          </div>
        </div>
      )}
    </div>
  )
}
