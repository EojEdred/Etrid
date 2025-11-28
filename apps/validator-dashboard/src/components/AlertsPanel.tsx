import React, { useState } from 'react';
import {
  Bell,
  AlertTriangle,
  AlertCircle,
  Info,
  CheckCircle,
  X,
  ExternalLink,
} from 'lucide-react';
import type { Alert } from '@/types';
import { formatTimeAgo } from '@/utils/format';

interface AlertsPanelProps {
  alerts?: Alert[];
  onDismiss?: (alertId: string) => void;
  onMarkRead?: (alertId: string) => void;
  isLoading?: boolean;
}

// Mock alerts for demonstration
const mockAlerts: Alert[] = [
  {
    id: '1',
    type: 'warning',
    title: 'Low Nominator Count',
    message: 'Your validator has fewer than 20 active nominators. Consider lowering commission to attract more stake.',
    timestamp: Date.now() - 2 * 60 * 60 * 1000,
    read: false,
  },
  {
    id: '2',
    type: 'info',
    title: 'New Era Started',
    message: 'Era 1245 has begun. Your validator earned 125.5 ETRID in the previous era.',
    timestamp: Date.now() - 6 * 60 * 60 * 1000,
    read: false,
  },
  {
    id: '3',
    type: 'success',
    title: 'Block Production Optimal',
    message: 'Your validator is performing excellently with 99.8% uptime over the last 30 days.',
    timestamp: Date.now() - 12 * 60 * 60 * 1000,
    read: true,
  },
  {
    id: '4',
    type: 'error',
    title: 'Missed Block',
    message: 'Your validator missed a block at height 12,345,678. Check your node connectivity.',
    timestamp: Date.now() - 24 * 60 * 60 * 1000,
    read: true,
    action: {
      label: 'View Logs',
      href: '/performance#logs',
    },
  },
];

