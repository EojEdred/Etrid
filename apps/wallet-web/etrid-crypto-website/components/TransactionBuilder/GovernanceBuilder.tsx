/**
 * GovernanceBuilder - Governance Transaction Builder Component
 * Handles proposal voting, proposal creation, and delegation
 */

'use client';

import React, { useState, useEffect } from 'react';
import { useForm } from 'react-hook-form';
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from '@/components/ui/card';
import { Button } from '@/components/ui/button';
import { Input } from '@/components/ui/input';
import { Label } from '@/components/ui/label';
import { Textarea } from '@/components/ui/textarea';
import { RadioGroup, RadioGroupItem } from '@/components/ui/radio-group';
import { Select, SelectContent, SelectItem, SelectTrigger, SelectValue } from '@/components/ui/select';
import { Alert, AlertDescription } from '@/components/ui/alert';
import { Badge } from '@/components/ui/badge';
import { Tabs, TabsContent, TabsList, TabsTrigger } from '@/components/ui/tabs';
import { useWallet } from '@/lib/polkadot/useWallet';
import { CHAINS, ChainId } from '@/lib/polkadot/chains';
import { AlertCircle, ArrowRight, ThumbsUp, ThumbsDown, FileText, Users, Loader2, Info } from 'lucide-react';

type GovernanceAction = 'vote' | 'propose' | 'delegate';
type VoteType = 'aye' | 'nay' | 'abstain';

interface Proposal {
  id: number;
  title: string;
  description: string;
  proposer: string;
  status: 'active' | 'passed' | 'rejected';
  ayeVotes: number;
  nayVotes: number;
  endsAt: string;
}

interface GovernanceFormData {
  action: GovernanceAction;
  proposalId?: number;
  voteType?: VoteType;
  conviction?: number;
  voteAmount?: string;
  delegateAddress?: string;
  proposalTitle?: string;
  proposalDescription?: string;
  proposalDeposit?: string;
  chainId: ChainId;
}

interface GovernanceBuilderProps {
  onComplete: (data: GovernanceFormData & { estimatedFee: string }) => void;
}

