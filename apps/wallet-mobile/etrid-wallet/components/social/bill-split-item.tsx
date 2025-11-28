/**
 * Bill Split Item Component
 * Displays bill split information with participants and payment status
 */

'use client';

import { formatDistanceToNow } from 'date-fns';
import { Users, Clock, CheckCircle2, AlertCircle, Send } from 'lucide-react';
import { Button } from '@/components/ui/button';
import { Badge } from '@/components/ui/badge';
import { Avatar, AvatarFallback } from '@/components/ui/avatar';
import { Progress } from '@/components/ui/progress';
import type { BillSplit } from '@/lib/social/types';
import { ContactsService } from '@/lib/social/ContactsService';

interface BillSplitItemProps {
  split: BillSplit;
  currentUserId: string;
  onPay?: (split: BillSplit) => void;
  onRemind?: (split: BillSplit) => void;
  onClick?: (split: BillSplit) => void;
}

export function BillSplitItem({
  split,
  currentUserId,
  onPay,
  onRemind,
  onClick,
}: BillSplitItemProps) {
  const isCreator = split.creatorId === currentUserId;
  const userParticipant = split.participants.find((p) => p.userId === currentUserId);
  const userOwes = userParticipant
    ? userParticipant.amountOwed - userParticipant.amountPaid
    : 0;
  const userHasPaid = userParticipant?.status === 'paid';

  // Calculate totals
  const totalPaid = split.participants.reduce((sum, p) => sum + p.amountPaid, 0);
  const totalOwed = split.participants.reduce((sum, p) => sum + p.amountOwed, 0);
  const progress = totalOwed > 0 ? (totalPaid / totalOwed) * 100 : 0;

  const statusConfig = {
    pending: { label: 'Pending', color: 'text-yellow-600 dark:text-yellow-400', icon: Clock },
    partial: { label: 'In Progress', color: 'text-blue-600 dark:text-blue-400', icon: AlertCircle },
    completed: { label: 'Completed', color: 'text-green-600 dark:text-green-400', icon: CheckCircle2 },
    cancelled: { label: 'Cancelled', color: 'text-gray-600 dark:text-gray-400', icon: AlertCircle },
  };

  const status = statusConfig[split.status];
  const StatusIcon = status.icon;

  return (
    <div
      className="p-4 rounded-lg bg-card border border-border hover:bg-accent/5 transition-colors cursor-pointer"
      onClick={() => onClick?.(split)}
    >
      {/* Header */}
      <div className="flex items-start justify-between mb-3">
        <div className="flex-1 min-w-0">
          <h3 className="font-semibold text-foreground truncate">{split.name}</h3>
          {split.description && (
            <p className="text-sm text-muted-foreground truncate">{split.description}</p>
          )}
        </div>
        <Badge variant="outline" className="ml-2 flex-shrink-0">
          <StatusIcon className={`w-3 h-3 mr-1 ${status.color}`} />
          <span className={status.color}>{status.label}</span>
        </Badge>
      </div>

      {/* Total Amount */}
      <div className="flex items-baseline gap-2 mb-3">
        <span className="text-2xl font-bold text-foreground">{split.totalAmount} ÉTR</span>
        <span className="text-sm text-muted-foreground">
          • {split.splitType === 'equal' ? 'Split equally' : 'Custom split'}
        </span>
      </div>

      {/* Progress Bar */}
      {split.status !== 'completed' && split.status !== 'cancelled' && (
        <div className="mb-3 space-y-1">
          <div className="flex justify-between text-xs text-muted-foreground">
            <span>{totalPaid.toFixed(2)} ÉTR paid</span>
            <span>{progress.toFixed(0)}%</span>
          </div>
          <Progress value={progress} className="h-2" />
        </div>
      )}

      {/* Participants */}
      <div className="flex items-center gap-2 mb-3">
        <Users className="w-4 h-4 text-muted-foreground" />
        <div className="flex -space-x-2">
          {split.participants.slice(0, 5).map((participant, index) => {
            const avatarColor = ContactsService.generateAvatar(participant.address);
            const initials = participant.username
              ? participant.username.slice(0, 2).toUpperCase()
              : participant.address.slice(0, 2).toUpperCase();

            return (
              <Avatar
                key={participant.id}
                className="w-8 h-8 border-2 border-background"
                style={{ backgroundColor: avatarColor }}
              >
                <AvatarFallback className="text-white text-xs font-semibold">
                  {initials}
                </AvatarFallback>
              </Avatar>
            );
          })}
          {split.participants.length > 5 && (
            <div className="w-8 h-8 rounded-full border-2 border-background bg-muted flex items-center justify-center">
              <span className="text-xs font-medium text-muted-foreground">
                +{split.participants.length - 5}
              </span>
            </div>
          )}
        </div>
        <span className="text-sm text-muted-foreground ml-1">
          {split.participants.length} {split.participants.length === 1 ? 'person' : 'people'}
        </span>
      </div>

      {/* User's Share */}
      {userParticipant && (
        <div className="p-3 rounded-lg bg-accent/10 border border-border mb-3">
          <div className="flex items-center justify-between">
            <div>
              <p className="text-sm font-medium text-foreground">Your share</p>
              {userHasPaid ? (
                <p className="text-xs text-green-600 dark:text-green-400">
                  ✓ Paid {formatDistanceToNow(userParticipant.paidAt!, { addSuffix: true })}
                </p>
              ) : (
                <p className="text-xs text-muted-foreground">Payment pending</p>
              )}
            </div>
            <div className="text-right">
              <p className="text-lg font-bold text-accent">{userOwes.toFixed(2)} ÉTR</p>
              {userHasPaid && (
                <CheckCircle2 className="w-5 h-5 text-green-500 ml-auto" />
              )}
            </div>
          </div>
        </div>
      )}

      {/* Actions */}
      <div className="flex gap-2">
        {!userHasPaid && userOwes > 0 && onPay && (
          <Button
            size="sm"
            className="flex-1"
            onClick={(e) => {
              e.stopPropagation();
              onPay(split);
            }}
          >
            Pay {userOwes.toFixed(2)} ÉTR
          </Button>
        )}

        {isCreator && split.status !== 'completed' && split.status !== 'cancelled' && onRemind && (
          <Button
            size="sm"
            variant="outline"
            className="flex-1"
            onClick={(e) => {
              e.stopPropagation();
              onRemind(split);
            }}
          >
            <Send className="w-4 h-4 mr-2" />
            Remind
          </Button>
        )}
      </div>

      {/* Timestamp */}
      <p className="text-xs text-muted-foreground mt-3">
        Created {formatDistanceToNow(split.createdAt, { addSuffix: true })}
      </p>
    </div>
  );
}
