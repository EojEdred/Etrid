/**
 * Telegram Mini App - GPU Provider Dashboard
 *
 * This is a React component that runs inside Telegram as a Mini App.
 * GPU providers use this to monitor earnings, manage availability, and track reputation.
 *
 * Tech Stack:
 * - React 18 + TypeScript
 * - @telegram-apps/sdk-react (Telegram Mini Apps SDK)
 * - @polkadot/api (blockchain integration)
 * - recharts (for earnings graphs)
 */

import React, { useState, useEffect } from 'react';
import { useTelegramWebApp } from '@telegram-apps/sdk-react';
import { ApiPromise, WsProvider } from '@polkadot/api';
import { Line } from 'recharts';

interface GpuNode {
  id: number;
  model: string;
  vram_gb: number;
  stake: string;
  status: 'Active' | 'Paused' | 'Offline' | 'Slashed';
  reputation: {
    jobs_completed: number;
    jobs_failed: number;
    uptime_bps: number;
    rating: number; // 0-50000 (scaled)
    rating_count: number;
  };
  earnings_today: string;
  earnings_this_month: string;
}

export const ProviderDashboard: React.FC = () => {
  const webApp = useTelegramWebApp();
  const [api, setApi] = useState<ApiPromise | null>(null);
  const [gpuNodes, setGpuNodes] = useState<GpuNode[]>([]);
  const [loading, setLoading] = useState(true);
  const [totalEarnings, setTotalEarnings] = useState('0');

  // Connect to AI-Compute-PBC
  useEffect(() => {
    const connectBlockchain = async () => {
      const wsProvider = new WsProvider('wss://ai-compute-pbc.etrid.network');
      const apiInstance = await ApiPromise.create({ provider: wsProvider });
      setApi(apiInstance);

      // Fetch user's GPU nodes
      const userAccount = webApp.initDataUnsafe?.user?.id.toString();
      if (userAccount) {
        await fetchGpuNodes(apiInstance, userAccount);
      }

      setLoading(false);
    };

    connectBlockchain();
  }, []);

  const fetchGpuNodes = async (apiInstance: ApiPromise, account: string) => {
    // Query blockchain for user's GPU nodes
    const gpuIds = await apiInstance.query.gpuRegistry.providerGpus(account);

    const nodes: GpuNode[] = [];
    for (const gpuId of gpuIds) {
      const gpuData = await apiInstance.query.gpuRegistry.gpuNodes(gpuId);
      if (gpuData.isSome) {
        const node = gpuData.unwrap();
        nodes.push({
          id: gpuId.toNumber(),
          model: new TextDecoder().decode(node.specs.model),
          vram_gb: node.specs.vram_gb.toNumber(),
          stake: (node.stake.toBigInt() / 10n**18n).toString(), // Convert to ËDSC
          status: node.status.toString() as any,
          reputation: {
            jobs_completed: node.reputation.jobs_completed.toNumber(),
            jobs_failed: node.reputation.jobs_failed.toNumber(),
            uptime_bps: node.reputation.uptime_bps.toNumber(),
            rating: node.reputation.rating.toNumber(),
            rating_count: node.reputation.rating_count.toNumber(),
          },
          earnings_today: '47.32', // TODO: Calculate from job history
          earnings_this_month: '1203.45',
        });
      }
    }

    setGpuNodes(nodes);

    // Calculate total earnings
    const total = nodes.reduce((sum, node) =>
      sum + parseFloat(node.earnings_this_month), 0
    );
    setTotalEarnings(total.toFixed(2));
  };

  const handleWithdraw = async () => {
    if (!api) return;

    webApp.showConfirm('Withdraw all earnings to your Telegram Wallet?', (confirmed) => {
      if (confirmed) {
        // TODO: Call unregister_gpu extrinsic to withdraw stake + earnings
        webApp.showAlert('Withdrawal initiated! Funds will arrive in 2-3 minutes.');
      }
    });
  };

  const toggleGpuStatus = async (gpuId: number, currentStatus: string) => {
    if (!api) return;

    const newStatus = currentStatus === 'Active' ? 'Paused' : 'Active';

    // Call update_status extrinsic
    // TODO: Sign and submit transaction
    webApp.showAlert(`GPU ${gpuId} ${newStatus === 'Active' ? 'activated' : 'paused'}!`);
  };

  if (loading) {
    return (
      <div style={styles.container}>
        <div style={styles.loading}>Loading your GPUs...</div>
      </div>
    );
  }

  return (
    <div style={styles.container}>
      {/* Header */}
      <div style={styles.header}>
        <h1 style={styles.title}>Ëtrid AI Compute - Provider</h1>
      </div>

      {/* Earnings Summary */}
      <div style={styles.earningsCard}>
        <div style={styles.earningsStat}>
          <span style={styles.label}>Today's Earnings</span>
          <span style={styles.value}>${gpuNodes[0]?.earnings_today || '0'} ËDSC</span>
        </div>
        <div style={styles.earningsStat}>
          <span style={styles.label}>This Month</span>
          <span style={styles.value}>${totalEarnings} ËDSC</span>
        </div>
        {gpuNodes[0] && (
          <div style={styles.earningsStat}>
            <span style={styles.label}>Reputation</span>
            <span style={styles.value}>
              {(gpuNodes[0].reputation.rating / 10000).toFixed(1)}/5.0 ⭐
              <span style={styles.subtitle}>
                ({gpuNodes[0].reputation.rating_count} ratings)
              </span>
            </span>
          </div>
        )}
      </div>

      {/* GPU Nodes List */}
      <div style={styles.section}>
        <h2 style={styles.sectionTitle}>Your GPU Nodes</h2>
        {gpuNodes.map(node => (
          <div key={node.id} style={styles.gpuCard}>
            <div style={styles.gpuHeader}>
              <span style={styles.gpuModel}>{node.model}</span>
              <span style={{
                ...styles.status,
                backgroundColor: getStatusColor(node.status),
              }}>
                {node.status}
              </span>
            </div>
            <div style={styles.gpuStats}>
              <div style={styles.stat}>
                <span style={styles.statLabel}>VRAM:</span>
                <span style={styles.statValue}>{node.vram_gb} GB</span>
              </div>
              <div style={styles.stat}>
                <span style={styles.statLabel}>Stake:</span>
                <span style={styles.statValue}>{node.stake} ËDSC</span>
              </div>
              <div style={styles.stat}>
                <span style={styles.statLabel}>Jobs:</span>
                <span style={styles.statValue}>
                  {node.reputation.jobs_completed} completed
                </span>
              </div>
              <div style={styles.stat}>
                <span style={styles.statLabel}>Uptime:</span>
                <span style={styles.statValue}>
                  {(node.reputation.uptime_bps / 100).toFixed(1)}%
                </span>
              </div>
            </div>
            <div style={styles.actions}>
              <button
                style={styles.button}
                onClick={() => toggleGpuStatus(node.id, node.status)}
              >
                {node.status === 'Active' ? '⏸️ Pause' : '▶️ Activate'}
              </button>
              <button style={styles.buttonSecondary}>
                ⚙️ Settings
              </button>
            </div>
          </div>
        ))}
      </div>

      {/* Quick Actions */}
      <div style={styles.actions}>
        <button style={styles.buttonPrimary} onClick={handleWithdraw}>
          💸 Withdraw Earnings
        </button>
        <button style={styles.buttonSecondary}>
          📊 View Analytics
        </button>
        <button style={styles.buttonSecondary}>
          🏆 Leaderboard
        </button>
      </div>

      {/* Footer */}
      <div style={styles.footer}>
        <p style={styles.footerText}>
          Powered by Ëtrid Network • AI Compute PBC
        </p>
      </div>
    </div>
  );
};

