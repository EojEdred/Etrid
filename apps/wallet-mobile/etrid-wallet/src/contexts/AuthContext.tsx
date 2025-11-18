import React, { createContext, useContext, useState, useEffect, ReactNode } from 'react';
import { KeyringPair } from '@polkadot/keyring/types';
import KeychainService from '../services/KeychainService';
import BiometricService from '../services/BiometricService';
import EtridSDKService from '../services/EtridSDKService';

interface AuthContextType {
  isAuthenticated: boolean;
  isLoading: boolean;
  address: string | null;
  keypair: KeyringPair | null;
  biometricEnabled: boolean;
  biometricType: string;
  login: (keypair: KeyringPair, address: string) => Promise<void>;
  logout: () => Promise<void>;
  createWallet: (mnemonic?: string) => Promise<{ address: string; mnemonic: string }>;
  importWallet: (mnemonic: string) => Promise<string>;
  enableBiometric: () => Promise<boolean>;
  disableBiometric: () => Promise<void>;
  authenticateWithBiometric: () => Promise<boolean>;
}

const AuthContext = createContext<AuthContextType | undefined>(undefined);

interface AuthProviderProps {
  children: ReactNode;
}

export const AuthProvider: React.FC<AuthProviderProps> = ({ children }) => {
  const [isAuthenticated, setIsAuthenticated] = useState(false);
  const [isLoading, setIsLoading] = useState(true);
  const [address, setAddress] = useState<string | null>(null);
  const [keypair, setKeypair] = useState<KeyringPair | null>(null);
  const [biometricEnabled, setBiometricEnabled] = useState(false);
  const [biometricType, setBiometricType] = useState('Biometric');

  const sdk = EtridSDKService.getInstance();

  /**
   * Initialize auth state on app startup
   */
  useEffect(() => {
    initializeAuth();
  }, []);

  const initializeAuth = async () => {
    try {
      setIsLoading(true);

      // Check if wallet exists
      const hasWallet = await KeychainService.hasWallet();

      if (hasWallet) {
        // Load keypair from secure storage
        const storedKeypair = await KeychainService.loadKeypair();

        if (storedKeypair) {
          setKeypair(storedKeypair);
          setAddress(storedKeypair.address);

          // Connect to FlareChain
          await sdk.connect();
          sdk.setAccount(storedKeypair);

          setIsAuthenticated(true);

          // Check biometric settings
          const biometricAvailable = await BiometricService.isAvailable();
          if (biometricAvailable) {
            setBiometricEnabled(true);
            const bioType = await BiometricService.getBiometricTypeName();
            setBiometricType(bioType);
          }
        }
      }
    } catch (error) {
      console.error('Error initializing auth:', error);
    } finally {
      setIsLoading(false);
    }
  };

  /**
   * Create new wallet
   */
  const createWallet = async (mnemonic?: string): Promise<{ address: string; mnemonic: string }> => {
    try {
      const wallet = await KeychainService.createWallet(mnemonic);
      return wallet;
    } catch (error) {
      console.error('Error creating wallet:', error);
      throw error;
    }
  };

  /**
   * Import wallet from mnemonic
   */
  const importWallet = async (mnemonic: string): Promise<string> => {
    try {
      const walletAddress = await KeychainService.importWallet(mnemonic);
      return walletAddress;
    } catch (error) {
      console.error('Error importing wallet:', error);
      throw error;
    }
  };

  /**
   * Login user after wallet creation/import
   */
  const login = async (userKeypair: KeyringPair, userAddress: string): Promise<void> => {
    try {
      setIsLoading(true);

      // Connect to FlareChain
      await sdk.connect();
      sdk.setAccount(userKeypair);

      setKeypair(userKeypair);
      setAddress(userAddress);
      setIsAuthenticated(true);
    } catch (error) {
      console.error('Error logging in:', error);
      throw new Error('Failed to connect to Ëtrid network');
    } finally {
      setIsLoading(false);
    }
  };

  /**
   * Logout user
   */
  const logout = async (): Promise<void> => {
    try {
      // Disconnect from FlareChain
      await sdk.disconnect();

      setIsAuthenticated(false);
      setAddress(null);
      setKeypair(null);
      setBiometricEnabled(false);
    } catch (error) {
      console.error('Error logging out:', error);
    }
  };

  /**
   * Enable biometric authentication
   */
  const enableBiometric = async (): Promise<boolean> => {
    try {
      const canUse = await BiometricService.canUseBiometric();

      if (!canUse.canUse) {
        console.error('Cannot use biometric:', canUse.reason);
        return false;
      }

      // Test biometric authentication
      const result = await BiometricService.authenticate('Enable biometric authentication');

      if (result.success) {
        setBiometricEnabled(true);
        const bioType = await BiometricService.getBiometricTypeName();
        setBiometricType(bioType);
        return true;
      } else {
        console.error('Biometric authentication failed:', result.error);
        return false;
      }
    } catch (error) {
      console.error('Error enabling biometric:', error);
      return false;
    }
  };

  /**
   * Disable biometric authentication
   */
  const disableBiometric = async (): Promise<void> => {
    setBiometricEnabled(false);
  };

  /**
   * Authenticate with biometric
   */
  const authenticateWithBiometric = async (): Promise<boolean> => {
    try {
      if (!biometricEnabled) {
        return false;
      }

      const result = await BiometricService.authenticate('Confirm your identity');
      return result.success;
    } catch (error) {
      console.error('Error authenticating with biometric:', error);
      return false;
    }
  };

  const value: AuthContextType = {
    isAuthenticated,
    isLoading,
    address,
    keypair,
    biometricEnabled,
    biometricType,
    login,
    logout,
    createWallet,
    importWallet,
    enableBiometric,
    disableBiometric,
    authenticateWithBiometric,
  };

  return <AuthContext.Provider value={value}>{children}</AuthContext.Provider>;
};

/**
 * Hook to use auth context
 */
export const useAuth = (): AuthContextType => {
  const context = useContext(AuthContext);
  if (context === undefined) {
    throw new Error('useAuth must be used within an AuthProvider');
  }
  return context;
};
