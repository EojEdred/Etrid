/**
 * DAppCard Component
 * Displays a dApp in the directory with icon, name, and description
 */

'use client';

import React from 'react';
import { DApp } from '@/types/dapp';
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from '@/components/ui/card';
import { Badge } from '@/components/ui/badge';
import { Button } from '@/components/ui/button';
import { Star, Users, TrendingUp } from 'lucide-react';

interface DAppCardProps {
  dApp: DApp;
  onOpen: (dApp: DApp) => void;
}

export function DAppCard({ dApp, onOpen }: DAppCardProps) {
  return (
    <Card className="hover:shadow-lg transition-shadow cursor-pointer">
      <CardHeader className="flex-row gap-4 items-start space-y-0">
        {/* App Icon */}
        <div className="w-12 h-12 rounded-lg overflow-hidden flex-shrink-0 bg-muted">
          {dApp.iconUrl ? (
            <img
              src={dApp.iconUrl}
              alt={dApp.name}
              className="w-full h-full object-cover"
              onError={(e) => {
                (e.target as HTMLImageElement).src = '/placeholder-app-icon.png';
              }}
            />
          ) : (
            <div className="w-full h-full flex items-center justify-center text-lg font-bold text-muted-foreground">
              {dApp.name.charAt(0)}
            </div>
          )}
        </div>

        <div className="flex-1 min-w-0">
          <div className="flex items-center gap-2 mb-1">
            <CardTitle className="text-base truncate">{dApp.name}</CardTitle>
            {dApp.isFeatured && (
              <Badge variant="secondary" className="text-xs">
                Featured
              </Badge>
            )}
            {dApp.isTrending && (
              <TrendingUp className="w-4 h-4 text-orange-500" />
            )}
          </div>

          {/* Category Badge */}
          <Badge variant="outline" className="mb-2">
            {dApp.category}
          </Badge>

          <CardDescription className="text-sm line-clamp-2">
            {dApp.description}
          </CardDescription>
        </div>
      </CardHeader>

      <CardContent className="pt-0">
        <div className="flex items-center justify-between">
          {/* Rating and User Count */}
          <div className="flex items-center gap-4 text-sm text-muted-foreground">
            {dApp.rating && (
              <div className="flex items-center gap-1">
                <Star className="w-4 h-4 fill-yellow-400 text-yellow-400" />
                <span>{dApp.rating.toFixed(1)}</span>
              </div>
            )}
            {dApp.userCount && (
              <div className="flex items-center gap-1">
                <Users className="w-4 h-4" />
                <span>{formatUserCount(dApp.userCount)}</span>
              </div>
            )}
          </div>

          {/* Open Button */}
          <Button
            size="sm"
            onClick={(e) => {
              e.stopPropagation();
              onOpen(dApp);
            }}
          >
            Open
          </Button>
        </div>
      </CardContent>
    </Card>
  );
}

function formatUserCount(count: number): string {
  if (count >= 1000000) {
    return `${(count / 1000000).toFixed(1)}M`;
  }
  if (count >= 1000) {
    return `${(count / 1000).toFixed(1)}K`;
  }
  return count.toString();
}
