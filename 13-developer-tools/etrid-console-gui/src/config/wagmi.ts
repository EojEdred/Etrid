import { getDefaultConfig } from '@rainbow-me/rainbowkit'
import { primearcCoreEvm } from './chains'

export const config = getDefaultConfig({
  appName: 'Ëtrid Wallet',
  projectId: process.env.NEXT_PUBLIC_WALLETCONNECT_PROJECT_ID || 'YOUR_PROJECT_ID', // Get from https://cloud.walletconnect.com
  chains: [primearcCoreEvm],
  ssr: false, // Disable server-side rendering for static export to avoid hydration mismatches
})
