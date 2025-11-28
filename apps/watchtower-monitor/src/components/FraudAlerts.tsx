'use client';

import React, { useState } from 'react';
import { AlertTriangle, Shield, Clock, CheckCircle, XCircle, ExternalLink } from 'lucide-react';
import { formatCurrency, getSeverityColor, timeAgo, truncateAddress } from '../lib/utils';
import type { FraudAlert } from '../types';

interface FraudAlertsProps {
  alerts: FraudAlert[];
}

export default function FraudAlerts({ alerts }: FraudAlertsProps) {
  const [selectedAlert, setSelectedAlert] = useState<FraudAlert | null>(null);

  const unresolvedAlerts = alerts.filter((alert) => !alert.resolved);
  const resolvedAlerts = alerts.filter((alert) => alert.resolved);

  return (
    <div className="space-y-6">
      {/* Unresolved Alerts */}
      <div className="bg-white/10 backdrop-blur-lg rounded-xl border border-white/20">
        <div className="p-6 border-b border-white/20">
          <div className="flex items-center justify-between">
            <h2 className="text-xl font-semibold flex items-center gap-2">
              <AlertTriangle className="w-5 h-5 text-red-400" />
              Active Alerts
            </h2>
            <span className="px-3 py-1 bg-red-500/20 text-red-400 rounded-lg text-sm font-medium">
              {unresolvedAlerts.length} active
            </span>
          </div>
        </div>

        <div className="divide-y divide-white/10">
          {unresolvedAlerts.length === 0 ? (
            <div className="p-12 text-center">
              <Shield className="w-12 h-12 text-green-400 mx-auto mb-4" />
              <p className="text-gray-400">No active fraud alerts - all channels secure</p>
            </div>
          ) : (
            unresolvedAlerts.map((alert) => (
              <div
                key={alert.id}
                className="p-6 hover:bg-white/5 transition-colors cursor-pointer"
                onClick={() => setSelectedAlert(alert)}
              >
                <div className="flex items-start justify-between mb-3">
                  <div className="flex-1">
                    <div className="flex items-center gap-2 mb-2">
                      <h3 className="font-semibold">{alert.type.replace(/_/g, ' ').toUpperCase()}</h3>
                      <span className={`text-xs px-2 py-1 rounded ${getSeverityColor(alert.severity)}`}>
                        {alert.severity}
                      </span>
                    </div>
                    <p className="text-sm text-gray-400 mb-2">{alert.description}</p>
                    <div className="flex items-center gap-4 text-xs text-gray-500">
                      <span className="flex items-center gap-1">
                        <Clock className="w-3 h-3" />
                        {timeAgo(alert.timestamp)}
                      </span>
                      <span>Channel: {truncateAddress(alert.channelId)}</span>
                      {alert.reportedBy && <span>Reporter: {truncateAddress(alert.reportedBy)}</span>}
                    </div>
                  </div>
                  {alert.penaltyAmount && (
                    <div className="text-right">
                      <p className="text-xs text-gray-400">Penalty</p>
                      <p className="text-lg font-bold text-red-400">
                        {formatCurrency(alert.penaltyAmount)}
                      </p>
                    </div>
                  )}
                </div>

                <div className="flex items-center gap-2 mt-4">
                  <button className="px-4 py-2 bg-green-500 hover:bg-green-600 text-white rounded-lg text-sm font-medium transition-colors">
                    Intervene
                  </button>
                  <button className="px-4 py-2 bg-yellow-500 hover:bg-yellow-600 text-white rounded-lg text-sm font-medium transition-colors">
                    Flag False Positive
                  </button>
                  <button className="px-4 py-2 bg-blue-500 hover:bg-blue-600 text-white rounded-lg text-sm font-medium transition-colors flex items-center gap-1">
                    View Evidence
                    <ExternalLink className="w-3 h-3" />
                  </button>
                </div>
              </div>
            ))
          )}
        </div>
      </div>

      {/* Resolved Alerts */}
      <div className="bg-white/10 backdrop-blur-lg rounded-xl border border-white/20">
        <div className="p-6 border-b border-white/20">
          <div className="flex items-center justify-between">
            <h2 className="text-xl font-semibold flex items-center gap-2">
              <CheckCircle className="w-5 h-5 text-green-400" />
              Resolved Alerts
            </h2>
            <span className="px-3 py-1 bg-green-500/20 text-green-400 rounded-lg text-sm font-medium">
              {resolvedAlerts.length} resolved
            </span>
          </div>
        </div>

        <div className="divide-y divide-white/10 max-h-96 overflow-y-auto">
          {resolvedAlerts.length === 0 ? (
            <div className="p-8 text-center">
              <p className="text-gray-400 text-sm">No resolved alerts yet</p>
            </div>
          ) : (
            resolvedAlerts.slice(0, 10).map((alert) => (
              <div
                key={alert.id}
                className="p-4 hover:bg-white/5 transition-colors cursor-pointer"
                onClick={() => setSelectedAlert(alert)}
              >
                <div className="flex items-center justify-between">
                  <div className="flex-1">
                    <div className="flex items-center gap-2 mb-1">
                      <CheckCircle className="w-4 h-4 text-green-400" />
                      <h3 className="font-medium text-sm">{alert.type.replace(/_/g, ' ')}</h3>
                      <span className={`text-xs px-2 py-1 rounded ${getSeverityColor(alert.severity)}`}>
                        {alert.severity}
                      </span>
                    </div>
                    <p className="text-xs text-gray-400 ml-6">
                      {truncateAddress(alert.channelId)} • {timeAgo(alert.timestamp)}
                    </p>
                  </div>
                  {alert.penaltyAmount && (
                    <span className="text-sm font-semibold text-green-400">
                      +{formatCurrency(alert.penaltyAmount)}
                    </span>
                  )}
                </div>
              </div>
            ))
          )}
        </div>
      </div>

      {/* Alert Detail Modal */}
      {selectedAlert && (
        <div
          className="fixed inset-0 bg-black/50 backdrop-blur-sm flex items-center justify-center z-50 p-6"
          onClick={() => setSelectedAlert(null)}
        >
          <div
            className="bg-slate-800 rounded-xl border border-white/20 max-w-2xl w-full p-6"
            onClick={(e) => e.stopPropagation()}
          >
            <div className="flex items-start justify-between mb-6">
              <div>
                <h2 className="text-2xl font-bold mb-2">Alert Details</h2>
                <div className="flex items-center gap-2">
                  <span className={`text-xs px-2 py-1 rounded ${getSeverityColor(selectedAlert.severity)}`}>
                    {selectedAlert.severity}
                  </span>
                  <span
                    className={`text-xs px-2 py-1 rounded ${
                      selectedAlert.resolved
                        ? 'bg-green-500/20 text-green-400'
                        : 'bg-yellow-500/20 text-yellow-400'
                    }`}
                  >
                    {selectedAlert.resolved ? 'Resolved' : 'Pending'}
                  </span>
                </div>
              </div>
              <button
                onClick={() => setSelectedAlert(null)}
                className="text-gray-400 hover:text-white transition-colors"
              >
                <XCircle className="w-6 h-6" />
              </button>
            </div>

            <div className="space-y-4">
              <div>
                <p className="text-sm text-gray-400 mb-1">Alert Type</p>
                <p className="font-semibold">{selectedAlert.type.replace(/_/g, ' ').toUpperCase()}</p>
              </div>

              <div>
                <p className="text-sm text-gray-400 mb-1">Description</p>
                <p>{selectedAlert.description}</p>
              </div>

              <div className="grid grid-cols-2 gap-4">
                <div>
                  <p className="text-sm text-gray-400 mb-1">Channel ID</p>
                  <p className="font-mono text-sm">{selectedAlert.channelId}</p>
                </div>
                <div>
                  <p className="text-sm text-gray-400 mb-1">Timestamp</p>
                  <p className="text-sm">{new Date(selectedAlert.timestamp).toLocaleString()}</p>
                </div>
              </div>

              <div>
                <p className="text-sm text-gray-400 mb-1">Evidence Hash</p>
                <p className="font-mono text-sm break-all">{selectedAlert.evidenceHash}</p>
              </div>

              {selectedAlert.penaltyAmount && (
                <div>
                  <p className="text-sm text-gray-400 mb-1">Penalty Amount</p>
                  <p className="text-xl font-bold text-red-400">
                    {formatCurrency(selectedAlert.penaltyAmount)}
                  </p>
                </div>
              )}

              {selectedAlert.reportedBy && (
                <div>
                  <p className="text-sm text-gray-400 mb-1">Reported By</p>
                  <p className="font-mono text-sm">{selectedAlert.reportedBy}</p>
                </div>
              )}
            </div>

            <div className="mt-6 flex gap-3">
              <button className="flex-1 px-4 py-2 bg-blue-500 hover:bg-blue-600 text-white rounded-lg font-medium transition-colors">
                View on Explorer
              </button>
              <button
                onClick={() => setSelectedAlert(null)}
                className="px-4 py-2 bg-white/10 hover:bg-white/20 text-white rounded-lg font-medium transition-colors"
              >
                Close
              </button>
            </div>
          </div>
        </div>
      )}
    </div>
  );
}
