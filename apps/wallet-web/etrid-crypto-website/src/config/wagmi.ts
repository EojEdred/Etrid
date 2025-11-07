import { getDefaultConfig } from '@rainbow-me/rainbowkit'
import { ethPBC } from './chains'

export const config = getDefaultConfig({
  appName: 'Ëtrid Wallet',
  projectId: process.env.NEXT_PUBLIC_WALLETCONNECT_PROJECT_ID || 'YOUR_PROJECT_ID', // Get from https://cloud.walletconnect.com
  chains: [ethPBC],
  ssr: true, // Enable server-side rendering for Next.js
})
