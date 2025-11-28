"use client"

import { Loader2, CheckCircle2, XCircle, Users, Clock } from "lucide-react"
import { MixingStatus } from "@/lib/types/privacy"
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card"
import { Progress } from "@/components/ui/progress"
import { Badge } from "@/components/ui/badge"

interface MixingProgressProps {
  status: MixingStatus
}

export function MixingProgress({ status }: MixingProgressProps) {
  const statusConfig = {
    waiting: {
      icon: Clock,
      color: "text-yellow-500",
      bg: "bg-yellow-500/10",
      label: "Waiting for participants",
    },
    mixing: {
      icon: Loader2,
      color: "text-blue-500",
      bg: "bg-blue-500/10",
      label: "Mixing in progress",
    },
    complete: {
      icon: CheckCircle2,
      color: "text-green-500",
      bg: "bg-green-500/10",
      label: "Complete",
    },
    error: {
      icon: XCircle,
      color: "text-red-500",
      bg: "bg-red-500/10",
      label: "Error",
    },
  }

  const config = statusConfig[status.status]
  const StatusIcon = config.icon

  const formatTime = (seconds: number) => {
    if (seconds < 60) return `${seconds}s`
    const minutes = Math.floor(seconds / 60)
    const remainingSeconds = seconds % 60
    return `${minutes}m ${remainingSeconds}s`
  }

  return (
    <Card>
      <CardHeader>
        <CardTitle className="flex items-center gap-2">
          <StatusIcon
            className={`w-5 h-5 ${config.color} ${status.status === "mixing" ? "animate-spin" : ""}`}
          />
          Mixing Session
        </CardTitle>
      </CardHeader>
      <CardContent className="space-y-6">
        {/* Status Badge */}
        <div className="flex justify-center">
          <Badge className={`${config.bg} ${config.color} border-0 px-4 py-2`}>
            {config.label}
          </Badge>
        </div>

        {/* Round Progress */}
        <div>
          <div className="flex items-center justify-between mb-2">
            <p className="text-sm font-semibold">Round Progress</p>
            <p className="text-sm font-bold">
              Round {status.currentRound} of {status.totalRounds}
            </p>
          </div>
          <Progress value={status.progress} className="h-3" />
          <p className="text-xs text-muted-foreground mt-1 text-center">
            {status.progress.toFixed(0)}% complete
          </p>
        </div>

        {/* Round Visualization */}
        <div className="flex justify-center gap-2">
          {Array.from({ length: status.totalRounds }).map((_, i) => {
            const roundNumber = i + 1
            const isComplete = roundNumber < status.currentRound
            const isCurrent = roundNumber === status.currentRound
            const isPending = roundNumber > status.currentRound

            return (
              <div
                key={i}
                className={`w-10 h-10 rounded-full flex items-center justify-center text-xs font-bold transition-all ${
                  isComplete
                    ? "bg-green-500 text-white"
                    : isCurrent
                      ? "bg-blue-500 text-white animate-pulse"
                      : "bg-muted text-muted-foreground"
                }`}
              >
                {isComplete ? <CheckCircle2 className="w-5 h-5" /> : roundNumber}
              </div>
            )
          })}
        </div>

        {/* Participants */}
        <div className="flex items-center justify-between p-4 rounded-lg bg-muted/50">
          <div className="flex items-center gap-2">
            <Users className="w-5 h-5 text-muted-foreground" />
            <div>
              <p className="text-sm font-semibold">Participants</p>
              <p className="text-xs text-muted-foreground">
                In current round
              </p>
            </div>
          </div>
          <p className="text-2xl font-bold">{status.participantsInRound}</p>
        </div>

        {/* Time Remaining */}
        {status.status !== "complete" && status.estimatedTimeRemaining > 0 && (
          <div className="flex items-center justify-between p-4 rounded-lg bg-muted/50">
            <div className="flex items-center gap-2">
              <Clock className="w-5 h-5 text-muted-foreground" />
              <div>
                <p className="text-sm font-semibold">Time Remaining</p>
                <p className="text-xs text-muted-foreground">
                  Estimated completion
                </p>
              </div>
            </div>
            <p className="text-2xl font-bold">
              {formatTime(status.estimatedTimeRemaining)}
            </p>
          </div>
        )}

        {/* Status Message */}
        {status.message && (
          <div className={`p-3 rounded-lg ${config.bg}`}>
            <p className={`text-sm ${config.color}`}>{status.message}</p>
          </div>
        )}

        {/* Privacy Explanation */}
        {status.status === "mixing" && (
          <div className="p-3 rounded-lg bg-accent/10">
            <p className="text-xs text-muted-foreground">
              <strong className="text-foreground">How mixing works:</strong>{" "}
              Your coins are being combined with {status.participantsInRound - 1}{" "}
              other participant{status.participantsInRound > 2 ? "s" : ""} to break
              the transaction trail. More rounds = higher privacy.
            </p>
          </div>
        )}

        {/* Completion Message */}
        {status.status === "complete" && (
          <div className="p-4 rounded-lg bg-green-500/10 border border-green-500/50">
            <div className="flex items-center gap-2 mb-2">
              <CheckCircle2 className="w-5 h-5 text-green-500" />
              <p className="font-semibold text-green-500">Mixing Complete!</p>
            </div>
            <p className="text-xs text-muted-foreground">
              Your coins have been successfully mixed through {status.totalRounds}{" "}
              round{status.totalRounds > 1 ? "s" : ""}. Your privacy score has
              increased significantly.
            </p>
          </div>
        )}
      </CardContent>
    </Card>
  )
}
