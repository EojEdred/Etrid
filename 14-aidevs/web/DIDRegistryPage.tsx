/**
 * DID Registry Page Component
 *
 * Displays all registered AI Dev DIDs with their details
 *
 * To integrate into Next.js app:
 *   1. Copy to: apps/wallet-web/etrid-crypto-website/app/dids/page.tsx
 *   2. Update API_BASE_URL to match your deployment
 */

'use client';

import { useState, useEffect } from 'react';
import Link from 'next/link';

// Types
interface DIDDocument {
  id: string;
  '@context': string[];
  controller: string;
  verificationMethod: any[];
  authentication: string[];
  assertionMethod: string[];
  service: any[];
  metadata: {
    name: string;
    description: string;
    skills: string[];
    twitter: string;
    twitter_bio: string;
    github: string;
    created: string;
    updated: string;
  };
}

interface OnChainData {
  owner: string;
  controller: string;
  documentHash: string;
  registeredAt: string;
  updatedAt: string;
  revoked: string;
}

interface DIDResolution {
  did: string;
  onChain: OnChainData;
  document: DIDDocument | null;
  resolvedAt: string;
}

interface DIDListItem {
  did: string;
  didHash: string;
  owner: string;
  controller: string;
  registeredAt: string;
  updatedAt: string;
  revoked: string;
}

// Configuration
const API_BASE_URL = process.env.NEXT_PUBLIC_DID_API_URL || 'http://localhost:3001';

