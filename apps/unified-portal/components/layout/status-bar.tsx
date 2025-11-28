"use client"

import { Badge } from "@/components/ui/badge"
import { Activity, Users, Box, Zap } from "lucide-react"

export default function StatusBar() {
  return (
    <footer className="border-t border-zinc-200 bg-zinc-50 dark:border-zinc-800 dark:bg-zinc-900">
      <div className="container mx-auto flex h-10 items-center justify-between px-6 text-xs">
        <div className="flex items-center gap-6">
          <div className="flex items-center gap-2">
            <Activity className="h-3 w-3 text-green-500" />
            <span className="text-zinc-600 dark:text-zinc-400">Network:</span>
            <Badge variant="success" className="text-xs">
              Healthy
            </Badge>
          </div>

          <div className="flex items-center gap-2">
            <Box className="h-3 w-3 text-blue-500" />
            <span className="text-zinc-600 dark:text-zinc-400">Block:</span>
            <span className="font-mono text-zinc-900 dark:text-zinc-100">#12,450</span>
          </div>

          <div className="flex items-center gap-2">
            <Users className="h-3 w-3 text-purple-500" />
            <span className="text-zinc-600 dark:text-zinc-400">Peers:</span>
            <span className="font-mono text-zinc-900 dark:text-zinc-100">21</span>
          </div>

          <div className="flex items-center gap-2">
            <Zap className="h-3 w-3 text-yellow-500" />
            <span className="text-zinc-600 dark:text-zinc-400">TPS:</span>
            <span className="font-mono text-zinc-900 dark:text-zinc-100">1,250</span>
          </div>
        </div>

        <div className="text-zinc-500 dark:text-zinc-500">
          Etrid Protocol v1.0.0
        </div>
      </div>
    </footer>
  )
}
