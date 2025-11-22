/**
 * Contact Card Component
 * Displays contact information with avatar and quick actions
 */

'use client';

import { formatDistanceToNow } from 'date-fns';
import { Send, Star, MoreVertical, Trash2, Edit } from 'lucide-react';
import { Button } from '@/components/ui/button';
import { Badge } from '@/components/ui/badge';
import { Avatar, AvatarFallback } from '@/components/ui/avatar';
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuTrigger,
} from '@/components/ui/dropdown-menu';
import { ContactsService } from '@/lib/social/ContactsService';
import type { Contact } from '@/lib/social/types';

interface ContactCardProps {
  contact: Contact;
  onSend?: (contact: Contact) => void;
  onEdit?: (contact: Contact) => void;
  onDelete?: (contact: Contact) => void;
  onToggleFavorite?: (contact: Contact) => void;
}

export function ContactCard({
  contact,
  onSend,
  onEdit,
  onDelete,
  onToggleFavorite,
}: ContactCardProps) {
  const displayName = contact.nickname || contact.contactUsername || formatAddress(contact.contactAddress);
  const avatarColor = ContactsService.generateAvatar(contact.contactAddress);

  const getInitials = () => {
    if (contact.nickname) {
      return contact.nickname.slice(0, 2).toUpperCase();
    }
    if (contact.contactUsername) {
      return contact.contactUsername.slice(0, 2).toUpperCase();
    }
    return contact.contactAddress.slice(0, 2).toUpperCase();
  };

  return (
    <div className="flex items-center gap-3 p-4 rounded-lg bg-card border border-border hover:bg-accent/5 transition-colors">
      {/* Avatar */}
      <Avatar className="w-12 h-12" style={{ backgroundColor: avatarColor }}>
        <AvatarFallback className="text-white font-semibold">
          {getInitials()}
        </AvatarFallback>
      </Avatar>

      {/* Info */}
      <div className="flex-1 min-w-0 space-y-1">
        <div className="flex items-center gap-2">
          <h3 className="font-semibold text-foreground truncate">{displayName}</h3>
          {contact.isFavorite && (
            <Star className="w-4 h-4 fill-yellow-500 text-yellow-500 flex-shrink-0" />
          )}
        </div>

        {/* Username or Address */}
        {contact.contactUsername ? (
          <p className="text-sm text-accent font-mono">@{contact.contactUsername}</p>
        ) : (
          <p className="text-xs text-muted-foreground font-mono truncate">
            {contact.contactAddress}
          </p>
        )}

        {/* Tags */}
        {contact.tags.length > 0 && (
          <div className="flex flex-wrap gap-1 mt-1">
            {contact.tags.slice(0, 3).map((tag) => (
              <Badge key={tag} variant="secondary" className="text-xs">
                {tag}
              </Badge>
            ))}
            {contact.tags.length > 3 && (
              <Badge variant="secondary" className="text-xs">
                +{contact.tags.length - 3}
              </Badge>
            )}
          </div>
        )}

        {/* Last Transaction */}
        {contact.lastTransactionAt && (
          <p className="text-xs text-muted-foreground">
            Last tx {formatDistanceToNow(contact.lastTransactionAt, { addSuffix: true })}
          </p>
        )}
      </div>

      {/* Actions */}
      <div className="flex items-center gap-2">
        {/* Quick Send Button */}
        {onSend && (
          <Button
            size="icon"
            variant="outline"
            onClick={() => onSend(contact)}
            className="h-9 w-9"
          >
            <Send className="w-4 h-4" />
          </Button>
        )}

        {/* More Actions Menu */}
        <DropdownMenu>
          <DropdownMenuTrigger asChild>
            <Button size="icon" variant="ghost" className="h-9 w-9">
              <MoreVertical className="w-4 h-4" />
            </Button>
          </DropdownMenuTrigger>
          <DropdownMenuContent align="end">
            {onToggleFavorite && (
              <DropdownMenuItem onClick={() => onToggleFavorite(contact)}>
                <Star className={`w-4 h-4 mr-2 ${contact.isFavorite ? 'fill-yellow-500' : ''}`} />
                {contact.isFavorite ? 'Remove from favorites' : 'Add to favorites'}
              </DropdownMenuItem>
            )}
            {onEdit && (
              <DropdownMenuItem onClick={() => onEdit(contact)}>
                <Edit className="w-4 h-4 mr-2" />
                Edit contact
              </DropdownMenuItem>
            )}
            {onDelete && (
              <DropdownMenuItem
                onClick={() => onDelete(contact)}
                className="text-destructive focus:text-destructive"
              >
                <Trash2 className="w-4 h-4 mr-2" />
                Delete contact
              </DropdownMenuItem>
            )}
          </DropdownMenuContent>
        </DropdownMenu>
      </div>
    </div>
  );
}

/**
 * Format address to shortened version
 */
function formatAddress(address: string): string {
  if (address.length <= 13) return address;
  return `${address.slice(0, 6)}...${address.slice(-4)}`;
}
