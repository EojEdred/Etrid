/**
 * ProposalDetailScreen
 * Detailed view of a single proposal with voting and discussion
 */

'use client';

import React, { useEffect, useState } from 'react';
import { useDAOProposals } from '@/hooks/useDAOProposals';
import { Proposal, VoteType, VoteBreakdown as VoteBreakdownType } from '@/types/dao';
import { VoteBreakdown } from '@/components/dao/VoteBreakdown';
import { Button } from '@/components/ui/button';
import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card';
import { Badge } from '@/components/ui/badge';
import { Separator } from '@/components/ui/separator';
import { Avatar, AvatarFallback, AvatarImage } from '@/components/ui/avatar';
import { Textarea } from '@/components/ui/textarea';
import {
  Dialog,
  DialogContent,
  DialogDescription,
  DialogFooter,
  DialogHeader,
  DialogTitle,
} from '@/components/ui/dialog';
import {
  Clock,
  CheckCircle2,
  XCircle,
  ThumbsUp,
  ThumbsDown,
  Minus,
  ArrowLeft,
  Play,
  MessageSquare,
} from 'lucide-react';
import { formatDistanceToNow, format } from 'date-fns';

interface ProposalDetailScreenProps {
  proposalId: string;
  daoId: string;
  onBack: () => void;
}

