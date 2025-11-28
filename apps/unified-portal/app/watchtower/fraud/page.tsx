'use client';

import { useChannelMonitoring } from '@/hooks/watchtower/useChannelMonitoring';
import FraudAlerts from '@/components/watchtower/fraud-alerts';

export default function FraudPage() {
  const { isLoading } = useChannelMonitoring();

  // Mock alerts data for now until hook is updated
  const mockAlerts = [];

  return (
    <div className="container mx-auto p-8">
      <h1 className="text-3xl font-bold mb-6 text-zinc-900 dark:text-white">Fraud Detection</h1>
      <FraudAlerts alerts={mockAlerts} />
    </div>
  );
}
