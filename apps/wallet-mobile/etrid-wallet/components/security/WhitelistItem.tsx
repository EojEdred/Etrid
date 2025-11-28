"use client"

import { Trash2, Edit2, CheckCircle } from "lucide-react"
import { WhitelistedAddress } from "@/lib/types/security"
import { Button } from "@/components/ui/button"
import { Badge } from "@/components/ui/badge"
import { useState } from "react"
import {
  AlertDialog,
  AlertDialogAction,
  AlertDialogCancel,
  AlertDialogContent,
  AlertDialogDescription,
  AlertDialogFooter,
  AlertDialogHeader,
  AlertDialogTitle,
  AlertDialogTrigger,
} from "@/components/ui/alert-dialog"

interface WhitelistItemProps {
  address: WhitelistedAddress
  onRemove?: () => void
  onEditLabel?: (newLabel: string) => void
}

export function WhitelistItem({ address, onRemove, onEditLabel }: WhitelistItemProps) {
  const [isEditing, setIsEditing] = useState(false)
  const [editedLabel, setEditedLabel] = useState(address.label || "")

  const handleSaveLabel = () => {
    if (onEditLabel && editedLabel.trim()) {
      onEditLabel(editedLabel.trim())
    }
    setIsEditing(false)
  }

  return (
    <div className="flex items-center justify-between p-4 rounded-lg border bg-card hover:bg-accent/5 transition-colors">
      <div className="flex-1">
        {/* Label */}
        {isEditing ? (
          <div className="flex items-center gap-2 mb-2">
            <input
              type="text"
              value={editedLabel}
              onChange={(e) => setEditedLabel(e.target.value)}
              className="px-2 py-1 rounded border bg-background text-sm font-semibold"
              placeholder="Enter label..."
              autoFocus
            />
            <Button size="sm" variant="ghost" onClick={handleSaveLabel}>
              <CheckCircle className="w-4 h-4" />
            </Button>
          </div>
        ) : (
          <div className="flex items-center gap-2 mb-2">
            <p className="font-semibold">
              {address.label || "Unlabeled Address"}
            </p>
            <Badge variant="outline" className="text-xs">
              Whitelisted
            </Badge>
          </div>
        )}

        {/* Address */}
        <code className="text-xs text-muted-foreground bg-muted px-2 py-1 rounded">
          {address.address.slice(0, 12)}...{address.address.slice(-10)}
        </code>

        {/* Added Info */}
        <p className="text-xs text-muted-foreground mt-2">
          Added {new Date(address.addedAt).toLocaleDateString()} by{" "}
          {address.addedBy.slice(0, 6)}...{address.addedBy.slice(-4)}
        </p>
      </div>

      {/* Actions */}
      <div className="flex items-center gap-2 ml-4">
        {onEditLabel && !isEditing && (
          <Button
            variant="ghost"
            size="icon"
            className="h-8 w-8"
            onClick={() => setIsEditing(true)}
          >
            <Edit2 className="w-4 h-4" />
          </Button>
        )}

        {onRemove && (
          <AlertDialog>
            <AlertDialogTrigger asChild>
              <Button
                variant="ghost"
                size="icon"
                className="h-8 w-8 text-destructive hover:text-destructive hover:bg-destructive/10"
              >
                <Trash2 className="w-4 h-4" />
              </Button>
            </AlertDialogTrigger>
            <AlertDialogContent>
              <AlertDialogHeader>
                <AlertDialogTitle>Remove from Whitelist?</AlertDialogTitle>
                <AlertDialogDescription>
                  This address will no longer bypass security checks. Transactions
                  to this address will be subject to timelock and other restrictions.
                </AlertDialogDescription>
              </AlertDialogHeader>
              <AlertDialogFooter>
                <AlertDialogCancel>Cancel</AlertDialogCancel>
                <AlertDialogAction onClick={onRemove} className="bg-destructive text-destructive-foreground hover:bg-destructive/90">
                  Remove
                </AlertDialogAction>
              </AlertDialogFooter>
            </AlertDialogContent>
          </AlertDialog>
        )}
      </div>
    </div>
  )
}