export function ProposalDetailScreen({
  proposalId,
  daoId,
  onBack,
}: ProposalDetailScreenProps) {
  const { getProposal, vote, executeProposal, getVoteBreakdown } = useDAOProposals(daoId);
  const [proposal, setProposal] = useState<Proposal | null>(null);
  const [voteBreakdown, setVoteBreakdown] = useState<VoteBreakdownType | null>(null);
  const [showVoteDialog, setShowVoteDialog] = useState(false);
  const [selectedVote, setSelectedVote] = useState<VoteType>('for');
  const [voteReason, setVoteReason] = useState('');
  const [isLoading, setIsLoading] = useState(false);

  useEffect(() => {
    loadProposal();
  }, [proposalId]);

  const loadProposal = async () => {
    const p = await getProposal(proposalId);
    if (p) {
      setProposal(p);
      const breakdown = await getVoteBreakdown(proposalId);
      if (breakdown) {
        setVoteBreakdown(breakdown);
      }
    }
  };

  const handleVote = async () => {
    if (!proposal) return;
    setIsLoading(true);
    try {
      await vote(proposal.id, selectedVote, voteReason);
      setShowVoteDialog(false);
      setVoteReason('');
      await loadProposal();
    } catch (error) {
      console.error('Failed to vote:', error);
    } finally {
      setIsLoading(false);
    }
  };

  const handleExecute = async () => {
    if (!proposal) return;
    if (confirm('Are you sure you want to execute this proposal?')) {
      setIsLoading(true);
      try {
        await executeProposal(proposal.id);
        await loadProposal();
      } catch (error) {
        console.error('Failed to execute proposal:', error);
      } finally {
        setIsLoading(false);
      }
    }
  };

  if (!proposal) {
    return (
      <div className="min-h-screen bg-background flex items-center justify-center">
        <p className="text-muted-foreground">Loading proposal...</p>
      </div>
    );
  }

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
  const isActive = proposal.status === 'active';
  const canExecute = proposal.status === 'passed';

  return (
    <div className="min-h-screen bg-background">
      {/* Header */}
      <div className="border-b bg-background/95 backdrop-blur supports-[backdrop-filter]:bg-background/60">
        <div className="container py-4">
          <Button variant="ghost" onClick={onBack} className="mb-4">
            <ArrowLeft className="w-4 h-4 mr-2" />
            Back to Proposals
          </Button>

          <div className="flex items-start justify-between gap-4">
            <div className="flex-1">
              <div className="flex items-center gap-2 mb-2">
                <Badge variant="outline">{proposal.type}</Badge>
                <Badge className={`${status.color}`}>
                  <StatusIcon className="w-3 h-3 mr-1" />
                  {status.label}
                </Badge>
              </div>
              <h1 className="text-3xl font-bold mb-2">{proposal.title}</h1>
            </div>

            {isActive && (
              <Button
                size="lg"
                onClick={() => setShowVoteDialog(true)}
                disabled={isLoading}
              >
                Cast Vote
              </Button>
            )}

            {canExecute && (
              <Button
                size="lg"
                onClick={handleExecute}
                disabled={isLoading}
                variant="default"
              >
                <Play className="w-4 h-4 mr-2" />
                Execute
              </Button>
            )}
          </div>
        </div>
      </div>

      <div className="container py-6">
        <div className="grid grid-cols-1 lg:grid-cols-3 gap-6">
          {/* Main Content */}
          <div className="lg:col-span-2 space-y-6">
            {/* Proposal Details */}
            <Card>
              <CardHeader>
                <CardTitle>Proposal Details</CardTitle>
              </CardHeader>
              <CardContent className="space-y-4">
                <div>
                  <h3 className="font-semibold mb-2">Description</h3>
                  <p className="text-muted-foreground whitespace-pre-wrap">
                    {proposal.description}
                  </p>
                </div>

                <Separator />

                <div className="flex items-center gap-3">
                  <Avatar>
                    <AvatarImage src={proposal.proposer.avatarUrl} />
                    <AvatarFallback>
                      {proposal.proposer.username?.charAt(0) || 'U'}
                    </AvatarFallback>
                  </Avatar>
                  <div>
                    <p className="font-medium">
                      {proposal.proposer.username || 'Unknown'}
                    </p>
                    <p className="text-sm text-muted-foreground">
                      Proposed{' '}
                      {formatDistanceToNow(proposal.createdAt, { addSuffix: true })}
                    </p>
                  </div>
                </div>

                <Separator />

                <div className="grid grid-cols-2 gap-4 text-sm">
                  <div>
                    <p className="text-muted-foreground mb-1">Voting Starts</p>
                    <p className="font-medium">
                      {format(proposal.votingStartsAt, 'MMM d, yyyy HH:mm')}
                    </p>
                  </div>
                  <div>
                    <p className="text-muted-foreground mb-1">Voting Ends</p>
                    <p className="font-medium">
                      {format(proposal.votingEndsAt, 'MMM d, yyyy HH:mm')}
                    </p>
                  </div>
                </div>

                {isActive && (
                  <div className="p-3 rounded-lg bg-blue-50 dark:bg-blue-950 border border-blue-200 dark:border-blue-800">
                    <p className="text-sm text-blue-700 dark:text-blue-300">
                      <Clock className="w-4 h-4 inline mr-1" />
                      Voting ends{' '}
                      {formatDistanceToNow(proposal.votingEndsAt, {
                        addSuffix: true,
                      })}
                    </p>
                  </div>
                )}

                {proposal.executedAt && (
                  <div className="p-3 rounded-lg bg-purple-50 dark:bg-purple-950 border border-purple-200 dark:border-purple-800">
                    <p className="text-sm text-purple-700 dark:text-purple-300">
                      <CheckCircle2 className="w-4 h-4 inline mr-1" />
                      Executed on {format(proposal.executedAt, 'MMM d, yyyy HH:mm')}
                    </p>
                  </div>
                )}
              </CardContent>
            </Card>

            {/* Discussion (Placeholder) */}
            <Card>
              <CardHeader>
                <CardTitle className="flex items-center gap-2">
                  <MessageSquare className="w-5 h-5" />
                  Discussion
                </CardTitle>
              </CardHeader>
              <CardContent>
                <div className="text-center py-12 text-muted-foreground">
                  <MessageSquare className="w-12 h-12 mx-auto mb-3 opacity-50" />
                  <p>Discussion feature coming soon</p>
                </div>
              </CardContent>
            </Card>
          </div>

          {/* Sidebar */}
          <div className="space-y-6">
            {/* Vote Breakdown */}
            {voteBreakdown && <VoteBreakdown breakdown={voteBreakdown} />}
          </div>
        </div>
      </div>

      {/* Vote Dialog */}
      <Dialog open={showVoteDialog} onOpenChange={setShowVoteDialog}>
        <DialogContent>
          <DialogHeader>
            <DialogTitle>Cast Your Vote</DialogTitle>
            <DialogDescription>
              Choose your vote and optionally provide a reason
            </DialogDescription>
          </DialogHeader>

          <div className="space-y-4">
            {/* Vote Options */}
            <div className="grid grid-cols-3 gap-2">
              <Button
                variant={selectedVote === 'for' ? 'default' : 'outline'}
                className="flex-col h-auto py-4"
                onClick={() => setSelectedVote('for')}
              >
                <ThumbsUp className="w-6 h-6 mb-2" />
                <span>For</span>
              </Button>
              <Button
                variant={selectedVote === 'against' ? 'default' : 'outline'}
                className="flex-col h-auto py-4"
                onClick={() => setSelectedVote('against')}
              >
                <ThumbsDown className="w-6 h-6 mb-2" />
                <span>Against</span>
              </Button>
              <Button
                variant={selectedVote === 'abstain' ? 'default' : 'outline'}
                className="flex-col h-auto py-4"
                onClick={() => setSelectedVote('abstain')}
              >
                <Minus className="w-6 h-6 mb-2" />
                <span>Abstain</span>
              </Button>
            </div>

            {/* Reason (Optional) */}
            <div>
              <label className="text-sm font-medium mb-2 block">
                Reason (Optional)
              </label>
              <Textarea
                value={voteReason}
                onChange={(e) => setVoteReason(e.target.value)}
                placeholder="Explain your vote..."
                rows={4}
              />
            </div>
          </div>

          <DialogFooter>
            <Button
              variant="outline"
              onClick={() => setShowVoteDialog(false)}
              disabled={isLoading}
            >
              Cancel
            </Button>
            <Button onClick={handleVote} disabled={isLoading}>
              {isLoading ? 'Submitting...' : 'Submit Vote'}
            </Button>
          </DialogFooter>
        </DialogContent>
      </Dialog>
    </div>
  );
}
