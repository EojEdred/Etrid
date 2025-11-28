/**
 * WalletConnectModal Component
 * Modal for approving/rejecting WalletConnect connection requests
 */

'use client';

import React, { useState } from 'react';
import { WalletConnectProposal } from '@/types/dapp';
import {
  Dialog,
  DialogContent,
  DialogDescription,
  DialogFooter,
  DialogHeader,
  DialogTitle,
} from '@/components/ui/dialog';
import { Button } from '@/components/ui/button';
import { Checkbox } from '@/components/ui/checkbox';
import { Badge } from '@/components/ui/badge';
import { ScrollArea } from '@/components/ui/scroll-area';
import { AlertTriangle, CheckCircle2, XCircle } from 'lucide-react';

interface WalletConnectModalProps {
  proposal: WalletConnectProposal | null;
  open: boolean;
  onApprove: (rememberChoice: boolean) => void;
  onReject: () => void;
  onClose: () => void;
}

export function WalletConnectModal({
  proposal,
  open,
  onApprove,
  onReject,
  onClose,
}: WalletConnectModalProps) {
  const [rememberChoice, setRememberChoice] = useState(false);
  const [isLoading, setIsLoading] = useState(false);

  if (!proposal) return null;

  const handleApprove = async () => {
    setIsLoading(true);
    try {
      await onApprove(rememberChoice);
    } finally {
      setIsLoading(false);
    }
  };

  const handleReject = async () => {
    setIsLoading(true);
    try {
      await onReject();
    } finally {
      setIsLoading(false);
    }
  };

  return (
    <Dialog open={open} onOpenChange={onClose}>
      <DialogContent className="max-w-md">
        <DialogHeader>
          <DialogTitle className="flex items-center gap-2">
            Connection Request
          </DialogTitle>
          <DialogDescription>
            Review the permissions requested by this dApp
          </DialogDescription>
        </DialogHeader>

        <div className="space-y-4">
          {/* dApp Info */}
          <div className="flex items-start gap-3 p-3 rounded-lg bg-muted">
            {proposal.proposer.icons?.[0] && (
              <img
                src={proposal.proposer.icons[0]}
                alt={proposal.proposer.name}
                className="w-12 h-12 rounded-lg"
              />
            )}
            <div className="flex-1 min-w-0">
              <h3 className="font-semibold truncate">
                {proposal.proposer.name}
              </h3>
              <p className="text-sm text-muted-foreground truncate">
                {proposal.proposer.url}
              </p>
              {proposal.proposer.description && (
                <p className="text-xs text-muted-foreground mt-1">
                  {proposal.proposer.description}
                </p>
              )}
            </div>
          </div>

          {/* Requested Permissions */}
          <div>
            <h4 className="text-sm font-medium mb-2">Requested Permissions:</h4>
            <ScrollArea className="h-32 border rounded-md p-3">
              <div className="space-y-2">
                {/* Chains */}
                <div>
                  <p className="text-xs font-medium text-muted-foreground mb-1">
                    Networks:
                  </p>
                  <div className="flex flex-wrap gap-1">
                    {proposal.permissions.chains.map((chain) => (
                      <Badge key={chain} variant="outline" className="text-xs">
                        {chain}
                      </Badge>
                    ))}
                  </div>
                </div>

                {/* Methods */}
                <div>
                  <p className="text-xs font-medium text-muted-foreground mb-1">
                    Methods:
                  </p>
                  <div className="flex flex-wrap gap-1">
                    {proposal.permissions.methods.map((method) => (
                      <Badge key={method} variant="outline" className="text-xs">
                        {method}
                      </Badge>
                    ))}
                  </div>
                </div>

                {/* Events */}
                {proposal.permissions.events.length > 0 && (
                  <div>
                    <p className="text-xs font-medium text-muted-foreground mb-1">
                      Events:
                    </p>
                    <div className="flex flex-wrap gap-1">
                      {proposal.permissions.events.map((event) => (
                        <Badge key={event} variant="outline" className="text-xs">
                          {event}
                        </Badge>
                      ))}
                    </div>
                  </div>
                )}
              </div>
            </ScrollArea>
          </div>

          {/* Warning */}
          <div className="flex items-start gap-2 p-3 rounded-lg bg-yellow-50 dark:bg-yellow-950 border border-yellow-200 dark:border-yellow-800">
            <AlertTriangle className="w-4 h-4 text-yellow-600 dark:text-yellow-400 mt-0.5 flex-shrink-0" />
            <div className="text-xs text-yellow-700 dark:text-yellow-300">
              Only connect to dApps you trust. This dApp will be able to read
              your account balance and request transaction approvals.
            </div>
          </div>

          {/* Remember Choice */}
          <div className="flex items-center space-x-2">
            <Checkbox
              id="remember"
              checked={rememberChoice}
              onCheckedChange={(checked) => setRememberChoice(checked as boolean)}
            />
            <label
              htmlFor="remember"
              className="text-sm font-medium leading-none peer-disabled:cursor-not-allowed peer-disabled:opacity-70"
            >
              Auto-approve future requests from this dApp
            </label>
          </div>
        </div>

        <DialogFooter className="gap-2 sm:gap-0">
          <Button
            variant="outline"
            onClick={handleReject}
            disabled={isLoading}
            className="flex-1"
          >
            <XCircle className="w-4 h-4 mr-2" />
            Reject
          </Button>
          <Button
            onClick={handleApprove}
            disabled={isLoading}
            className="flex-1"
          >
            <CheckCircle2 className="w-4 h-4 mr-2" />
            {isLoading ? 'Connecting...' : 'Approve'}
          </Button>
        </DialogFooter>
      </DialogContent>
    </Dialog>
  );
}
