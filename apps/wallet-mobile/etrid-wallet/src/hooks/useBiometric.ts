import { useState, useEffect } from 'react';
import BiometricService from '../services/BiometricService';

export const useBiometric = () => {
  const [isAvailable, setIsAvailable] = useState(false);
  const [isEnrolled, setIsEnrolled] = useState(false);
  const [biometricType, setBiometricType] = useState('Biometric');
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    checkBiometric();
  }, []);

  const checkBiometric = async () => {
    try {
      setLoading(true);
      const available = await BiometricService.isAvailable();
      const enrolled = await BiometricService.isEnrolled();
      const type = await BiometricService.getBiometricTypeName();

      setIsAvailable(available);
      setIsEnrolled(enrolled);
      setBiometricType(type);
    } catch (error) {
      console.error('Error checking biometric:', error);
    } finally {
      setLoading(false);
    }
  };

  const authenticate = async (promptMessage?: string) => {
    return await BiometricService.authenticate(promptMessage);
  };

  const canUse = isAvailable && isEnrolled;

  return {
    isAvailable,
    isEnrolled,
    biometricType,
    canUse,
    loading,
    authenticate,
    refresh: checkBiometric,
  };
};
