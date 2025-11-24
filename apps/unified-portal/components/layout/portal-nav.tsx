"use client"

import Link from "next/link"
import { usePathname } from "next/navigation"
import { cn } from "@/lib/utils"
import {
  LayoutDashboard,
  Zap,
  Shield,
  Eye,
  Wallet,
  Vote,
  Activity,
  Coins,
} from "lucide-react"

const navItems = [
  {
    name: "Dashboard",
    href: "/",
    icon: LayoutDashboard,
  },
  {
    name: "Lightning",
    href: "/lightning",
    icon: Zap,
  },
  {
    name: "Validator",
    href: "/validator",
    icon: Shield,
  },
  {
    name: "Watchtower",
    href: "/watchtower",
    icon: Eye,
  },
  {
    name: "Wallet",
    href: "/wallet",
    icon: Wallet,
  },
  {
    name: "Governance",
    href: "/governance",
    icon: Vote,
  },
  {
    name: "Monitoring",
    href: "/monitoring",
    icon: Activity,
  },
  {
    name: "MasterChef",
    href: "/masterchef",
    icon: Coins,
  },
]

export default function PortalNav() {
  const pathname = usePathname()

  return (
    <nav className="border-b border-zinc-200 bg-zinc-50 dark:border-zinc-800 dark:bg-zinc-900">
      <div className="container mx-auto px-6">
        <div className="flex h-14 items-center gap-1 overflow-x-auto">
          {navItems.map((item) => {
            const Icon = item.icon
            const isActive = pathname === item.href || (item.href !== "/" && pathname.startsWith(item.href))

            return (
              <Link
                key={item.href}
                href={item.href}
                className={cn(
                  "flex items-center gap-2 px-4 py-2 rounded-md text-sm font-medium transition-colors whitespace-nowrap",
                  isActive
                    ? "bg-white text-purple-600 shadow-sm dark:bg-zinc-800 dark:text-purple-400"
                    : "text-zinc-600 hover:text-zinc-900 hover:bg-white/50 dark:text-zinc-400 dark:hover:text-zinc-100 dark:hover:bg-zinc-800/50"
                )}
              >
                <Icon className="h-4 w-4" />
                {item.name}
              </Link>
            )
          })}
        </div>
      </div>
    </nav>
  )
}
