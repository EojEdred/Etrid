/**
 * Social Feed Screen
 * Activity feed showing friend transactions, milestones, and achievements
 */

'use client';

import { useState } from 'react';
import { ArrowLeft, Filter, TrendingUp, Users } from 'lucide-react';
import { Button } from '@/components/ui/button';
import { Tabs, TabsContent, TabsList, TabsTrigger } from '@/components/ui/tabs';
import {
  DropdownMenu,
  DropdownMenuCheckboxItem,
  DropdownMenuContent,
  DropdownMenuLabel,
  DropdownMenuSeparator,
  DropdownMenuTrigger,
} from '@/components/ui/dropdown-menu';
import { ActivityFeedItem } from '@/components/social/activity-feed-item';
import { useSocialFeed, MILESTONES } from '@/hooks/useSocialFeed';
import { useWallet } from '@/lib/polkadot/useWallet';
import type { ActivityType } from '@/lib/social/types';

interface SocialFeedScreenProps {
  onBack: () => void;
}

export function SocialFeedScreen({ onBack }: SocialFeedScreenProps) {
  const { selectedAccount } = useWallet();
  const userId = selectedAccount?.address || '';

  const { filteredActivities, isLoading, setFilter, likeActivity, commentOnActivity } =
    useSocialFeed(userId);

  const [selectedTypes, setSelectedTypes] = useState<ActivityType[]>([]);
  const [friendsOnly, setFriendsOnly] = useState(false);

  const handleFilterChange = () => {
    setFilter({
      type: selectedTypes.length > 0 ? selectedTypes : undefined,
      friendsOnly,
    });
  };

  const toggleActivityType = (type: ActivityType) => {
    setSelectedTypes((prev) => {
      const newTypes = prev.includes(type)
        ? prev.filter((t) => t !== type)
        : [...prev, type];
      return newTypes;
    });
  };

  return (
    <div className="min-h-screen bg-background pb-24">
      {/* Header */}
      <header className="sticky top-0 z-10 glass-strong border-b border-border">
        <div className="flex items-center gap-4 p-6">
          <Button variant="ghost" size="icon" onClick={onBack}>
            <ArrowLeft className="w-5 h-5" />
          </Button>
          <div className="flex-1">
            <h1 className="text-xl font-bold">Social Feed</h1>
            <p className="text-sm text-muted-foreground">
              Community activity and achievements
            </p>
          </div>

          {/* Filter Menu */}
          <DropdownMenu>
            <DropdownMenuTrigger asChild>
              <Button variant="outline" size="icon">
                <Filter className="w-5 h-5" />
              </Button>
            </DropdownMenuTrigger>
            <DropdownMenuContent align="end" className="w-56">
              <DropdownMenuLabel>Filter Activities</DropdownMenuLabel>
              <DropdownMenuSeparator />

              <DropdownMenuCheckboxItem
                checked={friendsOnly}
                onCheckedChange={(checked) => {
                  setFriendsOnly(checked);
                  handleFilterChange();
                }}
              >
                Friends Only
              </DropdownMenuCheckboxItem>

              <DropdownMenuSeparator />
              <DropdownMenuLabel>Activity Types</DropdownMenuLabel>

              <DropdownMenuCheckboxItem
                checked={selectedTypes.includes('transaction_sent')}
                onCheckedChange={() => {
                  toggleActivityType('transaction_sent');
                  handleFilterChange();
                }}
              >
                Transactions
              </DropdownMenuCheckboxItem>

              <DropdownMenuCheckboxItem
                checked={selectedTypes.includes('milestone_reached')}
                onCheckedChange={() => {
                  toggleActivityType('milestone_reached');
                  handleFilterChange();
                }}
              >
                Milestones
              </DropdownMenuCheckboxItem>

              <DropdownMenuCheckboxItem
                checked={selectedTypes.includes('staking_reward')}
                onCheckedChange={() => {
                  toggleActivityType('staking_reward');
                  handleFilterChange();
                }}
              >
                Staking
              </DropdownMenuCheckboxItem>

              <DropdownMenuCheckboxItem
                checked={selectedTypes.includes('governance_vote')}
                onCheckedChange={() => {
                  toggleActivityType('governance_vote');
                  handleFilterChange();
                }}
              >
                Governance
              </DropdownMenuCheckboxItem>

              <DropdownMenuCheckboxItem
                checked={selectedTypes.includes('bill_split_created')}
                onCheckedChange={() => {
                  toggleActivityType('bill_split_created');
                  handleFilterChange();
                }}
              >
                Bill Splits
              </DropdownMenuCheckboxItem>
            </DropdownMenuContent>
          </DropdownMenu>
        </div>
      </header>

      {/* Content */}
      <main className="p-6">
        <Tabs defaultValue="feed" className="space-y-6">
          <TabsList className="grid w-full grid-cols-2">
            <TabsTrigger value="feed">
              <Users className="w-4 h-4 mr-2" />
              Feed
            </TabsTrigger>
            <TabsTrigger value="milestones">
              <TrendingUp className="w-4 h-4 mr-2" />
              Milestones
            </TabsTrigger>
          </TabsList>

          {/* Activity Feed */}
          <TabsContent value="feed" className="space-y-4">
            {isLoading ? (
              <div className="text-center py-12">
                <p className="text-muted-foreground">Loading activities...</p>
              </div>
            ) : filteredActivities.length === 0 ? (
              <div className="text-center py-12">
                <Users className="w-12 h-12 mx-auto text-muted-foreground mb-4" />
                <p className="text-muted-foreground">No activities yet</p>
                <p className="text-sm text-muted-foreground mt-2">
                  Connect with friends to see their activities
                </p>
              </div>
            ) : (
              <div className="space-y-3">
                {filteredActivities.map((activity) => (
                  <ActivityFeedItem
                    key={activity.id}
                    activity={activity}
                    onLike={likeActivity}
                    onComment={commentOnActivity}
                  />
                ))}
              </div>
            )}
          </TabsContent>

          {/* Milestones */}
          <TabsContent value="milestones" className="space-y-4">
            <div className="grid gap-4">
              {MILESTONES.map((milestone) => (
                <div
                  key={`${milestone.type}-${milestone.threshold}`}
                  className="p-4 rounded-lg bg-card border border-border"
                >
                  <div className="flex items-start gap-4">
                    <div className="text-3xl">{milestone.icon}</div>
                    <div className="flex-1">
                      <h3 className="font-semibold text-foreground mb-1">
                        {milestone.title}
                      </h3>
                      <p className="text-sm text-muted-foreground mb-2">
                        {milestone.description}
                      </p>
                      <div className="flex items-center gap-2">
                        <div className="text-xs text-muted-foreground uppercase tracking-wide">
                          {milestone.type}
                        </div>
                        <div className="text-xs text-accent font-semibold">
                          {milestone.threshold.toLocaleString()}
                        </div>
                      </div>
                    </div>
                  </div>
                </div>
              ))}
            </div>

            <div className="p-6 rounded-lg bg-accent/10 border border-accent/20 text-center">
              <TrendingUp className="w-12 h-12 mx-auto text-accent mb-3" />
              <h3 className="font-semibold text-foreground mb-2">
                Keep Growing!
              </h3>
              <p className="text-sm text-muted-foreground">
                Complete actions to unlock milestones and celebrate with the community
              </p>
            </div>
          </TabsContent>
        </Tabs>
      </main>
    </div>
  );
}
