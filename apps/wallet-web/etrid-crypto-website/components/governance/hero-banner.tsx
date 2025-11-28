"use client"

import { useEffect, useState } from "react"

export default function HeroBanner() {
  const [timeLeft, setTimeLeft] = useState({
    days: 5,
    hours: 12,
    minutes: 34,
    seconds: 0,
  })

  useEffect(() => {
    const timer = setInterval(() => {
      setTimeLeft((prev) => {
        if (prev.seconds > 0) {
          return { ...prev, seconds: prev.seconds - 1 }
        } else if (prev.minutes > 0) {
          return { ...prev, minutes: prev.minutes - 1, seconds: 59 }
        } else if (prev.hours > 0) {
          return { ...prev, hours: prev.hours - 1, minutes: 59, seconds: 59 }
        } else if (prev.days > 0) {
          return { ...prev, days: prev.days - 1, hours: 23, minutes: 59, seconds: 59 }
        }
        return prev
      })
    }, 1000)

    return () => clearInterval(timer)
  }, [])

  return (
    <div className="relative overflow-hidden bg-gradient-to-r from-primary/20 via-accent/20 to-primary/20 border-b border-border">
      <div className="container mx-auto px-4 py-16 text-center">
        <h1 className="text-5xl md:text-6xl font-bold mb-6 bg-gradient-to-r from-primary to-accent bg-clip-text text-transparent">
          Shape Ëtrid's Future
        </h1>

        <div className="mb-6">
          <p className="text-muted-foreground mb-2">Voting ends in:</p>
          <div className="flex justify-center gap-4 text-3xl md:text-4xl font-bold">
            <div className="flex flex-col items-center">
              <span className="text-accent">{timeLeft.days}</span>
              <span className="text-xs text-muted-foreground">days</span>
            </div>
            <span className="text-muted-foreground">:</span>
            <div className="flex flex-col items-center">
              <span className="text-accent">{timeLeft.hours}</span>
              <span className="text-xs text-muted-foreground">hours</span>
            </div>
            <span className="text-muted-foreground">:</span>
            <div className="flex flex-col items-center">
              <span className="text-accent">{timeLeft.minutes}</span>
              <span className="text-xs text-muted-foreground">minutes</span>
            </div>
            <span className="text-muted-foreground">:</span>
            <div className="flex flex-col items-center">
              <span className="text-accent">{timeLeft.seconds}</span>
              <span className="text-xs text-muted-foreground">seconds</span>
            </div>
          </div>
        </div>

        <p className="text-lg text-muted-foreground">Cast your vote on fiscal policy and network upgrades</p>
      </div>
    </div>
  )
}
