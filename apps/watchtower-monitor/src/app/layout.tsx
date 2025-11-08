import type { Metadata } from 'next';
import './globals.css';
import Navigation from '../components/Navigation';
import Footer from '../components/Footer';

export const metadata: Metadata = {
  title: 'Watchtower Monitor - Lightning-Bloc Channel Monitoring',
  description: 'Real-time monitoring and fraud detection for Lightning-Bloc payment channels',
  keywords: 'watchtower, lightning network, fraud detection, blockchain monitoring, etrid',
};

export default function RootLayout({
  children,
}: {
  children: React.ReactNode;
}) {
  return (
    <html lang="en">
      <body className="font-sans">
        <div className="min-h-screen bg-gradient-to-br from-slate-900 via-blue-900 to-slate-900 text-white">
          <Navigation />
          <main>{children}</main>
          <Footer />
        </div>
      </body>
    </html>
  );
}
