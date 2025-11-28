/**
 * Contacts Service
 * Manages address book, contacts, and contact groups
 */

import type {
  Contact,
  ContactInput,
  ContactGroup,
  ApiResponse,
  PaginatedResponse,
} from './types';
import { SocialFeatureError } from './types';
import { usernameService } from './UsernameService';

export class ContactsService {
  private apiUrl: string;
  private userId: string;

  constructor(
    userId: string,
    apiUrl: string = process.env.NEXT_PUBLIC_API_URL || 'http://localhost:3001'
  ) {
    this.userId = userId;
    this.apiUrl = apiUrl;
  }

  /**
   * Get all contacts for the current user
   */
  async getContacts(): Promise<Contact[]> {
    try {
      const response = await fetch(`${this.apiUrl}/contacts?userId=${this.userId}`);
      const result: ApiResponse<Contact[]> = await response.json();

      if (!result.success || !result.data) {
        throw new SocialFeatureError(
          result.error?.message || 'Failed to fetch contacts',
          result.error?.code || 'FETCH_FAILED'
        );
      }

      return result.data;
    } catch (error) {
      if (error instanceof SocialFeatureError) throw error;

      // Fallback to mock data for development
      console.warn('[ContactsService] API not available, using mock data');
      return this.mockGetContacts();
    }
  }

  /**
   * Get a specific contact by ID
   */
  async getContact(id: string): Promise<Contact | null> {
    try {
      const response = await fetch(`${this.apiUrl}/contacts/${id}`);
      const result: ApiResponse<Contact> = await response.json();

      if (!result.success || !result.data) {
        return null;
      }

      return result.data;
    } catch (error) {
      console.error('[ContactsService] Failed to fetch contact:', error);
      return null;
    }
  }

  /**
   * Add a new contact
   */
  async addContact(input: ContactInput): Promise<Contact> {
    // Validate input
    if (!input.contactAddress && !input.contactUsername) {
      throw new SocialFeatureError(
        'Either contact address or username is required',
        'INVALID_INPUT'
      );
    }

    // If username provided, resolve to address
    let contactAddress = input.contactAddress;
    if (input.contactUsername && !contactAddress) {
      try {
        contactAddress = await usernameService.resolveUsername(input.contactUsername);
      } catch (error) {
        throw new SocialFeatureError(
          'Failed to resolve username',
          'USERNAME_NOT_FOUND'
        );
      }
    }

    // If address provided but no username, try reverse lookup
    let contactUsername = input.contactUsername;
    if (contactAddress && !contactUsername) {
      contactUsername = (await usernameService.reverseResolve(contactAddress)) || undefined;
    }

    try {
      const response = await fetch(`${this.apiUrl}/contacts`, {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({
          userId: this.userId,
          contactAddress,
          contactUsername,
          nickname: input.nickname,
          notes: input.notes,
          tags: input.tags || [],
        }),
      });

      const result: ApiResponse<Contact> = await response.json();

      if (!result.success || !result.data) {
        throw new SocialFeatureError(
          result.error?.message || 'Failed to add contact',
          result.error?.code || 'ADD_FAILED'
        );
      }

      return result.data;
    } catch (error) {
      if (error instanceof SocialFeatureError) throw error;

      // Fallback to mock data for development
      console.warn('[ContactsService] API not available, using mock add');
      return this.mockAddContact(input, contactAddress!, contactUsername);
    }
  }

  /**
   * Update an existing contact
   */
  async updateContact(id: string, updates: Partial<Contact>): Promise<Contact> {
    try {
      const response = await fetch(`${this.apiUrl}/contacts/${id}`, {
        method: 'PATCH',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify(updates),
      });

      const result: ApiResponse<Contact> = await response.json();

      if (!result.success || !result.data) {
        throw new SocialFeatureError(
          result.error?.message || 'Failed to update contact',
          result.error?.code || 'UPDATE_FAILED'
        );
      }

      return result.data;
    } catch (error) {
      if (error instanceof SocialFeatureError) throw error;
      throw new SocialFeatureError('Failed to update contact', 'UPDATE_FAILED');
    }
  }

  /**
   * Delete a contact
   */
  async deleteContact(id: string): Promise<void> {
    try {
      const response = await fetch(`${this.apiUrl}/contacts/${id}`, {
        method: 'DELETE',
      });

      const result: ApiResponse<void> = await response.json();

      if (!result.success) {
        throw new SocialFeatureError(
          result.error?.message || 'Failed to delete contact',
          result.error?.code || 'DELETE_FAILED'
        );
      }
    } catch (error) {
      if (error instanceof SocialFeatureError) throw error;
      throw new SocialFeatureError('Failed to delete contact', 'DELETE_FAILED');
    }
  }

  /**
   * Search contacts by name, username, or address
   */
  async searchContacts(query: string): Promise<Contact[]> {
    if (!query || query.trim().length === 0) {
      return this.getContacts();
    }

    try {
      const response = await fetch(
        `${this.apiUrl}/contacts/search?userId=${this.userId}&q=${encodeURIComponent(query)}`
      );
      const result: ApiResponse<Contact[]> = await response.json();

      if (!result.success || !result.data) {
        throw new SocialFeatureError(
          result.error?.message || 'Failed to search contacts',
          result.error?.code || 'SEARCH_FAILED'
        );
      }

      return result.data;
    } catch (error) {
      if (error instanceof SocialFeatureError) throw error;

      // Fallback to local search
      const contacts = await this.getContacts();
      return this.localSearchContacts(contacts, query);
    }
  }

