"use client"

import { useState } from "react"
import { useRouter } from "next/navigation"
import { ArrowLeft, ArrowRight, Users, Shield, CheckCircle } from "lucide-react"
import { useMultiSig } from "@/hooks/useMultiSig"
import { WALLET_CONFIGS } from "@/lib/types/multisig"
import { Button } from "@/components/ui/button"
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card"
import { Input } from "@/components/ui/input"
import { Label } from "@/components/ui/label"
import { Badge } from "@/components/ui/badge"
import { Slider } from "@/components/ui/slider"

export default function CreateMultiSigScreen() {
  const router = useRouter()
  const { createWallet, loading } = useMultiSig()

  const [step, setStep] = useState(1)
  const [name, setName] = useState("")
  const [purpose, setPurpose] = useState<"personal" | "business" | "dao" | "couples">("personal")
  const [signers, setSigners] = useState<string[]>([""])
  const [threshold, setThreshold] = useState(2)

  const addSigner = () => {
    setSigners([...signers, ""])
  }

  const updateSigner = (index: number, value: string) => {
    const updated = [...signers]
    updated[index] = value
    setSigners(updated)
  }

  const removeSigner = (index: number) => {
    setSigners(signers.filter((_, i) => i !== index))
  }

  const handleCreate = async () => {
    const validSigners = signers.filter((s) => s.trim())
    if (validSigners.length < 2) {
      alert("Need at least 2 signers")
      return
    }

    try {
      await createWallet({
        name,
        purpose,
        signers: validSigners,
        threshold,
      })
      router.push("/multisig")
    } catch (error) {
      console.error("Failed to create wallet:", error)
    }
  }

  const renderStep = () => {
    switch (step) {
      case 1:
        return (
          <Card>
            <CardHeader>
              <CardTitle>Step 1: Wallet Name & Purpose</CardTitle>
            </CardHeader>
            <CardContent className="space-y-6">
              <div>
                <Label htmlFor="name">Wallet Name</Label>
                <Input
                  id="name"
                  value={name}
                  onChange={(e) => setName(e.target.value)}
                  placeholder="e.g., Family Savings, Company Treasury"
                  className="mt-2"
                />
              </div>

              <div>
                <Label>Purpose</Label>
                <div className="grid grid-cols-2 gap-4 mt-2">
                  {Object.entries(WALLET_CONFIGS).map(([key, config]) => (
                    <Card
                      key={key}
                      className={`cursor-pointer transition-all ${
                        purpose === key
                          ? "border-2 border-accent shadow-lg"
                          : "border-2 border-transparent hover:border-muted"
                      }`}
                      onClick={() => setPurpose(key as any)}
                    >
                      <CardContent className="p-4">
                        <h4 className="font-semibold mb-1">{config.name}</h4>
                        <p className="text-xs text-muted-foreground mb-2">
                          {config.description}
                        </p>
                        <Badge variant="outline" className="text-xs">
                          {config.recommendedThreshold}
                        </Badge>
                      </CardContent>
                    </Card>
                  ))}
                </div>
              </div>
            </CardContent>
          </Card>
        )

      case 2:
        return (
          <Card>
            <CardHeader>
              <CardTitle>Step 2: Add Signers</CardTitle>
            </CardHeader>
            <CardContent className="space-y-4">
              {signers.map((signer, index) => (
                <div key={index} className="flex gap-2">
                  <Input
                    value={signer}
                    onChange={(e) => updateSigner(index, e.target.value)}
                    placeholder="Username or wallet address"
                  />
                  {signers.length > 1 && (
                    <Button
                      variant="ghost"
                      onClick={() => removeSigner(index)}
                    >
                      Remove
                    </Button>
                  )}
                </div>
              ))}
              <Button variant="outline" onClick={addSigner} className="w-full">
                <Users className="w-4 h-4 mr-2" />
                Add Signer
              </Button>
              <p className="text-xs text-muted-foreground">
                {WALLET_CONFIGS[purpose].minSigners}-
                {WALLET_CONFIGS[purpose].maxSigners} signers recommended
              </p>
            </CardContent>
          </Card>
        )

      case 3:
        const totalSigners = signers.filter((s) => s.trim()).length
        return (
          <Card>
            <CardHeader>
              <CardTitle>Step 3: Set Threshold</CardTitle>
            </CardHeader>
            <CardContent className="space-y-6">
              <div className="text-center p-6 rounded-lg bg-gradient-to-br from-accent/10 to-accent/5">
                <p className="text-sm text-muted-foreground mb-2">
                  Signatures Required
                </p>
                <p className="text-5xl font-bold">
                  {threshold}-of-{totalSigners}
                </p>
              </div>

              <div>
                <Label>Threshold: {threshold} signatures required</Label>
                <Slider
                  value={[threshold]}
                  onValueChange={(v) => setThreshold(v[0])}
                  min={1}
                  max={totalSigners}
                  step={1}
                  className="mt-4"
                />
              </div>

              <div className="p-4 rounded-lg bg-muted/50">
                <p className="text-sm text-muted-foreground">
                  {threshold === totalSigners &&
                    "All signers must approve (maximum security)"}
                  {threshold === Math.ceil(totalSigners / 2) &&
                    "Majority approval required (balanced)"}
                  {threshold === 1 && "Any signer can approve (least secure)"}
                </p>
              </div>
            </CardContent>
          </Card>
        )

      case 4:
        return (
          <Card>
            <CardHeader>
              <CardTitle>Step 4: Review & Deploy</CardTitle>
            </CardHeader>
            <CardContent className="space-y-6">
              <div className="space-y-4">
                <div>
                  <Label className="text-muted-foreground">Wallet Name</Label>
                  <p className="font-semibold">{name}</p>
                </div>
                <div>
                  <Label className="text-muted-foreground">Purpose</Label>
                  <p className="font-semibold capitalize">{purpose}</p>
                </div>
                <div>
                  <Label className="text-muted-foreground">Signers</Label>
                  <div className="space-y-1 mt-2">
                    {signers.filter((s) => s.trim()).map((signer, i) => (
                      <div key={i} className="flex items-center gap-2">
                        <CheckCircle className="w-4 h-4 text-green-500" />
                        <code className="text-sm">{signer}</code>
                      </div>
                    ))}
                  </div>
                </div>
                <div>
                  <Label className="text-muted-foreground">Threshold</Label>
                  <p className="font-semibold">
                    {threshold}-of-{signers.filter((s) => s.trim()).length}
                  </p>
                </div>
              </div>

              <Button
                onClick={handleCreate}
                disabled={loading}
                className="w-full"
                size="lg"
              >
                {loading ? "Creating..." : "Deploy Multi-Sig Wallet"}
              </Button>
            </CardContent>
          </Card>
        )

      default:
        return null
    }
  }

  return (
    <div className="container mx-auto p-6 max-w-3xl">
      {/* Header */}
      <div className="mb-8">
        <Button
          variant="ghost"
          onClick={() => (step > 1 ? setStep(step - 1) : router.push("/multisig"))}
          className="mb-4"
        >
          <ArrowLeft className="w-4 h-4 mr-2" />
          Back
        </Button>
        <h1 className="text-3xl font-bold flex items-center gap-2">
          <Shield className="w-8 h-8" />
          Create Multi-Sig Wallet
        </h1>
        <p className="text-muted-foreground mt-1">Step {step} of 4</p>
      </div>

      {/* Progress */}
      <div className="flex items-center gap-2 mb-8">
        {[1, 2, 3, 4].map((s) => (
          <div
            key={s}
            className={`h-2 flex-1 rounded-full ${s <= step ? "bg-accent" : "bg-muted"}`}
          />
        ))}
      </div>

      {/* Content */}
      {renderStep()}

      {/* Navigation */}
      {step < 4 && (
        <div className="flex justify-end mt-6">
          <Button
            onClick={() => setStep(step + 1)}
            disabled={
              (step === 1 && !name) ||
              (step === 2 && signers.filter((s) => s.trim()).length < 2)
            }
          >
            Next
            <ArrowRight className="w-4 h-4 ml-2" />
          </Button>
        </div>
      )}
    </div>
  )
}
