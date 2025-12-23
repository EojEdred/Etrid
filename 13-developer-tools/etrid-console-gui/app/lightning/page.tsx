'use client'

import dynamic from 'next/dynamic'
import { PageHeader } from '@/components/layout/PageHeader'

const LightningContent = dynamic(
  () => import('@/components/lightning/LightningContent'),
  {
    ssr: false,
    loading: () => (
      <div className="min-h-screen gradient-bg-animated flex items-center justify-center">
        <div className="animate-pulse flex flex-col items-center">
          <div className="w-16 h-16 rounded-2xl bg-gradient-to-br from-yellow-500 via-orange-500 to-amber-500 mb-4" />
          <div className="h-4 w-32 bg-white/10 rounded" />
        </div>
      </div>
    )
  }
)

export default function LightningPage() {
  return (
    <div className="min-h-screen gradient-bg-animated">
      <PageHeader title="Lightning" subtitle="Fast payments" />
      <LightningContent />
    </div>
  )
}
