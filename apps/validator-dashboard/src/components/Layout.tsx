import React, { ReactNode } from 'react';
import Link from 'next/link';
import { useRouter } from 'next/router';
import {
  LayoutDashboard,
  Activity,
  Settings,
  Bell,
  Wifi,
  WifiOff,
  Menu,
  X,
  Users,
  Award,
} from 'lucide-react';

interface LayoutProps {
  children: ReactNode;
  isConnected?: boolean;
  onConnectWallet?: () => void;
}

export default function Layout({
  children,
  isConnected = false,
  onConnectWallet,
}: LayoutProps) {
  const router = useRouter();
  const [sidebarOpen, setSidebarOpen] = React.useState(false);

  const navigation = [
    { name: 'Dashboard', href: '/', icon: LayoutDashboard },
    { name: 'Performance', href: '/performance', icon: Activity },
    { name: 'Nominators', href: '/nominators', icon: Users },
    { name: 'Rewards', href: '/rewards', icon: Award },
    { name: 'Settings', href: '/settings', icon: Settings },
  ];

  const isActive = (href: string) => {
    return router.pathname === href;
  };

  return (
    <div className="min-h-screen bg-gray-50">
      {/* Header */}
      <header className="bg-white shadow-sm sticky top-0 z-40">
        <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
          <div className="flex justify-between items-center h-16">
            <div className="flex items-center space-x-4">
              <button
                onClick={() => setSidebarOpen(!sidebarOpen)}
                className="lg:hidden p-2 rounded-md text-gray-600 hover:text-gray-900 hover:bg-gray-100"
              >
                {sidebarOpen ? <X className="w-6 h-6" /> : <Menu className="w-6 h-6" />}
              </button>

              <Link href="/" className="flex items-center space-x-3">
                <div className="w-8 h-8 bg-gradient-to-br from-etrid-500 to-etrid-700 rounded-lg flex items-center justify-center">
                  <span className="text-white font-bold text-lg">Ë</span>
                </div>
                <div>
                  <h1 className="text-xl font-bold text-gray-900">Ëtrid Validator</h1>
                  <p className="text-xs text-gray-500">Dashboard</p>
                </div>
              </Link>
            </div>

            <div className="flex items-center space-x-4">
              <div className="flex items-center space-x-2">
                {isConnected ? (
                  <Wifi className="w-5 h-5 text-success-500" />
                ) : (
                  <WifiOff className="w-5 h-5 text-danger-500" />
                )}
                <span className="text-sm text-gray-600">
                  {isConnected ? 'Connected' : 'Disconnected'}
                </span>
              </div>

              {!isConnected && onConnectWallet && (
                <button
                  onClick={onConnectWallet}
                  className="px-4 py-2 bg-etrid-600 text-white rounded-lg hover:bg-etrid-700 transition-colors font-medium text-sm"
                >
                  Connect Wallet
                </button>
              )}

              <button className="relative p-2 text-gray-600 hover:text-gray-900 hover:bg-gray-100 rounded-lg">
                <Bell className="w-5 h-5" />
                <span className="absolute top-1 right-1 w-2 h-2 bg-danger-500 rounded-full"></span>
              </button>
            </div>
          </div>
        </div>
      </header>

      {/* Mobile sidebar backdrop */}
      {sidebarOpen && (
        <div
          className="fixed inset-0 bg-gray-600 bg-opacity-75 z-30 lg:hidden"
          onClick={() => setSidebarOpen(false)}
        />
      )}

      <div className="flex">
        {/* Sidebar */}
        <aside
          className={`fixed inset-y-0 left-0 z-30 w-64 bg-white shadow-lg transform transition-transform duration-300 ease-in-out lg:translate-x-0 lg:static lg:inset-auto ${
            sidebarOpen ? 'translate-x-0' : '-translate-x-full'
          } top-16`}
        >
          <nav className="p-4 space-y-2">
            {navigation.map((item) => {
              const Icon = item.icon;
              const active = isActive(item.href);
              return (
                <Link
                  key={item.name}
                  href={item.href}
                  className={`flex items-center space-x-3 px-4 py-3 rounded-lg transition-colors ${
                    active
                      ? 'bg-etrid-50 text-etrid-700 font-semibold'
                      : 'text-gray-700 hover:bg-gray-100'
                  }`}
                  onClick={() => setSidebarOpen(false)}
                >
                  <Icon className="w-5 h-5" />
                  <span>{item.name}</span>
                </Link>
              );
            })}
          </nav>
        </aside>

        {/* Main content */}
        <main className="flex-1 p-4 lg:p-8">
          <div className="max-w-7xl mx-auto">{children}</div>
        </main>
      </div>

      {/* Footer */}
      <footer className="bg-white border-t border-gray-200 mt-12">
        <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-6">
          <div className="flex flex-col md:flex-row justify-between items-center space-y-4 md:space-y-0">
            <p className="text-sm text-gray-600">
              © 2024 Ëtrid Protocol. All rights reserved.
            </p>
            <div className="flex space-x-6 text-sm text-gray-600">
              <a href="#" className="hover:text-etrid-600">Documentation</a>
              <a href="#" className="hover:text-etrid-600">Support</a>
              <a href="#" className="hover:text-etrid-600">GitHub</a>
            </div>
          </div>
        </div>
      </footer>
    </div>
  );
}
