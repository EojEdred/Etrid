'use client';

import React, { useState, useMemo } from 'react';
import Link from 'next/link';
import {
  Search,
  Filter,
  TrendingUp,
  TrendingDown,
  Award,
  Users,
  Shield,
  ArrowUpDown,
  X,
  CheckCircle2,
  AlertTriangle
} from 'lucide-react';
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from '@/components/ui/card';
import { Button } from '@/components/ui/button';
import { Input } from '@/components/ui/input';
import { Badge } from '@/components/ui/badge';
import { Slider } from '@/components/ui/slider';
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue
} from '@/components/ui/select';
import {
  Sheet,
  SheetContent,
  SheetDescription,
  SheetHeader,
  SheetTitle,
  SheetTrigger
} from '@/components/ui/sheet';
import { Checkbox } from '@/components/ui/checkbox';
import { ValidatorCard } from '@/components/staking/validator-card';

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

type SortField = 'apy' | 'commission' | 'totalStake' | 'uptime' | 'nominatorCount';
type SortDirection = 'asc' | 'desc';

export default function ValidatorBrowser() {
  const [validators] = useState<Validator[]>([
    {
      id: '1',
      name: 'Validator Alpha',
      address: '0x1234...5678',
      apy: 13.2,
      commission: 5,
      totalStake: '1.2M ETD',
      nominatorCount: 234,
      uptime: 99.8,
      blocksProduced: 12453,
      isVerified: true,
      status: 'active',
      lastReward: '124.5 ETD',
      riskScore: 'low'
    },
    {
      id: '2',
      name: 'Beta Staking',
      address: '0x2345...6789',
      apy: 12.8,
      commission: 7,
      totalStake: '980K ETD',
      nominatorCount: 189,
      uptime: 99.5,
      blocksProduced: 11234,
      isVerified: true,
      status: 'active',
      lastReward: '98.2 ETD',
      riskScore: 'low'
    },
    {
      id: '3',
      name: 'Gamma Node',
      address: '0x3456...7890',
      apy: 12.1,
      commission: 8,
      totalStake: '750K ETD',
      nominatorCount: 145,
      uptime: 98.9,
      blocksProduced: 10123,
      isVerified: false,
      status: 'active',
      lastReward: '75.8 ETD',
      riskScore: 'medium'
    },
    {
      id: '4',
      name: 'Delta Validator',
      address: '0x4567...8901',
      apy: 11.9,
      commission: 6,
      totalStake: '890K ETD',
      nominatorCount: 167,
      uptime: 99.2,
      blocksProduced: 10987,
      isVerified: true,
      status: 'active',
      lastReward: '89.4 ETD',
      riskScore: 'low'
    },
    {
      id: '5',
      name: 'Epsilon Staking',
      address: '0x5678...9012',
      apy: 12.4,
      commission: 5,
      totalStake: '1.1M ETD',
      nominatorCount: 212,
      uptime: 99.7,
      blocksProduced: 12156,
      isVerified: true,
      status: 'active',
      lastReward: '118.3 ETD',
      riskScore: 'low'
    },
    {
      id: '6',
      name: 'Zeta Protocol',
      address: '0x6789...0123',
      apy: 14.1,
      commission: 10,
      totalStake: '450K ETD',
      nominatorCount: 98,
      uptime: 97.8,
      blocksProduced: 8945,
      isVerified: false,
      status: 'active',
      lastReward: '45.2 ETD',
      riskScore: 'high'
    },
    {
      id: '7',
      name: 'Eta Validator',
      address: '0x7890...1234',
      apy: 11.5,
      commission: 9,
      totalStake: '620K ETD',
      nominatorCount: 134,
      uptime: 98.5,
      blocksProduced: 9567,
      isVerified: true,
      status: 'active',
      lastReward: '62.8 ETD',
      riskScore: 'medium'
    },
    {
      id: '8',
      name: 'Theta Staking',
      address: '0x8901...2345',
      apy: 13.5,
      commission: 6,
      totalStake: '1.3M ETD',
      nominatorCount: 267,
      uptime: 99.9,
      blocksProduced: 13245,
      isVerified: true,
      status: 'active',
      lastReward: '135.7 ETD',
      riskScore: 'low'
    }
  ]);

  const [searchQuery, setSearchQuery] = useState('');
  const [sortField, setSortField] = useState<SortField>('apy');
  const [sortDirection, setSortDirection] = useState<SortDirection>('desc');
  const [selectedValidators, setSelectedValidators] = useState<string[]>([]);

  // Filters
  const [minAPY, setMinAPY] = useState([0]);
  const [maxCommission, setMaxCommission] = useState([20]);
  const [minUptime, setMinUptime] = useState([95]);
  const [showVerifiedOnly, setShowVerifiedOnly] = useState(false);
  const [riskLevels, setRiskLevels] = useState<string[]>(['low', 'medium', 'high']);

  // Filtered and sorted validators
  const filteredValidators = useMemo(() => {
    return validators
      .filter((v) => {
        const matchesSearch =
          v.name.toLowerCase().includes(searchQuery.toLowerCase()) ||
          v.address.toLowerCase().includes(searchQuery.toLowerCase());
        const matchesAPY = v.apy >= minAPY[0];
        const matchesCommission = v.commission <= maxCommission[0];
        const matchesUptime = v.uptime >= minUptime[0];
        const matchesVerified = !showVerifiedOnly || v.isVerified;
        const matchesRisk = riskLevels.includes(v.riskScore);

        return (
          matchesSearch &&
          matchesAPY &&
          matchesCommission &&
          matchesUptime &&
          matchesVerified &&
          matchesRisk
        );
      })
      .sort((a, b) => {
        let aVal: number, bVal: number;

        switch (sortField) {
          case 'apy':
            aVal = a.apy;
            bVal = b.apy;
            break;
          case 'commission':
            aVal = a.commission;
            bVal = b.commission;
            break;
          case 'totalStake':
            aVal = parseFloat(a.totalStake.replace(/[^0-9.]/g, ''));
            bVal = parseFloat(b.totalStake.replace(/[^0-9.]/g, ''));
            break;
          case 'uptime':
            aVal = a.uptime;
            bVal = b.uptime;
            break;
          case 'nominatorCount':
            aVal = a.nominatorCount;
            bVal = b.nominatorCount;
            break;
          default:
            return 0;
        }

        return sortDirection === 'asc' ? aVal - bVal : bVal - aVal;
      });
  }, [validators, searchQuery, sortField, sortDirection, minAPY, maxCommission, minUptime, showVerifiedOnly, riskLevels]);

  const toggleSort = (field: SortField) => {
    if (sortField === field) {
      setSortDirection(sortDirection === 'asc' ? 'desc' : 'asc');
    } else {
      setSortField(field);
      setSortDirection('desc');
    }
  };

  const toggleValidatorSelection = (id: string) => {
    setSelectedValidators((prev) =>
      prev.includes(id) ? prev.filter((v) => v !== id) : [...prev, id]
    );
  };

  const clearFilters = () => {
    setMinAPY([0]);
    setMaxCommission([20]);
    setMinUptime([95]);
    setShowVerifiedOnly(false);
    setRiskLevels(['low', 'medium', 'high']);
  };

  const toggleRiskLevel = (level: string) => {
    setRiskLevels((prev) =>
      prev.includes(level) ? prev.filter((l) => l !== level) : [...prev, level]
    );
  };

  return (
    <div className="min-h-screen bg-gradient-to-b from-background to-muted/20 p-4 md:p-8">
      <div className="max-w-7xl mx-auto space-y-6">
        {/* Header */}
        <div className="flex flex-col md:flex-row justify-between items-start md:items-center gap-4">
          <div>
            <h1 className="text-4xl font-bold tracking-tight">Validator Browser</h1>
            <p className="text-muted-foreground mt-2">
              Discover and compare validators for delegated staking
            </p>
          </div>
          <div className="flex gap-3">
            <Link href="/staking/nominator-dashboard">
              <Button variant="outline">
                <Award className="mr-2 h-4 w-4" />
                My Dashboard
              </Button>
            </Link>
            {selectedValidators.length > 0 && (
              <Link href="/staking/nomination-manager">
                <Button>
                  Nominate ({selectedValidators.length})
                </Button>
              </Link>
            )}
          </div>
        </div>

        {/* Stats Overview */}
        <div className="grid grid-cols-1 md:grid-cols-4 gap-4">
          <Card>
            <CardHeader className="pb-2">
              <CardTitle className="text-sm font-medium">Total Validators</CardTitle>
            </CardHeader>
            <CardContent>
              <div className="text-2xl font-bold">{validators.length}</div>
            </CardContent>
          </Card>
          <Card>
            <CardHeader className="pb-2">
              <CardTitle className="text-sm font-medium">Avg APY</CardTitle>
            </CardHeader>
            <CardContent>
              <div className="text-2xl font-bold text-green-600">
                {(validators.reduce((acc, v) => acc + v.apy, 0) / validators.length).toFixed(1)}%
              </div>
            </CardContent>
          </Card>
          <Card>
            <CardHeader className="pb-2">
              <CardTitle className="text-sm font-medium">Verified</CardTitle>
            </CardHeader>
            <CardContent>
              <div className="text-2xl font-bold">
                {validators.filter((v) => v.isVerified).length}
              </div>
            </CardContent>
          </Card>
          <Card>
            <CardHeader className="pb-2">
              <CardTitle className="text-sm font-medium">Selected</CardTitle>
            </CardHeader>
            <CardContent>
              <div className="text-2xl font-bold">{selectedValidators.length}</div>
            </CardContent>
          </Card>
        </div>

        {/* Search and Filters */}
        <Card>
          <CardContent className="pt-6">
            <div className="flex flex-col md:flex-row gap-4">
              <div className="flex-1 relative">
                <Search className="absolute left-3 top-1/2 -translate-y-1/2 h-4 w-4 text-muted-foreground" />
                <Input
                  placeholder="Search by name or address..."
                  className="pl-10"
                  value={searchQuery}
                  onChange={(e) => setSearchQuery(e.target.value)}
                />
              </div>
              <Select
                value={sortField}
                onValueChange={(value) => setSortField(value as SortField)}
              >
                <SelectTrigger className="w-full md:w-[180px]">
                  <SelectValue placeholder="Sort by" />
                </SelectTrigger>
                <SelectContent>
                  <SelectItem value="apy">APY</SelectItem>
                  <SelectItem value="commission">Commission</SelectItem>
                  <SelectItem value="totalStake">Total Stake</SelectItem>
                  <SelectItem value="uptime">Uptime</SelectItem>
                  <SelectItem value="nominatorCount">Nominators</SelectItem>
                </SelectContent>
              </Select>
              <Button
                variant="outline"
                onClick={() => toggleSort(sortField)}
              >
                {sortDirection === 'asc' ? <TrendingUp className="h-4 w-4" /> : <TrendingDown className="h-4 w-4" />}
              </Button>
              <Sheet>
                <SheetTrigger asChild>
                  <Button variant="outline">
                    <Filter className="mr-2 h-4 w-4" />
                    Filters
                  </Button>
                </SheetTrigger>
                <SheetContent>
                  <SheetHeader>
                    <SheetTitle>Filter Validators</SheetTitle>
                    <SheetDescription>
                      Refine your search with advanced filters
                    </SheetDescription>
                  </SheetHeader>
                  <div className="space-y-6 mt-6">
                    {/* APY Filter */}
                    <div className="space-y-2">
                      <label className="text-sm font-medium">
                        Minimum APY: {minAPY[0]}%
                      </label>
                      <Slider
                        value={minAPY}
                        onValueChange={setMinAPY}
                        max={20}
                        step={0.5}
                      />
                    </div>

                    {/* Commission Filter */}
                    <div className="space-y-2">
                      <label className="text-sm font-medium">
                        Maximum Commission: {maxCommission[0]}%
                      </label>
                      <Slider
                        value={maxCommission}
                        onValueChange={setMaxCommission}
                        max={20}
                        step={1}
                      />
                    </div>

                    {/* Uptime Filter */}
                    <div className="space-y-2">
                      <label className="text-sm font-medium">
                        Minimum Uptime: {minUptime[0]}%
                      </label>
                      <Slider
                        value={minUptime}
                        onValueChange={setMinUptime}
                        min={90}
                        max={100}
                        step={0.1}
                      />
                    </div>

                    {/* Verified Only */}
                    <div className="flex items-center space-x-2">
                      <Checkbox
                        id="verified"
                        checked={showVerifiedOnly}
                        onCheckedChange={(checked) => setShowVerifiedOnly(checked as boolean)}
                      />
                      <label htmlFor="verified" className="text-sm font-medium">
                        Show verified validators only
                      </label>
                    </div>

                    {/* Risk Level */}
                    <div className="space-y-2">
                      <label className="text-sm font-medium">Risk Level</label>
                      <div className="space-y-2">
                        {['low', 'medium', 'high'].map((level) => (
                          <div key={level} className="flex items-center space-x-2">
                            <Checkbox
                              id={level}
                              checked={riskLevels.includes(level)}
                              onCheckedChange={() => toggleRiskLevel(level)}
                            />
                            <label htmlFor={level} className="text-sm capitalize">
                              {level}
                            </label>
                          </div>
                        ))}
                      </div>
                    </div>

                    <Button variant="outline" onClick={clearFilters} className="w-full">
                      <X className="mr-2 h-4 w-4" />
                      Clear Filters
                    </Button>
                  </div>
                </SheetContent>
              </Sheet>
            </div>
          </CardContent>
        </Card>

        {/* Results */}
        <div className="space-y-3">
          <div className="flex justify-between items-center">
            <p className="text-sm text-muted-foreground">
              Showing {filteredValidators.length} of {validators.length} validators
            </p>
            {selectedValidators.length > 0 && (
              <Button
                variant="ghost"
                size="sm"
                onClick={() => setSelectedValidators([])}
              >
                Clear Selection
              </Button>
            )}
          </div>

          {filteredValidators.length === 0 ? (
            <Card>
              <CardContent className="py-12 text-center">
                <p className="text-muted-foreground">
                  No validators found matching your criteria
                </p>
              </CardContent>
            </Card>
          ) : (
            <div className="grid grid-cols-1 gap-4">
              {filteredValidators.map((validator) => (
                <ValidatorCard
                  key={validator.id}
                  validator={validator}
                  isSelected={selectedValidators.includes(validator.id)}
                  onToggleSelect={() => toggleValidatorSelection(validator.id)}
                />
              ))}
            </div>
          )}
        </div>
      </div>
    </div>
  );
}
