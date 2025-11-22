"use client"

import { Shield, Lock, Clock, AlertTriangle, Settings } from "lucide-react"
import { useSecurity } from "@/hooks/useSecurity"
import { SecurityScoreCard } from "@/components/security/SecurityScoreCard"
import { Button } from "@/components/ui/button"
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card"
import { Switch } from "@/components/ui/switch"
import { Badge } from "@/components/ui/badge"
import { Skeleton } from "@/components/ui/skeleton"
import { useRouter } from "next/navigation"

export default function SecurityCenterScreen() {
  const { settings, score, events, loading, updateSettings, setBiometrics, setTwoFactor } = useSecurity()
  const router = useRouter()

  if (loading && !settings) {
    return (
      <div className="container mx-auto p-6 max-w-7xl">
        <Skeleton className="h-10 w-64 mb-6" />
        <div className="grid grid-cols-1 md:grid-cols-3 gap-6">
          <Skeleton className="h-96" />
          <Skeleton className="h-96 md:col-span-2" />
        </div>
      </div>
    )
  }

  if (!settings || !score) return null

  return (
    <div className="container mx-auto p-6 max-w-7xl">
      {/* Header */}
      <div className="flex items-center justify-between mb-8">
        <div>
          <h1 className="text-3xl font-bold flex items-center gap-2">
            <Shield className="w-8 h-8" />
            Security Center
          </h1>
          <p className="text-muted-foreground mt-1">
            Protect your wallet with advanced security features
          </p>
        </div>
        <Button variant="outline" onClick={() => router.push("/security/settings")}>
          <Settings className="w-4 h-4 mr-2" />
          Advanced Settings
        </Button>
      </div>

      <div className="grid grid-cols-1 md:grid-cols-3 gap-6 mb-8">
        {/* Security Score */}
        <div>
          <SecurityScoreCard score={score} />
        </div>

        {/* Quick Toggles */}
        <div className="md:col-span-2 space-y-4">
          {/* Biometrics */}
          <Card>
            <CardContent className="p-6">
              <div className="flex items-center justify-between">
                <div className="flex items-center gap-3">
                  <div className="w-10 h-10 rounded-full bg-blue-500/10 flex items-center justify-center">
                    <Shield className="w-5 h-5 text-blue-500" />
                  </div>
                  <div>
                    <h4 className="font-semibold">Biometric Authentication</h4>
                    <p className="text-xs text-muted-foreground">
                      Face ID or fingerprint to unlock wallet
                    </p>
                  </div>
                </div>
                <Switch
                  checked={settings.biometricsEnabled}
                  onCheckedChange={setBiometrics}
                />
              </div>
            </CardContent>
          </Card>

          {/* 2FA */}
          <Card>
            <CardContent className="p-6">
              <div className="flex items-center justify-between">
                <div className="flex items-center gap-3">
                  <div className="w-10 h-10 rounded-full bg-green-500/10 flex items-center justify-center">
                    <Lock className="w-5 h-5 text-green-500" />
                  </div>
                  <div>
                    <h4 className="font-semibold">Two-Factor Authentication</h4>
                    <p className="text-xs text-muted-foreground">
                      Extra layer of security for transactions
                    </p>
                  </div>
                </div>
                <Switch
                  checked={settings.twoFactorEnabled}
                  onCheckedChange={setTwoFactor}
                />
              </div>
            </CardContent>
          </Card>

          {/* Whitelist */}
          <Card className="cursor-pointer hover:shadow-lg transition-shadow" onClick={() => router.push("/security/whitelist")}>
            <CardContent className="p-6">
              <div className="flex items-center justify-between">
                <div className="flex items-center gap-3">
                  <div className="w-10 h-10 rounded-full bg-purple-500/10 flex items-center justify-center">
                    <Shield className="w-5 h-5 text-purple-500" />
                  </div>
                  <div>
                    <h4 className="font-semibold">Address Whitelist</h4>
                    <p className="text-xs text-muted-foreground">
                      {settings.whitelistEnabled ? "Active" : "Disabled"}
                    </p>
                  </div>
                </div>
                <Badge variant={settings.whitelistEnabled ? "default" : "outline"}>
                  {settings.whitelistEnabled ? "ON" : "OFF"}
                </Badge>
              </div>
            </CardContent>
          </Card>

          {/* Timelock */}
          <Card className="cursor-pointer hover:shadow-lg transition-shadow" onClick={() => router.push("/security/timelock")}>
            <CardContent className="p-6">
              <div className="flex items-center justify-between">
                <div className="flex items-center gap-3">
                  <div className="w-10 h-10 rounded-full bg-yellow-500/10 flex items-center justify-center">
                    <Clock className="w-5 h-5 text-yellow-500" />
                  </div>
                  <div>
                    <h4 className="font-semibold">Withdrawal Timelock</h4>
                    <p className="text-xs text-muted-foreground">
                      {settings.timelockEnabled
                        ? `${settings.timelockHours}h delay`
                        : "Disabled"}
                    </p>
                  </div>
                </div>
                <Badge variant={settings.timelockEnabled ? "default" : "outline"}>
                  {settings.timelockEnabled ? "ON" : "OFF"}
                </Badge>
              </div>
            </CardContent>
          </Card>

          {/* Spending Limits */}
          <Card className="cursor-pointer hover:shadow-lg transition-shadow" onClick={() => router.push("/security/limits")}>
            <CardContent className="p-6">
              <div className="flex items-center justify-between">
                <div className="flex items-center gap-3">
                  <div className="w-10 h-10 rounded-full bg-red-500/10 flex items-center justify-center">
                    <AlertTriangle className="w-5 h-5 text-red-500" />
                  </div>
                  <div>
                    <h4 className="font-semibold">Spending Limits</h4>
                    <p className="text-xs text-muted-foreground">
                      Daily: ${parseFloat(settings.dailyLimit).toLocaleString()}
                    </p>
                  </div>
                </div>
                <Badge>Configured</Badge>
              </div>
            </CardContent>
          </Card>

          {/* Panic Mode */}
          <Card className={`cursor-pointer hover:shadow-lg transition-shadow ${settings.panicModeActive ? "border-2 border-red-500" : ""}`} onClick={() => router.push("/security/panic")}>
            <CardContent className="p-6">
              <div className="flex items-center justify-between">
                <div className="flex items-center gap-3">
                  <div className={`w-10 h-10 rounded-full ${settings.panicModeActive ? "bg-red-500 animate-pulse" : "bg-red-500/10"} flex items-center justify-center`}>
                    <AlertTriangle className={`w-5 h-5 ${settings.panicModeActive ? "text-white" : "text-red-500"}`} />
                  </div>
                  <div>
                    <h4 className="font-semibold">Panic Mode</h4>
                    <p className="text-xs text-muted-foreground">
                      {settings.panicModeActive
                        ? "ACTIVE - All transactions frozen"
                        : "Emergency lockdown"}
                    </p>
                  </div>
                </div>
                <Badge variant={settings.panicModeActive ? "destructive" : "outline"}>
                  {settings.panicModeActive ? "ACTIVE" : "Standby"}
                </Badge>
              </div>
            </CardContent>
          </Card>
        </div>
      </div>

      {/* Recent Security Events */}
      {events.length > 0 && (
        <Card>
          <CardHeader>
            <CardTitle>Recent Security Events</CardTitle>
          </CardHeader>
          <CardContent>
            <div className="space-y-3">
              {events.slice(0, 5).map((event) => {
                const severityColors = {
                  low: "text-blue-500",
                  medium: "text-yellow-500",
                  high: "text-orange-500",
                  critical: "text-red-500",
                }

                return (
                  <div
                    key={event.id}
                    className="flex items-center justify-between p-3 rounded-lg bg-muted/50"
                  >
                    <div className="flex items-center gap-3">
                      <AlertTriangle
                        className={`w-5 h-5 ${severityColors[event.severity]}`}
                      />
                      <div>
                        <p className="text-sm font-medium">{event.description}</p>
                        <p className="text-xs text-muted-foreground">
                          {new Date(event.timestamp).toLocaleString()}
                        </p>
                      </div>
                    </div>
                    <Badge
                      variant="outline"
                      className={severityColors[event.severity]}
                    >
                      {event.severity}
                    </Badge>
                  </div>
                )
              })}
            </div>
          </CardContent>
        </Card>
      )}
    </div>
  )
}
