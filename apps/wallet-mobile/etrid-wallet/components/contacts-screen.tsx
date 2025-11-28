/**
 * Contacts Screen
 * Address book with search, favorites, and contact management
 */

'use client';

import { useState } from 'react';
import { ArrowLeft, Search, Plus, Star, Users, Tag } from 'lucide-react';
import { Button } from '@/components/ui/button';
import { Input } from '@/components/ui/input';
import { Tabs, TabsContent, TabsList, TabsTrigger } from '@/components/ui/tabs';
import {
  Dialog,
  DialogContent,
  DialogDescription,
  DialogFooter,
  DialogHeader,
  DialogTitle,
  DialogTrigger,
} from '@/components/ui/dialog';
import { Label } from '@/components/ui/label';
import { ContactCard } from '@/components/social/contact-card';
import { useContacts } from '@/hooks/useContacts';
import { useWallet } from '@/lib/polkadot/useWallet';
import type { Contact, ContactInput } from '@/lib/social/types';

interface ContactsScreenProps {
  onBack: () => void;
  onSendToContact?: (contact: Contact) => void;
}

export function ContactsScreen({ onBack, onSendToContact }: ContactsScreenProps) {
  const { selectedAccount } = useWallet();
  const userId = selectedAccount?.address || '';

  const {
    contacts,
    favorites,
    recentContacts,
    tags,
    isLoading,
    addContact,
    updateContact,
    deleteContact,
    toggleFavorite,
    getContactsByTag,
  } = useContacts(userId);

  const [searchQuery, setSearchQuery] = useState('');
  const [isAddDialogOpen, setIsAddDialogOpen] = useState(false);
  const [selectedTag, setSelectedTag] = useState<string | null>(null);

  // Add contact form state
  const [newContact, setNewContact] = useState<ContactInput>({
    contactAddress: '',
    contactUsername: '',
    nickname: '',
    notes: '',
    tags: [],
  });

  const handleAddContact = async () => {
    try {
      await addContact(newContact);
      setIsAddDialogOpen(false);
      setNewContact({
        contactAddress: '',
        contactUsername: '',
        nickname: '',
        notes: '',
        tags: [],
      });
    } catch (error) {
      console.error('Failed to add contact:', error);
    }
  };

  const handleDeleteContact = async (contact: Contact) => {
    if (confirm(`Are you sure you want to delete ${contact.nickname || contact.contactUsername}?`)) {
      await deleteContact(contact.id);
    }
  };

  // Filter contacts
  const filteredContacts = searchQuery
    ? contacts.filter(
        (c) =>
          c.nickname?.toLowerCase().includes(searchQuery.toLowerCase()) ||
          c.contactUsername?.toLowerCase().includes(searchQuery.toLowerCase()) ||
          c.contactAddress.toLowerCase().includes(searchQuery.toLowerCase())
      )
    : selectedTag
    ? getContactsByTag(selectedTag)
    : contacts;

  return (
    <div className="min-h-screen bg-background pb-24">
      {/* Header */}
      <header className="sticky top-0 z-10 glass-strong border-b border-border">
        <div className="p-6 space-y-4">
          <div className="flex items-center gap-4">
            <Button variant="ghost" size="icon" onClick={onBack}>
              <ArrowLeft className="w-5 h-5" />
            </Button>
            <div className="flex-1">
              <h1 className="text-xl font-bold">Contacts</h1>
              <p className="text-sm text-muted-foreground">
                {contacts.length} {contacts.length === 1 ? 'contact' : 'contacts'}
              </p>
            </div>
            <Dialog open={isAddDialogOpen} onOpenChange={setIsAddDialogOpen}>
              <DialogTrigger asChild>
                <Button size="icon">
                  <Plus className="w-5 h-5" />
                </Button>
              </DialogTrigger>
              <DialogContent>
                <DialogHeader>
                  <DialogTitle>Add Contact</DialogTitle>
                  <DialogDescription>
                    Add a new contact by username or wallet address
                  </DialogDescription>
                </DialogHeader>
                <div className="space-y-4 py-4">
                  <div className="space-y-2">
                    <Label htmlFor="username">Username (optional)</Label>
                    <Input
                      id="username"
                      placeholder="alice.etrid"
                      value={newContact.contactUsername}
                      onChange={(e) =>
                        setNewContact({ ...newContact, contactUsername: e.target.value })
                      }
                    />
                  </div>
                  <div className="space-y-2">
                    <Label htmlFor="address">Wallet Address (optional if username provided)</Label>
                    <Input
                      id="address"
                      placeholder="5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY"
                      value={newContact.contactAddress}
                      onChange={(e) =>
                        setNewContact({ ...newContact, contactAddress: e.target.value })
                      }
                    />
                  </div>
                  <div className="space-y-2">
                    <Label htmlFor="nickname">Nickname (optional)</Label>
                    <Input
                      id="nickname"
                      placeholder="Alice"
                      value={newContact.nickname}
                      onChange={(e) =>
                        setNewContact({ ...newContact, nickname: e.target.value })
                      }
                    />
                  </div>
                  <div className="space-y-2">
                    <Label htmlFor="notes">Notes (optional)</Label>
                    <Input
                      id="notes"
                      placeholder="Friend from work"
                      value={newContact.notes}
                      onChange={(e) =>
                        setNewContact({ ...newContact, notes: e.target.value })
                      }
                    />
                  </div>
                </div>
                <DialogFooter>
                  <Button variant="outline" onClick={() => setIsAddDialogOpen(false)}>
                    Cancel
                  </Button>
                  <Button
                    onClick={handleAddContact}
                    disabled={!newContact.contactAddress && !newContact.contactUsername}
                  >
                    Add Contact
                  </Button>
                </DialogFooter>
              </DialogContent>
            </Dialog>
          </div>

          {/* Search */}
          <div className="relative">
            <Search className="absolute left-3 top-1/2 -translate-y-1/2 w-4 h-4 text-muted-foreground" />
            <Input
              placeholder="Search contacts..."
              value={searchQuery}
              onChange={(e) => setSearchQuery(e.target.value)}
              className="pl-10"
            />
          </div>
        </div>
      </header>

      {/* Content */}
      <main className="p-6">
        <Tabs defaultValue="all" className="space-y-6">
          <TabsList className="grid w-full grid-cols-3">
            <TabsTrigger value="all">
              <Users className="w-4 h-4 mr-2" />
              All
            </TabsTrigger>
            <TabsTrigger value="favorites">
              <Star className="w-4 h-4 mr-2" />
              Favorites
            </TabsTrigger>
            <TabsTrigger value="recent">
              <Users className="w-4 h-4 mr-2" />
              Recent
            </TabsTrigger>
          </TabsList>

          {/* All Contacts */}
          <TabsContent value="all" className="space-y-4">
            {/* Tags Filter */}
            {tags.length > 0 && (
              <div className="flex gap-2 flex-wrap">
                <Button
                  size="sm"
                  variant={selectedTag === null ? 'default' : 'outline'}
                  onClick={() => setSelectedTag(null)}
                >
                  All
                </Button>
                {tags.map((tag) => (
                  <Button
                    key={tag}
                    size="sm"
                    variant={selectedTag === tag ? 'default' : 'outline'}
                    onClick={() => setSelectedTag(tag)}
                  >
                    <Tag className="w-3 h-3 mr-1" />
                    {tag}
                  </Button>
                ))}
              </div>
            )}

            {isLoading ? (
              <div className="text-center py-12">
                <p className="text-muted-foreground">Loading contacts...</p>
              </div>
            ) : filteredContacts.length === 0 ? (
              <div className="text-center py-12">
                <Users className="w-12 h-12 mx-auto text-muted-foreground mb-4" />
                <p className="text-muted-foreground">
                  {searchQuery ? 'No contacts found' : 'No contacts yet'}
                </p>
              </div>
            ) : (
              <div className="space-y-3">
                {filteredContacts.map((contact) => (
                  <ContactCard
                    key={contact.id}
                    contact={contact}
                    onSend={onSendToContact}
                    onDelete={handleDeleteContact}
                    onToggleFavorite={toggleFavorite}
                  />
                ))}
              </div>
            )}
          </TabsContent>

          {/* Favorites */}
          <TabsContent value="favorites" className="space-y-3">
            {favorites.length === 0 ? (
              <div className="text-center py-12">
                <Star className="w-12 h-12 mx-auto text-muted-foreground mb-4" />
                <p className="text-muted-foreground">No favorite contacts yet</p>
              </div>
            ) : (
              <div className="space-y-3">
                {favorites.map((contact) => (
                  <ContactCard
                    key={contact.id}
                    contact={contact}
                    onSend={onSendToContact}
                    onDelete={handleDeleteContact}
                    onToggleFavorite={toggleFavorite}
                  />
                ))}
              </div>
            )}
          </TabsContent>

          {/* Recent Contacts */}
          <TabsContent value="recent" className="space-y-3">
            {recentContacts.length === 0 ? (
              <div className="text-center py-12">
                <Users className="w-12 h-12 mx-auto text-muted-foreground mb-4" />
                <p className="text-muted-foreground">No recent transactions</p>
              </div>
            ) : (
              <div className="space-y-3">
                {recentContacts.map((contact) => (
                  <ContactCard
                    key={contact.id}
                    contact={contact}
                    onSend={onSendToContact}
                    onDelete={handleDeleteContact}
                    onToggleFavorite={toggleFavorite}
                  />
                ))}
              </div>
            )}
          </TabsContent>
        </Tabs>
      </main>
    </div>
  );
}
