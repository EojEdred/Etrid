/**
 * AI Devs Dashboard Component
 *
 * Real-time status dashboard for all AI Devs
 *
 * To integrate into Next.js app:
 *   1. Copy to: apps/wallet-web/etrid-crypto-website/app/ai-devs/page.tsx
 *   2. Update API_BASE_URL to match your deployment
 */

'use client';

import { useState, useEffect } from 'react';
import Link from 'next/link';

// Types
interface AIDevStatus {
  dev_id: string;
  name: string;
  status: 'ACTIVE' | 'IDLE' | 'OFFLINE';
  last_activity: string;
  current_task: string | null;
  skills: string[];
  stats: {
    executions_today: number;
    avg_execution_time: number;
    success_rate: number;
  };
}

interface MemoryEntry {
  timestamp: string;
  dev_id: string;
  event: string;
  action: string;
  status: 'PENDING' | 'IN_PROGRESS' | 'RESOLVED' | 'ESCALATED';
  priority: 'LOW' | 'MEDIUM' | 'HIGH' | 'CRITICAL';
  tags: string[];
}

// Mock data (replace with actual API calls)
const MOCK_AI_DEVS: AIDevStatus[] = [
  {
    dev_id: 'consensus-dev01',
    name: 'Consensus Dev',
    status: 'ACTIVE',
    last_activity: '2025-10-24T15:30:00Z',
    current_task: 'PPFA rotation optimization',
    skills: ['validator-rotation', 'ppfa-sealing', 'adaptive-timing'],
    stats: {
      executions_today: 42,
      avg_execution_time: 1.94,
      success_rate: 98.5
    }
  },
  // Add more mock devs...
];

const MOCK_MEMORY: MemoryEntry[] = [
  {
    timestamp: '2025-10-24T15:00:00Z',
    dev_id: 'gizzi',
    event: 'AI Devs DID Integration initialized',
    action: 'Generated 15 DID documents',
    status: 'RESOLVED',
    priority: 'HIGH',
    tags: ['did', 'initialization']
  },
  // Add more mock entries...
];

export default function AIDevsDashboard() {
  const [devs, setDevs] = useState<AIDevStatus[]>(MOCK_AI_DEVS);
  const [memory, setMemory] = useState<MemoryEntry[]>(MOCK_MEMORY);
  const [selectedDev, setSelectedDev] = useState<AIDevStatus | null>(null);
  const [filterStatus, setFilterStatus] = useState<string>('all');

  // Filter devs by status
  const filteredDevs = filterStatus === 'all'
    ? devs
    : devs.filter(d => d.status.toLowerCase() === filterStatus);

  return (
    <div className="min-h-screen bg-gradient-to-br from-gray-900 via-gray-800 to-black text-white p-8">
      {/* Header */}
      <div className="max-w-7xl mx-auto mb-12">
        <div className="flex items-center justify-between">
          <div>
            <h1 className="text-5xl font-bold mb-4 bg-gradient-to-r from-green-400 to-blue-500 bg-clip-text text-transparent">
              AI Devs Dashboard
            </h1>
            <p className="text-gray-400 text-lg">
              Real-time status and activity monitoring for all 12 AI Devs + 3 Gizzi personas
            </p>
          </div>

          <Link
            href="/dids"
            className="bg-blue-500 hover:bg-blue-600 px-6 py-3 rounded-lg font-semibold transition-colors"
          >
            View DID Registry →
          </Link>
        </div>
      </div>

      {/* Stats Overview */}
      <div className="max-w-7xl mx-auto grid grid-cols-1 md:grid-cols-4 gap-6 mb-12">
        <StatCard
          label="Active Devs"
          value={devs.filter(d => d.status === 'ACTIVE').length}
          total={devs.length}
          color="green"
          icon="✅"
        />
        <StatCard
          label="Total Executions"
          value={devs.reduce((sum, d) => sum + d.stats.executions_today, 0)}
          color="blue"
          icon="⚡"
        />
        <StatCard
          label="Avg Success Rate"
          value={Math.round(devs.reduce((sum, d) => sum + d.stats.success_rate, 0) / devs.length)}
          suffix="%"
          color="purple"
          icon="📊"
        />
        <StatCard
          label="Avg Response Time"
          value={parseFloat((devs.reduce((sum, d) => sum + d.stats.avg_execution_time, 0) / devs.length).toFixed(2))}
          suffix="ms"
          color="yellow"
          icon="⏱️"
        />
      </div>

      {/* Status Filter */}
      <div className="max-w-7xl mx-auto mb-8 flex gap-4">
        {['all', 'active', 'idle', 'offline'].map((status) => (
          <button
            key={status}
            onClick={() => setFilterStatus(status)}
            className={`px-6 py-2 rounded-lg font-semibold transition-all ${
              filterStatus === status
                ? 'bg-blue-500 text-white'
                : 'bg-gray-800 text-gray-400 hover:bg-gray-700'
            }`}
          >
            {status.charAt(0).toUpperCase() + status.slice(1)}
          </button>
        ))}
      </div>

      {/* AI Devs Grid */}
      <div className="max-w-7xl mx-auto mb-12">
        <h2 className="text-2xl font-bold mb-6">AI Developers</h2>
        <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
          {filteredDevs.map((dev) => (
            <AIDevCard
              key={dev.dev_id}
              dev={dev}
              onClick={() => setSelectedDev(dev)}
            />
          ))}
        </div>
      </div>

      {/* Global Memory Stream */}
      <div className="max-w-7xl mx-auto">
        <h2 className="text-2xl font-bold mb-6">Global Memory Stream</h2>
        <div className="bg-gray-800 border border-gray-700 rounded-lg p-6">
          <div className="space-y-4">
            {memory.slice(0, 10).map((entry, idx) => (
              <MemoryEntryCard key={idx} entry={entry} />
            ))}
          </div>

          <div className="mt-6 text-center">
            <Link
              href="/ai-devs/memory"
              className="text-blue-400 hover:underline"
            >
              View Full Memory Log →
            </Link>
          </div>
        </div>
      </div>

      {/* Dev Detail Modal */}
      {selectedDev && (
        <DevDetailModal
          dev={selectedDev}
          onClose={() => setSelectedDev(null)}
        />
      )}
    </div>
  );
}

