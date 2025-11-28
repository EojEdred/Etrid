/**
 * useContacts Hook
 * React hook for contacts management
 */

'use client';

import { useState, useCallback, useEffect } from 'react';
import { createContactsService } from '@/lib/social/ContactsService';
import type { Contact, ContactInput } from '@/lib/social/types';

export interface UseContactsReturn {
  // State
  contacts: Contact[];
  favorites: Contact[];
  recentContacts: Contact[];
  tags: string[];
  isLoading: boolean;
  error: string | null;

  // Actions
  fetchContacts: () => Promise<void>;
  addContact: (input: ContactInput) => Promise<Contact>;
  updateContact: (id: string, updates: Partial<Contact>) => Promise<Contact>;
  deleteContact: (id: string) => Promise<void>;
  searchContacts: (query: string) => Promise<Contact[]>;
  toggleFavorite: (id: string) => Promise<void>;
  getContactsByTag: (tag: string) => Contact[];
  clearError: () => void;
}

export function useContacts(userId: string): UseContactsReturn {
  const [contacts, setContacts] = useState<Contact[]>([]);
  const [favorites, setFavorites] = useState<Contact[]>([]);
  const [recentContacts, setRecentContacts] = useState<Contact[]>([]);
  const [tags, setTags] = useState<string[]>([]);
  const [isLoading, setIsLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);

  const service = createContactsService(userId);

  /**
   * Fetch all contacts
   */
  const fetchContacts = useCallback(async () => {
    setIsLoading(true);
    setError(null);

    try {
      const allContacts = await service.getContacts();
      setContacts(allContacts);

      // Update favorites
      const favs = allContacts.filter((c) => c.isFavorite);
      setFavorites(favs);

      // Update recent contacts
      const recent = allContacts
        .filter((c) => c.lastTransactionAt)
        .sort((a, b) => {
          const dateA = a.lastTransactionAt?.getTime() || 0;
          const dateB = b.lastTransactionAt?.getTime() || 0;
          return dateB - dateA;
        })
        .slice(0, 10);
      setRecentContacts(recent);

      // Update tags
      const allTags = await service.getTags();
      setTags(allTags);
    } catch (err) {
      const message = err instanceof Error ? err.message : 'Failed to fetch contacts';
      setError(message);
    } finally {
      setIsLoading(false);
    }
  }, [service]);

  /**
   * Add a new contact
   */
  const addContact = useCallback(
    async (input: ContactInput): Promise<Contact> => {
      setError(null);

      try {
        const newContact = await service.addContact(input);
        setContacts((prev) => [...prev, newContact]);
        return newContact;
      } catch (err) {
        const message = err instanceof Error ? err.message : 'Failed to add contact';
        setError(message);
        throw err;
      }
    },
    [service]
  );

  /**
   * Update an existing contact
   */
  const updateContact = useCallback(
    async (id: string, updates: Partial<Contact>): Promise<Contact> => {
      setError(null);

      try {
        const updated = await service.updateContact(id, updates);
        setContacts((prev) =>
          prev.map((c) => (c.id === id ? updated : c))
        );

        // Update favorites if needed
        if (updates.isFavorite !== undefined) {
          setFavorites((prev) =>
            updates.isFavorite
              ? [...prev, updated]
              : prev.filter((c) => c.id !== id)
          );
        }

        return updated;
      } catch (err) {
        const message = err instanceof Error ? err.message : 'Failed to update contact';
        setError(message);
        throw err;
      }
    },
    [service]
  );

  /**
   * Delete a contact
   */
  const deleteContact = useCallback(
    async (id: string): Promise<void> => {
      setError(null);

      try {
        await service.deleteContact(id);
        setContacts((prev) => prev.filter((c) => c.id !== id));
        setFavorites((prev) => prev.filter((c) => c.id !== id));
        setRecentContacts((prev) => prev.filter((c) => c.id !== id));
      } catch (err) {
        const message = err instanceof Error ? err.message : 'Failed to delete contact';
        setError(message);
        throw err;
      }
    },
    [service]
  );

  /**
   * Search contacts
   */
  const searchContacts = useCallback(
    async (query: string): Promise<Contact[]> => {
      setError(null);

      try {
        return await service.searchContacts(query);
      } catch (err) {
        const message = err instanceof Error ? err.message : 'Failed to search contacts';
        setError(message);
        return [];
      }
    },
    [service]
  );

  /**
   * Toggle favorite status
   */
  const toggleFavorite = useCallback(
    async (id: string): Promise<void> => {
      const contact = contacts.find((c) => c.id === id);
      if (!contact) return;

      await updateContact(id, { isFavorite: !contact.isFavorite });
    },
    [contacts, updateContact]
  );

  /**
   * Get contacts by tag
   */
  const getContactsByTag = useCallback(
    (tag: string): Contact[] => {
      return contacts.filter((c) => c.tags.includes(tag));
    },
    [contacts]
  );

  /**
   * Clear error state
   */
  const clearError = useCallback(() => {
    setError(null);
  }, []);

  // Fetch contacts on mount
  useEffect(() => {
    fetchContacts();
  }, [fetchContacts]);

  return {
    contacts,
    favorites,
    recentContacts,
    tags,
    isLoading,
    error,
    fetchContacts,
    addContact,
    updateContact,
    deleteContact,
    searchContacts,
    toggleFavorite,
    getContactsByTag,
    clearError,
  };
}
