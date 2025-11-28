"use client"

import { Shield, Eye, EyeOff, Shuffle } from "lucide-react"
import { usePrivacy } from "@/hooks/usePrivacy"
import { useStealthAddress } from "@/hooks/useStealthAddress"
import { useCoinMix } from "@/hooks/useCoinMix"
import { PrivacyLevel } from "@/components/privacy/PrivacyLevel"
import { StealthAddressCard } from "@/components/privacy/StealthAddressCard"
import { Button } from "@/components/ui/button"
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card"
import { Progress } from "@/components/ui/progress"
import { Badge } from "@/components/ui/badge"
import { Skeleton } from "@/components/ui/skeleton"
import { Switch } from "@/components/ui/switch"
import { useRouter } from "next/navigation"

export default function PrivacyScreen() {
  const { settings, score, loading: privacyLoading, setPrivacyLevel, setTor } = usePrivacy()
  const { addresses, generateAddress, getCounts } = useStealthAddress()
  const { getActiveSessions } = useCoinMix()
  const router = useRouter()

  const counts = getCounts()
  const activeSessions = getActiveSessions()

  if (privacyLoading && !settings) {
    return (
      <div className="container mx-auto p-6 max-w-7xl">
        <Skeleton className="h-10 w-64 mb-6" />
        <Skeleton className="h-96" />
      </div>
    )
  }

  if (!settings || !score) return null

  return (
    <div className="container mx-auto p-6 max-w-7xl">
      {/* Header */}
      <div className="mb-8">
        <h1 className="text-3xl font-bold flex items-center gap-2">
          <Shield className="w-8 h-8" />
          Privacy Center
        </h1>
        <p className="text-muted-foreground mt-1">
          Enhance your transaction privacy with advanced features
        </p>
      </div>

      {/* Privacy Score */}
      <Card className="mb-8">
        <CardContent className="p-6">
          <div className="flex items-center justify-between mb-4">
            <div>
              <h3 className="text-lg font-semibold mb-1">Privacy Score</h3>
              <p className="text-sm text-muted-foreground">
                Your current privacy protection level
              </p>
            </div>
            <div className="text-right">
              <p className="text-4xl font-bold">{score.overall}</p>
              <p className="text-xs text-muted-foreground">/ 100</p>
            </div>
          </div>

          <Progress value={score.overall} className="h-3 mb-4" />

          <div className="grid grid-cols-2 md:grid-cols-4 gap-4">
            <div className="text-center p-3 rounded-lg bg-muted/50">
              <p className="text-2xl font-bold">{score.breakdown.addressReuse}</p>
              <p className="text-xs text-muted-foreground">Address Reuse</p>
            </div>
            <div className="text-center p-3 rounded-lg bg-muted/50">
              <p className="text-2xl font-bold">{score.breakdown.mixingLevel}</p>
              <p className="text-xs text-muted-foreground">Mixing</p>
            </div>
            <div className="text-center p-3 rounded-lg bg-muted/50">
              <p className="text-2xl font-bold">
                {score.breakdown.metadataProtection}
              </p>
              <p className="text-xs text-muted-foreground">Metadata</p>
            </div>
            <div className="text-center p-3 rounded-lg bg-muted/50">
              <p className="text-2xl font-bold">
                {score.breakdown.networkPrivacy}
              </p>
              <p className="text-xs text-muted-foreground">Network</p>
            </div>
          </div>

          <Badge
            className={`mt-4 ${score.riskLevel === "low" ? "bg-green-500" : score.riskLevel === "medium" ? "bg-yellow-500" : "bg-red-500"}`}
          >
            Risk Level: {score.riskLevel.toUpperCase()}
          </Badge>
        </CardContent>
      </Card>

      {/* Privacy Level Selection */}
      <div className="mb-8">
        <h3 className="text-xl font-semibold mb-4">Privacy Level</h3>
        <PrivacyLevel
          selected={settings.privacyLevel}
          onSelect={setPrivacyLevel}
        />
      </div>

      {/* Quick Settings */}
      <div className="grid grid-cols-1 md:grid-cols-2 gap-6 mb-8">
        {/* Stealth Addresses */}
        <Card>
          <CardHeader>
            <CardTitle className="flex items-center gap-2">
              <EyeOff className="w-5 h-5" />
              Stealth Addresses
            </CardTitle>
          </CardHeader>
          <CardContent>
            <div className="flex items-center justify-between mb-4">
              <div>
                <p className="text-sm font-medium">Auto-generate new addresses</p>
                <p className="text-xs text-muted-foreground">
                  One-time use addresses for privacy
                </p>
              </div>
              <Switch
                checked={settings.stealthAddressesEnabled}
                onCheckedChange={(enabled) =>
                  setPrivacyLevel(enabled ? "medium" : "low")
                }
              />
            </div>

            <div className="grid grid-cols-3 gap-2 mb-4">
              <div className="text-center p-2 rounded-lg bg-muted/50">
                <p className="text-xl font-bold text-green-500">{counts.unused}</p>
                <p className="text-xs text-muted-foreground">Unused</p>
              </div>
              <div className="text-center p-2 rounded-lg bg-muted/50">
                <p className="text-xl font-bold text-blue-500">{counts.used}</p>
                <p className="text-xs text-muted-foreground">Used</p>
              </div>
              <div className="text-center p-2 rounded-lg bg-muted/50">
                <p className="text-xl font-bold">{counts.total}</p>
                <p className="text-xs text-muted-foreground">Total</p>
              </div>
            </div>

            <Button
              onClick={() => router.push("/privacy/stealth")}
              variant="outline"
              className="w-full"
            >
              Manage Addresses
            </Button>
          </CardContent>
        </Card>

        {/* Coin Mixing */}
        <Card>
          <CardHeader>
            <CardTitle className="flex items-center gap-2">
              <Shuffle className="w-5 h-5" />
              Coin Mixing
            </CardTitle>
          </CardHeader>
          <CardContent>
            <div className="flex items-center justify-between mb-4">
              <div>
                <p className="text-sm font-medium">Transaction anonymization</p>
                <p className="text-xs text-muted-foreground">
                  Mix coins to enhance privacy
                </p>
              </div>
              <Switch
                checked={settings.mixingEnabled}
                onCheckedChange={(enabled) =>
                  setPrivacyLevel(enabled ? "medium" : "low")
                }
              />
            </div>

            {activeSessions.length > 0 && (
              <div className="mb-4 p-3 rounded-lg bg-blue-500/10 border border-blue-500/50">
                <p className="text-sm font-semibold text-blue-500">
                  {activeSessions.length} Active Session
                  {activeSessions.length > 1 ? "s" : ""}
                </p>
                <p className="text-xs text-muted-foreground">
                  Mixing in progress...
                </p>
              </div>
            )}

            <Button
              onClick={() => router.push("/privacy/mixing")}
              variant="outline"
              className="w-full"
            >
              Start Mixing
            </Button>
          </CardContent>
        </Card>

        {/* Tor Routing */}
        <Card>
          <CardContent className="p-6">
            <div className="flex items-center justify-between">
              <div className="flex items-center gap-3">
                <div className="w-10 h-10 rounded-full bg-purple-500/10 flex items-center justify-center">
                  <Eye className="w-5 h-5 text-purple-500" />
                </div>
                <div>
                  <h4 className="font-semibold">Tor Routing</h4>
                  <p className="text-xs text-muted-foreground">
                    Hide your IP address
                  </p>
                </div>
              </div>
              <Switch
                checked={settings.torRoutingEnabled}
                onCheckedChange={setTor}
              />
            </div>
          </CardContent>
        </Card>

        {/* Metadata Scrubbing */}
        <Card>
          <CardContent className="p-6">
            <div className="flex items-center justify-between">
              <div className="flex items-center gap-3">
                <div className="w-10 h-10 rounded-full bg-green-500/10 flex items-center justify-center">
                  <Shield className="w-5 h-5 text-green-500" />
                </div>
                <div>
                  <h4 className="font-semibold">Metadata Scrubbing</h4>
                  <p className="text-xs text-muted-foreground">
                    Remove transaction metadata
                  </p>
                </div>
              </div>
              <Switch checked={settings.metadataScrubbing} />
            </div>
          </CardContent>
        </Card>
      </div>

      {/* Recent Stealth Addresses */}
      {addresses.length > 0 && (
        <div>
          <h3 className="text-xl font-semibold mb-4">Recent Stealth Addresses</h3>
          <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
            {addresses.slice(0, 2).map((address) => (
              <StealthAddressCard
                key={address.id}
                address={address}
                onGenerate={generateAddress}
              />
            ))}
          </div>
        </div>
      )}
    </div>
  )
}
