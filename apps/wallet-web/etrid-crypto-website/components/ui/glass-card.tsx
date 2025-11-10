/**
 * Glassmorphic Card Component
 * Source: Inspired by Uiverse.io
 *
 * A modern glass-effect card with backdrop blur
 * Perfect for feature cards and info panels
 */

import * as React from "react"
import { cn } from "@/lib/utils"

export interface GlassCardProps extends React.HTMLAttributes<HTMLDivElement> {
  children: React.ReactNode
  variant?: "default" | "bordered" | "glow"
  hover?: boolean
}

export function GlassCard({
  children,
  className,
  variant = "default",
  hover = true,
  ...props
}: GlassCardProps) {
  return (
    <div
      className={cn(
        // Base glass effect
        "rounded-2xl backdrop-blur-md",
        "bg-white/10 dark:bg-white/5",
        "shadow-lg",
        // Variants
        variant === "bordered" && "border border-white/20",
        variant === "glow" && "border border-white/20 shadow-[0_0_15px_rgba(255,255,255,0.1)]",
        // Hover effect
        hover && "transition-all duration-300 hover:scale-105 hover:shadow-xl",
        hover && variant === "glow" && "hover:shadow-[0_0_25px_rgba(255,255,255,0.2)]",
        className
      )}
      {...props}
    >
      {children}
    </div>
  )
}

/**
 * Usage Example:
 *
 * import { GlassCard } from "@/components/ui/glass-card"
 *
 * <GlassCard variant="glow" className="p-8">
 *   <h3>Feature Title</h3>
 *   <p>Feature description...</p>
 * </GlassCard>
 */
