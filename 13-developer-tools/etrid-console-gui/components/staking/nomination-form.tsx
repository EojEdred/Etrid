'use client';

import React, { useState } from 'react';
import {
  Plus,
  Minus,
  AlertCircle,
  Info,
  CheckCircle2,
  Wallet,
  TrendingUp,
  Search,
  X
} from 'lucide-react';
import { Button } from '@/components/ui/button';
import { Input } from '@/components/ui/input';
import { Label } from '@/components/ui/label';
import { Badge } from '@/components/ui/badge';
import { Alert, AlertDescription } from '@/components/ui/alert';
import { Separator } from '@/components/ui/separator';
import { Progress } from '@/components/ui/progress';
import {
  Dialog,
  DialogContent,
  DialogDescription,
  DialogHeader,
  DialogTitle,
  DialogTrigger
} from '@/components/ui/dialog';
import { ScrollArea } from '@/components/ui/scroll-area';

interface ValidatorOption {
  id: string;
  name: string;
  address: string;
  apy: number;
  commission: number;
  isVerified: boolean;
}

interface SelectedValidator extends ValidatorOption {
  stakeAmount: string;
}

export function NominationForm() {
  const [availableBalance] = useState('50,000 ETD');
  const [selectedValidators, setSelectedValidators] = useState<SelectedValidator[]>([]);
  const [searchQuery, setSearchQuery] = useState('');
  const [isValidatorDialogOpen, setIsValidatorDialogOpen] = useState(false);

  // Mock validator data
  const [validators] = useState<ValidatorOption[]>([
    {
      id: '1',
      name: 'Validator Alpha',
      address: '0x1234...5678',
      apy: 13.2,
      commission: 5,
      isVerified: true
    },
    {
      id: '2',
      name: 'Beta Staking',
      address: '0x2345...6789',
      apy: 12.8,
      commission: 7,
      isVerified: true
    },
    {
      id: '3',
      name: 'Gamma Node',
      address: '0x3456...7890',
      apy: 12.1,
      commission: 8,
      isVerified: false
    },
    {
      id: '4',
      name: 'Delta Validator',
      address: '0x4567...8901',
      apy: 11.9,
      commission: 6,
      isVerified: true
    },
    {
      id: '5',
      name: 'Epsilon Staking',
      address: '0x5678...9012',
      apy: 12.4,
      commission: 5,
      isVerified: true
    },
    {
      id: '6',
      name: 'Zeta Protocol',
      address: '0x6789...0123',
      apy: 14.1,
      commission: 10,
      isVerified: false
    },
    {
      id: '7',
      name: 'Eta Validator',
      address: '0x7890...1234',
      apy: 11.5,
      commission: 9,
      isVerified: true
    },
    {
      id: '8',
      name: 'Theta Staking',
      address: '0x8901...2345',
      apy: 13.5,
      commission: 6,
      isVerified: true
    }
  ]);

  const filteredValidators = validators.filter(
    (v) =>
      !selectedValidators.find((sv) => sv.id === v.id) &&
      (v.name.toLowerCase().includes(searchQuery.toLowerCase()) ||
        v.address.toLowerCase().includes(searchQuery.toLowerCase()))
  );

  const addValidator = (validator: ValidatorOption) => {
    if (selectedValidators.length >= 16) {
      alert('Maximum 16 validators allowed');
      return;
    }
    setSelectedValidators([...selectedValidators, { ...validator, stakeAmount: '' }]);
    setIsValidatorDialogOpen(false);
    setSearchQuery('');
  };

  const removeValidator = (id: string) => {
    setSelectedValidators(selectedValidators.filter((v) => v.id !== id));
  };

  const updateStakeAmount = (id: string, amount: string) => {
    setSelectedValidators(
      selectedValidators.map((v) => (v.id === id ? { ...v, stakeAmount: amount } : v))
    );
  };

  const distributeEvenly = () => {
    const totalStake = parseFloat(availableBalance.replace(/[^0-9.]/g, ''));
    const perValidator = (totalStake / selectedValidators.length).toFixed(2);
    setSelectedValidators(
      selectedValidators.map((v) => ({ ...v, stakeAmount: perValidator }))
    );
  };

  const totalStaked = selectedValidators.reduce((acc, v) => {
    const amount = parseFloat(v.stakeAmount || '0');
    return acc + amount;
  }, 0);

  const availableBalanceNum = parseFloat(availableBalance.replace(/[^0-9.]/g, ''));
  const remainingBalance = availableBalanceNum - totalStaked;
  const estimatedAPY =
    selectedValidators.reduce((acc, v) => {
      const amount = parseFloat(v.stakeAmount || '0');
      return acc + v.apy * amount;
    }, 0) / (totalStaked || 1);

  const canSubmit =
    selectedValidators.length > 0 &&
    selectedValidators.every((v) => parseFloat(v.stakeAmount || '0') >= 10) &&
    totalStaked <= availableBalanceNum;

  const handleSubmit = (e: React.FormEvent) => {
    e.preventDefault();
    console.log('Submitting nomination:', selectedValidators);
    // Implement submission logic
  };

  return (
    <form onSubmit={handleSubmit} className="space-y-6">
      {/* Balance Info */}
      <Alert>
        <Wallet className="h-4 w-4" />
        <AlertDescription>
          <div className="flex items-center justify-between">
            <span>Available Balance: <strong>{availableBalance}</strong></span>
            <span className={remainingBalance < 0 ? 'text-red-600' : 'text-green-600'}>
              Remaining: <strong>{remainingBalance.toFixed(2)} ETD</strong>
            </span>
          </div>
        </AlertDescription>
      </Alert>

      {/* Validator Selection */}
      <div className="space-y-3">
        <div className="flex items-center justify-between">
          <Label>Selected Validators ({selectedValidators.length}/16)</Label>
          <Dialog open={isValidatorDialogOpen} onOpenChange={setIsValidatorDialogOpen}>
            <DialogTrigger asChild>
              <Button type="button" variant="outline" size="sm">
                <Plus className="mr-2 h-4 w-4" />
                Add Validator
              </Button>
            </DialogTrigger>
            <DialogContent className="max-w-2xl">
              <DialogHeader>
                <DialogTitle>Select Validator</DialogTitle>
                <DialogDescription>
                  Choose a validator to delegate your stake
                </DialogDescription>
              </DialogHeader>
              <div className="space-y-4">
                <div className="relative">
                  <Search className="absolute left-3 top-1/2 -translate-y-1/2 h-4 w-4 text-muted-foreground" />
                  <Input
                    placeholder="Search validators..."
                    className="pl-10"
                    value={searchQuery}
                    onChange={(e) => setSearchQuery(e.target.value)}
                  />
                </div>
                <ScrollArea className="h-[400px]">
                  <div className="space-y-2">
                    {filteredValidators.length === 0 ? (
                      <p className="text-center text-muted-foreground py-8">
                        No validators found
                      </p>
                    ) : (
                      filteredValidators.map((validator) => (
                        <div
                          key={validator.id}
                          className="flex items-center justify-between p-3 border rounded-lg hover:bg-accent transition-colors"
                        >
                          <div className="space-y-1 flex-1">
                            <div className="flex items-center gap-2">
                              <p className="font-semibold">{validator.name}</p>
                              {validator.isVerified && (
                                <CheckCircle2 className="h-4 w-4 text-blue-600" />
                              )}
                            </div>
                            <p className="text-xs text-muted-foreground font-mono">
                              {validator.address}
                            </p>
                          </div>
                          <div className="flex items-center gap-3">
                            <div className="text-right">
                              <p className="text-sm font-semibold text-green-600">
                                {validator.apy}% APY
                              </p>
                              <p className="text-xs text-muted-foreground">
                                {validator.commission}% fee
                              </p>
                            </div>
                            <Button
                              type="button"
                              size="sm"
                              onClick={() => addValidator(validator)}
                            >
                              Add
                            </Button>
                          </div>
                        </div>
                      ))
                    )}
                  </div>
                </ScrollArea>
              </div>
            </DialogContent>
          </Dialog>
        </div>

        {selectedValidators.length === 0 ? (
          <Alert>
            <Info className="h-4 w-4" />
            <AlertDescription>
              Click "Add Validator" to select validators you want to delegate your stake to.
              You can select up to 16 validators.
            </AlertDescription>
          </Alert>
        ) : (
          <div className="space-y-3">
            {selectedValidators.map((validator) => (
              <div key={validator.id} className="border rounded-lg p-4 space-y-3">
                <div className="flex items-start justify-between">
                  <div className="space-y-1 flex-1">
                    <div className="flex items-center gap-2">
                      <p className="font-semibold">{validator.name}</p>
                      {validator.isVerified && (
                        <CheckCircle2 className="h-4 w-4 text-blue-600" />
                      )}
                    </div>
                    <p className="text-xs text-muted-foreground font-mono">
                      {validator.address}
                    </p>
                    <div className="flex items-center gap-3 text-xs">
                      <span className="text-green-600 font-medium">
                        {validator.apy}% APY
                      </span>
                      <span className="text-muted-foreground">
                        {validator.commission}% Commission
                      </span>
                    </div>
                  </div>
                  <Button
                    type="button"
                    variant="ghost"
                    size="icon"
                    onClick={() => removeValidator(validator.id)}
                  >
                    <X className="h-4 w-4" />
                  </Button>
                </div>
                <div className="space-y-2">
                  <Label htmlFor={`stake-${validator.id}`}>Stake Amount (ETD)</Label>
                  <Input
                    id={`stake-${validator.id}`}
                    type="number"
                    min="10"
                    step="0.01"
                    placeholder="Minimum 10 ETD"
                    value={validator.stakeAmount}
                    onChange={(e) => updateStakeAmount(validator.id, e.target.value)}
                  />
                  {validator.stakeAmount && parseFloat(validator.stakeAmount) < 10 && (
                    <p className="text-xs text-red-600">
                      Minimum stake is 10 ETD per validator
                    </p>
                  )}
                </div>
              </div>
            ))}

            {selectedValidators.length > 1 && (
              <Button
                type="button"
                variant="outline"
                className="w-full"
                onClick={distributeEvenly}
              >
                Distribute Evenly
              </Button>
            )}
          </div>
        )}
      </div>

      <Separator />

      {/* Summary */}
      {selectedValidators.length > 0 && (
        <div className="space-y-4">
          <h3 className="font-semibold">Nomination Summary</h3>
          <div className="grid grid-cols-1 md:grid-cols-3 gap-4">
            <div className="p-3 border rounded-lg">
              <p className="text-xs text-muted-foreground mb-1">Total Stake</p>
              <p className="text-xl font-bold">{totalStaked.toFixed(2)} ETD</p>
            </div>
            <div className="p-3 border rounded-lg">
              <p className="text-xs text-muted-foreground mb-1">Estimated APY</p>
              <p className="text-xl font-bold text-green-600">
                {estimatedAPY.toFixed(2)}%
              </p>
            </div>
            <div className="p-3 border rounded-lg">
              <p className="text-xs text-muted-foreground mb-1">Validators</p>
              <p className="text-xl font-bold">{selectedValidators.length}</p>
            </div>
          </div>

          {totalStaked > 0 && (
            <div className="space-y-2">
              <div className="flex justify-between text-sm">
                <span>Stake Distribution</span>
                <span>{((totalStaked / availableBalanceNum) * 100).toFixed(1)}%</span>
              </div>
              <Progress value={(totalStaked / availableBalanceNum) * 100} className="h-2" />
            </div>
          )}

          {/* Projected Earnings */}
          <Alert>
            <TrendingUp className="h-4 w-4" />
            <AlertDescription>
              <p className="font-medium mb-2">Projected Annual Earnings</p>
              <div className="grid grid-cols-3 gap-4 text-sm">
                <div>
                  <p className="text-muted-foreground">30 Days</p>
                  <p className="font-semibold text-green-600">
                    ~{((totalStaked * estimatedAPY) / 100 / 12).toFixed(2)} ETD
                  </p>
                </div>
                <div>
                  <p className="text-muted-foreground">90 Days</p>
                  <p className="font-semibold text-green-600">
                    ~{((totalStaked * estimatedAPY) / 100 / 4).toFixed(2)} ETD
                  </p>
                </div>
                <div>
                  <p className="text-muted-foreground">1 Year</p>
                  <p className="font-semibold text-green-600">
                    ~{((totalStaked * estimatedAPY) / 100).toFixed(2)} ETD
                  </p>
                </div>
              </div>
            </AlertDescription>
          </Alert>
        </div>
      )}

      {/* Warnings */}
      {remainingBalance < 0 && (
        <Alert variant="destructive">
          <AlertCircle className="h-4 w-4" />
          <AlertDescription>
            Total stake exceeds available balance by{' '}
            <strong>{Math.abs(remainingBalance).toFixed(2)} ETD</strong>
          </AlertDescription>
        </Alert>
      )}

      {/* Submit Button */}
      <Button type="submit" className="w-full" size="lg" disabled={!canSubmit}>
        {canSubmit ? 'Create Nomination' : 'Complete Form to Continue'}
      </Button>

      {/* Info */}
      <Alert>
        <Info className="h-4 w-4" />
        <AlertDescription className="text-xs space-y-1">
          <p>Your nomination will become active in the next era (~6 hours)</p>
          <p>Minimum stake per validator: 10 ETD</p>
          <p>Unbonding period: 28 days</p>
          <p>Rewards are distributed automatically at the end of each era</p>
        </AlertDescription>
      </Alert>
    </form>
  );
}
