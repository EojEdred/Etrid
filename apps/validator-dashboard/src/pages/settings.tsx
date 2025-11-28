import React, { useState } from 'react';
import Head from 'next/head';
import Layout from '@/components/Layout';
import CommissionSettings from '@/components/CommissionSettings';
import { useValidatorStats } from '@/hooks/useValidatorStats';
import {
  Settings as SettingsIcon,
  Bell,
  Shield,
  Key,
  Save,
  Info,
  Mail,
  MessageSquare,
  Server,
  AlertCircle,
} from 'lucide-react';

export default function Settings() {
  const [validatorAddress, setValidatorAddress] = useState<string | undefined>(
    process.env.NEXT_PUBLIC_VALIDATOR_ADDRESS
  );

  const { isConnected, validatorInfo } = useValidatorStats(validatorAddress);

  // Settings state
  const [alertsEnabled, setAlertsEnabled] = useState(true);
  const [emailNotifications, setEmailNotifications] = useState(true);
  const [discordNotifications, setDiscordNotifications] = useState(false);
  const [email, setEmail] = useState('');
  const [discordWebhook, setDiscordWebhook] = useState('');
  const [nodeEndpoint, setNodeEndpoint] = useState(
    process.env.NEXT_PUBLIC_WS_PROVIDER || 'ws://localhost:9944'
  );

  const [isSaving, setIsSaving] = useState(false);
  const [showSuccess, setShowSuccess] = useState(false);

  const handleSaveNotifications = async () => {
    setIsSaving(true);
    // Simulate save
    await new Promise((resolve) => setTimeout(resolve, 1000));
    setShowSuccess(true);
    setIsSaving(false);
    setTimeout(() => setShowSuccess(false), 3000);
  };

  const handleCommissionUpdate = async (newCommission: number, settings: any) => {
    console.log('Updating commission:', newCommission, settings);
    // Implement actual commission update logic here
    await new Promise((resolve) => setTimeout(resolve, 1500));
  };

  return (
    <>
      <Head>
        <title>Settings - Ëtrid Validator Dashboard</title>
        <meta name="description" content="Configure your validator settings and preferences" />
      </Head>

      <Layout isConnected={isConnected}>
        <div className="space-y-6">
          {/* Page Header */}
          <div>
            <h1 className="text-3xl font-bold text-gray-900">Settings</h1>
            <p className="text-gray-600 mt-1">
              Configure your validator settings and notification preferences
            </p>
          </div>

          {/* Commission Settings */}
          <CommissionSettings
            currentCommission={validatorInfo?.commission || 0}
            onUpdate={handleCommissionUpdate}
          />

          {/* Notifications Settings */}
          <div className="bg-white rounded-lg shadow-md overflow-hidden">
            <div className="p-6 border-b border-gray-200">
              <div className="flex items-center space-x-3">
                <div className="p-2 bg-blue-50 rounded-lg">
                  <Bell className="w-6 h-6 text-blue-600" />
                </div>
                <div>
                  <h2 className="text-xl font-bold text-gray-900">Notification Settings</h2>
                  <p className="text-sm text-gray-600">
                    Configure how you want to receive alerts
                  </p>
                </div>
              </div>
            </div>

            <div className="p-6 space-y-6">
              {showSuccess && (
                <div className="bg-success-50 border border-success-200 rounded-lg p-4 flex items-start space-x-3">
                  <Info className="w-5 h-5 text-success-600 flex-shrink-0 mt-0.5" />
                  <div>
                    <p className="text-sm font-medium text-success-800">Success</p>
                    <p className="text-sm text-success-700">Notification settings saved</p>
                  </div>
                </div>
              )}

              {/* Enable Alerts */}
              <div className="flex items-center justify-between p-4 bg-gray-50 rounded-lg">
                <div>
                  <p className="text-sm font-medium text-gray-900">Enable Alerts</p>
                  <p className="text-xs text-gray-500">
                    Receive notifications about validator issues and performance
                  </p>
                </div>
                <button
                  onClick={() => setAlertsEnabled(!alertsEnabled)}
                  className={`relative inline-flex h-6 w-11 items-center rounded-full transition-colors ${
                    alertsEnabled ? 'bg-etrid-600' : 'bg-gray-300'
                  }`}
                >
                  <span
                    className={`inline-block h-4 w-4 transform rounded-full bg-white transition-transform ${
                      alertsEnabled ? 'translate-x-6' : 'translate-x-1'
                    }`}
                  />
                </button>
              </div>

              {/* Email Notifications */}
              <div className="space-y-3">
                <div className="flex items-center justify-between">
                  <div className="flex items-center space-x-3">
                    <Mail className="w-5 h-5 text-gray-600" />
                    <div>
                      <p className="text-sm font-medium text-gray-900">Email Notifications</p>
                      <p className="text-xs text-gray-500">
                        Receive alerts via email
                      </p>
                    </div>
                  </div>
                  <button
                    onClick={() => setEmailNotifications(!emailNotifications)}
                    disabled={!alertsEnabled}
                    className={`relative inline-flex h-6 w-11 items-center rounded-full transition-colors ${
                      emailNotifications && alertsEnabled ? 'bg-etrid-600' : 'bg-gray-300'
                    } ${!alertsEnabled ? 'opacity-50 cursor-not-allowed' : ''}`}
                  >
                    <span
                      className={`inline-block h-4 w-4 transform rounded-full bg-white transition-transform ${
                        emailNotifications && alertsEnabled ? 'translate-x-6' : 'translate-x-1'
                      }`}
                    />
                  </button>
                </div>

                {emailNotifications && alertsEnabled && (
                  <div>
                    <label className="block text-sm font-medium text-gray-700 mb-2">
                      Email Address
                    </label>
                    <input
                      type="email"
                      value={email}
                      onChange={(e) => setEmail(e.target.value)}
                      placeholder="validator@example.com"
                      className="w-full px-3 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-etrid-500 focus:border-transparent"
                    />
                  </div>
                )}
              </div>

              {/* Discord Notifications */}
              <div className="space-y-3">
                <div className="flex items-center justify-between">
                  <div className="flex items-center space-x-3">
                    <MessageSquare className="w-5 h-5 text-gray-600" />
                    <div>
                      <p className="text-sm font-medium text-gray-900">Discord Notifications</p>
                      <p className="text-xs text-gray-500">
                        Receive alerts via Discord webhook
                      </p>
                    </div>
                  </div>
                  <button
                    onClick={() => setDiscordNotifications(!discordNotifications)}
                    disabled={!alertsEnabled}
                    className={`relative inline-flex h-6 w-11 items-center rounded-full transition-colors ${
                      discordNotifications && alertsEnabled ? 'bg-etrid-600' : 'bg-gray-300'
                    } ${!alertsEnabled ? 'opacity-50 cursor-not-allowed' : ''}`}
                  >
                    <span
                      className={`inline-block h-4 w-4 transform rounded-full bg-white transition-transform ${
                        discordNotifications && alertsEnabled ? 'translate-x-6' : 'translate-x-1'
                      }`}
                    />
                  </button>
                </div>

                {discordNotifications && alertsEnabled && (
                  <div>
                    <label className="block text-sm font-medium text-gray-700 mb-2">
                      Discord Webhook URL
                    </label>
                    <input
                      type="url"
                      value={discordWebhook}
                      onChange={(e) => setDiscordWebhook(e.target.value)}
                      placeholder="https://discord.com/api/webhooks/..."
                      className="w-full px-3 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-etrid-500 focus:border-transparent"
                    />
                    <p className="mt-1 text-xs text-gray-500">
                      <a
                        href="https://support.discord.com/hc/en-us/articles/228383668-Intro-to-Webhooks"
                        target="_blank"
                        rel="noopener noreferrer"
                        className="text-etrid-600 hover:text-etrid-700"
                      >
                        Learn how to create a Discord webhook
                      </a>
                    </p>
                  </div>
                )}
              </div>

              {/* Alert Types */}
              <div className="border-t border-gray-200 pt-4">
                <h3 className="text-sm font-semibold text-gray-900 mb-3">Alert Types</h3>
                <div className="space-y-2">
                  {[
                    { label: 'Missed Blocks', defaultChecked: true },
                    { label: 'Low Nominator Count', defaultChecked: true },
                    { label: 'Commission Changes', defaultChecked: true },
                    { label: 'Reward Payments', defaultChecked: false },
                    { label: 'Era Completion', defaultChecked: false },
                    { label: 'Node Connection Issues', defaultChecked: true },
                  ].map((alert, index) => (
                    <label key={index} className="flex items-center space-x-3 p-2 hover:bg-gray-50 rounded">
                      <input
                        type="checkbox"
                        defaultChecked={alert.defaultChecked}
                        disabled={!alertsEnabled}
                        className="w-4 h-4 text-etrid-600 border-gray-300 rounded focus:ring-etrid-500 disabled:opacity-50"
                      />
                      <span className="text-sm text-gray-700">{alert.label}</span>
                    </label>
                  ))}
                </div>
              </div>

              <div className="flex justify-end pt-4 border-t border-gray-200">
                <button
                  onClick={handleSaveNotifications}
                  disabled={isSaving}
                  className={`flex items-center space-x-2 px-6 py-2 rounded-lg font-medium transition-colors ${
                    !isSaving
                      ? 'bg-etrid-600 text-white hover:bg-etrid-700'
                      : 'bg-gray-300 text-gray-500 cursor-not-allowed'
                  }`}
                >
                  <Save className="w-4 h-4" />
                  <span>{isSaving ? 'Saving...' : 'Save Changes'}</span>
                </button>
              </div>
            </div>
          </div>

          {/* Node Configuration */}
          <div className="bg-white rounded-lg shadow-md overflow-hidden">
            <div className="p-6 border-b border-gray-200">
              <div className="flex items-center space-x-3">
                <div className="p-2 bg-purple-50 rounded-lg">
                  <Server className="w-6 h-6 text-purple-600" />
                </div>
                <div>
                  <h2 className="text-xl font-bold text-gray-900">Node Configuration</h2>
                  <p className="text-sm text-gray-600">
                    Configure connection to your validator node
                  </p>
                </div>
              </div>
            </div>

            <div className="p-6 space-y-4">
              <div>
                <label className="block text-sm font-medium text-gray-700 mb-2">
                  Node Endpoint
                </label>
                <input
                  type="text"
                  value={nodeEndpoint}
                  onChange={(e) => setNodeEndpoint(e.target.value)}
                  placeholder="ws://localhost:9944"
                  className="w-full px-3 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-etrid-500 focus:border-transparent"
                />
                <p className="mt-1 text-xs text-gray-500">
                  WebSocket endpoint for your Ëtrid node
                </p>
              </div>

              <div className="bg-blue-50 border border-blue-200 rounded-lg p-4">
                <div className="flex items-start space-x-3">
                  <Info className="w-5 h-5 text-blue-600 flex-shrink-0 mt-0.5" />
                  <div className="text-sm text-blue-700">
                    <p className="font-medium">Connection Status</p>
                    <p className="mt-1">
                      {isConnected
                        ? 'Successfully connected to the node'
                        : 'Not connected. Please check your node endpoint.'}
                    </p>
                  </div>
                </div>
              </div>
            </div>
          </div>

          {/* Security Settings */}
          <div className="bg-white rounded-lg shadow-md overflow-hidden">
            <div className="p-6 border-b border-gray-200">
              <div className="flex items-center space-x-3">
                <div className="p-2 bg-orange-50 rounded-lg">
                  <Shield className="w-6 h-6 text-orange-600" />
                </div>
                <div>
                  <h2 className="text-xl font-bold text-gray-900">Security</h2>
                  <p className="text-sm text-gray-600">
                    Manage your validator security settings
                  </p>
                </div>
              </div>
            </div>

            <div className="p-6 space-y-4">
              <div className="bg-warning-50 border border-warning-200 rounded-lg p-4">
                <div className="flex items-start space-x-3">
                  <AlertCircle className="w-5 h-5 text-warning-600 flex-shrink-0 mt-0.5" />
                  <div className="text-sm text-warning-700">
                    <p className="font-medium">Session Keys</p>
                    <p className="mt-1">
                      Ensure your session keys are properly set and rotated regularly for security.
                    </p>
                    <div className="mt-3 space-y-2">
                      <div className="bg-white rounded p-2 font-mono text-xs break-all">
                        {validatorInfo?.sessionKeys || 'Not set'}
                      </div>
                    </div>
                  </div>
                </div>
              </div>

              <button className="flex items-center space-x-2 px-4 py-2 bg-white border border-gray-300 rounded-lg hover:bg-gray-50 transition-colors text-sm font-medium">
                <Key className="w-4 h-4" />
                <span>Rotate Session Keys</span>
              </button>
            </div>
          </div>
        </div>
      </Layout>
    </>
  );
}
