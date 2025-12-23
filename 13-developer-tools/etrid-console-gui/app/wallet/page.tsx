'use client';

import React, { useState } from 'react';
import { useRouter } from 'next/navigation';
import { useWallet } from '@/contexts/WalletContext';
import { Button } from '@/components/ui/button';
import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card';
import { Input } from '@/components/ui/input';
import { Label } from '@/components/ui/label';
import { Tabs, TabsContent, TabsList, TabsTrigger } from '@/components/ui/tabs';
import { Wallet, Key, Upload, Copy, Eye, EyeOff, Loader2 } from 'lucide-react';
import { useToast } from '@/hooks/use-toast';

export default function WalletPage() {
  const router = useRouter();
  const { createNewWallet, importExistingWallet, status, account } = useWallet();
  const { toast } = useToast();
  
  const [activeTab, setActiveTab] = useState('create');
  const [walletName, setWalletName] = useState('');
  const [password, setPassword] = useState('');
  const [mnemonic, setMnemonic] = useState('');
  const [isSubmitting, setIsSubmitting] = useState(false);
  const [showPassword, setShowPassword] = useState(false);
  const [createdMnemonic, setCreatedMnemonic] = useState<string | null>(null);

  // If already unlocked and has account, redirect to dashboard or show wallet details
  if (status === 'unlocked' && account && !createdMnemonic) {
    return (
      <div className="container mx-auto px-4 py-8 max-w-2xl">
        <Card className="glass-card">
          <CardHeader>
            <CardTitle className="flex items-center gap-2">
              <Wallet className="w-6 h-6 text-green-400" />
              Wallet Active
            </CardTitle>
          </CardHeader>
          <CardContent className="space-y-6">
            <div className="p-4 bg-white/5 rounded-lg border border-white/10">
              <Label className="text-white/60 mb-1 block">Wallet Name</Label>
              <p className="text-xl font-medium">{account.name}</p>
            </div>
            
            <div className="p-4 bg-white/5 rounded-lg border border-white/10">
              <Label className="text-white/60 mb-1 block">Address</Label>
              <div className="flex items-center gap-2">
                <p className="font-mono text-sm break-all">{account.address}</p>
                <Button variant="ghost" size="icon" className="h-8 w-8" onClick={() => {
                  navigator.clipboard.writeText(account.address);
                  toast({ title: 'Copied', description: 'Address copied to clipboard' });
                }}>
                  <Copy className="w-4 h-4" />
                </Button>
              </div>
            </div>

            <div className="flex gap-4">
              <Button onClick={() => router.push('/')} className="flex-1">
                Go to Dashboard
              </Button>
            </div>
          </CardContent>
        </Card>
      </div>
    );
  }

  const handleCreate = async (e: React.FormEvent) => {
    e.preventDefault();
    if (!walletName || !password) return;
    
    setIsSubmitting(true);
    try {
      const result = await createNewWallet(password, walletName);
      setCreatedMnemonic(result.mnemonic);
      toast({
        title: "Wallet Created",
        description: "Your new wallet has been successfully created.",
      });
    } catch (error) {
      toast({
        title: "Error",
        description: error instanceof Error ? error.message : "Failed to create wallet",
        variant: "destructive",
      });
    } finally {
      setIsSubmitting(false);
    }
  };

  const handleImport = async (e: React.FormEvent) => {
    e.preventDefault();
    if (!walletName || !password || !mnemonic) return;

    setIsSubmitting(true);
    try {
      await importExistingWallet(mnemonic, password, walletName);
      toast({
        title: "Wallet Imported",
        description: "Your wallet has been successfully imported.",
      });
      router.push('/');
    } catch (error) {
      toast({
        title: "Error",
        description: error instanceof Error ? error.message : "Failed to import wallet",
        variant: "destructive",
      });
    } finally {
      setIsSubmitting(false);
    }
  };

  if (createdMnemonic) {
    return (
      <div className="container mx-auto px-4 py-8 max-w-2xl">
        <Card className="glass-card border-green-500/30">
          <CardHeader>
            <CardTitle className="text-green-400">Backup Your Recovery Phrase</CardTitle>
          </CardHeader>
          <CardContent className="space-y-6">
            <div className="bg-yellow-500/10 border border-yellow-500/20 p-4 rounded-lg text-yellow-200 text-sm">
              Important: Write down these 12 words in order and keep them safe. 
              If you lose this phrase, you will lose access to your funds forever.
            </div>

            <div className="grid grid-cols-3 gap-3 p-6 bg-black/40 rounded-xl border border-white/10">
              {createdMnemonic.split(' ').map((word, i) => (
                <div key={i} className="flex items-center gap-2">
                  <span className="text-white/30 text-xs w-4">{i + 1}.</span>
                  <span className="font-mono font-medium">{word}</span>
                </div>
              ))}
            </div>

            <Button 
              onClick={() => {
                setCreatedMnemonic(null);
                router.push('/');
              }} 
              className="w-full btn-primary"
            >
              I Have Saved My Recovery Phrase
            </Button>
          </CardContent>
        </Card>
      </div>
    );
  }

  return (
    <div className="container mx-auto px-4 py-8 max-w-md">
      <div className="mb-8 text-center">
        <h1 className="text-3xl font-bold gradient-text mb-2">Connect Wallet</h1>
        <p className="text-white/60">Create a new wallet or import an existing one</p>
      </div>

      <Tabs value={activeTab} onValueChange={setActiveTab} className="space-y-6">
        <TabsList className="w-full glass p-1">
          <TabsTrigger value="create" className="flex-1">Create New</TabsTrigger>
          <TabsTrigger value="import" className="flex-1">Import Existing</TabsTrigger>
        </TabsList>

        <TabsContent value="create">
          <Card className="glass-card">
            <CardContent className="pt-6">
              <form onSubmit={handleCreate} className="space-y-4">
                <div className="space-y-2">
                  <Label htmlFor="create-name">Wallet Name</Label>
                  <Input 
                    id="create-name" 
                    placeholder="My Main Wallet" 
                    value={walletName}
                    onChange={(e) => setWalletName(e.target.value)}
                    required
                  />
                </div>
                
                <div className="space-y-2">
                  <Label htmlFor="create-password">Set Password</Label>
                  <div className="relative">
                    <Input 
                      id="create-password" 
                      type={showPassword ? "text" : "password"}
                      placeholder="Strong password" 
                      value={password}
                      onChange={(e) => setPassword(e.target.value)}
                      required
                      minLength={8}
                    />
                    <button 
                      type="button"
                      className="absolute right-3 top-1/2 -translate-y-1/2 text-white/50 hover:text-white"
                      onClick={() => setShowPassword(!showPassword)}
                    >
                      {showPassword ? <EyeOff size={16} /> : <Eye size={16} />}
                    </button>
                  </div>
                </div>

                <Button type="submit" className="w-full btn-primary" disabled={isSubmitting}>
                  {isSubmitting ? (
                    <>
                      <Loader2 className="mr-2 h-4 w-4 animate-spin" />
                      Creating...
                    </>
                  ) : (
                    <>
                      <Wallet className="mr-2 h-4 w-4" />
                      Create Wallet
                    </>
                  )}
                </Button>
              </form>
            </CardContent>
          </Card>
        </TabsContent>

        <TabsContent value="import">
          <Card className="glass-card">
            <CardContent className="pt-6">
              <form onSubmit={handleImport} className="space-y-4">
                <div className="space-y-2">
                  <Label htmlFor="import-name">Wallet Name</Label>
                  <Input 
                    id="import-name" 
                    placeholder="Imported Wallet" 
                    value={walletName}
                    onChange={(e) => setWalletName(e.target.value)}
                    required
                  />
                </div>

                <div className="space-y-2">
                  <Label htmlFor="mnemonic">Recovery Phrase (12 words)</Label>
                  <textarea 
                    id="mnemonic"
                    className="flex min-h-[80px] w-full rounded-md border border-input bg-background px-3 py-2 text-sm ring-offset-background placeholder:text-muted-foreground focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:cursor-not-allowed disabled:opacity-50 glass-input"
                    placeholder="enter your secret twelve words here..."
                    value={mnemonic}
                    onChange={(e) => setMnemonic(e.target.value)}
                    required
                  />
                </div>
                
                <div className="space-y-2">
                  <Label htmlFor="import-password">Set Password</Label>
                  <Input 
                    id="import-password" 
                    type="password" 
                    placeholder="New password for this device" 
                    value={password}
                    onChange={(e) => setPassword(e.target.value)}
                    required
                    minLength={8}
                  />
                </div>

                <Button type="submit" className="w-full btn-primary" disabled={isSubmitting}>
                  {isSubmitting ? (
                    <>
                      <Loader2 className="mr-2 h-4 w-4 animate-spin" />
                      Importing...
                    </>
                  ) : (
                    <>
                      <Upload className="mr-2 h-4 w-4" />
                      Import Wallet
                    </>
                  )}
                </Button>
              </form>
            </CardContent>
          </Card>
        </TabsContent>
      </Tabs>
    </div>
  );
}
