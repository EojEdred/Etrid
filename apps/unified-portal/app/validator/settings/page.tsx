'use client';

import { useState } from 'react';
import { useValidatorStats } from '@/hooks/validator/useValidatorStats';
import CommissionSettings from '@/components/validator/commission-settings';

export default function ValidatorSettingsPage() {
  const [validatorAddress] = useState(process.env.NEXT_PUBLIC_VALIDATOR_ADDRESS);
  const { validatorInfo, isLoading } = useValidatorStats(validatorAddress);

  return (
    <div className="container mx-auto p-8">
      <h1 className="text-3xl font-bold mb-6 text-zinc-900 dark:text-white">Validator Settings</h1>
      <CommissionSettings
        currentCommission={validatorInfo?.commission || 0}
        isLoading={isLoading}
      />
    </div>
  );
}
