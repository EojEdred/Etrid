'use client'

import dynamic from 'next/dynamic'
import { QueryProvider } from '@/components/providers/QueryProvider'
import { PageHeader } from '@/components/layout/PageHeader'

const StakingContent = dynamic(
  () => import('@/components/staking/StakingContent'),
  {
    ssr: false,
    loading: () => (
      <div className="min-h-screen gradient-bg-animated flex items-center justify-center">
        <div className="animate-pulse flex flex-col items-center">
          <div className="w-16 h-16 rounded-2xl bg-gradient-to-br from-cyan-500 via-purple-500 to-blue-500 mb-4" />
          <div className="h-4 w-32 bg-white/10 rounded mb-2" />
          <div className="h-3 w-24 bg-white/5 rounded" />
        </div>
      </div>
    )
  }
)

export default function StakingPage() {
  return (
    <QueryProvider>
      <div className="min-h-screen gradient-bg-animated">
        <PageHeader title="Staking" subtitle="Stake ETR to earn rewards" />
        <StakingContent />
      </div>
    </QueryProvider>
  )
}
