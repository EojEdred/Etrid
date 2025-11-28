'use client';

import { useState } from 'react';
import { useValidatorStats } from '@/hooks/validator/useValidatorStats';
import NominatorList from '@/components/validator/nominator-list';

export default function NominatorsPage() {
  const [validatorAddress] = useState(process.env.NEXT_PUBLIC_VALIDATOR_ADDRESS);
  const { nominators, isLoading, refreshData } = useValidatorStats(validatorAddress);

  return (
    <div className="container mx-auto p-8">
      <h1 className="text-3xl font-bold mb-6 text-zinc-900 dark:text-white">Nominator Management</h1>
      <NominatorList nominators={nominators} isLoading={isLoading} />
    </div>
  );
}
