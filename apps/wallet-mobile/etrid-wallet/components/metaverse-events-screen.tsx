'use client'

import { ArrowLeft, Calendar as CalendarIcon } from 'lucide-react'
import { Button } from '@/components/ui/button'
import { useMetaverseEvents } from '@/hooks/useMetaverse'
import { EventCard } from '@/components/metaverse/EventCard'

interface MetaverseEventsScreenProps {
  onBack: () => void
}

export function MetaverseEventsScreen({ onBack }: MetaverseEventsScreenProps) {
  const { events, loading, rsvpEvent } = useMetaverseEvents()

  if (loading) {
    return (
      <div className="min-h-screen flex items-center justify-center">
        <div className="text-center">
          <div className="animate-spin rounded-full h-12 w-12 border-b-2 border-accent mx-auto mb-4" />
          <p className="text-muted-foreground">Loading events...</p>
        </div>
      </div>
    )
  }

  const upcomingEvents = events.filter(
    (e) => new Date(e.startDate) > new Date()
  )
  const pastEvents = events.filter((e) => new Date(e.endDate) < new Date())

  return (
    <div className="min-h-screen pb-6">
      {/* Header */}
      <header className="sticky top-0 z-10 bg-background/95 backdrop-blur supports-[backdrop-filter]:bg-background/60 border-b">
        <div className="flex items-center gap-4 p-6">
          <Button variant="ghost" size="icon" onClick={onBack}>
            <ArrowLeft className="w-5 h-5" />
          </Button>
          <div>
            <h1 className="text-2xl font-bold">Metaverse Events</h1>
            <p className="text-sm text-muted-foreground">
              {upcomingEvents.length} upcoming events
            </p>
          </div>
        </div>
      </header>

      <main className="px-6 space-y-6 mt-6">
        {/* Upcoming Events */}
        {upcomingEvents.length > 0 && (
          <div>
            <h3 className="text-lg font-semibold mb-3 flex items-center gap-2">
              <CalendarIcon className="w-5 h-5" />
              Upcoming Events
            </h3>
            <div className="grid gap-4">
              {upcomingEvents.map((event) => (
                <EventCard key={event.id} event={event} onRSVP={rsvpEvent} />
              ))}
            </div>
          </div>
        )}

        {/* Past Events */}
        {pastEvents.length > 0 && (
          <div>
            <h3 className="text-lg font-semibold mb-3">Past Events</h3>
            <div className="grid gap-4 opacity-60">
              {pastEvents.map((event) => (
                <EventCard key={event.id} event={event} />
              ))}
            </div>
          </div>
        )}

        {events.length === 0 && (
          <div className="text-center py-12">
            <CalendarIcon className="w-16 h-16 mx-auto mb-4 text-muted-foreground" />
            <p className="text-muted-foreground">No events found</p>
          </div>
        )}
      </main>
    </div>
  )
}
