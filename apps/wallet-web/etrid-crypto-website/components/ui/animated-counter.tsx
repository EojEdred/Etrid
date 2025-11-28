"use client"

/**
 * Animated Counter Component
 * Source: Inspired by Uiverse.io
 *
 * Animates numbers counting up from 0 to target value
 * Perfect for stats and metrics
 */

import { useEffect, useRef, useState } from "react"
import { cn } from "@/lib/utils"

export interface AnimatedCounterProps {
  value: number
  duration?: number
  className?: string
  prefix?: string
  suffix?: string
  decimals?: number
}

export function AnimatedCounter({
  value,
  duration = 2000,
  className,
  prefix = "",
  suffix = "",
  decimals = 0,
}: AnimatedCounterProps) {
  const [count, setCount] = useState(0)
  const [isVisible, setIsVisible] = useState(false)
  const counterRef = useRef<HTMLSpanElement>(null)

  useEffect(() => {
    const observer = new IntersectionObserver(
      ([entry]) => {
        if (entry.isIntersecting && !isVisible) {
          setIsVisible(true)
        }
      },
      { threshold: 0.1 }
    )

    if (counterRef.current) {
      observer.observe(counterRef.current)
    }

    return () => observer.disconnect()
  }, [isVisible])

  useEffect(() => {
    if (!isVisible) return

    let startTime: number
    let animationFrame: number

    const animate = (currentTime: number) => {
      if (!startTime) startTime = currentTime
      const progress = Math.min((currentTime - startTime) / duration, 1)

      // Easing function (ease-out)
      const easeOut = 1 - Math.pow(1 - progress, 3)
      const currentCount = Math.floor(easeOut * value)

      setCount(currentCount)

      if (progress < 1) {
        animationFrame = requestAnimationFrame(animate)
      } else {
        setCount(value)
      }
    }

    animationFrame = requestAnimationFrame(animate)

    return () => cancelAnimationFrame(animationFrame)
  }, [value, duration, isVisible])

  const formattedValue = decimals > 0 ? count.toFixed(decimals) : count.toString()

  return (
    <span ref={counterRef} className={cn("font-bold tabular-nums", className)}>
      {prefix}
      {formattedValue.replace(/\B(?=(\d{3})+(?!\d))/g, ",")}
      {suffix}
    </span>
  )
}

/**
 * Usage Example:
 *
 * import { AnimatedCounter } from "@/components/ui/animated-counter"
 *
 * <AnimatedCounter
 *   value={1000000}
 *   suffix=" ÉTR"
 *   className="text-4xl text-primary"
 * />
 *
 * <AnimatedCounter
 *   value={12.5}
 *   decimals={1}
 *   suffix="% APY"
 *   duration={1500}
 * />
 */
