import { Card } from "@/components/ui/card"
import { Calendar, BookOpen, History, Gift } from "lucide-react"
import Link from "next/link"

export default function Sidebar() {
  return (
    <div className="space-y-6 sticky top-24">
      {/* Upcoming Events */}
      <Card className="p-6">
        <div className="flex items-center gap-2 mb-4">
          <Calendar className="w-5 h-5 text-accent" />
          <h3 className="font-bold">Upcoming Events</h3>
        </div>
        <div className="space-y-3 text-sm">
          <div>
            <p className="text-muted-foreground">Next Consensus Day</p>
            <p className="font-semibold">June 1, 2027</p>
          </div>
          <div>
            <p className="text-muted-foreground">Proposal submission deadline</p>
            <p className="font-semibold">May 1, 2026</p>
          </div>
        </div>
      </Card>

      {/* Resources */}
      <Card className="p-6">
        <div className="flex items-center gap-2 mb-4">
          <BookOpen className="w-5 h-5 text-accent" />
          <h3 className="font-bold">Resources</h3>
        </div>
        <div className="space-y-2">
          <Link
            href="#"
            className="flex items-center gap-2 text-sm text-muted-foreground hover:text-accent transition-colors"
          >
            <span>How voting works</span>
          </Link>
          <Link
            href="#"
            className="flex items-center gap-2 text-sm text-muted-foreground hover:text-accent transition-colors"
          >
            <span>Proposal guidelines</span>
          </Link>
          <Link
            href="#"
            className="flex items-center gap-2 text-sm text-muted-foreground hover:text-accent transition-colors"
          >
            <History className="w-4 h-4" />
            <span>Past Consensus Days</span>
          </Link>
        </div>
      </Card>

      {/* Rewards */}
      <Card className="p-6 bg-accent/5 border-accent/20">
        <div className="flex items-center gap-2 mb-4">
          <Gift className="w-5 h-5 text-accent" />
          <h3 className="font-bold">Rewards</h3>
        </div>
        <p className="text-sm text-muted-foreground mb-2">Earn distribution pay by voting!</p>
        <p className="text-2xl font-bold text-accent mb-4">~50 ÉTR</p>
        <p className="text-xs text-muted-foreground">Estimated reward for participating</p>
      </Card>
    </div>
  )
}
