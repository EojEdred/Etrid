'use client';

import { useChannelMonitoring } from '@/hooks/watchtower/useChannelMonitoring';
import ChannelList from '@/components/watchtower/channel-list';

export default function ChannelsPage() {
  const { channels, isLoading } = useChannelMonitoring();

  return (
    <div className="container mx-auto p-8">
      <h1 className="text-3xl font-bold mb-6 text-zinc-900 dark:text-white">Channel Monitoring</h1>
      <ChannelList channels={channels} isLoading={isLoading} />
    </div>
  );
}