// Helper function for status colors
const getStatusColor = (status: string): string => {
  switch (status) {
    case 'Active': return '#4CAF50';
    case 'Paused': return '#FF9800';
    case 'Offline': return '#F44336';
    case 'Slashed': return '#9C27B0';
    default: return '#757575';
  }
};

// Styles (inline for simplicity in Telegram Mini App)
const styles = {
  container: {
    fontFamily: '-apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, sans-serif',
    padding: '20px',
    backgroundColor: '#f5f5f5',
    minHeight: '100vh',
  },
  header: {
    marginBottom: '20px',
  },
  title: {
    fontSize: '24px',
    fontWeight: 'bold',
    color: '#333',
    margin: '0',
  },
  earningsCard: {
    backgroundColor: 'white',
    borderRadius: '12px',
    padding: '20px',
    marginBottom: '20px',
    boxShadow: '0 2px 8px rgba(0,0,0,0.1)',
  },
  earningsStat: {
    display: 'flex',
    justifyContent: 'space-between',
    alignItems: 'center',
    marginBottom: '12px',
  },
  label: {
    fontSize: '14px',
    color: '#666',
  },
  value: {
    fontSize: '18px',
    fontWeight: 'bold',
    color: '#4CAF50',
  },
  subtitle: {
    fontSize: '12px',
    color: '#999',
    marginLeft: '8px',
  },
  section: {
    marginBottom: '20px',
  },
  sectionTitle: {
    fontSize: '18px',
    fontWeight: 'bold',
    color: '#333',
    marginBottom: '12px',
  },
  gpuCard: {
    backgroundColor: 'white',
    borderRadius: '12px',
    padding: '16px',
    marginBottom: '12px',
    boxShadow: '0 2px 4px rgba(0,0,0,0.1)',
  },
  gpuHeader: {
    display: 'flex',
    justifyContent: 'space-between',
    alignItems: 'center',
    marginBottom: '12px',
  },
  gpuModel: {
    fontSize: '16px',
    fontWeight: 'bold',
    color: '#333',
  },
  status: {
    padding: '4px 12px',
    borderRadius: '12px',
    fontSize: '12px',
    color: 'white',
    fontWeight: 'bold',
  },
  gpuStats: {
    display: 'grid',
    gridTemplateColumns: '1fr 1fr',
    gap: '8px',
    marginBottom: '12px',
  },
  stat: {
    display: 'flex',
    justifyContent: 'space-between',
  },
  statLabel: {
    fontSize: '13px',
    color: '#666',
  },
  statValue: {
    fontSize: '13px',
    fontWeight: 'bold',
    color: '#333',
  },
  actions: {
    display: 'flex',
    gap: '8px',
    marginTop: '12px',
  },
  button: {
    flex: 1,
    padding: '10px',
    borderRadius: '8px',
    border: 'none',
    backgroundColor: '#2196F3',
    color: 'white',
    fontSize: '14px',
    fontWeight: 'bold',
    cursor: 'pointer',
  },
  buttonSecondary: {
    flex: 1,
    padding: '10px',
    borderRadius: '8px',
    border: '1px solid #ddd',
    backgroundColor: 'white',
    color: '#333',
    fontSize: '14px',
    fontWeight: 'bold',
    cursor: 'pointer',
  },
  buttonPrimary: {
    flex: 1,
    padding: '14px',
    borderRadius: '8px',
    border: 'none',
    backgroundColor: '#4CAF50',
    color: 'white',
    fontSize: '16px',
    fontWeight: 'bold',
    cursor: 'pointer',
  },
  loading: {
    textAlign: 'center' as const,
    padding: '40px',
    fontSize: '16px',
    color: '#666',
  },
  footer: {
    marginTop: '40px',
    paddingTop: '20px',
    borderTop: '1px solid #ddd',
    textAlign: 'center' as const,
  },
  footerText: {
    fontSize: '12px',
    color: '#999',
  },
};

export default ProviderDashboard;