export default function AlertsPanel({
  alerts = mockAlerts,
  onDismiss,
  onMarkRead,
  isLoading = false,
}: AlertsPanelProps) {
  const [filter, setFilter] = useState<'all' | 'unread'>('all');

  const filteredAlerts = alerts.filter((alert) => {
    if (filter === 'unread') return !alert.read;
    return true;
  });

  const unreadCount = alerts.filter((a) => !a.read).length;

  const getAlertIcon = (type: Alert['type']) => {
    switch (type) {
      case 'error':
        return <AlertCircle className="w-5 h-5" />;
      case 'warning':
        return <AlertTriangle className="w-5 h-5" />;
      case 'success':
        return <CheckCircle className="w-5 h-5" />;
      default:
        return <Info className="w-5 h-5" />;
    }
  };

  const getAlertStyles = (type: Alert['type']) => {
    switch (type) {
      case 'error':
        return {
          bg: 'bg-danger-50',
          border: 'border-danger-200',
          icon: 'text-danger-600',
          title: 'text-danger-900',
        };
      case 'warning':
        return {
          bg: 'bg-warning-50',
          border: 'border-warning-200',
          icon: 'text-warning-600',
          title: 'text-warning-900',
        };
      case 'success':
        return {
          bg: 'bg-success-50',
          border: 'border-success-200',
          icon: 'text-success-600',
          title: 'text-success-900',
        };
      default:
        return {
          bg: 'bg-blue-50',
          border: 'border-blue-200',
          icon: 'text-blue-600',
          title: 'text-blue-900',
        };
    }
  };

  if (isLoading) {
    return (
      <div className="bg-white rounded-lg shadow-md p-6">
        <div className="animate-pulse space-y-4">
          <div className="h-6 bg-gray-200 rounded w-1/4"></div>
          {[...Array(3)].map((_, i) => (
            <div key={i} className="h-24 bg-gray-200 rounded"></div>
          ))}
        </div>
      </div>
    );
  }

  return (
    <div className="bg-white rounded-lg shadow-md overflow-hidden">
      <div className="p-6 border-b border-gray-200">
        <div className="flex items-center justify-between mb-4">
          <div className="flex items-center space-x-3">
            <div className="relative">
              <Bell className="w-6 h-6 text-gray-700" />
              {unreadCount > 0 && (
                <span className="absolute -top-1 -right-1 bg-danger-500 text-white text-xs font-bold rounded-full w-5 h-5 flex items-center justify-center">
                  {unreadCount}
                </span>
              )}
            </div>
            <div>
              <h2 className="text-xl font-bold text-gray-900">Alerts & Notifications</h2>
              <p className="text-sm text-gray-600">
                {unreadCount > 0
                  ? `${unreadCount} unread notification${unreadCount === 1 ? '' : 's'}`
                  : 'All caught up!'}
              </p>
            </div>
          </div>

          <div className="flex space-x-2">
            <button
              onClick={() => setFilter('all')}
              className={`px-3 py-1 text-sm font-medium rounded-lg transition-colors ${
                filter === 'all'
                  ? 'bg-etrid-600 text-white'
                  : 'bg-gray-100 text-gray-600 hover:bg-gray-200'
              }`}
            >
              All
            </button>
            <button
              onClick={() => setFilter('unread')}
              className={`px-3 py-1 text-sm font-medium rounded-lg transition-colors ${
                filter === 'unread'
                  ? 'bg-etrid-600 text-white'
                  : 'bg-gray-100 text-gray-600 hover:bg-gray-200'
              }`}
            >
              Unread ({unreadCount})
            </button>
          </div>
        </div>
      </div>

      <div className="divide-y divide-gray-200 max-h-[600px] overflow-y-auto">
        {filteredAlerts.length === 0 ? (
          <div className="p-12 text-center">
            <Bell className="w-16 h-16 text-gray-300 mx-auto mb-4" />
            <p className="text-lg font-medium text-gray-600">No alerts</p>
            <p className="text-sm text-gray-500 mt-1">
              {filter === 'unread'
                ? 'All notifications have been read'
                : 'Your validator is running smoothly'}
            </p>
          </div>
        ) : (
          filteredAlerts.map((alert) => {
            const styles = getAlertStyles(alert.type);
            return (
              <div
                key={alert.id}
                className={`p-4 transition-colors ${
                  alert.read ? 'bg-white' : 'bg-gray-50'
                } hover:bg-gray-100`}
              >
                <div className="flex items-start space-x-3">
                  <div className={`p-2 rounded-lg ${styles.bg} border ${styles.border}`}>
                    <div className={styles.icon}>{getAlertIcon(alert.type)}</div>
                  </div>

                  <div className="flex-1 min-w-0">
                    <div className="flex items-start justify-between">
                      <div className="flex-1">
                        <div className="flex items-center space-x-2 mb-1">
                          <h3 className={`text-sm font-semibold ${styles.title}`}>
                            {alert.title}
                          </h3>
                          {!alert.read && (
                            <span className="inline-block w-2 h-2 bg-etrid-600 rounded-full"></span>
                          )}
                        </div>
                        <p className="text-sm text-gray-700 leading-relaxed">{alert.message}</p>
                        <div className="flex items-center space-x-4 mt-2">
                          <span className="text-xs text-gray-500">
                            {formatTimeAgo(alert.timestamp)}
                          </span>
                          {alert.action && (
                            <a
                              href={alert.action.href}
                              className="text-xs font-medium text-etrid-600 hover:text-etrid-700 flex items-center space-x-1"
                            >
                              <span>{alert.action.label}</span>
                              <ExternalLink className="w-3 h-3" />
                            </a>
                          )}
                        </div>
                      </div>

                      <div className="flex items-center space-x-2 ml-4">
                        {!alert.read && onMarkRead && (
                          <button
                            onClick={() => onMarkRead(alert.id)}
                            className="p-1 text-gray-400 hover:text-etrid-600 rounded transition-colors"
                            title="Mark as read"
                          >
                            <CheckCircle className="w-4 h-4" />
                          </button>
                        )}
                        {onDismiss && (
                          <button
                            onClick={() => onDismiss(alert.id)}
                            className="p-1 text-gray-400 hover:text-danger-600 rounded transition-colors"
                            title="Dismiss"
                          >
                            <X className="w-4 h-4" />
                          </button>
                        )}
                      </div>
                    </div>
                  </div>
                </div>
              </div>
            );
          })
        )}
      </div>

      {filteredAlerts.length > 0 && (
        <div className="p-4 bg-gray-50 border-t border-gray-200">
          <div className="flex items-center justify-between">
            <p className="text-sm text-gray-600">
              Showing {filteredAlerts.length} of {alerts.length} alerts
            </p>
            {unreadCount > 0 && onMarkRead && (
              <button
                onClick={() => alerts.filter((a) => !a.read).forEach((a) => onMarkRead(a.id))}
                className="text-sm font-medium text-etrid-600 hover:text-etrid-700"
              >
                Mark all as read
              </button>
            )}
          </div>
        </div>
      )}
    </div>
  );
}
