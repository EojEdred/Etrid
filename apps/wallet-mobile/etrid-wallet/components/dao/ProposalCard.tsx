/**
 * ProposalCard Component
 * Displays a proposal with voting information
 */

'use client';

import React from 'react';
import { Proposal, VoteType } from '@/types/dao';
import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card';
import { Badge } from '@/components/ui/badge';
import { Button } from '@/components/ui/button';
import { Progress } from '@/components/ui/progress';
import { Clock, CheckCircle2, XCircle, ThumbsUp, ThumbsDown, Minus } from 'lucide-react';
import { formatDistanceToNow } from 'date-fns';

interface ProposalCardProps {
  proposal: Proposal;
  onVote?: (proposalId: string, vote: VoteType) => void;
  onViewDetails: (proposalId: string) => void;
  userHasVoted?: boolean;
}

export function ProposalCard({
  proposal,
  onVote,
  onViewDetails,
  userHasVoted = false,
}: ProposalCardProps) {
  const totalVotes = proposal.votesFor + proposal.votesAgainst + proposal.votesAbstain;
  const forPercentage = totalVotes > 0 ? (proposal.votesFor / totalVotes) * 100 : 0;
  const againstPercentage = totalVotes > 0 ? (proposal.votesAgainst / totalVotes) * 100 : 0;

  const isActive = proposal.status === 'active';
  const canVote = isActive && !userHasVoted && onVote;

  const statusConfig = {
    pending: { color: 'bg-gray-500', label: 'Pending', icon: Clock },
    active: { color: 'bg-blue-500', label: 'Active', icon: Clock },
    passed: { color: 'bg-green-500', label: 'Passed', icon: CheckCircle2 },
    rejected: { color: 'bg-red-500', label: 'Rejected', icon: XCircle },
    executed: { color: 'bg-purple-500', label: 'Executed', icon: CheckCircle2 },
    cancelled: { color: 'bg-gray-500', label: 'Cancelled', icon: XCircle },
  };

  const status = statusConfig[proposal.status];
  const StatusIcon = status.icon;

  return (
    <Card className="hover:shadow-md transition-shadow">
      <CardHeader>
        <div className="flex items-start justify-between gap-3">
          <div className="flex-1 min-w-0">
            <div className="flex items-center gap-2 mb-2">
              <Badge variant="outline" className="text-xs">
                {proposal.type}
              </Badge>
              <Badge className={`text-xs ${status.color}`}>
                <StatusIcon className="w-3 h-3 mr-1" />
                {status.label}
              </Badge>
            </div>
            <CardTitle className="text-base line-clamp-2">
              {proposal.title}
            </CardTitle>
          </div>
        </div>

        <p className="text-sm text-muted-foreground line-clamp-2 mt-2">
          {proposal.description}
        </p>

        <div className="flex items-center gap-2 text-xs text-muted-foreground mt-2">
          <span>By {proposal.proposer.username || 'Unknown'}</span>
          <span>•</span>
          {isActive ? (
            <span className="flex items-center gap-1">
              <Clock className="w-3 h-3" />
              Ends {formatDistanceToNow(proposal.votingEndsAt, { addSuffix: true })}
            </span>
          ) : (
            <span>
              Created {formatDistanceToNow(proposal.createdAt, { addSuffix: true })}
            </span>
          )}
        </div>
      </CardHeader>

      <CardContent className="space-y-4">
        {/* Vote Breakdown */}
        <div className="space-y-2">
          {/* For/Against Progress Bar */}
          <div className="relative h-2 bg-muted rounded-full overflow-hidden">
            <div
              className="absolute left-0 top-0 h-full bg-green-500 transition-all"
              style={{ width: `${forPercentage}%` }}
            />
            <div
              className="absolute right-0 top-0 h-full bg-red-500 transition-all"
              style={{ width: `${againstPercentage}%` }}
            />
          </div>

          {/* Vote Counts */}
          <div className="flex items-center justify-between text-sm">
            <div className="flex items-center gap-1 text-green-600">
              <ThumbsUp className="w-4 h-4" />
              <span className="font-medium">{proposal.votesFor}</span>
              <span className="text-muted-foreground">({forPercentage.toFixed(1)}%)</span>
            </div>
            <div className="flex items-center gap-1 text-red-600">
              <ThumbsDown className="w-4 h-4" />
              <span className="font-medium">{proposal.votesAgainst}</span>
              <span className="text-muted-foreground">({againstPercentage.toFixed(1)}%)</span>
            </div>
          </div>

          {/* Abstain */}
          {proposal.votesAbstain > 0 && (
            <div className="flex items-center justify-center gap-1 text-xs text-muted-foreground">
              <Minus className="w-3 h-3" />
              <span>{proposal.votesAbstain} abstained</span>
            </div>
          )}

          {/* Quorum */}
          {isActive && (
            <div className="flex items-center justify-between text-xs">
              <span className="text-muted-foreground">Quorum Progress:</span>
              <span
                className={
                  proposal.quorumReached ? 'text-green-600 font-medium' : 'text-muted-foreground'
                }
              >
                {proposal.quorumReached ? 'Reached ✓' : 'Not reached'}
              </span>
            </div>
          )}
        </div>

        {/* Voting Buttons or View Details */}
        {canVote ? (
          <div className="flex gap-2">
            <Button
              variant="outline"
              size="sm"
              className="flex-1 text-green-600 border-green-600 hover:bg-green-50"
              onClick={() => onVote(proposal.id, 'for')}
            >
              <ThumbsUp className="w-4 h-4 mr-2" />
              For
            </Button>
            <Button
              variant="outline"
              size="sm"
              className="flex-1 text-red-600 border-red-600 hover:bg-red-50"
              onClick={() => onVote(proposal.id, 'against')}
            >
              <ThumbsDown className="w-4 h-4 mr-2" />
              Against
            </Button>
            <Button
              variant="outline"
              size="sm"
              className="flex-1"
              onClick={() => onVote(proposal.id, 'abstain')}
            >
              <Minus className="w-4 h-4 mr-2" />
              Abstain
            </Button>
          </div>
        ) : (
          <Button
            variant="outline"
            size="sm"
            className="w-full"
            onClick={() => onViewDetails(proposal.id)}
          >
            View Details
          </Button>
        )}

        {userHasVoted && isActive && (
          <p className="text-xs text-center text-muted-foreground">
            ✓ You have already voted on this proposal
          </p>
        )}
      </CardContent>
    </Card>
  );
}
