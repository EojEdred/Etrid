/**
 * BrowserNavBar Component
 * Navigation bar for dApp browser with URL input and controls
 */

'use client';

import React, { useState } from 'react';
import { Input } from '@/components/ui/input';
import { Button } from '@/components/ui/button';
import {
  ArrowLeft,
  ArrowRight,
  RotateCw,
  Home,
  Bookmark,
  BookmarkPlus,
  Layers,
  Loader2,
} from 'lucide-react';
import { Badge } from '@/components/ui/badge';

interface BrowserNavBarProps {
  url: string;
  isLoading: boolean;
  canGoBack: boolean;
  canGoForward: boolean;
  isConnected: boolean;
  isBookmarked: boolean;
  onNavigate: (url: string) => void;
  onBack: () => void;
  onForward: () => void;
  onRefresh: () => void;
  onHome: () => void;
  onBookmark: () => void;
  onTabs: () => void;
}

export function BrowserNavBar({
  url,
  isLoading,
  canGoBack,
  canGoForward,
  isConnected,
  isBookmarked,
  onNavigate,
  onBack,
  onForward,
  onRefresh,
  onHome,
  onBookmark,
  onTabs,
}: BrowserNavBarProps) {
  const [inputUrl, setInputUrl] = useState(url);

  const handleSubmit = (e: React.FormEvent) => {
    e.preventDefault();
    if (inputUrl.trim()) {
      let finalUrl = inputUrl.trim();
      // Add https:// if no protocol specified
      if (!finalUrl.startsWith('http://') && !finalUrl.startsWith('https://')) {
        finalUrl = 'https://' + finalUrl;
      }
      onNavigate(finalUrl);
    }
  };

  React.useEffect(() => {
    setInputUrl(url);
  }, [url]);

  return (
    <div className="bg-background border-b">
      {/* Navigation Controls */}
      <div className="flex items-center gap-1 p-2">
        {/* Back */}
        <Button
          variant="ghost"
          size="icon"
          onClick={onBack}
          disabled={!canGoBack}
        >
          <ArrowLeft className="h-4 w-4" />
        </Button>

        {/* Forward */}
        <Button
          variant="ghost"
          size="icon"
          onClick={onForward}
          disabled={!canGoForward}
        >
          <ArrowRight className="h-4 w-4" />
        </Button>

        {/* Refresh */}
        <Button
          variant="ghost"
          size="icon"
          onClick={onRefresh}
          disabled={isLoading}
        >
          {isLoading ? (
            <Loader2 className="h-4 w-4 animate-spin" />
          ) : (
            <RotateCw className="h-4 w-4" />
          )}
        </Button>

        {/* Home */}
        <Button variant="ghost" size="icon" onClick={onHome}>
          <Home className="h-4 w-4" />
        </Button>

        {/* URL Bar */}
        <form onSubmit={handleSubmit} className="flex-1 flex items-center gap-2">
          <div className="flex-1 relative">
            {/* Connection Status Indicator */}
            {isConnected && (
              <div className="absolute left-3 top-1/2 -translate-y-1/2">
                <div className="w-2 h-2 bg-green-500 rounded-full" />
              </div>
            )}

            <Input
              type="text"
              value={inputUrl}
              onChange={(e) => setInputUrl(e.target.value)}
              placeholder="Search or enter URL"
              className={`pr-10 ${isConnected ? 'pl-8' : ''}`}
            />

            {/* Security Badge */}
            {url.startsWith('https://') && (
              <div className="absolute right-3 top-1/2 -translate-y-1/2">
                <Badge variant="outline" className="text-xs py-0">
                  🔒
                </Badge>
              </div>
            )}
          </div>
        </form>

        {/* Bookmark */}
        <Button
          variant="ghost"
          size="icon"
          onClick={onBookmark}
          className={isBookmarked ? 'text-yellow-500' : ''}
        >
          {isBookmarked ? (
            <Bookmark className="h-4 w-4 fill-current" />
          ) : (
            <BookmarkPlus className="h-4 w-4" />
          )}
        </Button>

        {/* Tabs */}
        <Button variant="ghost" size="icon" onClick={onTabs}>
          <Layers className="h-4 w-4" />
        </Button>
      </div>
    </div>
  );
}
