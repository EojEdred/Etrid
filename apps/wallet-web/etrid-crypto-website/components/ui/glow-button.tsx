/**
 * Glowing Button Component
 * Source: Inspired by Uiverse.io
 *
 * A button with an animated glowing border effect
 * Perfect for CTAs and important actions
 */

import * as React from "react"
import { cn } from "@/lib/utils"

export interface GlowButtonProps
  extends React.ButtonHTMLAttributes<HTMLButtonElement> {
  children: React.ReactNode
  glowColor?: "primary" | "accent" | "success" | "purple"
}

const glowColors = {
  primary: "bg-[conic-gradient(from_90deg_at_50%_50%,#0EA5E9_0%,#3B82F6_50%,#0EA5E9_100%)]",
  accent: "bg-[conic-gradient(from_90deg_at_50%_50%,#F59E0B_0%,#EF4444_50%,#F59E0B_100%)]",
  success: "bg-[conic-gradient(from_90deg_at_50%_50%,#10B981_0%,#059669_50%,#10B981_100%)]",
  purple: "bg-[conic-gradient(from_90deg_at_50%_50%,#E2CBFF_0%,#393BB2_50%,#E2CBFF_100%)]",
}

export function GlowButton({
  children,
  className,
  glowColor = "purple",
  ...props
}: GlowButtonProps) {
  return (
    <button
      className={cn(
        "relative inline-flex h-12 overflow-hidden rounded-full p-[1px]",
        "focus:outline-none focus:ring-2 focus:ring-slate-400 focus:ring-offset-2",
        "transition-all duration-300 hover:scale-105",
        className
      )}
      {...props}
    >
      <span
        className={cn(
          "absolute inset-[-1000%] animate-[spin_2s_linear_infinite]",
          glowColors[glowColor]
        )}
      />
      <span className="inline-flex h-full w-full cursor-pointer items-center justify-center rounded-full bg-slate-950 dark:bg-slate-950 px-8 py-1 text-sm font-medium text-white backdrop-blur-3xl gap-2">
        {children}
      </span>
    </button>
  )
}

/**
 * Usage Example:
 *
 * import { GlowButton } from "@/components/ui/glow-button"
 *
 * <GlowButton glowColor="primary">
 *   Launch App
 * </GlowButton>
 *
 * <GlowButton glowColor="accent">
 *   Governance
 * </GlowButton>
 */
