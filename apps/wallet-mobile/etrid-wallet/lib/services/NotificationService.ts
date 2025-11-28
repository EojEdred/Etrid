import { Notification, NotificationFilter, PushToken } from '../types/notifications'

export class NotificationService {
  private static instance: NotificationService

  static getInstance(): NotificationService {
    if (!NotificationService.instance) {
      NotificationService.instance = new NotificationService()
    }
    return NotificationService.instance
  }

  async getNotifications(filter?: NotificationFilter): Promise<Notification[]> {
    try {
      const params = new URLSearchParams()
      if (filter?.type) params.append('type', filter.type)
      if (filter?.read !== undefined) params.append('read', filter.read.toString())
      if (filter?.startDate) params.append('startDate', filter.startDate.toISOString())
      if (filter?.endDate) params.append('endDate', filter.endDate.toISOString())

      const response = await fetch(`/api/notifications?${params.toString()}`)
      if (!response.ok) throw new Error('Failed to fetch notifications')
      return await response.json()
    } catch (error) {
      console.error('Error fetching notifications:', error)
      return this.getMockNotifications()
    }
  }

  async getUnreadCount(): Promise<number> {
    try {
      const response = await fetch('/api/notifications/unread-count')
      if (!response.ok) throw new Error('Failed to fetch unread count')
      const data = await response.json()
      return data.count
    } catch (error) {
      console.error('Error fetching unread count:', error)
      return 5
    }
  }

  async markAsRead(id: string): Promise<void> {
    try {
      const response = await fetch(`/api/notifications/${id}/read`, {
        method: 'POST',
      })
      if (!response.ok) throw new Error('Failed to mark as read')
    } catch (error) {
      console.error('Error marking as read:', error)
      throw error
    }
  }

  async markAllAsRead(): Promise<void> {
    try {
      const response = await fetch('/api/notifications/read-all', {
        method: 'POST',
      })
      if (!response.ok) throw new Error('Failed to mark all as read')
    } catch (error) {
      console.error('Error marking all as read:', error)
      throw error
    }
  }

  async deleteNotification(id: string): Promise<void> {
    try {
      const response = await fetch(`/api/notifications/${id}`, {
        method: 'DELETE',
      })
      if (!response.ok) throw new Error('Failed to delete notification')
    } catch (error) {
      console.error('Error deleting notification:', error)
      throw error
    }
  }

  async registerPushToken(token: string, platform: 'ios' | 'android' | 'web'): Promise<void> {
    try {
      const response = await fetch('/api/notifications/push-token', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ token, platform }),
      })
      if (!response.ok) throw new Error('Failed to register push token')
    } catch (error) {
      console.error('Error registering push token:', error)
      throw error
    }
  }

  async requestPermission(): Promise<boolean> {
    if (!('Notification' in window)) {
      console.warn('This browser does not support notifications')
      return false
    }

    if (Notification.permission === 'granted') {
      return true
    }

    if (Notification.permission !== 'denied') {
      const permission = await Notification.requestPermission()
      return permission === 'granted'
    }

    return false
  }

  async sendLocalNotification(title: string, body: string, options?: NotificationOptions): Promise<void> {
    if (Notification.permission === 'granted') {
      new Notification(title, {
        body,
        icon: '/icon.png',
        ...options,
      })
    }
  }

  private getMockNotifications(): Notification[] {
    const now = new Date()
    return [
      {
        id: '1',
        userId: 'user-1',
        type: 'price',
        title: 'Price Alert: ÉTR',
        message: 'ÉTR has crossed $50.00',
        read: false,
        createdAt: new Date(now.getTime() - 5 * 60 * 1000),
      },
      {
        id: '2',
        userId: 'user-1',
        type: 'whale',
        title: 'Whale Alert',
        message: '1M ÉTR moved to exchange',
        read: false,
        createdAt: new Date(now.getTime() - 15 * 60 * 1000),
      },
      {
        id: '3',
        userId: 'user-1',
        type: 'governance',
        title: 'New Proposal',
        message: 'Proposal #42: Increase validator rewards',
        read: true,
        createdAt: new Date(now.getTime() - 2 * 60 * 60 * 1000),
      },
      {
        id: '4',
        userId: 'user-1',
        type: 'staking',
        title: 'Staking Rewards',
        message: 'You received 10 ÉTR in staking rewards',
        read: true,
        createdAt: new Date(now.getTime() - 24 * 60 * 60 * 1000),
      },
      {
        id: '5',
        userId: 'user-1',
        type: 'security',
        title: 'New Login Detected',
        message: 'Login from new device (Chrome on Windows)',
        read: false,
        createdAt: new Date(now.getTime() - 3 * 24 * 60 * 60 * 1000),
      },
    ]
  }
}

export const notificationService = NotificationService.getInstance()
