/**
 * useBookmarks Hook
 * Manages dApp bookmarks
 */

import { useState, useEffect, useCallback } from 'react';
import { Bookmark } from '@/types/dapp';
import { dAppDirectoryService } from '@/services/DAppDirectoryService';

export function useBookmarks() {
  const [bookmarks, setBookmarks] = useState<Bookmark[]>([]);
  const [folders, setFolders] = useState<string[]>([]);
  const [isLoading, setIsLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);

  // Load bookmarks on mount
  useEffect(() => {
    loadBookmarks();
    loadFolders();
  }, []);

  /**
   * Load all bookmarks
   */
  const loadBookmarks = async () => {
    setIsLoading(true);
    try {
      const loadedBookmarks = await dAppDirectoryService.getBookmarks();
      setBookmarks(loadedBookmarks);
      setError(null);
    } catch (err: any) {
      console.error('Failed to load bookmarks:', err);
      setError(err.message);
    } finally {
      setIsLoading(false);
    }
  };

  /**
   * Load bookmark folders
   */
  const loadFolders = async () => {
    try {
      const loadedFolders = await dAppDirectoryService.getFolders();
      setFolders(loadedFolders);
    } catch (err: any) {
      console.error('Failed to load folders:', err);
    }
  };

  /**
   * Get bookmarks by folder
   */
  const getBookmarksByFolder = useCallback(
    async (folder?: string) => {
      setIsLoading(true);
      try {
        const filtered = await dAppDirectoryService.getBookmarksByFolder(folder);
        setError(null);
        return filtered;
      } catch (err: any) {
        console.error('Failed to get bookmarks by folder:', err);
        setError(err.message);
        return [];
      } finally {
        setIsLoading(false);
      }
    },
    []
  );

  /**
   * Add bookmark
   */
  const addBookmark = useCallback(
    async (bookmark: Omit<Bookmark, 'id' | 'createdAt'>) => {
      setIsLoading(true);
      try {
        const newBookmark = await dAppDirectoryService.addBookmark(bookmark);
        setBookmarks((prev) => [...prev, newBookmark]);

        // Update folders if new folder added
        if (newBookmark.folder && !folders.includes(newBookmark.folder)) {
          setFolders((prev) => [...prev, newBookmark.folder!]);
        }

        setError(null);
        return newBookmark;
      } catch (err: any) {
        console.error('Failed to add bookmark:', err);
        setError(err.message);
        throw err;
      } finally {
        setIsLoading(false);
      }
    },
    [folders]
  );

  /**
   * Update bookmark
   */
  const updateBookmark = useCallback(
    async (
      id: string,
      updates: Partial<Omit<Bookmark, 'id' | 'userId' | 'createdAt'>>
    ) => {
      setIsLoading(true);
      try {
        const updated = await dAppDirectoryService.updateBookmark(id, updates);
        setBookmarks((prev) =>
          prev.map((b) => (b.id === id ? updated : b))
        );

        // Update folders if folder changed
        if (updates.folder && !folders.includes(updates.folder)) {
          setFolders((prev) => [...prev, updates.folder!]);
        }

        setError(null);
        return updated;
      } catch (err: any) {
        console.error('Failed to update bookmark:', err);
        setError(err.message);
        throw err;
      } finally {
        setIsLoading(false);
      }
    },
    [folders]
  );

  /**
   * Delete bookmark
   */
  const deleteBookmark = useCallback(async (id: string) => {
    setIsLoading(true);
    try {
      await dAppDirectoryService.deleteBookmark(id);
      setBookmarks((prev) => prev.filter((b) => b.id !== id));
      setError(null);
    } catch (err: any) {
      console.error('Failed to delete bookmark:', err);
      setError(err.message);
      throw err;
    } finally {
      setIsLoading(false);
    }
  }, []);

  /**
   * Check if URL is bookmarked
   */
  const isBookmarked = useCallback(
    async (url: string) => {
      return await dAppDirectoryService.isBookmarked(url);
    },
    []
  );

  /**
   * Get bookmark by URL
   */
  const getBookmarkByUrl = useCallback(
    (url: string) => {
      return bookmarks.find((b) => b.url === url);
    },
    [bookmarks]
  );

  return {
    bookmarks,
    folders,
    isLoading,
    error,
    addBookmark,
    updateBookmark,
    deleteBookmark,
    isBookmarked,
    getBookmarkByUrl,
    getBookmarksByFolder,
    refreshBookmarks: loadBookmarks,
    refreshFolders: loadFolders,
  };
}
