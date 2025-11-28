/**
 * useDAOs Hook
 * Manages user's DAOs and DAO operations
 */

import { useState, useEffect, useCallback } from 'react';
import { DAO, DAODetail, DAOParams } from '@/types/dao';
import { daoService } from '@/services/DAOService';

export function useDAOs() {
  const [daos, setDaos] = useState<DAO[]>([]);
  const [memberDAOs, setMemberDAOs] = useState<DAO[]>([]);
  const [ownedDAOs, setOwnedDAOs] = useState<DAO[]>([]);
  const [isLoading, setIsLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);

  // Load DAOs on mount
  useEffect(() => {
    loadDAOs();
  }, []);

  /**
   * Load all user's DAOs
   */
  const loadDAOs = async () => {
    setIsLoading(true);
    try {
      const [all, member, owned] = await Promise.all([
        daoService.getDAOs(),
        daoService.getDAOs('member'),
        daoService.getDAOs('owner'),
      ]);

      setDaos(all);
      setMemberDAOs(member);
      setOwnedDAOs(owned);
      setError(null);
    } catch (err: any) {
      console.error('Failed to load DAOs:', err);
      setError(err.message);
    } finally {
      setIsLoading(false);
    }
  };

  /**
   * Get detailed DAO information
   */
  const getDAODetails = useCallback(async (daoId: string): Promise<DAODetail | null> => {
    setIsLoading(true);
    try {
      const details = await daoService.getDAODetails(daoId);
      setError(null);
      return details;
    } catch (err: any) {
      console.error('Failed to get DAO details:', err);
      setError(err.message);
      return null;
    } finally {
      setIsLoading(false);
    }
  }, []);

  /**
   * Create a new DAO
   */
  const createDAO = useCallback(async (params: DAOParams) => {
    setIsLoading(true);
    try {
      const newDAO = await daoService.createDAO(params);
      setDaos((prev) => [...prev, newDAO]);
      setOwnedDAOs((prev) => [...prev, newDAO]);
      setError(null);
      return newDAO;
    } catch (err: any) {
      console.error('Failed to create DAO:', err);
      setError(err.message);
      throw err;
    } finally {
      setIsLoading(false);
    }
  }, []);

  /**
   * Join a DAO
   */
  const joinDAO = useCallback(async (daoId: string) => {
    setIsLoading(true);
    try {
      await daoService.joinDAO(daoId);
      await loadDAOs(); // Reload to update membership
      setError(null);
    } catch (err: any) {
      console.error('Failed to join DAO:', err);
      setError(err.message);
      throw err;
    } finally {
      setIsLoading(false);
    }
  }, []);

  /**
   * Leave a DAO
   */
  const leaveDAO = useCallback(async (daoId: string) => {
    setIsLoading(true);
    try {
      await daoService.leaveDAO(daoId);
      setDaos((prev) => prev.filter((d) => d.id !== daoId));
      setMemberDAOs((prev) => prev.filter((d) => d.id !== daoId));
      setError(null);
    } catch (err: any) {
      console.error('Failed to leave DAO:', err);
      setError(err.message);
      throw err;
    } finally {
      setIsLoading(false);
    }
  }, []);

  /**
   * Get DAO by ID
   */
  const getDAO = useCallback(
    (daoId: string) => {
      return daos.find((d) => d.id === daoId);
    },
    [daos]
  );

  /**
   * Check if user is member of DAO
   */
  const isMember = useCallback(
    (daoId: string) => {
      return daos.some((d) => d.id === daoId);
    },
    [daos]
  );

  /**
   * Check if user is owner of DAO
   */
  const isOwner = useCallback(
    (daoId: string) => {
      return ownedDAOs.some((d) => d.id === daoId);
    },
    [ownedDAOs]
  );

  return {
    daos,
    memberDAOs,
    ownedDAOs,
    isLoading,
    error,
    createDAO,
    joinDAO,
    leaveDAO,
    getDAO,
    getDAODetails,
    isMember,
    isOwner,
    refreshDAOs: loadDAOs,
  };
}
