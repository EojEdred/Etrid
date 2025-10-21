"use client"

import { useEffect, useRef, useState } from "react"

const stats = [
  { value: 1000000000, label: "Total Supply", suffix: " ÉTR" },
  { value: 8.5, label: "Staking Rewards", suffix: "% APY" },
  { value: 5, label: "Block Time", suffix: " Seconds" },
  { value: 100, label: "Network Security", suffix: "+ Validators" },
]

function CountUp({ end, duration = 2000, suffix = "" }: { end: number; duration?: number; suffix?: string }) {
  const [count, setCount] = useState(0)
  const [isVisible, setIsVisible] = useState(false)
  const ref = useRef<HTMLDivElement>(null)

  useEffect(() => {
    const observer = new IntersectionObserver(
      ([entry]) => {
        if (entry.isIntersecting) {
          setIsVisible(true)
        }
      },
      { threshold: 0.5 },
    )

    if (ref.current) {
      observer.observe(ref.current)
    }

    return () => observer.disconnect()
  }, [])

  useEffect(() => {
    if (!isVisible) return

    let startTime: number
    let animationFrame: number

    const animate = (currentTime: number) => {
      if (!startTime) startTime = currentTime
      const progress = Math.min((currentTime - startTime) / duration, 1)

      setCount(Math.floor(progress * end))

      if (progress < 1) {
        animationFrame = requestAnimationFrame(animate)
      }
    }

    animationFrame = requestAnimationFrame(animate)

    return () => cancelAnimationFrame(animationFrame)
  }, [end, duration, isVisible])

  const formatNumber = (num: number) => {
    if (num >= 1000000000) {
      return (num / 1000000000).toFixed(1) + " Billion"
    }
    if (num >= 1000000) {
      return (num / 1000000).toFixed(1) + "M"
    }
    return num.toLocaleString()
  }

  return (
    <div ref={ref} className="text-4xl md:text-5xl font-bold text-primary">
      {formatNumber(count)}
      {suffix}
    </div>
  )
}

export default function Stats() {
  return (
    <section className="py-24 px-4 bg-gradient-to-b from-background to-secondary/10">
      <div className="container mx-auto max-w-7xl">
        <div className="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-4 gap-8">
          {stats.map((stat) => (
            <div key={stat.label} className="text-center space-y-4">
              <CountUp end={stat.value} suffix={stat.suffix} />
              <p className="text-lg text-muted-foreground">{stat.label}</p>
            </div>
          ))}
        </div>
      </div>
    </section>
  )
}
