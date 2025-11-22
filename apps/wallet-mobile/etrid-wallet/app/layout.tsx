import type { Metadata } from 'next'
import { GeistSans } from 'geist/font/sans'
import { GeistMono } from 'geist/font/mono'
import { Analytics } from '@vercel/analytics/next'
import './globals.css'
import ServiceWorkerRegister from './sw-register'
import InstallPrompt from '@/components/pwa/InstallPrompt'
import OnlineStatus from '@/components/pwa/OnlineStatus'

export const metadata: Metadata = {
  title: 'Ëtrid Wallet - DeFi Mobile Wallet',
  description: 'Complete DeFi wallet with crypto debit card, NFTs, trading, and more',
  generator: 'Next.js',
  manifest: '/manifest.json',
  themeColor: '#8b5cf6',
  appleWebApp: {
    capable: true,
    statusBarStyle: 'black-translucent',
    title: 'Ëtrid Wallet',
  },
  formatDetection: {
    telephone: false,
  },
  icons: {
    icon: [
      { url: '/icons/icon-192x192.png', sizes: '192x192', type: 'image/png' },
      { url: '/icons/icon-512x512.png', sizes: '512x512', type: 'image/png' },
    ],
    apple: [
      { url: '/icons/apple-touch-icon.png', sizes: '180x180', type: 'image/png' },
    ],
  },
}

export default function RootLayout({
  children,
}: Readonly<{
  children: React.ReactNode
}>) {
  return (
    <html lang="en">
      <head>
        {/* PWA Meta Tags */}
        <meta name="application-name" content="Ëtrid Wallet" />
        <meta name="apple-mobile-web-app-capable" content="yes" />
        <meta name="apple-mobile-web-app-status-bar-style" content="black-translucent" />
        <meta name="apple-mobile-web-app-title" content="Ëtrid" />
        <meta name="format-detection" content="telephone=no" />
        <meta name="mobile-web-app-capable" content="yes" />
        <meta name="theme-color" content="#8b5cf6" />

        {/* Apple Splash Screens */}
        <link rel="apple-touch-startup-image" href="/splash/iphone5_splash.png" media="(device-width: 320px) and (device-height: 568px)" />
        <link rel="apple-touch-startup-image" href="/splash/iphone6_splash.png" media="(device-width: 375px) and (device-height: 667px)" />
        <link rel="apple-touch-startup-image" href="/splash/iphoneplus_splash.png" media="(device-width: 621px) and (device-height: 1104px)" />
        <link rel="apple-touch-startup-image" href="/splash/iphonex_splash.png" media="(device-width: 375px) and (device-height: 812px)" />
        <link rel="apple-touch-startup-image" href="/splash/iphonexr_splash.png" media="(device-width: 414px) and (device-height: 896px)" />
        <link rel="apple-touch-startup-image" href="/splash/iphonexsmax_splash.png" media="(device-width: 414px) and (device-height: 896px)" />
        <link rel="apple-touch-startup-image" href="/splash/ipad_splash.png" media="(device-width: 768px) and (device-height: 1024px)" />
        <link rel="apple-touch-startup-image" href="/splash/ipadpro1_splash.png" media="(device-width: 834px) and (device-height: 1112px)" />
        <link rel="apple-touch-startup-image" href="/splash/ipadpro2_splash.png" media="(device-width: 1024px) and (device-height: 1366px)" />
      </head>
      <body className={`font-sans ${GeistSans.variable} ${GeistMono.variable}`}>
        {children}
        <Analytics />
        <ServiceWorkerRegister />
        <InstallPrompt />
        <OnlineStatus />
      </body>
    </html>
  )
}
