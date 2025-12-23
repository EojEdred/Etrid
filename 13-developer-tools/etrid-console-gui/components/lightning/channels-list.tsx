"use client"

import { Card, CardContent, CardDescription, CardHeader, CardTitle } from "@/components/ui/card"
import { Badge } from "@/components/ui/badge"
import { ScrollArea } from "@/components/ui/scroll-area"
import { Circle } from "lucide-react"
import type { Channel } from "@/lib/lightning/types"
import { SUPPORTED_CHAINS } from "@/lib/lightning/types"

interface ChannelsListProps {
  channels: Channel[]
}

export function ChannelsList({ channels }: ChannelsListProps) {
  if (channels.length === 0) {
    return (
      <Card>
        <CardHeader>
          <CardTitle>Lightning Channels</CardTitle>
          <CardDescription>No channels yet</CardDescription>
        </CardHeader>
        <CardContent>
          <p className="text-sm text-muted-foreground">
            Open a channel to start using Lightning payments
          </p>
        </CardContent>
      </Card>
    )
  }

  return (
    <Card>
      <CardHeader>
        <CardTitle>Lightning Channels</CardTitle>
        <CardDescription>{channels.length} active channels</CardDescription>
      </CardHeader>
      <CardContent>
        <ScrollArea className="h-[400px]">
          <div className="space-y-3">
            {channels.map((channel) => {
              const chainInfo = SUPPORTED_CHAINS.find(c => c.id === channel.chain)
              const localPercent = (parseFloat(channel.localBalance) / parseFloat(channel.capacity)) * 100

              return (
                <div
                  key={channel.id}
                  className="p-3 border rounded-lg space-y-2"
                >
                  <div className="flex items-center justify-between">
                    <div className="font-medium text-sm">{chainInfo?.name}</div>
                    <Badge variant={channel.state === "active" ? "default" : "secondary"}>
                      <Circle className="mr-1 h-2 w-2 fill-current" />
                      {channel.state}
                    </Badge>
                  </div>

                  <div className="text-xs text-muted-foreground font-mono">
                    {channel.counterparty.slice(0, 10)}...{channel.counterparty.slice(-8)}
                  </div>

                  <div className="space-y-1">
                    <div className="flex justify-between text-xs">
                      <span className="text-muted-foreground">Local</span>
                      <span className="font-medium">
                        {(parseFloat(channel.localBalance) / 1e18).toFixed(4)} {chainInfo?.symbol}
                      </span>
                    </div>
                    <div className="h-2 bg-muted rounded-full overflow-hidden">
                      <div
                        className="h-full bg-primary"
                        style={{ width: `${localPercent}%` }}
                      />
                    </div>
                    <div className="flex justify-between text-xs">
                      <span className="font-medium">
                        {(parseFloat(channel.remoteBalance) / 1e18).toFixed(4)} {chainInfo?.symbol}
                      </span>
                      <span className="text-muted-foreground">Remote</span>
                    </div>
                  </div>
                </div>
              )
            })}
          </div>
        </ScrollArea>
      </CardContent>
    </Card>
  )
}
