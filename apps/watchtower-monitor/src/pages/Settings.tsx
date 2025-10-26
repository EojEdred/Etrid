'use client';

import React, { useState } from 'react';
import { Settings as SettingsIcon, Save, Bell, Globe, Shield, AlertCircle } from 'lucide-react';
import type { WatchtowerConfig } from '../types';

export default function Settings() {
  const [config, setConfig] = useState<WatchtowerConfig>({
    nodeEndpoint: 'http://localhost:9944',
    wsEndpoint: 'ws://localhost:9944',
    pollingInterval: 5000,
    alertThresholds: {
      responseTime: 1000,
      balanceDeviation: 0.1,
    },
    notifications: {
      email: true,
      push: false,
      webhook: '',
    },
  });

  const [saved, setSaved] = useState(false);

  const handleSave = () => {
    // Save configuration to localStorage or backend
    localStorage.setItem('watchtower-config', JSON.stringify(config));
    setSaved(true);
    setTimeout(() => setSaved(false), 3000);
  };

  return (
    <div className="min-h-screen bg-gradient-to-br from-slate-900 via-blue-900 to-slate-900 text-white p-6">
      <div className="max-w-4xl mx-auto">
        {/* Header */}
        <div className="mb-8">
          <h1 className="text-4xl font-bold flex items-center gap-3 mb-2">
            <SettingsIcon className="w-10 h-10 text-blue-400" />
            Settings
          </h1>
          <p className="text-gray-400">Configure your watchtower monitoring preferences</p>
        </div>

        {/* Save Success Message */}
        {saved && (
          <div className="mb-6 p-4 bg-green-500/20 border border-green-500/30 rounded-lg flex items-center gap-3">
            <AlertCircle className="w-5 h-5 text-green-400" />
            <span className="text-green-400">Settings saved successfully!</span>
          </div>
        )}

        <div className="space-y-6">
          {/* Node Configuration */}
          <div className="bg-white/10 backdrop-blur-lg rounded-xl p-6 border border-white/20">
            <h2 className="text-xl font-semibold mb-4 flex items-center gap-2">
              <Globe className="w-5 h-5 text-blue-400" />
              Node Configuration
            </h2>
            <div className="space-y-4">
              <div>
                <label className="block text-sm font-medium mb-2">Node Endpoint</label>
                <input
                  type="text"
                  value={config.nodeEndpoint}
                  onChange={(e) =>
                    setConfig({ ...config, nodeEndpoint: e.target.value })
                  }
                  className="w-full px-4 py-2 bg-white/5 border border-white/20 rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-500"
                  placeholder="http://localhost:9944"
                />
                <p className="text-xs text-gray-400 mt-1">
                  HTTP endpoint for blockchain node connection
                </p>
              </div>

              <div>
                <label className="block text-sm font-medium mb-2">WebSocket Endpoint</label>
                <input
                  type="text"
                  value={config.wsEndpoint}
                  onChange={(e) =>
                    setConfig({ ...config, wsEndpoint: e.target.value })
                  }
                  className="w-full px-4 py-2 bg-white/5 border border-white/20 rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-500"
                  placeholder="ws://localhost:9944"
                />
                <p className="text-xs text-gray-400 mt-1">
                  WebSocket endpoint for real-time updates
                </p>
              </div>

              <div>
                <label className="block text-sm font-medium mb-2">
                  Polling Interval (ms)
                </label>
                <input
                  type="number"
                  value={config.pollingInterval}
                  onChange={(e) =>
                    setConfig({ ...config, pollingInterval: parseInt(e.target.value) })
                  }
                  className="w-full px-4 py-2 bg-white/5 border border-white/20 rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-500"
                  placeholder="5000"
                  min="1000"
                  max="60000"
                />
                <p className="text-xs text-gray-400 mt-1">
                  How often to check for channel updates (1000-60000ms)
                </p>
              </div>
            </div>
          </div>

          {/* Alert Thresholds */}
          <div className="bg-white/10 backdrop-blur-lg rounded-xl p-6 border border-white/20">
            <h2 className="text-xl font-semibold mb-4 flex items-center gap-2">
              <Shield className="w-5 h-5 text-blue-400" />
              Alert Thresholds
            </h2>
            <div className="space-y-4">
              <div>
                <label className="block text-sm font-medium mb-2">
                  Response Time Alert (ms)
                </label>
                <input
                  type="number"
                  value={config.alertThresholds.responseTime}
                  onChange={(e) =>
                    setConfig({
                      ...config,
                      alertThresholds: {
                        ...config.alertThresholds,
                        responseTime: parseInt(e.target.value),
                      },
                    })
                  }
                  className="w-full px-4 py-2 bg-white/5 border border-white/20 rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-500"
                  placeholder="1000"
                  min="100"
                  max="10000"
                />
                <p className="text-xs text-gray-400 mt-1">
                  Alert when response time exceeds this threshold
                </p>
              </div>

              <div>
                <label className="block text-sm font-medium mb-2">
                  Balance Deviation (0-1)
                </label>
                <input
                  type="number"
                  step="0.01"
                  value={config.alertThresholds.balanceDeviation}
                  onChange={(e) =>
                    setConfig({
                      ...config,
                      alertThresholds: {
                        ...config.alertThresholds,
                        balanceDeviation: parseFloat(e.target.value),
                      },
                    })
                  }
                  className="w-full px-4 py-2 bg-white/5 border border-white/20 rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-500"
                  placeholder="0.1"
                  min="0"
                  max="1"
                />
                <p className="text-xs text-gray-400 mt-1">
                  Alert when channel balance deviates by this percentage
                </p>
              </div>
            </div>
          </div>

          {/* Notifications */}
          <div className="bg-white/10 backdrop-blur-lg rounded-xl p-6 border border-white/20">
            <h2 className="text-xl font-semibold mb-4 flex items-center gap-2">
              <Bell className="w-5 h-5 text-blue-400" />
              Notifications
            </h2>
            <div className="space-y-4">
              <div className="flex items-center justify-between">
                <div>
                  <p className="font-medium">Email Notifications</p>
                  <p className="text-sm text-gray-400">
                    Receive fraud alerts via email
                  </p>
                </div>
                <label className="relative inline-flex items-center cursor-pointer">
                  <input
                    type="checkbox"
                    checked={config.notifications.email}
                    onChange={(e) =>
                      setConfig({
                        ...config,
                        notifications: {
                          ...config.notifications,
                          email: e.target.checked,
                        },
                      })
                    }
                    className="sr-only peer"
                  />
                  <div className="w-11 h-6 bg-gray-600 peer-focus:outline-none peer-focus:ring-4 peer-focus:ring-blue-500/50 rounded-full peer peer-checked:after:translate-x-full peer-checked:after:border-white after:content-[''] after:absolute after:top-[2px] after:left-[2px] after:bg-white after:border-gray-300 after:border after:rounded-full after:h-5 after:w-5 after:transition-all peer-checked:bg-blue-500"></div>
                </label>
              </div>

              <div className="flex items-center justify-between">
                <div>
                  <p className="font-medium">Push Notifications</p>
                  <p className="text-sm text-gray-400">
                    Receive alerts via browser push
                  </p>
                </div>
                <label className="relative inline-flex items-center cursor-pointer">
                  <input
                    type="checkbox"
                    checked={config.notifications.push}
                    onChange={(e) =>
                      setConfig({
                        ...config,
                        notifications: {
                          ...config.notifications,
                          push: e.target.checked,
                        },
                      })
                    }
                    className="sr-only peer"
                  />
                  <div className="w-11 h-6 bg-gray-600 peer-focus:outline-none peer-focus:ring-4 peer-focus:ring-blue-500/50 rounded-full peer peer-checked:after:translate-x-full peer-checked:after:border-white after:content-[''] after:absolute after:top-[2px] after:left-[2px] after:bg-white after:border-gray-300 after:border after:rounded-full after:h-5 after:w-5 after:transition-all peer-checked:bg-blue-500"></div>
                </label>
              </div>

              <div>
                <label className="block text-sm font-medium mb-2">Webhook URL (Optional)</label>
                <input
                  type="url"
                  value={config.notifications.webhook || ''}
                  onChange={(e) =>
                    setConfig({
                      ...config,
                      notifications: {
                        ...config.notifications,
                        webhook: e.target.value,
                      },
                    })
                  }
                  className="w-full px-4 py-2 bg-white/5 border border-white/20 rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-500"
                  placeholder="https://your-webhook-url.com/alerts"
                />
                <p className="text-xs text-gray-400 mt-1">
                  Send POST requests to this URL when fraud is detected
                </p>
              </div>
            </div>
          </div>

          {/* Save Button */}
          <div className="flex justify-end">
            <button
              onClick={handleSave}
              className="flex items-center gap-2 px-6 py-3 bg-blue-500 hover:bg-blue-600 rounded-lg font-medium transition-colors"
            >
              <Save className="w-4 h-4" />
              Save Configuration
            </button>
          </div>
        </div>
      </div>
    </div>
  );
}
