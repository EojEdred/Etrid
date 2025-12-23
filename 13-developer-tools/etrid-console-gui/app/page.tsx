'use client'

import dynamic from 'next/dynamic'

const ConsoleDashboard = dynamic(
  () => import('@/components/console/ConsoleDashboard'),
  {
    ssr: false,
    loading: () => (
      <div className="min-h-screen gradient-bg-animated flex items-center justify-center">
        <div className="animate-pulse flex flex-col items-center">
          <div className="w-16 h-16 rounded-2xl bg-gradient-to-br from-cyan-500 via-purple-500 to-pink-500 mb-4" />
          <div className="h-4 w-48 bg-white/10 rounded" />
          <div className="h-4 w-32 bg-white/10 rounded mt-2" />
        </div>
      </div>
    )
  }
)

export default function Home() {
  return <ConsoleDashboard />
}
