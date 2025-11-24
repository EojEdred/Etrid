'use client';

import EarningsTracker from '@/components/watchtower/earnings-tracker';

export default function EarningsPage() {
  return (
    <div className="container mx-auto p-8">
      <h1 className="text-3xl font-bold mb-6 text-zinc-900 dark:text-white">Earnings Tracking</h1>
      <EarningsTracker />
    </div>
  );
}
