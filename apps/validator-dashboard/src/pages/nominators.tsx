import React, { useState } from 'react';
import Head from 'next/head';
import Layout from '@/components/Layout';
import { useValidatorStats } from '@/hooks/useValidatorStats';
import { Search, Download, TrendingUp, TrendingDown, Users, Coins } from 'lucide-react';
import { formatDistanceToNow } from 'date-fns';
import type { Nominator } from '@/types';

export default function NominatorsPage() {
  const [validatorAddress] = useState<string | undefined>(
    process.env.NEXT_PUBLIC_VALIDATOR_ADDRESS
  );

  const { isConnected, isLoading, nominators } = useValidatorStats(validatorAddress);

  const [searchTerm, setSearchTerm] = useState('');
  const [sortBy, setSortBy] = useState<'stake' | 'rewards' | 'time'>('stake');
  const [sortOrder, setSortOrder] = useState<'asc' | 'desc'>('desc');

  // Filter and sort nominators
  const filteredNominators = nominators
    .filter((nom) =>
      nom.address.toLowerCase().includes(searchTerm.toLowerCase())
    )
    .sort((a, b) => {
      let compareValue = 0;
      switch (sortBy) {
        case 'stake':
          compareValue = a.stake > b.stake ? 1 : -1;
          break;
        case 'rewards':
          compareValue = a.lastReward > b.lastReward ? 1 : -1;
          break;
        case 'time':
          compareValue = a.since > b.since ? 1 : -1;
          break;
      }
      return sortOrder === 'asc' ? compareValue : -compareValue;
    });

  const totalStake = nominators.reduce((sum, nom) => sum + nom.stake, BigInt(0));

  const formatBalance = (balance: bigint) => {
    return (Number(balance) / 1e18).toFixed(4);
  };

  const formatAddress = (address: string) => {
    return `${address.slice(0, 6)}...${address.slice(-4)}`;
  };

  const handleExportData = () => {
    const csvData = [
      ['Address', 'Stake (ETR)', 'Last Reward (ETR)', 'Since', 'Active'].join(','),
      ...filteredNominators.map((nom) =>
        [
          nom.address,
          formatBalance(nom.stake),
          formatBalance(nom.lastReward),
          new Date(nom.since).toISOString(),
          nom.active ? 'Yes' : 'No',
        ].join(',')
      ),
    ].join('\n');

    const blob = new Blob([csvData], { type: 'text/csv' });
    const url = window.URL.createObjectURL(blob);
    const a = document.createElement('a');
    a.href = url;
    a.download = `nominators-${Date.now()}.csv`;
    a.click();
  };

  const handleConnectWallet = () => {
    console.log('Connect wallet');
  };

  return (
    <>
      <Head>
        <title>Nominators - Validator Dashboard</title>
        <meta name="description" content="Manage your validator nominators" />
      </Head>

      <Layout isConnected={isConnected} onConnectWallet={handleConnectWallet}>
        <div className="space-y-6">
          {/* Page Header */}
          <div className="flex flex-col md:flex-row md:items-center md:justify-between gap-4">
            <div>
              <h1 className="text-3xl font-bold text-gray-900">Nominators</h1>
              <p className="text-gray-600 mt-1">
                View and manage all nominators staking to your validator
              </p>
            </div>

            <button
              onClick={handleExportData}
              className="flex items-center space-x-2 px-4 py-2 bg-etrid-600 text-white rounded-lg hover:bg-etrid-700 transition-colors"
            >
              <Download className="w-4 h-4" />
              <span>Export Data</span>
            </button>
          </div>

          {/* Summary Cards */}
          <div className="grid grid-cols-1 md:grid-cols-3 gap-6">
            <div className="bg-white rounded-lg shadow-md p-6">
              <div className="flex items-center justify-between">
                <div>
                  <p className="text-sm text-gray-600 mb-1">Total Nominators</p>
                  <p className="text-3xl font-bold text-gray-900">{nominators.length}</p>
                  <p className="text-xs text-gray-500 mt-1">
                    {nominators.filter((n) => n.active).length} active
                  </p>
                </div>
                <div className="w-12 h-12 bg-etrid-100 rounded-lg flex items-center justify-center">
                  <Users className="w-6 h-6 text-etrid-600" />
                </div>
              </div>
            </div>

            <div className="bg-white rounded-lg shadow-md p-6">
              <div className="flex items-center justify-between">
                <div>
                  <p className="text-sm text-gray-600 mb-1">Total Stake</p>
                  <p className="text-3xl font-bold text-gray-900">
                    {formatBalance(totalStake)}
                  </p>
                  <p className="text-xs text-gray-500 mt-1">ETR</p>
                </div>
                <div className="w-12 h-12 bg-purple-100 rounded-lg flex items-center justify-center">
                  <Coins className="w-6 h-6 text-purple-600" />
                </div>
              </div>
            </div>

            <div className="bg-white rounded-lg shadow-md p-6">
              <div className="flex items-center justify-between">
                <div>
                  <p className="text-sm text-gray-600 mb-1">Average Stake</p>
                  <p className="text-3xl font-bold text-gray-900">
                    {nominators.length > 0
                      ? formatBalance(totalStake / BigInt(nominators.length))
                      : '0.0000'}
                  </p>
                  <p className="text-xs text-gray-500 mt-1">ETR per nominator</p>
                </div>
                <div className="w-12 h-12 bg-success-100 rounded-lg flex items-center justify-center">
                  <TrendingUp className="w-6 h-6 text-success-600" />
                </div>
              </div>
            </div>
          </div>

          {/* Filters and Search */}
          <div className="bg-white rounded-lg shadow-md p-6">
            <div className="flex flex-col md:flex-row gap-4">
              {/* Search */}
              <div className="flex-1">
                <div className="relative">
                  <Search className="absolute left-3 top-1/2 transform -translate-y-1/2 text-gray-400 w-5 h-5" />
                  <input
                    type="text"
                    placeholder="Search by address..."
                    value={searchTerm}
                    onChange={(e) => setSearchTerm(e.target.value)}
                    className="w-full pl-10 pr-4 py-2 border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-etrid-500 focus:border-transparent"
                  />
                </div>
              </div>

              {/* Sort By */}
              <div className="flex items-center space-x-2">
                <label className="text-sm text-gray-600 whitespace-nowrap">Sort by:</label>
                <select
                  value={sortBy}
                  onChange={(e) => setSortBy(e.target.value as any)}
                  className="px-4 py-2 border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-etrid-500"
                >
                  <option value="stake">Stake</option>
                  <option value="rewards">Rewards</option>
                  <option value="time">Time</option>
                </select>
              </div>

              {/* Sort Order */}
              <button
                onClick={() => setSortOrder(sortOrder === 'asc' ? 'desc' : 'asc')}
                className="px-4 py-2 border border-gray-300 rounded-lg hover:bg-gray-50 transition-colors flex items-center space-x-2"
              >
                {sortOrder === 'desc' ? (
                  <TrendingDown className="w-4 h-4" />
                ) : (
                  <TrendingUp className="w-4 h-4" />
                )}
                <span className="text-sm">
                  {sortOrder === 'desc' ? 'Descending' : 'Ascending'}
                </span>
              </button>
            </div>
          </div>

          {/* Nominators Table */}
          <div className="bg-white rounded-lg shadow-md overflow-hidden">
            <div className="overflow-x-auto">
              <table className="w-full">
                <thead className="bg-gray-50 border-b border-gray-200">
                  <tr>
                    <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
                      Nominator
                    </th>
                    <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
                      Stake
                    </th>
                    <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
                      % of Total
                    </th>
                    <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
                      Last Reward
                    </th>
                    <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
                      Since
                    </th>
                    <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
                      Status
                    </th>
                  </tr>
                </thead>
                <tbody className="divide-y divide-gray-200">
                  {isLoading ? (
                    <tr>
                      <td colSpan={6} className="px-6 py-12 text-center">
                        <div className="flex justify-center">
                          <div className="w-8 h-8 border-4 border-etrid-600 border-t-transparent rounded-full animate-spin"></div>
                        </div>
                      </td>
                    </tr>
                  ) : filteredNominators.length === 0 ? (
                    <tr>
                      <td colSpan={6} className="px-6 py-12 text-center text-gray-500">
                        {searchTerm ? 'No nominators found matching your search' : 'No nominators yet'}
                      </td>
                    </tr>
                  ) : (
                    filteredNominators.map((nominator) => (
                      <tr
                        key={nominator.address}
                        className="hover:bg-gray-50 transition-colors"
                      >
                        <td className="px-6 py-4 whitespace-nowrap">
                          <div className="flex items-center">
                            <div className="w-10 h-10 bg-gradient-to-br from-etrid-500 to-etrid-700 rounded-full flex items-center justify-center text-white font-semibold mr-3">
                              {nominator.address.slice(0, 2)}
                            </div>
                            <div>
                              <p className="text-sm font-medium text-gray-900">
                                {formatAddress(nominator.address)}
                              </p>
                              <p className="text-xs text-gray-500 font-mono">
                                {nominator.address}
                              </p>
                            </div>
                          </div>
                        </td>
                        <td className="px-6 py-4 whitespace-nowrap">
                          <p className="text-sm font-semibold text-gray-900">
                            {formatBalance(nominator.stake)} ETR
                          </p>
                        </td>
                        <td className="px-6 py-4 whitespace-nowrap">
                          <div className="flex items-center space-x-2">
                            <div className="flex-1 bg-gray-200 rounded-full h-2 max-w-[100px]">
                              <div
                                className="bg-etrid-600 h-2 rounded-full"
                                style={{
                                  width: `${(Number(nominator.stake) / Number(totalStake)) * 100}%`,
                                }}
                              ></div>
                            </div>
                            <span className="text-sm text-gray-600">
                              {((Number(nominator.stake) / Number(totalStake)) * 100).toFixed(2)}%
                            </span>
                          </div>
                        </td>
                        <td className="px-6 py-4 whitespace-nowrap">
                          <p className="text-sm text-gray-900">
                            {formatBalance(nominator.lastReward)} ETR
                          </p>
                        </td>
                        <td className="px-6 py-4 whitespace-nowrap">
                          <p className="text-sm text-gray-900">
                            {formatDistanceToNow(new Date(nominator.since), {
                              addSuffix: true,
                            })}
                          </p>
                        </td>
                        <td className="px-6 py-4 whitespace-nowrap">
                          <span
                            className={`inline-flex items-center px-2.5 py-0.5 rounded-full text-xs font-medium ${
                              nominator.active
                                ? 'bg-success-100 text-success-800'
                                : 'bg-gray-100 text-gray-800'
                            }`}
                          >
                            {nominator.active ? 'Active' : 'Inactive'}
                          </span>
                        </td>
                      </tr>
                    ))
                  )}
                </tbody>
              </table>
            </div>

            {/* Pagination Info */}
            {filteredNominators.length > 0 && (
              <div className="px-6 py-4 bg-gray-50 border-t border-gray-200">
                <p className="text-sm text-gray-600">
                  Showing {filteredNominators.length} of {nominators.length} nominators
                </p>
              </div>
            )}
          </div>
        </div>
      </Layout>
    </>
  );
}
