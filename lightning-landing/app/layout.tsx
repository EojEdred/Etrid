import type { Metadata } from 'next';
import { Inter } from 'next/font/google';
import './globals.css';

const inter = Inter({ subsets: ['latin'] });

export const metadata: Metadata = {
  title: 'ÉTRID Lightning Network - Lightning-Fast Payments Across 14 Blockchains',
  description: 'Instant, low-cost cross-chain payments powered by the ÉTRID Lightning Network. Connect 14 blockchains with instant settlement, BOLT-11 compatibility, and enterprise-grade security.',
  keywords: 'Lightning Network, Cross-chain payments, Blockchain, Bitcoin, Ethereum, Solana, Instant payments, Cryptocurrency',
  authors: [{ name: 'ÉTRID Team' }],
  openGraph: {
    title: 'ÉTRID Lightning Network',
    description: 'Lightning-Fast Payments Across 14 Blockchains',
    type: 'website',
    url: 'https://etrid.org/lightning',
  },
  twitter: {
    card: 'summary_large_image',
    title: 'ÉTRID Lightning Network',
    description: 'Lightning-Fast Payments Across 14 Blockchains',
  },
};

export default function RootLayout({
  children,
}: {
  children: React.ReactNode;
}) {
  return (
    <html lang="en">
      <body className={inter.className}>{children}</body>
    </html>
  );
}