  /**
   * Get favorite contacts
   */
  async getFavorites(): Promise<Contact[]> {
    const contacts = await this.getContacts();
    return contacts.filter((c) => c.isFavorite);
  }

  /**
   * Get recent contacts (based on last transaction)
   */
  async getRecentContacts(limit: number = 10): Promise<Contact[]> {
    const contacts = await this.getContacts();
    return contacts
      .filter((c) => c.lastTransactionAt)
      .sort((a, b) => {
        const dateA = a.lastTransactionAt?.getTime() || 0;
        const dateB = b.lastTransactionAt?.getTime() || 0;
        return dateB - dateA;
      })
      .slice(0, limit);
  }

  /**
   * Toggle favorite status
   */
  async toggleFavorite(id: string): Promise<Contact> {
    const contact = await this.getContact(id);
    if (!contact) {
      throw new SocialFeatureError('Contact not found', 'NOT_FOUND');
    }

    return this.updateContact(id, { isFavorite: !contact.isFavorite });
  }

  /**
   * Get contacts by tag
   */
  async getContactsByTag(tag: string): Promise<Contact[]> {
    const contacts = await this.getContacts();
    return contacts.filter((c) => c.tags.includes(tag));
  }

  /**
   * Get all unique tags
   */
  async getTags(): Promise<string[]> {
    const contacts = await this.getContacts();
    const tagsSet = new Set<string>();
    contacts.forEach((c) => c.tags.forEach((tag) => tagsSet.add(tag)));
    return Array.from(tagsSet).sort();
  }

  // ============================================================================
  // CONTACT GROUPS
  // ============================================================================

  /**
   * Create a contact group
   */
  async createGroup(name: string, description?: string, contactIds: string[] = []): Promise<ContactGroup> {
    try {
      const response = await fetch(`${this.apiUrl}/contacts/groups`, {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ name, description, contactIds }),
      });

      const result: ApiResponse<ContactGroup> = await response.json();

      if (!result.success || !result.data) {
        throw new SocialFeatureError(
          result.error?.message || 'Failed to create group',
          result.error?.code || 'CREATE_FAILED'
        );
      }

      return result.data;
    } catch (error) {
      if (error instanceof SocialFeatureError) throw error;
      throw new SocialFeatureError('Failed to create group', 'CREATE_FAILED');
    }
  }

  /**
   * Get all contact groups
   */
  async getGroups(): Promise<ContactGroup[]> {
    try {
      const response = await fetch(`${this.apiUrl}/contacts/groups`);
      const result: ApiResponse<ContactGroup[]> = await response.json();

      if (!result.success || !result.data) {
        return [];
      }

      return result.data;
    } catch (error) {
      console.error('[ContactsService] Failed to fetch groups:', error);
      return [];
    }
  }

  // ============================================================================
  // UTILITIES
  // ============================================================================

  /**
   * Local search implementation (fallback)
   */
  private localSearchContacts(contacts: Contact[], query: string): Contact[] {
    const lowerQuery = query.toLowerCase();
    return contacts.filter(
      (c) =>
        c.nickname?.toLowerCase().includes(lowerQuery) ||
        c.contactUsername?.toLowerCase().includes(lowerQuery) ||
        c.contactAddress.toLowerCase().includes(lowerQuery) ||
        c.tags.some((tag) => tag.toLowerCase().includes(lowerQuery))
    );
  }

  /**
   * Generate avatar from address (deterministic)
   */
  static generateAvatar(address: string): string {
    // Use address hash to generate color
    const hash = address.split('').reduce((acc, char) => {
      return char.charCodeAt(0) + ((acc << 5) - acc);
    }, 0);

    const hue = Math.abs(hash) % 360;
    return `hsl(${hue}, 70%, 60%)`;
  }

  // ============================================================================
  // MOCK DATA (for development/testing)
  // ============================================================================

  private mockGetContacts(): Contact[] {
    return [
      {
        id: '1',
        userId: this.userId,
        contactAddress: '5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY',
        contactUsername: 'alice',
        nickname: 'Alice Cooper',
        isFavorite: true,
        tags: ['friends', 'defi'],
        createdAt: new Date('2024-01-15'),
        updatedAt: new Date('2024-01-15'),
        lastTransactionAt: new Date('2024-11-10'),
      },
      {
        id: '2',
        userId: this.userId,
        contactAddress: '5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty',
        contactUsername: 'bob',
        nickname: 'Bob Builder',
        isFavorite: false,
        tags: ['work'],
        createdAt: new Date('2024-02-20'),
        updatedAt: new Date('2024-02-20'),
        lastTransactionAt: new Date('2024-11-05'),
      },
    ];
  }

  private mockAddContact(
    input: ContactInput,
    contactAddress: string,
    contactUsername?: string
  ): Contact {
    return {
      id: `contact-${Date.now()}`,
      userId: this.userId,
      contactAddress,
      contactUsername,
      nickname: input.nickname,
      notes: input.notes,
      isFavorite: false,
      tags: input.tags || [],
      createdAt: new Date(),
      updatedAt: new Date(),
    };
  }
}

/**
 * Create a ContactsService instance
 */
export const createContactsService = (userId: string) => new ContactsService(userId);
