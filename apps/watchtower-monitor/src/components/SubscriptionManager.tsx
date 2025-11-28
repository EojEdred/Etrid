'use client';

import React, { useState } from 'react';
import { Clock, Plus, RefreshCw, Star, CheckCircle, XCircle } from 'lucide-react';
import { formatCurrency, truncateAddress } from '../lib/utils';
import type { Subscription } from '../types';

// Mock subscription data
const mockSubscriptions: Subscription[] = [
  {
    id: '1',
    channelId: 'ch_abc123def456',
    subscribedAt: new Date(Date.now() - 86400000 * 30),
    expiresAt: new Date(Date.now() + 86400000 * 30),
    fee: 125.00,
    autoRenew: true,
    tier: 'premium',
    features: ['24/7 monitoring', 'Instant alerts', 'Priority intervention', 'Analytics'],
  },
  {
    id: '2',
    channelId: 'ch_ghi789jkl012',
    subscribedAt: new Date(Date.now() - 86400000 * 15),
    expiresAt: new Date(Date.now() + 86400000 * 45),
    fee: 50.00,
    autoRenew: true,
    tier: 'basic',
    features: ['Daily monitoring', 'Email alerts'],
  },
  {
    id: '3',
    channelId: 'ch_mno345pqr678',
    subscribedAt: new Date(Date.now() - 86400000 * 60),
    expiresAt: new Date(Date.now() + 86400000 * 5),
    fee: 75.00,
    autoRenew: false,
    tier: 'basic',
    features: ['Daily monitoring', 'Email alerts'],
  },
  {
    id: '4',
    channelId: 'ch_stu901vwx234',
    subscribedAt: new Date(Date.now() - 86400000 * 10),
    expiresAt: new Date(Date.now() + 86400000 * 80),
    fee: 250.00,
    autoRenew: true,
    tier: 'enterprise',
    features: [
      '24/7 monitoring',
      'Instant alerts',
      'Priority intervention',
      'Advanced analytics',
      'Dedicated support',
      'Custom SLA',
    ],
  },
];

