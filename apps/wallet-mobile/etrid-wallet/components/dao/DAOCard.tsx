/**
 * DAOCard Component
 * Displays a DAO summary card
 */

'use client';

import React from 'react';
import { DAO } from '@/types/dao';
import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card';
import { Badge } from '@/components/ui/badge';
import { Button } from '@/components/ui/button';
import { Users, DollarSign, FileText, ChevronRight } from 'lucide-react';

interface DAOCardProps {
  dao: DAO;
  onClick: (dao: DAO) => void;
}

export function DAOCard({ dao, onClick }: DAOCardProps) {
  return (
    <Card
      className="hover:shadow-lg transition-shadow cursor-pointer"
      onClick={() => onClick(dao)}
    >
      <CardHeader>
        <div className="flex items-start gap-3">
          {/* DAO Logo */}
          <div className="w-12 h-12 rounded-lg overflow-hidden flex-shrink-0 bg-primary/10">
            {dao.logoUrl ? (
              <img
                src={dao.logoUrl}
                alt={dao.name}
                className="w-full h-full object-cover"
              />
            ) : (
              <div className="w-full h-full flex items-center justify-center text-lg font-bold text-primary">
                {dao.name.charAt(0)}
              </div>
            )}
          </div>

          <div className="flex-1 min-w-0">
            <div className="flex items-center gap-2 mb-1">
              <CardTitle className="text-base truncate">{dao.name}</CardTitle>
              {dao.userRole && (
                <Badge
                  variant={dao.userRole === 'owner' ? 'default' : 'secondary'}
                  className="text-xs"
                >
                  {dao.userRole}
                </Badge>
              )}
            </div>
            <p className="text-sm text-muted-foreground line-clamp-2">
              {dao.description}
            </p>
          </div>
        </div>
      </CardHeader>

      <CardContent>
        <div className="grid grid-cols-3 gap-4 mb-4">
          {/* Members */}
          <div className="text-center">
            <div className="flex items-center justify-center gap-1 text-muted-foreground mb-1">
              <Users className="w-4 h-4" />
            </div>
            <p className="text-lg font-semibold">{dao.memberCount}</p>
            <p className="text-xs text-muted-foreground">Members</p>
          </div>

          {/* Treasury */}
          <div className="text-center">
            <div className="flex items-center justify-center gap-1 text-muted-foreground mb-1">
              <DollarSign className="w-4 h-4" />
            </div>
            <p className="text-lg font-semibold">{formatValue(dao.treasuryValue)}</p>
            <p className="text-xs text-muted-foreground">Treasury</p>
          </div>

          {/* Proposals */}
          <div className="text-center">
            <div className="flex items-center justify-center gap-1 text-muted-foreground mb-1">
              <FileText className="w-4 h-4" />
            </div>
            <p className="text-lg font-semibold">{dao.activeProposalCount}</p>
            <p className="text-xs text-muted-foreground">Active</p>
          </div>
        </div>

        <Button
          variant="outline"
          className="w-full"
          onClick={(e) => {
            e.stopPropagation();
            onClick(dao);
          }}
        >
          View Details
          <ChevronRight className="w-4 h-4 ml-2" />
        </Button>
      </CardContent>
    </Card>
  );
}

function formatValue(value: string): string {
  const num = parseFloat(value);
  if (isNaN(num)) return '$0';

  if (num >= 1000000) {
    return `$${(num / 1000000).toFixed(1)}M`;
  }
  if (num >= 1000) {
    return `$${(num / 1000).toFixed(1)}K`;
  }
  return `$${num.toFixed(0)}`;
}
