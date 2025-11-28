/**
 * VoteBreakdown Component
 * Displays detailed voting results for a proposal
 */

'use client';

import React from 'react';
import { VoteBreakdown as VoteBreakdownType } from '@/types/dao';
import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card';
import { Progress } from '@/components/ui/progress';
import { Avatar, AvatarFallback, AvatarImage } from '@/components/ui/avatar';
import { Badge } from '@/components/ui/badge';
import { Separator } from '@/components/ui/separator';
import { ThumbsUp, ThumbsDown, Minus, CheckCircle2, XCircle } from 'lucide-react';

interface VoteBreakdownProps {
  breakdown: VoteBreakdownType;
}

export function VoteBreakdown({ breakdown }: VoteBreakdownProps) {
  const quorumMet = breakdown.quorumProgress >= 100;

  return (
    <Card>
      <CardHeader>
        <CardTitle>Vote Breakdown</CardTitle>
      </CardHeader>

      <CardContent className="space-y-6">
        {/* Quorum Progress */}
        <div className="space-y-2">
          <div className="flex items-center justify-between text-sm">
            <span className="text-muted-foreground">Quorum Progress:</span>
            <span className={quorumMet ? 'text-green-600 font-medium' : 'font-medium'}>
              {breakdown.quorumProgress.toFixed(1)}% / {breakdown.quorumRequired}%
            </span>
          </div>
          <Progress value={Math.min(breakdown.quorumProgress, 100)} className="h-2" />
          {quorumMet ? (
            <div className="flex items-center gap-1 text-sm text-green-600">
              <CheckCircle2 className="w-4 h-4" />
              <span>Quorum reached</span>
            </div>
          ) : (
            <div className="flex items-center gap-1 text-sm text-muted-foreground">
              <XCircle className="w-4 h-4" />
              <span>Quorum not reached</span>
            </div>
          )}
        </div>

        <Separator />

        {/* For Votes */}
        <div>
          <div className="flex items-center justify-between mb-3">
            <div className="flex items-center gap-2">
              <ThumbsUp className="w-5 h-5 text-green-600" />
              <h3 className="font-semibold">For</h3>
            </div>
            <div className="text-right">
              <p className="text-lg font-bold text-green-600">
                {breakdown.for.count.toLocaleString()}
              </p>
              <p className="text-sm text-muted-foreground">
                {breakdown.for.percentage.toFixed(1)}%
              </p>
            </div>
          </div>
          <Progress value={breakdown.for.percentage} className="h-2 mb-3" />
          <div className="space-y-2">
            {breakdown.for.voters.slice(0, 5).map((voter) => (
              <VoterCard key={voter.id} voter={voter} />
            ))}
            {breakdown.for.voters.length > 5 && (
              <p className="text-xs text-muted-foreground text-center">
                +{breakdown.for.voters.length - 5} more voters
              </p>
            )}
          </div>
        </div>

        <Separator />

        {/* Against Votes */}
        <div>
          <div className="flex items-center justify-between mb-3">
            <div className="flex items-center gap-2">
              <ThumbsDown className="w-5 h-5 text-red-600" />
              <h3 className="font-semibold">Against</h3>
            </div>
            <div className="text-right">
              <p className="text-lg font-bold text-red-600">
                {breakdown.against.count.toLocaleString()}
              </p>
              <p className="text-sm text-muted-foreground">
                {breakdown.against.percentage.toFixed(1)}%
              </p>
            </div>
          </div>
          <Progress value={breakdown.against.percentage} className="h-2 mb-3" />
          <div className="space-y-2">
            {breakdown.against.voters.slice(0, 5).map((voter) => (
              <VoterCard key={voter.id} voter={voter} />
            ))}
            {breakdown.against.voters.length > 5 && (
              <p className="text-xs text-muted-foreground text-center">
                +{breakdown.against.voters.length - 5} more voters
              </p>
            )}
          </div>
        </div>

        {/* Abstain Votes */}
        {breakdown.abstain.count > 0 && (
          <>
            <Separator />
            <div>
              <div className="flex items-center justify-between mb-3">
                <div className="flex items-center gap-2">
                  <Minus className="w-5 h-5 text-muted-foreground" />
                  <h3 className="font-semibold">Abstain</h3>
                </div>
                <div className="text-right">
                  <p className="text-lg font-bold">
                    {breakdown.abstain.count.toLocaleString()}
                  </p>
                  <p className="text-sm text-muted-foreground">
                    {breakdown.abstain.percentage.toFixed(1)}%
                  </p>
                </div>
              </div>
              <div className="space-y-2">
                {breakdown.abstain.voters.slice(0, 5).map((voter) => (
                  <VoterCard key={voter.id} voter={voter} />
                ))}
                {breakdown.abstain.voters.length > 5 && (
                  <p className="text-xs text-muted-foreground text-center">
                    +{breakdown.abstain.voters.length - 5} more voters
                  </p>
                )}
              </div>
            </div>
          </>
        )}
      </CardContent>
    </Card>
  );
}

function VoterCard({ voter }: { voter: any }) {
  return (
    <div className="flex items-center gap-2 p-2 rounded-lg bg-muted/50">
      <Avatar className="w-8 h-8">
        <AvatarImage src={voter.avatarUrl} alt={voter.username} />
        <AvatarFallback>
          {voter.username?.charAt(0) || voter.address.slice(0, 2)}
        </AvatarFallback>
      </Avatar>
      <div className="flex-1 min-w-0">
        <p className="text-sm font-medium truncate">
          {voter.username || `${voter.address.slice(0, 6)}...${voter.address.slice(-4)}`}
        </p>
      </div>
      <Badge variant="outline" className="text-xs">
        {voter.votingPower.toLocaleString()}
      </Badge>
    </div>
  );
}
