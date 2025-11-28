/**
 * Activity Feed Item Component
 * Displays social activity with user info and action details
 */

'use client';

import { formatDistanceToNow } from 'date-fns';
import {
  TrendingUp,
  Send,
  Inbox,
  Award,
  Lock,
  Vote,
  Users,
  Shield,
  Heart,
  MessageCircle,
} from 'lucide-react';
import { Avatar, AvatarFallback } from '@/components/ui/avatar';
import { Button } from '@/components/ui/button';
import { ContactsService } from '@/lib/social/ContactsService';
import type { SocialActivity } from '@/lib/social/types';

interface ActivityFeedItemProps {
  activity: SocialActivity;
  onLike?: (activityId: string) => void;
  onComment?: (activityId: string) => void;
}

export function ActivityFeedItem({ activity, onLike, onComment }: ActivityFeedItemProps) {
  const avatarColor = ContactsService.generateAvatar(activity.userAddress);
  const displayName = activity.username || formatAddress(activity.userAddress);

  const getInitials = () => {
    if (activity.username) {
      return activity.username.slice(0, 2).toUpperCase();
    }
    return activity.userAddress.slice(0, 2).toUpperCase();
  };

  const activityIcons = {
    transaction_sent: Send,
    transaction_received: Inbox,
    username_registered: Award,
    milestone_reached: TrendingUp,
    staking_reward: Lock,
    governance_vote: Vote,
    bill_split_created: Users,
    bill_split_paid: Users,
    guardian_added: Shield,
    recovery_initiated: Shield,
  };

  const ActivityIcon = activityIcons[activity.activityType] || Award;

  const activityColors = {
    transaction_sent: 'text-blue-500',
    transaction_received: 'text-green-500',
    username_registered: 'text-purple-500',
    milestone_reached: 'text-yellow-500',
    staking_reward: 'text-orange-500',
    governance_vote: 'text-indigo-500',
    bill_split_created: 'text-pink-500',
    bill_split_paid: 'text-teal-500',
    guardian_added: 'text-cyan-500',
    recovery_initiated: 'text-red-500',
  };

  const iconColor = activityColors[activity.activityType] || 'text-muted-foreground';

  return (
    <div className="flex gap-3 p-4 rounded-lg bg-card border border-border hover:bg-accent/5 transition-colors">
      {/* Avatar */}
      <div className="relative flex-shrink-0">
        <Avatar className="w-10 h-10" style={{ backgroundColor: avatarColor }}>
          <AvatarFallback className="text-white font-semibold text-sm">
            {getInitials()}
          </AvatarFallback>
        </Avatar>
        <div className="absolute -bottom-1 -right-1 w-6 h-6 rounded-full bg-background border-2 border-background flex items-center justify-center">
          <ActivityIcon className={`w-3 h-3 ${iconColor}`} />
        </div>
      </div>

      {/* Content */}
      <div className="flex-1 min-w-0 space-y-2">
        {/* Header */}
        <div>
          <div className="flex items-center gap-2 flex-wrap">
            <span className="font-semibold text-foreground">
              {activity.username ? `@${activity.username}` : displayName}
            </span>
            <span className="text-muted-foreground">•</span>
            <span className="text-sm text-muted-foreground">
              {formatDistanceToNow(activity.createdAt, { addSuffix: true })}
            </span>
          </div>
        </div>

        {/* Activity Details */}
        <div className="space-y-1">
          <p className="font-medium text-foreground">{activity.title}</p>
          <p className="text-sm text-muted-foreground">{activity.description}</p>

          {/* Amount (if applicable) */}
          {activity.amount !== undefined && (
            <div className="flex items-baseline gap-2 mt-2">
              <span className="text-lg font-bold text-accent">{activity.amount} ÉTR</span>
            </div>
          )}
        </div>

        {/* Interaction Buttons */}
        <div className="flex items-center gap-4 pt-2">
          {/* Like Button */}
          {onLike && (
            <Button
              size="sm"
              variant="ghost"
              className="h-8 px-2 gap-1 text-muted-foreground hover:text-pink-500"
              onClick={() => onLike(activity.id)}
            >
              <Heart className="w-4 h-4" />
              <span className="text-xs">{activity.likes > 0 ? activity.likes : ''}</span>
            </Button>
          )}

          {/* Comment Button */}
          {onComment && (
            <Button
              size="sm"
              variant="ghost"
              className="h-8 px-2 gap-1 text-muted-foreground hover:text-blue-500"
              onClick={() => onComment(activity.id)}
            >
              <MessageCircle className="w-4 h-4" />
              <span className="text-xs">{activity.comments > 0 ? activity.comments : ''}</span>
            </Button>
          )}
        </div>
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
