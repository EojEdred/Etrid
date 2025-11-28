import './globals.css';
import type { Metadata } from 'next';

export const metadata: Metadata = {
  title: 'MasterChef Dashboard | Ëtrid Protocol',
  description: 'Real-time monitoring dashboard for ÉTR MasterChef LP Rewards program',
  keywords: ['DeFi', 'Yield Farming', 'LP Rewards', 'Ëtrid', 'BSC'],
};

export default function RootLayout({
  children,
}: {
  children: React.ReactNode;
}) {
  return (
    <html lang="en">
      <body>{children}</body>
    </html>
  );
}
