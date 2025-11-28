import * as React from "react"
import { cn } from "./utils"
import { Copy, Check } from "lucide-react"

interface AddressDisplayProps extends React.HTMLAttributes<HTMLDivElement> {
  address: string
  showCopy?: boolean
  short?: boolean
}

export function AddressDisplay({
  address,
  showCopy = true,
  short = true,
  className,
  ...props
}: AddressDisplayProps) {
  const [copied, setCopied] = React.useState(false)

  const formatted = short && address.length > 12
    ? `${address.slice(0, 6)}...${address.slice(-4)}`
    : address

  const handleCopy = async () => {
    try {
      await navigator.clipboard.writeText(address)
      setCopied(true)
      setTimeout(() => setCopied(false), 2000)
    } catch (err) {
      console.error('Failed to copy:', err)
    }
  }

  return (
    <div
      className={cn("inline-flex items-center gap-2", className)}
      {...props}
    >
      <code className="relative rounded bg-muted px-2 py-1 text-sm font-mono">
        {formatted}
      </code>
      {showCopy && (
        <button
          onClick={handleCopy}
          className="inline-flex items-center justify-center rounded-md p-1 hover:bg-accent hover:text-accent-foreground transition-colors"
          title="Copy address"
        >
          {copied ? (
            <Check className="h-4 w-4 text-green-500" />
          ) : (
            <Copy className="h-4 w-4" />
          )}
        </button>
      )}
    </div>
  )
}
