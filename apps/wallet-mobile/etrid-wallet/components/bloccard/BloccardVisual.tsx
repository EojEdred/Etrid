"use client"

import { useState } from "react"
import { Eye, EyeOff, Copy } from "lucide-react"
import { Button } from "@/components/ui/button"
import { toast } from "sonner"
import type { BloccardAccount } from "@/lib/types/features"

interface BloccardVisualProps {
  account: BloccardAccount
  className?: string
}

export function BloccardVisual({ account, className = "" }: BloccardVisualProps) {
  const [isFlipped, setIsFlipped] = useState(false)
  const [showDetails, setShowDetails] = useState(false)

  const formatCardNumber = (number: string) => {
    if (!showDetails) {
      return "•••• •••• •••• " + number.slice(-4)
    }
    return number.match(/.{1,4}/g)?.join(" ") || number
  }

  const copyToClipboard = (text: string, label: string) => {
    navigator.clipboard.writeText(text)
    toast.success(`${label} copied to clipboard`)
  }

  return (
    <div className={`perspective-1000 ${className}`}>
      <div
        className={`relative w-full aspect-[1.586/1] transition-all duration-500 transform-style-3d ${
          isFlipped ? "rotate-y-180" : ""
        }`}
      >
        {/* Front of Card */}
        <div
          className={`absolute inset-0 backface-hidden ${isFlipped ? "hidden" : ""}`}
          onClick={() => setIsFlipped(!isFlipped)}
        >
          <div className="w-full h-full rounded-3xl bg-gradient-to-br from-purple-600 via-purple-700 to-purple-900 p-6 shadow-2xl cursor-pointer">
            {/* Card Header */}
            <div className="flex items-start justify-between mb-8">
              <div>
                <p className="text-white/70 text-xs font-medium mb-1">AU BLOCCARD</p>
                <p className="text-white text-sm font-semibold">
                  {account.cardType === "virtual" ? "Virtual Card" : "Physical Card"}
                </p>
              </div>
              <div className="flex items-center gap-2">
                <Button
                  variant="ghost"
                  size="icon"
                  onClick={(e) => {
                    e.stopPropagation()
                    setShowDetails(!showDetails)
                  }}
                  className="text-white hover:bg-white/10"
                >
                  {showDetails ? <EyeOff className="w-4 h-4" /> : <Eye className="w-4 h-4" />}
                </Button>
              </div>
            </div>

            {/* Card Number */}
            <div className="mb-6">
              <div className="flex items-center gap-2">
                <p className="text-white text-xl font-mono tracking-wider">{formatCardNumber(account.cardNumber)}</p>
                {showDetails && (
                  <Button
                    variant="ghost"
                    size="icon"
                    onClick={(e) => {
                      e.stopPropagation()
                      copyToClipboard(account.cardNumber, "Card number")
                    }}
                    className="text-white hover:bg-white/10 h-6 w-6"
                  >
                    <Copy className="w-3 h-3" />
                  </Button>
                )}
              </div>
            </div>

            {/* Card Details */}
            <div className="flex items-end justify-between">
              <div>
                <p className="text-white/70 text-xs mb-1">CARDHOLDER</p>
                <p className="text-white font-semibold text-sm">{account.cardholderName.toUpperCase()}</p>
              </div>
              <div className="text-right">
                <p className="text-white/70 text-xs mb-1">EXPIRES</p>
                <p className="text-white font-semibold text-sm font-mono">
                  {account.expiryMonth.toString().padStart(2, "0")}/{account.expiryYear.toString().slice(-2)}
                </p>
              </div>
              <div className="text-right">
                <div className="w-12 h-8 rounded bg-white/20 flex items-center justify-center">
                  <span className="text-white text-xs font-bold">VISA</span>
                </div>
              </div>
            </div>

            {/* Tap to flip indicator */}
            <div className="absolute bottom-2 left-1/2 -translate-x-1/2">
              <p className="text-white/50 text-xs">Tap to flip</p>
            </div>
          </div>
        </div>

        {/* Back of Card */}
        <div
          className={`absolute inset-0 backface-hidden rotate-y-180 ${!isFlipped ? "hidden" : ""}`}
          onClick={() => setIsFlipped(!isFlipped)}
        >
          <div className="w-full h-full rounded-3xl bg-gradient-to-br from-purple-900 via-purple-700 to-purple-600 shadow-2xl cursor-pointer overflow-hidden">
            {/* Magnetic Strip */}
            <div className="w-full h-12 bg-black mt-6"></div>

            {/* CVV Section */}
            <div className="p-6 mt-4">
              <div className="bg-white/10 rounded-lg p-4 backdrop-blur-sm">
                <div className="flex items-center justify-between mb-2">
                  <p className="text-white/70 text-xs">CVV</p>
                  <Button
                    variant="ghost"
                    size="icon"
                    onClick={(e) => {
                      e.stopPropagation()
                      setShowDetails(!showDetails)
                    }}
                    className="text-white hover:bg-white/10 h-6 w-6"
                  >
                    {showDetails ? <EyeOff className="w-3 h-3" /> : <Eye className="w-3 h-3" />}
                  </Button>
                </div>
                <div className="flex items-center gap-2">
                  <p className="text-white text-2xl font-mono tracking-widest">
                    {showDetails ? account.cvv : "•••"}
                  </p>
                  {showDetails && (
                    <Button
                      variant="ghost"
                      size="icon"
                      onClick={(e) => {
                        e.stopPropagation()
                        copyToClipboard(account.cvv, "CVV")
                      }}
                      className="text-white hover:bg-white/10 h-6 w-6"
                    >
                      <Copy className="w-3 h-3" />
                    </Button>
                  )}
                </div>
              </div>

              <div className="mt-6 text-white/50 text-xs">
                <p>For your security, do not share your card details with anyone.</p>
              </div>
            </div>

            {/* Tap to flip indicator */}
            <div className="absolute bottom-2 left-1/2 -translate-x-1/2">
              <p className="text-white/50 text-xs">Tap to flip</p>
            </div>
          </div>
        </div>
      </div>

      <style jsx>{`
        .perspective-1000 {
          perspective: 1000px;
        }
        .transform-style-3d {
          transform-style: preserve-3d;
        }
        .backface-hidden {
          backface-visibility: hidden;
        }
        .rotate-y-180 {
          transform: rotateY(180deg);
        }
      `}</style>
    </div>
  )
}
