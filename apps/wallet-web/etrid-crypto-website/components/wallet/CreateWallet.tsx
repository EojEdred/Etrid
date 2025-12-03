'use client'

import { useState, useEffect } from 'react'
import { Wallet, Copy, Check, Eye, EyeOff, AlertCircle, Download, Shield, Loader2 } from 'lucide-react'
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
import { getWalletService } from '@/lib/wallet/service'
import { generateMnemonic, initCrypto, deriveKeypair } from '@/lib/wallet/crypto'

interface CreateWalletProps {
  open: boolean
  onOpenChange: (open: boolean) => void
  onWalletCreated: (address: string, privateKey: string, mnemonic: string) => void
}

export default function CreateWallet({ open, onOpenChange, onWalletCreated }: CreateWalletProps) {
  const [step, setStep] = useState<'generate' | 'backup' | 'confirm'>('generate')
  const [mnemonic, setMnemonic] = useState<string[]>([])
  const [address, setAddress] = useState('')
  const [privateKey, setPrivateKey] = useState('')
  const [showMnemonic, setShowMnemonic] = useState(false)
  const [copied, setCopied] = useState(false)
  const [confirmed, setConfirmed] = useState(false)
  const [selectedWords, setSelectedWords] = useState<number[]>([])
  const [confirmWords, setConfirmWords] = useState<{ index: number; word: string }[]>([])
  const [isGenerating, setIsGenerating] = useState(false)
  const [cryptoReady, setCryptoReady] = useState(false)

  // Initialize crypto on mount
  useEffect(() => {
    const init = async () => {
      try {
        await initCrypto()
        setCryptoReady(true)
      } catch (error) {
        console.error('Failed to initialize crypto:', error)
      }
    }
    init()
  }, [])

  const generateWallet = async () => {
    if (!cryptoReady) {
      console.error('Crypto not ready')
      return
    }

    setIsGenerating(true)
    try {
      // Generate real BIP39 mnemonic using Polkadot's secure implementation
      const generatedMnemonicPhrase = generateMnemonic(12)
      const mnemonicWords = generatedMnemonicPhrase.split(' ')

      // Derive keypair from mnemonic
      const keypair = deriveKeypair(generatedMnemonicPhrase, '//0', 'sr25519', 42)

      // Convert publicKey to hex for storage (this serves as a reference, not the actual private key)
      const publicKeyHex = Array.from(keypair.publicKey)
        .map(b => b.toString(16).padStart(2, '0'))
        .join('')

      setMnemonic(mnemonicWords)
      setAddress(keypair.address)
      setPrivateKey(publicKeyHex) // Note: actual private key is derived from mnemonic

      // Select random words for confirmation
      const randomIndices: number[] = []
      while (randomIndices.length < 3) {
        const idx = Math.floor(Math.random() * 12)
        if (!randomIndices.includes(idx)) randomIndices.push(idx)
      }
      setSelectedWords(randomIndices.sort((a, b) => a - b))
      setConfirmWords(randomIndices.map(idx => ({ index: idx, word: '' })))

      setStep('backup')
    } catch (error) {
      console.error('Failed to generate wallet:', error)
    } finally {
      setIsGenerating(false)
    }
  }

  const copyMnemonic = () => {
    navigator.clipboard.writeText(mnemonic.join(' '))
    setCopied(true)
    setTimeout(() => setCopied(false), 2000)
  }

  const downloadMnemonic = () => {
    const element = document.createElement('a')
    const file = new Blob([mnemonic.join(' ')], { type: 'text/plain' })
    element.href = URL.createObjectURL(file)
    element.download = 'etrid-wallet-backup.txt'
    document.body.appendChild(element)
    element.click()
    document.body.removeChild(element)
  }

  const handleConfirmWordChange = (confirmIndex: number, value: string) => {
    const newConfirmWords = [...confirmWords]
    newConfirmWords[confirmIndex].word = value.trim().toLowerCase()
    setConfirmWords(newConfirmWords)
  }

  const verifyWords = () => {
    const allCorrect = confirmWords.every(({ index, word }) => {
      return word === mnemonic[index].toLowerCase()
    })

    if (allCorrect) {
      setConfirmed(true)
      onWalletCreated(address, privateKey, mnemonic.join(' '))
      onOpenChange(false)
      resetState()
    } else {
      alert('The words you entered do not match. Please try again.')
    }
  }

  const resetState = () => {
    setStep('generate')
    setMnemonic([])
    setAddress('')
    setPrivateKey('')
    setShowMnemonic(false)
    setCopied(false)
    setConfirmed(false)
    setSelectedWords([])
    setConfirmWords([])
  }

  return (
    <Dialog open={open} onOpenChange={onOpenChange}>
      <DialogContent className="glass-card border-white/20 max-w-2xl">
        {step === 'generate' && (
          <>
            <DialogHeader>
              <DialogTitle className="flex items-center gap-2 text-2xl gradient-text">
                <Wallet className="w-6 h-6" />
                Create New Wallet
              </DialogTitle>
              <DialogDescription className="text-white/60">
                Generate a secure new ETRID wallet. Your recovery phrase will be shown only once.
              </DialogDescription>
            </DialogHeader>

            <div className="space-y-6 py-6">
              <Card className="glass border-yellow-500/30 p-6">
                <div className="flex items-start gap-4">
                  <AlertCircle className="w-5 h-5 text-yellow-500 flex-shrink-0 mt-1" />
                  <div className="space-y-2">
                    <h3 className="font-semibold text-yellow-500">Important Security Notice</h3>
                    <ul className="text-sm text-white/80 space-y-1 list-disc list-inside">
                      <li>Your recovery phrase is the ONLY way to restore your wallet</li>
                      <li>Never share your recovery phrase with anyone</li>
                      <li>Store it securely offline in multiple locations</li>
                      <li>ETRID will never ask for your recovery phrase</li>
                    </ul>
                  </div>
                </div>
              </Card>

              <div className="flex items-center gap-4 p-4 glass rounded-lg">
                <Shield className="w-8 h-8 text-blue-400" />
                <div>
                  <h4 className="font-semibold">256-bit Encryption</h4>
                  <p className="text-sm text-white/60">Military-grade security for your assets</p>
                </div>
              </div>
            </div>

            <DialogFooter>
              <Button
                variant="outline"
                onClick={() => onOpenChange(false)}
                className="glass border-white/20"
                disabled={isGenerating}
              >
                Cancel
              </Button>
              <Button
                onClick={generateWallet}
                className="btn-primary"
                disabled={isGenerating || !cryptoReady}
              >
                {isGenerating ? (
                  <>
                    <Loader2 className="w-4 h-4 mr-2 animate-spin" />
                    Generating...
                  </>
                ) : !cryptoReady ? (
                  <>
                    <Loader2 className="w-4 h-4 mr-2 animate-spin" />
                    Initializing...
                  </>
                ) : (
                  'Generate Wallet'
                )}
              </Button>
            </DialogFooter>
          </>
        )}

        {step === 'backup' && (
          <>
            <DialogHeader>
              <DialogTitle className="flex items-center gap-2 text-2xl gradient-text">
                <Shield className="w-6 h-6" />
                Backup Recovery Phrase
              </DialogTitle>
              <DialogDescription className="text-white/60">
                Write down these 12 words in order and store them safely. You'll need to confirm them next.
              </DialogDescription>
            </DialogHeader>

            <div className="space-y-6 py-6">
              <Card className="glass border-red-500/30 p-4">
                <div className="flex items-start gap-3">
                  <AlertCircle className="w-5 h-5 text-red-500 flex-shrink-0" />
                  <p className="text-sm text-white/80">
                    Never share this phrase. Anyone with these words can access your wallet and funds.
                  </p>
                </div>
              </Card>

              <div className="relative">
                <div className={`grid grid-cols-3 gap-3 p-6 glass rounded-lg ${!showMnemonic ? 'blur-sm' : ''}`}>
                  {mnemonic.map((word, index) => (
                    <div
                      key={index}
                      className="glass-card p-3 rounded-lg flex items-center gap-2"
                    >
                      <span className="text-white/40 text-sm font-mono">{index + 1}.</span>
                      <span className="font-medium font-mono">{word}</span>
                    </div>
                  ))}
                </div>

                {!showMnemonic && (
                  <div className="absolute inset-0 flex items-center justify-center">
                    <Button
                      onClick={() => setShowMnemonic(true)}
                      className="btn-primary gap-2"
                    >
                      <Eye className="w-4 h-4" />
                      Reveal Recovery Phrase
                    </Button>
                  </div>
                )}
              </div>

              {showMnemonic && (
                <div className="flex gap-2">
                  <Button
                    onClick={copyMnemonic}
                    variant="outline"
                    className="glass border-white/20 flex-1"
                  >
                    {copied ? (
                      <>
                        <Check className="w-4 h-4 mr-2" />
                        Copied!
                      </>
                    ) : (
                      <>
                        <Copy className="w-4 h-4 mr-2" />
                        Copy to Clipboard
                      </>
                    )}
                  </Button>
                  <Button
                    onClick={downloadMnemonic}
                    variant="outline"
                    className="glass border-white/20 flex-1"
                  >
                    <Download className="w-4 h-4 mr-2" />
                    Download Backup
                  </Button>
                </div>
              )}
            </div>

            <DialogFooter>
              <Button
                variant="outline"
                onClick={() => setStep('generate')}
                className="glass border-white/20"
              >
                Back
              </Button>
              <Button
                onClick={() => setStep('confirm')}
                disabled={!showMnemonic}
                className="btn-primary"
              >
                I've Backed It Up
              </Button>
            </DialogFooter>
          </>
        )}

        {step === 'confirm' && (
          <>
            <DialogHeader>
              <DialogTitle className="flex items-center gap-2 text-2xl gradient-text">
                <Shield className="w-6 h-6" />
                Confirm Recovery Phrase
              </DialogTitle>
              <DialogDescription className="text-white/60">
                Enter the following words from your recovery phrase to confirm you've saved it correctly.
              </DialogDescription>
            </DialogHeader>

            <div className="space-y-6 py-6">
              <div className="space-y-4">
                {confirmWords.map((item, confirmIndex) => (
                  <div key={confirmIndex} className="space-y-2">
                    <label className="text-sm font-medium">
                      Word #{item.index + 1}
                    </label>
                    <input
                      type="text"
                      value={item.word}
                      onChange={(e) => handleConfirmWordChange(confirmIndex, e.target.value)}
                      placeholder={`Enter word #${item.index + 1}`}
                      className="w-full px-4 py-3 glass rounded-lg border border-white/20 bg-transparent text-white placeholder:text-white/40 focus:outline-none focus:border-blue-500/50 focus:ring-2 focus:ring-blue-500/20"
                    />
                  </div>
                ))}
              </div>

              <Card className="glass border-blue-500/30 p-4">
                <p className="text-sm text-white/80">
                  Make sure you've saved your recovery phrase securely before continuing. This is your last chance to verify it.
                </p>
              </Card>
            </div>

            <DialogFooter>
              <Button
                variant="outline"
                onClick={() => setStep('backup')}
                className="glass border-white/20"
              >
                Back
              </Button>
              <Button
                onClick={verifyWords}
                className="btn-primary"
              >
                Confirm & Create Wallet
              </Button>
            </DialogFooter>
          </>
        )}
      </DialogContent>
    </Dialog>
  )
}