export default function SubscriptionManager() {
  const [subscriptions, setSubscriptions] = useState<Subscription[]>(mockSubscriptions);
  const [showAddModal, setShowAddModal] = useState(false);

  const getTierColor = (tier: string) => {
    const colors: Record<string, string> = {
      basic: 'bg-blue-500/20 text-blue-400 border-blue-500/30',
      premium: 'bg-purple-500/20 text-purple-400 border-purple-500/30',
      enterprise: 'bg-yellow-500/20 text-yellow-400 border-yellow-500/30',
    };
    return colors[tier] || 'bg-gray-500/20 text-gray-400';
  };

  const getTierIcon = (tier: string) => {
    if (tier === 'enterprise') return <Star className="w-4 h-4" />;
    if (tier === 'premium') return <Star className="w-4 h-4" />;
    return <CheckCircle className="w-4 h-4" />;
  };

  const getDaysRemaining = (expiresAt: Date) => {
    return Math.floor((expiresAt.getTime() - Date.now()) / (1000 * 60 * 60 * 24));
  };

  const toggleAutoRenew = (id: string) => {
    setSubscriptions((prev) =>
      prev.map((sub) =>
        sub.id === id ? { ...sub, autoRenew: !sub.autoRenew } : sub
      )
    );
  };

  const activeSubscriptions = subscriptions.filter(
    (sub) => sub.expiresAt > new Date()
  );
  const expiringSubscriptions = activeSubscriptions.filter(
    (sub) => getDaysRemaining(sub.expiresAt) < 7
  );

  const totalMonthlyRevenue = activeSubscriptions.reduce(
    (sum, sub) => sum + sub.fee,
    0
  );

  return (
    <div className="bg-white/10 backdrop-blur-lg rounded-xl border border-white/20">
      <div className="p-6 border-b border-white/20">
        <div className="flex items-center justify-between">
          <h2 className="text-xl font-semibold flex items-center gap-2">
            <RefreshCw className="w-5 h-5 text-blue-400" />
            Subscription Management
          </h2>
          <button
            onClick={() => setShowAddModal(true)}
            className="flex items-center gap-2 px-4 py-2 bg-blue-500 hover:bg-blue-600 rounded-lg text-sm transition-colors"
          >
            <Plus className="w-4 h-4" />
            Add Channel
          </button>
        </div>
      </div>

      <div className="p-6">
        {/* Summary Stats */}
        <div className="grid grid-cols-1 md:grid-cols-3 gap-4 mb-6">
          <div className="bg-white/5 rounded-lg p-4">
            <p className="text-sm text-gray-400 mb-1">Active Subscriptions</p>
            <p className="text-2xl font-bold">{activeSubscriptions.length}</p>
          </div>
          <div className="bg-white/5 rounded-lg p-4">
            <p className="text-sm text-gray-400 mb-1">Monthly Revenue</p>
            <p className="text-2xl font-bold text-green-400">
              {formatCurrency(totalMonthlyRevenue)}
            </p>
          </div>
          <div className="bg-white/5 rounded-lg p-4">
            <p className="text-sm text-gray-400 mb-1">Expiring Soon</p>
            <p className="text-2xl font-bold text-yellow-400">
              {expiringSubscriptions.length}
            </p>
          </div>
        </div>

        {/* Expiring Soon Alert */}
        {expiringSubscriptions.length > 0 && (
          <div className="mb-6 p-4 bg-yellow-500/10 border border-yellow-500/30 rounded-lg">
            <div className="flex items-start gap-3">
              <Clock className="w-5 h-5 text-yellow-400 mt-0.5" />
              <div>
                <p className="text-sm font-semibold text-yellow-400 mb-1">
                  {expiringSubscriptions.length} subscription{expiringSubscriptions.length > 1 ? 's' : ''} expiring soon
                </p>
                <p className="text-xs text-gray-400">
                  Review and renew to maintain channel monitoring
                </p>
              </div>
            </div>
          </div>
        )}

        {/* Subscriptions List */}
        <div className="space-y-4">
          {subscriptions.map((subscription) => {
            const daysRemaining = getDaysRemaining(subscription.expiresAt);
            const isExpiring = daysRemaining < 7;
            const isExpired = daysRemaining < 0;

            return (
              <div
                key={subscription.id}
                className={`p-5 rounded-lg border transition-all ${
                  isExpired
                    ? 'bg-red-500/10 border-red-500/30'
                    : isExpiring
                    ? 'bg-yellow-500/10 border-yellow-500/30'
                    : 'bg-white/5 border-white/20 hover:bg-white/10'
                }`}
              >
                <div className="flex items-start justify-between mb-4">
                  <div className="flex-1">
                    <div className="flex items-center gap-2 mb-2">
                      <h3 className="font-mono text-sm">
                        {truncateAddress(subscription.channelId, 12)}
                      </h3>
                      <span
                        className={`flex items-center gap-1 text-xs px-2 py-1 rounded border ${getTierColor(
                          subscription.tier
                        )}`}
                      >
                        {getTierIcon(subscription.tier)}
                        {subscription.tier.toUpperCase()}
                      </span>
                      {subscription.autoRenew && (
                        <span className="flex items-center gap-1 text-xs px-2 py-1 rounded bg-green-500/20 text-green-400">
                          <RefreshCw className="w-3 h-3" />
                          Auto-renew
                        </span>
                      )}
                    </div>
                    <div className="flex flex-wrap gap-2 mt-2">
                      {subscription.features.map((feature, index) => (
                        <span
                          key={index}
                          className="text-xs px-2 py-1 bg-white/5 rounded text-gray-300"
                        >
                          {feature}
                        </span>
                      ))}
                    </div>
                  </div>
                  <div className="text-right">
                    <p className="text-2xl font-bold text-green-400">
                      {formatCurrency(subscription.fee)}
                    </p>
                    <p className="text-xs text-gray-400">per month</p>
                  </div>
                </div>

                <div className="grid grid-cols-2 gap-4 text-sm mb-4">
                  <div>
                    <p className="text-xs text-gray-400 mb-1">Subscribed</p>
                    <p>{subscription.subscribedAt.toLocaleDateString()}</p>
                  </div>
                  <div>
                    <p className="text-xs text-gray-400 mb-1">Expires</p>
                    <p className={isExpiring ? 'text-yellow-400 font-semibold' : ''}>
                      {subscription.expiresAt.toLocaleDateString()}
                      {!isExpired && ` (${daysRemaining} days)`}
                    </p>
                  </div>
                </div>

                {/* Progress Bar */}
                {!isExpired && (
                  <div className="mb-4">
                    <div className="h-1.5 bg-white/10 rounded-full overflow-hidden">
                      <div
                        className={`h-full transition-all ${
                          isExpiring ? 'bg-yellow-500' : 'bg-blue-500'
                        }`}
                        style={{
                          width: `${Math.max(
                            0,
                            Math.min(
                              100,
                              ((90 - daysRemaining) / 90) * 100
                            )
                          )}%`,
                        }}
                      />
                    </div>
                  </div>
                )}

                {/* Actions */}
                <div className="flex gap-2">
                  <button
                    onClick={() => toggleAutoRenew(subscription.id)}
                    className={`flex-1 px-4 py-2 rounded-lg text-sm font-medium transition-colors ${
                      subscription.autoRenew
                        ? 'bg-green-500/20 text-green-400 hover:bg-green-500/30'
                        : 'bg-white/5 text-gray-400 hover:bg-white/10'
                    }`}
                  >
                    {subscription.autoRenew ? 'Auto-Renew On' : 'Enable Auto-Renew'}
                  </button>
                  {isExpiring && !subscription.autoRenew && (
                    <button className="px-6 py-2 bg-blue-500 hover:bg-blue-600 text-white rounded-lg text-sm font-medium transition-colors">
                      Renew Now
                    </button>
                  )}
                  <button className="px-4 py-2 bg-white/5 hover:bg-white/10 text-white rounded-lg text-sm font-medium transition-colors">
                    Upgrade
                  </button>
                  <button className="px-4 py-2 bg-red-500/20 hover:bg-red-500/30 text-red-400 rounded-lg text-sm font-medium transition-colors">
                    Cancel
                  </button>
                </div>
              </div>
            );
          })}
        </div>

        {subscriptions.length === 0 && (
          <div className="text-center py-12">
            <RefreshCw className="w-12 h-12 text-gray-400 mx-auto mb-4" />
            <p className="text-gray-400 mb-4">No active subscriptions</p>
            <button
              onClick={() => setShowAddModal(true)}
              className="px-6 py-2 bg-blue-500 hover:bg-blue-600 text-white rounded-lg font-medium transition-colors"
            >
              Subscribe to Channel
            </button>
          </div>
        )}
      </div>

      {/* Add Subscription Modal */}
      {showAddModal && (
        <div
          className="fixed inset-0 bg-black/50 backdrop-blur-sm flex items-center justify-center z-50 p-6"
          onClick={() => setShowAddModal(false)}
        >
          <div
            className="bg-slate-800 rounded-xl border border-white/20 max-w-lg w-full p-6"
            onClick={(e) => e.stopPropagation()}
          >
            <h2 className="text-2xl font-bold mb-6">Subscribe to Channel</h2>

            <div className="space-y-4">
              <div>
                <label className="block text-sm font-medium mb-2">Channel ID</label>
                <input
                  type="text"
                  placeholder="ch_..."
                  className="w-full px-4 py-2 bg-white/5 border border-white/20 rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-500"
                />
              </div>

              <div>
                <label className="block text-sm font-medium mb-2">Subscription Tier</label>
                <div className="space-y-2">
                  {['basic', 'premium', 'enterprise'].map((tier) => (
                    <div
                      key={tier}
                      className="p-4 bg-white/5 rounded-lg border border-white/20 hover:border-blue-500/50 cursor-pointer transition-colors"
                    >
                      <div className="flex items-center justify-between mb-2">
                        <span className="font-semibold capitalize">{tier}</span>
                        <span className="text-green-400 font-bold">
                          {tier === 'basic' && '$50/mo'}
                          {tier === 'premium' && '$125/mo'}
                          {tier === 'enterprise' && '$250/mo'}
                        </span>
                      </div>
                      <p className="text-xs text-gray-400">
                        {tier === 'basic' && 'Daily monitoring, email alerts'}
                        {tier === 'premium' && '24/7 monitoring, instant alerts, priority support'}
                        {tier === 'enterprise' && 'Full features, dedicated support, custom SLA'}
                      </p>
                    </div>
                  ))}
                </div>
              </div>

              <div className="flex gap-3 mt-6">
                <button className="flex-1 px-4 py-2 bg-blue-500 hover:bg-blue-600 text-white rounded-lg font-medium transition-colors">
                  Subscribe
                </button>
                <button
                  onClick={() => setShowAddModal(false)}
                  className="px-4 py-2 bg-white/10 hover:bg-white/20 text-white rounded-lg font-medium transition-colors"
                >
                  Cancel
                </button>
              </div>
            </div>
          </div>
        </div>
      )}
    </div>
  );
}
