/**
 * MemberList Component
 * Displays list of DAO members with voting power
 */

'use client';

import React from 'react';
import { DAOMember } from '@/types/dao';
import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card';
import { Badge } from '@/components/ui/badge';
import { Button } from '@/components/ui/button';
import { Avatar, AvatarFallback, AvatarImage } from '@/components/ui/avatar';
import { UserMinus } from 'lucide-react';
import { formatDistanceToNow } from 'date-fns';

interface MemberListProps {
  members: DAOMember[];
  totalVotingPower: number;
  canRemove?: boolean;
  onRemoveMember?: (memberId: string) => void;
}

export function MemberList({
  members,
  totalVotingPower,
  canRemove = false,
  onRemoveMember,
}: MemberListProps) {
  const sortedMembers = [...members].sort((a, b) => b.votingPower - a.votingPower);

  return (
    <Card>
      <CardHeader>
        <CardTitle>Members ({members.length})</CardTitle>
      </CardHeader>

      <CardContent>
        <div className="space-y-3">
          {sortedMembers.map((member) => (
            <div
              key={member.id}
              className="flex items-center gap-3 p-3 rounded-lg bg-muted/50 hover:bg-muted transition-colors"
            >
              {/* Avatar */}
              <Avatar>
                <AvatarImage src={member.avatarUrl} alt={member.username} />
                <AvatarFallback>
                  {member.username?.charAt(0) || member.address.slice(0, 2)}
                </AvatarFallback>
              </Avatar>

              {/* Member Info */}
              <div className="flex-1 min-w-0">
                <div className="flex items-center gap-2 mb-1">
                  <p className="font-medium truncate">
                    {member.username || `${member.address.slice(0, 6)}...${member.address.slice(-4)}`}
                  </p>
                  <Badge
                    variant={member.role === 'owner' ? 'default' : 'secondary'}
                    className="text-xs"
                  >
                    {member.role}
                  </Badge>
                </div>

                <div className="flex items-center gap-3 text-xs text-muted-foreground">
                  <span className="truncate">
                    {member.address.slice(0, 10)}...{member.address.slice(-8)}
                  </span>
                  <span>•</span>
                  <span>
                    Joined {formatDistanceToNow(member.joinedAt, { addSuffix: true })}
                  </span>
                </div>

                <div className="flex items-center gap-3 text-xs text-muted-foreground mt-1">
                  <span>{member.proposalsCreated} proposals</span>
                  <span>•</span>
                  <span>{member.votesCast} votes</span>
                </div>
              </div>

              {/* Voting Power */}
              <div className="text-right flex-shrink-0">
                <p className="font-semibold text-lg">
                  {member.votingPower.toLocaleString()}
                </p>
                <p className="text-xs text-muted-foreground">
                  {totalVotingPower > 0
                    ? ((member.votingPower / totalVotingPower) * 100).toFixed(1)
                    : 0}
                  % power
                </p>

                {canRemove && member.role !== 'owner' && onRemoveMember && (
                  <Button
                    variant="ghost"
                    size="sm"
                    className="mt-2 text-destructive"
                    onClick={() => {
                      if (
                        confirm(
                          `Are you sure you want to remove ${
                            member.username || member.address
                          } from the DAO?`
                        )
                      ) {
                        onRemoveMember(member.id);
                      }
                    }}
                  >
                    <UserMinus className="w-4 h-4" />
                  </Button>
                )}
              </div>
            </div>
          ))}

          {members.length === 0 && (
            <div className="text-center py-12 text-muted-foreground">
              <p>No members found</p>
            </div>
          )}
        </div>
      </CardContent>
    </Card>
  );
}