export default function DIDRegistryPage() {
  const [dids, setDids] = useState<DIDListItem[]>([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);
  const [selectedDID, setSelectedDID] = useState<DIDResolution | null>(null);
  const [searchQuery, setSearchQuery] = useState('');

  // Fetch all DIDs on mount
  useEffect(() => {
    fetchAllDIDs();
  }, []);

  async function fetchAllDIDs() {
    try {
      setLoading(true);
      const response = await fetch(`${API_BASE_URL}/api/dids`);

      if (!response.ok) {
        throw new Error('Failed to fetch DIDs');
      }

      const data = await response.json();
      setDids(data.dids || []);
    } catch (err: any) {
      setError(err.message);
    } finally {
      setLoading(false);
    }
  }

  async function resolveDID(didId: string) {
    try {
      const response = await fetch(`${API_BASE_URL}/api/did/${didId}`);

      if (!response.ok) {
        throw new Error('Failed to resolve DID');
      }

      const data = await response.json();
      setSelectedDID(data);
    } catch (err: any) {
      setError(err.message);
    }
  }

  // Filter DIDs by search query
  const filteredDIDs = dids.filter(d =>
    d.did.toLowerCase().includes(searchQuery.toLowerCase())
  );

  // Separate AI Devs from Gizzi personas
  const aiDevs = filteredDIDs.filter(d => !d.did.includes('gizzi'));
  const gizziPersonas = filteredDIDs.filter(d => d.did.includes('gizzi'));

  return (
    <div className="min-h-screen bg-gradient-to-br from-gray-900 via-gray-800 to-black text-white p-8">
      {/* Header */}
      <div className="max-w-7xl mx-auto mb-12">
        <h1 className="text-5xl font-bold mb-4 bg-gradient-to-r from-blue-400 to-purple-500 bg-clip-text text-transparent">
          Ëtrid AI Devs - DID Registry
        </h1>
        <p className="text-gray-400 text-lg">
          Decentralized identifiers for all AI Devs, secured on-chain via OpenDID
        </p>
      </div>

      {/* Search Bar */}
      <div className="max-w-7xl mx-auto mb-8">
        <input
          type="text"
          placeholder="Search DIDs (e.g., consensus, governance, gizzi)"
          value={searchQuery}
          onChange={(e) => setSearchQuery(e.target.value)}
          className="w-full px-6 py-4 bg-gray-800 border border-gray-700 rounded-lg text-white placeholder-gray-500 focus:outline-none focus:ring-2 focus:ring-blue-500"
        />
      </div>

      {/* Loading State */}
      {loading && (
        <div className="max-w-7xl mx-auto text-center py-20">
          <div className="animate-spin rounded-full h-16 w-16 border-t-2 border-blue-500 mx-auto"></div>
          <p className="mt-4 text-gray-400">Loading DIDs from blockchain...</p>
        </div>
      )}

      {/* Error State */}
      {error && (
        <div className="max-w-7xl mx-auto bg-red-900/20 border border-red-500 rounded-lg p-6 mb-8">
          <h3 className="text-red-400 font-bold mb-2">Error</h3>
          <p className="text-gray-300">{error}</p>
        </div>
      )}

      {/* DID Lists */}
      {!loading && !error && (
        <div className="max-w-7xl mx-auto grid grid-cols-1 lg:grid-cols-2 gap-8">
          {/* AI Devs Section */}
          <div>
            <h2 className="text-2xl font-bold mb-6 flex items-center">
              <span className="bg-blue-500 w-3 h-3 rounded-full mr-3"></span>
              AI Devs ({aiDevs.length})
            </h2>

            <div className="space-y-4">
              {aiDevs.map((did) => (
                <DIDCard
                  key={did.didHash}
                  did={did}
                  onSelect={() => resolveDID(did.did)}
                />
              ))}
            </div>
          </div>

          {/* Gizzi Personas Section */}
          <div>
            <h2 className="text-2xl font-bold mb-6 flex items-center">
              <span className="bg-purple-500 w-3 h-3 rounded-full mr-3"></span>
              Gizzi Personas ({gizziPersonas.length})
            </h2>

            <div className="space-y-4">
              {gizziPersonas.map((did) => (
                <DIDCard
                  key={did.didHash}
                  did={did}
                  onSelect={() => resolveDID(did.did)}
                />
              ))}
            </div>
          </div>
        </div>
      )}

      {/* DID Detail Modal */}
      {selectedDID && (
        <DIDDetailModal
          resolution={selectedDID}
          onClose={() => setSelectedDID(null)}
        />
      )}

      {/* Footer Stats */}
      <div className="max-w-7xl mx-auto mt-16 grid grid-cols-1 md:grid-cols-3 gap-6">
        <StatCard
          label="Total DIDs"
          value={dids.length}
          icon="🆔"
        />
        <StatCard
          label="AI Devs"
          value={aiDevs.length}
          icon="🤖"
        />
        <StatCard
          label="Gizzi Personas"
          value={gizziPersonas.length}
          icon="🧠"
        />
      </div>
    </div>
  );
}

// DID Card Component
function DIDCard({ did, onSelect }: { did: DIDListItem; onSelect: () => void }) {
  const identity = did.did.replace('did:etrid:', '');

  return (
    <button
      onClick={onSelect}
      className="w-full bg-gray-800 hover:bg-gray-750 border border-gray-700 hover:border-blue-500 rounded-lg p-6 text-left transition-all duration-200 transform hover:scale-105"
    >
      <div className="flex items-start justify-between mb-3">
        <div className="flex-1">
          <h3 className="text-xl font-bold text-blue-400 mb-1">
            {identity}
          </h3>
          <p className="text-sm text-gray-500 font-mono break-all">
            {did.did}
          </p>
        </div>
        {did.revoked === 'true' && (
          <span className="bg-red-500 text-white text-xs px-2 py-1 rounded">
            REVOKED
          </span>
        )}
      </div>

      <div className="grid grid-cols-2 gap-3 text-sm">
        <div>
          <p className="text-gray-500">Owner</p>
          <p className="text-gray-300 font-mono truncate">{did.owner.substring(0, 12)}...</p>
        </div>
        <div>
          <p className="text-gray-500">Registered</p>
          <p className="text-gray-300">Block #{did.registeredAt}</p>
        </div>
      </div>
    </button>
  );
}

// DID Detail Modal
function DIDDetailModal({ resolution, onClose }: { resolution: DIDResolution; onClose: () => void }) {
  return (
    <div className="fixed inset-0 bg-black/80 flex items-center justify-center p-4 z-50" onClick={onClose}>
      <div className="bg-gray-900 border border-gray-700 rounded-lg p-8 max-w-4xl w-full max-h-[90vh] overflow-y-auto" onClick={(e) => e.stopPropagation()}>
        <div className="flex items-start justify-between mb-6">
          <div>
            <h2 className="text-3xl font-bold text-blue-400 mb-2">
              {resolution.document?.metadata.name || resolution.did}
            </h2>
            <p className="text-gray-500 font-mono text-sm">{resolution.did}</p>
          </div>
          <button
            onClick={onClose}
            className="text-gray-500 hover:text-white text-2xl"
          >
            ×
          </button>
        </div>

        {resolution.document && (
          <>
            <div className="mb-6">
              <p className="text-gray-300 mb-4">{resolution.document.metadata.description}</p>

              <div className="flex flex-wrap gap-2">
                {resolution.document.metadata.skills.map((skill) => (
                  <span key={skill} className="bg-blue-500/20 text-blue-400 px-3 py-1 rounded-full text-sm">
                    {skill}
                  </span>
                ))}
              </div>
            </div>

            <div className="grid grid-cols-1 md:grid-cols-2 gap-6 mb-6">
              <InfoBox label="Twitter" value={resolution.document.metadata.twitter} />
              <InfoBox label="GitHub" value={resolution.document.metadata.github} link />
              <InfoBox label="Controller" value={resolution.onChain.controller} />
              <InfoBox label="Document Hash" value={resolution.onChain.documentHash.substring(0, 16) + '...'} />
            </div>
          </>
        )}

        <div className="mt-6">
          <button
            onClick={() => {
              navigator.clipboard.writeText(JSON.stringify(resolution.document, null, 2));
            }}
            className="bg-blue-500 hover:bg-blue-600 px-6 py-2 rounded-lg font-semibold transition-colors"
          >
            Copy DID Document
          </button>
        </div>
      </div>
    </div>
  );
}

// Info Box Component
function InfoBox({ label, value, link }: { label: string; value: string; link?: boolean }) {
  return (
    <div>
      <p className="text-gray-500 text-sm mb-1">{label}</p>
      {link ? (
        <a href={value} target="_blank" rel="noopener noreferrer" className="text-blue-400 hover:underline break-all">
          {value}
        </a>
      ) : (
        <p className="text-gray-300 font-mono break-all">{value}</p>
      )}
    </div>
  );
}

// Stat Card Component
function StatCard({ label, value, icon }: { label: string; value: number; icon: string }) {
  return (
    <div className="bg-gray-800 border border-gray-700 rounded-lg p-6">
      <div className="flex items-center justify-between">
        <div>
          <p className="text-gray-500 text-sm">{label}</p>
          <p className="text-3xl font-bold text-white mt-2">{value}</p>
        </div>
        <span className="text-4xl">{icon}</span>
      </div>
    </div>
  );
}
