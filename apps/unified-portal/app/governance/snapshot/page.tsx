"use client"

import { useEffect, useRef, useState } from "react"
import { Card, CardContent } from "@/components/ui/card"
import { Alert, AlertDescription } from "@/components/ui/alert"
import { Button } from "@/components/ui/button"
import { Badge } from "@/components/ui/badge"
import { ExternalLink, AlertCircle, CheckCircle2, Loader2 } from "lucide-react"

export default function SnapshotEmbedPage() {
  const iframeRef = useRef<HTMLIFrameElement>(null)
  const [isLoading, setIsLoading] = useState(true)
  const [connectionStatus, setConnectionStatus] = useState<"connecting" | "connected" | "error">("connecting")
  const [snapshotEvents, setSnapshotEvents] = useState<Array<{ type: string; timestamp: number }>>([])

  // Mock wallet state (in production, this would come from Polkadot.js wallet)
  const [wallet] = useState({
    connected: true,
    address: "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY",
    balance: "1250.5000"
  })

  // Send wallet data to Vue app via postMessage
  useEffect(() => {
    const sendWalletData = () => {
      if (iframeRef.current?.contentWindow && wallet.connected) {
        try {
          iframeRef.current.contentWindow.postMessage(
            {
              type: "WALLET_CONNECTED",
              address: wallet.address,
              balance: wallet.balance,
              source: "etrid-portal",
              timestamp: Date.now()
            },
            "*" // In production, specify exact origin: "http://localhost:8082"
          )
          console.log("[Portal → Snapshot] Sent wallet data:", wallet.address)
        } catch (error) {
          console.error("[Portal → Snapshot] Failed to send wallet data:", error)
        }
      }
    }

    // Send wallet data when iframe loads
    const timer = setTimeout(sendWalletData, 1000)

    // Send wallet data every 5 seconds to maintain sync
    const interval = setInterval(sendWalletData, 5000)

    return () => {
      clearTimeout(timer)
      clearInterval(interval)
    }
  }, [wallet])

  // Listen for events from Vue app
  useEffect(() => {
    const handleMessage = (event: MessageEvent) => {
      // In production, verify event.origin === "http://localhost:8082"
      if (event.data && event.data.source === "snapshot-ui") {
        console.log("[Snapshot → Portal] Received event:", event.data)

        setSnapshotEvents((prev) => [
          ...prev.slice(-9), // Keep last 10 events
          { type: event.data.type, timestamp: Date.now() }
        ])

        switch (event.data.type) {
          case "READY":
            setConnectionStatus("connected")
            setIsLoading(false)
            break

          case "VOTE_SUBMITTED":
            console.log("Vote submitted:", event.data.proposalId, event.data.choice)
            // Handle vote submission (e.g., show notification)
            break

          case "PROPOSAL_VIEWED":
            console.log("Proposal viewed:", event.data.proposalId)
            break

          case "ERROR":
            console.error("Snapshot error:", event.data.error)
            setConnectionStatus("error")
            break

          default:
            console.log("Unknown event type:", event.data.type)
        }
      }
    }

    window.addEventListener("message", handleMessage)
    return () => window.removeEventListener("message", handleMessage)
  }, [])

  // Iframe load handler
  const handleIframeLoad = () => {
    console.log("[Portal] Snapshot iframe loaded")
    // Wait a bit for Vue app to initialize before marking as connected
    setTimeout(() => {
      setIsLoading(false)
      setConnectionStatus("connected")
    }, 2000)
  }

  return (
    <div className="container mx-auto p-8">
      <div className="mb-6">
        <div className="flex items-center justify-between">
          <div>
            <h1 className="text-3xl font-bold mb-2">Snapshot Governance</h1>
            <p className="text-muted-foreground">Vote on active proposals using Snapshot</p>
          </div>

          <div className="flex items-center gap-3">
            <Badge
              variant="outline"
              className={
                connectionStatus === "connected"
                  ? "bg-green-500/10 text-green-500 border-green-500/20"
                  : connectionStatus === "error"
                    ? "bg-red-500/10 text-red-500 border-red-500/20"
                    : "bg-amber-500/10 text-amber-500 border-amber-500/20"
              }
            >
              {connectionStatus === "connected" && (
                <>
                  <CheckCircle2 className="w-3 h-3 mr-1" />
                  Connected
                </>
              )}
              {connectionStatus === "connecting" && (
                <>
                  <Loader2 className="w-3 h-3 mr-1 animate-spin" />
                  Connecting
                </>
              )}
              {connectionStatus === "error" && (
                <>
                  <AlertCircle className="w-3 h-3 mr-1" />
                  Error
                </>
              )}
            </Badge>

            <a href="http://localhost:8082" target="_blank" rel="noopener noreferrer">
              <Button variant="outline" size="sm">
                Open in New Tab
                <ExternalLink className="w-4 h-4 ml-2" />
              </Button>
            </a>
          </div>
        </div>
      </div>

      {/* Warning Alert */}
      <Alert className="mb-6 bg-amber-500/10 border-amber-500/20">
        <AlertCircle className="h-4 w-4 text-amber-500" />
        <AlertDescription className="text-amber-500">
          <strong>Development Mode:</strong> The Snapshot UI should be running on{" "}
          <code className="bg-amber-500/20 px-1 py-0.5 rounded">localhost:8082</code>. Start it with{" "}
          <code className="bg-amber-500/20 px-1 py-0.5 rounded">yarn dev --port=8082</code> in the{" "}
          <code className="bg-amber-500/20 px-1 py-0.5 rounded">apps/governance-ui/etrid-snapshot</code> directory.
        </AlertDescription>
      </Alert>

      {/* Snapshot Iframe */}
      <Card className="overflow-hidden">
        <CardContent className="p-0">
          {isLoading && (
            <div className="absolute inset-0 bg-background/80 backdrop-blur-sm flex items-center justify-center z-10">
              <div className="text-center">
                <Loader2 className="w-8 h-8 animate-spin text-primary mx-auto mb-3" />
                <p className="text-sm text-muted-foreground">Loading Snapshot UI...</p>
              </div>
            </div>
          )}

          <iframe
            ref={iframeRef}
            src="http://localhost:8082"
            onLoad={handleIframeLoad}
            className="w-full border-0"
            style={{ height: "calc(100vh - 300px)", minHeight: "600px" }}
            title="Snapshot Governance"
            sandbox="allow-same-origin allow-scripts allow-forms allow-popups allow-modals"
          />
        </CardContent>
      </Card>

      {/* Debug Info (Development Only) */}
      {process.env.NODE_ENV === "development" && (
        <Card className="mt-6">
          <CardContent className="pt-6">
            <h3 className="font-semibold mb-4">Debug Info</h3>
            <div className="space-y-2 text-sm font-mono">
              <div>
                <span className="text-muted-foreground">Wallet Address:</span>{" "}
                <span className="text-primary">{wallet.address}</span>
              </div>
              <div>
                <span className="text-muted-foreground">Balance:</span>{" "}
                <span className="text-primary">{wallet.balance} ÉTR</span>
              </div>
              <div>
                <span className="text-muted-foreground">Connection Status:</span>{" "}
                <span className="text-primary">{connectionStatus}</span>
              </div>
              <div>
                <span className="text-muted-foreground">Recent Events:</span>
                <ul className="ml-4 mt-1">
                  {snapshotEvents.length === 0 && <li className="text-muted-foreground">No events yet</li>}
                  {snapshotEvents.map((event, idx) => (
                    <li key={idx} className="text-primary">
                      {new Date(event.timestamp).toLocaleTimeString()} - {event.type}
                    </li>
                  ))}
                </ul>
              </div>
            </div>
          </CardContent>
        </Card>
      )}

      {/* Integration Instructions */}
      <Card className="mt-6 bg-blue-500/10 border-blue-500/20">
        <CardContent className="pt-6">
          <h3 className="font-semibold mb-2 flex items-center gap-2">
            <AlertCircle className="w-5 h-5 text-blue-500" />
            Snapshot Integration Status
          </h3>
          <p className="text-sm text-muted-foreground mb-3">
            This page embeds the Vue 3 Snapshot UI via iframe using a micro-frontend approach. Communication happens
            via postMessage API.
          </p>
          <div className="space-y-1 text-sm">
            <div className="flex items-center gap-2">
              <CheckCircle2 className="w-4 h-4 text-green-500" />
              <span>Iframe embed working</span>
            </div>
            <div className="flex items-center gap-2">
              <CheckCircle2 className="w-4 h-4 text-green-500" />
              <span>PostMessage bridge configured</span>
            </div>
            <div className="flex items-center gap-2">
              <AlertCircle className="w-4 h-4 text-amber-500" />
              <span>Wallet sync via postMessage (requires Snapshot UI modifications)</span>
            </div>
            <div className="flex items-center gap-2">
              <AlertCircle className="w-4 h-4 text-amber-500" />
              <span>Vote event listeners (requires Snapshot UI modifications)</span>
            </div>
          </div>
        </CardContent>
      </Card>
    </div>
  )
}
