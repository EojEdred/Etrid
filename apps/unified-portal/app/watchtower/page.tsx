'use client';

import { useState, useEffect } from 'react';
import { useChannelMonitoring } from '@/hooks/watchtower/useChannelMonitoring';
import ChannelList from '@/components/watchtower/channel-list';
import FraudAlerts from '@/components/watchtower/fraud-alerts';
import WebSocketStatus from '@/components/watchtower/websocket-status';

export default function WatchtowerDashboard() {
  const { channels, isConnected, isLoading } = useChannelMonitoring();

  // Mock alerts data for now until hook is updated
  const mockAlerts = [];

  return (
    <div className="container mx-auto p-8">
      <div className="flex items-center justify-between mb-6">
        <div>
          <h1 className="text-3xl font-bold text-zinc-900 dark:text-white">Watchtower Monitor</h1>
          <p className="text-zinc-600 dark:text-zinc-400 mt-1">
            Monitor Lightning channels and detect fraud attempts
          </p>
        </div>
        <WebSocketStatus isConnected={isConnected} />
      </div>

      <div className="grid grid-cols-1 lg:grid-cols-3 gap-6">
        <div className="lg:col-span-2">
          <ChannelList channels={channels} isLoading={isLoading} />
        </div>
        <div>
          <FraudAlerts alerts={mockAlerts} />
        </div>
      </div>
    </div>
  );
}
