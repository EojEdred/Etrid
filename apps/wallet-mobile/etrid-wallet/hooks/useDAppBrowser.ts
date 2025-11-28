/**
 * useDAppBrowser Hook
 * Manages dApp browser state and navigation
 */

import { useState, useEffect, useCallback } from 'react';
import { BrowserTab, BrowserHistory } from '@/types/dapp';
import { dAppBrowserService } from '@/services/DAppBrowserService';

export function useDAppBrowser() {
  const [tabs, setTabs] = useState<BrowserTab[]>([]);
  const [activeTabId, setActiveTabId] = useState<string | null>(null);
  const [history, setHistory] = useState<BrowserHistory[]>([]);
  const [canGoBack, setCanGoBack] = useState(false);
  const [canGoForward, setCanGoForward] = useState(false);
  const [isLoading, setIsLoading] = useState(false);

  // Initialize with a default tab
  useEffect(() => {
    if (tabs.length === 0) {
      createTab('https://app.uniswap.org');
    }
  }, []);

  /**
   * Create a new tab
   */
  const createTab = useCallback((url: string) => {
    const newTab: BrowserTab = {
      id: Date.now().toString(),
      url,
      title: 'Loading...',
      isActive: true,
    };

    setTabs((prevTabs) => {
      const updatedTabs = prevTabs.map((tab) => ({ ...tab, isActive: false }));
      return [...updatedTabs, newTab];
    });

    setActiveTabId(newTab.id);
  }, []);

  /**
   * Switch to a tab
   */
  const switchTab = useCallback((tabId: string) => {
    setTabs((prevTabs) =>
      prevTabs.map((tab) => ({
        ...tab,
        isActive: tab.id === tabId,
      }))
    );
    setActiveTabId(tabId);
  }, []);

  /**
   * Close a tab
   */
  const closeTab = useCallback((tabId: string) => {
    setTabs((prevTabs) => {
      const filtered = prevTabs.filter((tab) => tab.id !== tabId);

      // If closing active tab, activate another
      if (activeTabId === tabId && filtered.length > 0) {
        filtered[0].isActive = true;
        setActiveTabId(filtered[0].id);
      }

      return filtered;
    });
  }, [activeTabId]);

  /**
   * Update tab info
   */
  const updateTab = useCallback(
    (tabId: string, updates: Partial<BrowserTab>) => {
      setTabs((prevTabs) =>
        prevTabs.map((tab) =>
          tab.id === tabId ? { ...tab, ...updates } : tab
        )
      );
    },
    []
  );

  /**
   * Navigate to URL in active tab
   */
  const navigateTo = useCallback(
    (url: string) => {
      if (!activeTabId) return;

      updateTab(activeTabId, { url, title: 'Loading...' });
      setIsLoading(true);

      // Add to history
      addToHistory(url, 'Loading...');
    },
    [activeTabId, updateTab]
  );

  /**
   * Go back in history
   */
  const goBack = useCallback(() => {
    // TODO: Implement WebView back navigation
    setCanGoBack(false);
  }, []);

  /**
   * Go forward in history
   */
  const goForward = useCallback(() => {
    // TODO: Implement WebView forward navigation
    setCanGoForward(false);
  }, []);

  /**
   * Reload current tab
   */
  const reload = useCallback(() => {
    setIsLoading(true);
    // WebView reload would be triggered
  }, []);

  /**
   * Stop loading
   */
  const stopLoading = useCallback(() => {
    setIsLoading(false);
  }, []);

  /**
   * Add to browser history
   */
  const addToHistory = useCallback((url: string, title: string) => {
    const historyItem: BrowserHistory = {
      id: Date.now().toString(),
      url,
      title,
      visitedAt: new Date(),
    };

    setHistory((prevHistory) => [historyItem, ...prevHistory.slice(0, 99)]);
  }, []);

  /**
   * Clear browser history
   */
  const clearHistory = useCallback(() => {
    setHistory([]);
  }, []);

  /**
   * Get active tab
   */
  const activeTab = tabs.find((tab) => tab.id === activeTabId);

  return {
    tabs,
    activeTab,
    activeTabId,
    history,
    canGoBack,
    canGoForward,
    isLoading,
    createTab,
    switchTab,
    closeTab,
    updateTab,
    navigateTo,
    goBack,
    goForward,
    reload,
    stopLoading,
    clearHistory,
  };
}
