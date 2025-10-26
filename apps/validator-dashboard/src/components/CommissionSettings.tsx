import React, { useState } from 'react';
import { Settings, Save, AlertCircle, Info, TrendingUp } from 'lucide-react';
import type { ValidatorSettings } from '@/types';
import { formatCommission } from '@/utils/format';

interface CommissionSettingsProps {
  currentCommission: number;
  settings?: ValidatorSettings;
  onUpdate?: (newCommission: number, settings: Partial<ValidatorSettings>) => Promise<void>;
  isLoading?: boolean;
}

export default function CommissionSettings({
  currentCommission,
  settings,
  onUpdate,
  isLoading = false,
}: CommissionSettingsProps) {
  const [commission, setCommission] = useState(currentCommission / 10000000);
  const [paymentDestination, setPaymentDestination] = useState<string>(
    settings?.paymentDestination || 'staked'
  );
  const [autoCompound, setAutoCompound] = useState(settings?.autoCompound ?? true);
  const [isSaving, setIsSaving] = useState(false);
  const [showSuccess, setShowSuccess] = useState(false);
  const [error, setError] = useState<string | null>(null);

  const handleSave = async () => {
    try {
      setIsSaving(true);
      setError(null);

      const newCommission = Math.floor(commission * 10000000);

      if (onUpdate) {
        await onUpdate(newCommission, {
          commission: newCommission,
          paymentDestination: paymentDestination as any,
          autoCompound,
        });
      }

      setShowSuccess(true);
      setTimeout(() => setShowSuccess(false), 3000);
    } catch (err) {
      setError(err instanceof Error ? err.message : 'Failed to update settings');
    } finally {
      setIsSaving(false);
    }
  };

  const hasChanges =
    commission !== currentCommission / 10000000 ||
    paymentDestination !== settings?.paymentDestination ||
    autoCompound !== settings?.autoCompound;

  const estimatedAnnualIncome = (stake: number) => {
    const dailyReward = (stake * 0.15) / 365; // Assuming 15% APY
    const validatorCut = dailyReward * (commission / 100);
    return validatorCut * 365;
  };

  if (isLoading) {
    return (
      <div className="bg-white rounded-lg shadow-md p-6">
        <div className="animate-pulse space-y-4">
          <div className="h-6 bg-gray-200 rounded w-1/4"></div>
          <div className="h-32 bg-gray-200 rounded"></div>
        </div>
      </div>
    );
  }

  return (
    <div className="bg-white rounded-lg shadow-md overflow-hidden">
      <div className="p-6 border-b border-gray-200">
        <div className="flex items-center space-x-3">
          <div className="p-2 bg-etrid-50 rounded-lg">
            <Settings className="w-6 h-6 text-etrid-600" />
          </div>
          <div>
            <h2 className="text-xl font-bold text-gray-900">Commission Settings</h2>
            <p className="text-sm text-gray-600">
              Current: {formatCommission(currentCommission)}
            </p>
          </div>
        </div>
      </div>

      <div className="p-6 space-y-6">
        {error && (
          <div className="bg-danger-50 border border-danger-200 rounded-lg p-4 flex items-start space-x-3">
            <AlertCircle className="w-5 h-5 text-danger-600 flex-shrink-0 mt-0.5" />
            <div>
              <p className="text-sm font-medium text-danger-800">Error</p>
              <p className="text-sm text-danger-700">{error}</p>
            </div>
          </div>
        )}

        {showSuccess && (
          <div className="bg-success-50 border border-success-200 rounded-lg p-4 flex items-start space-x-3">
            <Info className="w-5 h-5 text-success-600 flex-shrink-0 mt-0.5" />
            <div>
              <p className="text-sm font-medium text-success-800">Success</p>
              <p className="text-sm text-success-700">Settings updated successfully</p>
            </div>
          </div>
        )}

        <div className="space-y-4">
          <div>
            <label className="block text-sm font-medium text-gray-700 mb-2">
              Commission Rate (%)
            </label>
            <div className="space-y-2">
              <input
                type="range"
                min="0"
                max="100"
                step="0.1"
                value={commission}
                onChange={(e) => setCommission(parseFloat(e.target.value))}
                className="w-full h-2 bg-gray-200 rounded-lg appearance-none cursor-pointer accent-etrid-600"
              />
              <div className="flex items-center justify-between">
                <span className="text-xs text-gray-500">0%</span>
                <div className="flex items-center space-x-2">
                  <input
                    type="number"
                    min="0"
                    max="100"
                    step="0.1"
                    value={commission}
                    onChange={(e) => setCommission(parseFloat(e.target.value) || 0)}
                    className="w-20 px-2 py-1 text-center border border-gray-300 rounded focus:ring-2 focus:ring-etrid-500 focus:border-transparent"
                  />
                  <span className="text-sm font-semibold text-gray-700">%</span>
                </div>
                <span className="text-xs text-gray-500">100%</span>
              </div>
            </div>

            <div className="mt-3 bg-blue-50 border border-blue-200 rounded-lg p-3">
              <div className="flex items-start space-x-2">
                <Info className="w-4 h-4 text-blue-600 flex-shrink-0 mt-0.5" />
                <div className="text-xs text-blue-700">
                  <p className="font-medium">Commission Impact</p>
                  <p className="mt-1">
                    Lower commission attracts more nominators, but reduces your earnings per
                    stake. Consider your validator&apos;s performance and market rates.
                  </p>
                </div>
              </div>
            </div>
          </div>

          <div>
            <label className="block text-sm font-medium text-gray-700 mb-2">
              Reward Destination
            </label>
            <select
              value={paymentDestination}
              onChange={(e) => setPaymentDestination(e.target.value)}
              className="w-full px-3 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-etrid-500 focus:border-transparent"
            >
              <option value="staked">Staked (Auto-compound)</option>
              <option value="stash">Stash Account</option>
              <option value="controller">Controller Account</option>
              <option value="account">Custom Account</option>
            </select>
            <p className="mt-1 text-xs text-gray-500">
              Where validator rewards should be sent
            </p>
          </div>

          <div className="flex items-center justify-between p-4 bg-gray-50 rounded-lg">
            <div>
              <p className="text-sm font-medium text-gray-900">Auto-compound Rewards</p>
              <p className="text-xs text-gray-500">Automatically stake rewards to increase earnings</p>
            </div>
            <button
              onClick={() => setAutoCompound(!autoCompound)}
              className={`relative inline-flex h-6 w-11 items-center rounded-full transition-colors ${
                autoCompound ? 'bg-etrid-600' : 'bg-gray-300'
              }`}
            >
              <span
                className={`inline-block h-4 w-4 transform rounded-full bg-white transition-transform ${
                  autoCompound ? 'translate-x-6' : 'translate-x-1'
                }`}
              />
            </button>
          </div>

          <div className="border-t border-gray-200 pt-4">
            <h3 className="text-sm font-semibold text-gray-900 mb-3 flex items-center space-x-2">
              <TrendingUp className="w-4 h-4" />
              <span>Estimated Annual Income</span>
            </h3>
            <div className="grid grid-cols-1 md:grid-cols-3 gap-3">
              {[
                { stake: 100000, label: '100K ETRID stake' },
                { stake: 500000, label: '500K ETRID stake' },
                { stake: 1000000, label: '1M ETRID stake' },
              ].map(({ stake, label }) => (
                <div key={stake} className="bg-gradient-to-br from-etrid-50 to-etrid-100 rounded-lg p-3">
                  <p className="text-xs text-gray-600 mb-1">{label}</p>
                  <p className="text-lg font-bold text-etrid-700">
                    {estimatedAnnualIncome(stake).toLocaleString(undefined, {
                      maximumFractionDigits: 0,
                    })}{' '}
                    ETRID
                  </p>
                </div>
              ))}
            </div>
          </div>
        </div>

        <div className="flex items-center justify-between pt-4 border-t border-gray-200">
          <p className="text-sm text-gray-600">
            {hasChanges ? 'You have unsaved changes' : 'All changes saved'}
          </p>
          <button
            onClick={handleSave}
            disabled={!hasChanges || isSaving}
            className={`flex items-center space-x-2 px-6 py-2 rounded-lg font-medium transition-colors ${
              hasChanges && !isSaving
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
  );
}
