'use client'

import { ArrowLeft, CheckCheck, Filter } from 'lucide-react'
import { Button } from '@/components/ui/button'
import { useNotifications } from '@/hooks/useNotifications'
import { NotificationItem } from '@/components/notifications/NotificationItem'
import { NotificationType } from '@/lib/types/notifications'
import { useState } from 'react'

interface NotificationCenterScreenProps {
  onBack: () => void
}

export function NotificationCenterScreen({ onBack }: NotificationCenterScreenProps) {
  const [filterType, setFilterType] = useState<NotificationType | 'all'>('all')
  const [filterRead, setFilterRead] = useState<boolean | undefined>(undefined)

  const filter = {
    ...(filterType !== 'all' && { type: filterType }),
    ...(filterRead !== undefined && { read: filterRead }),
  }

  const {
    notifications,
    loading,
    unreadCount,
    markAsRead,
    markAllAsRead,
    deleteNotification,
  } = useNotifications(filter)

  if (loading) {
    return (
      <div className="min-h-screen flex items-center justify-center">
        <div className="text-center">
          <div className="animate-spin rounded-full h-12 w-12 border-b-2 border-accent mx-auto mb-4" />
          <p className="text-muted-foreground">Loading notifications...</p>
        </div>
      </div>
    )
  }

  const filteredNotifications = notifications.filter((n) => {
    if (filterType !== 'all' && n.type !== filterType) return false
    if (filterRead === true && !n.read) return false
    if (filterRead === false && n.read) return false
    return true
  })

  return (
    <div className="min-h-screen pb-6">
      {/* Header */}
      <header className="sticky top-0 z-10 bg-background/95 backdrop-blur supports-[backdrop-filter]:bg-background/60 border-b">
        <div className="flex items-center justify-between p-6">
          <div className="flex items-center gap-4">
            <Button variant="ghost" size="icon" onClick={onBack}>
              <ArrowLeft className="w-5 h-5" />
            </Button>
            <div>
              <h1 className="text-2xl font-bold">Notifications</h1>
              {unreadCount > 0 && (
                <p className="text-sm text-muted-foreground">
                  {unreadCount} unread
                </p>
              )}
            </div>
          </div>

          {unreadCount > 0 && (
            <Button
              variant="ghost"
              size="sm"
              onClick={markAllAsRead}
              className="gap-2"
            >
              <CheckCheck className="w-4 h-4" />
              Mark all read
            </Button>
          )}
        </div>

        {/* Filter Tabs */}
        <div className="flex gap-2 px-6 pb-4 overflow-x-auto">
          {(['all', 'price', 'whale', 'governance', 'staking', 'security'] as const).map(
            (type) => (
              <Button
                key={type}
                variant={filterType === type ? 'default' : 'outline'}
                size="sm"
                onClick={() => setFilterType(type)}
                className="capitalize whitespace-nowrap"
              >
                {type}
              </Button>
            )
          )}
        </div>

        {/* Read/Unread Filter */}
        <div className="flex gap-2 px-6 pb-4">
          <Button
            variant={filterRead === undefined ? 'default' : 'outline'}
            size="sm"
            onClick={() => setFilterRead(undefined)}
          >
            All
          </Button>
          <Button
            variant={filterRead === false ? 'default' : 'outline'}
            size="sm"
            onClick={() => setFilterRead(false)}
          >
            Unread
          </Button>
          <Button
            variant={filterRead === true ? 'default' : 'outline'}
            size="sm"
            onClick={() => setFilterRead(true)}
          >
            Read
          </Button>
        </div>
      </header>

      <main className="px-6 space-y-3 mt-6">
        {filteredNotifications.length === 0 ? (
          <div className="text-center py-12">
            <div className="w-16 h-16 rounded-full bg-muted flex items-center justify-center mx-auto mb-4">
              <Filter className="w-8 h-8 text-muted-foreground" />
            </div>
            <p className="text-muted-foreground">No notifications found</p>
          </div>
        ) : (
          filteredNotifications.map((notification) => (
            <NotificationItem
              key={notification.id}
              notification={notification}
              onRead={markAsRead}
              onDelete={deleteNotification}
            />
          ))
        )}
      </main>
    </div>
  )
}
