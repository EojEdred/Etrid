'use client';

import { useState } from 'react';
import { useValidatorStats } from '@/hooks/validator/useValidatorStats';
import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card';

export default function PerformancePage() {
  const [validatorAddress] = useState(process.env.NEXT_PUBLIC_VALIDATOR_ADDRESS);
  const { performance, isLoading } = useValidatorStats(validatorAddress);

  if (isLoading) {
    return (
      <div className="container mx-auto p-8">
        <h1 className="text-3xl font-bold mb-6 text-zinc-900 dark:text-white">Performance Metrics</h1>
        <div className="animate-pulse">Loading...</div>
      </div>
    );
  }

  return (
    <div className="container mx-auto p-8">
      <h1 className="text-3xl font-bold mb-6 text-zinc-900 dark:text-white">Performance Metrics</h1>

      <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
        <Card>
          <CardHeader>
            <CardTitle>Blocks Produced</CardTitle>
          </CardHeader>
          <CardContent>
            <p className="text-3xl font-bold text-zinc-900 dark:text-white">{performance?.blocksProduced || 0}</p>
          </CardContent>
        </Card>

        <Card>
          <CardHeader>
            <CardTitle>Uptime</CardTitle>
          </CardHeader>
          <CardContent>
            <p className="text-3xl font-bold text-zinc-900 dark:text-white">{performance?.uptime.toFixed(2)}%</p>
          </CardContent>
        </Card>

        <Card>
          <CardHeader>
            <CardTitle>Rank</CardTitle>
          </CardHeader>
          <CardContent>
            <p className="text-3xl font-bold text-zinc-900 dark:text-white">
              #{performance?.rank} / {performance?.totalValidators}
            </p>
          </CardContent>
        </Card>
      </div>
    </div>
  );
}
