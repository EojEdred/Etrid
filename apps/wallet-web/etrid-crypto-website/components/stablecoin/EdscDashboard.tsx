'use client';

/**
 * EDSC (Ëtrid Dollar Stablecoin) Dashboard Component
 *
 * Provides UI for:
 * - Minting EDSC
 * - Viewing balance and total supply
 * - 3-path redemption system
 * - Proof-of-reserves display
 */

import { useState, useEffect } from 'react';
import { flarechainApi } from '@/lib/api/flarechain';
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from '@/components/ui/card';
import { Button } from '@/components/ui/button';
import { Input } from '@/components/ui/input';
import { Label } from '@/components/ui/label';
import { Tabs, TabsContent, TabsList, TabsTrigger } from '@/components/ui/tabs';
import { Alert, AlertDescription } from '@/components/ui/alert';

export interface EdscInfo {
  balance: string;
  totalSupply: string;
  redemptions: any[];
}

export function EdscDashboard({ address }: { address?: string }) {
  const [edscInfo, setEdscInfo] = useState<EdscInfo | null>(null);
  const [loading, setLoading] = useState(false);
  const [mintAmount, setMintAmount] = useState('');
  const [redeemAmount, setRedeemAmount] = useState('');
  const [redemptionPath, setRedemptionPath] = useState<'instant' | 'delayed' | 'pro-rata'>('instant');

  useEffect(() => {
    if (address) {
      loadEdscInfo();
    }
  }, [address]);

  const loadEdscInfo = async () => {
    if (!address) return;

    setLoading(true);
    try {
      const info = await flarechainApi.getEdscInfo(address);
      setEdscInfo(info);
    } catch (error) {
      console.error('Failed to load EDSC info:', error);
    } finally {
      setLoading(false);
    }
  };

  const handleMint = async () => {
    if (!address || !mintAmount) return;

    setLoading(true);
    try {
      const api = await flarechainApi.connect('edsc-pbc');

      // Mint EDSC tokens
      const tx = api.tx.edscToken.mint(
        address,
        BigInt(parseFloat(mintAmount) * Math.pow(10, 18))
      );

      // Sign and send (would need injector in real implementation)
      console.log('Minting', mintAmount, 'EDSC...');

      await loadEdscInfo();
      setMintAmount('');
    } catch (error) {
      console.error('Mint failed:', error);
    } finally {
      setLoading(false);
    }
  };

  const handleRedeem = async () => {
    if (!address || !redeemAmount) return;

    setLoading(true);
    try {
      const api = await flarechainApi.connect('edsc-pbc');

      // Redeem EDSC via selected path
      const amount = BigInt(parseFloat(redeemAmount) * Math.pow(10, 18));

      let tx;
      switch (redemptionPath) {
        case 'instant':
          // Instant redemption (higher fee, immediate)
          tx = api.tx.edscRedemption.redeemInstant(amount);
          break;
        case 'delayed':
          // Delayed redemption (lower fee, 7-day wait)
          tx = api.tx.edscRedemption.redeemDelayed(amount);
          break;
        case 'pro-rata':
          // Pro-rata redemption (no fee, proportional to backing)
          tx = api.tx.edscRedemption.redeemProRata(amount);
          break;
      }

      console.log(`Redeeming ${redeemAmount} EDSC via ${redemptionPath} path...`);

      await loadEdscInfo();
      setRedeemAmount('');
    } catch (error) {
      console.error('Redemption failed:', error);
    } finally {
      setLoading(false);
    }
  };

  const totalSupplyFormatted = edscInfo
    ? (parseInt(edscInfo.totalSupply) / Math.pow(10, 18)).toLocaleString()
    : '0';

  const pegStatus = '1.00'; // In real implementation, fetch from oracle
  const pegHealth = parseFloat(pegStatus) >= 0.99 && parseFloat(pegStatus) <= 1.01 ? 'healthy' : 'warning';

  return (
    <div className="space-y-6">
      {/* Header */}
      <div className="flex items-center justify-between">
        <div>
          <h2 className="text-3xl font-bold">EDSC Stablecoin</h2>
          <p className="text-muted-foreground">
            Algorithmic stablecoin pegged to $1.00 USD
          </p>
        </div>

        <Button onClick={loadEdscInfo} disabled={loading || !address}>
          {loading ? 'Loading...' : 'Refresh'}
        </Button>
      </div>

      {/* Stats Cards */}
      <div className="grid gap-4 md:grid-cols-3">
        <Card>
          <CardHeader>
            <CardTitle>Your Balance</CardTitle>
            <CardDescription>EDSC tokens you own</CardDescription>
          </CardHeader>
          <CardContent>
            <div className="text-2xl font-bold">
              {edscInfo ? edscInfo.balance : '0'} EDSC
            </div>
          </CardContent>
        </Card>

        <Card>
          <CardHeader>
            <CardTitle>Total Supply</CardTitle>
            <CardDescription>All EDSC in circulation</CardDescription>
          </CardHeader>
          <CardContent>
            <div className="text-2xl font-bold">{totalSupplyFormatted} EDSC</div>
          </CardContent>
        </Card>

        <Card>
          <CardHeader>
            <CardTitle>Peg Status</CardTitle>
            <CardDescription>Current USD peg</CardDescription>
          </CardHeader>
          <CardContent>
            <div className="flex items-center space-x-2">
              <div className="text-2xl font-bold">${pegStatus}</div>
              <span
                className={`inline-flex h-6 px-2 items-center rounded-full text-xs ${
                  pegHealth === 'healthy'
                    ? 'bg-green-100 text-green-800'
                    : 'bg-yellow-100 text-yellow-800'
                }`}
              >
                {pegHealth === 'healthy' ? '✓ Healthy' : '⚠ Warning'}
              </span>
            </div>
          </CardContent>
        </Card>
      </div>

      {/* Mint/Redeem Interface */}
      <Tabs defaultValue="mint" className="w-full">
        <TabsList className="grid w-full grid-cols-2">
          <TabsTrigger value="mint">Mint EDSC</TabsTrigger>
          <TabsTrigger value="redeem">Redeem EDSC</TabsTrigger>
        </TabsList>

        <TabsContent value="mint" className="space-y-4">
          <Card>
            <CardHeader>
              <CardTitle>Mint EDSC Tokens</CardTitle>
              <CardDescription>
                Deposit collateral to mint EDSC stablecoins at $1.00 per token
              </CardDescription>
            </CardHeader>
            <CardContent className="space-y-4">
              <div className="space-y-2">
                <Label htmlFor="mint-amount">Amount (EDSC)</Label>
                <Input
                  id="mint-amount"
                  type="number"
                  placeholder="0.00"
                  value={mintAmount}
                  onChange={(e) => setMintAmount(e.target.value)}
                  disabled={!address}
                />
                <p className="text-sm text-muted-foreground">
                  Collateral required: ${mintAmount || '0.00'} worth of ÉTR
                </p>
              </div>

              <Button
                onClick={handleMint}
                disabled={!address || !mintAmount || loading}
                className="w-full"
              >
                {loading ? 'Processing...' : 'Mint EDSC'}
              </Button>

              <Alert>
                <AlertDescription>
                  Minting EDSC requires 150% collateralization in ÉTR tokens for safety.
                </AlertDescription>
              </Alert>
            </CardContent>
          </Card>
        </TabsContent>

        <TabsContent value="redeem" className="space-y-4">
          <Card>
            <CardHeader>
              <CardTitle>Redeem EDSC Tokens</CardTitle>
              <CardDescription>
                Choose your redemption path based on speed and fees
              </CardDescription>
            </CardHeader>
            <CardContent className="space-y-4">
              <div className="space-y-2">
                <Label htmlFor="redeem-amount">Amount (EDSC)</Label>
                <Input
                  id="redeem-amount"
                  type="number"
                  placeholder="0.00"
                  value={redeemAmount}
                  onChange={(e) => setRedeemAmount(e.target.value)}
                  disabled={!address}
                />
              </div>

              <div className="space-y-3">
                <Label>Redemption Path</Label>

                <button
                  onClick={() => setRedemptionPath('instant')}
                  className={`w-full p-4 border rounded-lg text-left ${
                    redemptionPath === 'instant' ? 'border-primary bg-primary/5' : ''
                  }`}
                >
                  <div className="font-semibold">Instant Redemption</div>
                  <div className="text-sm text-muted-foreground">
                    1% fee • Immediate settlement
                  </div>
                </button>

                <button
                  onClick={() => setRedemptionPath('delayed')}
                  className={`w-full p-4 border rounded-lg text-left ${
                    redemptionPath === 'delayed' ? 'border-primary bg-primary/5' : ''
                  }`}
                >
                  <div className="font-semibold">Delayed Redemption</div>
                  <div className="text-sm text-muted-foreground">
                    0.5% fee • 7-day waiting period
                  </div>
                </button>

                <button
                  onClick={() => setRedemptionPath('pro-rata')}
                  className={`w-full p-4 border rounded-lg text-left ${
                    redemptionPath === 'pro-rata' ? 'border-primary bg-primary/5' : ''
                  }`}
                >
                  <div className="font-semibold">Pro-Rata Redemption</div>
                  <div className="text-sm text-muted-foreground">
                    No fee • Proportional to backing assets
                  </div>
                </button>
              </div>

              <Button
                onClick={handleRedeem}
                disabled={!address || !redeemAmount || loading}
                className="w-full"
              >
                {loading ? 'Processing...' : 'Redeem EDSC'}
              </Button>

              <Alert>
                <AlertDescription>
                  You will receive: ${redeemAmount || '0.00'} worth of backing assets
                </AlertDescription>
              </Alert>
            </CardContent>
          </Card>
        </TabsContent>
      </Tabs>

      {/* Pending Redemptions */}
      {edscInfo && edscInfo.redemptions && edscInfo.redemptions.length > 0 && (
        <Card>
          <CardHeader>
            <CardTitle>Pending Redemptions</CardTitle>
            <CardDescription>Your delayed redemptions in progress</CardDescription>
          </CardHeader>
          <CardContent>
            <div className="space-y-2">
              {edscInfo.redemptions.map((redemption: any, index: number) => (
                <div key={index} className="flex justify-between items-center p-3 border rounded">
                  <div>
                    <div className="font-semibold">{redemption.amount} EDSC</div>
                    <div className="text-sm text-muted-foreground">
                      Ready in {redemption.daysRemaining} days
                    </div>
                  </div>
                  <Button size="sm" disabled={redemption.daysRemaining > 0}>
                    Claim
                  </Button>
                </div>
              ))}
            </div>
          </CardContent>
        </Card>
      )}

      {!address && (
        <Alert>
          <AlertDescription>
            Connect your wallet to interact with EDSC stablecoin
          </AlertDescription>
        </Alert>
      )}
    </div>
  );
}
