"use client"

import Link from "next/link"
import { Moon, Sun, Wallet } from "lucide-react"
import { useTheme } from "next-themes"
import { Button } from "@/components/ui/button"
import { Badge } from "@/components/ui/badge"

export default function PortalHeader() {
  const { theme, setTheme } = useTheme()

  return (
    <header className="border-b border-zinc-200 bg-white dark:border-zinc-800 dark:bg-zinc-950">
      <div className="container mx-auto flex h-16 items-center justify-between px-6">
        <div className="flex items-center gap-4">
          <Link href="/" className="flex items-center gap-3">
            <div className="flex h-10 w-10 items-center justify-center rounded-lg bg-gradient-to-br from-purple-600 to-blue-600">
              <span className="text-xl font-bold text-white">E</span>
            </div>
            <div className="flex flex-col">
              <span className="text-xl font-bold bg-gradient-to-r from-purple-600 to-blue-600 bg-clip-text text-transparent">
                Etrid Portal
              </span>
              <span className="text-xs text-zinc-500 dark:text-zinc-400">
                Control Center
              </span>
            </div>
          </Link>
          <Badge variant="success" className="ml-2">
            Live
          </Badge>
        </div>

        <div className="flex items-center gap-3">
          <Button
            variant="outline"
            size="sm"
            className="gap-2"
          >
            <Wallet className="h-4 w-4" />
            Connect Wallet
          </Button>

          <Button
            variant="ghost"
            size="icon"
            onClick={() => setTheme(theme === "dark" ? "light" : "dark")}
          >
            <Sun className="h-5 w-5 rotate-0 scale-100 transition-all dark:-rotate-90 dark:scale-0" />
            <Moon className="absolute h-5 w-5 rotate-90 scale-0 transition-all dark:rotate-0 dark:scale-100" />
            <span className="sr-only">Toggle theme</span>
          </Button>
        </div>
      </div>
    </header>
  )
}
