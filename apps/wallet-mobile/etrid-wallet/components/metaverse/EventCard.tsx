'use client'

import { Card } from '@/components/ui/card'
import { Button } from '@/components/ui/button'
import { MetaverseEvent } from '@/lib/types/metaverse'
import { Calendar, Users, Award, Check } from 'lucide-react'
import { format } from 'date-fns'

interface EventCardProps {
  event: MetaverseEvent
  onRSVP?: (id: string) => void
}

export function EventCard({ event, onRSVP }: EventCardProps) {
  const getCategoryColor = (category: string): string => {
    switch (category) {
      case 'concert':
        return 'bg-purple-500/10 text-purple-500'
      case 'exhibition':
        return 'bg-blue-500/10 text-blue-500'
      case 'conference':
        return 'bg-green-500/10 text-green-500'
      case 'game':
        return 'bg-orange-500/10 text-orange-500'
      default:
        return 'bg-gray-500/10 text-gray-500'
    }
  }

  return (
    <Card className="p-4">
      <div className="aspect-video rounded-lg overflow-hidden mb-3 bg-gradient-to-br from-accent/20 to-primary/20 flex items-center justify-center">
        <Calendar className="w-12 h-12 text-accent" />
      </div>

      <div className="mb-3">
        <div className="flex items-start justify-between mb-2">
          <h4 className="font-semibold flex-1">{event.name}</h4>
          <span
            className={`text-xs px-2 py-1 rounded capitalize ${getCategoryColor(
              event.category
            )}`}
          >
            {event.category}
          </span>
        </div>

        <p className="text-xs text-muted-foreground mb-2">
          {event.description}
        </p>

        <div className="flex items-center gap-4 text-xs text-muted-foreground">
          <div className="flex items-center gap-1">
            <Calendar className="w-3 h-3" />
            {format(event.startDate, 'MMM d, h:mm a')}
          </div>
          <div className="flex items-center gap-1">
            <Users className="w-3 h-3" />
            {event.rsvpCount} attending
          </div>
        </div>
      </div>

      {event.rewards && event.rewards.length > 0 && (
        <div className="mb-3 p-2 bg-accent/10 rounded">
          <div className="flex items-center gap-2 text-xs">
            <Award className="w-4 h-4 text-accent" />
            <span className="font-semibold">
              {event.rewards.length} Reward{event.rewards.length > 1 ? 's' : ''}
            </span>
          </div>
        </div>
      )}

      <Button
        variant={event.userRSVP ? 'outline' : 'default'}
        size="sm"
        className="w-full"
        onClick={() => onRSVP?.(event.id)}
        disabled={event.userRSVP}
      >
        {event.userRSVP ? (
          <>
            <Check className="w-4 h-4 mr-2" />
            You're attending
          </>
        ) : (
          'RSVP to Event'
        )}
      </Button>
    </Card>
  )
}
