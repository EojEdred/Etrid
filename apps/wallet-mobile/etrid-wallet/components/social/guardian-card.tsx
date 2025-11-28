/**
 * Guardian Card Component
 * Displays guardian information for social recovery
 */

'use client';

import { formatDistanceToNow } from 'date-fns';
import { Shield, Clock, CheckCircle2, XCircle, Send, Trash2 } from 'lucide-react';
import { Button } from '@/components/ui/button';
import { Badge } from '@/components/ui/badge';
import { Avatar, AvatarFallback } from '@/components/ui/avatar';
import {
  AlertDialog,
  AlertDialogAction,
  AlertDialogCancel,
  AlertDialogContent,
  AlertDialogDescription,
  AlertDialogFooter,
  AlertDialogHeader,
  AlertDialogTitle,
  AlertDialogTrigger,
} from '@/components/ui/alert-dialog';
import { ContactsService } from '@/lib/social/ContactsService';
import type { Guardian } from '@/lib/social/types';

interface GuardianCardProps {
  guardian: Guardian;
  onRemove?: (guardian: Guardian) => void;
  onResendInvitation?: (guardian: Guardian) => void;
}

export function GuardianCard({ guardian, onRemove, onResendInvitation }: GuardianCardProps) {
  const avatarColor = ContactsService.generateAvatar(guardian.guardianAddress);
  const displayName = guardian.guardianUsername || formatAddress(guardian.guardianAddress);

  const getInitials = () => {
    if (guardian.guardianUsername) {
      return guardian.guardianUsername.slice(0, 2).toUpperCase();
    }
    return guardian.guardianAddress.slice(0, 2).toUpperCase();
  };

  const statusConfig = {
    pending: {
      label: 'Pending',
      color: 'bg-yellow-500/10 text-yellow-600 dark:text-yellow-400 border-yellow-500/20',
      icon: Clock,
    },
    active: {
      label: 'Active',
      color: 'bg-green-500/10 text-green-600 dark:text-green-400 border-green-500/20',
      icon: CheckCircle2,
    },
    declined: {
      label: 'Declined',
      color: 'bg-red-500/10 text-red-600 dark:text-red-400 border-red-500/20',
      icon: XCircle,
    },
    removed: {
      label: 'Removed',
      color: 'bg-gray-500/10 text-gray-600 dark:text-gray-400 border-gray-500/20',
      icon: XCircle,
    },
  };

  const status = statusConfig[guardian.status];
  const StatusIcon = status.icon;

  const canResendInvitation =
    guardian.status === 'pending' &&
    onResendInvitation &&
    (!guardian.lastReminderAt ||
      Date.now() - guardian.lastReminderAt.getTime() > 24 * 60 * 60 * 1000); // 24 hours

  return (
    <div className="flex items-center gap-3 p-4 rounded-lg bg-card border border-border">
      {/* Avatar */}
      <Avatar className="w-12 h-12" style={{ backgroundColor: avatarColor }}>
        <AvatarFallback className="text-white font-semibold">
          {getInitials()}
        </AvatarFallback>
      </Avatar>

      {/* Info */}
      <div className="flex-1 min-w-0 space-y-1">
        <div className="flex items-center gap-2">
          <Shield className="w-4 h-4 text-accent flex-shrink-0" />
          <h3 className="font-semibold text-foreground truncate">{displayName}</h3>
        </div>

        {/* Username or Address */}
        {guardian.guardianUsername ? (
          <p className="text-sm text-accent font-mono">@{guardian.guardianUsername}</p>
        ) : (
          <p className="text-xs text-muted-foreground font-mono truncate">
            {guardian.guardianAddress}
          </p>
        )}

        {/* Status Badge */}
        <Badge variant="outline" className={`${status.color} w-fit`}>
          <StatusIcon className="w-3 h-3 mr-1" />
          {status.label}
        </Badge>

        {/* Timestamp */}
        <p className="text-xs text-muted-foreground">
          {guardian.status === 'active' && guardian.activatedAt ? (
            <>Active since {formatDistanceToNow(guardian.activatedAt, { addSuffix: true })}</>
          ) : guardian.status === 'pending' && guardian.invitationSentAt ? (
            <>Invited {formatDistanceToNow(guardian.invitationSentAt, { addSuffix: true })}</>
          ) : (
            <>Added {formatDistanceToNow(guardian.addedAt, { addSuffix: true })}</>
          )}
        </p>
      </div>

      {/* Actions */}
      <div className="flex items-center gap-2">
        {/* Resend Invitation */}
        {canResendInvitation && (
          <Button
            size="sm"
            variant="outline"
            onClick={() => onResendInvitation(guardian)}
          >
            <Send className="w-4 h-4 mr-2" />
            Resend
          </Button>
        )}

        {/* Remove Guardian */}
        {onRemove && (
          <AlertDialog>
            <AlertDialogTrigger asChild>
              <Button size="sm" variant="ghost" className="text-destructive hover:text-destructive">
                <Trash2 className="w-4 h-4" />
              </Button>
            </AlertDialogTrigger>
            <AlertDialogContent>
              <AlertDialogHeader>
                <AlertDialogTitle>Remove Guardian</AlertDialogTitle>
                <AlertDialogDescription>
                  Are you sure you want to remove {displayName} as a guardian? This action cannot
                  be undone. Make sure you have enough active guardians to maintain account
                  recovery capabilities.
                </AlertDialogDescription>
              </AlertDialogHeader>
              <AlertDialogFooter>
                <AlertDialogCancel>Cancel</AlertDialogCancel>
                <AlertDialogAction
                  onClick={() => onRemove(guardian)}
                  className="bg-destructive text-destructive-foreground hover:bg-destructive/90"
                >
                  Remove
                </AlertDialogAction>
              </AlertDialogFooter>
            </AlertDialogContent>
          </AlertDialog>
        )}
      </div>
    </div>
  );
}

/**
 * Format address to shortened version
 */
function formatAddress(address: string): string {
  if (address.length <= 13) return address;
  return `${address.slice(0, 6)}...${address.slice(-4)}`;
}
