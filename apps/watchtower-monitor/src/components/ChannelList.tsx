'use client';

import React, { useState } from 'react';
import { Activity, CheckCircle, XCircle, AlertTriangle, Search } from 'lucide-react';
import { formatCurrency, formatNumber, getStatusColor, truncateAddress } from '../lib/utils';
import type { Channel } from '../types';

interface ChannelListProps {
  channels: Channel[];
  isLoading: boolean;
}

export default function ChannelList({ channels, isLoading }: ChannelListProps) {
  const [searchTerm, setSearchTerm] = useState('');
  const [statusFilter, setStatusFilter] = useState<string>('all');

  const filteredChannels = channels.filter((channel) => {
    const matchesSearch =
      channel.channelId.toLowerCase().includes(searchTerm.toLowerCase()) ||
      channel.node1.toLowerCase().includes(searchTerm.toLowerCase()) ||
      channel.node2.toLowerCase().includes(searchTerm.toLowerCase());

    const matchesStatus = statusFilter === 'all' || channel.status === statusFilter;

    return matchesSearch && matchesStatus;
  });

  if (isLoading) {
    return (
      <div className="bg-white/10 backdrop-blur-lg rounded-xl p-8 border border-white/20">
        <div className="flex items-center justify-center">
          <div className="animate-spin rounded-full h-12 w-12 border-b-2 border-blue-500"></div>
          <span className="ml-4 text-gray-400">Loading channels...</span>
        </div>
      </div>
    );
  }

  return (
    <div className="bg-white/10 backdrop-blur-lg rounded-xl border border-white/20">
      <div className="p-6 border-b border-white/20">
        <div className="flex items-center justify-between mb-4">
          <h2 className="text-xl font-semibold">Monitored Channels</h2>
          <span className="text-sm text-gray-400">{filteredChannels.length} channels</span>
        </div>

        <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
          <div className="relative">
            <Search className="absolute left-3 top-1/2 -translate-y-1/2 w-4 h-4 text-gray-400" />
            <input
              type="text"
              placeholder="Search by channel ID or node address..."
              value={searchTerm}
              onChange={(e) => setSearchTerm(e.target.value)}
              className="w-full pl-10 pr-4 py-2 bg-white/5 border border-white/20 rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-500"
            />
          </div>

          <select
            value={statusFilter}
            onChange={(e) => setStatusFilter(e.target.value)}
            className="px-4 py-2 bg-white/5 border border-white/20 rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-500"
          >
            <option value="all">All Status</option>
            <option value="active">Active</option>
            <option value="inactive">Inactive</option>
            <option value="disputed">Disputed</option>
            <option value="closed">Closed</option>
          </select>
        </div>
      </div>

      <div className="divide-y divide-white/10">
        {filteredChannels.map((channel) => (
          <div
            key={channel.id}
            className="p-6 hover:bg-white/5 transition-colors"
          >
            <div className="flex items-start justify-between mb-4">
              <div className="flex-1">
                <div className="flex items-center gap-3 mb-2">
                  <h3 className="font-mono text-sm">{truncateAddress(channel.channelId, 12)}</h3>
                  <span className={`text-xs px-2 py-1 rounded ${getStatusColor(channel.status)}`}>
                    {channel.status}
                  </span>
                  {channel.watchtowerActive && (
                    <span className="flex items-center gap-1 text-xs text-green-400">
                      <CheckCircle className="w-3 h-3" />
                      Monitoring
                    </span>
                  )}
                </div>

                <div className="grid grid-cols-2 gap-4 text-sm">
                  <div>
                    <p className="text-gray-400 text-xs mb-1">Node 1</p>
                    <p className="font-mono text-xs">{truncateAddress(channel.node1)}</p>
                  </div>
                  <div>
                    <p className="text-gray-400 text-xs mb-1">Node 2</p>
                    <p className="font-mono text-xs">{truncateAddress(channel.node2)}</p>
                  </div>
                </div>
              </div>
            </div>

            <div className="grid grid-cols-4 gap-4 text-sm">
              <div>
                <p className="text-gray-400 text-xs mb-1">Capacity</p>
                <p className="font-semibold">{formatCurrency(channel.capacity)}</p>
              </div>
              <div>
                <p className="text-gray-400 text-xs mb-1">Balance 1</p>
                <p className="font-semibold text-blue-400">{formatCurrency(channel.balance1)}</p>
              </div>
              <div>
                <p className="text-gray-400 text-xs mb-1">Balance 2</p>
                <p className="font-semibold text-purple-400">{formatCurrency(channel.balance2)}</p>
              </div>
              <div>
                <p className="text-gray-400 text-xs mb-1">Commitment #</p>
                <p className="font-semibold">{formatNumber(channel.commitmentNumber)}</p>
              </div>
            </div>

            {/* Balance Bar */}
            <div className="mt-4">
              <div className="flex justify-between text-xs text-gray-400 mb-1">
                <span>Balance Distribution</span>
                <span>
                  {((channel.balance1 / channel.capacity) * 100).toFixed(1)}% /
                  {((channel.balance2 / channel.capacity) * 100).toFixed(1)}%
                </span>
              </div>
              <div className="h-2 bg-white/10 rounded-full overflow-hidden flex">
                <div
                  className="bg-blue-500 transition-all"
                  style={{ width: `${(channel.balance1 / channel.capacity) * 100}%` }}
                />
                <div
                  className="bg-purple-500 transition-all"
                  style={{ width: `${(channel.balance2 / channel.capacity) * 100}%` }}
                />
              </div>
            </div>

            <div className="mt-3 flex items-center justify-between text-xs text-gray-400">
              <span>Last update: {new Date(channel.lastUpdate).toLocaleString()}</span>
              {channel.status === 'disputed' && (
                <span className="flex items-center gap-1 text-yellow-400">
                  <AlertTriangle className="w-3 h-3" />
                  Channel in dispute
                </span>
              )}
            </div>
          </div>
        ))}

        {filteredChannels.length === 0 && (
          <div className="p-12 text-center">
            <Activity className="w-12 h-12 text-gray-400 mx-auto mb-4" />
            <p className="text-gray-400">No channels found matching your criteria</p>
          </div>
        )}
      </div>
    </div>
  );
}
