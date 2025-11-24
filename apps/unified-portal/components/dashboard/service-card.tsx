import Link from "next/link"
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card"
import { Button } from "@/components/ui/button"
import { ArrowRight, LucideIcon } from "lucide-react"

interface ServiceCardProps {
  name: string
  href: string
  description?: string
  icon?: LucideIcon
}

export function ServiceCard({ name, href, description, icon: Icon }: ServiceCardProps) {
  return (
    <Link href={href} className="block group">
      <Card className="transition-all hover:shadow-md hover:border-purple-200 dark:hover:border-purple-800">
        <CardHeader>
          <div className="flex items-center justify-between">
            <CardTitle className="text-lg">{name}</CardTitle>
            {Icon && <Icon className="h-6 w-6 text-purple-600 dark:text-purple-400" />}
          </div>
        </CardHeader>
        <CardContent>
          <p className="text-sm text-zinc-600 dark:text-zinc-400 mb-4">
            {description || `Manage ${name.toLowerCase()} operations`}
          </p>
          <Button
            variant="ghost"
            size="sm"
            className="gap-2 group-hover:text-purple-600 dark:group-hover:text-purple-400"
          >
            Open Dashboard
            <ArrowRight className="h-4 w-4 transition-transform group-hover:translate-x-1" />
          </Button>
        </CardContent>
      </Card>
    </Link>
  )
}
