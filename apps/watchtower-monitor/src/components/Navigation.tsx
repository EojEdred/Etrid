'use client';

import React from 'react';
import Link from 'next/link';
import { usePathname } from 'next/navigation';
import { LayoutDashboard, FileText, Settings, Shield } from 'lucide-react';

export default function Navigation() {
  const pathname = usePathname();

  const navItems = [
    { href: '/', label: 'Monitor', icon: LayoutDashboard },
    { href: '/reports', label: 'Reports', icon: FileText },
    { href: '/settings', label: 'Settings', icon: Settings },
  ];

  return (
    <nav className="bg-slate-900/50 backdrop-blur-lg border-b border-white/10 sticky top-0 z-50">
      <div className="max-w-7xl mx-auto px-6 py-4">
        <div className="flex items-center justify-between">
          <Link href="/" className="flex items-center gap-3 hover:opacity-80 transition-opacity">
            <Shield className="w-8 h-8 text-blue-400" />
            <div>
              <h1 className="text-xl font-bold text-white">Watchtower Monitor</h1>
              <p className="text-xs text-gray-400">Lightning-Bloc Protection</p>
            </div>
          </Link>

          <div className="flex gap-2">
            {navItems.map((item) => {
              const Icon = item.icon;
              const isActive = pathname === item.href;

              return (
                <Link
                  key={item.href}
                  href={item.href}
                  className={`flex items-center gap-2 px-4 py-2 rounded-lg font-medium transition-all ${
                    isActive
                      ? 'bg-blue-500 text-white'
                      : 'text-gray-400 hover:text-white hover:bg-white/10'
                  }`}
                >
                  <Icon className="w-4 h-4" />
                  {item.label}
                </Link>
              );
            })}
          </div>
        </div>
      </div>
    </nav>
  );
}
