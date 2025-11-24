'use client';

import { useState } from 'react';
import { useValidatorStats } from '@/hooks/validator/useValidatorStats';
import RewardHistory from '@/components/validator/reward-history';

export default function RewardsPage() {
  const [validatorAddress] = useState(process.env.NEXT_PUBLIC_VALIDATOR_ADDRESS);
  const { rewards, isLoading } = useValidatorStats(validatorAddress);

  return (
    <div className="container mx-auto p-8">
      <h1 className="text-3xl font-bold mb-6 text-zinc-900 dark:text-white">Reward Analytics</h1>
      <RewardHistory rewards={rewards} isLoading={isLoading} />
    </div>
  );
}
