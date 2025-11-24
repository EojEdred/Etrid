import Link from 'next/link'
import { Card, CardContent } from '@/components/ui/card'
import { ArrowRight } from 'lucide-react'

interface ServiceLinkCardProps {
  title: string
  description: string
  href: string
  icon?: string
}

export function ServiceLinkCard({ title, description, href, icon }: ServiceLinkCardProps) {
  return (
    <Link href={href} className="group">
      <Card className="h-full hover:shadow-xl transition-all hover:scale-105 duration-200 cursor-pointer border-2 hover:border-purple-500">
        <CardContent className="p-6">
          <div className="flex flex-col h-full">
            <div className="flex items-center gap-3 mb-3">
              {icon && <span className="text-3xl">{icon}</span>}
              <h3 className="text-lg font-semibold">{title}</h3>
            </div>
            <p className="text-sm text-muted-foreground flex-1">
              {description}
            </p>
            <div className="flex items-center gap-2 text-purple-600 dark:text-purple-400 mt-4 group-hover:gap-3 transition-all">
              <span className="text-sm font-medium">Explore</span>
              <ArrowRight className="w-4 h-4" />
            </div>
          </div>
        </CardContent>
      </Card>
    </Link>
  )
}
