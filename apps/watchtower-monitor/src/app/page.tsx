'use client';

import React from 'react';
import { useChannelMonitoring } from '../hooks/useChannelMonitoring';
import { useFraudDetection } from '../hooks/useFraudDetection';
import ChannelList from '../components/ChannelList';
import FraudAlerts from '../components/FraudAlerts';
import ReputationScore from '../components/ReputationScore';
import EarningsTracker from '../components/EarningsTracker';
import { Activity, Shield, TrendingUp, AlertTriangle, Eye } from 'lucide-react';
import { useState } from 'react';

export default function Home() {
  const { stats, channels, isLoading } = useChannelMonitoring();
  const { alerts, fraudStats } = useFraudDetection();
  const [selectedView, setSelectedView] = useState<'overview' | 'channels' | 'alerts'>('overview');

  return (
    <div className="min-h-screen p-6">
      <div className="max-w-7xl mx-auto">
        {/* Header */}
        <div className="mb-8">
          <div className="flex items-center justify-between mb-2">
            <h1 className="text-4xl font-bold flex items-center gap-3">
              <Shield className="w-10 h-10 text-blue-400" />
              Watchtower Monitor
            </h1>
            <div className="flex items-center gap-2 px-4 py-2 bg-green-500/20 rounded-lg border border-green-500/30">
              <div className="w-2 h-2 bg-green-400 rounded-full animate-pulse" />
              <span className="text-sm font-medium">System Active</span>
            </div>
          </div>
          <p className="text-gray-400">Real-time Lightning-Bloc channel monitoring and fraud detection</p>
        </div>

        {/* Stats Overview */}
        <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-4 mb-8">
          <div className="bg-white/10 backdrop-blur-lg rounded-xl p-6 border border-white/20">
            <div className="flex items-center justify-between mb-2">
              <span className="text-gray-400 text-sm">Active Channels</span>
              <Eye className="w-5 h-5 text-blue-400" />
            </div>
            <p className="text-3xl font-bold">{stats.activeChannels}</p>
            <p className="text-xs text-gray-400 mt-1">of {stats.totalMonitored} total</p>
          </div>

          <div className="bg-white/10 backdrop-blur-lg rounded-xl p-6 border border-white/20">
            <div className="flex items-center justify-between mb-2">
              <span className="text-gray-400 text-sm">Frauds Detected</span>
              <AlertTriangle className="w-5 h-5 text-red-400" />
            </div>
            <p className="text-3xl font-bold text-red-400">{stats.fraudsDetected}</p>
            <p className="text-xs text-gray-400 mt-1">Last 30 days</p>
          </div>

          <div className="bg-white/10 backdrop-blur-lg rounded-xl p-6 border border-white/20">
            <div className="flex items-center justify-between mb-2">
              <span className="text-gray-400 text-sm">Uptime</span>
              <Activity className="w-5 h-5 text-green-400" />
            </div>
            <p className="text-3xl font-bold text-green-400">{stats.uptime.toFixed(2)}%</p>
            <p className="text-xs text-gray-400 mt-1">Response: {stats.averageResponseTime}ms</p>
          </div>

          <div className="bg-white/10 backdrop-blur-lg rounded-xl p-6 border border-white/20">
            <div className="flex items-center justify-between mb-2">
              <span className="text-gray-400 text-sm">Performance</span>
              <TrendingUp className="w-5 h-5 text-purple-400" />
            </div>
            <p className="text-3xl font-bold text-purple-400">
              {fraudStats.accuracy.toFixed(1)}%
            </p>
            <p className="text-xs text-gray-400 mt-1">Detection accuracy</p>
          </div>
        </div>

        {/* View Selector */}
        <div className="flex gap-2 mb-6">
          <button
            onClick={() => setSelectedView('overview')}
            className={`px-6 py-2 rounded-lg font-medium transition-all ${
              selectedView === 'overview'
                ? 'bg-blue-500 text-white'
                : 'bg-white/10 text-gray-400 hover:bg-white/20'
            }`}
          >
            Overview
          </button>
          <button
            onClick={() => setSelectedView('channels')}
            className={`px-6 py-2 rounded-lg font-medium transition-all ${
              selectedView === 'channels'
                ? 'bg-blue-500 text-white'
                : 'bg-white/10 text-gray-400 hover:bg-white/20'
            }`}
          >
            Channels
          </button>
          <button
            onClick={() => setSelectedView('alerts')}
            className={`px-6 py-2 rounded-lg font-medium transition-all ${
              selectedView === 'alerts'
                ? 'bg-blue-500 text-white'
                : 'bg-white/10 text-gray-400 hover:bg-white/20'
            }`}
          >
            Alerts {alerts.length > 0 && `(${alerts.length})`}
          </button>
        </div>

        {/* Content Area */}
        <div className="grid grid-cols-1 lg:grid-cols-3 gap-6">
          <div className="lg:col-span-2">
            {selectedView === 'overview' && (
              <div className="space-y-6">
                <ReputationScore />
                <EarningsTracker />
              </div>
            )}
            {selectedView === 'channels' && <ChannelList channels={channels} isLoading={isLoading} />}
            {selectedView === 'alerts' && <FraudAlerts alerts={alerts} />}
          </div>

          <div className="space-y-6">
            {/* Recent Alerts Sidebar */}
            <div className="bg-white/10 backdrop-blur-lg rounded-xl p-6 border border-white/20">
              <h3 className="text-lg font-semibold mb-4 flex items-center gap-2">
                <AlertTriangle className="w-5 h-5 text-yellow-400" />
                Recent Alerts
              </h3>
              {alerts.slice(0, 5).map((alert) => (
                <div
                  key={alert.id}
                  className="mb-3 pb-3 border-b border-white/10 last:border-0"
                >
                  <div className="flex items-start justify-between">
                    <div className="flex-1">
                      <p className="text-sm font-medium">{alert.type.replace(/_/g, ' ')}</p>
                      <p className="text-xs text-gray-400 mt-1">
                        Channel: {alert.channelId.slice(0, 8)}...
                      </p>
                    </div>
                    <span
                      className={`text-xs px-2 py-1 rounded ${
                        alert.severity === 'critical'
                          ? 'bg-red-500/20 text-red-400'
                          : alert.severity === 'high'
                          ? 'bg-orange-500/20 text-orange-400'
                          : 'bg-yellow-500/20 text-yellow-400'
                      }`}
                    >
                      {alert.severity}
                    </span>
                  </div>
                </div>
              ))}
              {alerts.length === 0 && (
                <p className="text-sm text-gray-400 text-center py-4">No recent alerts</p>
              )}
            </div>

            {/* System Status */}
            <div className="bg-white/10 backdrop-blur-lg rounded-xl p-6 border border-white/20">
              <h3 className="text-lg font-semibold mb-4">System Status</h3>
              <div className="space-y-3">
                <div className="flex justify-between items-center">
                  <span className="text-sm text-gray-400">WebSocket</span>
                  <span className="text-sm text-green-400">Connected</span>
                </div>
                <div className="flex justify-between items-center">
                  <span className="text-sm text-gray-400">Node Connection</span>
                  <span className="text-sm text-green-400">Active</span>
                </div>
                <div className="flex justify-between items-center">
                  <span className="text-sm text-gray-400">Last Check</span>
                  <span className="text-sm text-gray-300">
                    {new Date(stats.lastCheck).toLocaleTimeString()}
                  </span>
                </div>
              </div>
            </div>
          </div>
        </div>
      </div>
    </div>
  );
}
