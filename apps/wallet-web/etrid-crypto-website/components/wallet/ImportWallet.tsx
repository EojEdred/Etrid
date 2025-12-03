'use client'

import { useState } from 'react'
import { Wallet, AlertCircle, Upload, Key, FileText } from 'lucide-react'
import { Button } from '@/components/ui/button'
import {
  Dialog,
  DialogContent,
  DialogDescription,
  DialogFooter,
  DialogHeader,
  DialogTitle,
} from '@/components/ui/dialog'
import { Card } from '@/components/ui/card'
import { Textarea } from '@/components/ui/textarea'
import { Input } from '@/components/ui/input'

interface ImportWalletProps {
  open: boolean
  onOpenChange: (open: boolean) => void
  onWalletImported: (address: string, privateKey: string, mnemonic?: string) => void
}

type ImportMethod = 'mnemonic' | 'privateKey'

export default function ImportWallet({ open, onOpenChange, onWalletImported }: ImportWalletProps) {
  const [importMethod, setImportMethod] = useState<ImportMethod>('mnemonic')
  const [mnemonic, setMnemonic] = useState('')
  const [privateKey, setPrivateKey] = useState('')
  const [isImporting, setIsImporting] = useState(false)
  const [error, setError] = useState('')

  const validateMnemonic = (phrase: string): boolean => {
    const words = phrase.trim().toLowerCase().split(/\s+/)
    return words.length === 12 || words.length === 24
  }

  const validatePrivateKey = (key: string): boolean => {
    const cleanKey = key.trim()
    // Check if it's a valid hex string (with or without 0x prefix)
    const hexPattern = /^(0x)?[0-9a-fA-F]{64}$/
    return hexPattern.test(cleanKey)
  }

  const handleImportMnemonic = async () => {
    setError('')

    if (!validateMnemonic(mnemonic)) {
      setError('Invalid recovery phrase. Please enter a valid 12 or 24 word phrase.')
      return
    }

    setIsImporting(true)

    try {
      // This will be replaced with actual wallet import from lib/wallet/service.ts
      // For now, generating placeholder values
      await new Promise(resolve => setTimeout(resolve, 1000))

      const importedAddress = '0x' + Array.from({ length: 40 }, () => Math.floor(Math.random() * 16).toString(16)).join('')
      const importedPrivateKey = '0x' + Array.from({ length: 64 }, () => Math.floor(Math.random() * 16).toString(16)).join('')

      onWalletImported(importedAddress, importedPrivateKey, mnemonic)
      onOpenChange(false)
      resetState()
    } catch (err) {
      setError('Failed to import wallet. Please check your recovery phrase and try again.')
      console.error('Import error:', err)
    } finally {
      setIsImporting(false)
    }
  }

  const handleImportPrivateKey = async () => {
    setError('')

    if (!validatePrivateKey(privateKey)) {
      setError('Invalid private key. Please enter a valid 64-character hexadecimal private key.')
      return
    }

    setIsImporting(true)

    try {
      // This will be replaced with actual wallet import from lib/wallet/service.ts
      await new Promise(resolve => setTimeout(resolve, 1000))

      const cleanKey = privateKey.trim().startsWith('0x') ? privateKey.trim() : '0x' + privateKey.trim()
      const importedAddress = '0x' + Array.from({ length: 40 }, () => Math.floor(Math.random() * 16).toString(16)).join('')

      onWalletImported(importedAddress, cleanKey)
      onOpenChange(false)
      resetState()
    } catch (err) {
      setError('Failed to import wallet. Please check your private key and try again.')
      console.error('Import error:', err)
    } finally {
      setIsImporting(false)
    }
  }

  const handleImport = () => {
    if (importMethod === 'mnemonic') {
      handleImportMnemonic()
    } else {
      handleImportPrivateKey()
    }
  }

  const resetState = () => {
    setMnemonic('')
    setPrivateKey('')
    setError('')
    setImportMethod('mnemonic')
  }

  const handlePaste = async () => {
    try {
      const text = await navigator.clipboard.readText()
      if (importMethod === 'mnemonic') {
        setMnemonic(text)
      } else {
        setPrivateKey(text)
      }
    } catch (err) {
      console.error('Failed to paste:', err)
    }
  }

  return (
    <Dialog open={open} onOpenChange={onOpenChange}>
      <DialogContent className="glass-card border-white/20 max-w-2xl">
        <DialogHeader>
          <DialogTitle className="flex items-center gap-2 text-2xl gradient-text">
            <Upload className="w-6 h-6" />
            Import Wallet
          </DialogTitle>
          <DialogDescription className="text-white/60">
            Import an existing wallet using your recovery phrase or private key.
          </DialogDescription>
        </DialogHeader>

        <div className="space-y-6 py-6">
          {/* Import Method Selection */}
          <div className="grid grid-cols-2 gap-3">
            <button
              onClick={() => setImportMethod('mnemonic')}
              className={`p-4 rounded-lg border-2 transition-all ${
                importMethod === 'mnemonic'
                  ? 'border-blue-500 glass-card'
                  : 'border-white/10 glass hover:border-white/20'
              }`}
            >
              <FileText className="w-6 h-6 mx-auto mb-2 text-blue-400" />
              <div className="font-semibold text-sm">Recovery Phrase</div>
              <div className="text-xs text-white/60 mt-1">12 or 24 words</div>
            </button>

            <button
              onClick={() => setImportMethod('privateKey')}
              className={`p-4 rounded-lg border-2 transition-all ${
                importMethod === 'privateKey'
                  ? 'border-purple-500 glass-card'
                  : 'border-white/10 glass hover:border-white/20'
              }`}
            >
              <Key className="w-6 h-6 mx-auto mb-2 text-purple-400" />
              <div className="font-semibold text-sm">Private Key</div>
              <div className="text-xs text-white/60 mt-1">Hexadecimal string</div>
            </button>
          </div>

          {/* Security Warning */}
          <Card className="glass border-yellow-500/30 p-4">
            <div className="flex items-start gap-3">
              <AlertCircle className="w-5 h-5 text-yellow-500 flex-shrink-0" />
              <div className="space-y-1">
                <p className="text-sm font-semibold text-yellow-500">Security Warning</p>
                <p className="text-sm text-white/80">
                  Never share your recovery phrase or private key. Only import on trusted devices.
                  ETRID will never ask for this information.
                </p>
              </div>
            </div>
          </Card>

          {/* Import Input */}
          <div className="space-y-3">
            {importMethod === 'mnemonic' ? (
              <>
                <div className="flex items-center justify-between">
                  <label className="text-sm font-medium">Recovery Phrase</label>
                  <Button
                    type="button"
                    variant="ghost"
                    size="sm"
                    onClick={handlePaste}
                    className="text-blue-400 hover:text-blue-300"
                  >
                    Paste
                  </Button>
                </div>
                <Textarea
                  value={mnemonic}
                  onChange={(e) => setMnemonic(e.target.value)}
                  placeholder="Enter your 12 or 24 word recovery phrase separated by spaces"
                  className="min-h-32 font-mono glass border-white/20 bg-transparent text-white placeholder:text-white/40 focus:border-blue-500/50 focus:ring-2 focus:ring-blue-500/20"
                  rows={4}
                />
                <p className="text-xs text-white/40">
                  Enter each word separated by a space. Words should be lowercase.
                </p>
              </>
            ) : (
              <>
                <div className="flex items-center justify-between">
                  <label className="text-sm font-medium">Private Key</label>
                  <Button
                    type="button"
                    variant="ghost"
                    size="sm"
                    onClick={handlePaste}
                    className="text-purple-400 hover:text-purple-300"
                  >
                    Paste
                  </Button>
                </div>
                <Input
                  type="password"
                  value={privateKey}
                  onChange={(e) => setPrivateKey(e.target.value)}
                  placeholder="0x... (64 character hexadecimal string)"
                  className="font-mono glass border-white/20 bg-transparent text-white placeholder:text-white/40"
                />
                <p className="text-xs text-white/40">
                  Enter your private key with or without the 0x prefix.
                </p>
              </>
            )}
          </div>

          {/* Error Message */}
          {error && (
            <Card className="glass border-red-500/30 p-4">
              <div className="flex items-start gap-3">
                <AlertCircle className="w-5 h-5 text-red-500 flex-shrink-0" />
                <p className="text-sm text-red-400">{error}</p>
              </div>
            </Card>
          )}
        </div>

        <DialogFooter>
          <Button
            variant="outline"
            onClick={() => {
              onOpenChange(false)
              resetState()
            }}
            disabled={isImporting}
            className="glass border-white/20"
          >
            Cancel
          </Button>
          <Button
            onClick={handleImport}
            disabled={
              isImporting ||
              (importMethod === 'mnemonic' ? !mnemonic.trim() : !privateKey.trim())
            }
            className="btn-primary"
          >
            {isImporting ? (
              <>
                <div className="w-4 h-4 border-2 border-white/30 border-t-white rounded-full animate-spin mr-2" />
                Importing...
              </>
            ) : (
              <>
                <Wallet className="w-4 h-4 mr-2" />
                Import Wallet
              </>
            )}
          </Button>
        </DialogFooter>
      </DialogContent>
    </Dialog>
  )
}
