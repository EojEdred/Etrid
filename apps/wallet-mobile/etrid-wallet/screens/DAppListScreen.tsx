/**
 * DAppListScreen
 * Main dApp directory screen with featured dApps, categories, and search
 */

'use client';

import React, { useState } from 'react';
import { useDAppDirectory } from '@/hooks/useDAppDirectory';
import { DApp, DAppCategory } from '@/types/dapp';
import { DAppCard } from '@/components/dapp/DAppCard';
import { Input } from '@/components/ui/input';
import { Tabs, TabsContent, TabsList, TabsTrigger } from '@/components/ui/tabs';
import { Button } from '@/components/ui/button';
import { Carousel, CarouselContent, CarouselItem, CarouselNext, CarouselPrevious } from '@/components/ui/carousel';
import { Search, Plus, TrendingUp, Clock } from 'lucide-react';

interface DAppListScreenProps {
  onOpenDApp: (dApp: DApp) => void;
  onAddCustomDApp: () => void;
}

export function DAppListScreen({ onOpenDApp, onAddCustomDApp }: DAppListScreenProps) {
  const {
    featuredDApps,
    trendingDApps,
    recentlyVisited,
    searchResults,
    isLoading,
    searchDApps,
    getDAppsByCategory,
    clearSearch,
  } = useDAppDirectory();

  const [searchQuery, setSearchQuery] = useState('');
  const [selectedCategory, setSelectedCategory] = useState<DAppCategory | 'all'>('all');

  const categories: (DAppCategory | 'all')[] = [
    'all',
    'DeFi',
    'NFT',
    'Gaming',
    'Social',
    'Tools',
  ];

  const handleSearch = (query: string) => {
    setSearchQuery(query);
    if (query.trim()) {
      searchDApps(query);
    } else {
      clearSearch();
    }
  };

  const handleCategoryChange = (category: DAppCategory | 'all') => {
    setSelectedCategory(category);
    if (category !== 'all') {
      getDAppsByCategory(category);
    } else {
      clearSearch();
    }
  };

  const displayDApps = searchQuery
    ? searchResults
    : selectedCategory !== 'all'
    ? searchResults
    : [];

  return (
    <div className="min-h-screen bg-background">
      {/* Header */}
      <div className="border-b bg-background/95 backdrop-blur supports-[backdrop-filter]:bg-background/60">
        <div className="container py-4">
          <div className="flex items-center justify-between mb-4">
            <h1 className="text-3xl font-bold">Explore dApps</h1>
            <Button onClick={onAddCustomDApp} variant="outline" size="sm">
              <Plus className="w-4 h-4 mr-2" />
              Add Custom
            </Button>
          </div>

          {/* Search Bar */}
          <div className="relative">
            <Search className="absolute left-3 top-1/2 -translate-y-1/2 w-4 h-4 text-muted-foreground" />
            <Input
              type="text"
              placeholder="Search dApps..."
              value={searchQuery}
              onChange={(e) => handleSearch(e.target.value)}
              className="pl-10"
            />
          </div>
        </div>
      </div>

      <div className="container py-6 space-y-8">
        {!searchQuery && (
          <>
            {/* Featured dApps Carousel */}
            {featuredDApps.length > 0 && (
              <section>
                <h2 className="text-2xl font-bold mb-4">Featured</h2>
                <Carousel className="w-full">
                  <CarouselContent>
                    {featuredDApps.map((dApp) => (
                      <CarouselItem key={dApp.id} className="md:basis-1/2 lg:basis-1/3">
                        <DAppCard dApp={dApp} onOpen={onOpenDApp} />
                      </CarouselItem>
                    ))}
                  </CarouselContent>
                  <CarouselPrevious />
                  <CarouselNext />
                </Carousel>
              </section>
            )}

            {/* Recently Visited */}
            {recentlyVisited.length > 0 && (
              <section>
                <div className="flex items-center gap-2 mb-4">
                  <Clock className="w-5 h-5" />
                  <h2 className="text-2xl font-bold">Recently Visited</h2>
                </div>
                <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
                  {recentlyVisited.slice(0, 6).map((dApp) => (
                    <DAppCard key={dApp.id} dApp={dApp} onOpen={onOpenDApp} />
                  ))}
                </div>
              </section>
            )}

            {/* Trending dApps */}
            {trendingDApps.length > 0 && (
              <section>
                <div className="flex items-center gap-2 mb-4">
                  <TrendingUp className="w-5 h-5 text-orange-500" />
                  <h2 className="text-2xl font-bold">Trending</h2>
                </div>
                <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
                  {trendingDApps.slice(0, 6).map((dApp) => (
                    <DAppCard key={dApp.id} dApp={dApp} onOpen={onOpenDApp} />
                  ))}
                </div>
              </section>
            )}
          </>
        )}

        {/* Categories */}
        <section>
          <h2 className="text-2xl font-bold mb-4">Browse by Category</h2>
          <Tabs value={selectedCategory} onValueChange={(v) => handleCategoryChange(v as any)}>
            <TabsList className="mb-4">
              {categories.map((category) => (
                <TabsTrigger key={category} value={category}>
                  {category === 'all' ? 'All' : category}
                </TabsTrigger>
              ))}
            </TabsList>

            {categories.map((category) => (
              <TabsContent key={category} value={category}>
                {displayDApps.length > 0 || searchQuery ? (
                  <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
                    {displayDApps.map((dApp) => (
                      <DAppCard key={dApp.id} dApp={dApp} onOpen={onOpenDApp} />
                    ))}
                  </div>
                ) : category === 'all' ? (
                  <div className="text-center py-12 text-muted-foreground">
                    <p>Select a category to browse dApps</p>
                  </div>
                ) : (
                  <div className="text-center py-12">
                    {isLoading ? (
                      <p className="text-muted-foreground">Loading...</p>
                    ) : (
                      <p className="text-muted-foreground">No dApps found in this category</p>
                    )}
                  </div>
                )}
              </TabsContent>
            ))}
          </Tabs>
        </section>

        {/* Search Results */}
        {searchQuery && (
          <section>
            <h2 className="text-2xl font-bold mb-4">
              Search Results for "{searchQuery}"
            </h2>
            {searchResults.length > 0 ? (
              <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
                {searchResults.map((dApp) => (
                  <DAppCard key={dApp.id} dApp={dApp} onOpen={onOpenDApp} />
                ))}
              </div>
            ) : (
              <div className="text-center py-12 text-muted-foreground">
                <p>No dApps found for "{searchQuery}"</p>
              </div>
            )}
          </section>
        )}
      </div>
    </div>
  );
}
