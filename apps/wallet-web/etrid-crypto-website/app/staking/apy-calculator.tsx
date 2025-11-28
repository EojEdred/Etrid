'use client';

import React from 'react';
import Link from 'next/link';
import { ArrowLeft } from 'lucide-react';
import { Button } from '@/components/ui/button';
import { APYCalculator } from '@/components/staking/apy-calculator';

export default function APYCalculatorPage() {
  return (
    <div className="min-h-screen bg-gradient-to-b from-background to-muted/20 p-4 md:p-8">
      <div className="max-w-5xl mx-auto space-y-6">
        {/* Header */}
        <div className="flex items-center gap-4">
          <Link href="/staking/nominator-dashboard">
            <Button variant="ghost" size="icon">
              <ArrowLeft className="h-5 w-5" />
            </Button>
          </Link>
          <div className="flex-1">
            <h1 className="text-4xl font-bold tracking-tight">APY Calculator</h1>
            <p className="text-muted-foreground mt-2">
              Estimate your potential staking rewards with customizable parameters
            </p>
          </div>
        </div>

        {/* Calculator Component */}
        <APYCalculator />
      </div>
    </div>
  );
}
