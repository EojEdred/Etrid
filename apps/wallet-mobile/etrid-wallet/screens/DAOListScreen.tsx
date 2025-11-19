/**
 * DAOListScreen
 * Displays user's DAOs with filter options
 */

'use client';

import React, { useState } from 'react';
import { useDAOs } from '@/hooks/useDAOs';
import { DAO } from '@/types/dao';
import { DAOCard } from '@/components/dao/DAOCard';
import { Button } from '@/components/ui/button';
import { Tabs, TabsContent, TabsList, TabsTrigger } from '@/components/ui/tabs';
import { Plus, Users, Crown, Loader2 } from 'lucide-react';

interface DAOListScreenProps {
  onViewDAO: (dao: DAO) => void;
  onCreateDAO: () => void;
}

export function DAOListScreen({ onViewDAO, onCreateDAO }: DAOListScreenProps) {
  const { daos, memberDAOs, ownedDAOs, isLoading } = useDAOs();
  const [filter, setFilter] = useState<'all' | 'member' | 'owner'>('all');

  const displayDAOs =
    filter === 'all' ? daos : filter === 'member' ? memberDAOs : ownedDAOs;

  return (
    <div className="min-h-screen bg-background">
      {/* Header */}
      <div className="border-b bg-background/95 backdrop-blur supports-[backdrop-filter]:bg-background/60">
        <div className="container py-6">
          <div className="flex items-center justify-between">
            <div>
              <h1 className="text-3xl font-bold mb-2">DAOs</h1>
              <p className="text-muted-foreground">
                Manage your Decentralized Autonomous Organizations
              </p>
            </div>
            <Button onClick={onCreateDAO} size="lg">
              <Plus className="w-5 h-5 mr-2" />
              Create DAO
            </Button>
          </div>
        </div>
      </div>

      <div className="container py-6">
        {/* Tabs */}
        <Tabs value={filter} onValueChange={(v) => setFilter(v as any)} className="mb-6">
          <TabsList>
            <TabsTrigger value="all" className="flex items-center gap-2">
              <Users className="w-4 h-4" />
              All DAOs
              <span className="ml-1 text-xs text-muted-foreground">
                ({daos.length})
              </span>
            </TabsTrigger>
            <TabsTrigger value="member" className="flex items-center gap-2">
              <Users className="w-4 h-4" />
              Member
              <span className="ml-1 text-xs text-muted-foreground">
                ({memberDAOs.length})
              </span>
            </TabsTrigger>
            <TabsTrigger value="owner" className="flex items-center gap-2">
              <Crown className="w-4 h-4" />
              Owner
              <span className="ml-1 text-xs text-muted-foreground">
                ({ownedDAOs.length})
              </span>
            </TabsTrigger>
          </TabsList>

          <TabsContent value={filter} className="mt-6">
            {isLoading ? (
              <div className="flex items-center justify-center py-12">
                <Loader2 className="w-8 h-8 animate-spin text-muted-foreground" />
              </div>
            ) : displayDAOs.length > 0 ? (
              <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
                {displayDAOs.map((dao) => (
                  <DAOCard key={dao.id} dao={dao} onClick={onViewDAO} />
                ))}
              </div>
            ) : (
              <div className="text-center py-12">
                <Users className="w-16 h-16 mx-auto mb-4 text-muted-foreground" />
                <h3 className="text-xl font-semibold mb-2">
                  {filter === 'owner'
                    ? "You don't own any DAOs yet"
                    : filter === 'member'
                    ? "You're not a member of any DAOs yet"
                    : "You don't have any DAOs yet"}
                </h3>
                <p className="text-muted-foreground mb-6">
                  {filter === 'owner'
                    ? 'Create your first DAO to get started'
                    : 'Join or create a DAO to participate in decentralized governance'}
                </p>
                <Button onClick={onCreateDAO}>
                  <Plus className="w-5 h-5 mr-2" />
                  Create Your First DAO
                </Button>
              </div>
            )}
          </TabsContent>
        </Tabs>

        {/* Stats Overview */}
        {daos.length > 0 && (
          <div className="mt-8 grid grid-cols-1 md:grid-cols-3 gap-4">
            <div className="p-6 rounded-lg bg-muted">
              <h3 className="text-sm font-medium text-muted-foreground mb-1">
                Total DAOs
              </h3>
              <p className="text-3xl font-bold">{daos.length}</p>
            </div>
            <div className="p-6 rounded-lg bg-muted">
              <h3 className="text-sm font-medium text-muted-foreground mb-1">
                DAOs Owned
              </h3>
              <p className="text-3xl font-bold">{ownedDAOs.length}</p>
            </div>
            <div className="p-6 rounded-lg bg-muted">
              <h3 className="text-sm font-medium text-muted-foreground mb-1">
                Total Members
              </h3>
              <p className="text-3xl font-bold">
                {daos.reduce((sum, dao) => sum + dao.memberCount, 0)}
              </p>
            </div>
          </div>
        )}
      </div>
    </div>
  );
}
