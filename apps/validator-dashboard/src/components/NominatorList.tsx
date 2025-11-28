import React, { useState } from 'react';
import { Search, ArrowUpDown, ExternalLink, TrendingUp, TrendingDown } from 'lucide-react';
import type { Nominator } from '@/types';
import { formatTokenAmount, formatAddress, formatTimeAgo } from '@/utils/format';

interface NominatorListProps {
  nominators: Nominator[];
  isLoading?: boolean;
}

export default function NominatorList({ nominators, isLoading = false }: NominatorListProps) {
  const [searchTerm, setSearchTerm] = useState('');
  const [sortBy, setSortBy] = useState<'stake' | 'reward' | 'since'>('stake');
  const [sortOrder, setSortOrder] = useState<'asc' | 'desc'>('desc');

  const filteredNominators = nominators
    .filter((nom) =>
      nom.address.toLowerCase().includes(searchTerm.toLowerCase())
    )
    .sort((a, b) => {
      let comparison = 0;
      switch (sortBy) {
        case 'stake':
          comparison = a.stake > b.stake ? 1 : -1;
          break;
        case 'reward':
          comparison = a.lastReward > b.lastReward ? 1 : -1;
          break;
        case 'since':
          comparison = a.since - b.since;
          break;
      }
      return sortOrder === 'asc' ? comparison : -comparison;
    });

  const totalStake = nominators.reduce((sum, nom) => sum + nom.stake, 0n);

  const toggleSort = (field: 'stake' | 'reward' | 'since') => {
    if (sortBy === field) {
      setSortOrder(sortOrder === 'asc' ? 'desc' : 'asc');
    } else {
      setSortBy(field);
      setSortOrder('desc');
    }
  };

  if (isLoading) {
    return (
      <div className="bg-white rounded-lg shadow-md p-6">
        <div className="animate-pulse space-y-4">
          <div className="h-6 bg-gray-200 rounded w-1/4"></div>
          <div className="h-10 bg-gray-200 rounded"></div>
          {[...Array(5)].map((_, i) => (
            <div key={i} className="h-16 bg-gray-200 rounded"></div>
          ))}
        </div>
      </div>
    );
  }

  return (
    <div className="bg-white rounded-lg shadow-md overflow-hidden">
      <div className="p-6 border-b border-gray-200">
        <div className="flex items-center justify-between mb-4">
          <h2 className="text-xl font-bold text-gray-900">
            Nominators ({nominators.length})
          </h2>
          <div className="text-sm text-gray-600">
            Total Stake: <span className="font-semibold">{formatTokenAmount(totalStake)}</span>
          </div>
        </div>

        <div className="relative">
          <Search className="absolute left-3 top-1/2 transform -translate-y-1/2 text-gray-400 w-5 h-5" />
          <input
            type="text"
            placeholder="Search by address..."
            value={searchTerm}
            onChange={(e) => setSearchTerm(e.target.value)}
            className="w-full pl-10 pr-4 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-etrid-500 focus:border-transparent"
          />
        </div>
      </div>

      <div className="overflow-x-auto">
        <table className="w-full">
          <thead className="bg-gray-50">
            <tr>
              <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
                Nominator
              </th>
              <th
                className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider cursor-pointer hover:bg-gray-100"
                onClick={() => toggleSort('stake')}
              >
                <div className="flex items-center space-x-1">
                  <span>Stake</span>
                  <ArrowUpDown className="w-4 h-4" />
                </div>
              </th>
              <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
                Share
              </th>
              <th
                className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider cursor-pointer hover:bg-gray-100"
                onClick={() => toggleSort('reward')}
              >
                <div className="flex items-center space-x-1">
                  <span>Last Reward</span>
                  <ArrowUpDown className="w-4 h-4" />
                </div>
              </th>
              <th
                className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider cursor-pointer hover:bg-gray-100"
                onClick={() => toggleSort('since')}
              >
                <div className="flex items-center space-x-1">
                  <span>Since</span>
                  <ArrowUpDown className="w-4 h-4" />
                </div>
              </th>
              <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
                Status
              </th>
            </tr>
          </thead>
          <tbody className="bg-white divide-y divide-gray-200">
            {filteredNominators.length === 0 ? (
              <tr>
                <td colSpan={6} className="px-6 py-8 text-center text-gray-500">
                  {searchTerm ? 'No nominators found matching your search' : 'No nominators yet'}
                </td>
              </tr>
            ) : (
              filteredNominators.map((nominator, index) => {
                const sharePercentage = Number(nominator.stake) / Number(totalStake) * 100;
                return (
                  <tr key={nominator.address} className="hover:bg-gray-50">
                    <td className="px-6 py-4 whitespace-nowrap">
                      <div className="flex items-center space-x-2">
                        <div className="flex-shrink-0 w-8 h-8 bg-gradient-to-br from-etrid-400 to-etrid-600 rounded-full flex items-center justify-center text-white text-xs font-bold">
                          {index + 1}
                        </div>
                        <div>
                          <div className="text-sm font-medium text-gray-900">
                            {formatAddress(nominator.address, 6)}
                          </div>
                        </div>
                        <a
                          href={`https://polkadot.js.org/apps/?rpc=${process.env.NEXT_PUBLIC_WS_PROVIDER}#/explorer/query/${nominator.address}`}
                          target="_blank"
                          rel="noopener noreferrer"
                          className="text-gray-400 hover:text-etrid-600"
                        >
                          <ExternalLink className="w-4 h-4" />
                        </a>
                      </div>
                    </td>
                    <td className="px-6 py-4 whitespace-nowrap">
                      <div className="text-sm font-semibold text-gray-900">
                        {formatTokenAmount(nominator.stake)}
                      </div>
                    </td>
                    <td className="px-6 py-4 whitespace-nowrap">
                      <div className="flex items-center space-x-1">
                        <div className="text-sm text-gray-900">{sharePercentage.toFixed(2)}%</div>
                        {sharePercentage > 10 ? (
                          <TrendingUp className="w-4 h-4 text-success-500" />
                        ) : sharePercentage < 2 ? (
                          <TrendingDown className="w-4 h-4 text-danger-500" />
                        ) : null}
                      </div>
                    </td>
                    <td className="px-6 py-4 whitespace-nowrap">
                      <div className="text-sm text-gray-900">
                        {formatTokenAmount(nominator.lastReward)}
                      </div>
                    </td>
                    <td className="px-6 py-4 whitespace-nowrap">
                      <div className="text-sm text-gray-500">
                        {formatTimeAgo(nominator.since)}
                      </div>
                    </td>
                    <td className="px-6 py-4 whitespace-nowrap">
                      <span
                        className={`px-2 inline-flex text-xs leading-5 font-semibold rounded-full ${
                          nominator.active
                            ? 'bg-success-50 text-success-700'
                            : 'bg-gray-100 text-gray-700'
                        }`}
                      >
                        {nominator.active ? 'Active' : 'Inactive'}
                      </span>
                    </td>
                  </tr>
                );
              })
            )}
          </tbody>
        </table>
      </div>
    </div>
  );
}
