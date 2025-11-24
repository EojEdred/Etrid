import { useState, useEffect, useCallback } from 'react'
import { web3Accounts, web3Enable, web3FromAddress } from '@polkadot/extension-dapp'
import type { InjectedAccountWithMeta } from '@polkadot/extension-inject/types'

export interface UseWalletReturn {
  accounts: InjectedAccountWithMeta[]
  selectedAccount: InjectedAccountWithMeta | null
  isConnected: boolean
  isLoading: boolean
  error: Error | null
  connect: () => Promise<void>
  disconnect: () => void
  selectAccount: (account: InjectedAccountWithMeta) => void
}

/**
 * Hook to manage Polkadot.js wallet connection
 * @param appName Application name for wallet authorization
 * @returns Wallet state and methods
 */
export function useWallet(appName: string = 'Etrid Portal'): UseWalletReturn {
  const [accounts, setAccounts] = useState<InjectedAccountWithMeta[]>([])
  const [selectedAccount, setSelectedAccount] = useState<InjectedAccountWithMeta | null>(null)
  const [isConnected, setIsConnected] = useState(false)
  const [isLoading, setIsLoading] = useState(false)
  const [error, setError] = useState<Error | null>(null)

  const connect = useCallback(async () => {
    try {
      setIsLoading(true)
      setError(null)

      // Request authorization from wallet extension
      const extensions = await web3Enable(appName)

      if (extensions.length === 0) {
        throw new Error('No Polkadot.js extension found. Please install it.')
      }

      // Get all accounts
      const allAccounts = await web3Accounts()

      if (allAccounts.length === 0) {
        throw new Error('No accounts found in wallet')
      }

      setAccounts(allAccounts)
      setSelectedAccount(allAccounts[0])
      setIsConnected(true)
    } catch (err) {
      setError(err as Error)
      setIsConnected(false)
    } finally {
      setIsLoading(false)
    }
  }, [appName])

  const disconnect = useCallback(() => {
    setAccounts([])
    setSelectedAccount(null)
    setIsConnected(false)
  }, [])

  const selectAccount = useCallback((account: InjectedAccountWithMeta) => {
    setSelectedAccount(account)
  }, [])

  // Auto-connect if previously connected (from localStorage)
  useEffect(() => {
    const wasConnected = localStorage.getItem('wallet-connected')
    if (wasConnected === 'true') {
      connect()
    }
  }, [connect])

  // Save connection state
  useEffect(() => {
    localStorage.setItem('wallet-connected', isConnected.toString())
  }, [isConnected])

  return {
    accounts,
    selectedAccount,
    isConnected,
    isLoading,
    error,
    connect,
    disconnect,
    selectAccount
  }
}
