'use client'

import dynamic from 'next/dynamic'

const GovernanceContent = dynamic(
  () => import('@/components/governance/GovernanceContent'),
  {
    ssr: false,
    loading: () => (
      <div className="min-h-screen gradient-bg-animated flex items-center justify-center">
        <div className="animate-pulse flex flex-col items-center">
          <div className="w-16 h-16 rounded-2xl bg-gradient-to-br from-cyan-500 via-purple-500 to-blue-500 mb-4" />
          <div className="h-4 w-32 bg-white/10 rounded" />
        </div>
      </div>
    )
  }
)

export default function GovernancePage() {
  return <GovernanceContent />
}
