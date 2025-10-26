'use client';

import { useState, useEffect, useCallback, useRef } from 'react';
import type { Channel, MonitoringStats, WebSocketMessage } from '../types';

// Mock channel data generator
const generateMockChannels = (count: number): Channel[] => {
  const statuses: Channel['status'][] = ['active', 'inactive', 'disputed', 'closed'];

  return Array.from({ length: count }, (_, i) => ({
    id: `channel-${i + 1}`,
    channelId: `0x${Math.random().toString(16).substring(2, 42)}`,
    node1: `0x${Math.random().toString(16).substring(2, 42)}`,
    node2: `0x${Math.random().toString(16).substring(2, 42)}`,
    capacity: Math.floor(Math.random() * 100000) + 10000,
    balance1: Math.floor(Math.random() * 50000),
    balance2: Math.floor(Math.random() * 50000),
    status: statuses[Math.floor(Math.random() * statuses.length)],
    lastUpdate: new Date(Date.now() - Math.random() * 86400000),
    commitmentNumber: Math.floor(Math.random() * 1000),
    watchtowerActive: Math.random() > 0.2,
  }));
};

export function useChannelMonitoring() {
  const [channels, setChannels] = useState<Channel[]>([]);
  const [stats, setStats] = useState<MonitoringStats>({
    activeChannels: 0,
    totalMonitored: 0,
    fraudsDetected: 0,
    uptime: 99.8,
    lastCheck: new Date(),
    averageResponseTime: 342,
  });
  const [isLoading, setIsLoading] = useState(true);
  const [isConnected, setIsConnected] = useState(false);
  const wsRef = useRef<WebSocket | null>(null);

  // Initialize WebSocket connection
  const connectWebSocket = useCallback(() => {
    const config = JSON.parse(localStorage.getItem('watchtower-config') || '{}');
    const wsEndpoint = config.wsEndpoint || 'ws://localhost:9944';

    try {
      // Production WebSocket connection
      const ws = new WebSocket(wsEndpoint);

      ws.onopen = () => {
        console.log('WebSocket connected to', wsEndpoint);
        setIsConnected(true);
      };

      ws.onmessage = (event) => {
        const message: WebSocketMessage = JSON.parse(event.data);
        handleWebSocketMessage(message);
      };

      ws.onerror = (error) => {
        console.error('WebSocket error:', error);
        setIsConnected(false);
      };

      ws.onclose = () => {
        console.log('WebSocket disconnected');
        setIsConnected(false);
        // Attempt to reconnect after 5 seconds
        setTimeout(connectWebSocket, 5000);
      };

      wsRef.current = ws;
    } catch (error) {
      console.error('Failed to connect WebSocket:', error);
      setIsConnected(false);
      // Fallback to demo mode if WebSocket fails
      console.warn('Falling back to demo mode');
      setIsConnected(true);
    }
  }, []);

  // Handle WebSocket messages
  const handleWebSocketMessage = useCallback((message: WebSocketMessage) => {
    switch (message.type) {
      case 'channel_update':
        updateChannel(message.payload);
        break;
      case 'fraud_alert':
        // Handle fraud alert
        break;
      case 'subscription_update':
        // Handle subscription update
        break;
      default:
        console.log('Unknown message type:', message.type);
    }
  }, []);

  // Update channel data
  const updateChannel = useCallback((channelData: Partial<Channel>) => {
    setChannels((prev) =>
      prev.map((channel) =>
        channel.channelId === channelData.channelId
          ? { ...channel, ...channelData, lastUpdate: new Date() }
          : channel
      )
    );
  }, []);

  // Fetch channels from API or blockchain node
  const fetchChannels = useCallback(async () => {
    try {
      setIsLoading(true);

      // In production, fetch from your API or blockchain node
      // const response = await fetch('/api/channels');
      // const data = await response.json();

      // For demo, generate mock data
      await new Promise((resolve) => setTimeout(resolve, 1000));
      const mockChannels = generateMockChannels(15);

      setChannels(mockChannels);

      // Calculate stats
      const activeChannels = mockChannels.filter((c) => c.status === 'active').length;
      const fraudsDetected = Math.floor(Math.random() * 20);

      setStats({
        activeChannels,
        totalMonitored: mockChannels.length,
        fraudsDetected,
        uptime: 99.8,
        lastCheck: new Date(),
        averageResponseTime: Math.floor(Math.random() * 500) + 200,
      });
    } catch (error) {
      console.error('Failed to fetch channels:', error);
    } finally {
      setIsLoading(false);
    }
  }, []);

  // Refresh channel data
  const refreshChannels = useCallback(() => {
    fetchChannels();
  }, [fetchChannels]);

  // Subscribe to a new channel
  const subscribeToChannel = useCallback(async (channelId: string, tier: string) => {
    try {
      // In production, call your API to subscribe
      // await fetch('/api/subscribe', {
      //   method: 'POST',
      //   body: JSON.stringify({ channelId, tier }),
      // });

      console.log(`Subscribing to channel ${channelId} with tier ${tier}`);

      // Refresh channels after subscription
      await fetchChannels();

      return { success: true };
    } catch (error) {
      console.error('Failed to subscribe to channel:', error);
      return { success: false, error };
    }
  }, [fetchChannels]);

  // Unsubscribe from a channel
  const unsubscribeFromChannel = useCallback(async (channelId: string) => {
    try {
      // In production, call your API to unsubscribe
      // await fetch(`/api/subscribe/${channelId}`, {
      //   method: 'DELETE',
      // });

      console.log(`Unsubscribing from channel ${channelId}`);

      // Update local state
      setChannels((prev) => prev.filter((c) => c.channelId !== channelId));

      return { success: true };
    } catch (error) {
      console.error('Failed to unsubscribe from channel:', error);
      return { success: false, error };
    }
  }, []);

  // Initialize
  useEffect(() => {
    fetchChannels();
    connectWebSocket();

    // Set up polling interval
    const config = JSON.parse(localStorage.getItem('watchtower-config') || '{}');
    const pollingInterval = config.pollingInterval || 5000;

    const intervalId = setInterval(() => {
      if (!wsRef.current || wsRef.current.readyState !== WebSocket.OPEN) {
        fetchChannels();
      }
    }, pollingInterval);

    return () => {
      clearInterval(intervalId);
      if (wsRef.current) {
        wsRef.current.close();
      }
    };
  }, [fetchChannels, connectWebSocket]);

  return {
    channels,
    stats,
    isLoading,
    isConnected,
    refreshChannels,
    subscribeToChannel,
    unsubscribeFromChannel,
  };
}