// AI Dev Card Component
function AIDevCard({ dev, onClick }: { dev: AIDevStatus; onClick: () => void }) {
  const statusColors = {
    ACTIVE: 'bg-green-500',
    IDLE: 'bg-yellow-500',
    OFFLINE: 'bg-red-500'
  };

  return (
    <button
      onClick={onClick}
      className="bg-gray-800 hover:bg-gray-750 border border-gray-700 hover:border-blue-500 rounded-lg p-6 text-left transition-all duration-200 transform hover:scale-105"
    >
      {/* Header */}
      <div className="flex items-start justify-between mb-4">
        <div>
          <h3 className="text-xl font-bold text-white mb-1">{dev.name}</h3>
          <p className="text-sm text-gray-500 font-mono">{dev.dev_id}</p>
        </div>
        <span className={`${statusColors[dev.status]} w-3 h-3 rounded-full`}></span>
      </div>

      {/* Current Task */}
      {dev.current_task && (
        <div className="mb-4 bg-blue-500/10 border border-blue-500/30 rounded px-3 py-2">
          <p className="text-xs text-blue-400 mb-1">Current Task</p>
          <p className="text-sm text-white">{dev.current_task}</p>
        </div>
      )}

      {/* Stats */}
      <div className="grid grid-cols-3 gap-3 mb-4">
        <div>
          <p className="text-xs text-gray-500">Executions</p>
          <p className="text-lg font-bold text-white">{dev.stats.executions_today}</p>
        </div>
        <div>
          <p className="text-xs text-gray-500">Success</p>
          <p className="text-lg font-bold text-green-400">{dev.stats.success_rate}%</p>
        </div>
        <div>
          <p className="text-xs text-gray-500">Avg Time</p>
          <p className="text-lg font-bold text-blue-400">{dev.stats.avg_execution_time}ms</p>
        </div>
      </div>

      {/* Skills */}
      <div className="flex flex-wrap gap-1">
        {dev.skills.slice(0, 3).map((skill) => (
          <span key={skill} className="text-xs bg-gray-700 text-gray-300 px-2 py-1 rounded">
            {skill}
          </span>
        ))}
        {dev.skills.length > 3 && (
          <span className="text-xs bg-gray-700 text-gray-400 px-2 py-1 rounded">
            +{dev.skills.length - 3}
          </span>
        )}
      </div>
    </button>
  );
}

