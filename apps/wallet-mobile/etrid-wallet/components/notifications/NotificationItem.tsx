'use client'

import { Notification } from '@/lib/types/notifications'
import { Card } from '@/components/ui/card'
import { Button } from '@/components/ui/button'
import {
  DollarSign,
  Whale,
  Vote,
  Shield,
  TrendingUp,
  Bell,
  Trash2,
} from 'lucide-react'
import { formatDistanceToNow } from 'date-fns'

interface NotificationItemProps {
  notification: Notification
  onRead?: (id: string) => void
  onDelete?: (id: string) => void
}

export function NotificationItem({
  notification,
  onRead,
  onDelete,
}: NotificationItemProps) {
  const getIcon = () => {
    switch (notification.type) {
      case 'price':
        return <DollarSign className="w-5 h-5" />
      case 'whale':
        return <Whale className="w-5 h-5" />
      case 'governance':
        return <Vote className="w-5 h-5" />
      case 'security':
        return <Shield className="w-5 h-5" />
      case 'staking':
        return <TrendingUp className="w-5 h-5" />
      default:
        return <Bell className="w-5 h-5" />
    }
  }

  const getIconColor = () => {
    switch (notification.type) {
      case 'price':
        return 'text-green-500 bg-green-500/10'
      case 'whale':
        return 'text-blue-500 bg-blue-500/10'
      case 'governance':
        return 'text-purple-500 bg-purple-500/10'
      case 'security':
        return 'text-red-500 bg-red-500/10'
      case 'staking':
        return 'text-yellow-500 bg-yellow-500/10'
      default:
        return 'text-gray-500 bg-gray-500/10'
    }
  }

  const handleClick = () => {
    if (!notification.read && onRead) {
      onRead(notification.id)
    }
  }

  return (
    <Card
      className={`p-4 cursor-pointer transition-colors ${
        !notification.read ? 'bg-accent/5 border-accent/20' : ''
      }`}
      onClick={handleClick}
    >
      <div className="flex items-start gap-3">
        <div className={`w-10 h-10 rounded-full flex items-center justify-center ${getIconColor()}`}>
          {getIcon()}
        </div>

        <div className="flex-1 min-w-0">
          <div className="flex items-start justify-between gap-2 mb-1">
            <h4 className="font-semibold text-sm">{notification.title}</h4>
            {!notification.read && (
              <div className="w-2 h-2 rounded-full bg-accent flex-shrink-0 mt-1" />
            )}
          </div>

          <p className="text-sm text-muted-foreground mb-2">
            {notification.message}
          </p>

          <div className="flex items-center justify-between">
            <span className="text-xs text-muted-foreground">
              {formatDistanceToNow(notification.createdAt, { addSuffix: true })}
            </span>

            {onDelete && (
              <Button
                variant="ghost"
                size="sm"
                onClick={(e) => {
                  e.stopPropagation()
                  onDelete(notification.id)
                }}
                className="h-auto p-1"
              >
                <Trash2 className="w-4 h-4 text-muted-foreground hover:text-red-500" />
              </Button>
            )}
          </div>
        </div>
      </div>
    </Card>
  )
}
