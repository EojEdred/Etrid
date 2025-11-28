'use client';

import SubscriptionManager from '@/components/watchtower/subscription-manager';

export default function WatchtowerSettingsPage() {
  return (
    <div className="container mx-auto p-8">
      <h1 className="text-3xl font-bold mb-6 text-zinc-900 dark:text-white">Watchtower Settings</h1>
      <SubscriptionManager />
    </div>
  );
}
