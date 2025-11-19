'use client'

import { useState, useEffect } from 'react'
import { notificationService } from '@/lib/services/NotificationService'
import { Notification, NotificationFilter } from '@/lib/types/notifications'

export function useNotifications(filter?: NotificationFilter) {
  const [notifications, setNotifications] = useState<Notification[]>([])
  const [loading, setLoading] = useState(true)
  const [error, setError] = useState<string | null>(null)
  const [unreadCount, setUnreadCount] = useState(0)

  useEffect(() => {
    loadNotifications()
    loadUnreadCount()

    // Poll for new notifications every 30 seconds
    const interval = setInterval(() => {
      loadNotifications()
      loadUnreadCount()
    }, 30000)

    return () => clearInterval(interval)
  }, [filter])

  const loadNotifications = async () => {
    try {
      setLoading(true)
      setError(null)
      const data = await notificationService.getNotifications(filter)
      setNotifications(data)
    } catch (err) {
      setError(err instanceof Error ? err.message : 'Failed to load notifications')
    } finally {
      setLoading(false)
    }
  }

  const loadUnreadCount = async () => {
    try {
      const count = await notificationService.getUnreadCount()
      setUnreadCount(count)
    } catch (err) {
      console.error('Failed to load unread count:', err)
    }
  }

  const markAsRead = async (id: string) => {
    try {
      await notificationService.markAsRead(id)
      setNotifications((prev) =>
        prev.map((n) => (n.id === id ? { ...n, read: true } : n))
      )
      setUnreadCount((prev) => Math.max(0, prev - 1))
    } catch (err) {
      console.error('Failed to mark as read:', err)
    }
  }

  const markAllAsRead = async () => {
    try {
      await notificationService.markAllAsRead()
      setNotifications((prev) => prev.map((n) => ({ ...n, read: true })))
      setUnreadCount(0)
    } catch (err) {
      console.error('Failed to mark all as read:', err)
    }
  }

  const deleteNotification = async (id: string) => {
    try {
      await notificationService.deleteNotification(id)
      setNotifications((prev) => prev.filter((n) => n.id !== id))
      const deletedNotification = notifications.find((n) => n.id === id)
      if (deletedNotification && !deletedNotification.read) {
        setUnreadCount((prev) => Math.max(0, prev - 1))
      }
    } catch (err) {
      console.error('Failed to delete notification:', err)
    }
  }

  const requestPermission = async () => {
    return await notificationService.requestPermission()
  }

  return {
    notifications,
    loading,
    error,
    unreadCount,
    markAsRead,
    markAllAsRead,
    deleteNotification,
    requestPermission,
    refresh: loadNotifications,
  }
}
