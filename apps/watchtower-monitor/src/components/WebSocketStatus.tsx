'use client';

import React, { useEffect, useState } from 'react';
import { Wifi, WifiOff, AlertCircle, Loader2 } from 'lucide-react';
import type { WebSocketStatus as WSStatus } from '../lib/websocket';

interface WebSocketStatusProps {
  status: WSStatus;
  compact?: boolean;
}

export default function WebSocketStatus({ status, compact = false }: WebSocketStatusProps) {
  const [retryCount, setRetryCount] = useState(0);

  const getStatusConfig = (status: WSStatus) => {
    switch (status) {
      case 'connected':
        return {
          icon: Wifi,
          color: 'text-green-400',
          bgColor: 'bg-green-500/20',
          borderColor: 'border-green-500/30',
          label: 'Connected',
          description: 'Real-time updates active',
        };
      case 'connecting':
        return {
          icon: Loader2,
          color: 'text-blue-400',
          bgColor: 'bg-blue-500/20',
          borderColor: 'border-blue-500/30',
          label: 'Connecting',
          description: 'Establishing connection...',
          animate: true,
        };
      case 'disconnected':
        return {
          icon: WifiOff,
          color: 'text-yellow-400',
          bgColor: 'bg-yellow-500/20',
          borderColor: 'border-yellow-500/30',
          label: 'Disconnected',
          description: 'WebSocket connection lost',
        };
      case 'error':
        return {
          icon: AlertCircle,
          color: 'text-red-400',
          bgColor: 'bg-red-500/20',
          borderColor: 'border-red-500/30',
          label: 'Error',
          description: 'Connection failed',
        };
    }
  };

  const config = getStatusConfig(status);
  const Icon = config.icon;

  if (compact) {
    return (
      <div className={`flex items-center gap-2 px-3 py-1.5 ${config.bgColor} rounded-lg border ${config.borderColor}`}>
        <Icon
          className={`w-4 h-4 ${config.color} ${config.animate ? 'animate-spin' : ''}`}
        />
        <span className={`text-sm font-medium ${config.color}`}>{config.label}</span>
      </div>
    );
  }

  return (
    <div className={`p-4 ${config.bgColor} rounded-xl border ${config.borderColor}`}>
      <div className="flex items-start gap-3">
        <Icon
          className={`w-5 h-5 ${config.color} mt-0.5 ${config.animate ? 'animate-spin' : ''}`}
        />
        <div className="flex-1">
          <div className="flex items-center justify-between mb-1">
            <h3 className={`font-semibold ${config.color}`}>{config.label}</h3>
            {status === 'connected' && (
              <div className="flex items-center gap-1">
                <div className="w-2 h-2 bg-green-400 rounded-full animate-pulse" />
              </div>
            )}
          </div>
          <p className="text-sm text-gray-400">{config.description}</p>

          {status === 'disconnected' && (
            <button
              onClick={() => window.location.reload()}
              className="mt-2 text-xs text-blue-400 hover:text-blue-300 underline"
            >
              Retry Connection
            </button>
          )}

          {status === 'error' && (
            <div className="mt-2">
              <p className="text-xs text-gray-500">
                Please check your network connection and node configuration.
              </p>
              <button
                onClick={() => window.location.reload()}
                className="mt-1 text-xs text-blue-400 hover:text-blue-300 underline"
              >
                Reload Page
              </button>
            </div>
          )}
        </div>
      </div>
    </div>
  );
}
