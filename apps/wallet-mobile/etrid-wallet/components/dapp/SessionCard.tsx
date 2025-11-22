/**
 * SessionCard Component
 * Displays a WalletConnect session with disconnect option
 */

'use client';

import React from 'react';
import { WalletConnectSession } from '@/types/dapp';
import { Card, CardContent, CardHeader } from '@/components/ui/card';
import { Button } from '@/components/ui/button';
import { Badge } from '@/components/ui/badge';
import { Separator } from '@/components/ui/separator';
import { Link2Off, ExternalLink, Clock } from 'lucide-react';
import { formatDistanceToNow } from 'date-fns';

interface SessionCardProps {
  session: WalletConnectSession;
  onDisconnect: (sessionId: string) => void;
}

export function SessionCard({ session, onDisconnect }: SessionCardProps) {
  const handleDisconnect = () => {
    if (confirm('Are you sure you want to disconnect from this dApp?')) {
      onDisconnect(session.id);
    }
  };

  const isExpiringSoon =
    session.expiresAt.getTime() - Date.now() < 24 * 60 * 60 * 1000; // 24 hours

  return (
    <Card>
      <CardHeader className="pb-3">
        <div className="flex items-start justify-between gap-3">
          {/* dApp Info */}
          <div className="flex items-center gap-3 flex-1 min-w-0">
            {session.dAppIcon && (
              <img
                src={session.dAppIcon}
                alt={session.dAppName}
                className="w-10 h-10 rounded-lg flex-shrink-0"
              />
            )}
            <div className="flex-1 min-w-0">
              <h3 className="font-semibold truncate">{session.dAppName}</h3>
              <p className="text-xs text-muted-foreground truncate">
                {session.dAppUrl}
              </p>
            </div>
          </div>

          {/* Connection Status */}
          <Badge variant="secondary" className="flex-shrink-0">
            <div className="w-2 h-2 bg-green-500 rounded-full mr-1" />
            Connected
          </Badge>
        </div>
      </CardHeader>

      <CardContent className="space-y-3">
        {/* Connection Time */}
        <div className="flex items-center gap-2 text-sm text-muted-foreground">
          <Clock className="w-4 h-4" />
          <span>
            Connected {formatDistanceToNow(session.createdAt, { addSuffix: true })}
          </span>
        </div>

        {/* Expiration Warning */}
        {isExpiringSoon && (
          <div className="flex items-center gap-2 p-2 rounded-lg bg-yellow-50 dark:bg-yellow-950 border border-yellow-200 dark:border-yellow-800">
            <Clock className="w-4 h-4 text-yellow-600 dark:text-yellow-400 flex-shrink-0" />
            <p className="text-xs text-yellow-700 dark:text-yellow-300">
              Expires {formatDistanceToNow(session.expiresAt, { addSuffix: true })}
            </p>
          </div>
        )}

        <Separator />

        {/* Permissions */}
        <div>
          <p className="text-xs font-medium text-muted-foreground mb-2">
            Granted Permissions:
          </p>
          <div className="flex flex-wrap gap-1">
            {session.permissions.map((permission, idx) => (
              <Badge key={idx} variant="outline" className="text-xs">
                {permission}
              </Badge>
            ))}
          </div>
        </div>

        {/* Chains */}
        {session.chains.length > 0 && (
          <div>
            <p className="text-xs font-medium text-muted-foreground mb-2">
              Networks:
            </p>
            <div className="flex flex-wrap gap-1">
              {session.chains.map((chain, idx) => (
                <Badge key={idx} variant="outline" className="text-xs">
                  {chain}
                </Badge>
              ))}
            </div>
          </div>
        )}

        <Separator />

        {/* Actions */}
        <div className="flex gap-2">
          <Button
            variant="outline"
            size="sm"
            className="flex-1"
            onClick={() => window.open(session.dAppUrl, '_blank')}
          >
            <ExternalLink className="w-4 h-4 mr-2" />
            Open dApp
          </Button>
          <Button
            variant="destructive"
            size="sm"
            className="flex-1"
            onClick={handleDisconnect}
          >
            <Link2Off className="w-4 h-4 mr-2" />
            Disconnect
          </Button>
        </div>
      </CardContent>
    </Card>
  );
}