export function GovernanceBuilder({ onComplete }: GovernanceBuilderProps) {
  const { selectedAccount, selectedChain } = useWallet();
  const [estimatedFee, setEstimatedFee] = useState<string>('0');
  const [isCalculating, setIsCalculating] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const [proposals, setProposals] = useState<Proposal[]>([]);
  const [selectedProposal, setSelectedProposal] = useState<Proposal | null>(null);

  const {
    register,
    handleSubmit,
    watch,
    formState: { errors, isValid },
    setValue,
  } = useForm<GovernanceFormData>({
    mode: 'onChange',
    defaultValues: {
      action: 'vote',
      voteType: 'aye',
      conviction: 1,
      voteAmount: '',
      chainId: selectedChain,
    },
  });

  const watchedAction = watch('action');
  const watchedProposalId = watch('proposalId');
  const watchedVoteType = watch('voteType');
  const watchedVoteAmount = watch('voteAmount');

  // Fetch active proposals
  useEffect(() => {
    const fetchProposals = async () => {
      try {
        // Simulated proposals for demonstration
        const mockProposals: Proposal[] = [
          {
            id: 1,
            title: 'Upgrade Runtime to v2.0',
            description: 'Proposal to upgrade the runtime to version 2.0 with improved performance and new features.',
            proposer: '5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY',
            status: 'active',
            ayeVotes: 1250000,
            nayVotes: 450000,
            endsAt: '2025-11-15',
          },
          {
            id: 2,
            title: 'Reduce Transaction Fees',
            description: 'Proposal to reduce transaction fees by 20% to improve accessibility.',
            proposer: '5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty',
            status: 'active',
            ayeVotes: 980000,
            nayVotes: 320000,
            endsAt: '2025-11-20',
          },
          {
            id: 3,
            title: 'Treasury Funding for Developer Tools',
            description: 'Allocate 50,000 ETR from treasury to fund developer tooling improvements.',
            proposer: '5DAAnrj7VHTznn2AWBemMuyBwZWs6FNFjdyVXUeYum3PTXFy',
            status: 'active',
            ayeVotes: 750000,
            nayVotes: 850000,
            endsAt: '2025-11-18',
          },
        ];
        setProposals(mockProposals);
      } catch (err) {
        console.error('Failed to fetch proposals:', err);
      }
    };

    fetchProposals();
  }, [selectedChain]);

  // Update selected proposal
  useEffect(() => {
    if (watchedProposalId) {
      const proposal = proposals.find((p) => p.id === watchedProposalId);
      setSelectedProposal(proposal || null);
    }
  }, [watchedProposalId, proposals]);

  // Estimate transaction fee
  useEffect(() => {
    const calculateFee = async () => {
      setIsCalculating(true);
      try {
        const config = CHAINS[selectedChain];
        let baseFee = '0.01';

        if (watchedAction === 'vote') {
          baseFee = '0.005';
        } else if (watchedAction === 'propose') {
          baseFee = '0.1'; // Higher fee for creating proposals
        } else if (watchedAction === 'delegate') {
          baseFee = '0.01';
        }

        setEstimatedFee(baseFee);
      } catch (err) {
        console.error('Fee calculation error:', err);
        setEstimatedFee('0.01');
      } finally {
        setIsCalculating(false);
      }
    };

    const timer = setTimeout(calculateFee, 500);
    return () => clearTimeout(timer);
  }, [watchedAction, selectedChain]);

  const onSubmit = (data: GovernanceFormData) => {
    setError(null);

    // Validation based on action
    if (data.action === 'vote' && !data.proposalId) {
      setError('Please select a proposal to vote on');
      return;
    }

    if (data.action === 'delegate' && !data.delegateAddress) {
      setError('Please provide a delegate address');
      return;
    }

    if (data.action === 'propose') {
      if (!data.proposalTitle || !data.proposalDescription) {
        setError('Please provide proposal title and description');
        return;
      }
      if (!data.proposalDeposit || parseFloat(data.proposalDeposit) < 10) {
        setError('Minimum proposal deposit is 10 ETR');
        return;
      }
    }

    // Pass data to parent
    onComplete({
      ...data,
      estimatedFee,
    });
  };

  const chainConfig = CHAINS[selectedChain];

  return (
    <form onSubmit={handleSubmit(onSubmit)} className="space-y-6">
      {/* Action Selection */}
      <Tabs value={watchedAction} onValueChange={(v) => setValue('action', v as GovernanceAction)}>
        <TabsList className="grid w-full grid-cols-3">
          <TabsTrigger value="vote" className="gap-2">
            <ThumbsUp className="w-4 h-4" />
            Vote
          </TabsTrigger>
          <TabsTrigger value="propose" className="gap-2">
            <FileText className="w-4 h-4" />
            Propose
          </TabsTrigger>
          <TabsTrigger value="delegate" className="gap-2">
            <Users className="w-4 h-4" />
            Delegate
          </TabsTrigger>
        </TabsList>

        {/* Vote Tab */}
        <TabsContent value="vote" className="space-y-4 mt-6">
          {/* Proposal Selection */}
          <div className="space-y-3">
            <Label>Select Proposal</Label>
            {proposals.length === 0 ? (
              <Alert>
                <Info className="h-4 w-4" />
                <AlertDescription>No active proposals at the moment</AlertDescription>
              </Alert>
            ) : (
              <div className="space-y-2">
                {proposals.map((proposal) => (
                  <Card
                    key={proposal.id}
                    className={`cursor-pointer transition-all ${
                      watchedProposalId === proposal.id
                        ? 'border-primary ring-2 ring-primary/20'
                        : 'hover:border-primary/50'
                    }`}
                    onClick={() => setValue('proposalId', proposal.id)}
                  >
                    <CardContent className="pt-4 pb-4">
                      <div className="flex items-start justify-between gap-4">
                        <div className="flex-1 space-y-2">
                          <div className="flex items-center gap-2">
                            <Badge variant="secondary">#{proposal.id}</Badge>
                            <h4 className="font-medium">{proposal.title}</h4>
                          </div>
                          <p className="text-sm text-muted-foreground line-clamp-2">
                            {proposal.description}
                          </p>
                          <div className="flex items-center gap-4 text-xs text-muted-foreground">
                            <span className="flex items-center gap-1">
                              <ThumbsUp className="w-3 h-3 text-green-500" />
                              {proposal.ayeVotes.toLocaleString()}
                            </span>
                            <span className="flex items-center gap-1">
                              <ThumbsDown className="w-3 h-3 text-red-500" />
                              {proposal.nayVotes.toLocaleString()}
                            </span>
                            <span>Ends: {proposal.endsAt}</span>
                          </div>
                        </div>
                      </div>
                    </CardContent>
                  </Card>
                ))}
              </div>
            )}
          </div>

          {/* Vote Type */}
          {watchedProposalId && (
            <>
              <div className="space-y-3">
                <Label>Your Vote</Label>
                <RadioGroup
                  value={watchedVoteType}
                  onValueChange={(v) => setValue('voteType', v as VoteType)}
                >
                  <div className="grid grid-cols-3 gap-3">
                    <Card
                      className={`cursor-pointer ${
                        watchedVoteType === 'aye' ? 'border-green-500 ring-2 ring-green-500/20' : ''
                      }`}
                      onClick={() => setValue('voteType', 'aye')}
                    >
                      <CardContent className="pt-4 pb-4 flex flex-col items-center gap-2">
                        <RadioGroupItem value="aye" id="aye" className="sr-only" />
                        <ThumbsUp className="w-6 h-6 text-green-500" />
                        <Label htmlFor="aye" className="cursor-pointer">Aye</Label>
                      </CardContent>
                    </Card>
                    <Card
                      className={`cursor-pointer ${
                        watchedVoteType === 'nay' ? 'border-red-500 ring-2 ring-red-500/20' : ''
                      }`}
                      onClick={() => setValue('voteType', 'nay')}
                    >
                      <CardContent className="pt-4 pb-4 flex flex-col items-center gap-2">
                        <RadioGroupItem value="nay" id="nay" className="sr-only" />
                        <ThumbsDown className="w-6 h-6 text-red-500" />
                        <Label htmlFor="nay" className="cursor-pointer">Nay</Label>
                      </CardContent>
                    </Card>
                    <Card
                      className={`cursor-pointer ${
                        watchedVoteType === 'abstain' ? 'border-primary ring-2 ring-primary/20' : ''
                      }`}
                      onClick={() => setValue('voteType', 'abstain')}
                    >
                      <CardContent className="pt-4 pb-4 flex flex-col items-center gap-2">
                        <RadioGroupItem value="abstain" id="abstain" className="sr-only" />
                        <span className="w-6 h-6 flex items-center justify-center text-xl">—</span>
                        <Label htmlFor="abstain" className="cursor-pointer">Abstain</Label>
                      </CardContent>
                    </Card>
                  </div>
                </RadioGroup>
              </div>

              {/* Vote Amount & Conviction */}
              <div className="grid grid-cols-2 gap-4">
                <div className="space-y-2">
                  <Label htmlFor="voteAmount">Vote Amount</Label>
                  <div className="relative">
                    <Input
                      id="voteAmount"
                      type="number"
                      step="any"
                      placeholder="0.00"
                      {...register('voteAmount', {
                        required: 'Vote amount is required',
                      })}
                      className="pr-16"
                    />
                    <div className="absolute right-3 top-1/2 -translate-y-1/2">
                      <Badge variant="secondary">{chainConfig.symbol}</Badge>
                    </div>
                  </div>
                </div>
                <div className="space-y-2">
                  <Label htmlFor="conviction">Conviction</Label>
                  <Select
                    value={watch('conviction')?.toString()}
                    onValueChange={(v) => setValue('conviction', parseInt(v))}
                  >
                    <SelectTrigger id="conviction">
                      <SelectValue />
                    </SelectTrigger>
                    <SelectContent>
                      <SelectItem value="1">1x (0 days)</SelectItem>
                      <SelectItem value="2">2x (7 days)</SelectItem>
                      <SelectItem value="3">3x (14 days)</SelectItem>
                      <SelectItem value="4">4x (28 days)</SelectItem>
                      <SelectItem value="5">5x (56 days)</SelectItem>
                      <SelectItem value="6">6x (112 days)</SelectItem>
                    </SelectContent>
                  </Select>
                </div>
              </div>

              <Alert>
                <Info className="h-4 w-4" />
                <AlertDescription className="text-xs">
                  Higher conviction multiplies your voting power but locks your tokens for longer
                </AlertDescription>
              </Alert>
            </>
          )}
        </TabsContent>

        {/* Propose Tab */}
        <TabsContent value="propose" className="space-y-4 mt-6">
          <div className="space-y-2">
            <Label htmlFor="proposalTitle">Proposal Title</Label>
            <Input
              id="proposalTitle"
              placeholder="Enter proposal title"
              {...register('proposalTitle', { required: true })}
            />
          </div>

          <div className="space-y-2">
            <Label htmlFor="proposalDescription">Proposal Description</Label>
            <Textarea
              id="proposalDescription"
              placeholder="Describe your proposal in detail..."
              rows={6}
              {...register('proposalDescription', { required: true })}
            />
          </div>

          <div className="space-y-2">
            <Label htmlFor="proposalDeposit">Proposal Deposit</Label>
            <div className="relative">
              <Input
                id="proposalDeposit"
                type="number"
                step="any"
                placeholder="Minimum 10.00"
                {...register('proposalDeposit', {
                  required: true,
                  min: { value: 10, message: 'Minimum deposit is 10 ETR' },
                })}
                className="pr-16"
              />
              <div className="absolute right-3 top-1/2 -translate-y-1/2">
                <Badge variant="secondary">{chainConfig.symbol}</Badge>
              </div>
            </div>
            {errors.proposalDeposit && (
              <p className="text-sm text-destructive">{errors.proposalDeposit.message}</p>
            )}
          </div>

          <Alert>
            <Info className="h-4 w-4" />
            <AlertDescription className="text-xs">
              Proposal deposit will be returned if the proposal passes. Minimum deposit: 10 ETR
            </AlertDescription>
          </Alert>
        </TabsContent>

        {/* Delegate Tab */}
        <TabsContent value="delegate" className="space-y-4 mt-6">
          <div className="space-y-2">
            <Label htmlFor="delegateAddress">Delegate Address</Label>
            <Input
              id="delegateAddress"
              placeholder="5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY"
              {...register('delegateAddress', {
                required: watchedAction === 'delegate',
              })}
            />
          </div>

          <Alert>
            <Info className="h-4 w-4" />
            <AlertDescription className="text-xs">
              Delegating allows another account to vote on your behalf with your voting power
            </AlertDescription>
          </Alert>
        </TabsContent>
      </Tabs>

      {/* Fee Estimation */}
      <Card className="bg-muted/50">
        <CardContent className="pt-6">
          <div className="flex items-center justify-between">
            <span className="text-sm font-medium">Estimated Fee</span>
            {isCalculating ? (
              <Loader2 className="w-4 h-4 animate-spin" />
            ) : (
              <Badge variant="secondary" className="font-mono">
                {estimatedFee} {chainConfig.symbol}
              </Badge>
            )}
          </div>
        </CardContent>
      </Card>

      {/* Error Display */}
      {error && (
        <Alert variant="destructive">
          <AlertCircle className="h-4 w-4" />
          <AlertDescription>{error}</AlertDescription>
        </Alert>
      )}

      {/* Submit Button */}
      <div className="flex gap-3 pt-4">
        <Button type="submit" className="flex-1" disabled={!isValid || isCalculating}>
          Review {watchedAction === 'vote' ? 'Vote' : watchedAction === 'propose' ? 'Proposal' : 'Delegation'}
          <ArrowRight className="w-4 h-4 ml-2" />
        </Button>
      </div>
    </form>
  );
}
