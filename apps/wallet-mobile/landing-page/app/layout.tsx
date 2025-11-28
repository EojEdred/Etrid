import type { Metadata } from 'next';
import { Inter } from 'next/font/google';
import './globals.css';
import Navigation from '@/components/Navigation';
import Footer from '@/components/Footer';

const inter = Inter({ subsets: ['latin'] });

export const metadata: Metadata = {
  title: 'Ëtrid Mobile Wallet - The Complete DeFi Wallet',
  description: 'Your crypto secured debit card, NFT marketplace, advanced trading, and 15 more features in one beautiful app. The only wallet you\'ll ever need.',
  keywords: 'crypto wallet, defi, nft, blockchain, ethereum, bitcoin, mobile wallet, crypto debit card',
  authors: [{ name: 'Ëtrid' }],
  creator: 'Ëtrid',
  publisher: 'Ëtrid',
  openGraph: {
    type: 'website',
    locale: 'en_US',
    url: 'https://wallet.etrid.com',
    title: 'Ëtrid Mobile Wallet - The Complete DeFi Wallet',
    description: 'Your crypto secured debit card, NFT marketplace, advanced trading, and 15 more features in one beautiful app.',
    siteName: 'Ëtrid Mobile Wallet',
    images: [
      {
        url: 'https://wallet.etrid.com/og-image.jpg',
        width: 1200,
        height: 630,
        alt: 'Ëtrid Mobile Wallet',
      },
    ],
  },
  twitter: {
    card: 'summary_large_image',
    title: 'Ëtrid Mobile Wallet - The Complete DeFi Wallet',
    description: 'Your crypto secured debit card, NFT marketplace, advanced trading, and 15 more features in one beautiful app.',
    creator: '@etrid',
    images: ['https://wallet.etrid.com/og-image.jpg'],
  },
  viewport: {
    width: 'device-width',
    initialScale: 1,
    maximumScale: 1,
  },
  icons: {
    icon: '/favicon.ico',
    apple: '/apple-touch-icon.png',
  },
  manifest: '/manifest.json',
};

export default function RootLayout({
  children,
}: {
  children: React.ReactNode;
}) {
  return (
    <html lang="en" className="scroll-smooth">
      <body className={inter.className}>
        <Navigation />
        {children}
        <Footer />
      </body>
    </html>
  );
}
