import * as React from "react"
import { cn } from "./utils"
import { Blocks } from "lucide-react"

interface BlockHeightProps extends React.HTMLAttributes<HTMLDivElement> {
  height: number
  finalized?: number
  showIcon?: boolean
}

export function BlockHeight({
  height,
  finalized,
  showIcon = true,
  className,
  ...props
}: BlockHeightProps) {
  return (
    <div
      className={cn("inline-flex items-center gap-2", className)}
      {...props}
    >
      {showIcon && <Blocks className="h-4 w-4" />}
      <div className="flex flex-col">
        <span className="font-mono font-medium">
          #{height.toLocaleString()}
        </span>
        {finalized !== undefined && (
          <span className="text-xs text-muted-foreground">
            Finalized: #{finalized.toLocaleString()}
          </span>
        )}
      </div>
    </div>
  )
}
