'use client';

import React from 'react';
import Link from 'next/link';
import {
  CheckCircle2,
  AlertTriangle,
  Shield,
  TrendingUp,
  Users,
  Award,
  Info,
  ExternalLink
} from 'lucide-react';
import { Card, CardContent } from '@/components/ui/card';
import { Button } from '@/components/ui/button';
import { Badge } from '@/components/ui/badge';
import { Progress } from '@/components/ui/progress';
import { Checkbox } from '@/components/ui/checkbox';
import {
  Tooltip,
  TooltipContent,
  TooltipProvider,
  TooltipTrigger
} from '@/components/ui/tooltip';

interface Validator {
  id: string;
  name: string;
  address: string;
  apy: number;
  commission: number;
  totalStake: string;
  nominatorCount: number;
  uptime: number;
  blocksProduced: number;
  isVerified: boolean;
  status: 'active' | 'waiting' | 'inactive';
  lastReward: string;
  riskScore: 'low' | 'medium' | 'high';
}

interface ValidatorCardProps {
  validator: Validator;
  isSelected?: boolean;
  onToggleSelect?: () => void;
  showCheckbox?: boolean;
}

export function ValidatorCard({
  validator,
  isSelected = false,
  onToggleSelect,
  showCheckbox = true
}: ValidatorCardProps) {
  const getRiskBadgeVariant = (score: Validator['riskScore']) => {
    switch (score) {
      case 'low':
        return 'default';
      case 'medium':
        return 'secondary';
      case 'high':
        return 'destructive';
    }
  };

  const getRiskColor = (score: Validator['riskScore']) => {
    switch (score) {
      case 'low':
        return 'text-green-600';
      case 'medium':
        return 'text-yellow-600';
      case 'high':
        return 'text-red-600';
    }
  };

  return (
    <Card
      className={`transition-all duration-200 ${
        isSelected ? 'ring-2 ring-primary shadow-lg' : 'hover:shadow-md'
      }`}
    >
      <CardContent className="pt-6">
        <div className="space-y-4">
          {/* Header Section */}
          <div className="flex items-start justify-between">
            <div className="flex items-start gap-3 flex-1">
              {showCheckbox && onToggleSelect && (
                <Checkbox
                  checked={isSelected}
                  onCheckedChange={onToggleSelect}
                  className="mt-1"
                />
              )}
              <div className="flex-1 space-y-1">
                <div className="flex items-center gap-2 flex-wrap">
                  <h3 className="text-xl font-bold">{validator.name}</h3>
                  {validator.isVerified && (
                    <TooltipProvider>
                      <Tooltip>
                        <TooltipTrigger>
                          <CheckCircle2 className="h-5 w-5 text-blue-600" />
                        </TooltipTrigger>
                        <TooltipContent>
                          <p>Verified Validator</p>
                        </TooltipContent>
                      </Tooltip>
                    </TooltipProvider>
                  )}
                  <Badge variant={validator.status === 'active' ? 'default' : 'secondary'}>
                    {validator.status}
                  </Badge>
                </div>
                <p className="text-sm text-muted-foreground font-mono">
                  {validator.address}
                </p>
              </div>
            </div>

            {/* Risk Score */}
            <TooltipProvider>
              <Tooltip>
                <TooltipTrigger>
                  <div className="text-right">
                    <Badge variant={getRiskBadgeVariant(validator.riskScore)}>
                      <Shield className="h-3 w-3 mr-1" />
                      {validator.riskScore.toUpperCase()}
                    </Badge>
                  </div>
                </TooltipTrigger>
                <TooltipContent>
                  <p>Risk Assessment</p>
                </TooltipContent>
              </Tooltip>
            </TooltipProvider>
          </div>

          {/* Key Metrics Grid */}
          <div className="grid grid-cols-2 md:grid-cols-4 gap-4">
            <div className="space-y-1">
              <div className="flex items-center gap-1">
                <TrendingUp className="h-3 w-3 text-muted-foreground" />
                <p className="text-xs text-muted-foreground">APY</p>
              </div>
              <p className="text-2xl font-bold text-green-600">{validator.apy}%</p>
            </div>

            <div className="space-y-1">
              <div className="flex items-center gap-1">
                <Award className="h-3 w-3 text-muted-foreground" />
                <p className="text-xs text-muted-foreground">Commission</p>
              </div>
              <p className="text-2xl font-bold">{validator.commission}%</p>
            </div>

            <div className="space-y-1">
              <div className="flex items-center gap-1">
                <Users className="h-3 w-3 text-muted-foreground" />
                <p className="text-xs text-muted-foreground">Nominators</p>
              </div>
              <p className="text-2xl font-bold">{validator.nominatorCount}</p>
            </div>

            <div className="space-y-1">
              <p className="text-xs text-muted-foreground">Total Stake</p>
              <p className="text-xl font-bold">{validator.totalStake}</p>
            </div>
          </div>

          {/* Uptime Progress */}
          <div className="space-y-2">
            <div className="flex items-center justify-between">
              <div className="flex items-center gap-1">
                <p className="text-sm font-medium">Uptime</p>
                <TooltipProvider>
                  <Tooltip>
                    <TooltipTrigger>
                      <Info className="h-3 w-3 text-muted-foreground" />
                    </TooltipTrigger>
                    <TooltipContent>
                      <p>Validator availability in the last 100 eras</p>
                    </TooltipContent>
                  </Tooltip>
                </TooltipProvider>
              </div>
              <span className={`text-sm font-semibold ${
                validator.uptime >= 99 ? 'text-green-600' :
                validator.uptime >= 98 ? 'text-yellow-600' : 'text-red-600'
              }`}>
                {validator.uptime}%
              </span>
            </div>
            <Progress
              value={validator.uptime}
              className="h-2"
            />
          </div>

          {/* Additional Stats */}
          <div className="flex items-center justify-between text-sm">
            <div className="space-y-1">
              <p className="text-muted-foreground">Blocks Produced</p>
              <p className="font-semibold">{validator.blocksProduced.toLocaleString()}</p>
            </div>
            <div className="text-right space-y-1">
              <p className="text-muted-foreground">Last Reward</p>
              <p className="font-semibold">{validator.lastReward}</p>
            </div>
          </div>

          {/* Performance Indicators */}
          <div className="flex items-center gap-2 flex-wrap">
            {validator.apy >= 13 && (
              <Badge variant="secondary" className="text-xs">
                <TrendingUp className="h-3 w-3 mr-1" />
                High APY
              </Badge>
            )}
            {validator.commission <= 5 && (
              <Badge variant="secondary" className="text-xs">
                <Award className="h-3 w-3 mr-1" />
                Low Commission
              </Badge>
            )}
            {validator.uptime >= 99.5 && (
              <Badge variant="secondary" className="text-xs">
                <CheckCircle2 className="h-3 w-3 mr-1" />
                High Uptime
              </Badge>
            )}
            {validator.uptime < 99 && (
              <Badge variant="destructive" className="text-xs">
                <AlertTriangle className="h-3 w-3 mr-1" />
                Low Uptime
              </Badge>
            )}
          </div>

          {/* Action Buttons */}
          <div className="flex gap-2 pt-2">
            <Link href={`/staking/nomination-manager?validator=${validator.id}`} className="flex-1">
              <Button className="w-full">
                Nominate
              </Button>
            </Link>
            <TooltipProvider>
              <Tooltip>
                <TooltipTrigger asChild>
                  <Link
                    href={`/staking/validators/${validator.id}`}
                    className="flex-none"
                  >
                    <Button variant="outline" size="icon">
                      <Info className="h-4 w-4" />
                    </Button>
                  </Link>
                </TooltipTrigger>
                <TooltipContent>
                  <p>View Details</p>
                </TooltipContent>
              </Tooltip>
            </TooltipProvider>
            <TooltipProvider>
              <Tooltip>
                <TooltipTrigger asChild>
                  <Link
                    href={`https://explorer.etrid.network/validator/${validator.address}`}
                    target="_blank"
                    rel="noopener noreferrer"
                  >
                    <Button variant="outline" size="icon">
                      <ExternalLink className="h-4 w-4" />
                    </Button>
                  </Link>
                </TooltipTrigger>
                <TooltipContent>
                  <p>View on Explorer</p>
                </TooltipContent>
              </Tooltip>
            </TooltipProvider>
          </div>
        </div>
      </CardContent>
    </Card>
  );
}
