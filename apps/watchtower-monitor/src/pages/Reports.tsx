'use client';

import React, { useState } from 'react';
import { useFraudDetection } from '../hooks/useFraudDetection';
import { FileText, Download, Filter, Search, AlertCircle } from 'lucide-react';
import { formatCurrency, getSeverityColor, timeAgo } from '../lib/utils';
import type { FraudAlert } from '../types';

export default function Reports() {
  const { alerts, fraudStats } = useFraudDetection();
  const [searchTerm, setSearchTerm] = useState('');
  const [severityFilter, setSeverityFilter] = useState<string>('all');
  const [statusFilter, setStatusFilter] = useState<string>('all');

  const filteredAlerts = alerts.filter((alert) => {
    const matchesSearch =
      alert.channelId.toLowerCase().includes(searchTerm.toLowerCase()) ||
      alert.type.toLowerCase().includes(searchTerm.toLowerCase()) ||
      alert.description.toLowerCase().includes(searchTerm.toLowerCase());

    const matchesSeverity = severityFilter === 'all' || alert.severity === severityFilter;
    const matchesStatus =
      statusFilter === 'all' ||
      (statusFilter === 'resolved' && alert.resolved) ||
      (statusFilter === 'pending' && !alert.resolved);

    return matchesSearch && matchesSeverity && matchesStatus;
  });

  const exportReport = () => {
    const data = filteredAlerts.map((alert) => ({
      timestamp: alert.timestamp.toISOString(),
      channelId: alert.channelId,
      type: alert.type,
      severity: alert.severity,
      description: alert.description,
      evidenceHash: alert.evidenceHash,
      resolved: alert.resolved,
      penaltyAmount: alert.penaltyAmount,
    }));

    const blob = new Blob([JSON.stringify(data, null, 2)], { type: 'application/json' });
    const url = URL.createObjectURL(blob);
    const a = document.createElement('a');
    a.href = url;
    a.download = `fraud-report-${new Date().toISOString().split('T')[0]}.json`;
    document.body.appendChild(a);
    a.click();
    document.body.removeChild(a);
    URL.revokeObjectURL(url);
  };

  return (
    <div className="min-h-screen bg-gradient-to-br from-slate-900 via-blue-900 to-slate-900 text-white p-6">
      <div className="max-w-7xl mx-auto">
        {/* Header */}
        <div className="mb-8">
          <div className="flex items-center justify-between mb-2">
            <h1 className="text-4xl font-bold flex items-center gap-3">
              <FileText className="w-10 h-10 text-blue-400" />
              Fraud Reports
            </h1>
            <button
              onClick={exportReport}
              className="flex items-center gap-2 px-4 py-2 bg-blue-500 hover:bg-blue-600 rounded-lg transition-colors"
            >
              <Download className="w-4 h-4" />
              Export Report
            </button>
          </div>
          <p className="text-gray-400">Comprehensive fraud detection and intervention reports</p>
        </div>

        {/* Statistics */}
        <div className="grid grid-cols-1 md:grid-cols-4 gap-4 mb-6">
          <div className="bg-white/10 backdrop-blur-lg rounded-xl p-4 border border-white/20">
            <p className="text-sm text-gray-400">Total Detections</p>
            <p className="text-2xl font-bold mt-1">{fraudStats.totalDetections}</p>
          </div>
          <div className="bg-white/10 backdrop-blur-lg rounded-xl p-4 border border-white/20">
            <p className="text-sm text-gray-400">Successful Interventions</p>
            <p className="text-2xl font-bold mt-1 text-green-400">
              {fraudStats.successfulInterventions}
            </p>
          </div>
          <div className="bg-white/10 backdrop-blur-lg rounded-xl p-4 border border-white/20">
            <p className="text-sm text-gray-400">False Positives</p>
            <p className="text-2xl font-bold mt-1 text-yellow-400">{fraudStats.falsePositives}</p>
          </div>
          <div className="bg-white/10 backdrop-blur-lg rounded-xl p-4 border border-white/20">
            <p className="text-sm text-gray-400">Accuracy Rate</p>
            <p className="text-2xl font-bold mt-1 text-purple-400">{fraudStats.accuracy.toFixed(1)}%</p>
          </div>
        </div>

        {/* Filters */}
        <div className="bg-white/10 backdrop-blur-lg rounded-xl p-6 border border-white/20 mb-6">
          <div className="grid grid-cols-1 md:grid-cols-3 gap-4">
            <div className="relative">
              <Search className="absolute left-3 top-1/2 -translate-y-1/2 w-4 h-4 text-gray-400" />
              <input
                type="text"
                placeholder="Search by channel, type, or description..."
                value={searchTerm}
                onChange={(e) => setSearchTerm(e.target.value)}
                className="w-full pl-10 pr-4 py-2 bg-white/5 border border-white/20 rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-500"
              />
            </div>

            <select
              value={severityFilter}
              onChange={(e) => setSeverityFilter(e.target.value)}
              className="px-4 py-2 bg-white/5 border border-white/20 rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-500"
            >
              <option value="all">All Severities</option>
              <option value="low">Low</option>
              <option value="medium">Medium</option>
              <option value="high">High</option>
              <option value="critical">Critical</option>
            </select>

            <select
              value={statusFilter}
              onChange={(e) => setStatusFilter(e.target.value)}
              className="px-4 py-2 bg-white/5 border border-white/20 rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-500"
            >
              <option value="all">All Status</option>
              <option value="pending">Pending</option>
              <option value="resolved">Resolved</option>
            </select>
          </div>
        </div>

        {/* Alerts Table */}
        <div className="bg-white/10 backdrop-blur-lg rounded-xl border border-white/20 overflow-hidden">
          <div className="overflow-x-auto">
            <table className="w-full">
              <thead className="bg-white/5">
                <tr>
                  <th className="px-6 py-4 text-left text-sm font-semibold">Timestamp</th>
                  <th className="px-6 py-4 text-left text-sm font-semibold">Channel ID</th>
                  <th className="px-6 py-4 text-left text-sm font-semibold">Type</th>
                  <th className="px-6 py-4 text-left text-sm font-semibold">Severity</th>
                  <th className="px-6 py-4 text-left text-sm font-semibold">Description</th>
                  <th className="px-6 py-4 text-left text-sm font-semibold">Penalty</th>
                  <th className="px-6 py-4 text-left text-sm font-semibold">Status</th>
                </tr>
              </thead>
              <tbody className="divide-y divide-white/10">
                {filteredAlerts.map((alert) => (
                  <tr key={alert.id} className="hover:bg-white/5 transition-colors">
                    <td className="px-6 py-4 text-sm text-gray-300">
                      <div>{new Date(alert.timestamp).toLocaleDateString()}</div>
                      <div className="text-xs text-gray-500">
                        {new Date(alert.timestamp).toLocaleTimeString()}
                      </div>
                    </td>
                    <td className="px-6 py-4 text-sm font-mono">
                      {alert.channelId.slice(0, 12)}...
                    </td>
                    <td className="px-6 py-4 text-sm">
                      <span className="px-2 py-1 bg-blue-500/20 text-blue-400 rounded text-xs">
                        {alert.type.replace(/_/g, ' ')}
                      </span>
                    </td>
                    <td className="px-6 py-4 text-sm">
                      <span className={`px-2 py-1 rounded text-xs ${getSeverityColor(alert.severity)}`}>
                        {alert.severity.toUpperCase()}
                      </span>
                    </td>
                    <td className="px-6 py-4 text-sm text-gray-300 max-w-xs truncate">
                      {alert.description}
                    </td>
                    <td className="px-6 py-4 text-sm font-semibold text-red-400">
                      {alert.penaltyAmount ? formatCurrency(alert.penaltyAmount) : '-'}
                    </td>
                    <td className="px-6 py-4 text-sm">
                      <span
                        className={`px-2 py-1 rounded text-xs ${
                          alert.resolved
                            ? 'bg-green-500/20 text-green-400'
                            : 'bg-yellow-500/20 text-yellow-400'
                        }`}
                      >
                        {alert.resolved ? 'Resolved' : 'Pending'}
                      </span>
                    </td>
                  </tr>
                ))}
              </tbody>
            </table>

            {filteredAlerts.length === 0 && (
              <div className="text-center py-12">
                <AlertCircle className="w-12 h-12 text-gray-400 mx-auto mb-4" />
                <p className="text-gray-400">No fraud reports found matching your criteria</p>
              </div>
            )}
          </div>
        </div>

        {/* Summary */}
        <div className="mt-6 text-sm text-gray-400 text-center">
          Showing {filteredAlerts.length} of {alerts.length} total reports
        </div>
      </div>
    </div>
  );
}