// Dev Detail Modal
function DevDetailModal({ dev, onClose }: { dev: AIDevStatus; onClose: () => void }) {
  return (
    <div className="fixed inset-0 bg-black/80 flex items-center justify-center p-4 z-50" onClick={onClose}>
      <div className="bg-gray-900 border border-gray-700 rounded-lg p-8 max-w-4xl w-full max-h-[90vh] overflow-y-auto" onClick={(e) => e.stopPropagation()}>
        <div className="flex items-start justify-between mb-6">
          <div>
            <h2 className="text-3xl font-bold text-white mb-2">{dev.name}</h2>
            <p className="text-gray-500 font-mono text-sm">DID: did:etrid:{dev.dev_id}</p>
          </div>
          <button
            onClick={onClose}
            className="text-gray-500 hover:text-white text-2xl"
          >
            ×
          </button>
        </div>

        <div className="grid grid-cols-1 md:grid-cols-2 gap-6 mb-6">
          <InfoBox label="Status" value={dev.status} />
          <InfoBox label="Last Activity" value={new Date(dev.last_activity).toLocaleString()} />
          <InfoBox label="Current Task" value={dev.current_task || 'Idle'} />
          <InfoBox label="Executions Today" value={dev.stats.executions_today.toString()} />
        </div>

        <div className="mb-6">
          <h3 className="text-xl font-bold mb-3">Skills</h3>
          <div className="flex flex-wrap gap-2">
            {dev.skills.map((skill) => (
              <span key={skill} className="bg-blue-500/20 text-blue-400 px-3 py-2 rounded-lg">
                {skill}
              </span>
            ))}
          </div>
        </div>

        <div className="flex gap-4">
          <Link
            href={`/dids/${dev.dev_id}`}
            className="bg-blue-500 hover:bg-blue-600 px-6 py-2 rounded-lg font-semibold transition-colors"
          >
            View DID Document
          </Link>
          <Link
            href={`/ai-devs/${dev.dev_id}/memory`}
            className="bg-gray-700 hover:bg-gray-600 px-6 py-2 rounded-lg font-semibold transition-colors"
          >
            View Memory Log
          </Link>
        </div>
      </div>
    </div>
  );
}

// Memory Entry Card
function MemoryEntryCard({ entry }: { entry: MemoryEntry }) {
  const statusColors = {
    PENDING: 'text-yellow-400',
    IN_PROGRESS: 'text-blue-400',
    RESOLVED: 'text-green-400',
    ESCALATED: 'text-red-400'
  };

  const priorityColors = {
    LOW: 'bg-gray-600',
    MEDIUM: 'bg-blue-600',
    HIGH: 'bg-orange-600',
    CRITICAL: 'bg-red-600'
  };

  return (
    <div className="border-l-4 border-blue-500 pl-4 py-2">
      <div className="flex items-start justify-between mb-2">
        <div className="flex-1">
          <p className="text-sm text-gray-500">{new Date(entry.timestamp).toLocaleString()} · {entry.dev_id}</p>
          <h4 className="text-white font-semibold">{entry.event}</h4>
          <p className="text-gray-400 text-sm">{entry.action}</p>
        </div>
        <div className="flex items-center gap-2">
          <span className={`${priorityColors[entry.priority]} text-white text-xs px-2 py-1 rounded`}>
            {entry.priority}
          </span>
          <span className={`${statusColors[entry.status]} font-semibold text-xs`}>
            {entry.status}
          </span>
        </div>
      </div>
      <div className="flex flex-wrap gap-1">
        {entry.tags.map((tag) => (
          <span key={tag} className="text-xs text-gray-500">#{tag}</span>
        ))}
      </div>
    </div>
  );
}

// Stat Card Component
function StatCard({ label, value, total, suffix, color, icon }: {
  label: string;
  value: number;
  total?: number;
  suffix?: string;
  color: string;
  icon: string;
}) {
  const colors = {
    green: 'from-green-500 to-green-600',
    blue: 'from-blue-500 to-blue-600',
    purple: 'from-purple-500 to-purple-600',
    yellow: 'from-yellow-500 to-yellow-600'
  };

  return (
    <div className="bg-gray-800 border border-gray-700 rounded-lg p-6">
      <div className="flex items-center justify-between mb-3">
        <p className="text-gray-500 text-sm">{label}</p>
        <span className="text-3xl">{icon}</span>
      </div>
      <div>
        <p className="text-3xl font-bold text-white">
          {value}{suffix}
          {total && <span className="text-lg text-gray-500">/{total}</span>}
        </p>
      </div>
    </div>
  );
}

// Info Box Component
function InfoBox({ label, value }: { label: string; value: string }) {
  return (
    <div>
      <p className="text-gray-500 text-sm mb-1">{label}</p>
      <p className="text-white font-mono">{value}</p>
    </div>
  );
}
